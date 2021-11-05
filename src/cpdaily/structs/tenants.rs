
use curl::easy::List;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::cpdaily::client::Client;
use crate::cpdaily::loginprovider::LoginProvider;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TenantDetail {
    pub allow_send_msg: i32,
    pub amp3_url: String,
    pub amp_robot_url: String,
    pub amp_url: String,
    pub amp_url2: String,
    pub app_cache_disable: String,
    pub app_id: String,
    pub app_secret: String,
    pub app_store_url: String,
    pub app_style_res_url: String,
    pub app_style_version_id: String,
    pub bad_https_block: String,
    pub campus_req_proxy: String,
    pub can_ids_login: String,
    pub can_interactive: i32,
    pub cas_login_url: String,
    pub circle_can_see_off_campus: String,
    pub circle_show_type: String,
    pub college_town: String,
    pub contact_display_item: String,
    pub contact_display_item_teacher: String,
    pub distance: String,
    pub faq_forum_id: String,
    pub fresh_post_range: String,
    pub has_open_message_fresh: String,
    pub home_first_show: String,
    pub home_page_display_item: String,
    pub home_page_display_item_teacher: String,
    pub i_robot_url: String,
    pub id: String,
    pub ids_url: String,
    pub img: String,
    pub is_amp_proxy: String,
    pub is_enter: i32,
    pub is_ids_proxy: String,
    pub is_need_alias: String,
    pub is_open_fission: String,
    pub is_open_oauth: String,
    pub is_show_hot_list: String,
    pub join_type: String,
    pub like_btn_space: String,
    pub loss_pwd_desc: String,
    pub media_version: String,
    pub modify_pass_descr: String,
    pub modify_pass_success_url: String,
    pub modify_pass_url: String,
    pub msg_access_token: String,
    pub msg_app_id: String,
    pub msg_app_id_ios: String,
    pub msg_url: String,
    pub name: String,
    pub no_auth_home_pages: Vec<i32>,
    pub priority_url: String,
    pub province_id: String,
    pub schedule_all_data_url: String,
    pub schedule_data_url: String,
    pub schedule_open_url: String,
    pub schedule_update_data_url: String,
    pub second_hand_switch: String,
    pub service_page_place: String,
    pub shop_url: String,
    pub short_name: String,
    pub student_home_pages:  Vec<i32>,
    pub student_version: String,
    pub tao_banner_id: String,
    pub task_app_id: String,
    pub task_url: String,
    pub teacher_home_pages: Vec<String>,
    pub teacher_version: String,
    pub tenant_code: String,
    pub tenant_name_img: String,
    pub user_show_college: String,
    pub xyk_url: String,
    pub yb_switch: String,
    pub yiban_auth_type: String,
    pub yiban_build: i32,
    pub ykt_balance_url: String,
    pub ykt_qr_code_url: String,
    pub ykt_transfer_url: String,
    pub ywt_prefix_url: String,
    pub ywt_service_url: String,
    pub ywt_status: String,
    pub zg_app_key: String,
}

impl Tenant {
    pub fn get_info(&self) -> anyhow::Result<TenantDetail> {
        let result = get("")?;
        Ok(serde_json::from_value(result.get("data").expect("Data not found").get(0).expect("School not found").to_owned())?)
    }
}

impl TenantDetail {
    pub fn create_login() -> LoginProvider {
        
    }
}

fn get(url: &str) -> anyhow::Result<Value> {
    let mut headers = List::new();
    headers.append("Accept: application/json").unwrap();
    headers.append("User-Agent: Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36").unwrap();
    let data = Client::perform(url, Some(headers), Some("GET"), None, true)?.1;
    let parsed = serde_json::from_str(&data).expect("Parsing JSON");
    Ok(parsed)
}

