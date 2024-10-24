use chrono::{DateTime, Utc};
use diesel::{result::Error, PgConnection};
use password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use uchat_domain::{ids::UserId, Username};
use diesel::prelude::*;

use crate::QueryError;
use crate::DieselError;

pub fn new<T: AsRef<str>>(
    conn: &mut PgConnection,
    hash: PasswordHashString,
    handle: T
) -> Result<UserId, QueryError> {
    use crate::schema::users::{self, columns};

    let user_id = UserId::new();

    diesel::insert_into(users::table)
        .values((
            columns::id.eq(user_id),
            columns::password_hash.eq(hash.as_str()),
            columns::handle.eq(handle.as_ref()),
        ))
        .execute(conn)?;

    Ok(user_id)
}

pub fn get_password_hash(
    conn: &mut PgConnection,
    username: &Username,
) -> Result<String, QueryError> {
    use crate::schema::users::dsl::*;
    Ok(users.filter(handle.eq(username.as_ref()))
        .select(password_hash)
        .get_result(conn)?
    )
}

#[derive(Deserialize, Queryable)]
pub struct User {
    pub id: UserId,
    pub email: Option<String>,
    pub email_confirmed: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub handle: String, 
    pub created_at: DateTime<Utc>,
    pub profile_image: Option<String>,
}

pub fn get(
    conn: &mut PgConnection,
    user_id: UserId
) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.filter(id.eq(user_id)).get_result(conn)
}

pub fn find(
    conn: &mut PgConnection,
    username: &Username
) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.filter(handle.eq(username.as_ref())).get_result(conn)
}