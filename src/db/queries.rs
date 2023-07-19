use sqlx::{PgExecutor, Row};

use crate::models::{link::Link, platform::Platform};

pub async fn get_all_links_for_user<'a>(
    userId: String,
    executor: impl PgExecutor<'_>,
) -> Result<Vec<Link>, String> {
    let query_res = sqlx::query(
        "SELECT * FROM links WHERE userid = $1
    
    ORDER  BY ctid DESC;
    
    ",
    )
    .bind(&userId)
    .fetch_all(executor)
    .await;

    match query_res {
        Ok(res) => {
            return Ok(res
                .iter()
                .map(|row| Link {
                    linkid: row.get("linkid"),
                    val: row.get("val"),
                    userid: row.get("userid"),
                    platform: Platform::GITHUB,
                    active: row.get("active"),
                })
                .collect::<Vec<Link>>());
        }
        Err(_) => return Err(format!("Failed to fetch links for user: ${}", &userId)),
    }
}
