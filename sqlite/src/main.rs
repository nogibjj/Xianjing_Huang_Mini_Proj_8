// fn main() {
//     println!("Hello, world!");
// }
//this will be the CLI portion of the project where we accept
//user defined arguments and call lib.rs logic to handle them
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::UpdateFields;
use sqlite::{
    create_exec, create_table, delete_exec, drop_table, extract, load_data_from_csv, read_exec,
    update_exec,
};

//Here we define a struct (or object) to hold our CLI arguments
//for #[STUFF HERE] syntax, these are called attributes.
//for now, they define behavior for elements in rust.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//Think of a struct as a class which makes objects in python
//This is designed to generate an object out of the CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//An enum is a type in rust that can have multiple exauhstive and mutually exclusive options
//Create, Load, Query, Drop, Insert, Update, Delete

#[derive(Debug, Subcommand)]
//By separating out the commands as enum types we can easily match what the user is
//trying to do in main
enum Commands {
    ///Extract a url to a file path
    #[command(alias = "e", short_flag = 'e')]
    Extract {},
    ///Pass a table name to create a table
    /// "sqlite -c table1"
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },
    ///Pass a table name and a file path to load data from csv
    /// "sqlite -l table1 data/fifa_countries_audience.csv"
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
    ///Pass a table name to read data
    /// "sqlite -q table1"
    #[command(alias = "q", short_flag = 'q')]
    Query { table_name: String },
    ///Pass a table name to drop
    /// "sqlite -d table1"
    #[command(alias = "d", short_flag = 'd')]
    Drop { table_name: String },
    ///Pass a new record to insert
    /// "sqlite -i table1 TestCountry1 TestConfederation2 0.1 0.2 0.3"
    #[command(alias = "i", short_flag = 'i')]
    Insert {
        table_name: String,
        country: String,
        confederation: String,
        population_share: f64,
        tv_audience_share: f64,
        gdp_weighted_share: f64,
    },
    ///Pass a new record to update
    /// "sqlite -u table1 192 TestCountry1 TestConfederation2 1.1 2.2 3.3"
    #[command(alias = "u", short_flag = 'u')]
    Update {
        table_name: String,
        id: i32,
        new_country: Option<String>,
        new_confederation: Option<String>,
        new_population_share: Option<f64>,
        new_tv_audience_share: Option<f64>,
        new_gdp_weighted_share: Option<f64>,
    },
    ///Delete a record by id
    /// "sqlite -x table1 192"
    #[command(alias = "x", short_flag = 'x')]
    Delete { table_name: String, id: i32 },
}

fn main() -> Result<()> {
    //Here we parse the CLI arguments and store them in the args object
    let args = Cli::parse();
    //generate connection
    let conn = Connection::open("my_database.db")?;

    //Here we can match the behavior on the subcommand and call our lib logic
    match args.command {
        Commands::Extract {} => {
            println!("Extract a url to a file path");
            extract().expect("Failed to extract");
        }
        Commands::Create { table_name } => {
            println!("Creating Table {}", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { table_name } => {
            println!("Read Table: {}", table_name);
            read_exec(&conn, &table_name).expect("Failed to execute query");
        }
        Commands::Drop { table_name } => {
            println!("Deleting: {}", table_name);
            drop_table(&conn, &table_name).expect("Failed to drop table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_data_from_csv(&conn, &table_name, &file_path)
                .expect("Failed to load data from csv");
        }
        Commands::Insert {
            table_name,
            country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share,
        } => {
            println!(
                "Insert record in table '{}' with country {}, confederation {}, population_share {}, tv_audience_share {}, gdp_weighted_share {}",
                table_name, country, confederation, population_share, tv_audience_share, gdp_weighted_share
            );
            create_exec(
                &conn,
                &table_name,
                &country,
                &confederation,
                population_share,
                tv_audience_share,
                gdp_weighted_share,
            )
            .expect("Failed to insert record");
        }
        Commands::Update {
            table_name,
            id,
            new_country,
            new_confederation,
            new_population_share,
            new_tv_audience_share,
            new_gdp_weighted_share,
        } => {
            println!("Updating record in table '{}' with ID {}", table_name, id);
            let fields = UpdateFields {
                new_country: new_country.as_deref(),
                new_confederation: new_confederation.as_deref(),
                new_population_share,
                new_tv_audience_share,
                new_gdp_weighted_share,
            };
            update_exec(&conn, &table_name, id, fields).expect("Failed to update record");
        }
        Commands::Delete { table_name, id } => {
            println!("Delete record in table '{}' with ID {}", table_name, id);
            delete_exec(&conn, &table_name, id).expect("Failed to delete record");
        }
    }
    Ok(())
}
