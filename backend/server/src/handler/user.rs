use chrono::{Duration, Utc};

use axum::{async_trait, Json};
use hyper::StatusCode;
use tracing::info;
use uchat_endpoint::user::{endpoint::{CreateUser, CreateUserOk, Login, LoginOk}, types::PublicUserProfile};
use uchat_query::{session::{self, Session}, user::User, AsyncConnection};
use uchat_domain::{ids::*, user::DisplayName};

use crate::{error::ApiResult, extractor::{DbConnection, UserSession}, AppState};

use super::PublicApiRequest;

#[derive(Clone)]
pub struct SessionSignature(String);


pub fn to_public(
    user: User,
) -> ApiResult<PublicUserProfile> {
    Ok(PublicUserProfile {
            id: user.id,
            display_name: user.display_name.and_then(|name| DisplayName::new(name).ok()),
            handle: user.handle,
            profile_image: None,
            created_at: user.created_at,
            am_following: false,
        }
    )
}
pub fn new_session(
    state: &AppState,
    conn: &mut uchat_query::AsyncConnection,
    user_id: UserId

) -> ApiResult<(Session, SessionSignature, Duration)> {
    let fingerprint = serde_json::json!({});
    let session_duration = Duration::weeks(3);
    let session = uchat_query::session::new(
        conn,
        user_id,
        session_duration,
        fingerprint.into(),
    )?;

    let mut rng = state.rng.clone();
    let signature = state.signing_keys.sign(
        &mut rng,
        session.id.as_uuid().as_bytes()
    );
    // convert from raw bytes to base64 for user
    let signature = uchat_crypto::encode_base64(signature);
    Ok((session, SessionSignature(signature), session_duration))
    
}
#[async_trait]
impl PublicApiRequest for CreateUser {
    // Tuple with status code and data can be sent as a response
    type Response = (StatusCode, Json<CreateUserOk> );

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = uchat_crypto::hash_password(&self.password)?;
        let user_id = uchat_query::user::new(
            &mut conn,
            password_hash,
            &self.username
        )?;

        let (session, signature, session_duration) = new_session(&state, &mut conn, user_id)?;

        info!(username = self.username.as_ref(), "new user created");
        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                user_id,
                username: self.username,
                session_id: session.id,
                session_signature: signature.0,
                session_expires: Utc::now() +session_duration,
            })
        ))
    }
}

#[async_trait]
impl PublicApiRequest for Login {
    // Tuple with status code and data can be sent as a response
    type Response = (StatusCode, Json<LoginOk> );

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        // any log after this will contain this user name
        let _span = tracing::span!(tracing::Level::INFO, "logging in",
            user = %self.username.as_ref())
            .entered();
        // verify password take hash from db and compare to request
        let hash = uchat_query::user::get_password_hash(
            &mut conn,
            &self.username)?;
        let hash = uchat_crypto::password::deserialize_hash(&hash)?;

        uchat_crypto::verify_password(self.password, &hash)?;

        let user = uchat_query::user::find(&mut conn, &self.username)?;

        let (session, signature, session_duration) = new_session(&state, &mut conn, user.id)?;

        Ok((
            StatusCode::OK,
            Json(LoginOk {
                session_id: session.id,
                session_expires: Utc::now() + session_duration,
                session_signature: signature.0,
                display_name: user.display_name,
                email: user.email,
                profile_image: None,
                user_id: user.id
            })
        ))
    }
}