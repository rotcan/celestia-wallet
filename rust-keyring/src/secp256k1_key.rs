use bip39::{Language, Mnemonic, MnemonicType, Seed};
//use tracing::info;
use std::str::FromStr;
use keyring::{Entry};
use crate::state::{FileCredential,Metadata,Result};
//use bip32::{ExtendedPrivateKey,DerivationPath};
use cosmrs::crypto::PublicKey;
use cosmrs::crypto::secp256k1::SigningKey;


pub fn create_key(
    key_dir: &str, key_name: &str,
    password: &str,
    //seed: Vec<u8>,derivation_path: &str, 
    private_key_bytes: Vec<u8>,
    overwrite: bool)->Result<Entry>{
    let entry=Entry::new_with_credential(Box::new(FileCredential::new(key_dir,key_name,
        Some(password.to_string()),overwrite).unwrap()));
    let credential: &FileCredential = entry
        .get_credential()
        .downcast_ref()
        .expect("Not a file credential");
    //let metadata=Metadata::new(create_mnemonic(derivation_path,address_prefix));
    let metadata=Metadata::new(private_key_bytes);
    let file_pass=credential.encode_data(&metadata).unwrap();
    entry
        .set_password(&file_pass.as_str())
        .expect("Can't set password for get_credential");
    credential.get_credential()?;
    Ok(entry)
}

// pub fn create_key_from_seed(key_dir: &str, key_name: &str,
//     password: &str,seed: Vec<u8>){
//     let derived_key=SigningKey::from_slice(&seed[..]).unwrap();
//     // ExtendedPrivateKey::from(&derived_key)
    
// }

pub fn get_new_mnemoic(language: Option<&str>)->Mnemonic{
    
    Mnemonic::new(MnemonicType::Words12, Language::from_language_code(language.unwrap_or("en")).unwrap())
}

pub fn create_new_seed_from_phrase(phrase: &str, language: Option<&str>)->Vec<u8>{
    let mnemonic= Mnemonic::from_phrase(phrase,Language::from_language_code(language.unwrap_or("en")).unwrap()).unwrap();
    create_new_seed_from_mnemonic(&mnemonic)
}

pub fn create_new_seed_from_mnemonic(mnemonic: &Mnemonic)->Vec<u8>{
    let seed: Vec<u8> =Seed::new(mnemonic, "").as_bytes().into();
    seed
}

pub fn derive_private_key(seed: Vec<u8>,derivation_path: &str)->Vec<u8>{
    let derived_key: &cosmrs::bip32::XPrv=&cosmrs::bip32::ExtendedPrivateKey::derive_from_path(seed,
    &cosmrs::bip32::DerivationPath::from_str(derivation_path).unwrap()).unwrap();
   // let derived_key: &cosmrs::bip32::XPrv=&cosmrs::bip32::ExtendedPrivateKey::new(seed).unwrap();
    //println!("derived_key={:?} {:?}",derived_key,derivation_path);
    derived_key.to_bytes().to_vec()
}

// fn create_mnemonic(derivation_path: &str, address_prefix: &str)->Vec<u8>{
//     let mnemonic = get_new_mnemoic(None);
//     info!("Key mnemonic={:?}",mnemonic);
//     //let seed: Vec<u8> =Seed::new(&mnemonic, password).as_bytes().into();
    
//     // let derived_key=SigningKey::derive_from_path(seed.clone(),
//     // &cosmrs::bip32::DerivationPath::from_str(derivation_path).unwrap()).unwrap();
//     // let public_key=derived_key.public_key();
//     // //info!("account bytes={:?} {}",public_key.to_bytes(),public_key.to_bytes().len());
//     // let account=public_key.account_id(address_prefix).unwrap();
//     // info!("account={}",account);
//     // let key_bytes: &[u8]=derived_key.into();
//     // info!("key={:?}",key_bytes);
    
//     //seed
//     create_new_seed_from_mnemonic(&mnemonic)
// }

pub fn get_public_key(private_key: Vec<u8>)->PublicKey{
    // let derived_key_bytes=derive_private_key(seed,derivation_path);
    let derived_key=SigningKey::from_slice(&private_key[..]).unwrap();
    derived_key.public_key()
}

pub fn get_account_id(address_prefix: &str, pubkey: &PublicKey)->String{
    pubkey.account_id(address_prefix).unwrap().to_string()
}