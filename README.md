# Xianjing_Huang_Mini_Proj_7
[![CI](https://github.com/nogibjj/Xianjing_Huang_Mini_Proj_7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Xianjing_Huang_Mini_Proj_7/actions/workflows/CI.yml)

### Directory Tree Structure
```
Xianjing_Huang_Mini_Proj_7/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/
│       └── CI.yml
├── add/
│   ├── src/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── Makefile
├── data/
│   └── customer_new.csv
├── imgs/
├── sqlite/
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── target/
│   │   └── test.rs
│   ├── tests/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Makefile
│   └── my_database.db
├── .gitignore
└── README.md
```
`Cargo.toml`: Package and dependencies.

`sqlite/src/lib.rs`: 
- `create_table`: Create a table.
- `load_data_from_csv`: Load data from a file path to a table.
- `drop_table`: Drop a table.
- `query_exec`: Read records in table.
- `update_exec`: Update a record in the table.
- `insert_exec`: Insert a record in the table.
- `delete_exec`: Delete a record in the table.

`sqlite/src/main.rs`: 
Handle CLI features by parsing input as one of 7 options: Create, Query, Drop, Load, Insert, Update, Delete. 

`sqlite/tests/test.rs`: Test for lib.

`Makefile`: Defines scripts for common project tasks such as cargo check, cargo build.

`CI.yml`: Defines the GitHub Actions workflow for Check, Format, Test, Release, Upload Binary Artifact.

### Requirements
* Package a Python script with setuptools or a similar tool
* Include a user guide on how to install and use the tool
* Include communication with an external or internal database (NoSQL, SQL, etc) [If you use Rust you can skip the DB part]


### Preparation
1. Open codespaces
2. Install cargo
>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
>cargo --version

### Build
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
![2](/imgs/002.png)
![3](/imgs/003.png)
![4](/imgs/004.png)
![5](/imgs/005.png)

### Project Breakdown
In this project, I use rust to realize SQLite operation and use CLI(Command-Line Tool) features.

See main.rs for a commented example of how we make our CLI. Note that by using clap over standard library options (std::env for rust or argparse for python) we get a lot of free functionality like help menu guides.

Add compiled binary (--release) to your path, this way you can use your CLI normally without using:

>cargo run -- -\<flag> argument

and instead:

>sqlite -c users

Command to add compiled binary to path for use:

*If in codespaces*

>export PATH=$PATH:/workspaces/\<REPO_NAME>/sqlite/target/release

Once you build your CLI binary you can the use it like a regular CLI:
![6](/imgs/006.png)

#### CLI demo
![7](/imgs/007.png)
`sqlite -c table1` Create Table table1.

`sqlite -l table1 ../data/customer_new.csv` Load data into table 'table1' from '../data/customer_new.csv'.

`sqlite -q "SELECT * FROM table1;"` Query: SELECT * FROM table1;

`sqlite -i table1 11 Remi female Durham` Insert person with ID '11' into the 'table1' table.

`sqlite -u table1 11 Remi female 'Los Angeles'` Updating record in table 'table1' with ID 11.

`sqlite -x table1 11` Delete record in table 'table1' with ID 11

`sqlite -d table1` Table 'table1' dropped.

### Binary Download Link
https://github.com/nogibjj/Xianjing_Huang_Mini_Proj_7/actions/runs/11467682732/artifacts/2090141328

The binary location is what gets uploaded as an artifact in the yml file.
![8](/imgs/008.png)

### Continuous Integration (CI/CD Pipeline)

![9](/imgs/009.png)


