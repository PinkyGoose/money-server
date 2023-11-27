// use self::auth::Auth;

use crate::storage::data::Data;


use crate::storage::auth::Auth;
mod auth;
mod data;
#[derive(Default)]
pub struct Storage{
    datas:Data,
    auth: Auth
}