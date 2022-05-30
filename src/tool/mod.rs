//! Wraps logic for interfacing with command line tools.

use std::cmp::Ordering;
use std::process;
use std::str;

use anyhow::Result;

use crate::util::get_tool_path;

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
