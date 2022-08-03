//! Copyright (c) 2022 MASSA LABS <info@massa.net>

//! Pool of operation and endorsements waiting to be included in a block

#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]
#![feature(map_first_last)]

mod controller_impl;
mod endorsement_pool;
mod operation_pool;
mod run;
mod types;

#[cfg(test)]
mod tests;
