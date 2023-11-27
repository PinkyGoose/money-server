
use diesel::r2d2::Error;
use diesel::sql_types::Bool;
use diesel::{RunQueryDsl, QueryDsl, QueryResult, ExpressionMethods};
use rand::{distributions::Alphanumeric,Rng};

use super::schema::users::{username, email};
use super::{models::NewUser, schema::users};
use super::{Postgre, create_connection_pool, Conn, get_connection};

pub struct Auth{
    pool:Postgre,
}
pub enum MyError{
    SomeThingWrong,
    Error(Error)
}

impl Auth {

    pub fn get_names(conn: &mut Conn) -> QueryResult<Vec<String>> {
        let names = users::table.select(users::username).load(conn)?;
        Ok(names)
    }
    pub fn check_user(&self,mail:&String, name:&String)->bool{
        
        let mut conn = get_connection(&self.pool);
        // let isds = users::table.filter(username.eq(name)).or_filter( email.eq(mail))
        let good_animals:Vec<String> = users::table
        .filter(username.eq(name))
        .or_filter(email.eq(mail))
        .select(users::username)
        .get_results(&mut conn).expect("drfeh");
        if good_animals.len()>0 {
            return true
        }
        false
    }
    pub fn insert_user(&self, mail: String, name:String, pass:String)->Result<String, MyError> {
        if self.check_user(&mail, &name) {
            return Err(MyError::SomeThingWrong);
        }
        let a = generate_token();
        let mut u:String=String::new();
        let mut conn = get_connection(&self.pool);
        String::clone_from( &mut u, &a);
        let nu=NewUser{email:mail,username:name, enc_pass:pass, token: u};
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