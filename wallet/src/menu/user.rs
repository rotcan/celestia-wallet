use rust_keyring::{secp256k1_key,state::{FileCredential,ErrorCode,EXTENSION}};
use serde::{Serialize,Deserialize};
use crate::menu::settings::{SEED_KEY,GlobalConfig,Network};
use crate::menu::accounts::Account;
use crate::utils;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use crate::error::WalletError;
use std::path::PathBuf;
use query_client::tx::TxnListResponse;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Copy)]
pub enum UserState{
    Empty,
    New,
    Existing,
    Loaded
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct User{
    password: Option<String>,
    //private key
    seed: Vec<u8>,
    pub config: Option<UserConfig>,
    phrase: Option<String>,
    language: Option<String>,
    state: UserState,
    // account_map: HashMap<String,Account>,
}

//import a wallet
//create new wallet
//check from file
impl User{

    pub fn default()->Self{
        User{
            password: None,
            seed: vec![],
            config: None,
            phrase: None,
            state : UserState::Empty,
            language: None,
        }
    }
    //new
    pub fn new(&mut self,language: Option<&str>)->Result<(),WalletError>{
        let config=GlobalConfig::default();
        //check from file
        // if file exists , state  = existing
        let credential_exists=Self::check_credential_exists(&config);
        self.state = if credential_exists == true {
            UserState::Existing
        }else{
            UserState::New
        };
        //else state = new
        self.phrase = if self.state == UserState::New {
            Some(secp256k1_key::get_new_mnemoic(language.clone()).into_phrase())
        }else{
            None
        };
        self.language=language.map(|m| m.to_string());
        Ok(())
    }

   
    pub fn get_mnemonic(language: Option<&str>)->String{
        let mnemonic=secp256k1_key::get_new_mnemoic(language);
        mnemonic.phrase().to_owned()
    }
    //load
    pub fn load_wallet(&mut self,password: &str)->Result<(),WalletError>{
        let config=GlobalConfig::default();
        if self.state != UserState::Loaded {
            //load settings
            self.seed= if self.state == UserState::New{
                let phrase=self.phrase.clone().unwrap();
                secp256k1_key::create_new_seed_from_phrase(&phrase,self.language.clone().as_deref())
            }else if self.state == UserState::Existing {
                Self::load_credential_from_file(&config,Some(password.to_string()))?
            }else{
                panic!("Cannot load wallet as private key is not set");
            };
            self.state=UserState::Loaded;
            self.config=Some(UserConfig::new(password,false));
            self.password=Some(password.to_string());
        }
        Ok(())
    }

    pub fn load_wallet_from_mnemoic(&mut self, phrase: &str,language: Option<&str> , password: &str)->Result<(),WalletError>{
        let mut user_config=UserConfig::new(password,true);
        let global_config=GlobalConfig::default();
        let settings_path=global_config.settings_path.clone();
        let seed=secp256k1_key::create_new_seed_from_phrase(phrase,language.clone());
        //Check accounts
        let accounts_count=user_config.get_accounts().len();
        if accounts_count > 0 {
            return Err(WalletError::AccountError("Cannot delete old wallet".to_owned()));
        };
        user_config.new_hd_account(password, seed.clone(),None)?;
        
        self.password=Some(password.to_string());
        self.seed=seed.clone();
        self.config=Some(user_config);
        self.state=UserState::Loaded;
        self.language=language.map(|m| m.to_string());
        //Save credential
        User::save_credential_to_file(&global_config,seed.clone(), password)?;
        //Save Settings
        self.config.as_ref().map(|config| config.save_settings(settings_path).unwrap());
        Ok(())
    }

    // fn create_account_map(&mut self){
    //     if self.state == UserState::Loaded {
    //         let accounts=user_config.get_accounts();

    //     }
    // }

    fn check_credential_exists(config: &GlobalConfig)->bool{
        let dir = &config.base_path;
        let entry=FileCredential::get_entry_from_file(&dir.clone(),SEED_KEY,None,true).unwrap();
        !matches!(entry.get_password(), Err(ErrorCode::NoEntry)) 
    }
     
    fn load_credential_from_file(config: &GlobalConfig,password: Option<String>)->Result<Vec<u8>,WalletError>{
        let dir = &config.base_path;
        let entry=FileCredential::get_entry_from_file(&dir.clone(),SEED_KEY,password,true)?;
        let credential: &FileCredential = entry
        .get_credential()
        .downcast_ref()
        .expect("Not a file credential");
        let metadata=credential.decode_data()?;
        let private_key=metadata.get_item();
        Ok(private_key)
    }
    //save

