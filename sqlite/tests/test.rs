#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
    use std::path::Path;
    use sqlite::UpdateFields;
    use sqlite::{
        create_exec, create_table, delete_exec, drop_table, extract, load_data_from_csv, read_exec,
        update_exec,
    };
    use std::error::Error;
    use std::sync::Mutex;

    static DB_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn setup_db() -> Connection {
        let conn = Connection::open("my_database.db").expect("Failed to open database");
        create_table(&conn, "test_table").expect("Failed to create table");
        conn
    }

    fn teardown_db(conn: &Connection) {
        drop_table(conn, "test_table").expect("Failed to drop table");
    }

    #[test]
    fn test_extract() {
        let result = extract();
        assert!(result.is_ok());
        let file_path = "data/fifa_countries_audience.csv";
        assert!(Path::new(file_path).exists());

        // if Path::new(file_path).exists() {
        //     fs::remove_file(file_path).expect("Failed to remove test file");
        // }
    }

    #[test]
    fn test_create_table() {
        let _lock = DB_MUTEX.lock().unwrap(); // default: multi-thread
        let conn = setup_db();
        let table_check_query =
            "SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'";
        let mut stmt = conn.prepare(table_check_query).unwrap();
        let result_iter = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
        let result: Vec<String> = result_iter.filter_map(Result::ok).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "test_table");
        teardown_db(&conn);
    }

    #[test]
    fn test_load_data_from_csv() -> Result<(), Box<dyn Error>> {
        let _lock = DB_MUTEX.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let conn = setup_db();
        let _ = extract();
        let csv_path = "data/fifa_countries_audience.csv";
        let result = load_data_from_csv(&conn, "test_table", csv_path);
        assert!(result.is_ok());
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_table", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 191);

        teardown_db(&conn);
        Ok(())
    }

    #[test]
    fn test_create_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        let result = create_exec(
            &conn,
            "test_table",
            "TestCountry",
            "TestConfederation",
            1.1,
            2.2,
            3.3,
        );
        assert!(result.is_ok());
        teardown_db(&conn);
    }

    #[test]
    fn test_update_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        
        create_exec(
            &conn,
            "test_table",
            "TestCountry",
            "TestConfederation",
            1.1,
            2.2,
            3.3,
        ).unwrap();
    
        let fields = UpdateFields {
            new_country: Some("UpdatedCountry"),
            new_confederation: None,
            new_population_share: Some(4.4),
            new_tv_audience_share: None,
            new_gdp_weighted_share: None,
        };
    
        let result = update_exec(
            &conn,
            "test_table",
            1,
            fields,
        );
    
        assert!(result.is_ok());
        teardown_db(&conn);
    }    

    #[test]
    fn test_delete_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        create_exec(
            &conn,
            "test_table",
            "TestCountry",
            "TestConfederation",
            1.1,
            2.2,
            3.3,
        )
        .unwrap();
        delete_exec(&conn, "test_table", 1).expect("Failed to delete record");
        let select_query = "SELECT id FROM test_table WHERE id = 1";
        let mut stmt = conn.prepare(select_query).unwrap();
        let result_iter = stmt.query_map([], |row| row.get::<_, i32>(0)).unwrap();
        let result: Vec<i32> = result_iter.filter_map(Result::ok).collect();
        assert_eq!(result.len(), 0);
        teardown_db(&conn);
    }

    #[test]
    fn test_drop_table() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        drop_table(&conn, "test_table").expect("Failed to drop table");
        let table_check_query =
            "SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'";
        let mut stmt = conn.prepare(table_check_query).unwrap();
        let result_iter = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
        let result: Vec<String> = result_iter.filter_map(Result::ok).collect();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_read_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        create_exec(
            &conn,
            "test_table",
            "TestCountry",
            "TestConfederation",
            1.1,
            2.2,
            3.3,
        )
        .unwrap();
        let result = read_exec(&conn, "test_table");
        assert!(result.is_ok());

        teardown_db(&conn);
    }
}
