#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate anyhow;

mod config;
mod error;

mod protocol;

mod content;
mod render;
mod utils;

use config::Opts;

pub const ART_PATH: &str = "/tmp/nvim_render/";

use nvim_oxi as oxi;
use oxi::{
    api::{self, opts::*, types::*, Error},
    Dictionary, Function,
};

fn setup(cmd_opts: Opts) -> Result<(), Error> {
    todo!()
}

// fn autocmd() -> Result<(), Error> {
//     // let opts: CreateAutocmdOpts =
//
//     // oxi::api::create_autocmd(, opts)
// }

#[oxi::plugin]
fn render() -> oxi::Result<Dictionary> {
    todo!()
    // Ok(Dictionary::from_iter([("setup", Function::from_fn(setup))]))
}
