mod accounts;
pub mod activity;
pub mod data;
pub mod settings;
pub mod user;

#[cfg(test)]
pub mod tests{
    use crate::menu::user::{User};
    use crate::CosmosCoin;
    use crate::tx::send;
    use query_client::blob;
    //use query_client::{tx::send,state::CosmosCoin};
    

    #[tokio::test]
    pub async fn account_test(){
        let password="123456";
        let chain_id="arabica-11";
        let address_prefix="celestia";
        let denom="utia";
        
        //let mut user=User::new(None);
        let mut user=User::default();
        user.load_wallet_from_mnemoic("deal story donkey sort fault unusual nothing flock dice match review survey",None,password).unwrap();
        //create account
        // user.new(None).unwrap();
        // user.load_wallet(password).unwrap();
        //assert_eq!(user.get_accounts().unwrap().len(),1);

        
        let account =user.search_account_by_name("account-0").unwrap();
        println!("account= {:?}",account.get_account_id());
        println!("next account={:?}",user.get_new_account_address());
        println!("coins = {:?}",account.get_all_balances(&user).await);
        println!("txns={:?}",account.get_txns(&user.config.clone().unwrap()).await.unwrap());
        
        //test blob
        // let tx_hash=blob::add_blob(&mut account.get_signer(&user,chain_id.to_string(),address_prefix,denom.to_string()),"test-name","test-data".as_bytes().to_vec()).await.unwrap();
        // println!("tx_hash={:?}",tx_hash);
        //delete account
        assert_eq!(user.get_accounts().unwrap_or(vec![]).len(),1);

    }

    #[tokio::test]
    pub async fn send_test(){
        let password="123456";
        let chain_id="arabica-11";
        let address_prefix="celestia";
        let denom="utia";
        //let mut user=User::new(None);
        let mut user=User::default();
        // user.load_wallet_from_mnemoic("paper mushroom betray smile demand dry liberty duck shy chronic nothing ensure carry reward fault remind blouse diamond regular cash stadium blanket elegant feature",None,password);
        user.new(None).unwrap();
        //create account
        user.load_wallet(password).unwrap();

        if user.get_accounts().unwrap().len()==0{
            user.add_account(None).unwrap();
            user.add_account(None).unwrap();
        }
        if user.get_accounts().unwrap().len()==1{
            user.add_account(None).unwrap();
        }

        //let accounts =user.get_accounts().unwrap();
        let account1=user.search_account_by_name("account-0").unwrap();
        let account2 = user.search_account_by_name("account-1").unwrap();
        println!("account 1= {:?} , 2={:?}",account1.get_account_id(),account2.get_account_id());

        let old_balance=account2.get_balance(&user,denom.to_string()).await.map(|m| m.amount);
        
        send(&mut account1.get_signer(&user,chain_id.to_string(),address_prefix,denom.to_string()), account2.get_account_id().as_str(),vec![CosmosCoin {
            amount: 1_53u128,
            denom: "utia".parse().unwrap(),
         }],None,None).await.unwrap();

        let new_balance=account2.get_balance(&user,denom.to_string()).await.map(|m| m.amount);
        let new_balance_u64: u128=u128::from_str_radix(&new_balance.unwrap_or("0".to_string()),10).unwrap();
        let old_balance_u64: u128=u128::from_str_radix(&old_balance.unwrap_or("0".to_string()),10).unwrap();
        println!("new_balance_u64={:?}",new_balance_u64);
        assert_eq!(new_balance_u64-old_balance_u64,1_53u128);

        user.remove_account(&account2.get_title());
        user.remove_account(&account1.get_title());
        

    }
    //Airdrop value
    //
}