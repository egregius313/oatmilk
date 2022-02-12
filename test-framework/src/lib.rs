// use std::io::Write;
use std::process::{Command, ExitStatus};

use tempfile::TempDir;

/// Test expectation
pub enum ExpectedPassFail {
    CompileFail,
    PassValue(ExitStatus),
    PassOutput(String),
    Failure,
}

pub struct TestCase {
    filename: String,
    pass_fail: ExpectedPassFail,
}

/// TODO: Replace with compilation error from oatmilk compiler
#[derive(Debug, PartialEq)]
struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test failure")
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Test failure"
    }
}

impl TestCase {
    pub fn new(filename: String, pass_fail: ExpectedPassFail) -> TestCase {
        TestCase {
            filename,
            pass_fail,
        }
    }

    pub fn compile(self, tmpdir: TempDir) -> Result<bool, Error> {
        let exec_path = tmpdir.path().join("a.out");
        let command = Command::new("oatmilk")
            .arg(self.filename)
            .arg("-o")
            .arg(exec_path);

        match self.pass_fail {
            ExpectedPassFail::CompileFail => {
                if let Ok(status) = command.status() {
                    if status.success() {
                        return Ok(!status.success());
                    }
                }
            }
            _ => {
                if let Ok(status) = command.status() {
                    return Ok(status.success());
                }
            }
        }
        Err(Error)
    }

    pub fn run(self, tmpdir: TempDir) -> Result<bool, Error> {
        let exec_path = tmpdir.path().join("a.out");
        let command = Command::new(exec_path);
        use ExpectedPassFail::*;
        match self.pass_fail {
            PassValue(i) => {
                if let Ok(status) = command.status() {
                    Ok(i == status)
                } else {
                    Ok(false)
                }
            }
            PassOutput(desired_output) => {
                if let Ok(output) = command.output() {
                    if let text = output.stdout {
                        if let Ok(output_text) = String::from_utf8(text) {
                            return Ok(output_text.contains(&desired_output));
                        }
                    }
                }
                return Err(Error);
            }
            Failure => {
                if let Ok(status) = command.status() {
                    return Ok(!status.success());
                }
                return Ok(false);
            }
        }
    }

    pub fn exec_test_case(self) -> bool {
        let tmpdir = TempDir::new().unwrap();

        if self.compile(tmpdir) == Ok(true) {
            if self.run(tmpdir) == Ok(true) {
                return true;
            }
        }
        false
    }
}

struct TestSuite(Vec<TestCase>);

impl TestSuite {
    pub fn new(tests: Vec<TestCase>) -> Self {
        Self(tests)
    }

    pub fn run_all(&self) {
        for testcase in self.0.iter() {
            if testcase.exec_test_case() {
                println!("Test case {}: passed", testcase.filename);
            } else {
                println!("Test case {}: failed", testcase.filename);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
