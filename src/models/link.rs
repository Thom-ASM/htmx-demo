use super::platform::Platform;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Link {
    pub linkid: String,
    pub val: String,
    pub platform: Platform,
    pub userid: String,
    pub active: bool,
}

impl Link {
    pub fn new(
        userid: &str,
        val: Option<&str>,
        platform: Option<Platform>,
        isActive: Option<bool>,
    ) -> Link {
        return Link {
            linkid: Uuid::new_v4().to_string(),
            val: val.or_else(|| Some("")).unwrap().to_string(),
            platform: platform.or_else(|| Some(Platform::GITHUB)).unwrap(),
            userid: userid.to_string(),
            active: isActive.or_else(|| Some(false)).unwrap(),
        };
    }
}
