pub mod connection;
mod cursor;
mod table_row_vector;




//////////////////////////////////////////////////////////////////////////////////
//  Tests
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::connection::Connection;
    use sqlx::{Row, Column};
    use futures::StreamExt;
    use std::ops::Deref;
    use sqlx::postgres::PgRow;
    use std::fmt::Pointer;

    #[test]
    fn it_works() {
        let mut con = Connection::new("", "", "", false, false);
        con.connect();
        let mut cursor = con.query("select * from public.\"students\"");
        let rows = cursor.fetch_all();
        for  row in rows {
            for col in row.columns() {
                let str: &str = row.get(col.name());
                println!("{:?}", str)
            }
        }
    }
}