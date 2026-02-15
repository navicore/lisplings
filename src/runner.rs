//! Exercise execution via the `seqlisp` interpreter
//!
//! Both compile and test modes run `seqlisp <path>` and check for errors.
//! SeqLisp exits 0 even on parse/runtime errors (errors go to stdout),
//! so compile mode scans output for "Error" lines.
//! Test mode exercises embed assertions that call `(exit 1)` on failure,
//! so we check the exit code and scan for "FAIL".

use std::path::Path;
use std::process::Command;

/// Run a SeqLisp file and check for errors (compile mode).
///
/// SeqLisp exits 0 even on errors, so we scan combined output for "Error".
/// Returns Ok(()) if no errors found, Err with output otherwise.
pub fn compile(path: &Path) -> Result<(), String> {
    let output = Command::new("seqlisp")
        .arg(path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run seqlisp: {}. Is seqlisp installed and in PATH?",
                e
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}{}", stdout, stderr);

    // SeqLisp reports errors on stdout with "Error" prefix
    if combined.lines().any(|l| l.starts_with("Error")) {
        Err(combined)
    } else {
        Ok(())
    }
}

/// Run a SeqLisp file's embedded tests and return the output.
///
/// Test exercises embed assertion logic that calls `(exit 1)` on failure.
/// We check both exit code and scan for "FAIL" in output.
pub fn run_tests(path: &Path) -> Result<String, String> {
    let output = Command::new("seqlisp")
        .arg(path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run seqlisp: {}. Is seqlisp installed and in PATH?",
                e
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}{}", stdout, stderr);

    if !output.status.success() || combined.contains("FAIL") || combined.lines().any(|l| l.starts_with("Error")) {
        Err(combined)
    } else {
        Ok(combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn temp_slisp_file(content: &str) -> PathBuf {
        let dir = std::env::temp_dir();
        let path = dir.join("test_exercise.slisp");
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    #[ignore] // Requires seqlisp to be installed
    fn test_compile_valid() {
        let path = temp_slisp_file("(print 42)\n(exit 0)");
        assert!(compile(&path).is_ok());
        fs::remove_file(&path).ok();
    }

    #[test]
    #[ignore] // Requires seqlisp to be installed
    fn test_compile_invalid() {
        let path = temp_slisp_file("(+ 1 \"hello\")\n(exit 0)");
        assert!(compile(&path).is_err());
        fs::remove_file(&path).ok();
    }
}
