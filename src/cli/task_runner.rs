use crate::common::Value;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Write;
use std::process::Command;

/// A single download specification: fetch a URL to a local path
#[derive(Debug, Clone)]
pub struct DownloadSpec {
    pub url: String,
    pub to: String,
}

/// Represents a single task definition
#[derive(Debug, Clone)]
pub struct TaskDef {
    pub name: String,
    pub cmd: String,
    pub deps: Vec<String>,
    pub desc: Option<String>,
    pub env: HashMap<String, String>,
    /// Working directory — `cd` here before running `cmd`
    pub dir: Option<String>,
    /// Files to download (URL → path) before running `cmd`
    pub downloads: Vec<DownloadSpec>,
    /// If true, suppress "Running: ..." output
    pub quiet: bool,
    /// If true, continue even if `cmd` exits non-zero
    pub ignore_errors: bool,
    /// String to pipe into the command's stdin
    pub stdin: Option<String>,
}

impl TaskDef {
    /// Create a new TaskDef from name and command.
    /// Used for programmatic task construction and in tests.
    #[allow(dead_code)]
    pub fn new(name: &str, cmd: &str, deps: Vec<String>) -> Self {
        TaskDef {
            name: name.to_string(),
            cmd: cmd.to_string(),
            deps,
            desc: None,
            env: HashMap::new(),
            dir: None,
            downloads: Vec::new(),
            quiet: false,
            ignore_errors: false,
            stdin: None,
        }
    }

    /// Parse a TaskDef from a Value (either String or Dict)
    pub fn from_value(name: String, value: &Value) -> Result<Self, TaskError> {
        match value {
            // Simple format: "command string"
            Value::String(cmd) => Ok(TaskDef {
                name,
                cmd: cmd.clone(),
                deps: Vec::new(),
                desc: None,
                env: HashMap::new(),
                dir: None,
                downloads: Vec::new(),
                quiet: false,
                ignore_errors: false,
                stdin: None,
            }),
            // Structured format: { cmd: "...", deps: [...], desc: "..." }
            Value::Dict(dict) => {
                // Extract cmd field (required)
                // Can be either a string or a list of strings (which are joined with &&)
                let cmd = match dict.get("cmd") {
                    Some(Value::String(c)) => c.clone(),
                    Some(Value::List(cmd_list)) => {
                        // List of commands - join with " && "
                        let mut cmd_parts = Vec::new();
                        for cmd_value in cmd_list {
                            match cmd_value {
                                Value::String(c) => cmd_parts.push(c.clone()),
                                _ => {
                                    return Err(TaskError::InvalidTaskDef {
                                        task: name,
                                        reason: "cmd list elements must be strings".to_string(),
                                    })
                                }
                            }
                        }
                        if cmd_parts.is_empty() {
                            return Err(TaskError::InvalidTaskDef {
                                task: name,
                                reason: "cmd list cannot be empty".to_string(),
                            });
                        }
                        cmd_parts.join(" && ")
                    }
                    Some(_) => {
                        return Err(TaskError::InvalidTaskDef {
                            task: name,
                            reason: "cmd must be a string or list of strings".to_string(),
                        })
                    }
                    None => {
                        return Err(TaskError::InvalidTaskDef {
                            task: name,
                            reason: "missing required 'cmd' field".to_string(),
                        })
                    }
                };

                // Extract deps field (optional)
                let mut deps = Vec::new();
                if let Some(Value::List(dep_list)) = dict.get("deps") {
                    for dep_value in dep_list {
                        match dep_value {
                            Value::String(dep) => deps.push(dep.clone()),
                            _ => {
                                return Err(TaskError::InvalidTaskDef {
                                    task: name,
                                    reason: "deps must be a list of strings".to_string(),
                                })
                            }
                        }
                    }
                } else if dict.get("deps").is_some() {
                    return Err(TaskError::InvalidTaskDef {
                        task: name,
                        reason: "deps must be a list".to_string(),
                    });
                }

                // Extract desc field (optional)
                let desc = dict.get("desc").and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                });

                // Extract env field (optional)
                let mut env = HashMap::new();
                if let Some(Value::Dict(env_dict)) = dict.get("env") {
                    for (key, val) in env_dict {
                        if let Value::String(s) = val {
                            env.insert(key.clone(), s.clone());
                        }
                    }
                }

                // Extract dir field (optional) — working directory
                let dir = dict.get("dir").and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                });

                // Extract download field (optional)
                // Accepts a single dict {url, to} or a list of dicts
                let mut downloads = Vec::new();
                match dict.get("download") {
                    Some(Value::Dict(dl_dict)) => {
                        if let (Some(Value::String(url)), Some(Value::String(to))) =
                            (dl_dict.get("url"), dl_dict.get("to"))
                        {
                            downloads.push(DownloadSpec {
                                url: url.clone(),
                                to: to.clone(),
                            });
                        } else {
                            return Err(TaskError::InvalidTaskDef {
                                task: name,
                                reason: "download must have 'url' and 'to' string fields"
                                    .to_string(),
                            });
                        }
                    }
                    Some(Value::List(dl_list)) => {
                        for item in dl_list {
                            match item {
                                Value::Dict(dl_dict) => {
                                    if let (
                                        Some(Value::String(url)),
                                        Some(Value::String(to)),
                                    ) = (dl_dict.get("url"), dl_dict.get("to"))
                                    {
                                        downloads.push(DownloadSpec {
                                            url: url.clone(),
                                            to: to.clone(),
                                        });
                                    } else {
                                        return Err(TaskError::InvalidTaskDef {
                                            task: name,
                                            reason:
                                                "each download entry must have 'url' and 'to' string fields"
                                                    .to_string(),
                                        });
                                    }
                                }
                                _ => {
                                    return Err(TaskError::InvalidTaskDef {
                                        task: name,
                                        reason: "download list entries must be dicts with 'url' and 'to'"
                                            .to_string(),
                                    });
                                }
                            }
                        }
                    }
                    Some(_) => {
                        return Err(TaskError::InvalidTaskDef {
                            task: name,
                            reason: "download must be a dict {url, to} or list of dicts"
                                .to_string(),
                        });
                    }
                    None => {} // no downloads
                }

                // Extract quiet field (optional, default false)
                let quiet = match dict.get("quiet") {
                    Some(Value::Bool(b)) => *b,
                    Some(_) => {
                        return Err(TaskError::InvalidTaskDef {
                            task: name,
                            reason: "quiet must be a boolean".to_string(),
                        });
                    }
                    None => false,
                };

                // Extract ignore_errors field (optional, default false)
                let ignore_errors = match dict.get("ignore_errors") {
                    Some(Value::Bool(b)) => *b,
                    Some(_) => {
                        return Err(TaskError::InvalidTaskDef {
                            task: name,
                            reason: "ignore_errors must be a boolean".to_string(),
                        });
                    }
                    None => false,
                };

                // Extract stdin field (optional) — string to pipe into command
                let stdin = dict.get("stdin").and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                });

                Ok(TaskDef {
                    name,
                    cmd,
                    deps,
                    desc,
                    env,
                    dir,
                    downloads,
                    quiet,
                    ignore_errors,
                    stdin,
                })
            }
            _ => Err(TaskError::InvalidTaskDef {
                task: name,
                reason: "task must be a string or dict with 'cmd' key".to_string(),
            }),
        }
    }
}

