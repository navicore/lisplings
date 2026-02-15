//! Lisplings - Interactive exercises for learning SeqLisp
//!
//! A rustlings-inspired tool for learning Lisp programming with SeqLisp.

mod exercise;
mod runner;

use clap::{Parser, Subcommand};
use colored::Colorize;
use exercise::{Exercise, ExerciseStatus, load_exercises};
use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Duration, SystemTime};

// Embed exercise files at compile time
static EXERCISES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/exercises");
static SOLUTIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/solutions");
static HINTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/hints");

/// Cache for exercise status to avoid repeated interpreter invocations
struct StatusCache {
    /// Maps exercise path to (last_mtime, cached_status)
    cache: HashMap<PathBuf, (SystemTime, ExerciseStatus)>,
}

impl StatusCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get the status of an exercise, using cache when possible.
    fn get_status(&mut self, exercise: &Exercise) -> ExerciseStatus {
        // Get current file mtime
        let current_mtime = match std::fs::metadata(&exercise.path) {
            Ok(meta) => meta.modified().ok(),
            Err(_) => return ExerciseStatus::CompileError,
        };

        // Quick pre-filter: if file contains marker, skip expensive checks
        if let Ok(content) = std::fs::read_to_string(&exercise.path) {
            if content.contains("; I AM NOT DONE") {
                if let Some(mtime) = current_mtime {
                    self.cache
                        .insert(exercise.path.clone(), (mtime, ExerciseStatus::NotDone));
                }
                return ExerciseStatus::NotDone;
            }
        }

        // Check cache: if mtime unchanged, return cached status
        if let Some(mtime) = current_mtime {
            if let Some((cached_mtime, cached_status)) = self.cache.get(&exercise.path) {
                if *cached_mtime == mtime {
                    return cached_status.clone();
                }
            }
        }

        // Cache miss or file changed - run the full status check
        let status = exercise.status();

        // Update cache
        if let Some(mtime) = current_mtime {
            self.cache
                .insert(exercise.path.clone(), (mtime, status.clone()));
        }

        status
    }
}

#[derive(Parser)]
#[command(name = "lisplings")]
#[command(version, about = "Interactive exercises for learning SeqLisp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new lisplings project directory
    Init {
        /// Directory name (defaults to "my-lisplings")
        #[arg(default_value = "my-lisplings")]
        path: PathBuf,
    },
    /// Watch for file changes and auto-verify exercises
    Watch {
        /// Filter to a specific chapter (e.g., "02" or "02-lists")
        #[arg(short, long)]
        chapter: Option<String>,
    },
    /// List all exercises with their status
    List {
        /// Filter to a specific chapter (e.g., "02" or "02-lists")
        #[arg(short, long)]
        chapter: Option<String>,
    },
    /// Show hint for the current or specified exercise
    Hint {
        /// Exercise name (optional, defaults to current)
        name: Option<String>,
    },
    /// Reset an exercise to its original state
    Reset {
        /// Exercise name (optional, defaults to current)
        name: Option<String>,
    },
    /// Verify all exercises and show progress
    Verify,
    /// Skip to the next exercise
    Next,
}

fn main() {
    let cli = Cli::parse();

    // Handle init command before loading exercises (since exercises may not exist yet)
    if let Some(Commands::Init { path }) = cli.command {
        cmd_init(&path);
        return;
    }

    // Load exercises
    let exercises = match load_exercises() {
        Ok(ex) => ex,
        Err(e) => {
            eprintln!("{} {}", "Error loading exercises:".red(), e);
            eprintln!(
                "\n{}",
                "Hint: Run 'lisplings init' to create a new project.".yellow()
            );
            process::exit(1);
        }
    };

    if exercises.is_empty() {
        eprintln!("{}", "No exercises found in exercises/info.toml".red());
        eprintln!(
            "\n{}",
            "Hint: Run 'lisplings init' to create a new project.".yellow()
        );
        process::exit(1);
    }

    match cli.command {
        Some(Commands::Init { .. }) => unreachable!(),
        Some(Commands::Watch { chapter }) => {
            let filtered = filter_by_chapter(&exercises, chapter.as_deref());
            cmd_watch(&filtered);
        }
        Some(Commands::List { chapter }) => {
            let filtered = filter_by_chapter(&exercises, chapter.as_deref());
            cmd_list(&filtered);
        }
        Some(Commands::Hint { name }) => cmd_hint(&exercises, name),
        Some(Commands::Reset { name }) => cmd_reset(&exercises, name),
        Some(Commands::Verify) => cmd_verify(&exercises),
        Some(Commands::Next) => cmd_next(&exercises),
        None => cmd_watch(&exercises), // Default to watch mode
    }
}

