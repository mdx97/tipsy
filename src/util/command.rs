//! Contains utility functions that interact with the command line

use std::process;

use anyhow::Result;

/// Attempts to get the path to the tool with the given identifier using the `which` command.
/// For example, if you have `rg` installed on your PATH at /opt/homebrew/bin, this function
/// should return "/opt/homebrew/bin/rg".
pub fn get_tool_path<'a>(tool: impl Into<&'a str>) -> Result<String> {
    let mut command = process::Command::new("which");
    command.arg(tool.into());

    let output = command.output()?;
    let path = std::str::from_utf8(output.stdout.as_slice())?;

    Ok(String::from(path))
}
