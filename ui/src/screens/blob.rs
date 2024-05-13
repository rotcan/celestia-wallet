use crate::helper;
use cel_wallet::utils::CoinMetadata;
pub use crate::components::{
    button::WalletButton,
    text::{WalletText,CoinText,AddressText},
    tx_response::TxResponse,
};

#[derive(Clone, PartialEq)]
pub enum BuyBlobAction{
    None,
    BuyBlobClick,
    BuyBlobClickWait,
}

struct BuyBlobLabels{
    coin_label: CoinText,
    tx_response: TxResponse,
}


impl BuyBlobLabels{
    pub fn new()->Self{
        BuyBlobLabels{
            coin_label: CoinText::new(vec![12.0,10.0],
                vec![egui::Color32::WHITE,egui::Color32::WHITE],Some(0.2)),
            tx_response: TxResponse::new(0.1),
    
        }
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


struct BuyBlobButtons{
    file_dialog: WalletButton,
    buy_blob_button: WalletButton,
}


impl BuyBlobButtons{
    pub fn new()->Self{
        BuyBlobButtons{
            buy_blob_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Buy".to_string(), 0.3, 0.1,true,true),
                file_dialog: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "...".to_string(), 0.3, 0.1,false,true),
        }
    }
}


pub struct BuyBlob{
    coin_denom: String,
    coin_exponent: u32,
    balance: u128,
    buy_blob_detail: BuyBlobDetail,
    //buttons
    //labels    
    labels: BuyBlobLabels,
    buttons: BuyBlobButtons,
    action : BuyBlobAction,
    opened_file: Option<std::path::PathBuf>,
    open_file_dialog: Option<egui_file::FileDialog>,
}



impl BuyBlob{

    pub fn new()->Self{
        BuyBlob{
            coin_denom: "".to_string(),
            coin_exponent : 0,
            balance: 0,
            labels: BuyBlobLabels::new(),
            buttons: BuyBlobButtons::new(),
            buy_blob_detail: BuyBlobDetail::default(),
            action: BuyBlobAction::None,
            opened_file: None,
            open_file_dialog : None,
        }
    }

    pub fn update(&mut self, coin_denom: String,exponent: u32, balance: u128){
        self.coin_denom=coin_denom.clone();
        self.coin_exponent=exponent;
        self.balance=balance;
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

    pub fn update_action(&mut self,action: BuyBlobAction){
        self.action=action;
    }

    pub fn update_labels(&mut self){
        
        self.labels.coin_label.set_value(self.coin_denom.clone(),
        helper::get_rounded_value(self.balance,self.coin_exponent),
        helper::get_rounded_value(self.balance,self.coin_exponent));
    }

    pub fn get_action(&self)->BuyBlobAction{
        self.action.clone()
    }

    pub fn get_blob_details(&self)->BuyBlobDetail{
        self.buy_blob_detail.clone()
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
        let file_path_width=0.8*width;
        let button_width=0.1*width;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(margin_top);
                //coin
                self.labels.coin_label.ui(ui,width,height);
                 //name space
                ui.horizontal(|ui| {
                    let namespace_text=egui::TextEdit::singleline(&mut self.buy_blob_detail.namespace)
                        .desired_width(address_width)
                        .min_size(egui::Vec2::new(address_width,text_height))
                        .vertical_align(egui::Align::Center);
                    ui.add_space(10.0);
                    ui.add(namespace_text);
                //   println!("self.receiver_detail.to={}",self.receiver_detail.to);
                    
                });
                //file
                ui.horizontal(|ui|{
                    let namespace_text=egui::TextEdit::singleline(&mut self.buy_blob_detail.file_path)
                        .desired_width(file_path_width)
                        .min_size(egui::Vec2::new(file_path_width,text_height))
                        .vertical_align(egui::Align::Min);
                    ui.add_space(10.0);
                    ui.add(namespace_text);

                    self.buttons.file_dialog.ui(ui,width,height);
                    if self.buttons.file_dialog.get_is_clicked() && self.open_file_dialog.is_none() {
                        let mut dialog = egui_file::FileDialog::open_file(self.opened_file.clone())
                        .default_size(egui::Vec2::new(width,height-64.0));
                        dialog.open();
                        self.open_file_dialog = Some(dialog);
                    }
                });
                
                ui.horizontal(|ui|{
                    self.buttons.buy_blob_button.ui(ui,width,height);
                    if self.buttons.buy_blob_button.get_is_clicked() 
                    && self.buy_blob_detail.file_path.len() > 0 {
                        let bytes=self.buy_blob_detail.namespace.as_bytes();
                        if bytes.len()>9 {
                            self.labels.tx_response.set_state(crate::components::tx_response::TxState::Failure, "Error: Namespace should be less than 9 bytes".to_string());
                            
                        }else{
                        //load file and get byte data
                            self.buy_blob_detail.data = cel_wallet::utils::load_file(self.buy_blob_detail.file_path.clone()).unwrap();
                            self.labels.tx_response.set_state(crate::components::tx_response::TxState::Pending, "Pending".to_string());
                            self.action=BuyBlobAction::BuyBlobClick;
                        }
                    };
                });
                //response
                self.labels.tx_response.ui(ui,width,height);
            });
            
        });

        
        self.ctx_update(ctx,width,height);

    }

    fn ctx_update(&mut self,ctx: &egui::Context,  width: f32, height: f32){
        if let Some(dialog) = &mut self.open_file_dialog {
            match dialog.show(ctx).state() {
                egui_file::State::Selected => {
                    if let Some(file) = dialog.path() {
                        self.opened_file = Some(file.to_path_buf());
                        // println!("opened_file={:?}",self.opened_file);
                        self.opened_file.clone().map(|m| 
                            self.buy_blob_detail.file_path=m.into_os_string().into_string().unwrap()
                        );
                    };
                },
                egui_file::State::Closed =>{
                    self.open_file_dialog=None;
                },
                egui_file::State::Cancelled =>{
                    self.open_file_dialog=None;
                },
                _ =>{ }
            };
        };
    }

    

}

