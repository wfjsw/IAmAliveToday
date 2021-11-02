pub mod client;
pub mod crypto;
pub mod structs;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::io::{stdout, Write};
use curl::easy::{Easy, List};
use std::str;

pub fn GetHostByName(SchoolName:String) -> String{
    let mut Headers = List::new();
    
    let mut dst = Vec::new();
    Headers.append("User-Agent: Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36").unwrap();
    {
        let mut HttpReq = Easy::new();
        HttpReq.http_headers(Headers);
        HttpReq.url("https://mobile.campushoy.com/v6/config/guest/tenant/list").unwrap();
        let mut tf = HttpReq.transfer();
        tf.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        });
        tf.perform().unwrap();
    }
    let s = match str::from_utf8(&dst) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    
    let school_json: Value = serde_json::from_str(s).unwrap();
    let schools = school_json["data"].as_array().unwrap();
    for ele in schools {
        if(ele["name"]==SchoolName){
            let schoolid = String::from(ele["id"].as_str().unwrap());
        }
    }
    println!("{0}\n{1}",schools.len(),s.len());
    return String::from("tr");
}
