//! Functions that handle building the clap app

use clap::{arg, command};

// Top level command

pub fn build_command<'a>() -> clap::Command<'a> {
    command!()
        .subcommand(build_tools_subcommand())
        .subcommand(build_tips_subcommand())
        .subcommand_required(true)
}

// Tools subcommand

fn build_tools_subcommand<'a>() -> clap::Command<'a> {
    clap::Command::new("tools")
        .about("Manage tools you are learning about")
        .subcommand(build_tools_add_subcommand())
        .subcommand(build_tools_remove_subcommand())
        .subcommand_required(true)
}

fn build_tools_add_subcommand<'a>() -> clap::Command<'a> {
    clap::Command::new("add")
        .about("Add a tool you want to learn about")
        .arg(arg!([TOOL]))
        .arg_required_else_help(true)
}

fn build_tools_remove_subcommand<'a>() -> clap::Command<'a> {
    clap::Command::new("remove")
        .about("Remove a tool you no longer want to learn about")
        .arg(arg!([TOOL]))
        .arg_required_else_help(true)
}

// Tips subcommand

fn build_tips_subcommand<'a>() -> clap::Command<'a> {
    clap::Command::new("tips")
        .about("Manage tips")
        .subcommand(build_tips_get_subcommand())
        .subcommand_required(true)
}

fn build_tips_get_subcommand<'a>() -> clap::Command<'a> {
    clap::Command::new("get")
        .about("Get a new tip")
        .arg(arg!([TOOL]))
        .arg_required_else_help(true)
}
