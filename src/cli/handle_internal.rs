//! Contains functions that pipe data down to terminal command handlers

use super::{
    handle_tips_get_command,
    handle_tools_add_command,
    handle_tools_remove_command,
    handle_tools_list_command
};

pub fn handle_command(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("tools", sub_matches)) => handle_tools_command(sub_matches),
        Some(("tips", sub_matches)) => handle_tips_command(sub_matches),
        _ => unreachable!("Should not be reached due to Command::subcommand_required"),
    }
}

fn handle_tools_command(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("add", sub_matches)) => handle_tools_add_command(sub_matches),
        Some(("remove", sub_matches)) => handle_tools_remove_command(sub_matches),
        Some(("list", sub_matches)) => handle_tools_list_command(sub_matches),
        _ => unreachable!("Should not be reached due to Command::subcommand_required"),
    }
}

fn handle_tips_command(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("get", sub_matches)) => handle_tips_get_command(sub_matches),
        _ => unreachable!("Should not be reached due to Command::subcommand_required"),
    }
}
