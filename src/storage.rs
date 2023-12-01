// use self::auth::Auth;

use std::env;

use crate::storage::data::Data;


pub mod schema;
pub mod models;
use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, PgConnection};
use log::{error, warn, info, debug};

pub type Postgre = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() ->   Option<Postgre> {
    let db_url = env::var("DATABASE_URL").expect("Can't get DB URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    
    let a = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    Some(a)
    // let pool = 

}
type Conn = PooledConnection<ConnectionManager<PgConnection>>;

fn get_connection(pool: &Postgre) -> Conn {
    pool.get().expect("Can't get DB connection")
}


use crate::storage::auth::Auth;

use self::auth::MyError;
mod auth;
mod data;
#[derive(Default)]
pub struct Storage{
    pub datas:Data,
    pub auth: Auth
}

impl Storage {


    pub fn insert_user(&self ,email: String, name: String, pass: String)->Result<String, MyError> {
        
        self.auth.insert_user(email,name, pass)
        // let names = users::table.select(users::username).load(conn)?;
    }
}