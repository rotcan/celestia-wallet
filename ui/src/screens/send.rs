use crate::helper;
use cel_wallet::utils::CoinMetadata;

pub use crate::components::{
    button::WalletButton,
    text::{WalletText,CoinText,AddressText},
    tx_response::TxResponse,
};

#[derive(Clone, PartialEq)]
pub enum SendAction{
    None,
    SendClick,
    SendClickWait,
}
struct SendLabels{
    coin_label: CoinText,
    tx_response: TxResponse,
}

impl SendLabels{
    pub fn new()->Self{
        SendLabels{
            coin_label: CoinText::new(vec![12.0,10.0],
                vec![egui::Color32::WHITE,egui::Color32::WHITE],Some(0.2)),
                tx_response: TxResponse::new(0.1),
    
        }
    }
}

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

struct SendButtons{
    send_button: WalletButton,
}

impl SendButtons{
    pub fn new()->Self{
        SendButtons{
            send_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Send".to_string(), 0.3, 0.1,false,true),
        }
    }
}

pub struct Send{
    coin_denom: String,
    coin_exponent: u32,
    balance: u128,
    receiver_detail: SendDetail,
    //buttons
    //labels    
    labels: SendLabels,
    buttons: SendButtons,
    action : SendAction,
}


impl Send{

    pub fn new()->Self{
        Send{
            coin_denom: "".to_string(),
            coin_exponent : 0,
            balance: 0,
            labels: SendLabels::new(),
            buttons: SendButtons::new(),
            receiver_detail: SendDetail::default(),
            action: SendAction::None,
        }
    }

    pub fn update(&mut self, coin_denom: String,exponent: u32, balance: u128){
        self.coin_denom=coin_denom.clone();
        self.coin_exponent=exponent;
        self.balance=balance;
        self.receiver_detail.denom=coin_denom;
        self.receiver_detail.exponent=exponent;
        self.update_labels();
    }

    pub fn update_balances_from_app(&mut self,coins: &Vec<CoinMetadata>){
        for c in coins.iter() {
            if c.denom == self.coin_denom {
                self.balance=c.balance;
                self.update_labels();
            };
        };
    }

    pub fn update_action(&mut self,action: SendAction){
        self.action=action;
    }

    pub fn update_labels(&mut self){
        
        self.labels.coin_label.set_value(self.coin_denom.clone(),
        helper::get_rounded_value(self.balance,self.coin_exponent),
        None);
    }

    pub fn get_action(&self)->SendAction{
        self.action.clone()
    }

    pub fn get_receiver_details(&self)->SendDetail{
        self.receiver_detail.clone()
    }

    pub fn update_tx_response(&mut self,success_message: Option<String>, error_message: Option<String>){
        if error_message.is_some() {
            self.labels.tx_response.set_state(crate::components::tx_response::TxState::Failure, error_message.unwrap());
        }else if success_message.is_some(){
            self.labels.tx_response.set_state(crate::components::tx_response::TxState::Success, success_message.unwrap());
        }
    }

    pub fn ui(&mut self,ctx: &egui::Context){
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let width=screen_size.x-16.0;
        let height=screen_size.y;

        let margin_top=height * 0.1;
        let address_width=0.9*width;
        let text_height=0.1*height;
        let amount_width=0.4*width;
        let button_width=0.1*width;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(margin_top);
                //coin
                self.labels.coin_label.ui(ui,width,height);
                 //address
                 ui.horizontal(|ui| {
                    let receiver_text=egui::TextEdit::singleline(&mut self.receiver_detail.to)
                        .desired_width(address_width)
                        .min_size(egui::Vec2::new(address_width,text_height))
                        .vertical_align(egui::Align::Center);
                    ui.add_space(10.0);
                    ui.add(receiver_text);
                //   println!("self.receiver_detail.to={}",self.receiver_detail.to);
                    
                });
                //amount
                ui.horizontal(|ui| {
                    let receiver_text=egui::TextEdit::singleline(&mut self.receiver_detail.amount)
                        .desired_width(amount_width)
                        .min_size(egui::Vec2::new(amount_width,text_height))
                        .vertical_align(egui::Align::Center);
                    ui.add_space(10.0);
                    ui.add(receiver_text);
                    self.buttons.send_button.ui(ui,width,height);
                    if self.buttons.send_button.get_is_clicked() {
                        self.labels.tx_response.set_state(crate::components::tx_response::TxState::Pending, "Pending".to_string());
                        self.action=SendAction::SendClick;
                    };
                    
                });
               
                //response
                self.labels.tx_response.ui(ui,width,height);
            });
            
        });

    }

    

}

