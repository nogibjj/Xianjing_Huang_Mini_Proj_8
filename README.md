# Xianjing_Huang_Mini_Proj_8
[![CI](https://github.com/nogibjj/Xianjing_Huang_Mini_Proj_7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Xianjing_Huang_Mini_Proj_7/actions/workflows/CI.yml)

### Python Project Structure
```
.
├── mylib
│   ├── extract.py            # Extract a dataset from a URL
│   ├── query.py              # CRUD and log funcitons
│   └── transform_load.py     # Transforms and Loads data into the local SQLite3 database
├── main.py                   # Perform CRUD, add log, track running time and memory usage
└── test_main.py              # test main
```
### RUST Project Structure
```
.
├── sqlite/
│   ├── data/
│   ├── src/
│   │   ├── lib.rs            # ETL-CRUD funcitons
│   │   └── main.rs           # Create a list of commands, 
│   │                           track running time and memory usage
│   ├── tests/                # Test for lib
│   ├── Cargo.lock            
│   ├── Cargo.toml            # Package and dependencies
│   ├── Makefile              # Defines scripts for common project tasks 
│   │                           such as cargo check, cargo build.
│   └── my_database.db
```
### Continuous Integration (CI/CD Pipeline)
```
.
├── .github/
│   └── workflows/
│       ├── PythonCI.yml      # Defines the GitHub Actions workflow for 
│       │                       Install, Lint, Format, Test, generate_and_push
│       └── RustCI.yml        # Defines the GitHub Actions workflow for 
                                Check, Format, Test, Release, Upload
                                Binary Artifact
```

### Requirements
* Take an existing Python script for data processing
* Rewrite it in Rust
* Highlight improvements in speed and resource usage

## Performance Comparison: Python vs. Rust

| Metric               | Python                     | Rust                        | Difference                        |
|----------------------|----------------------------|-----------------------------|-----------------------------------|
| **Execution Time**   | 0.12 seconds      | 0.28 seconds       | Python is faster by ~0.16 s      |
| **Memory Usage**     | 30.64 MB                    | 17.33 MB                     | Rust uses ~13.31 MB less memory   |

### Performance comparison report (markdown)
1. Set Executable Permissions:
2. Run the Make Command:
   ```bash
   chmod +x generate_report.sh
   ./generate_report.sh
   ```
[Performance Report](performance_report.md)

## Summary of Improvements
The Python version completes the process a little bit faster while Rust uses less memory for this particular dataset. 

Rust, generally outperforms Python for CPU-bound tasks and is optimized for memory and performance in long-running applications or complex, multi-threaded scenarios. However, Python can sometimes be faster for certain types of tasks, especially I/O-bound or simple computational tasks, due to its fast startup, efficient libraries, and ability to handle asynchronous operations effectively. **In this case**, Python being faster than Rust is likely due to I/O-heavy operations or quick, short tasks.

For resource usage, Rust has a clear advantage due to its unique memory management model, which eliminates the need for a garbage collector. Python, on the other hand, uses dynamic typing and relies on garbage collection, which can lead to higher memory usage and occasional pauses during cleanup cycles. **In this case**, we see that Rust used significantly less memory than Python for the same task. This difference—around 13.31 MB less memory in Rust—demonstrates Rust’s efficiency in resource management.

In essence, Rust is better suited for scenarios where performance and efficient resource management are critical, like systems programming and high-performance applications. Python, however, remains highly productive for rapid development and prototyping, especially in areas like data science, where speed and memory usage can be traded off for ease of development.

### Rust Preparation
1. Open codespaces
2. Install cargo
>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
>cargo --version

### Rust Build
Rust is a compiled language and so to run programs you will need to compile the file first. This is done a few ways:

>cargo check

* a quick compile that works off of a cached version to only rebuild changes

>cargo build

* an unoptimized build which has debug functionality

>cargo fmt

* automatically formats Rust code according to the standard Rust style guidelines.

>cargo clippy

* It’s a linter that catches all those little things you might overlook—warnings, performance issues, or just some best practice suggestions. 

>cargo test
* is your tool for running tests in your Rust code.

>cargo build --release

* generates an optimized binary in your target/release/\<projectname> directory

![0](/imgs/000.png)
![1](/imgs/001.png)


### Project Breakdown
In this project, I rewrite SQLite operation in Rust (/sqlite), including ETL (Extract, Transform, Load), CRUD (Create, Read, Update, Delete).

I perform a list of commands and use sysinfo to track running time and memory usage (while using psutil in Python script for tracking).

See result in [Performance Report](performance_report.md)


### Binary Download Link
https://github.com/nogibjj/Xianjing_Huang_Mini_Proj_8/actions/runs/11542766983/artifacts/2109506629

The binary location is what gets uploaded as an artifact in the yml file.
![2](/imgs/002.png)
