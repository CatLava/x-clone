use diesel::PgConnection;
use password_hash::PasswordHashString;
use uchat_domain::{ids::UserId, Username};
use diesel::prelude::*;

use crate::QueryError;

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