/// Filter exercises to a specific chapter by prefix match
fn filter_by_chapter(exercises: &[Exercise], chapter: Option<&str>) -> Vec<Exercise> {
    match chapter {
        None => exercises.to_vec(),
        Some(prefix) => {
            let filtered: Vec<Exercise> = exercises
                .iter()
                .filter(|e| {
                    let chapter_name = e
                        .path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    chapter_name.starts_with(prefix)
                })
                .cloned()
                .collect();

            if filtered.is_empty() {
                eprintln!(
                    "{} No exercises found for chapter '{}'",
                    "Warning:".yellow(),
                    prefix
                );
                eprintln!("Available chapters:");
                let mut chapters: Vec<&str> = exercises
                    .iter()
                    .filter_map(|e| {
                        e.path
                            .parent()
                            .and_then(|p| p.file_name())
                            .and_then(|s| s.to_str())
                    })
                    .collect();
                chapters.sort();
                chapters.dedup();
                for ch in chapters {
                    eprintln!("  {}", ch);
                }
                process::exit(1);
            }

            println!(
                "{} Filtering to chapter '{}' ({} exercises)\n",
                "Note:".cyan(),
                prefix,
                filtered.len()
            );
            filtered
        }
    }
}

/// Initialize a new lisplings project directory
fn cmd_init(path: &Path) {
    if path.exists() {
        eprintln!(
            "{} Directory '{}' already exists.",
            "Error:".red(),
            path.display()
        );
        eprintln!("Choose a different name or remove the existing directory.");
        process::exit(1);
    }

    println!(
        "{} Initializing lisplings project in '{}'...",
        "->".cyan(),
        path.display()
    );

    if let Err(e) = std::fs::create_dir_all(path) {
        eprintln!("{} Failed to create directory: {}", "Error:".red(), e);
        process::exit(1);
    }

    // Extract exercises
    let exercises_path = path.join("exercises");
    if let Err(e) = extract_dir(&EXERCISES_DIR, &exercises_path) {
        eprintln!("{} Failed to extract exercises: {}", "Error:".red(), e);
        process::exit(1);
    }
    println!("  {} exercises/", "ok".green());

    // Extract solutions
    let solutions_path = path.join("solutions");
    if let Err(e) = extract_dir(&SOLUTIONS_DIR, &solutions_path) {
        eprintln!("{} Failed to extract solutions: {}", "Error:".red(), e);
        process::exit(1);
    }
    println!("  {} solutions/", "ok".green());

    // Extract hints
    let hints_path = path.join("hints");
    if let Err(e) = extract_dir(&HINTS_DIR, &hints_path) {
        eprintln!("{} Failed to extract hints: {}", "Error:".red(), e);
        process::exit(1);
    }
    println!("  {} hints/", "ok".green());

    println!(
        "\n{} Project initialized successfully!",
        "ok".green().bold()
    );
    println!("\nTo get started:");
    println!("  {} {}", "cd".cyan(), path.display());
    println!("  {}", "lisplings".cyan());
}

/// Extract an embedded directory to the filesystem
fn extract_dir(dir: &Dir, target: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(target)?;

    for entry in dir.entries() {
        let entry_path = target.join(entry.path().file_name().unwrap_or_default());

        match entry {
            include_dir::DirEntry::Dir(subdir) => {
                extract_dir(subdir, &entry_path)?;
            }
            include_dir::DirEntry::File(file) => {
                if let Some(parent) = entry_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&entry_path, file.contents())?;
            }
        }
    }

    Ok(())
}

