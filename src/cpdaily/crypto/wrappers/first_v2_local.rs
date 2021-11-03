use super::traits::FirstV2;
const ckey: &'static str = "CNCytgOo";
const fkey: &'static str = "yZtuU8Qm";

pub struct FirstV2Local {
    chk: String,
    fhk: String,
}

impl FirstV2 for FirstV2Local {
    fn new() -> Self {
        // fetch from getSecret
        
        FirstV2Local {
            chk: ckey.to_string(),
            fhk: fkey.to_string(),
        }
    }
}
