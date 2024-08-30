use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_query::DieselError;

// storing browser information and computer information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct Fingerprint(serde_json::Value);

impl From<serde_json::Value> for Finderprint {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Queryable, Insertable)]
#[diesel(table_name = schema::web)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub finerprint: Fringerprint,
}

pub fn new(
    conn: &mut PgConnection,
    user_id: UserId,
    duration: chrono::Duration,
    fingerprint: Fingerprint,
) -> Result<Session, DieselError> {
    let uid = user_id;
    let new_session = Session {
        id: SessionId::new(),
        user_id: uid,
        expires_at: Utc::now() + duration,
        created_at: Utc::now(),
        fingerprint: finderprint,

    };
    {
        use crate::schema::web;
        diesel::insert_into(web::table)
            .values(&new_session)
    }


}