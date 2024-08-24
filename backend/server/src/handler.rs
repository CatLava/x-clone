use axum::{async_trait, extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{error::ApiResult, extractor::DbConnection, AppState};

// wrap this around certain endpoints
#[async_trait]
pub trait PublicApiRequest {
    type Response: IntoResponse;
    async fn process_request(
        self,
        conn: DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

// Use this with routers
pub async fn with_public_handler<'a, Req>(
    conn: DbConnection,
    State(state): State<AppState>,
    Json(payload): Json<Req>,

) -> ApiResult<Req::Response>
where
    Req: PublicApiRequest + Deserialize<'a>,
    {
        payload.process_request(conn, state).await
    }