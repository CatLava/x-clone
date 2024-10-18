pub mod register;
pub mod login;
pub mod home;
pub mod new_post;
pub mod trending;

pub use register::Register;
pub use login::Login;
pub use home::Home;
pub use new_post::*;
pub use route::*;
pub use trending::Trending;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
    pub const ACCOUNT_LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
    pub const POSTS_TRENDING: &str = "/posts/trending";


}