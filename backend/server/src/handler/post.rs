use chrono::{Duration, Utc};

use axum::{async_trait, Json};
use hyper::StatusCode;
use uchat_endpoint::{post::{endpoint::{Bookmark, BookmarkOk, NewPost, NewPostOk, TrendingPosts, TrendingPostsOk}, types::{LikeStatus, PublicPost}}, RequestFailed};
use uchat_endpoint::post::types::BookmarkAction;
use uchat_query::{post::Post, session::{self, Session}, AsyncConnection};
use uchat_domain::{ids::*, Username};

use crate::{error::{ApiError, ApiResult}, extractor::{DbConnection, UserSession}, AppState};

use super::AuthorizedApiRequest;


pub fn to_public(
    conn: &mut AsyncConnection,
    post: Post,
    session: Option<&UserSession>
) -> ApiResult<PublicPost> {
    use uchat_query::post as query_post;
    use uchat_query::user as query_user;

    if let Ok(mut content) = serde_json::from_value(post.content.0) {
        // convert DB post to a public post 
        Ok(PublicPost {
            id: post.id,
            by_user: {
                let profile = query_user::get(conn, post.user_id)?;
                super::user::to_public(profile)?
            },
            content: content,
            time_posted: post.time_posted,
            reply_to: {
                match post.reply_to {
                    Some(other_post_id) => {
                        let original_post = query_post::get(conn, other_post_id)?;
                        let original_user = query_user::get(conn, original_post.user_id)?;
                        Some((
                            Username::new(original_user.handle).unwrap(),
                            original_user.id,
                            other_post_id,
                        ))
                    },
                    None => None,
                }
            },
            like_status: LikeStatus::NoReaction,
            bookmarked: {
                match session {
                    Some(session) => query_post::get_bookmark(conn,
                         session.user_id, post.id)?,
                    None => false,
                }
            },
            boosted: false,
            likes: 0,
            dislikes: 0,
            boosts: 0,
        })
    } else {
        Err(ApiError {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR),
            err: color_eyre::Report::new(RequestFailed {
                msg: "invalid post data".to_string(),
            })
        })
    }
}

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

        Ok((StatusCode::OK, Json(NewPostOk { post_id })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for TrendingPosts {
    // Tuple with status code and data can be sent as a response
    type Response = (StatusCode, Json<TrendingPostsOk> );

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        use uchat_query::post as query_post;

        let mut posts = vec![];

        for post in query_post::get_trending(&mut conn)? {
            let post_id = post.id;
            match to_public(&mut conn, post, Some(&session)) {
                Ok(post) => posts.push(post),
                Err(e) => {
                    tracing::error!(err = %e.err, post_id = ?post_id, "post contains invalid data")
                }
            }
        }

        Ok((StatusCode::OK, Json(TrendingPostsOk { posts })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for Bookmark {
    // Tuple with status code and data can be sent as a response
    type Response = (StatusCode, Json<BookmarkOk> );

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        match self.action {
            BookmarkAction::Add => {
                uchat_query::post::bookmark(&mut conn, session.user_id, self.post_id);
            }
            BookmarkAction::Remove => {
                uchat_query::post::delete_bookmark(&mut conn, session.user_id, self.post_id);
            }
        }

        Ok((
            StatusCode::OK,
            Json(BookmarkOk {
                status: self.action,
            }),
        ))
    }
}