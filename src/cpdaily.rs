pub mod client;
pub mod crypto;
pub mod structs;

use serde_json::Value;
use curl::easy::{Easy, List};
use std::str;
use anyhow::Result;

use self::structs::tenants::Tenant;

fn get(url: &str) -> Result<Value> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    let mut headers = List::new();
    headers.append("Accept: application/json").unwrap();
    headers.append("User-Agent: Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36").unwrap();

    easy.url(url).unwrap();
    easy.get(true).unwrap();
    easy.http_headers(headers).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|r| {
            data.extend_from_slice(r);
            Ok(r.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let code = easy.response_code().expect("Retrieving response code");
    if code != 200 {
        return Err(anyhow::anyhow!("Response code is not 200"));
    }

    let s = str::from_utf8(&data).expect("Converting [u8] to string");
    let parsed = serde_json::from_str(s).expect("Parsing JSON");
    Ok(parsed)
}

pub fn get_all_tenants() -> Result<Vec<Tenant>> {
    let response = get("https://mobile.campushoy.com/v6/config/guest/tenant/list").expect("Fetching tenant list");
    let tenants : Vec<Tenant> = serde_json::from_value(response.get("data").expect("Retrieving data from tenant list response").to_owned()).expect("Parsing tenant list");
    Ok(tenants)
}

// pub fn GetHostByName(SchoolName:String) -> String{
//     let mut Headers = List::new();
    
//     let mut dst = Vec::new();
//     Headers.append("User-Agent: Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36").unwrap();
//     {
//         let mut HttpReq = Easy::new();
//         HttpReq.http_headers(Headers);
//         HttpReq.url("https://mobile.campushoy.com/v6/config/guest/tenant/list").unwrap();
//         let mut tf = HttpReq.transfer();
//         tf.write_function(|data| {
//             dst.extend_from_slice(data);
//             Ok(data.len())
//         });
//         tf.perform().unwrap();
//     }
//     let s = match str::from_utf8(&dst) {
//         Ok(v) => v,
//         Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//     };
    
//     let school_json: Value = serde_json::from_str(s).unwrap();
//     let schools = school_json["data"].as_array().unwrap();
//     for ele in schools {
//         if(ele["name"]==SchoolName){
//             let schoolid = String::from(ele["id"].as_str().unwrap());
//         }
//     }
//     println!("{0}\n{1}",schools.len(),s.len());
//     return String::from("tr");
// }

#[cfg(test)]
mod tests {
    use crate::cpdaily;

    #[test]
    fn test_get_all_tenants() {
        let tenant_list = cpdaily::get_all_tenants();
        tenant_list.expect("Fetching tenant list");
    }
}
