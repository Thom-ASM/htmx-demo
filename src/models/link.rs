use super::platform::Platform;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub linkid: String,
    pub val: String,
    pub platform: Platform,
    pub userid: String,
}
