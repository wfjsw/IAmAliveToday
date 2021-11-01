use std::{fs::File, io::Read};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub users: Vec<User>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub school_name: String,
    pub username: String,
    pub password: String,
    pub actions: Vec<Action>,
    pub device_info: DeviceInfo,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub model: String,
    pub app_version: String,
    pub system_version: String,
    pub system_name: String,
    pub device_id: String,
    pub lat: String,
    pub lon: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    FormFill(FormFillAction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FormFillAction {

}

pub fn get_config(path: &str) -> Config {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = serde_yaml::from_str(&contents).unwrap();
    config
}

impl User {
    pub fn get_cpdaily_extension() -> String {
        "com.cpdaily.extension".to_string()
    }
}