/// Watch mode: continuously monitor exercises and provide feedback
fn cmd_watch(exercises: &[Exercise]) {
    println!("\n{}", "Welcome to lisplings watch mode!".green().bold());
    println!(
        "{}",
        "Edit exercises in your editor. Progress updates automatically.".dimmed()
    );
    println!("{}", "Press Ctrl+C to exit.\n".dimmed());

    let mut cache = StatusCache::new();

    // Warm up cache with progress indicator
    print!("{}", "Checking exercises...".dimmed());
    use std::io::Write;
    std::io::stdout().flush().ok();

    for (i, ex) in exercises.iter().enumerate() {
        cache.get_status(ex);
        if (i + 1) % 5 == 0 {
            print!(".");
            std::io::stdout().flush().ok();
        }
    }
    println!(" {}", "done".green());

    // Initial display
    let mut current_exercise_name = String::new();
    display_current_exercise(exercises, &mut current_exercise_name, &mut cache);

    loop {
        std::thread::sleep(Duration::from_millis(250));

        // Check files every 250ms
        let mut changed = false;
        for ex in exercises {
            if let Ok(meta) = std::fs::metadata(&ex.path) {
                if let Ok(mtime) = meta.modified() {
                    if mtime.elapsed().unwrap_or(Duration::from_secs(1000))
                        < Duration::from_millis(500)
                    {
                        changed = true;
                        break;
                    }
                }
            }
        }

        if changed {
            clear_screen();
            display_current_exercise(exercises, &mut current_exercise_name, &mut cache);
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    use std::io::Write;
    std::io::stdout().flush().ok();
}

fn display_current_exercise(
    exercises: &[Exercise],
    previous_name: &mut String,
    cache: &mut StatusCache,
) {
    // Find first incomplete exercise
    let current = exercises.iter().find(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current {
        Some(exercise) => {
            let status = cache.get_status(exercise);

            // Check if we moved to a new exercise
            if !previous_name.is_empty() && *previous_name != exercise.name {
                println!(
                    "{} Completed {}!\n",
                    "ok".green().bold(),
                    previous_name.cyan()
                );
            }
            *previous_name = exercise.name.clone();

            println!(
                "{} {}\n",
                "Current exercise:".green().bold(),
                exercise.name.cyan()
            );

            // Show file path (absolute)
            let abs_path = std::env::current_dir()
                .map(|cwd| cwd.join(&exercise.path))
                .unwrap_or_else(|_| exercise.path.clone());
            println!("  File: {}", abs_path.display().to_string().dimmed());

            match status {
                ExerciseStatus::NotDone => {
                    println!("  Status: {}\n", "Waiting for you to start...".yellow());

                    // Show exercise description from ; comment header
                    if let Ok(content) = std::fs::read_to_string(&exercise.path) {
                        let header: Vec<&str> = content
                            .lines()
                            .take_while(|l| l.starts_with(';') || l.is_empty())
                            .filter(|l| !l.contains("I AM NOT DONE"))
                            .collect();
                        for line in header {
                            println!("  {}", line.dimmed());
                        }
                    }

                    println!();
                    println!(
                        "  {}",
                        "Delete the '; I AM NOT DONE' line when you've solved it.".yellow()
                    );
                }
                ExerciseStatus::CompileError => {
                    println!("  Status: {}\n", "Error".red().bold());

                    if let Err(e) = runner::compile(&exercise.path) {
                        for line in e.lines().take(15) {
                            println!("  {}", line.red());
                        }
                    }
                }
                ExerciseStatus::TestFail => {
                    println!("  Status: {}\n", "Tests Failed".red().bold());

                    match runner::run_tests(&exercise.path) {
                        Ok(output) | Err(output) => {
                            for line in output.lines().take(20) {
                                if line.contains("FAIL") {
                                    println!("  {}", line.red());
                                } else {
                                    println!("  {}", line);
                                }
                            }
                        }
                    }
                }
                ExerciseStatus::Done => {
                    println!("  Status: {}", "Done".green());
                }
            }

            println!();
            println!("  {} lisplings hint", "Hint:".cyan());
            show_progress(exercises, cache);
        }
        None => {
            clear_screen();
            println!("\n{}", "=".repeat(50).green());
            println!(
                "{}",
                "  Congratulations! You've completed all exercises!"
                    .green()
                    .bold()
            );
            println!("{}\n", "=".repeat(50).green());
            show_progress(exercises, cache);
            println!("\n{}", "You're now a Lisp programmer!".cyan().bold());
            process::exit(0);
        }
    }
}

/// List all exercises
fn cmd_list(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

    println!("\n{}\n", "Lisplings Exercises".green().bold());

    let mut current_topic = String::new();
    for exercise in exercises {
        let topic = exercise
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        if topic != current_topic {
            println!("\n  {}", topic.cyan().bold());
            current_topic = topic.to_string();
        }

        let status = cache.get_status(exercise);
        let status_icon = match status {
            ExerciseStatus::Done => "ok".green(),
            ExerciseStatus::NotDone => " .".yellow(),
            ExerciseStatus::CompileError => "!!".red(),
            ExerciseStatus::TestFail => "!!".red(),
        };

        println!("    {} {}", status_icon, exercise.name);
    }

    println!();
    show_progress(exercises, &mut cache);
}

/// Show hint for an exercise
fn cmd_hint(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::new();
    let name_provided = name.is_some();
    let exercise = match &name {
        Some(n) => exercises.iter().find(|e| &e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                cache.get_status(e),
                ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
            )
        }),
    };

    match exercise {
        Some(ex) => {
            let hint_path = ex.hint_path();
            if hint_path.exists() {
                match std::fs::read_to_string(&hint_path) {
                    Ok(content) => {
                        println!("\n{} {}\n", "Hint for".green(), ex.name.cyan());
                        println!("{}", content);
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error reading hint:".red(), e);
                    }
                }
            } else {
                println!("\n{} {}", "No hint available for".yellow(), ex.name.cyan());
                println!("Hint file not found: {}", hint_path.display());
            }
        }
        None => {
            if name_provided {
                eprintln!("{}", "Exercise not found".red());
            } else {
                println!("{}", "All exercises complete! No hints needed.".green());
            }
        }
    }
}

/// Reset an exercise
fn cmd_reset(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::new();
    let exercise = match name {
        Some(n) => exercises.iter().find(|e| e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                cache.get_status(e),
                ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
            )
        }),
    };

    match exercise {
        Some(ex) => {
            let solution_path = ex.solution_path();
            if solution_path.exists() {
                match std::fs::read_to_string(&ex.path) {
                    Ok(mut content) => {
                        if !content.contains("; I AM NOT DONE") {
                            // Add marker back after the header comment block
                            let insert_pos = content
                                .lines()
                                .take_while(|l| l.starts_with(';') || l.is_empty())
                                .map(|l| l.len() + 1)
                                .sum::<usize>();
                            content.insert_str(insert_pos, "\n; I AM NOT DONE\n");
                            if std::fs::write(&ex.path, content).is_ok() {
                                println!("{} {}", "Reset".green(), ex.name.cyan());
                            }
                        } else {
                            println!("{} is already in incomplete state", ex.name.cyan());
                        }
                    }
                    Err(e) => eprintln!("{} {}", "Error reading exercise:".red(), e),
                }
            } else {
                println!("{}", "No original version found. Cannot reset.".yellow());
            }
        }
        None => {
            eprintln!("{}", "Exercise not found".red());
        }
    }
}

