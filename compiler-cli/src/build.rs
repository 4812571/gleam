use std::time::Instant;

use gleam_core::{
    build::{Options, Package, ProjectCompiler},
    Result,
};

use crate::{cli, fs};

pub fn main(options: &Options) -> Result<Package> {
    let manifest = crate::dependencies::download(None)?;

    let root_config = crate::config::root_config()?;
    let telemetry = Box::new(cli::Reporter::new());
    let io = fs::ProjectIO::new();
    let start = Instant::now();

    tracing::info!("Compiling packages");
    let compiled =
        ProjectCompiler::new(root_config, options, &manifest.packages, telemetry, io).compile()?;

    cli::print_compiled(start.elapsed());
    Ok(compiled)
}
