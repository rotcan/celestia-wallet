use cel_wallet::menu::{user::User};
use cel_wallet::utils::CoinMetadata;
use crate::helper;
use crate::components::{
    text::{CoinText,AddressText},
    button::WalletButton
};
use crate::api::balance;
//use std::sync::mpsc::{channel, Receiver, Sender};


#[derive(PartialEq,Copy,Clone,Debug)]
pub enum HomeState{
    Default,
    Send,
    Blob,
}

pub struct Home{
   // account_id: String,
   active_account_name: String,
   balances: Option<Vec<CoinMetadata>>,
   balance_promise: balance::BalancePromise,
   main_coin: Option<CoinMetadata>,
   transfer_coin: Option<CoinMetadata>,
    labels: HomeLabels,
    buttons: HomeButtons,
    state: HomeState,
//    send: Sender<Account>,
//    recv: Receiver<Vec<Balance>>,
}

#[derive(Clone)]
pub struct HomeLabels{
    main_coin: CoinText,
    address: AddressText,
    _all_coins: Vec<CoinText>,
}

impl HomeLabels{
    pub fn default()->Self{
        HomeLabels{
            main_coin: CoinText::new(vec![18.0,9.0],
                vec![egui::Color32::WHITE,egui::Color32::WHITE],Some(0.2)),
            address: AddressText::new(vec![12.0,6.0],
                vec![egui::Color32::WHITE,egui::Color32::WHITE],Some(0.05)),
            _all_coins: vec![],
        }
    }

    pub fn set_main_coin(&mut self,symbol: String, value: String, usd: Option<String> ){
        self.main_coin.set_value(symbol,value,usd);
        self.main_coin.set_back_color(egui::Color32::DARK_GRAY);
    }

    pub fn set_address_value(&mut self, address : String, title: String){
        self.address.set_value(address,title);
        
    }
}

#[derive(Clone)]
pub struct HomeButtons{
    pub send_button: WalletButton,
    pub buy_blob_button: WalletButton,
}

impl HomeButtons{
    pub fn new()->Self{
        HomeButtons{
            send_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Send".to_string(), 0.3, 0.1,false,true),
            buy_blob_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Buy Blob".to_string(), 0.3, 0.1,false,true),
        }
    }
}

impl Home{
    pub fn default()->Self{
        // let (send, recv) = channel();
        Home{
            active_account_name: "".to_string(),
            balances: None,
            balance_promise: balance::BalancePromise::new(),
            main_coin: None,
            transfer_coin: None,
            labels: HomeLabels::default(),
            buttons: HomeButtons::new(),
            state: HomeState::Default,
            // send,
            // recv,
        }
    }

    pub fn get_state(&self)->HomeState{
        self.state
    }

    pub fn set_active_account_name(&mut self, account_name: String,address_address: String,){
        self.active_account_name=account_name.clone();
        self.labels.set_address_value(address_address.clone(),account_name);
    }

    pub fn get_active_account_name(&self)->String{
        self.active_account_name.clone()
    }

    pub fn get_coin_to_transfer(&self)->Option<CoinMetadata>{
        self.transfer_coin.clone()
    }

    pub fn get_main_coin(&self)->Option<CoinMetadata>{
        self.main_coin.clone()
    }

    pub fn update_balances_from_app(&mut self,coins: &Vec<CoinMetadata>){
        if let Some(main_coin)=&self.main_coin{
            for c in coins.iter() {
                if c.denom == main_coin.denom {
                    self.labels.set_main_coin(c.symbol.clone(), 
                    helper::get_rounded_value(c.balance,c.exponent),
                    helper::get_rounded_value(c.balance,c.exponent));
                
                };
            };
        }
    }

    pub fn update_balances(&mut self, user: &User){
        self.balance_promise.init(user,&self.active_account_name.clone());
       
    }
 


    //pub fn ui(&mut self,ui: &mut egui::Ui){
    pub fn ui(&mut self,ctx: &egui::Context){
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let width=screen_size.x-16.0;
        let height=screen_size.y;

        egui::CentralPanel::default().show(ctx, |ui| {

            //address
            self.labels.address.label.ui(ui,width,height);
            if self.labels.address.label.get_is_clicked(){
                ctx.output_mut(|o| o.copied_text = self.labels.address.value.clone());
                println!("copied value non android");
                
            };
            //main balance
            ui.allocate_ui_with_layout(egui::Vec2::new(width,height*0.3),egui::Layout::default(),
              |ui|{  egui::Frame::default().fill(egui::Color32::DARK_GRAY).show(ui,|ui| 
                {
                    self.labels.main_coin.ui(ui,width,height);
                    
                })
            }
            );
            //buttons
            ui.horizontal(|ui|{
                ui.add_space(width*0.18);
                self.buttons.send_button.ui(ui,width,height);
                if self.buttons.send_button.get_is_clicked() {
                    //Set main coin for now
                    self.transfer_coin=self.main_coin.clone();
                    self.state=HomeState::Send;
                    
                }
                self.buttons.buy_blob_button.ui(ui,width,height);
                if self.buttons.buy_blob_button.get_is_clicked() {
                    self.state=HomeState::Blob;
                }
            });
        
            
        });

        
        self.frame_update();
    }

    fn frame_update(&mut self){
        self.balance_promise.check_result();
        self.balance_promise.consume_result().map(|m| {
            self.balances=Some(m.clone());
            self.main_coin=self.balances.clone().as_ref().map(|m| m.first().clone()).unwrap().cloned();
            self.main_coin.clone().map(|m| self.labels.set_main_coin(m.symbol.clone(), 
                helper::get_rounded_value(m.balance,m.exponent),
                helper::get_rounded_value(m.balance,m.exponent)
            ));
        });
        self.balance_promise.consume_failure().map(|_|{});
    }
}
