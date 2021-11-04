
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub tenant_code: String,
    pub img: String,
    pub distance: String,
    pub dis: f64,
    pub ids_url: String,
    pub join_type: String,
    pub app_id: String,
    pub cas_login_url: String,
    pub is_enter: i32
}
