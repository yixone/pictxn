/// Unique UUID identifier for the user
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub struct UserId(pub uuid::Uuid);

/// Unique numeric identifier of the card
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub struct CardId(pub i64);
