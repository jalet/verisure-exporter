#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_label: String,
    pub area: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArmState {
    pub status_type: String,
    pub date: Option<String>,
    pub name: Option<String>,
    pub changed_via: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClimateValue {
    pub device: Device,
    pub humidity_enabled: Option<bool>,
    pub humidity_timestamp: Option<String>,
    pub humidity_value: Option<f64>,
    pub temperature_timestamp: Option<String>,
    pub temperature_value: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoorWindow {
    pub device: Device,
    pub state: String,
    pub wired: Option<bool>,
    pub report_time: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoorLock {
    pub device: Device,
    pub lock_status: Option<String>,
    pub door_state: Option<String>,
    pub lock_method: Option<String>,
    pub event_time: Option<String>,
    pub door_lock_type: Option<String>,
    pub secure_mode: Option<String>,
    pub user_string: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmartPlug {
    pub device: Device,
    pub current_state: String,
    pub icon: Option<String>,
    pub is_hazardous: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Broadband {
    pub test_date: Option<String>,
    pub is_broadband_connected: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct VerisureData {
    pub arm_state: Option<ArmState>,
    pub climate_values: Vec<ClimateValue>,
    pub door_windows: Vec<DoorWindow>,
    pub door_locks: Vec<DoorLock>,
    pub smart_plugs: Vec<SmartPlug>,
    pub broadband: Option<Broadband>,
}
