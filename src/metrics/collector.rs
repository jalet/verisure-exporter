use chrono::DateTime;
use tracing::warn;

use crate::verisure::VerisureData;

use super::registry::{ClimateLabels, DeviceLabels, InstallationLabels, Metrics};

fn parse_timestamp(s: &str) -> Option<i64> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.timestamp())
        .ok()
}

pub fn update_metrics(data: &VerisureData, metrics: &Metrics, giid: &str) {
    let inst_labels = InstallationLabels {
        installation: giid.to_string(),
    };

    if let Some(arm) = &data.arm_state {
        let value: i64 = match arm.status_type.as_str() {
            "DISARMED" => 0,
            "ARMED_HOME" => 1,
            "ARMED_AWAY" => 2,
            other => {
                warn!(status = other, "Unknown arm state");
                -1
            }
        };
        metrics
            .alarm_armed_state
            .get_or_create(&inst_labels)
            .set(value);

        if let Some(date) = &arm.date {
            if let Some(ts) = parse_timestamp(date) {
                metrics
                    .alarm_changed_timestamp
                    .get_or_create(&inst_labels)
                    .set(ts);
            }
        }
    }

    for cv in &data.climate_values {
        let labels = ClimateLabels {
            installation: giid.to_string(),
            device_label: cv.device.device_label.clone(),
            area: cv.device_area.clone().unwrap_or_default(),
            device_type: cv.device_type.clone().unwrap_or_default(),
        };
        if let Some(temp) = cv.temperature {
            metrics.temperature_celsius.get_or_create(&labels).set(temp);
        }
        if let Some(hum) = cv.humidity {
            metrics.humidity_percent.get_or_create(&labels).set(hum);
        }
    }

    for dw in &data.door_windows {
        let labels = DeviceLabels {
            installation: giid.to_string(),
            device_label: dw.device.device_label.clone(),
            area: dw.area.clone().unwrap_or_default(),
        };
        let open: i64 = match dw.state.as_str() {
            "OPEN" => 1,
            "CLOSE" | "CLOSED" => 0,
            other => {
                warn!(state = other, "Unknown door/window state");
                -1
            }
        };
        metrics.door_window_open.get_or_create(&labels).set(open);

        if let Some(rt) = &dw.report_time {
            if let Some(ts) = parse_timestamp(rt) {
                metrics
                    .door_window_report_timestamp
                    .get_or_create(&labels)
                    .set(ts);
            }
        }
    }

    for lock in &data.door_locks {
        let labels = DeviceLabels {
            installation: giid.to_string(),
            device_label: lock.device.device_label.clone(),
            area: lock.area.clone().unwrap_or_default(),
        };
        let locked: i64 = match lock.current_lock_state.as_str() {
            "LOCKED" => 1,
            "UNLOCKED" => 0,
            other => {
                warn!(state = other, "Unknown lock state");
                -1
            }
        };
        metrics.lock_locked.get_or_create(&labels).set(locked);
        metrics
            .lock_motor_jam
            .get_or_create(&labels)
            .set(i64::from(lock.motor_jam.unwrap_or(false)));
        metrics
            .lock_secure_mode
            .get_or_create(&labels)
            .set(i64::from(lock.secure_mode_active.unwrap_or(false)));
    }

    for plug in &data.smart_plugs {
        let labels = DeviceLabels {
            installation: giid.to_string(),
            device_label: plug.device.device_label.clone(),
            area: plug.area.clone().unwrap_or_default(),
        };
        let on: i64 = match plug.current_state.as_str() {
            "ON" => 1,
            "OFF" => 0,
            other => {
                warn!(state = other, "Unknown smart plug state");
                -1
            }
        };
        metrics.smartplug_on.get_or_create(&labels).set(on);
    }

    if let Some(bb) = &data.broadband {
        metrics
            .broadband_connected
            .get_or_create(&inst_labels)
            .set(i64::from(bb.is_broadband_connected.unwrap_or(false)));
    }
}
