#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
    use sqlite::{
        create_table, delete_exec, drop_table, insert_exec, load_data_from_csv, query_exec,
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
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        let csv_path = "../data/customer_new.csv";
        load_data_from_csv(&conn, "test_table", csv_path)?;

        let select_query = "SELECT id, name, gender, city FROM test_table ORDER BY id";
        let mut stmt = conn.prepare(select_query).unwrap();
        let result_iter = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .unwrap();

        let result: Vec<(i32, String, String, String)> =
            result_iter.filter_map(Result::ok).collect();
        assert_eq!(result.len(), 10);
        assert_eq!(
            result[0],
            (
                1,
                "Alice".to_string(),
                "female".to_string(),
                "New York".to_string()
            )
        );
        assert_eq!(
            result[9],
            (
                10,
                "Julia".to_string(),
                "female".to_string(),
                "Chicago".to_string()
            )
        );
        teardown_db(&conn);
        Ok(())
    }

    #[test]
    fn test_insert_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        insert_exec(&conn, "test_table", 1, "Jo", "male", "Durham")
            .expect("Failed to insert record");
        let select_query = "SELECT id, name, gender, city FROM test_table WHERE id = 1";
        let mut stmt = conn.prepare(select_query).unwrap();
        let result_iter = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .unwrap();

        let result: Vec<(i32, String, String, String)> =
            result_iter.filter_map(Result::ok).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            (
                1,
                "Jo".to_string(),
                "male".to_string(),
                "Durham".to_string()
            )
        );
        teardown_db(&conn);
    }

    #[test]
    fn test_update_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        insert_exec(&conn, "test_table", 1, "Doe", "male", "New York")
            .expect("Failed to insert record");
        update_exec(
            &conn,
            "test_table",
            1,
            Some("Doe"),
            None,
            Some("San Francisco"),
        )
        .expect("Failed to update record");
        let select_query = "SELECT id, name, gender, city FROM test_table WHERE id = 1";
        let mut stmt = conn.prepare(select_query).unwrap();
        let result_iter = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .unwrap();

        let result: Vec<(i32, String, String, String)> =
            result_iter.filter_map(Result::ok).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            (
                1,
                "Doe".to_string(),
                "male".to_string(),
                "San Francisco".to_string()
            )
        );
        teardown_db(&conn);
    }

    #[test]
    fn test_delete_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        insert_exec(&conn, "test_table", 1, "Mike", "male", "New York")
            .expect("Failed to insert record");
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
    fn test_query_exec() {
        let _lock = DB_MUTEX.lock().unwrap();
        let conn = setup_db();
        insert_exec(&conn, "test_table", 1, "Jo", "male", "New York")
            .expect("Failed to insert record");

        let query_string = "SELECT id, name, gender, city FROM test_table ORDER BY id";
        query_exec(&conn, query_string).expect("Failed to execute query");
        teardown_db(&conn);
    }
}
