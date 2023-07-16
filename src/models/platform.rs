use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Platform {
    GITHUB,
    LINKEDIN,
    YOUTUBE,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Self::GITHUB => "Github".to_owned(),
            Self::LINKEDIN => "LinkedIn".to_owned(),
            Self::YOUTUBE => "Youtube".to_owned(),
        }
    }
}
