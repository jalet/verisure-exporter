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
        "query": "query Climate($giid: String!) { installation(giid: $giid) { climateValues { device { deviceLabel } deviceArea deviceType temperature humidity time } } }"
    })
}

pub fn door_window_query(giid: &str) -> Value {
    json!({
        "operationName": "DoorWindow",
        "variables": { "giid": giid },
        "query": "query DoorWindow($giid: String!) { installation(giid: $giid) { doorWindows { device { deviceLabel } area state wired reportTime } } }"
    })
}

pub fn door_lock_query(giid: &str) -> Value {
    json!({
        "operationName": "DoorLock",
        "variables": { "giid": giid },
        "query": "query DoorLock($giid: String!) { installation(giid: $giid) { doorLockStatusList { device { deviceLabel } currentLockState area eventTime secureModeActive motorJam } } }"
    })
}

pub fn smart_plug_query(giid: &str) -> Value {
    json!({
        "operationName": "SmartPlug",
        "variables": { "giid": giid },
        "query": "query SmartPlug($giid: String!) { installation(giid: $giid) { smartplugs { device { deviceLabel } area currentState } } }"
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
