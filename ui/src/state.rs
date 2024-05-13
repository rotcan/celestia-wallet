
use cel_wallet::utils::CoinMetadata;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::RngCore;

pub const PASSWORD_ERROR: &str="Password should be 6 chars long";

#[derive(Clone,Debug)]
pub struct SendDetail{
    pub to: String,
    pub amount: String,
    pub denom: String,
    pub exponent: u32,
}

impl SendDetail{

    pub fn default()->Self{
        SendDetail::new("".to_string(),"0".to_string(),"".to_string(),0)
    }
    pub fn new(to: String, amount: String, denom: String,exponent: u32)->Self{
        SendDetail {
            to,
            amount,
            denom,
            exponent
        }
    }

    pub fn set_value(&mut self, coin: &CoinMetadata){
        self.denom=coin.denom.clone();
        self.exponent=coin.exponent;
    }
}


#[derive(Clone,Debug)]
pub struct BuyBlobDetail{
    pub data: Vec<u8>,
    pub namespace: String,
    pub file_path: String,
}


impl BuyBlobDetail{

    pub fn default()->Self{
        BuyBlobDetail::new(vec![],"".to_string(),"".to_string())
    }
    pub fn new(data: Vec<u8>, namespace: String,file_path: String,)->Self{
        BuyBlobDetail {
            data,
            namespace,
            file_path,
        }
    }
}

#[derive(Clone,Debug)]
pub struct AccountData{
    pub name: String,
    pub address: String,
    pub title: String,
}

impl AccountData{

    pub fn default()->Self{
        Self::new("".to_string(),"".to_string(),"".to_string())
    }

    pub fn new(name: String, address: String,title : String,)->Self{
        AccountData{
            name,
            address,
            title
        }
    }

    pub fn update(&mut self, name: String, address: String,title: String,){
        self.name=name;
        self.address=address;
        self.title=title;
    }

    pub fn update_self(&mut self, data: AccountData){
        self.name=data.name;
        self.address=data.address;
        self.title=data.title;
    }

    pub fn update_add_account(&mut self, data: &AddAccountDetail){
        data.name.as_ref().map(|name| self.name=name.clone());
        data.title.as_ref().map(|title| self.title=title.clone());
        data.address.as_ref().map(|address| self.address=address.clone());
    }
}

#[derive(Clone,Debug)]
pub struct AddAccountDetail{
    pub name: Option<String>,
    pub title: Option<String>,
    pub address: Option<String>,
    pub private_key_hex: String,
    //pub phrase: Option<Vec<String>>,
}

impl AddAccountDetail{
    pub fn new()->Self{
        AddAccountDetail{
            name: None,
            title: None,
            address: None,
            private_key_hex : "".to_string(),
            //phrase: None,
        }
    }

    // pub fn set_name(&mut self,name: String){
    //     self.name=Some(name);
    // }
 
}

impl From<AddAccountDetail> for AccountData{
    fn from(a: AddAccountDetail)->AccountData{
        AccountData{
            name: a.name.as_ref().map_or_else(|| "".to_owned(),|name| name.to_owned()),
            address:a.address.as_ref().map_or_else(|| "".to_owned(),|address| address.to_owned() ),
            title: a.title.as_ref().map_or_else(|| "".to_owned(),|title| title.to_owned()),
        }
    }
}

#[derive(Clone,Debug)]
pub struct AccountMnemonicDetail{
    pub mnemonic: Vec<String>,
    pub verify: Vec<String>,
}

impl AccountMnemonicDetail{
    pub fn new()->Self{
        AccountMnemonicDetail{
            mnemonic: vec!["".to_owned();12],
            verify: vec![],
        }
    }

    pub fn clear(&mut self){
        self.mnemonic=vec!["".to_owned();12];
        self.verify=vec![];
    }

    pub fn set_verification_phrase(&mut self){
        
        let mut rnd=StdRng::from_entropy();
        let mut hidden_words: Vec<String>=vec![];
        for _ in 0..100{
            let index=rnd.next_u32()%12;
            let w=self.mnemonic.get(index as usize).unwrap();
            if hidden_words.iter().filter(|val| val == &w).collect::<Vec<_>>().len()==0 {
                hidden_words.push(w.to_owned());
            }
            if hidden_words.len() == 3{
                break;
            }
        }
        self.verify=vec![];
        for word in self.mnemonic.iter(){
            if hidden_words.iter().filter(|val| val == &word).collect::<Vec<_>>().len()>0 {
                self.verify.push("".to_owned());
            }else{
                self.verify.push(word.to_owned());
            }
        };
        
    }

    pub fn match_verify(&self)->bool{
        for i in 0..12{
            let m = self.mnemonic.get(i).unwrap();
            let v= self.verify.get(i).unwrap();
            if m != v {
                return false;
            };
        };
        true
    }

    pub fn break_phrase_into_mnemonic(phrase: &str)->Vec<String>{
        phrase.split(' ').collect::<Vec<&str>>().into_iter().map(|m| m.into()).collect::<Vec<String>>()
    }
    pub fn set_phrase(&mut self, phrase: &str){
        self.mnemonic=Self::break_phrase_into_mnemonic(phrase);
    }

    pub fn is_mnemonic_valid(&self)->bool{
        let empty_word="".to_owned();
        if self.mnemonic.iter().filter(|word| word==&&empty_word ).collect::<Vec<_>>().len() > 0 {
            return false;
        }
        return true;
    }

    pub fn get_phrase(&self)->String{
        self.mnemonic.join(" ")
    }

}

#[derive(Clone,Debug)]
pub struct DeleteAccountDetail{
    pub name: Option<String>,
}


impl DeleteAccountDetail{

    pub fn default()->Self{
        Self::new(None)
    }
    pub fn new(name: Option<String>)->Self{
        DeleteAccountDetail{
            name,
             
        }
    }
}
