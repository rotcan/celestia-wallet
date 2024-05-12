use keyring::credential::{CredentialApi};
use josekit::{jwe::alg::direct::DirectJweAlgorithm,
jwe::deserialize_compact,jwe::serialize_compact,jwe::JweHeader};
use std::iter;
use keyring::error::{decode_password};
use std::path::PathBuf;
use serde::{Serialize,Deserialize};
use std::fs;
use std::io::{Read,Write,Error as IOError,ErrorKind};
use keyring::Entry;
use crate::error::FileCredentialError;

pub type Result<T>=std::result::Result<T,FileCredentialError>;
pub type KeyringResult<T>=std::result::Result<T,keyring::error::Error>;

pub const CIPHER: &str ="A128CBC-HS256";
pub const KEY_SIZE: usize=32;
pub const EXTENSION: &str=".cel";
pub type ErrorCode=keyring::error::Error;
// #[derive(Debug,Serialize,Deserialize, PartialEq, Eq)]
// pub struct Item{
//     data: Vec<u8>,
// }

#[derive(Debug,Serialize,Deserialize, PartialEq, Eq)]
pub struct Metadata{
    item : Vec<u8>,
    modification_time: i64,
}

impl Metadata{
    pub fn new(item: Vec<u8>)->Self{
        Metadata{
            item,
            modification_time: 0
        }
    }

    pub fn get_item(&self)->Vec<u8>{
        self.item.clone()
    }
}

#[derive(Debug,Serialize,Deserialize, PartialEq, Eq)]
pub struct FileCredential{
    key: String,
    dir: String,
    // metadata: Metadata,
    password: Option<String>,
    allow_overwrite: bool
}

impl CredentialApi for FileCredential{
    //Update Password
    fn set_password(&self, password: &str) -> KeyringResult<()> {
        //encode and save data
        //let fmt_pass=decode_password(FileCredential::get_formatted_key(password).unwrap()).unwrap();
        self.save_file(password).expect("Error while setting password");
        Ok(())
    }

    fn get_password(&self)->KeyringResult<String>{
        //read from command line
        //let pass= FileCredential::parse_password(&FileCredential::read_password_from_prompt(Some(&self.key)).unwrap()).unwrap();
        //decode file and return as string
        //self.decode_file(pass.clone())?;
        Ok(FileCredential::read_file(
            &mut self.load_file().map_err(|_e| {
                keyring::error::Error::NoEntry
        })?
        ).map_err(|_| keyring::error::Error::NoEntry)?)
        //
    }

    fn delete_password(&self)->KeyringResult<()>{
        // let pass= FileCredential::parse_password(
        //     &FileCredential::read_password_from_prompt(Some(&self.key)).unwrap().as_str()).unwrap();
        // let file_data=FileCredential::read_file(&mut self.load_file()?)?;
        // self.decode_data(&file_data.as_str())?;
        self.decode_data().map_err(|_| keyring::error::Error::Ambiguous(vec![])).unwrap();
        self.delete_file().map_err(|_| keyring::error::Error::NoEntry).unwrap();
        Ok(())
    }

    fn as_any(&self)->&dyn std::any::Any{
        self
    }
}

impl FileCredential{

    pub fn get_credential(&self) -> Result<Self> {
        self.get_password()?;
        Ok(FileCredential::new(&self.dir,&self.key,self.password.clone(),self.allow_overwrite).unwrap())
    }

    pub fn set_jwe_passphrase(&mut self, pass_phrase: Option<String>)->Result<()>{
        self.password=pass_phrase;
        Ok(())
    }

    pub fn new(dir: &str, file_name: &str, password: Option<String>,allow_overwrite : bool)->Result<Self>{
        //println!("state new = {:?} {:?} {:?}",dir,file_name,password);
        let file_name_ext= if file_name.ends_with(EXTENSION) == false {
            format!("{}{}",file_name,EXTENSION)
        }else{
            file_name.to_owned()
        };
        Ok(
            FileCredential{
                key: file_name_ext,
                dir: dir.to_string(),
                password,
                allow_overwrite,
            }
        )
    }

    fn get_path(&self)->Result<String>{
        Ok(PathBuf::from(self.dir.clone()).join(self.key.clone()).to_str().unwrap().to_string())
    }

    pub fn load_file(&self)->Result<fs::File>{
        let path= self.get_path()?;
        //let path_with_extension=format!("{}{}",path,EXTENSION);
        Ok(fs::File::open(&path).map_err(decode_error)?  )
    }

