use serde_json::{json, Value};

pub fn arm_state_query(giid: &str) -> Value {
    json!({
        "operationName": "ArmState",
        "variables": { "giid": giid },
        "query": "query ArmState($giid: String!) { installation(giid: $giid) { armState { type statusType date name changedVia } } }"
    })
}

pub fn climate_query(giid: &str) -> Value {
    json!({
        "operationName": "Climate",
        "variables": { "giid": giid },
        "query": "query Climate($giid: String!) { installation(giid: $giid) { climates { device { deviceLabel area } humidityEnabled humidityTimestamp humidityValue temperatureTimestamp temperatureValue } } }"
    })
}

pub fn door_window_query(giid: &str) -> Value {
    json!({
        "operationName": "DoorWindow",
        "variables": { "giid": giid },
        "query": "query DoorWindow($giid: String!) { installation(giid: $giid) { doorWindows { device { deviceLabel area } state wired reportTime } } }"
    })
}

pub fn door_lock_query(giid: &str) -> Value {
    json!({
        "operationName": "DoorLock",
        "variables": { "giid": giid },
        "query": "query DoorLock($giid: String!) { installation(giid: $giid) { smartLocks { device { deviceLabel area } lockStatus doorState lockMethod eventTime doorLockType secureMode user { name } } } }"
    })
}

pub fn smart_plug_query(giid: &str) -> Value {
    json!({
        "operationName": "SmartPlug",
        "variables": { "giid": giid },
        "query": "query SmartPlug($giid: String!) { installation(giid: $giid) { smartplugs { device { deviceLabel area } currentState icon isHazardous } } }"
    })
}

pub fn broadband_query(giid: &str) -> Value {
    json!({
        "operationName": "Broadband",
        "variables": { "giid": giid },
        "query": "query Broadband($giid: String!) { installation(giid: $giid) { broadband { testDate isBroadbandConnected } } }"
    })
}

pub fn account_installations_query(email: &str) -> Value {
    json!({
        "operationName": "AccountInstallations",
        "variables": { "email": email },
        "query": "query AccountInstallations($email: String!) { account(email: $email) { installations { giid alias } } }"
    })
}
