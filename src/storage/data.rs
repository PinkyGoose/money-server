use log::debug;

use super::{Postgre, create_connection_pool};


pub struct Data{
    pool: Postgre

}
impl Data {
    pub fn new(pool2: Postgre)->Data{
        Self { pool: pool2 }
    }
}
impl Default for Data {
    fn default() -> Self {
        debug!("creating data connection pool");
        let res = Self { pool: create_connection_pool().expect("RRR")};
        debug!("creating data connection pool success");
        res
    }
}
