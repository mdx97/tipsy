//! Controls the persistence of data related to tipsy.

use std::cmp::Ordering;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process;
use std::str;

use anyhow::Result;

use crate::util::{get_tool_path, tipsy_path};

/// Controls data persistence and loading from disk (namely, persistent data in the .tipsy
/// directory.)
pub struct Database {
    tools: Vec<Tool>,
}

impl Database {
    /// Attempts to create a new instance of `Database` and syncs the initial
    /// state between this struct and the data on disk.
    pub fn new() -> Result<Self> {
        let status = create_db_if_not_exists()?;
        let mut tools = Vec::new();

        // There is no need to read from the database if it is fresh.
        if status == DatabaseStatus::Existed {
            tools = read_tools_from_file()?;
        }

        Ok(Database { tools })
    }

    /// Attempts to create a new instance of `Database` and panics on failure.
    pub fn require() -> Self {
        Self::new().expect("Failed to create database")
    }

    /// Adds a new tool to the tools field.
    pub fn add_tool(&mut self, tool: impl Into<String>) {
        self.tools.push(Tool(tool.into()));
    }

    /// Removes a tool from the tools field.
    pub fn remove_tool(&mut self, tool: impl Into<String>) {
        let tool = tool.into();
        self.tools.retain(|x| *x.0 == tool);
    }

    /// Attempts to find a tool from the tools field with the given name.
    pub fn get_tool(&self, tool: impl Into<String>) -> Option<&Tool> {
        let tool = tool.into();
        self.tools.iter().find(|&x| x.0 == tool)
    }

    /// Returns the contents of the tools field.
    pub fn get_all_tools(&self) -> &Vec<Tool> {
        &self.tools
    }

    /// Returns a random entry from the tools field.
    pub fn get_random_tool(&self) -> Option<&Tool> {
        self.tools.get(0)
    }

    /// Writes all persistent data to disk (calls to methods such as Database::add_tool or
    /// Database::remove_tool are not persisted automatically and require a Database::save call in
    /// order to write these changes to disk.)
    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(tipsy_path().join("tools"))?;

        for tool in &self.tools {
            file.write_all(format!("{}\n", tool.0).as_bytes())?;
        }

        Ok(())
    }
}

/// Signals what state the database was in before this program run.
#[derive(PartialEq)]
enum DatabaseStatus {
    Existed,
    Fresh,
}

/// Creates the .tipsy directory (if necessary).
fn create_db_if_not_exists() -> Result<DatabaseStatus> {
    let tipsy_path = tipsy_path();

    if tipsy_path.exists() {
        return Ok(DatabaseStatus::Existed);
    }

    create_dir(&tipsy_path)?;
    File::create(tipsy_path.join("tools"))?;

    Ok(DatabaseStatus::Fresh)
}

/// Reads in the .tipsy/tools file.
fn read_tools_from_file() -> Result<Vec<Tool>> {
    let file = File::open(tipsy_path().join("tools"))?;
    let reader = BufReader::new(file);
    let mut tools = Vec::new();

    for line in reader.lines() {
        tools.push(Tool(line?.to_string()));
    }

    Ok(tools)
}

/// Wrapper type for a tool name.
pub struct Tool(pub String);

impl Tool {
    /// Gets a random tip from this program's help text.
    pub fn get_random_tip(&self) -> Result<String> {
        let path = get_tool_path(self.0.as_str())?;
        let format = infer_tool_output_format(&self.0)?;

        let mut command = process::Command::new(path);
        command.arg("-h");

        let output = command.output()?;
        let output = str::from_utf8(output.stdout.as_slice())?;

        let tips = get_available_tips_from_output(output, format);
        Ok(get_random_tip(&tips).to_string())
    }
}

/// Takes the given help output and the format of it and calculates all the potential tips for
/// the command.
fn get_available_tips_from_output(
    output: impl AsRef<str>,
    _format: ToolOutputFormat,
) -> Vec<String> {
    let lines = output.as_ref().split("\n").collect::<Vec<&str>>();
    let mut tips = Vec::new();
    let mut idx = 0;

    // Skip lines until we find the section with command line options.
    loop {
        if idx >= lines.len() {
            return Vec::new();
        }

        if let Some(&line) = lines.get(idx) {
            if line.trim().cmp("OPTIONS:") == Ordering::Equal {
                break;
            }
        }
        idx += 1;
    }

    idx += 1;

    // Take each command line option as a tip.
    for idx in idx..lines.len() {
        if let Some(&line) = lines.get(idx) {
            tips.push(String::from(line));
        }
    }

    tips
}

/// Returns a formatted, random tip from the given list of tips.
fn get_random_tip(tips: &Vec<String>) -> String {
    tips.get(0)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Represents how to interpret the help output for a program.
enum ToolOutputFormat {
    /// Basic UNIX format with USAGE, OPTIONS, etc. sections
    Basic,
}

/// Infers what sort of help output the given tool will have. Right now, ToolOutputFormat::Basic
/// is the only options, but this api exists to prevent breaking changes happening.
fn infer_tool_output_format(_tool: &impl Into<String>) -> Result<ToolOutputFormat> {
    Ok(ToolOutputFormat::Basic)
}
