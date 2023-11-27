


use diesel::Insertable;

use super::schema::users;
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser{
    pub username: String,
    pub email: String,
    pub enc_pass: String,
    pub token: String,
}