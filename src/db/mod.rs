//! Contains structs that house data from the .tipsy directory

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::str;

use anyhow::Result;

use crate::consts::TIPSY_DIRECTORY;
use crate::util::get_tool_path;

pub struct Database {
    tools: Vec<Tool>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let status = create_db_if_not_exists()?;
        let mut tools = Vec::new();
        
        // There is no need to read from the database if it is fresh.
        if status == DatabaseStatus::Existed {
            tools = read_tools_from_file()?;
        }

        Ok(Database {
            tools,
        })
    }
    
    pub fn require() -> Self {
        Self::new().expect("Failed to create database")
    }

    pub fn add_tool(&mut self, tool: impl Into<String>) {
        self.tools.push(Tool(tool.into())); 
    }

    pub fn remove_tool(&mut self, tool: impl Into<String>) {
        let tool = tool.into();
        self.tools.retain(|x| *x.0 == tool);
    }

    pub fn get_tool(&self, tool: impl Into<String>) -> Option<&Tool> {
        let tool = tool.into();
        self.tools.iter().find(|&x| x.0 == tool)
    }

    pub fn save(&self) {
        // TODO: Write to file
    }
}

#[derive(PartialEq)]
enum DatabaseStatus {
    Existed,
    Fresh,
}

/// Creates the local database files (if necessary) in the .tipsy directory 
fn create_db_if_not_exists() -> Result<DatabaseStatus> {
    Ok(DatabaseStatus::Fresh)
}

/// Reads in the list of tools from the ~/.tipsy/tools file
fn read_tools_from_file() -> Result<Vec<Tool>> {
    let file = File::open(format!("{}/tools", TIPSY_DIRECTORY))?;
    let reader = BufReader::new(file);
    let mut tools = Vec::new();

    for line in reader.lines() {
        tools.push(Tool(line?.to_string()));
    }

    Ok(tools)
}

pub struct Tool(String);

impl Tool {
    pub fn get_random_tip(&self) -> Result<String> {
        let path = get_tool_path(self.0.as_str())?;
        let format = infer_tool_output_format(&self.0)?;

        let command = process::Command::new(path).arg("-h");
        let output = command.output()?;
        let output = str::from_utf8(output.stdout.as_slice())?;

        let tips = get_available_tips_from_output(output, format);
        Ok(tips.get(0).unwrap().clone())
    }
}

/// Takes the given command output and the format of it and calculates all the potential tips for
/// the command.
fn get_available_tips_from_output(
    output: impl AsRef<str>,
    format: ToolOutputFormat
) -> Vec<String> {
    let lines = output.as_ref().split("\n").collect::<Vec<&str>>();
    let mut tips = Vec::new();
    let mut idx = 0;

    loop {
        if let Some(&line) = lines.get(idx) {
            if line.cmp("USAGE") == Ordering::Equal {
                break;
            }
        }
        idx += 1;
    }

    idx += 1;

    loop {
        if let Some(&line) = lines.get(idx) {
            tips.push(String::from(line));
        } else {
            break;
        }
        idx += 1;
    }

    tips
}

enum ToolOutputFormat {
    /// Basic UNIX format with USAGE, OPTIONS, etc. sections
    Basic,
}

/// Infers what sort of help output the given tool will have
/// Right now, ToolOutputFormat::Basic is the only options, but this api exists to prevent breaking
/// changes happening.
fn infer_tool_output_format(tool: &impl Into<String>) -> Result<ToolOutputFormat> {
    Ok(ToolOutputFormat::Basic)
}
