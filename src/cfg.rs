use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    pub record_pth: String,
    pub pswd: String,
}

impl Cfg {
    pub fn new(reocrd_path: String) -> Cfg {
        Cfg {
            record_pth: reocrd_path,
            pswd: String::new(),
        }
    }
}
