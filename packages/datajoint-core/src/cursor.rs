
use sqlx::postgres::PgRow;
use futures::stream::StreamExt;
use crate::connection::Connection;
use sqlx::*;
use futures_core::stream::BoxStream;
use std::option::Option::Some;
use std::pin::Pin;
use futures::{TryFutureExt, FutureExt, TryStreamExt, Stream, TryStream};
use futures::prelude::stream::{Next, TryNext};
use std::prelude::rust_2021::TryInto;

// Used to execute queries and access their results.
// Wraps sqlx::Executor.
pub struct Cursor<'e, 'm> {
    pub stream: Option <BoxStream<'m, Result<PgRow>>>,
    conn : & 'e Connection
}
impl<'l,'f> Cursor <'l,'f>  {
    pub fn new<'c,'v>(conn : & 'c Connection) -> Cursor<'c,'v>{

         Cursor  {
            stream : None::<BoxStream<'v, Result<PgRow>>>,
            conn
        }
    }

    pub fn execute(& mut self, query: & 'f str){

        let mut q = sqlx::query(query)
            .fetch(self.conn.pool.as_ref().unwrap());

        self.stream = Some(q);

    }


    pub fn fetch_one(&mut self) -> Option<PgRow> {

        if let Some(ref mut stream) = self.stream {
            let row = self.conn.run_time.block_on( stream.next());

            if let Some(res) = row {
                return  match res{
                    Ok(r) => Some(r),
                    Err(_) => None
                }
            }
            return None
        }
        return None
    }


    pub fn fetch_all(&mut self) -> Vec<PgRow> {

        let mut v = vec![];

        while let Some(row) = self.fetch_one() {
            v.push(row)
        }

        v

    }

}