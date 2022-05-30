//! Contains the core logic of the terminal command handlers

use crate::db::Database;
use crate::util::get_tool_path;

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

/// Shows all tools currently saved in the database.
pub fn handle_tools_list_command(_matches: &clap::ArgMatches) {
    let database = Database::require();
    let tools = database.get_all_tools();

    for tool in tools {
        println!("{}", tool.0);
    }
}

// Tips commands

/// Prints out a tip for a given tool (or a random one, if one was not specified.)
pub fn handle_tips_get_command(matches: &clap::ArgMatches) {
    let database = Database::require();

    let tool = match matches.value_of("TOOL") {
        Some(tool) => database.get_tool(tool),
        None => database.get_random_tool(),
    };

    if let Some(tool) = tool {
        println!(
            "[{}] {}",
            tool.0,
            tool.get_random_tip().expect("Could not get tip")
        );
    }
}
