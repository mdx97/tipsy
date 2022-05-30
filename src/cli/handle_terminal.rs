//! Contains the core logic of the terminal command handlers

use crate::db::Database;
use crate::util::get_tool_path;

// There are several Option::unwrap calls in this file on the return value of
// matches.value_of("TOOL"). These are all safe because our clap definitions prevent this option
// from ever being None.

// Tools commands

/// Adds a new tool to the database.
pub fn handle_tools_add_command(matches: &clap::ArgMatches) {
    let tool = matches.value_of("TOOL").unwrap();

    // We will recompute the tool path on each run, so we just want to verify that the tool is
    // installed on path for this run.
    get_tool_path(tool).expect("Failed to get path for tool");

    let mut database = Database::require();
    database.add_tool(tool);
    database.save().expect("Failed to save for tool addition");
}

/// Attempts to remove a tool from the database.
pub fn handle_tools_remove_command(matches: &clap::ArgMatches) {
    let tool = matches.value_of("TOOL").unwrap();
    let mut database = Database::require();
    database.remove_tool(tool);
    database.save().expect("Failed to save for tool removal");
}

// Tips commands

/// Get a tip for a given tool (or, eventually a random one.)
pub fn handle_tips_get_command(matches: &clap::ArgMatches) {
    let tool = matches.value_of("TOOL").unwrap();
    let database = Database::require();

    if let Some(tool) = database.get_tool(tool) {
        println!("{}", tool.get_random_tip().expect("Could not get tip"));
    }
}