    fn save_credential_to_file(config: &GlobalConfig,seed: Vec<u8>,password: &str )->Result<bool, WalletError>{
        let dir = &config.base_path;
        let entry=secp256k1_key::create_key(&dir.clone(),SEED_KEY,password,seed,true).unwrap();
        let _credential: &FileCredential = entry
        .get_credential()
        .downcast_ref()
        .expect("Not a file credential");
        Ok(true)
    }
 

    pub fn add_account(&mut self, name: Option<String>)->Result<(),WalletError>{
        let seed=self.get_seed();
        let password=self.get_password()?;
        self.config.as_mut().map(|config| config.new_hd_account( &password ,seed,name));
        Ok(())
    }

    pub fn get_new_account_address(&self)->Result<(String,String),WalletError>{
        let seed=self.get_seed();
        self.config
            .as_ref()
            .map_or_else(
                || Err(WalletError::UserConfigError), 
                |config| Ok(config.get_new_hd_account_address( seed).unwrap())
            )
        
    }

    pub fn add_account_using_private_key(&mut self,private_key: &str, name: &str)->Result<(),WalletError>{
        let password=self.get_password()?;
        self.config.as_mut().map(|config| config.new_account_using_private_key(&password,private_key, name));
        Ok(())
    }

    pub fn get_accounts(&self)->Option<Vec<Account>>{
        Some(self.config.clone().map(|m| m.get_accounts())?)
    }

    pub fn search_account_by_name(&self, account_name: &str)->Option<Account>{
        self.config.as_ref().map(|m| m.search_account_by_name(account_name))?
    }

    // pub fn remove_all_accounts(&mut self)->bool{
    //     self.config.as_mut().map( |m| m.remove_all_accounts()).unwrap_or(false)
    // }
    

    pub fn remove_account(&mut self,name: &str)->bool{
        self.config.as_mut().map(| m| 
            m.remove_account(name, self.password.clone().unwrap().as_str())
        ).unwrap_or(false)
    }

    pub async fn get_account_txns(&self,account_id: String)->Result<Vec<TxnListResponse>,WalletError>{
        if self.config.is_some() {
            return self.config.clone().unwrap().get_account_txns(account_id).await;
        };
        Err(WalletError::UserConfigError)
    }

    pub fn get_config(&self)->Option<UserConfig>{
        self.config.clone()
    }

    pub fn get_seed(&self)->Vec<u8>{
        self.seed.clone()
    }

    pub fn get_grpc_url(&self)->String{
        self.get_config().unwrap().user_settings.get_grpc_url()
    }

    pub fn get_root_account(&self)->Account{
        self.search_account_by_name("account-0").unwrap().clone()
    }

    pub fn set_current_account(&mut self,value: String){
        self.config.as_mut().map(|config| config.set_current_account(value));
    }
    pub fn get_current_account_name(&self)->String{
        self.config.as_ref().map(|m| m.get_current_account()).unwrap_or("account-0".to_string())
    }

    pub fn get_current_account_address_detail(&self)->(String,String){
        let name=self.get_current_account_name();
        let account=self.search_account_by_name(&name).unwrap();
        (account.get_account_id(),account.title)
    }

    pub fn get_state(&self)->UserState{
        self.state
    }

    pub fn get_password(&self)->Result<String,WalletError>{
        self.password.clone()
            .map_or_else(|| Err(WalletError::PasswordError("Password not found".to_string())) , |m| Ok(m))
    }
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct UserConfig{
    accounts: Vec<Account>,
    pub user_settings: UserSettings,
    current_account: Option<String>,
    global_config: GlobalConfig,
}

impl UserConfig{
 
    pub fn new(password: &str,reset: bool)->UserConfig{
        let config=GlobalConfig::default();
        let settings_path=config.settings_path.clone();
        let accounts_path=config.accounts_path.clone();
        utils::init_dir(accounts_path.clone()).unwrap();
        utils::init_file(settings_path.clone()).unwrap();
        //println!("accounts_path={:?} settings_path={:?}",accounts_path,settings_path);
        
        let user_settings = if reset == false {
            Self::load_settings_data(settings_path).unwrap_or(UserSettings::default())
        }else{
            UserSettings::default()
        };
        if reset==true {
            Self::remove_all_accounts(accounts_path.clone()).unwrap();
        }
        let accounts= Self::load_accounts_data(accounts_path,password).unwrap_or(vec![]);
        UserConfig{
            accounts,
            user_settings,
            current_account: None,
            global_config: config,
        }
    }

    fn save_settings_data(settings: &UserSettings, file: String)->Result<(),WalletError>{
        let toml=toml::to_string(settings).unwrap();
        let mut file = fs::File::create(file)?;
        file.write_all(toml.as_bytes())?;
        Ok(())
    }

    fn load_settings_data(file: String)->Result<UserSettings,WalletError>{
        let contents = fs::read_to_string(file)?;
        let config: UserSettings=toml::from_str(&contents).unwrap_or(UserSettings::default());
        Ok(config)
    }

