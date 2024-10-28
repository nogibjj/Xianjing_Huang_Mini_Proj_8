use csv::ReaderBuilder; //for loading from csv
use reqwest::blocking;
use rusqlite::ToSql;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;

// Extract a url to a file path
pub fn extract() -> Result<(), Box<dyn Error>> {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/fifa/fifa_countries_audience.csv";
    let directory = "data";
    let file_path = "data/fifa_countries_audience.csv";

    // Create the directory if it doesn't exist
    if !Path::new(directory).exists() {
        create_dir_all(directory)?;
    }

    let response = blocking::get(url)?;
    let mut dest = File::create(file_path)?;
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut dest)?;

    println!("File has been downloaded to {}", file_path);
    Ok(())
}

// Load data from a file path to a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    //Box<dyn Error> is a trait object that can represent any error type
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (country,confederation,population_share,tv_audience_share,gdp_weighted_share) VALUES (?, ?, ?, ?, ?)",
        table_name
    );

    for result in rdr.records() {
        let record = result?;
        let country = &record[0];
        let confederation = &record[1];
        let population_share: f64 = record[2].parse()?;
        let tv_audience_share: f64 = record[3].parse()?;
        let gdp_weighted_share: f64 = record[4].parse()?;

        conn.execute(
            &insert_query,
            params![
                country,
                confederation,
                population_share,
                tv_audience_share,
                gdp_weighted_share
            ],
        )?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT,
            confederation TEXT,
            population_share REAL,
            tv_audience_share REAL,
            gdp_weighted_share REAL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(()) //returns nothing except an error if it occurs
}

// Drop a table
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Read records in table
pub fn read_exec(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    let query_string = format!("SELECT * FROM {}", table_name);
    let mut stmt = conn.prepare(&query_string)?;

    // Use query_map to handle multiple rows
    let rows: Vec<_> = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let country: String = row.get(1)?;
            let confederation: String = row.get(2)?;
            let population_share: f64 = row.get(3)?;
            let tv_audience_share: f64 = row.get(4)?;
            let gdp_weighted_share: f64 = row.get(5)?;
            Ok((
                id,
                country,
                confederation,
                population_share,
                tv_audience_share,
                gdp_weighted_share,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // Iterate over the rows and print the results
    // for row in rows {
    //     let (id, country, confederation, population_share, tv_audience_share, gdp_weighted_share) =
    //         row?;
    //     println!(
    //         "ID: {}, Country: {}, Confederation: {}, Population Share: {}, TV Audience Share: {}, GDP Weighted Share: {}",
    //         id, country, confederation, population_share, tv_audience_share, gdp_weighted_share
    //     );
    // }

    // Limit output to first 10 rows
    let max_rows = 10;
    for row in rows.iter().take(max_rows) {
        let (id, country, confederation, population_share, tv_audience_share, gdp_weighted_share) =
            row;
        println!(
            "ID: {}, Country: {}, Confederation: {}, Population Share: {}, TV Audience Share: {}, GDP Weighted Share: {}",
            id, country, confederation, population_share, tv_audience_share, gdp_weighted_share
        );
    }

    if rows.len() > max_rows {
        println!("...\nand {} more rows not shown", rows.len() - max_rows);
    }

    Ok(())
}

pub struct UpdateFields<'a> {
    pub new_country: Option<&'a str>,
    pub new_confederation: Option<&'a str>,
    pub new_population_share: Option<f64>,
    pub new_tv_audience_share: Option<f64>,
    pub new_gdp_weighted_share: Option<f64>,
}

// Update a record in the table
pub fn update_exec(
    conn: &Connection,
    table_name: &str,
    id: i32,
    fields: UpdateFields,
) -> Result<(), Box<dyn Error>> {
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn ToSql>> = Vec::new(); // Vector to hold owned params

    if let Some(country) = fields.new_country {
        updates.push("country = ?");
        params.push(Box::new(country.to_string()));
    }
    if let Some(confederation) = fields.new_confederation {
        updates.push("confederation = ?");
        params.push(Box::new(confederation.to_string()));
    }
    if let Some(population_share) = fields.new_population_share {
        updates.push("population_share = ?");
        params.push(Box::new(population_share));
    }
    if let Some(tv_audience_share) = fields.new_tv_audience_share {
        updates.push("tv_audience_share = ?");
        params.push(Box::new(tv_audience_share));
    }
    if let Some(gdp_weighted_share) = fields.new_gdp_weighted_share {
        updates.push("gdp_weighted_share = ?");
        params.push(Box::new(gdp_weighted_share));
    }

    if updates.is_empty() {
        println!("No fields to update for ID: {}", id);
        return Ok(());
    }

    let update_query = format!(
        "UPDATE {} SET {} WHERE id = ?",
        table_name,
        updates.join(", ")
    );
    params.push(Box::new(id));

    conn.execute(
        &update_query,
        params
            .iter()
            .map(|b| &**b)
            .collect::<Vec<&dyn ToSql>>()
            .as_slice(),
    )?;

    println!(
        "Record with ID '{}' updated successfully in table '{}'.",
        id, table_name
    );
    Ok(())
}

// Insert a record in the table
pub fn create_exec(
    conn: &Connection,
    table_name: &str,
    country: &str,
    confederation: &str,
    population_share: f64,
    tv_audience_share: f64,
    gdp_weighted_share: f64,
) -> Result<(), Box<dyn Error>> {
    let insert_query = format!(
        "INSERT INTO {} (country, confederation, population_share, tv_audience_share, gdp_weighted_share) VALUES (?, ?, ?, ?, ?)",
        table_name
    );

    conn.execute(
        &insert_query,
        params![
            country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share
        ],
    )?;
    println!(
        "Inserted record into the '{}' table successfully!",
        table_name
    );
    Ok(())
}

// Delete a record in the table
pub fn delete_exec(conn: &Connection, table_name: &str, id: i32) -> Result<(), Box<dyn Error>> {
    let delete_query = format!("DELETE FROM {} WHERE id = ?", table_name);
    conn.execute(&delete_query, params![id])?;
    println!(
        "Deleted person with ID '{}' from the '{}' table successfully!",
        id, table_name
    );
    Ok(())
}
