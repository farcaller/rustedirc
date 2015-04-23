#![feature(plugin, collections, convert, core)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)] extern crate hamcrest;

pub mod message;
pub mod context;
