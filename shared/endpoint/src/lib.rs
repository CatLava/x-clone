use serde::{Deserialize, Serialize};

pub mod user;
pub mod post;

pub trait Endpoint {
    const URL: &'static str;
    fn url(&self) -> &'static str {
        Self::URL
    }
}

macro_rules! route {
    ($url:literal => $request_type:ty) => {
        impl Endpoint for $request_type {
            const URL: &'static str = $url;
        }
    };
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String
}

//public
route!("/account/login" => user::endpoint::Login);
route!("/account/create" => user::endpoint::CreateUser);


// private authorize route
route!("/post/new" => post::endpoint::NewPost);
route!("/post/bookmark" => post::endpoint::Bookmark);
route!("/post/boost" => post::endpoint::Boost);
route!("/posts/trending" => post::endpoint::TrendingPosts);
route!("/posts/react" => post::endpoint::React);