    fn load_accounts_data(dir: String,password: &str)->Result<Vec<Account>,WalletError>{
        let files= utils::find_all_files(dir.clone()).unwrap();
        let mut accounts=vec![];
        for key in files{
            accounts.push(Account::load( &dir,&key,password).unwrap());
        };
        Ok(accounts)
    }

    pub fn save_settings(&self, file: String)->Result<(),WalletError>{
        Self::save_settings_data(&self.user_settings,file)
    }

    
    pub fn load_settings(&mut self,file: String)->Result<(),WalletError>{
        self.user_settings=Self::load_settings_data(file).unwrap();
        Ok(())
    }

    fn get_new_hd_account_address(&self, seed: Vec<u8>)->Result<(String,String),WalletError>{
        let global_config=GlobalConfig::default();
        Account::get_new_hd_account_address(self,&global_config,seed)
    }

    fn new_hd_account(&mut self, password: &str, seed: Vec<u8>, name: Option<String>)->Result<(),WalletError>{
        let global_config=GlobalConfig::default();
        self.accounts.push(Account::new_hd_account(self,&global_config,password,seed,name)?);
        self.user_settings.key_serial_no=self.user_settings.key_serial_no+1;
        //save 
        self.save_settings(self.global_config.settings_path.clone())?;
        Ok(())
    }

    fn new_account_using_private_key(&mut self, password: &str, private_key: &str, name: &str)->Result<(),WalletError>{
        let global_config=GlobalConfig::default();
        self.accounts.push(Account::new_account_by_private_key(&global_config,password,private_key,name)?);
        self.user_settings.key_serial_no=self.user_settings.key_serial_no+1;
        Ok(())
    }
    //Do not delete all accounts. Is it required?
    pub fn remove_account(&mut self,name: &str,password: &str)->bool{
        if self.accounts.len() == 1 {
            return false;
        };
        let index= self.accounts.iter().position(|x| x.value_match(name));
        if index.is_some() {
            let index=index.unwrap();
            self.accounts.get(index).unwrap().remove(password).unwrap();
            self.accounts.remove(index);
            return true;
        };
        false
    }

    pub fn remove_all_accounts(dir: String)->Result<bool,WalletError>{
        let files= utils::find_all_files(dir.clone()).unwrap();
        for key in files{
            if key.ends_with(EXTENSION) == true {
                let full_path=PathBuf::from(dir.clone()).join(key);
                //accounts.push(Account::load( &dir,&key,password).unwrap());
                FileCredential::force_delete_file(full_path.as_path().as_os_str().to_str().expect("Path error"))?;
            };
        };
        Ok(true)

    }

    pub fn get_accounts(&self)->Vec<Account>{
        self.accounts.clone()
    }

    pub fn search_account_by_name(&self, name: &str)->Option<Account>{
        let index= self.accounts.iter().position(|x| x.value_match(name));
        if index.is_some() {
            let index=index.unwrap();
            return Some(self.accounts.get(index).unwrap().clone());
        };
        None
    }

    pub fn set_current_account(&mut self,account: String){
        self.current_account=Some(account);
    }
    pub fn get_current_account(&self)->String{
        self.current_account.clone().unwrap_or("account-0".to_string())
    }

    pub async fn get_account_txns(&self, account_id: String)->Result<Vec<TxnListResponse>,WalletError>{
        let account=self.search_account_by_name(&account_id);
        if account.is_some(){
            return account.clone().unwrap().get_txns(self).await;
        };
        Err(WalletError::AccountError("Acount not found".to_owned()))
    }
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct UserSettings{
    network_list: HashMap<String, Network>,
    current_network: String,
    pub key_serial_no : u64,
}

impl UserSettings{
    pub fn default()->Self{
        let default_network=Network::default();
        let current_network=default_network.get_name();
        
        Self::new(HashMap::from([(current_network.clone(),default_network)]),current_network,0)
    }

    pub fn new(network_list: HashMap<String,Network>,current_network: String, serial_no: u64)->Self{
        UserSettings{
            network_list,
            current_network,
            key_serial_no: serial_no,
        }
    }

    pub fn add_network(&mut self, name: String,
        grpc_url: String,
        rpc_url: Option<String>,
        indexer_url: String,){
        self.network_list.insert(name.clone(),Network::new(name,grpc_url,rpc_url,indexer_url));
    }

    pub fn select_network(&mut self,network: String)->bool{
        if self.network_list.contains_key(&network) {
            self.current_network=network.clone();
            return true;
        };
        false
    }

    pub fn get_current_network(&self)->Network{
        self.network_list.get(&self.current_network).unwrap().clone()
    }

    pub fn get_grpc_url(&self)->String{
        self.get_current_network().get_grpc_url()
    }

     
}
