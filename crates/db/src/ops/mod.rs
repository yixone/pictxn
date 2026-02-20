pub mod cards;
pub mod files;

pub trait AbstractBase: cards::AbstractCards + files::AbstractFiles {}
