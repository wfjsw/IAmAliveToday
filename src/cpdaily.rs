pub mod client;
pub mod crypto;
pub mod structs;
pub mod loginprovider;

use serde_json::Value;
use curl::easy::List;
use std::str;
use anyhow::Result;

use self::client::Client;
use self::structs::tenants::Tenant;

fn get(url: &str) -> Result<Value> {
    let mut headers = List::new();
    headers.append("Accept: application/json").unwrap();
    headers.append("User-Agent: Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36").unwrap();
    let data = Client::perform(url, Some(headers), Some("GET"), None, true)?.1;
    let parsed = serde_json::from_str(&data).expect("Parsing JSON");
    Ok(parsed)
}

pub fn get_all_tenants() -> Result<Vec<Tenant>> {
    let response = get("https://mobile.campushoy.com/v6/config/guest/tenant/list").expect("Fetching tenant list");
    let tenants : Vec<Tenant> = serde_json::from_value(response.get("data").expect("Retrieving data from tenant list response").to_owned()).expect("Parsing tenant list");
    Ok(tenants)
}



#[cfg(test)]
mod tests {
    use crate::cpdaily;

    #[test]
    fn test_get_all_tenants() {
        let tenant_list = cpdaily::get_all_tenants();
        tenant_list.expect("Fetching tenant list");
    }
}