    fn read_file(f: &mut fs::File)->Result<String>{
        let mut jwe = vec![];
        f.read_to_end(&mut jwe).unwrap();
        Ok(std::str::from_utf8(&jwe).unwrap().to_string())
    }

    fn delete_file(&self)->Result<()>{
        let path= PathBuf::from(self.dir.clone()).join(self.key.clone());
        fs::remove_file(path.clone().to_str().unwrap()).map_err(decode_error)?;
        Ok(())
    }

    pub fn force_delete_file( path: &str)->Result<()>{
        fs::remove_file(path).map_err(decode_error)?;
        Ok(())
    }

    pub fn save_file(&self, data: &str)->Result<()>{
        if matches!(self.get_password(), Err(ErrorCode::NoEntry))==false && !self.allow_overwrite {
            return Err(FileCredentialError::KeyringError(ErrorCode::NoEntry))
        };
        //let mut f= self.load_file()?;
        let path= self.get_path()?;
        let mut f=fs::File::create(&path.as_str()).map_err(decode_error)?;
        f.write_all(data.as_bytes()).unwrap();
        Ok(())
    }

    pub fn encode_data<'a>(&self,metadata: &'a Metadata)->Result<String>{
        let alg = DirectJweAlgorithm::Dir;
        let pass=self.parse_password()?;
        let payload=serde_json::to_string(metadata).unwrap();
        let encrypter = alg.encrypter_from_bytes(&pass.as_bytes()).unwrap();
        let mut header = JweHeader::new();
        header.set_content_encryption(CIPHER);
        Ok(serialize_compact(payload.as_bytes(), &header, &encrypter).unwrap())
        //Ok(save_file(true,jwe)?)
    }

    pub fn decode_data(&self)->Result<Metadata>{
        let file_data=FileCredential::read_file(&mut self.load_file()?)?;
        let alg = DirectJweAlgorithm::Dir;
        let pass=self.parse_password()?;
        let decrypter = alg.decrypter_from_bytes(&pass.as_bytes())?;
        let (data, _header) = deserialize_compact(&file_data.as_str(), &decrypter)?;
        let metadata=serde_json::from_str(std::str::from_utf8(&data).unwrap())?;
        Ok(metadata)
    }


    pub fn read_password_from_prompt(txt: Option<&str>)->Result<String>{
        Ok(rpassword::prompt_password(format!("Enter your password for {} : ",txt.unwrap_or(""))).unwrap())
        //decode_password(FileCredential::get_formatted_key(password.as_str()).unwrap())
    }

    pub fn parse_password(&self)->Result<String>{
        decode_password(FileCredential::get_formatted_key(&self.password.clone().unwrap_or("".to_string()).as_str())?)
        .map_err(|e| FileCredentialError::KeyringError(e))
    }

    fn get_formatted_key(pass: &str)-> Result<Vec<u8>>{
        let mut bytes: Vec<u8>=pass.as_bytes().into();
        if bytes.len() < KEY_SIZE {
            let mut fmt_key= iter::repeat(0).take(KEY_SIZE-bytes.len()).collect::<Vec<u8>>();
            fmt_key.append(&mut bytes);
            Ok(fmt_key)
        }else if bytes.len() == KEY_SIZE {
            Ok(bytes)
        }  else{
            panic!("Password cannot be greater than 32 chars")
        }
    }

    pub fn get_key(&self)->String{
        self.key.clone()
    }

    pub fn get_dir(&self)->String{
        self.dir.clone()
    }

    pub fn get_entry(&self)->Result<Entry>{
        // Entry::new_with_credential(Box::new(FileCredential::new(key_dir,key_name,
        //     Some(password.to_string()),true).unwrap()))
        Ok(Entry::new_with_credential(Box::new(self.get_credential().unwrap())))
    }

    pub fn get_entry_from_file(dir: &str, key: &str, password: Option<String>, allow_overwrite: bool)->Result<Entry>{
        Ok(Entry::new_with_credential(Box::new(FileCredential::new(dir,key,password,allow_overwrite).unwrap())))
    }
}


pub fn decode_error(err: IOError) -> ErrorCode {
    match err.kind() {
        ErrorKind::NotFound => ErrorCode::NoEntry,                        // file not found
        _ => ErrorCode::PlatformFailure(Box::new(err)),
    }
}
