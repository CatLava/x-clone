use serde::Serialize;

#[derive(
    Clone, Copy, Debug, Eq,
    Hash, serde::Serialize, serde::Deserialize,
    PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "query", derive(DieselNewType))]
pub struct UserId(uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn into_inner(self) -> uuid::Uuid {
        self.0
    }

    pub fn as_uuid()
}