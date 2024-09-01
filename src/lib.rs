#![doc = include_str!("../README.md")]
//TODO
// #![doc(html_logo_url = "")]
#![warn(missing_docs)]
#![cfg_attr(all(test, feature = "benchmarks"), feature(test))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(all(test, feature = "benchmarks"))]
extern crate test;

pub use crate::prelude::*;

mod animation;
mod buffer;
#[cfg(feature = "rayon")]
mod buffer_par;
mod codecs;
mod dynimage;
mod error;
mod flat;
mod image;
mod image_reader;
mod prelude;
mod rect;
mod utils;
