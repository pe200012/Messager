#![feature(vec_remove_item)]
#[macro_use]
extern crate serde_derive;

pub mod account;
pub mod client;
pub mod my_channel;

#[cfg(test)]
mod test;
