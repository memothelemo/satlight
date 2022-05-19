#![allow(clippy::or_fun_call)]

use ansi_term::Style;
use std::{env::args, path::PathBuf};

mod parser;
mod project;
mod typeck;

pub type TestResult = Result<(), String>;

const INDENT: &str = "    ";

lazy_static::lazy_static! {
    pub static ref TEST_REAL_PATH: PathBuf = PathBuf::from("./test-suite/samples")
        .canonicalize()
        .map_err(|e| e.to_string())
        .unwrap();
}

#[derive(Debug)]
pub struct StringError {
    pub message: String,
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for StringError {}

pub struct TestEnv {
    before_each_calls: Vec<Box<dyn FnMut() + 'static>>,
    filtered: Vec<String>,
    stack: Vec<&'static str>,
    indentation: usize,
    total_tests: usize,
    failed_tests: usize,
}

impl TestEnv {
    pub fn sample_path(&self) -> PathBuf {
        PathBuf::from("./test-suite/samples")
    }

    fn can_print(&self) -> bool {
        if self.filtered.is_empty() || self.stack.is_empty() {
            return true;
        }

        for stack in self.stack.iter() {
            if self.filtered.contains(&stack.to_string()) {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn push_indentation(&mut self) {
        self.indentation += 1;
    }

    #[inline(always)]
    fn pop_indentation(&mut self) {
        self.indentation -= 1;
    }

    #[inline(always)]
    fn make_indentation_str(&self) -> String {
        if self.indentation == 0 {
            String::new()
        } else {
            INDENT.repeat(self.indentation)
        }
    }

    pub fn before_each(&mut self, callback: impl FnMut() + 'static) {
        self.before_each_calls.push(Box::new(callback));
    }

    pub fn describe(&mut self, text: &'static str) {
        // execute beforeEach calls
        self.stack.push(text);

        if !self.can_print() {
            return;
        }

        self.before_each_calls.iter_mut().for_each(|v| v());
        eprintln!(
            "{}- {}",
            self.make_indentation_str(),
            Style::new().bold().paint(text)
        );
        self.push_indentation();
    }

    pub fn fail(&mut self, text: &str) {
        if !self.can_print() {
            return;
        }

        use ansi_term::Color::*;
        eprintln!(
            "{}[{}] {}",
            self.make_indentation_str(),
            Style::new().bold().fg(Red).paint("X"),
            text
        );
        self.pop_indentation();
    }

    pub fn describe_end(&mut self) {
        self.stack.pop();

        if !self.can_print() {
            return;
        }

        self.pop_indentation();
    }

    pub fn it(&mut self, text: &str, mut callback: impl FnMut() -> TestResult) {
        if !self.can_print() {
            return;
        }

        use ansi_term::Color::*;
        self.total_tests += 1;

        let now = std::time::Instant::now();
        let result = callback();
        let elapsed = now.elapsed();

        #[cfg(feature = "debug")]
        eprintln!(
            "{0} {1} {0}",
            Style::new().bold().fg(Yellow).paint("^^^^^^^^^^"),
            text
        );

        match result {
            Ok(..) => {
                #[cfg(not(feature = "debug"))]
                eprintln!(
                    "{}[{}] {} ({:.2?})",
                    self.make_indentation_str(),
                    Style::new().bold().fg(Green).paint("/"),
                    text,
                    elapsed
                );
            }
            Err(err) => {
                self.failed_tests += 1;
                eprintln!(
                    "{}[{}] {} ({:.2?})\n",
                    self.make_indentation_str(),
                    Style::new().bold().fg(Red).paint("X"),
                    text,
                    elapsed
                );
                eprintln!(
                    "{}{}{}: {}\n",
                    self.make_indentation_str(),
                    INDENT,
                    Style::new().bold().paint("Reason"),
                    err
                );
            }
        };
    }

    pub fn done(&mut self) {
        self.stack.pop();
        self.before_each_calls.clear();
        self.indentation = 0;
    }
}

pub trait TestCase {
    fn name(&self) -> &'static str;
    fn on_run(&self, env: &mut TestEnv);
}

macro_rules! tasks {
    [ $( $task:expr ),* ] => {
        vec![ $( Box::new($task) ),* ]
    };
}

fn main() {
    use ansi_term::Color::*;

    #[cfg(feature = "no-out")]
    eprintln!(
        "{}",
        Style::new()
            .bold()
            .fg(Yellow)
            .paint("!! Running in no output mode !!")
    );

    let mut filtered_tests = Vec::new();
    let mut args = args();

    if args.len() > 1 {
        args.next();
        for arg in args {
            filtered_tests.push(arg);
        }
    }

    let tasks: Vec<Box<dyn TestCase>> =
        tasks![project::ProjectCase, parser::ParserCase, typeck::TypeckCase];

    let mut env = TestEnv {
        before_each_calls: Vec::new(),
        filtered: filtered_tests,
        stack: Vec::new(),
        indentation: 0,
        failed_tests: 0,
        total_tests: 0,
    };

    for task in tasks.iter() {
        env.describe(task.name());
        task.on_run(&mut env);
        env.done();
    }

    eprintln!(
        "\n{}",
        Style::new()
            .bold()
            .paint(format!("Out of the total of {} tests:", env.total_tests))
    );

    if env.failed_tests > 0 {
        eprintln!(
            "{}- {}",
            INDENT,
            Style::new().bold().fg(Green).paint(format!(
                "{} tests passed",
                env.total_tests - env.failed_tests
            ))
        );
        eprintln!(
            "{}- {}",
            INDENT,
            Style::new()
                .bold()
                .fg(Red)
                .paint(format!("{} tests failed", env.failed_tests))
        );
    } else {
        eprintln!(
            "{}- {}",
            INDENT,
            Style::new()
                .bold()
                .fg(Green)
                .paint("All of them are passed",)
        );
    }

    std::process::exit(match env.failed_tests > 0 {
        true => 1,
        false => 0,
    })
}
