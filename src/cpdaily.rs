pub mod client;
pub mod crypto;
pub mod loginprovider;
pub mod structs;

use anyhow::Result;
use serde_json::Value;
use std::str;

use self::structs::tenants::Tenant;

pub fn get_all_tenants() -> Result<Vec<Tenant>> {
    let response: Value = client::unauth()?
        .get("https://mobile.campushoy.com/v6/config/guest/tenant/list")
        .send()?
        .json()?;
    let tenants: Vec<Tenant> = serde_json::from_value(
        response
            .get("data")
            .expect("Retrieving data from tenant list response")
            .to_owned(),
    )
    .expect("Parsing tenant list");
    Ok(tenants)
}

pub fn match_school_from_tenant_list<'a>(
    list: &'a Vec<Tenant>,
    identifier: &str,
) -> anyhow::Result<&'a Tenant> {
    for tenant in list {
        if tenant.id == identifier {
            return Ok(tenant);
        } else if tenant.name.contains(identifier) {
            return Ok(tenant);
        }
    }
    Err(anyhow::anyhow!("No matching tenant found"))
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
