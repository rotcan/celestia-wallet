use cel_wallet::tx::TxnListResponse;
use cel_wallet::error::WalletError;
use cel_wallet::menu::user::User;

pub struct TxnListPromise{
    promise: Option<poll_promise::Promise<Result<Vec<TxnListResponse>,WalletError>>>,
    result: Option<Vec<TxnListResponse>>,
    error: Option<String>,
}

impl TxnListPromise{

    pub fn new()->Self{
        TxnListPromise{
            promise: None,
           result: None,
           error: None
        }
    }
    //init
    pub fn init(&mut self, user: &User, account_name: &String,sleep_time: u64,){
        let user_clone=user.clone();
        let name=account_name.clone();
        self.promise = Some(poll_promise::Promise::spawn_async(async move {
            std::thread::sleep(tokio::time::Duration::from_millis(sleep_time));
            let txns=user_clone.get_account_txns(name).await;
            txns
        }));
    }
    //check result

    pub fn check_result(&mut self, ) 
    {
        
        match &self.promise{
            Some(promise)=>{
                if let Some(result) = promise.ready() {
                    match result{
                        Ok(result)=>{
                            self.result=Some(result.clone());
                           
                        },
                        Err(_)=>{
                            //failure_fn();
                            self.error=Some("Failed to get response".to_string());
                        }
                    }
                    
                    self.promise=None;
                } else {
                    // Show a loading screen
                    //println!("waiting");
                }
                
            },
            None=>{}
        }
    }

    pub fn consume_result(&mut self)->Option<Vec<TxnListResponse>>{
        if self.result.is_some(){
            let result=self.result.clone();
            self.result=None;
            return result;
        };
        return None;
    }

    pub fn consume_failure(&mut self)->Option<String>{
        if self.error.is_some(){
            let error=self.error.clone();
            self.error=None;
            return error;
        };
        return None;
    }
}
