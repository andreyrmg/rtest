extern crate regex;
use regex::Regex;

use std::io::{IoError,TempDir};
use std::io::fs;
use std::io::fs::PathExtensions;
use std::io::process::{Command,ProcessOutput};
use std::os;
use std::path::Path;
use std::str::from_utf8;

struct Compiler {
    build: String,
    build_args: Vec<String>,
    build_arg_working_dir: String,
}

enum CompilationResult {
    Success(Path),
    Error(ProcessOutput),
    Failed(IoError),
}

impl Compiler {
    fn compile(&self, source_file: &Path, working_dir: &Path) -> CompilationResult {
        let mut command = Command::new(&self.build);
        command.args(self.build_args.as_slice())
                .arg(format!("{}{}", &self.build_arg_working_dir, working_dir.display()))
                .arg(source_file);
        println!("Compilation command: {}", command);
        match command.output() {
            Ok(process_output) => {
                if process_output.status.success() {
                    match source_file.filename() {
                        Some(name) => Success(working_dir.join(name).with_extension("")),
                        None => panic!("cannot extract file name from {}", source_file.display()),
                    }
                } else {
                    Error(process_output)
                }
            },
            Err(e) => Failed(e),
        }
    }
}

struct Problem {
    input_regex: String,
    output_rep: String
}

struct ProblemTest {
    input: Path,
    answer: Path,
}

impl Problem {
    fn find_tests(&self, dir: &Path) -> Vec<ProblemTest> {
        if !dir.is_dir() {
            panic!("{} not a dir", dir.display());
        }
        let test_re = Regex::new(self.input_regex.as_slice()).unwrap();
        match fs::readdir(dir) {
            Ok(contents) => {
                let tests = contents.iter()
                        .filter(|&path| path.is_file())
                        .map(|path| path.filename_str().unwrap())
                        .filter(|&s| test_re.is_match(s))
                        .collect::<Vec<&str>>();
                for p in tests.iter() {
                    println!("{}", p);
                }
                vec![]
            },
            Err(e) => panic!("cannot read directory: {}", dir.display()),
        }
    }
}

fn main() {
    println!("The current directory: {}", os::getcwd().display());

    let source_path = Path::new("solution.pas");
    let abs_source_path = os::make_absolute(&source_path);
    if !abs_source_path.is_file() {
        panic!("{} not a file", abs_source_path.display());
    }

    let working_dir = TempDir::new("rtest").unwrap();

    println!("Working directory: {}", working_dir.path().display());

    let c = Compiler {
        build: "fpc".to_string(),
        build_args: vec!["-So".to_string(), "-XS".to_string()],
        build_arg_working_dir: "-FE".to_string(),
    };
    // match c.compile(&abs_source_path, working_dir.path()) {
    //     Success(binary) => println!("Binary: {}", binary.display()),
    //     Error(process_output) => {
    //         let output = from_utf8(process_output.output.as_slice()).unwrap_or("Invalid UTF-8 sequence");
    //         let error = from_utf8(process_output.error.as_slice()).unwrap_or("Invalid UTF-8 sequence");
    //         println!("Compilation error.");
    //         println!("status: {}", process_output.status);
    //         println!("output:\n{}", output);
    //         println!("error:\n{}", error);
    //     },
    //     Failed(e) => panic!("compiler failed: {}", e),
    // };

    let p = Problem {
        input_regex: r"(?i)(?P<test>\d+).in".to_string(),
        output_rep: "$test.out".to_string(),
    };

    p.find_tests(&Path::new("/Users/andrey/Desktop/Gorod/A/tests"));

}