/// Execution plan for a set of tasks
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub tasks: Vec<TaskDef>,
    pub order: Vec<String>,
}

impl ExecutionPlan {
    /// Format execution plan for display
    pub fn format(&self) -> String {
        let mut output = String::from("Execution Plan:\n");
        output.push_str("================\n");
        for (idx, task_name) in self.order.iter().enumerate() {
            if let Some(task) = self.tasks.iter().find(|t| &t.name == task_name) {
                output.push_str(&format!("{}. {} (cmd: {})\n", idx + 1, task.name, task.cmd));
                if !task.deps.is_empty() {
                    output.push_str(&format!("   deps: {}\n", task.deps.join(", ")));
                }
            }
        }
        output
    }
}

/// Task runner for executing tasks with dependency resolution
#[derive(Debug)]
pub struct TaskRunner {
    tasks: HashMap<String, TaskDef>,
    executed: HashSet<String>,
}

impl TaskRunner {
    /// Create a new TaskRunner from a task dict
    pub fn new(tasks_dict: HashMap<String, TaskDef>) -> Result<Self, TaskError> {
        // Validate all tasks
        TaskRunner::validate_tasks(&tasks_dict)?;

        Ok(TaskRunner {
            tasks: tasks_dict,
            executed: HashSet::new(),
        })
    }

    /// Validate all task definitions
    fn validate_tasks(tasks: &HashMap<String, TaskDef>) -> Result<(), TaskError> {
        // Check for undefined dependencies
        for (name, task) in tasks {
            for dep in &task.deps {
                if !tasks.contains_key(dep) {
                    // Suggest similar task names for typos
                    let suggestions = TaskRunner::find_similar_names(dep, tasks);
                    let suggestion = if suggestions.is_empty() {
                        None
                    } else {
                        Some(suggestions[0].clone())
                    };
                    return Err(TaskError::UndefinedDependency {
                        task: name.clone(),
                        dep: dep.clone(),
                        suggestion,
                    });
                }
            }
        }

        // Check for cycles
        for task_name in tasks.keys() {
            let mut visited = HashSet::new();
            let mut rec_stack = HashSet::new();
            if TaskRunner::has_cycle(task_name, tasks, &mut visited, &mut rec_stack)? {
                return Err(TaskError::CyclicDependency(vec![task_name.clone()]));
            }
        }

        Ok(())
    }

