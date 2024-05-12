pub mod state;
pub mod secp256k1_key;
pub mod error;
// pub const APP_NAME: &str="rust_keyring";
// use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

// fn init_tracing(){

//     std::env::set_var("RUST_LOG", "info");

//     tracing_subscriber::fmt()
//             .with_env_filter(
//                 EnvFilter::from_default_env().add_directive(
//                     format!("{}={}", APP_NAME,"info")
//                         .parse()
//                         .expect("Error parsing directive"),
//                 ),
//             )
//             .with_span_events(FmtSpan::FULL)
//             .init();
// }
#[cfg(test)]
mod tests {
    use super::*;
    // use josekit::{jwe::JweContext,jwe::alg::direct::DirectJweAlgorithm,
    //     jwe::deserialize_compact,jwe::serialize_compact,jwe::JweHeader};
    // use std::iter;
    use crate::state::{FileCredential,Metadata};
    use crate::secp256k1_key::create_key;
    use keyring::{credential::CredentialApi,Error,Result,Entry};
    // use keyring::macos::{MacCredential,MacCredentialBuilder};

    // #[test]
    // pub fn test_josekit(){
    //     let payload=b"Hello";
    //     // let ctx=JweContext::new();
    //     let alg = DirectJweAlgorithm::Dir;
    //     let cipher="A128CBC-HS256";
    //     let key_len=32 as usize;
    //     let mut key:Vec<u8>="test".as_bytes().into();
    //     println!("key={:?}",key);
    //     let mut fmt_key=iter::repeat(0).take(key_len-key.len()).collect::<Vec<u8>>();
    //     fmt_key.append(&mut key);
    //     let encrypter = alg.encrypter_from_bytes(&fmt_key).unwrap();
    //     let mut header = JweHeader::new();
    //     header.set_content_encryption(cipher);
    //     let jwe = serialize_compact(payload, &header, &encrypter).unwrap();
    //     println!("jwe={}", jwe);

    //     let mut fmt_key_2=iter::repeat(0).take(key_len-"test2".len()).collect::<Vec<u8>>();
    //     fmt_key_2.append(&mut "test2".as_bytes().into());
        
    //     let decrypter = alg.decrypter_from_bytes(&fmt_key_2).unwrap();
    //     let (data, header) = deserialize_compact(&jwe, &decrypter).unwrap();
    //     println!("header={}",header);
    //     // assert_eq!(data, payload);
    // }

    #[test]
    pub fn test_file_credential(){
        //New with credential just creates an object with path for creds without store anythign
        //get credential returns empty , no file stored
        //set password -> stores encoded value to file
        //create credential

        let entry=create_key(".","test","123","m/44'/118'/0'/0/0","celestia",true).unwrap();
        // let entry=Entry::new_with_credential(Box::new(FileCredential::new(".","test",Some("1234".to_string()),true).unwrap()));
        // let mut credential: &FileCredential = entry
        //     .get_credential()
        //     .downcast_ref()
        //     .expect("Not a mac credential");
        // //bip passphrase 
        // assert!(matches!(entry.get_password(), Err(Error::NoEntry)));
        // //println!("Cred get");
        // //save password
        // let metadata=Metadata::new(create_mnemonic("123","m/44'/118'/0'/0/0","celestia"));
        // let password=credential.encode_data(&metadata).unwrap();
        // entry
        //     .set_password(&password.as_str())
        //     .expect("Can't set password for get_credential");
        // assert!(credential.get_credential().is_ok());
        println!("Cred set");
        
        let credential2: &FileCredential = entry
            .get_credential()
            .downcast_ref()
            .expect("Not a mac credential");
        println!("Cred get");
        //delete password
        entry
            .delete_password()
            .expect("Couldn't delete after get_credential");
        println!("Cred deleted");
        assert!(matches!(entry.get_password(), Err(Error::NoEntry)));
    }    


    // pub fn entry_from_constructor<F, T>(f: F, service: &str, user: &str) -> Entry
    // where
    //     F: FnOnce(Option<&str>, &str, &str) -> Result<T>,
    //     T: 'static + CredentialApi + Send + Sync,
    // {
    //     match f(None, service, user) {
    //         Ok(credential) => Entry::new_with_credential(Box::new(credential)),
    //         Err(err) => {
    //             panic!("Couldn't create entry (service: {service}, user: {user}): {err:?}")
    //         }
    //     }
    // }

    // fn entry_new(service: &str, user: &str) -> Entry {
    //     entry_from_constructor(MacCredential::new_with_target, service, user)
    // }

    // #[test]
    // fn test_mac() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    //     let entry=entry_new("test_service","test_name");
    //     let credential: &MacCredential = entry
    //         .get_credential()
    //         .downcast_ref()
    //         .expect("Not a mac credential");
    //     assert!(
    //         credential.get_credential().is_err(),
    //         "Platform credential shouldn't exist yet!"
    //     );
    //     entry
    //         .set_password("test get_credential")
    //         .expect("Can't set password for get_credential");
    //     assert!(credential.get_credential().is_ok());
    //     println!("pass={:?}",entry.get_password());
    //     entry
    //         .delete_password()
    //         .expect("Couldn't delete after get_credential");
    //     assert!(matches!(entry.get_password(), Err(Error::NoEntry)));
    // }
}
