use std::fs;
use std::path::{PathBuf,Path};
use std::error::Error;
use query_client::state::ProtoMetadata;

#[derive(Clone,Debug)]
pub struct CoinMetadata{
    pub name: String,
    pub symbol: String,
    pub exponent: u32,
    pub balance: u128,
    pub denom: String,
}

impl CoinMetadata{
    pub fn new(name: String, symbol: String, exponent: u32, balance: u128,denom: String)->Self{
        CoinMetadata{
            name,
            symbol,
            exponent,
            balance,
            denom
        }
    }

    pub fn update_balance(&mut self, balance: u128){
        self.balance=balance;
    }
}

impl From<&ProtoMetadata> for CoinMetadata{
    fn from(metadata: &ProtoMetadata)->CoinMetadata{
        let units: Vec<_> =metadata.denom_units.iter().filter(|m| m.denom == metadata.name).collect::<Vec<_>>();
        let unit = units.first().unwrap();
        CoinMetadata::new(metadata.name.clone(),metadata.symbol.clone(),unit.exponent,0,metadata.base.to_string())
    }
}

pub fn init_dir(path_str: String)->Result<(),Box<dyn Error>>{
    match fs::create_dir_all(path_str){
        Ok(_v)=>Ok(()),
        Err(err)=>panic!("error in creating directory {}",err),
    }
}

pub fn init_file(path_str: String)->Result<(),Box<dyn Error>>{
    if Path::new(&path_str).exists() == false {
        return match fs::File::create(path_str){
            Ok(_v)=>Ok(()),
            Err(err)=>panic!("error in creating file {}",err),
        }
    };
    Ok(())
}

pub fn check_write_permission(path_str: String)->Result<(),Box<dyn Error>>{
    let file_path=PathBuf::from(path_str).join(".check");
    //info!("creating file {:?}",file_path);
    //create file
    let _file=fs::File::create(file_path.clone())?;
    //delete file
    fs::remove_file(file_path)?;

    Ok(())
}


pub fn find_all_files(path_str: String)->Result<Vec<String>,Box<dyn Error>>{
    let paths = fs::read_dir(path_str).unwrap();
    let mut files=vec![];
    for path in paths{
        let p=path.unwrap();
        let md=p.metadata().unwrap();
        if md.is_file() {
            files.push(p.file_name().into_string().unwrap())
        }
    }
    Ok(files)
}
 
pub fn load_file(path_str: String)->Result<Vec<u8>,crate::error::WalletError>{
    match fs::read(path_str){
        Ok(data)=>Ok(data),
        Err(_)=>Err(crate::error::WalletError::FileOpenError),
    }
}