    /// Detect if there's a cycle in dependencies
    fn has_cycle(
        task_name: &str,
        tasks: &HashMap<String, TaskDef>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<bool, TaskError> {
        visited.insert(task_name.to_string());
        rec_stack.insert(task_name.to_string());

        if let Some(task) = tasks.get(task_name) {
            for dep in &task.deps {
                if !visited.contains(dep) {
                    if TaskRunner::has_cycle(dep, tasks, visited, rec_stack)? {
                        return Ok(true);
                    }
                } else if rec_stack.contains(dep) {
                    return Ok(true);
                }
            }
        }

        rec_stack.remove(task_name);
        Ok(false)
    }

    /// Find similar task names (for typo suggestions)
    fn find_similar_names(query: &str, tasks: &HashMap<String, TaskDef>) -> Vec<String> {
        let mut scores: Vec<(String, usize)> = tasks
            .keys()
            .map(|name| {
                let distance = Self::levenshtein_distance(query, name);
                (name.clone(), distance)
            })
            .collect();

        scores.sort_by_key(|(_name, dist)| *dist);
        scores
            .into_iter()
            .take(3)
            .filter(|(_name, dist)| *dist <= 3)
            .map(|(name, _)| name)
            .collect()
    }

    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(a: &str, b: &str) -> usize {
        let a_len = a.len();
        let b_len = b.len();

        if a_len == 0 {
            return b_len;
        }
        if b_len == 0 {
            return a_len;
        }

        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

        for (i, row) in matrix.iter_mut().enumerate().take(a_len + 1) {
            row[0] = i;
        }
        for (j, val) in matrix[0].iter_mut().enumerate().take(b_len + 1) {
            *val = j;
        }

        for (i, a_char) in a.chars().enumerate() {
            for (j, b_char) in b.chars().enumerate() {
                let cost = if a_char == b_char { 0 } else { 1 };
                matrix[i + 1][j + 1] = std::cmp::min(
                    std::cmp::min(
                        matrix[i][j + 1] + 1, // deletion
                        matrix[i + 1][j] + 1, // insertion
                    ),
                    matrix[i][j] + cost, // substitution
                );
            }
        }

        matrix[a_len][b_len]
    }

    /// Build execution plan for a task
    pub fn build_execution_plan(&mut self, task_name: &str) -> Result<ExecutionPlan, TaskError> {
        if !self.tasks.contains_key(task_name) {
            let suggestions = Self::find_similar_names(task_name, &self.tasks);
            return Err(TaskError::TaskNotFound {
                task: task_name.to_string(),
                suggestions,
            });
        }

        let mut order = Vec::new();
        let mut visited = HashSet::new();
        self.topological_sort(task_name, &mut order, &mut visited)?;

        let tasks = self.tasks.values().cloned().collect();

        Ok(ExecutionPlan { tasks, order })
    }

    /// Topological sort for dependency resolution
    fn topological_sort(
        &self,
        task_name: &str,
        order: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), TaskError> {
        if visited.contains(task_name) {
            return Ok(());
        }

        visited.insert(task_name.to_string());

        if let Some(task) = self.tasks.get(task_name) {
            for dep in &task.deps {
                self.topological_sort(dep, order, visited)?;
            }
        }

        order.push(task_name.to_string());
        Ok(())
    }

    /// Run a task and its dependencies
    pub fn run(&mut self, task_name: &str) -> Result<(), TaskError> {
        let plan = self.build_execution_plan(task_name)?;

        for task_to_run in plan.order {
            if self.executed.contains(&task_to_run) {
                continue;
            }

            if let Some(task) = self.tasks.get(&task_to_run).cloned() {
                if !task.quiet {
                    println!("Running: {}", task.name);
                    if let Some(desc) = &task.desc {
                        println!("  {}", desc);
                    }
                }

                // Phase 1: Process downloads before running the command
                for dl in &task.downloads {
                    if !task.quiet {
                        println!("  Downloading: {} → {}", dl.url, dl.to);
                    }
                    Self::download_file(&dl.url, &dl.to).map_err(|e| {
                        TaskError::ExecutionFailed {
                            task: task.name.clone(),
                            exit_code: -1,
                            output: format!(
                                "Download failed: {} → {}: {}",
                                dl.url, dl.to, e
                            ),
                        }
                    })?;
                }

                // Phase 2: Execute the command
                let expanded_cmd = Self::expand_env_vars(&task.cmd, &task.env);

                let mut cmd = Command::new("sh");
                cmd.arg("-c").arg(&expanded_cmd).envs(&task.env);

                // Set working directory if specified
                if let Some(ref dir) = task.dir {
                    cmd.current_dir(dir);
                }

                // Handle stdin piping
                if task.stdin.is_some() {
                    cmd.stdin(std::process::Stdio::piped());
                }

                let result = if let Some(ref stdin_data) = task.stdin {
                    // Spawn, write to stdin, then wait
                    let mut child = cmd.spawn().map_err(|e| TaskError::ExecutionFailed {
                        task: task.name.clone(),
                        exit_code: -1,
                        output: format!("Failed to execute: {}", e),
                    })?;

                    if let Some(ref mut pipe) = child.stdin {
                        let _ = pipe.write_all(stdin_data.as_bytes());
                    }
                    // Close stdin so child can finish
                    drop(child.stdin.take());

                    child.wait().map_err(|e| TaskError::ExecutionFailed {
                        task: task.name.clone(),
                        exit_code: -1,
                        output: format!("Failed to wait on process: {}", e),
                    })?
                } else {
                    cmd.status().map_err(|e| TaskError::ExecutionFailed {
                        task: task.name.clone(),
                        exit_code: -1,
                        output: format!("Failed to execute: {}", e),
                    })?
                };

                if !result.success() && !task.ignore_errors {
                    let exit_code = result.code().unwrap_or(-1);
                    return Err(TaskError::ExecutionFailed {
                        task: task.name.clone(),
                        exit_code,
                        output: format!("Task failed with exit code {}", exit_code),
                    });
                }

                self.executed.insert(task_to_run);
            }
        }

        Ok(())
    }

    /// Expand environment variables in a command string.
    /// Supports $VAR and ${VAR} syntax.
    /// Task-level env vars take priority over system env vars.
    pub fn expand_env_vars(cmd: &str, task_env: &HashMap<String, String>) -> String {
        let mut result = String::with_capacity(cmd.len());
        let chars: Vec<char> = cmd.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i] == '$' && i + 1 < len {
                if chars[i + 1] == '{' {
                    // ${VAR} syntax
                    if let Some(end) = chars[i + 2..].iter().position(|&c| c == '}') {
                        let var_name: String = chars[i + 2..i + 2 + end].iter().collect();
                        if let Some(val) = task_env.get(&var_name) {
                            result.push_str(val);
                        } else if let Ok(val) = std::env::var(&var_name) {
                            result.push_str(&val);
                        } else {
                            // Keep the original if not found
                            let orig: String = chars[i..i + 2 + end + 1].iter().collect();
                            result.push_str(&orig);
                        }
                        i += 2 + end + 1; // skip past }
                    } else {
                        result.push(chars[i]);
                        i += 1;
                    }
                } else if chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_' {
                    // $VAR syntax
                    let start = i + 1;
                    let mut end = start;
                    while end < len && (chars[end].is_ascii_alphanumeric() || chars[end] == '_') {
                        end += 1;
                    }
                    let var_name: String = chars[start..end].iter().collect();
                    if let Some(val) = task_env.get(&var_name) {
                        result.push_str(val);
                    } else if let Ok(val) = std::env::var(&var_name) {
                        result.push_str(&val);
                    } else {
                        let orig: String = chars[i..end].iter().collect();
                        result.push_str(&orig);
                    }
                    i = end;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        result
    }

    /// Download a file from a URL to a local path
    fn download_file(url: &str, to: &str) -> Result<(), String> {
        // Create parent directories if needed
        if let Some(parent) = std::path::Path::new(to).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
            }
        }

