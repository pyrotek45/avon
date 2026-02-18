use std::collections::{HashMap, HashSet};
use std::process::Command;
use crate::common::Value;
use std::fmt;

/// Represents a single task definition
#[derive(Debug, Clone)]
pub struct TaskDef {
    pub name: String,
    pub cmd: String,
    pub deps: Vec<String>,
    pub desc: Option<String>,
    pub env: HashMap<String, String>,
}

impl TaskDef {
    /// Create a new TaskDef from name and command
    pub fn new(name: &str, cmd: &str, deps: Vec<String>) -> Self {
        TaskDef {
            name: name.to_string(),
            cmd: cmd.to_string(),
            deps,
            desc: None,
            env: HashMap::new(),
        }
    }

    /// Parse a TaskDef from a Value (either String or Dict)
    pub fn from_value(name: String, value: &Value) -> Result<Self, TaskError> {
        match value {
            // Simple format: "command string"
            Value::String(cmd) => {
                Ok(TaskDef {
                    name,
                    cmd: cmd.clone(),
                    deps: Vec::new(),
                    desc: None,
                    env: HashMap::new(),
                })
            }
            // Structured format: { cmd: "...", deps: [...], desc: "..." }
            Value::Dict(dict) => {
                // Extract cmd field (required)
                let cmd = match dict.get("cmd") {
                    Some(Value::String(c)) => c.clone(),
                    Some(_) => {
                        return Err(TaskError::InvalidTaskDef {
                            task: name,
                            reason: "cmd must be a string".to_string(),
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
                let desc = dict
                    .get("desc")
                    .and_then(|v| match v {
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

                Ok(TaskDef {
                    name,
                    cmd,
                    deps,
                    desc,
                    env,
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
                    let suggestion_text = if !suggestions.is_empty() {
                        format!(" Did you mean '{}'?", suggestions[0])
                    } else {
                        String::new()
                    };
                    return Err(TaskError::UndefinedDependency {
                        task: name.clone(),
                        dep: dep.clone(),
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

        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }

        for (i, a_char) in a.chars().enumerate() {
            for (j, b_char) in b.chars().enumerate() {
                let cost = if a_char == b_char { 0 } else { 1 };
                matrix[i + 1][j + 1] = std::cmp::min(
                    std::cmp::min(
                        matrix[i][j + 1] + 1,     // deletion
                        matrix[i + 1][j] + 1,     // insertion
                    ),
                    matrix[i][j] + cost,          // substitution
                );
            }
        }

        matrix[a_len][b_len]
    }

    /// Build execution plan for a task
    pub fn build_execution_plan(&mut self, task_name: &str) -> Result<ExecutionPlan, TaskError> {
        if !self.tasks.contains_key(task_name) {
            let _suggestions = Self::find_similar_names(task_name, &self.tasks);
            return Err(TaskError::TaskNotFound(task_name.to_string()));
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
                println!("Running: {}", task.name);
                if let Some(desc) = &task.desc {
                    println!("  {}", desc);
                }

                let status = Command::new("sh")
                    .arg("-c")
                    .arg(&task.cmd)
                    .envs(&task.env)
                    .status()
                    .map_err(|e| TaskError::ExecutionFailed {
                        task: task.name.clone(),
                        exit_code: -1,
                        output: format!("Failed to execute: {}", e),
                    })?;

                if !status.success() {
                    let exit_code = status.code().unwrap_or(-1);
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
    TaskNotFound(String),
    UndefinedDependency { task: String, dep: String },
    CyclicDependency(Vec<String>),
    InvalidTaskDef { task: String, reason: String },
    ExecutionFailed {
        task: String,
        exit_code: i32,
        output: String,
    },
    ParseError { reason: String },
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::TaskNotFound(name) => {
                write!(f, "Error: Task '{}' not found", name)
            }
            TaskError::UndefinedDependency { task, dep } => {
                write!(
                    f,
                    "Error: Task '{}' depends on '{}' which does not exist",
                    task, dep
                )
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
        assert_eq!(plan.order.iter().filter(|&&t| t == "build").count(), 1);
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
}
