use chrono::{DateTime, Utc};
use diesel::{result::Error, PgConnection};
use password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use uchat_domain::ids::PostId;
use uchat_domain::{ids::UserId, Username};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{schema, QueryError};
use crate::DieselError;

// This is a content wrap to convert it to JSON
#[derive(Clone, Debug, DieselNewType, Serialize, Deserialize)]
pub struct Content(pub serde_json::Value);

#[derive(Debug, Queryable, Selectable, Insertable )]
#[diesel(table_name = schema::posts)]
pub struct Post {
    pub id : PostId,
    pub user_id : UserId,
    pub content : Content,
    pub time_posted : DateTime<Utc>,
    pub direct_message_to : Option<UserId>,
    pub reply_to : Option<PostId>,
    pub created_at : DateTime<Utc>,
}

impl Post {
    pub fn new (
        posted_by: UserId,
        content: uchat_endpoint::post::types::Content,
        options: uchat_endpoint::post::types::NewPostOptions,
    ) -> Result<Self, serde_json::Error> {
        Ok( Self{
            id : Uuid::new_v4().into(),
            user_id : posted_by,
            content : Content(serde_json::to_value(content)?),
            time_posted : options.time_posted,
            direct_message_to : options.direct_message_to,
            reply_to : options.reply_to,
            created_at : Utc::now(), 
        })
    }
}

pub fn new(conn: &mut PgConnection, post: Post) -> Result<PostId, DieselError> {
    conn.transaction::<PostId, DieselError, _>(|conn| {
        // transaction allows multiple queries, numerous queries
        diesel::insert_into(schema::posts::table)
            .values(&post)
            .execute(conn)?;
        Ok(post.id)
    })
}