        let response = ureq::get(url)
            .call()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let mut reader = response.into_body().into_reader();
        let mut file = std::fs::File::create(to)
            .map_err(|e| format!("Failed to create file {}: {}", to, e))?;
        std::io::copy(&mut reader, &mut file)
            .map_err(|e| format!("Failed to write file {}: {}", to, e))?;

        Ok(())
    }

    /// Run a task without executing (dry-run)
    pub fn run_dry(&mut self, task_name: &str) -> Result<ExecutionPlan, TaskError> {
        self.build_execution_plan(task_name)
    }

    /// List all available tasks
    pub fn list_tasks(&self) -> Vec<&TaskDef> {
        let mut tasks: Vec<&TaskDef> = self.tasks.values().collect();
        tasks.sort_by(|a, b| a.name.cmp(&b.name));
        tasks
    }

    /// Get a specific task
    pub fn get_task(&self, name: &str) -> Option<&TaskDef> {
        self.tasks.get(name)
    }

    /// Get all tasks
    pub fn get_all_tasks(&self) -> &HashMap<String, TaskDef> {
        &self.tasks
    }
}

/// Errors that can occur during task execution
#[derive(Debug)]
pub enum TaskError {
    TaskNotFound {
        task: String,
        suggestions: Vec<String>,
    },
    UndefinedDependency {
        task: String,
        dep: String,
        suggestion: Option<String>,
    },
    CyclicDependency(Vec<String>),
    InvalidTaskDef {
        task: String,
        reason: String,
    },
    ExecutionFailed {
        task: String,
        exit_code: i32,
        output: String,
    },
    ParseError {
        reason: String,
    },
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::TaskNotFound { task, suggestions } => {
                write!(f, "Error: Task '{}' not found", task)?;
                if !suggestions.is_empty() {
                    write!(f, ". Did you mean '{}'?", suggestions[0])?;
                }
                Ok(())
            }
            TaskError::UndefinedDependency {
                task,
                dep,
                suggestion,
            } => {
                write!(
                    f,
                    "Error: Task '{}' depends on '{}' which does not exist",
                    task, dep
                )?;
                if let Some(s) = suggestion {
                    write!(f, ". Did you mean '{}'?", s)?;
                }
                Ok(())
            }
            TaskError::CyclicDependency(cycle) => {
                write!(
                    f,
                    "Error: Cyclic dependency detected: {}",
                    cycle.join(" -> ")
                )
            }
            TaskError::InvalidTaskDef { task, reason } => {
                write!(f, "Error: Task '{}' has invalid format: {}", task, reason)
            }
            TaskError::ExecutionFailed {
                task,
                exit_code,
                output,
            } => {
                write!(
                    f,
                    "Error: Task '{}' failed with exit code {}\n{}",
                    task, exit_code, output
                )
            }
            TaskError::ParseError { reason } => {
                write!(f, "Error: Parse error: {}", reason)
            }
        }
    }
}

