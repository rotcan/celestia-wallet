use cel_wallet::utils::CoinMetadata;
use cel_wallet::menu::user;

pub struct BalancePromise{
    promise: Option<poll_promise::Promise<Result<Option<Vec<CoinMetadata>>,cel_wallet::error::WalletError>>>,
    result: Option<Vec<CoinMetadata>>,
    error: Option<String>,
}

impl BalancePromise{

    pub fn new()->Self{
        BalancePromise{
            promise: None,
           result: None,
           error: None
        }
    }
    //init
    pub fn init(&mut self, user: &user::User, account_name: &String){
        if let Some(account) = user.search_account_by_name(account_name) {
            let account_clone=std::sync::Arc::clone(&std::sync::Arc::new(account));
            let user_clone=user.clone();

            self.promise = Some(poll_promise::Promise::spawn_async(async move {
                // std::thread::sleep(tokio::time::Duration::from_millis(1000));
                let balances=account_clone.get_all_balances(&user_clone).await;
                balances
            }));
        };
    }
    //check result

    pub fn check_result(&mut self, 
        // mut success_fn: F1,
        // mut failure_fn: F2
    ) 
    // where F1: std::ops::FnMut( Option<Vec<CoinMetadata>>), F2: std::ops::FnMut()
    {
        
        match &self.promise{
            Some(promise)=>{
                if let Some(result) = promise.ready() {
                    match result{
                        Ok(result)=>{
                            self.result=result.clone();
                           
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

    pub fn consume_result(&mut self)->Option<Vec<CoinMetadata>>{
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