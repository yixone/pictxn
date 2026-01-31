use crate::models::domains::{files::FileId, users::UserId};

pub struct ProfileDomain {
    id: UserId,

    display_name: String,

    avatar_id: Option<FileId>,
    banner_id: Option<FileId>,
}

impl ProfileDomain {
    pub fn new(id: UserId, display_name: String) -> Self {
        ProfileDomain {
            id,
            display_name,
            avatar_id: None,
            banner_id: None,
        }
    }
}
