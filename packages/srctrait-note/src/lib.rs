#![doc = include_str!("../docs/DOC/1.head.md")]
//! ## Example
//! ```rust
#![doc = include_str!("../examples/example.rs")]
//! ```
#![doc = include_str!("../docs/DOC/3.foot.md")]

pub mod config;
pub mod date;
pub mod dir;
pub mod error;
pub mod kind;
pub mod note;
pub mod template;

pub use self::{config::*, date::*,  dir::*, error::*, kind::*, note::*, template::*};

pub(crate) mod com {
    pub(crate) use srctrait_common_chronox as chrono;
}
