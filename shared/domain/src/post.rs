use serde::{Deserialize, Serialize};
use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(not_empty,  len_char_max=30), derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq))]
// #[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Headline(String);

impl UserFacingError for HeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadlineError::LenCharMaxViolated => "headline too long, no more than 30",
            HeadlineError::NotEmptyViolated => "headline missing"
        }
    }
}

#[nutype(validate(not_empty,  len_char_max=100), derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq))]
// #[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message(String);

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::LenCharMaxViolated => "message too long, no more than 100",
            MessageError::NotEmptyViolated => "message missing"
        }
    }
}
