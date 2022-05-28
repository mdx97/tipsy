mod build;
mod handle_internal;
mod handle_terminal;

pub use build::build_command;
pub use handle_internal::handle_command;

use handle_terminal::*;
