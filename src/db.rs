use rusqlite::{Connection, OptionalExtension};

thread_local! {
    pub static DB: Connection = {
        let conn = Connection::open("./bauchbinde.db").expect("Failed to open database");
        let _ = conn.execute_batch(include_str!("sql/migrate.sql"));
        conn
    };
}

pub fn add_title(name: &str, label: &str) -> Result<usize, String> {
    match DB.with(|f| f.execute(include_str!("sql/add.sql"), (name, label))) {
        Ok(n) => Ok(n),
        Err(e) => {
            // catch duplicate error
            if let rusqlite::Error::SqliteFailure(e, _) = e
                && e.code == rusqlite::ffi::ErrorCode::ConstraintViolation
            {
                return Ok(0);
            }
            Err(format!("{e}"))
        }
    }
}

pub fn delete_title(id: i64) -> Result<usize, String> {
    match DB.with(|f| f.execute(include_str!("sql/delete.sql"), [id])) {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn edit_title(id: i64, name: &str, label: &str) -> Result<usize, String> {
    match DB.with(|f| f.execute(include_str!("sql/edit.sql"), (name, label, id))) {
        Ok(n) => Ok(n),
        Err(e) => {
            // catch duplicate error
            if let rusqlite::Error::SqliteFailure(e, _) = e
                && e.code == rusqlite::ffi::ErrorCode::ConstraintViolation
            {
                return Ok(0);
            }
            Err(format!("{e}"))
        }
    }
}

pub fn get_title(id: i64) -> Result<Option<(String, String)>, String> {
    match DB.with(|f| {
        f.query_one(include_str!("sql/get.sql"), [id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .optional()
    }) {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn list_titles() -> Result<Vec<(i64, String, String)>, String> {
    match DB.with(|f| {
        Ok::<Vec<(i64, String, String)>, rusqlite::Error>(
            f.prepare(include_str!("sql/list.sql"))?
                .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
                .map(|row| row.unwrap())
                .collect(),
        )
    }) {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("{e}")),
    }
}
