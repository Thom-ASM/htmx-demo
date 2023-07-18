use sqlx;
use uuid::Uuid;

use crate::models::platform::Platform;

use super::QueryT;

pub fn insert_new_link<'a>(userId: String) -> QueryT<'a> {
    return sqlx::query(
        "INSERT INTO links(linkid,val,platform,userid,active)
    VALUES($1,$2,$3,$4,$5)
    ",
    )
    .bind(Uuid::new_v4().to_string())
    .bind("")
    .bind(Platform::GITHUB)
    .bind(userId)
    .bind(false);
}
