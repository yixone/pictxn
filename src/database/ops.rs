use crate::{
    domains::{
        cards::ops::AbstractCards, content_sources::ops::AbstractContentSources,
        files::ops::AbstractFiles,
    },
    scout::external_content::ops::AbstractExternalContent,
};

pub trait AbstractDatabase:
    Send + Sync + AbstractCards + AbstractFiles + AbstractContentSources + AbstractExternalContent
{
}
