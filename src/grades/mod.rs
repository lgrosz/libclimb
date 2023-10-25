//! Insight to climbing route difficulty.
//!
//! Climb route difficulty is specified through various grading systems. This module allows for use
//! of various grading systems.

use core::fmt::Display;
use std::str::FromStr;

pub mod fontainebleau;
pub mod hueco;

/// A grade is a measure of difficulty
///
/// This measure of difficulty also
/// 1. can be ordered
/// 2. can be shown to the user
/// 3. can be well-defined by a string the user provides
pub trait Grade:
    PartialOrd +
    Display +
    FromStr
{ }
