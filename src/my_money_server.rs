
use log::{warn, error};
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
    Ok(Response::new(
    match &res {
        Ok(u)=>{
            RegisterResponce{
                status:0,
                token:u.to_string()
            }
        },
        Err(e)=>{
            error!("error in register");
            RegisterResponce{
                status:2,
                token:String::from("")
            }
        }
        
    }
    // let reply = RegisterResponce{
    //     status:0,
    //     token:res
    // };
    ))
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
    _request:Request<QuitRequest>,
)->Result<Response<QuitResponce>,Status>{
    let _a = String::from("");
    let reply = QuitResponce {
        stat:0,
    };
    Ok(Response::new(reply))
}

async fn new_limit(&self,
    _request:Request<NewLimitRequest>,
)->Result<Response<NewLimitResponce>,Status>{
    let _a = String::from("");
    let reply = NewLimitResponce{
        status:0,
    };
    Ok(Response::new(reply))
}

async fn add_row_to_data(&self,
    _request:Request<AddRowToDataRequest>,
)->Result<Response<AddRowToDataResponce>,Status>{
    let _a = String::from("");
    let reply = AddRowToDataResponce{
        status:0,
    };
    Ok(Response::new(reply))
}




async fn new_list(&self,
    _request:Request<NewListRequest>,
)->Result<Response<NewListResponce>,Status>{
    let _a = String::from("");
    let reply = NewListResponce{
        status: 0,
    };
    Ok(Response::new(reply))
}

async fn get_all_data(&self,
    _request:Request<GetAllDataRequest>,
)->Result<Response<GetAllDataResponce>,Status>{
    let reply = GetAllDataResponce { 
        datas: Vec::<RowData>::with_capacity(0),
    };
    Ok(Response::new(reply))
}
async fn get_all_limits(&self,
    _request:Request<GetAllLimitsRequest>,
)->Result<Response<GetAllLimitsResponce>,Status>{
    let _a = String::from("");
    let reply = GetAllLimitsResponce { 
        limits: Vec::<Limit>::with_capacity(0),
    };
    Ok(Response::new(reply))
}

}
