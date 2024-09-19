use chrono::{Duration, Utc};

use axum::{async_trait, Json};
use hyper::StatusCode;
use tracing::info;
use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk};
use uchat_query::session::{self, Session};
use uchat_domain::ids::*;

use crate::{error::ApiResult, extractor::{DbConnection, UserSession}, AppState};

use super::AuthorizedApiRequest;


#[async_trait]
impl AuthorizedApiRequest for NewPost {
    // Tuple with status code and data can be sent as a response
    type Response = (StatusCode, Json<NewPostOk> );

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let post = Post::new(session.user_id, self.content, self.options)?;

        let post_id = uchat_query::post::new(&mut conn, post)?;

        Ok((StatusCode::Ok, Json(NewPostOk { post_id })))
    }
}