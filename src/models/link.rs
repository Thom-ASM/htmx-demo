use super::platform::{self, Platform};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub linkid: String,
    pub val: String,
    pub platform: Platform,
    pub userid: String,
}

impl Link {
    pub fn new(val: Option<&str>, platform: Option<Platform>, userid: &str) -> Link {
        return Link {
            linkid: Uuid::new_v4().to_string(),
            val: val.or_else(|| Some("")).unwrap().to_string(),
            platform: platform.or_else(|| Some(Platform::GITHUB)).unwrap(),
            userid: userid.to_string(),
        };
    }
}