impl std::error::Error for TaskError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taskdef_from_string() {
        let value = Value::String("cargo build".to_string());
        let task = TaskDef::from_value("build".to_string(), &value).unwrap();
        assert_eq!(task.name, "build");
        assert_eq!(task.cmd, "cargo build");
        assert!(task.deps.is_empty());
    }

    #[test]
    fn test_taskdef_from_dict() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("cargo test".to_string()));
        dict.insert(
            "deps".to_string(),
            Value::List(vec![Value::String("build".to_string())]),
        );
        dict.insert("desc".to_string(), Value::String("Run tests".to_string()));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("test".to_string(), &value).unwrap();
        assert_eq!(task.name, "test");
        assert_eq!(task.cmd, "cargo test");
        assert_eq!(task.deps, vec!["build"]);
        assert_eq!(task.desc, Some("Run tests".to_string()));
    }

    #[test]
    fn test_taskdef_missing_cmd() {
        let mut dict = HashMap::new();
        dict.insert(
            "deps".to_string(),
            Value::List(vec![Value::String("build".to_string())]),
        );

        let value = Value::Dict(dict);
        let result = TaskDef::from_value("test".to_string(), &value);
        assert!(result.is_err());
    }

    #[test]
    fn test_linear_dependency_chain() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new("test", "cargo test", vec!["build".to_string()]),
            TaskDef::new("deploy", "./deploy.sh", vec!["test".to_string()]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let plan = runner.build_execution_plan("deploy").unwrap();

        assert_eq!(plan.order, vec!["build", "test", "deploy"]);
    }

    #[test]
    fn test_diamond_dependency() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new("test", "cargo test", vec!["build".to_string()]),
            TaskDef::new("lint", "cargo clippy", vec!["build".to_string()]),
            TaskDef::new(
                "check",
                "echo ok",
                vec!["test".to_string(), "lint".to_string()],
            ),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let plan = runner.build_execution_plan("check").unwrap();

        // build should come first
        assert_eq!(plan.order[0], "build");
        // build should only appear once
        assert_eq!(plan.order.iter().filter(|t| *t == "build").count(), 1);
        // check should be last
        assert_eq!(plan.order[plan.order.len() - 1], "check");
    }

    #[test]
    fn test_undefined_dependency() {
        let tasks = vec![TaskDef::new(
            "test",
            "cargo test",
            vec!["nonexistent".to_string()],
        )];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let result = TaskRunner::new(tasks_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_direct_cycle() {
        let tasks = vec![
            TaskDef::new("a", "cmd a", vec!["b".to_string()]),
            TaskDef::new("b", "cmd b", vec!["a".to_string()]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let result = TaskRunner::new(tasks_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(TaskRunner::levenshtein_distance("build", "bild"), 1);
        assert_eq!(TaskRunner::levenshtein_distance("cat", "dog"), 3);
        assert_eq!(TaskRunner::levenshtein_distance("", "test"), 4);
    }

    #[test]
    fn test_list_tasks() {
        let tasks = vec![
            TaskDef::new("zebra", "cmd", vec![]),
            TaskDef::new("apple", "cmd", vec![]),
            TaskDef::new("monkey", "cmd", vec![]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let runner = TaskRunner::new(tasks_map).unwrap();
        let listed = runner.list_tasks();
        let names: Vec<&str> = listed.iter().map(|t| t.name.as_str()).collect();

        assert_eq!(names, vec!["apple", "monkey", "zebra"]);
    }

    // ========== Phase 3 Tests ==========

    #[test]
    fn test_task_not_found_with_suggestion() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new("test", "cargo test", vec![]),
            TaskDef::new("deploy", "./deploy.sh", vec![]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let err = runner.build_execution_plan("bild").unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("not found"), "should say task not found");
        assert!(
            msg.contains("build"),
            "should suggest 'build' for typo 'bild'"
        );
    }

    #[test]
    fn test_task_not_found_no_suggestion_for_distant_name() {
        let tasks = vec![TaskDef::new("build", "cargo build", vec![])];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let err = runner.build_execution_plan("zzzzzzz").unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("not found"));
        // Should NOT suggest 'build' since 'zzzzzzz' is too far
        assert!(
            !msg.contains("Did you mean"),
            "should not suggest for distant names"
        );
    }

    #[test]
    fn test_undefined_dep_with_suggestion() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new(
                "test",
                "cargo test",
                vec!["bild".to_string()], // typo: should be "build"
            ),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let result = TaskRunner::new(tasks_map);
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("does not exist"));
        assert!(
            msg.contains("Did you mean 'build'"),
            "should suggest 'build' for typo 'bild', got: {}",
            msg
        );
    }

    #[test]
    fn test_expand_env_vars_task_env() {
        let mut env = HashMap::new();
        env.insert("NAME".to_string(), "Alice".to_string());
        env.insert("ROLE".to_string(), "admin".to_string());

        let result = TaskRunner::expand_env_vars("Hello $NAME, role: $ROLE", &env);
        assert_eq!(result, "Hello Alice, role: admin");
    }

    #[test]
    fn test_expand_env_vars_braces_syntax() {
        let mut env = HashMap::new();
        env.insert("APP".to_string(), "myapp".to_string());

        let result = TaskRunner::expand_env_vars("Running ${APP}_server", &env);
        assert_eq!(result, "Running myapp_server");
    }

    #[test]
    fn test_expand_env_vars_system_fallback() {
        let env = HashMap::new();
        // HOME should exist on all Unix systems
        let result = TaskRunner::expand_env_vars("home: $HOME", &env);
        assert!(
            !result.contains("$HOME"),
            "should expand $HOME from system env"
        );
        assert!(result.starts_with("home: /"), "should start with home: /");
    }

    #[test]
    fn test_expand_env_vars_unknown_kept() {
        let env = HashMap::new();
        let result = TaskRunner::expand_env_vars("value: $AVON_NONEXISTENT_VAR_12345", &env);
        assert_eq!(
            result, "value: $AVON_NONEXISTENT_VAR_12345",
            "unknown vars should be kept as-is"
        );
    }

    #[test]
    fn test_expand_env_vars_task_env_overrides_system() {
        let mut env = HashMap::new();
        env.insert("HOME".to_string(), "/custom/home".to_string());

        let result = TaskRunner::expand_env_vars("home: $HOME", &env);
        assert_eq!(
            result, "home: /custom/home",
            "task env should override system env"
        );
    }

    #[test]
    fn test_expand_env_vars_no_vars() {
        let env = HashMap::new();
        let result = TaskRunner::expand_env_vars("echo hello world", &env);
        assert_eq!(result, "echo hello world");
    }

    #[test]
    fn test_expand_env_vars_dollar_at_end() {
        let env = HashMap::new();
        let result = TaskRunner::expand_env_vars("price is 5$", &env);
        assert_eq!(result, "price is 5$");
    }

    #[test]
    fn test_expand_env_vars_empty_braces() {
        let env = HashMap::new();
        let result = TaskRunner::expand_env_vars("value: ${}", &env);
        // Empty var name between braces - should keep as-is or handle gracefully
        assert!(result.contains("${}") || result.contains("value:"));
    }

    #[test]
    fn test_expand_env_vars_multiple_occurrences() {
        let mut env = HashMap::new();
        env.insert("X".to_string(), "1".to_string());

        let result = TaskRunner::expand_env_vars("$X + $X = 2", &env);
        assert_eq!(result, "1 + 1 = 2");
    }

    #[test]
    fn test_taskdef_with_env() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo $VAR".to_string()));
        let mut env_dict = HashMap::new();
        env_dict.insert("VAR".to_string(), Value::String("hello".to_string()));
        dict.insert("env".to_string(), Value::Dict(env_dict));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("test".to_string(), &value).unwrap();
        assert_eq!(task.env.get("VAR"), Some(&"hello".to_string()));
    }

    #[test]
    fn test_execution_plan_format() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new("test", "cargo test", vec!["build".to_string()]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let plan = runner.build_execution_plan("test").unwrap();
        let formatted = plan.format();

        assert!(formatted.contains("Execution Plan:"));
        assert!(formatted.contains("build"));
        assert!(formatted.contains("test"));
        assert!(formatted.contains("cargo build"));
        assert!(formatted.contains("cargo test"));
    }

    #[test]
    fn test_single_task_no_deps() {
        let tasks = vec![TaskDef::new("hello", "echo hi", vec![])];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let plan = runner.build_execution_plan("hello").unwrap();
        assert_eq!(plan.order, vec!["hello"]);
    }

    #[test]
    fn test_task_error_display_variants() {
        // TaskNotFound with suggestions
        let err = TaskError::TaskNotFound {
            task: "bild".to_string(),
            suggestions: vec!["build".to_string()],
        };
        assert!(format!("{}", err).contains("Did you mean 'build'"));

        // TaskNotFound without suggestions
        let err = TaskError::TaskNotFound {
            task: "xyz".to_string(),
            suggestions: vec![],
        };
        let msg = format!("{}", err);
        assert!(msg.contains("xyz"));
        assert!(!msg.contains("Did you mean"));

        // UndefinedDependency with suggestion
        let err = TaskError::UndefinedDependency {
            task: "test".to_string(),
            dep: "bild".to_string(),
            suggestion: Some("build".to_string()),
        };
        assert!(format!("{}", err).contains("Did you mean 'build'"));

        // UndefinedDependency without suggestion
        let err = TaskError::UndefinedDependency {
            task: "test".to_string(),
            dep: "xyz".to_string(),
            suggestion: None,
        };
        let msg = format!("{}", err);
        assert!(!msg.contains("Did you mean"));

        // CyclicDependency
        let err = TaskError::CyclicDependency(vec!["a".to_string(), "b".to_string()]);
        assert!(format!("{}", err).contains("Cyclic"));

        // InvalidTaskDef
        let err = TaskError::InvalidTaskDef {
            task: "test".to_string(),
            reason: "bad format".to_string(),
        };
        assert!(format!("{}", err).contains("bad format"));

        // ExecutionFailed
        let err = TaskError::ExecutionFailed {
            task: "build".to_string(),
            exit_code: 1,
            output: "compile error".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("build"));
        assert!(msg.contains("1"));

        // ParseError
        let err = TaskError::ParseError {
            reason: "unexpected token".to_string(),
        };
        assert!(format!("{}", err).contains("unexpected token"));
    }

    #[test]
    fn test_get_task() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new("test", "cargo test", vec![]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let runner = TaskRunner::new(tasks_map).unwrap();
        assert!(runner.get_task("build").is_some());
        assert!(runner.get_task("test").is_some());
        assert!(runner.get_task("nonexistent").is_none());
    }

    #[test]
    fn test_levenshtein_equal_strings() {
        assert_eq!(TaskRunner::levenshtein_distance("hello", "hello"), 0);
    }

    #[test]
    fn test_levenshtein_both_empty() {
        assert_eq!(TaskRunner::levenshtein_distance("", ""), 0);
    }

    #[test]
    fn test_find_similar_names() {
        let tasks = vec![
            TaskDef::new("build", "cmd", vec![]),
            TaskDef::new("test", "cmd", vec![]),
            TaskDef::new("deploy", "cmd", vec![]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let suggestions = TaskRunner::find_similar_names("bild", &tasks_map);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0], "build");
    }

    #[test]
    fn test_taskdef_invalid_type() {
        let value = Value::Number(crate::common::Number::Int(42));
        let result = TaskDef::from_value("test".to_string(), &value);
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("must be a string or dict"));
    }

    #[test]
    fn test_taskdef_invalid_cmd_type() {
        let mut dict = HashMap::new();
        dict.insert(
            "cmd".to_string(),
            Value::Number(crate::common::Number::Int(42)),
        );
        let value = Value::Dict(dict);
        let result = TaskDef::from_value("test".to_string(), &value);
        assert!(result.is_err());
    }

    #[test]
    fn test_taskdef_invalid_deps_type() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo hi".to_string()));
        dict.insert("deps".to_string(), Value::String("not a list".to_string()));
        let value = Value::Dict(dict);
        let result = TaskDef::from_value("test".to_string(), &value);
        assert!(result.is_err());
    }

    #[test]
    fn test_dry_run() {
        let tasks = vec![
            TaskDef::new("build", "cargo build", vec![]),
            TaskDef::new("test", "cargo test", vec!["build".to_string()]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let plan = runner.run_dry("test").unwrap();
        assert_eq!(plan.order, vec!["build", "test"]);
    }

    #[test]
    fn test_three_level_chain() {
        let tasks = vec![
            TaskDef::new("a", "echo a", vec![]),
            TaskDef::new("b", "echo b", vec!["a".to_string()]),
            TaskDef::new("c", "echo c", vec!["b".to_string()]),
            TaskDef::new("d", "echo d", vec!["c".to_string()]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let plan = runner.build_execution_plan("d").unwrap();
        assert_eq!(plan.order, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_indirect_cycle() {
        let tasks = vec![
            TaskDef::new("a", "cmd", vec!["c".to_string()]),
            TaskDef::new("b", "cmd", vec!["a".to_string()]),
            TaskDef::new("c", "cmd", vec!["b".to_string()]),
        ];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let result = TaskRunner::new(tasks_map);
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("Cyclic"));
    }

    #[test]
    fn test_cmd_as_list_of_strings() {
        let mut dict = HashMap::new();
        dict.insert(
            "cmd".to_string(),
            Value::List(vec![
                Value::String("echo step1".to_string()),
                Value::String("echo step2".to_string()),
                Value::String("echo step3".to_string()),
            ]),
        );

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("multi".to_string(), &value).unwrap();

        // Commands should be joined with " && "
        assert_eq!(task.cmd, "echo step1 && echo step2 && echo step3");
    }

    #[test]
    fn test_cmd_as_single_element_list() {
        let mut dict = HashMap::new();
        dict.insert(
            "cmd".to_string(),
            Value::List(vec![Value::String("cargo build".to_string())]),
        );

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("build".to_string(), &value).unwrap();

        // Single element list should work fine
        assert_eq!(task.cmd, "cargo build");
    }

    #[test]
    fn test_cmd_as_empty_list_fails() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::List(vec![]));

        let value = Value::Dict(dict);
        let result = TaskDef::from_value("empty".to_string(), &value);

        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("empty"));
    }

    #[test]
    fn test_cmd_as_list_with_non_string_fails() {
        let mut dict = HashMap::new();
        dict.insert(
            "cmd".to_string(),
            Value::List(vec![
                Value::String("echo step1".to_string()),
                Value::Number(crate::common::Number::Int(42)),
            ]),
        );

        let value = Value::Dict(dict);
        let result = TaskDef::from_value("bad".to_string(), &value);

        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("strings"));
    }

    #[test]
    fn test_cmd_as_list_with_deps() {
        let mut dict = HashMap::new();
        dict.insert(
            "cmd".to_string(),
            Value::List(vec![
                Value::String("cargo build".to_string()),
                Value::String("cargo test".to_string()),
            ]),
        );
        dict.insert(
            "deps".to_string(),
            Value::List(vec![Value::String("clean".to_string())]),
        );
        dict.insert(
            "desc".to_string(),
            Value::String("Build and test".to_string()),
        );

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("ci".to_string(), &value).unwrap();

        assert_eq!(task.cmd, "cargo build && cargo test");
        assert_eq!(task.deps, vec!["clean".to_string()]);
        assert_eq!(task.desc, Some("Build and test".to_string()));
    }

    #[test]
    fn test_cmd_as_list_with_env() {
        let mut dict = HashMap::new();
        dict.insert(
            "cmd".to_string(),
            Value::List(vec![
                Value::String("cargo build".to_string()),
                Value::String("cargo run".to_string()),
            ]),
        );

        let mut env_dict = HashMap::new();
        env_dict.insert("PROFILE".to_string(), Value::String("release".to_string()));
        dict.insert("env".to_string(), Value::Dict(env_dict));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("build_and_run".to_string(), &value).unwrap();

        assert_eq!(task.cmd, "cargo build && cargo run");
        assert_eq!(task.env.get("PROFILE"), Some(&"release".to_string()));
    }

    // ── New key tests ────────────────────────────────────

    #[test]
    fn test_taskdef_dir_field() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("ls".to_string()));
        dict.insert("dir".to_string(), Value::String("/tmp".to_string()));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("list_tmp".to_string(), &value).unwrap();

        assert_eq!(task.dir, Some("/tmp".to_string()));
    }

    #[test]
    fn test_taskdef_quiet_field() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo hi".to_string()));
        dict.insert("quiet".to_string(), Value::Bool(true));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("silent".to_string(), &value).unwrap();

        assert!(task.quiet);
    }

    #[test]
    fn test_taskdef_quiet_default_false() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo hi".to_string()));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("loud".to_string(), &value).unwrap();

        assert!(!task.quiet);
    }

    #[test]
    fn test_taskdef_quiet_bad_type() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo".to_string()));
        dict.insert(
            "quiet".to_string(),
            Value::String("yes".to_string()),
        );

        let value = Value::Dict(dict);
        let result = TaskDef::from_value("bad".to_string(), &value);
        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("boolean"));
    }

    #[test]
    fn test_taskdef_ignore_errors_field() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("exit 1".to_string()));
        dict.insert("ignore_errors".to_string(), Value::Bool(true));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("might_fail".to_string(), &value).unwrap();

        assert!(task.ignore_errors);
    }

    #[test]
    fn test_taskdef_ignore_errors_default_false() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo ok".to_string()));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("strict".to_string(), &value).unwrap();

        assert!(!task.ignore_errors);
    }

    #[test]
    fn test_taskdef_stdin_field() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("cat".to_string()));
        dict.insert(
            "stdin".to_string(),
            Value::String("hello world".to_string()),
        );

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("pipe_in".to_string(), &value).unwrap();

        assert_eq!(task.stdin, Some("hello world".to_string()));
    }

    #[test]
    fn test_taskdef_download_single() {
        let mut dl_dict = HashMap::new();
        dl_dict.insert(
            "url".to_string(),
            Value::String("https://example.com/file.txt".to_string()),
        );
        dl_dict.insert(
            "to".to_string(),
            Value::String("vendor/file.txt".to_string()),
        );

        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo done".to_string()));
        dict.insert("download".to_string(), Value::Dict(dl_dict));

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("fetch".to_string(), &value).unwrap();

        assert_eq!(task.downloads.len(), 1);
        assert_eq!(task.downloads[0].url, "https://example.com/file.txt");
        assert_eq!(task.downloads[0].to, "vendor/file.txt");
    }

    #[test]
    fn test_taskdef_download_list() {
        let mut dl1 = HashMap::new();
        dl1.insert(
            "url".to_string(),
            Value::String("https://a.com/1.txt".to_string()),
        );
        dl1.insert("to".to_string(), Value::String("a.txt".to_string()));

        let mut dl2 = HashMap::new();
        dl2.insert(
            "url".to_string(),
            Value::String("https://b.com/2.txt".to_string()),
        );
        dl2.insert("to".to_string(), Value::String("b.txt".to_string()));

        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo done".to_string()));
        dict.insert(
            "download".to_string(),
            Value::List(vec![Value::Dict(dl1), Value::Dict(dl2)]),
        );

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("fetch_many".to_string(), &value).unwrap();

        assert_eq!(task.downloads.len(), 2);
        assert_eq!(task.downloads[0].to, "a.txt");
        assert_eq!(task.downloads[1].to, "b.txt");
    }

    #[test]
    fn test_taskdef_download_missing_url() {
        let mut dl_dict = HashMap::new();
        dl_dict.insert("to".to_string(), Value::String("file.txt".to_string()));

        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo".to_string()));
        dict.insert("download".to_string(), Value::Dict(dl_dict));

        let value = Value::Dict(dict);
        let result = TaskDef::from_value("bad".to_string(), &value);
        assert!(result.is_err());
    }

    #[test]
    fn test_taskdef_download_bad_type() {
        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("echo".to_string()));
        dict.insert(
            "download".to_string(),
            Value::String("http://bad".to_string()),
        );

        let value = Value::Dict(dict);
        let result = TaskDef::from_value("bad".to_string(), &value);
        assert!(result.is_err());
    }

    #[test]
    fn test_taskdef_all_new_keys_together() {
        let mut dl_dict = HashMap::new();
        dl_dict.insert(
            "url".to_string(),
            Value::String("https://example.com/data.json".to_string()),
        );
        dl_dict.insert(
            "to".to_string(),
            Value::String("data.json".to_string()),
        );

        let mut dict = HashMap::new();
        dict.insert("cmd".to_string(), Value::String("cat data.json".to_string()));
        dict.insert("dir".to_string(), Value::String("/tmp".to_string()));
        dict.insert("quiet".to_string(), Value::Bool(true));
        dict.insert("ignore_errors".to_string(), Value::Bool(true));
        dict.insert(
            "stdin".to_string(),
            Value::String("input data".to_string()),
        );
        dict.insert("download".to_string(), Value::Dict(dl_dict));
        dict.insert(
            "desc".to_string(),
            Value::String("Full featured task".to_string()),
        );

        let value = Value::Dict(dict);
        let task = TaskDef::from_value("full".to_string(), &value).unwrap();

        assert_eq!(task.dir, Some("/tmp".to_string()));
        assert!(task.quiet);
        assert!(task.ignore_errors);
        assert_eq!(task.stdin, Some("input data".to_string()));
        assert_eq!(task.downloads.len(), 1);
        assert_eq!(task.desc, Some("Full featured task".to_string()));
    }

    #[test]
    fn test_dir_execution() {
        let tasks = vec![{
            let mut t = TaskDef::new("list_tmp", "pwd", vec![]);
            t.dir = Some("/tmp".to_string());
            t
        }];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let result = runner.run("list_tmp");
        assert!(result.is_ok());
    }

    #[test]
    fn test_ignore_errors_execution() {
        let tasks = vec![{
            let mut t = TaskDef::new("fail_ok", "exit 1", vec![]);
            t.ignore_errors = true;
            t
        }];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let result = runner.run("fail_ok");
        assert!(result.is_ok(), "ignore_errors should allow exit code 1");
    }

    #[test]
    fn test_stdin_execution() {
        let tasks = vec![{
            let mut t = TaskDef::new("pipe_test", "cat", vec![]);
            t.stdin = Some("hello from stdin".to_string());
            t
        }];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let result = runner.run("pipe_test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_quiet_does_not_crash() {
        let tasks = vec![{
            let mut t = TaskDef::new("silent", "echo quiet", vec![]);
            t.quiet = true;
            t
        }];

        let mut tasks_map = HashMap::new();
        for task in tasks {
            tasks_map.insert(task.name.clone(), task);
        }

        let mut runner = TaskRunner::new(tasks_map).unwrap();
        let result = runner.run("silent");
        assert!(result.is_ok());
    }
}
