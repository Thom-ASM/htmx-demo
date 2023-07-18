use super::QueryT;

pub fn get_all_links_for_user<'a>(userId: String) -> QueryT<'a> {
    return sqlx::query(
        "SELECT * FROM links WHERE userid = $1
    
    ORDER  BY ctid DESC;
    
    ",
    )
    .bind(userId);
}
