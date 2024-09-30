use serde::{Deserialize, Serialize};
use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(not_empty, len_char_min=3, len_char_max=30), derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq))]
// #[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Username(String);

impl UserFacingError for UsernameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            UsernameError::LenCharMinViolated => "User name too short, must be 3",
            UsernameError::LenCharMaxViolated => "Username too long, no more than 20",
            UsernameError::NotEmptyViolated => "Username missing"
        }
    }
}

#[nutype(validate(not_empty, len_char_min=8), derive(AsRef, Clone, Serialize, Deserialize, PartialEq))]
pub struct Password(String);

impl UserFacingError for PasswordError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PasswordError::LenCharMinViolated => "Password too short, must be at least 8",
            PasswordError::NotEmptyViolated => "Password missing"
        }
    }
}


#[nutype(validate(len_char_max=30), derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq))]
// #[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DisplayName(String);

impl DisplayName {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for DisplayNameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            DisplayNameError::LenCharMaxViolated => "Display name too long",

        }
    }
}