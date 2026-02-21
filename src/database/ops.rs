use crate::domains::{
    cards::ops::AbstractCards, content_sources::ops::AbstractContentSources,
    files::ops::AbstractFiles,
};

pub trait AbstractDatabase:
    Send + Sync + AbstractCards + AbstractFiles + AbstractContentSources
{
}
