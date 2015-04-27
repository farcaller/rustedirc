#![feature(plugin, collections, convert, core)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)] extern crate hamcrest;
extern crate core;

pub mod message;
// pub mod context;
pub mod server;
#[cfg(test)] mod server_test;
pub mod uidgen;
