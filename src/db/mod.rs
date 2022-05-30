//! Controls the persistence of data related to tipsy.

use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use anyhow::Result;

use crate::tool::Tool;
use crate::util::tipsy_path;

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
