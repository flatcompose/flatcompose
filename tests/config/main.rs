//! The `config` block

mod bind;
mod boot;
mod capsule;
mod config;
mod desktop;
mod env;
mod options;
mod overlay;
mod perms;
mod sandbox;
mod unshare;

#[path = "../harness/common.rs"]
mod common;

#[path = "../harness/flatimage.rs"]
mod flatimage;

#[path = "../harness/mock/mod.rs"]
mod mock;
