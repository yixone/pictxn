pub mod cards;
pub mod content_source;
pub mod files;

pub trait AbstractBase:
    cards::AbstractCards + files::AbstractFiles + content_source::AbstractContentSource
{
}
