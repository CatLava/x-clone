use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension, Json, RequestPartsExt};
use hyper::{header, StatusCode};
use tracing::info;
use uchat_domain::ids::{SessionId, UserId};
use uchat_endpoint::RequestFailed;
use uchat_query::OwnedAsyncConnection;
use std::str::FromStr;

use crate::AppState;

pub struct DbConnection(pub OwnedAsyncConnection);
// Whenever extractor is written need a wrapper structure

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where S: Send + Sync,
{
    // auto convert this into a response
    type Rejection = (StatusCode, &'static str);

    // This is puled from the docs
    async fn from_request_parts(
        parts: &mut Parts,
        _: &S) -> Result<Self, Self::Rejection> {
            let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();
            let connection = state.db_pool.get_owned().await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to conect to database"
                )
            })?;
            Ok(Self(connection))
        }
}

// need to validate a user session
#[derive(Copy, Clone)]
pub struct UserSession {
    pub user_id: UserId,
    pub session_id: SessionId
}
#[async_trait]
impl<S> FromRequestParts<S> for UserSession
where S: Send + Sync,
{
    // auto convert this into a response
    type Rejection = (StatusCode, Json<RequestFailed>);

    // This is puled from the docs
    async fn from_request_parts(
        parts: &mut Parts,
        _: &S) -> Result<Self, Self::Rejection> {  
            let unauthorized = || {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(RequestFailed {
                        msg: "unauthorized".into()
                    })
                )
            };

            let DbConnection(mut conn) = parts.extract::<DbConnection>().await.unwrap();
            let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();

            let cookies = parts
                .headers
                .get(header::COOKIE)
                .and_then(|header| header.to_str().ok()) //makes it an option
                .ok_or_else(unauthorized)?; // turns it back into a result
            let session_id = uchat_cookie::get_from_str(cookies, uchat_cookie::SESSION_ID)
                .and_then(|id| SessionId::from_str(id).ok())
                .ok_or_else(unauthorized)?;
            let session_signature = uchat_cookie::get_from_str(cookies, uchat_cookie::SESSION_SIGNATURE)
                .and_then(|sig| uchat_crypto::decode_base64(sig).ok())
                .and_then(|sig| uchat_crypto::sign::signature_from_bytes(sig).ok())
                .ok_or_else(unauthorized)?;

            state
                .signing_keys
                .verify(session_id.as_uuid().as_bytes(), session_signature)
                .map_err(|_| unauthorized())?;

            // session is valid thus far, can pull it from DB    
            let session = uchat_query::session::get(&mut conn, session_id)
                .ok()
                .flatten()
                .ok_or_else(unauthorized)?;

            info!(
                user_id = session.user_id.into_inner().to_string(),
                "user logged in"
            );

            Ok(Self {
                user_id: session.user_id,
                session_id: session.id

            })
          }
}