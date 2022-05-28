//! Contains the core logic of the terminal command handlers

use crate::util::get_tool_path;

// There are several Option::unwrap calls in this file on the return value of
// matches.value_of("TOOL"). These are all safe because our clap definitions prevent this option
// from ever being None.

// Tools commands

pub fn handle_tools_add_command(matches: &clap::ArgMatches) {
    let tool = matches.value_of("TOOL").unwrap();
    let path = get_tool_path(tool).expect("Failed to get path for tool");
    println!("{}", path);
}

pub fn handle_tools_remove_command(matches: &clap::ArgMatches) {
    let tool = matches.value_of("TOOL").unwrap();
    println!("TODO: Remove {} from list of tools!", tool);
}

// Tips commands

pub fn handle_tips_get_command(matches: &clap::ArgMatches) {
    let tool = matches.value_of("TOOL").unwrap();
    let path = get_tool_path(tool).expect("Failed to get path for tool");
    println!("TODO: Get tip from {}", path);
}
