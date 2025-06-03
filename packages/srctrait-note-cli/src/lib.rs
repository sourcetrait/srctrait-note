#![doc = include_str!("../docs/DOC/1.head.md")]
#![doc = include_str!("../docs/DOC/3.foot.md")]

mod env;
mod cli;
mod run;

pub use run::run;

pub(crate) use self::{env::*, cli::*};
pub(crate) use srctrait_note as lib;
pub(crate) use anyhow::Context;
