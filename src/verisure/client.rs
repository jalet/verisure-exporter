use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use tracing::{debug, info, warn};

use super::{queries, types::*};
use crate::config::Config;

#[derive(Debug, Error)]
pub enum VerisureError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("No installations found")]
    NoInstallations,
    #[error(
        "Multiple installations found, please specify --giid. Found installations logged above."
    )]
    MultipleInstallations,
}

pub struct VerisureClient {
    http: Client,
    base_url: String,
    username: String,
    password: String,
    giid: Arc<Mutex<Option<String>>>,
}

impl VerisureClient {
    pub async fn new(config: &Config) -> Result<Self, VerisureError> {
        let http = Client::builder()
            .cookie_store(true)
            .user_agent("verisure-exporter/0.1.0")
            .build()?;

        Ok(Self {
            http,
            base_url: config.api_url.clone(),
            username: config.username.clone(),
            password: config.password.clone(),
            giid: Arc::new(Mutex::new(config.giid.clone())),
        })
    }

    pub async fn init(&self) -> Result<(), VerisureError> {
        self.login().await?;
        self.ensure_giid().await?;
        Ok(())
    }

    async fn login(&self) -> Result<(), VerisureError> {
        info!("Authenticating with Verisure API");
        let resp = self
            .http
            .post(format!("{}/auth/login", self.base_url))
            .header("APPLICATION_ID", "PS_PYTHON")
            .basic_auth(&self.username, Some(&self.password))
            .header("Content-Length", "0")
            .body("")
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(VerisureError::Auth(format!(
                "Login failed with status {}: {}",
                status, body
            )));
        }

        let body: Value = resp.json().await?;
        debug!("Login response: {:?}", body);

        if let Some(step_up) = body.get("stepUpToken") {
            if !step_up.is_null() {
                return Err(VerisureError::Auth(
                    "MFA (stepUpToken) required but not supported".to_string(),
                ));
            }
        }

        info!("Authentication successful");
        Ok(())
    }

    async fn ensure_giid(&self) -> Result<(), VerisureError> {
        let mut giid_lock = self.giid.lock().await;
        if giid_lock.is_some() {
            return Ok(());
        }

        info!("Auto-detecting installation GIID");
        let query = queries::account_installations_query(&self.username);
        let resp = self
            .http
            .post(format!("{}/graphql", self.base_url))
            .json(&query)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            return Err(VerisureError::Api(format!(
                "Installations query failed with status {}",
                status
            )));
        }

        let body: Value = resp.json().await?;
        let installations_val = body
            .get("data")
            .and_then(|d| d.get("account"))
            .and_then(|a| a.get("installations"))
            .cloned()
            .unwrap_or(Value::Array(vec![]));

        #[derive(Deserialize)]
        struct Inst {
            giid: String,
            alias: Option<String>,
        }

        let installations: Vec<Inst> = serde_json::from_value(installations_val)?;

        match installations.len() {
            0 => Err(VerisureError::NoInstallations),
            1 => {
                let giid = installations[0].giid.clone();
                info!(
                    giid = %giid,
                    alias = installations[0].alias.as_deref().unwrap_or("unnamed"),
                    "Found single installation"
                );
                *giid_lock = Some(giid);
                Ok(())
            }
            _ => {
                for inst in &installations {
                    warn!(
                        giid = %inst.giid,
                        alias = inst.alias.as_deref().unwrap_or("unnamed"),
                        "Found installation"
                    );
                }
                Err(VerisureError::MultipleInstallations)
            }
        }
    }

    pub async fn introspect(&self, type_name: &str) -> Result<String, VerisureError> {
        let query = serde_json::json!({
            "query": format!(
                "{{ __type(name: \"{type_name}\") {{ fields {{ name type {{ name kind ofType {{ name }} }} }} }} }}"
            )
        });
        let resp = self
            .http
            .post(format!("{}/graphql", self.base_url))
            .json(&query)
            .send()
            .await?;
        let body: Value = resp.json().await?;
        Ok(serde_json::to_string_pretty(&body).unwrap_or_default())
    }

    pub async fn get_giid(&self) -> Option<String> {
        self.giid.lock().await.clone()
    }

    pub async fn fetch_all(&self) -> Result<VerisureData, VerisureError> {
        let giid = {
            let lock = self.giid.lock().await;
            lock.clone()
                .ok_or_else(|| VerisureError::Api("GIID not set".to_string()))?
        };

        match self.do_fetch(&giid).await {
            Err(VerisureError::Auth(_)) => {
                warn!("Session expired, re-authenticating");
                self.login().await?;
                self.do_fetch(&giid).await
            }
            other => other,
        }
    }

    async fn do_fetch(&self, giid: &str) -> Result<VerisureData, VerisureError> {
        let queries = vec![
            queries::arm_state_query(giid),
            queries::climate_query(giid),
            queries::door_window_query(giid),
            queries::door_lock_query(giid),
            queries::smart_plug_query(giid),
            queries::broadband_query(giid),
        ];

        let resp = self
            .http
            .post(format!("{}/graphql", self.base_url))
            .json(&queries)
            .send()
            .await?;

        let status = resp.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(VerisureError::Auth("Unauthorized".to_string()));
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(VerisureError::Api(format!(
                "GraphQL request failed {}: {}",
                status, body
            )));
        }

        let body: Vec<Value> = resp.json().await?;
        self.parse_response(body)
    }

    fn parse_response(&self, responses: Vec<Value>) -> Result<VerisureData, VerisureError> {
        let mut data = VerisureData::default();

        for (i, resp) in responses.iter().enumerate() {
            if let Some(errors) = resp.get("errors") {
                warn!(index = i, errors = %errors, "GraphQL errors in response");
                continue;
            }

            let Some(inst) = resp.get("data").and_then(|d| d.get("installation")) else {
                continue;
            };

            if let Some(arm) = inst.get("armState") {
                match serde_json::from_value::<ArmState>(arm.clone()) {
                    Ok(a) => data.arm_state = Some(a),
                    Err(e) => warn!("Failed to parse armState: {}", e),
                }
            }

            if let Some(climate) = inst.get("climates") {
                match serde_json::from_value::<Vec<ClimateValue>>(climate.clone()) {
                    Ok(v) => data.climate_values = v,
                    Err(e) => warn!("Failed to parse climates: {}", e),
                }
            }

            if let Some(dw) = inst.get("doorWindows") {
                match serde_json::from_value::<Vec<DoorWindow>>(dw.clone()) {
                    Ok(v) => data.door_windows = v,
                    Err(e) => warn!("Failed to parse doorWindows: {}", e),
                }
            }

            if let Some(locks) = inst.get("smartLocks") {
                match serde_json::from_value::<Vec<DoorLock>>(locks.clone()) {
                    Ok(v) => data.door_locks = v,
                    Err(e) => warn!("Failed to parse smartLocks: {}", e),
                }
            }

            if let Some(plugs) = inst.get("smartplugs") {
                match serde_json::from_value::<Vec<SmartPlug>>(plugs.clone()) {
                    Ok(v) => data.smart_plugs = v,
                    Err(e) => warn!("Failed to parse smartplugs: {}", e),
                }
            }

            if let Some(bb) = inst.get("broadband") {
                match serde_json::from_value::<Broadband>(bb.clone()) {
                    Ok(b) => data.broadband = Some(b),
                    Err(e) => warn!("Failed to parse broadband: {}", e),
                }
            }
        }

        Ok(data)
    }
}
