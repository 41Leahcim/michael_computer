#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_const_for_fn,
    missing_docs
)]
#![allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]

//! This library contains the implementation of gates, circuits, and datatypes used by the computer

pub mod bit;
pub mod byte;
pub mod circuit;
