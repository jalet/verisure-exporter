#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_label: String,
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
    pub device_area: Option<String>,
    pub device_type: Option<String>,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub time: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoorWindow {
    pub device: Device,
    pub area: Option<String>,
    pub state: String,
    pub wired: Option<bool>,
    pub report_time: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoorLock {
    pub device: Device,
    pub current_lock_state: String,
    pub area: Option<String>,
    pub event_time: Option<String>,
    pub secure_mode_active: Option<bool>,
    pub motor_jam: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmartPlug {
    pub device: Device,
    pub area: Option<String>,
    pub current_state: String,
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
