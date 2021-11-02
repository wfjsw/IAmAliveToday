
use serde::{Deserialize, Serialize};

use crate::config::{User, DeviceInfo};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    model: String,
    app_version: String,
    system_version: String,
    user_id: String,
    system_name: String,
    lat: f64,
    lon: f64,
    device_id: String,
}

impl Extensions {
    pub fn from_user_id_and_deviceinfo(username: &str, deviceinfo: &DeviceInfo) -> Self {
        Extensions {
            model: deviceinfo.model.clone(),
            app_version: deviceinfo.app_version.clone(),
            system_version: deviceinfo.system_version.clone(),
            user_id: username.to_string(), // TODO
            system_name: deviceinfo.system_name.clone(),
            lat: deviceinfo.lat,
            lon: deviceinfo.lon,
            device_id: deviceinfo.device_id.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
