
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, QueryResult, Insertable};
use rand::{distributions::Alphanumeric,Rng};

use super::{models::NewUser, schema::users};
use super::{Postgre, create_connection_pool, Conn, get_connection};

pub struct Auth{
    pool:Postgre,
}
impl Auth {

    pub fn get_names(conn: &mut Conn) -> QueryResult<Vec<String>> {
        let names = users::table.select(users::username).load(conn)?;
        Ok(names)
    }
    pub fn insert_user(&self, email: String, name:String, pass:String)->Result<String, Error> {
        let a = generate_token();
        let mut u:String=String::new();
        let mut conn = get_connection(&self.pool);
        String::clone_from( &mut u, &a);
        let nu=NewUser{email:email,username:name, enc_pass:pass, token: u};
        diesel::insert_into(users::table).values(&nu).returning(users::id).get_result::<i32>(&mut conn).expect("Error by pushing to db");
        // let names = users::table.select(users::username).load(conn)?;
        Ok(a)
    }
}
impl Default for Auth {
    fn default() -> Self {
        Self { pool: create_connection_pool().expect("RRR")}
    }
}


fn generate_token()->String{
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect()
}