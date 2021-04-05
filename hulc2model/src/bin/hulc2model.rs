// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

#[cfg(not(windows))]
mod cli;
#[cfg(not(windows))]
use anyhow::Result;

#[cfg(not(windows))]
fn main() -> Result<()> {
    cli::cli_main()
}

// TODO: investigar iui https://docs.rs/crate/iui/0.3.0
#[cfg(windows)]
mod wingui;

#[cfg(windows)]
fn main() {
    wingui::run_wingui();
}