/// Verify all exercises
fn cmd_verify(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

    println!("\n{}\n", "Verifying all exercises...".green().bold());

    for exercise in exercises {
        let status = cache.get_status(exercise);
        let status_str = format_status(&status);
        let icon = match status {
            ExerciseStatus::Done => "ok".green(),
            _ => "!!".red(),
        };
        println!("  {} {} - {}", icon, exercise.name, status_str);
    }

    println!();
    show_progress(exercises, &mut cache);
}

/// Skip to next exercise
fn cmd_next(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

    let current_idx = exercises.iter().position(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current_idx {
        Some(idx) if idx + 1 < exercises.len() => {
            let next = &exercises[idx + 1];
            println!("Skipping to: {}", next.name.cyan());
        }
        _ => {
            println!("{}", "No more exercises to skip to.".yellow());
        }
    }
}

fn format_status(status: &ExerciseStatus) -> colored::ColoredString {
    match status {
        ExerciseStatus::Done => "Done".green(),
        ExerciseStatus::NotDone => "Not Done".yellow(),
        ExerciseStatus::CompileError => "Error".red(),
        ExerciseStatus::TestFail => "Test Failed".red(),
    }
}

fn show_progress(exercises: &[Exercise], cache: &mut StatusCache) {
    let done = exercises
        .iter()
        .filter(|e| matches!(cache.get_status(e), ExerciseStatus::Done))
        .count();
    let total = exercises.len();
    let pct = if total > 0 {
        (done as f64 / total as f64 * 100.0) as usize
    } else {
        0
    };

    let bar_width = 30;
    let filled = if total > 0 {
        (done * bar_width) / total
    } else {
        0
    };
    let empty = bar_width - filled;

    println!(
        "Progress: [{}{}] {}/{} ({}%)",
        "=".repeat(filled).green(),
        "-".repeat(empty),
        done,
        total,
        pct
    );
}
