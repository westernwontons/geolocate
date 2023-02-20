#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

use clap_complete::{generate_to, shells};
use geolocate_lib::cli::build_cli;
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    // let outdir = match env::var_os("OUT_DIR") {
    //     Some(outdir) => outdir,
    //     None => return Ok(()),
    // };

    // let mut cmd = build_cli();
    // let path = generate_to(shells::Zsh, &mut cmd, "geolocate", outdir)?;

    // println!("cargo:warning=completion file generated to {:?}", path);

    Ok(())
}
