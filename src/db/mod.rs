use rusqlite::{Connection, Error, Result};

static DB_INIT_SQL: &'static str = include_str!("./init_db.sql");

pub fn init_db(db_loc: &str) -> Result<Connection, Error> {
    let conn = Connection::open(db_loc)?;
    conn.execute_batch(DB_INIT_SQL)?;
    return Ok(conn);
}

#[cfg(test)]
mod tests {
    static DB_INIT_SQL: &'static str = include_str!("./testing_values.sql");

    use super::*;

    fn init() -> Result<Connection, Error> {
        let conn = init_db("./test_db.sqlite")?;
        conn.execute_batch(DB_INIT_SQL)?;
        return Ok(conn);
    }

    #[test]
    fn getting_a_record() -> Result<(), Error> {
        let conn = init();

        return Ok(());
    }
}
