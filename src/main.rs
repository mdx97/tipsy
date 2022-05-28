//! Command line tool to get tips about your other favorite command line tools

mod cli;
mod util;

use crate::cli::{build_command, handle_command};

fn main() {
    let matches = build_command().get_matches();
    handle_command(&matches);
}

