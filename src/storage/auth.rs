use rand::{distributions::Alphanumeric,Rng};

#[derive(Default)]
pub struct Auth{}

impl Auth {
    async fn add_user_to_db(&self, name:String, mail:String, enc_pass:String,
    )->Result<String,String>{
        let a = generate_token();
            //TODO to db
        Ok(a)
    }
}



fn generate_token()->String{
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect()
}