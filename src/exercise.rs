//! Exercise loading and status tracking

use crate::runner;
use serde::Deserialize;
use std::path::PathBuf;

/// Exercise mode - how to verify completion
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseMode {
    /// Just needs to run without errors
    Compile,
    /// Needs to run and pass embedded assertion checks
    Test,
}

impl Default for ExerciseMode {
    fn default() -> Self {
        Self::Test
    }
}

/// An exercise definition from info.toml
#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseInfo {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub mode: ExerciseMode,
}

/// Exercises file structure
#[derive(Debug, Deserialize)]
struct ExercisesFile {
    #[serde(rename = "exercises")]
    exercises: Vec<ExerciseInfo>,
}

/// Runtime exercise with resolved paths
#[derive(Debug, Clone)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: ExerciseMode,
}

/// Exercise completion status
#[derive(Debug, Clone, PartialEq)]
pub enum ExerciseStatus {
    /// Exercise is complete (runs, tests pass, no NOT DONE marker)
    Done,
    /// Exercise still has ; I AM NOT DONE marker
    NotDone,
    /// Exercise has runtime/parse errors
    CompileError,
    /// Exercise runs but tests fail
    TestFail,
}

impl Exercise {
    /// Check the current status of this exercise
    pub fn status(&self) -> ExerciseStatus {
        // First check if file exists
        let content = match std::fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(_) => return ExerciseStatus::CompileError,
        };

        // Check for NOT DONE marker
        if content.contains("; I AM NOT DONE") {
            return ExerciseStatus::NotDone;
        }

        // Try to run (compile check)
        if runner::compile(&self.path).is_err() {
            return ExerciseStatus::CompileError;
        }

        // If mode is test, run and check tests
        if self.mode == ExerciseMode::Test {
            match runner::run_tests(&self.path) {
                Ok(output) => {
                    if output.contains("FAIL") {
                        return ExerciseStatus::TestFail;
                    }
                }
                Err(_) => return ExerciseStatus::TestFail,
            }
        }

        ExerciseStatus::Done
    }

    /// Get the path to the hint file for this exercise
    pub fn hint_path(&self) -> PathBuf {
        // Convert exercises/00-intro/01-hello.slisp to hints/00-intro/01-hello.md
        let mut hint_path = PathBuf::from("hints");
        if let Some(parent) = self.path.parent() {
            if let Some(topic) = parent.file_name() {
                hint_path.push(topic);
            }
        }
        if let Some(stem) = self.path.file_stem() {
            hint_path.push(format!("{}.md", stem.to_string_lossy()));
        }
        hint_path
    }

    /// Get the path to the solution file for this exercise
    pub fn solution_path(&self) -> PathBuf {
        let mut solution_path = PathBuf::from("solutions");
        if let Some(parent) = self.path.parent() {
            if let Some(topic) = parent.file_name() {
                solution_path.push(topic);
            }
        }
        if let Some(name) = self.path.file_name() {
            solution_path.push(name);
        }
        solution_path
    }
}

/// Load all exercises from info.toml
pub fn load_exercises() -> Result<Vec<Exercise>, String> {
    let info_path = PathBuf::from("exercises/info.toml");

    if !info_path.exists() {
        return Err(
            "exercises/info.toml not found. Are you in the lisplings directory?".to_string(),
        );
    }

    let content = std::fs::read_to_string(&info_path)
        .map_err(|e| format!("Failed to read info.toml: {}", e))?;

    let exercises_file: ExercisesFile =
        toml::from_str(&content).map_err(|e| format!("Failed to parse info.toml: {}", e))?;

    let exercises = exercises_file
        .exercises
        .into_iter()
        .map(|info| Exercise {
            name: info.name,
            path: PathBuf::from(&info.path),
            mode: info.mode,
        })
        .collect();

    Ok(exercises)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hint_path() {
        let exercise = Exercise {
            name: "01-hello".to_string(),
            path: PathBuf::from("exercises/00-intro/01-hello.slisp"),
            mode: ExerciseMode::Compile,
        };
        assert_eq!(
            exercise.hint_path(),
            PathBuf::from("hints/00-intro/01-hello.md")
        );
    }

    #[test]
    fn test_solution_path() {
        let exercise = Exercise {
            name: "01-hello".to_string(),
            path: PathBuf::from("exercises/00-intro/01-hello.slisp"),
            mode: ExerciseMode::Compile,
        };
        assert_eq!(
            exercise.solution_path(),
            PathBuf::from("solutions/00-intro/01-hello.slisp")
        );
    }
}
