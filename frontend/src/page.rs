pub mod register;
pub mod login;

pub use register::Register;
pub use login::Login;
pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
    pub const ACCOUNT_LOGIN: &str = "/account/login";

}