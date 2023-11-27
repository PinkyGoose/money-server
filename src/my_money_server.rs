
use log::warn;
use tonic::{Request, Status, Response};

use crate::storage::Storage;

use self::money_server::Money;

tonic::include_proto!("money");

#[derive(Default)]
pub struct MyMoneyServer{

    storage: Storage,
}

#[tonic::async_trait]
impl Money for MyMoneyServer{
async fn register(&self,
    request:Request<RegisterRequest>,
)->Result<Response<RegisterResponce>,Status>{
    let a=request.into_inner();

    let res = self.storage.insert_user(a.mail,a.username,a.encrypted_pass);
    match &res {
        Ok(u)=>{
            let reply = RegisterResponce{
                status:0,
                token:u.to_string()
            };
        }
        Err(_)=>{
            let reply = RegisterResponce{
                status:0,
                token:String::from("")
            };
        }
        
    }
    let reply = RegisterResponce{
        status:0,
        token:res.unwrap()
    };
    Ok(Response::new(reply))
}
    async fn login(&self,
    request:Request<LoginRequest>,
)->Result<Response<LoginResponce>,Status>{
    warn!("LogIn Request consumed but no functionally");
    // let a = String::from("Geese are best");
    let reply = LoginResponce {
        token:request.into_inner().token,
    };
    Ok(Response::new(reply))
}

async fn quit(&self,
    request:Request<QuitRequest>,
)->Result<Response<QuitResponce>,Status>{
    let a = String::from("");
    let reply = QuitResponce {
        stat:0,
    };
    Ok(Response::new(reply))
}

async fn new_limit(&self,
    request:Request<NewLimitRequest>,
)->Result<Response<NewLimitResponce>,Status>{
    let a = String::from("");
    let reply = NewLimitResponce{
        status:0,
    };
    Ok(Response::new(reply))
}

async fn add_row_to_data(&self,
    request:Request<AddRowToDataRequest>,
)->Result<Response<AddRowToDataResponce>,Status>{
    let a = String::from("");
    let reply = AddRowToDataResponce{
        status:0,
    };
    Ok(Response::new(reply))
}




async fn new_list(&self,
    request:Request<NewListRequest>,
)->Result<Response<NewListResponce>,Status>{
    let a = String::from("");
    let reply = NewListResponce{
        status: 0,
    };
    Ok(Response::new(reply))
}

async fn get_all_data(&self,
    request:Request<GetAllDataRequest>,
)->Result<Response<GetAllDataResponce>,Status>{
    let reply = GetAllDataResponce { 
        datas: Vec::<RowData>::with_capacity(0),
    };
    Ok(Response::new(reply))
}
async fn get_all_limits(&self,
    request:Request<GetAllLimitsRequest>,
)->Result<Response<GetAllLimitsResponce>,Status>{
    let a = String::from("");
    let reply = GetAllLimitsResponce { 
        limits: Vec::<Limit>::with_capacity(0),
    };
    Ok(Response::new(reply))
}

}
