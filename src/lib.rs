#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate anyhow;

mod config;
mod render;
mod protocol;
mod utils;
use config::Opts;

use nvim_oxi as oxi;
use oxi::{
    api::{self, opts::*, types::*, Error},
    Dictionary, Function,
};

fn setup(cmd_opts: Opts) -> Result<(), Error> {
    todo!()
}


#[oxi::module]
fn render() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([
        ("setup", Function::from_fn(setup)),
    ]))
}
