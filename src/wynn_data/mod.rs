//! A module for creating wynncraft builds
//! 
//! TODO features: 
//! - Redo how ability tree is parsed
//! - Add major-id support
//! - Maybe do something with crafted items
pub mod items;
pub mod builder;
pub mod sets;
pub mod atree;
mod general;
pub use general::*;
