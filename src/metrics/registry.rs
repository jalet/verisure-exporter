use std::sync::atomic::AtomicU64;

use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct InstallationLabels {
    pub installation: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ClimateLabels {
    pub installation: String,
    pub device_label: String,
    pub area: String,
    pub device_type: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct DeviceLabels {
    pub installation: String,
    pub device_label: String,
    pub area: String,
}

pub struct Metrics {
    pub alarm_armed_state: Family<InstallationLabels, Gauge>,
    pub alarm_changed_timestamp: Family<InstallationLabels, Gauge>,
    pub temperature_celsius: Family<ClimateLabels, Gauge<f64, AtomicU64>>,
    pub humidity_percent: Family<ClimateLabels, Gauge<f64, AtomicU64>>,
    pub door_window_open: Family<DeviceLabels, Gauge>,
    pub door_window_report_timestamp: Family<DeviceLabels, Gauge>,
    pub lock_locked: Family<DeviceLabels, Gauge>,
    pub lock_motor_jam: Family<DeviceLabels, Gauge>,
    pub lock_secure_mode: Family<DeviceLabels, Gauge>,
    pub smartplug_on: Family<DeviceLabels, Gauge>,
    pub broadband_connected: Family<InstallationLabels, Gauge>,
    pub scrape_duration_seconds: Gauge<f64, AtomicU64>,
    pub scrape_success: Gauge,
    pub scrape_errors_total: Counter,
}

impl Metrics {
    pub fn new(registry: &mut Registry) -> Self {
        let sub = registry.sub_registry_with_prefix("verisure");

        let alarm_armed_state = Family::default();
        sub.register(
            "alarm_armed_state",
            "Arm state: 0=disarmed 1=armed_home 2=armed_away",
            alarm_armed_state.clone(),
        );

        let alarm_changed_timestamp = Family::default();
        sub.register(
            "alarm_changed_timestamp_seconds",
            "Unix timestamp of last arm state change",
            alarm_changed_timestamp.clone(),
        );

        let temperature_celsius = Family::default();
        sub.register(
            "temperature_celsius",
            "Temperature in degrees Celsius",
            temperature_celsius.clone(),
        );

        let humidity_percent = Family::default();
        sub.register(
            "humidity_percent",
            "Relative humidity percentage",
            humidity_percent.clone(),
        );

        let door_window_open = Family::default();
        sub.register(
            "door_window_open",
            "1 if door or window is open 0 if closed",
            door_window_open.clone(),
        );

        let door_window_report_timestamp = Family::default();
        sub.register(
            "door_window_report_timestamp_seconds",
            "Unix timestamp of last door window state report",
            door_window_report_timestamp.clone(),
        );

        let lock_locked = Family::default();
        sub.register(
            "lock_locked",
            "1 if lock is locked 0 if unlocked",
            lock_locked.clone(),
        );

        let lock_motor_jam = Family::default();
        sub.register(
            "lock_motor_jam",
            "1 if lock motor is jammed",
            lock_motor_jam.clone(),
        );

        let lock_secure_mode = Family::default();
        sub.register(
            "lock_secure_mode",
            "1 if lock secure mode is active",
            lock_secure_mode.clone(),
        );

        let smartplug_on = Family::default();
        sub.register(
            "smartplug_on",
            "1 if smart plug is on 0 if off",
            smartplug_on.clone(),
        );

        let broadband_connected = Family::default();
        sub.register(
            "broadband_connected",
            "1 if broadband is connected",
            broadband_connected.clone(),
        );

        let scrape_duration_seconds = Gauge::default();
        sub.register(
            "scrape_duration_seconds",
            "Duration of last scrape in seconds",
            scrape_duration_seconds.clone(),
        );

        let scrape_success = Gauge::default();
        sub.register(
            "scrape_success",
            "1 if last scrape was successful",
            scrape_success.clone(),
        );

        let scrape_errors_total = Counter::default();
        sub.register(
            "scrape_errors_total",
            "Total number of scrape errors",
            scrape_errors_total.clone(),
        );

        Self {
            alarm_armed_state,
            alarm_changed_timestamp,
            temperature_celsius,
            humidity_percent,
            door_window_open,
            door_window_report_timestamp,
            lock_locked,
            lock_motor_jam,
            lock_secure_mode,
            smartplug_on,
            broadband_connected,
            scrape_duration_seconds,
            scrape_success,
            scrape_errors_total,
        }
    }
}
