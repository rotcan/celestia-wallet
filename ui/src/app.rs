use egui::{Context};
// use crate::screens::{
//     splash::Splash,home::Home,send::{Send,SendDetail},blob::{BuyBlob,BuyBlobDetail},
// };
use cel_wallet::{menu::user::{User,UserState},utils::CoinMetadata};
//use std::convert::TryInto;
use crate::helper;
use crate::state::{BuyBlobDetail,SendDetail,AccountData,AddAccountDetail,
    DeleteAccountDetail,AccountMnemonicDetail};
use crate::api::{balance,buy_blob,send,txn_list::TxnListPromise};
use crate::blocks::{new_wallet::NewWalletBlock,existing_wallet::ExistingWalletBlock
    ,message::MessageBlock,
    gas::GasBlock,balance::BalanceBlock,
    top_panel::TopPanelBlock,
    home_buttons::HomeButtonsBlock,
    send::SendBlock,
    buy_blob::BuyBlobBlock,
    bottom_panel::BottomPanelBlock,
    account_list::AccountListBlock,
    add_account::AddAccountBlock,
    txn_list::TxnListBlock,
};
use crate::components::timer::Timer;
use crate::components::tx_response::TxState;
use cel_wallet::tx::TxnListResponse;

#[derive(PartialEq)]
pub enum AppView{
    SplashExisting,
    SplashImport,
    SplashNewP1,
    SplashNewP2,
    SplashNewP3,
    LoadUser,
    Home,
    SendScreen,
    BuyBlobScreen,
    AccountListScreen,
    AddAccountScreen,
    TxnList,
}

#[derive(PartialEq)]
pub enum AppState{
    None,
    //Splash
    SplashWalletSelect,
    SplashWalletNew,
    SplashWalletExisting,
    SplashPasswordCheck,
    SplashPasswordReset,

    //Main Screen
    SendView,
    BuyBlobView,

    //SendView
    SendClick,
    SendClickWait,

    //Buy Blob
    BuyBlobClick,
    BuyBlobWait,

    //
    ActiveAccountUpdate,
    AddAccountPubkey,
    AddAccountClick,
    AccountDeleteClick,
    ImportPrivateKeyClick,
    NewAccountNext,
    SplashReset,
    VerifyMnemonic,
    MatchMnemonic,
    ImportMnemonic,

    TxnListClick,
    SettingsClick,
    HomeClick,
    ShowError(String),

}

pub const CHAIN_ID : &str = "arabica-11";
pub const ADDRESS_PREFIX : &str ="celestia";
pub const DEFAULT_SYMBOL: &str ="TIA";
pub const DEFAULT_EXPONENT: u32= 6;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct App{
    user: User,
    app_state: AppState,
    view: AppView,
    timer: Timer,
    password: String,
    
    blocks: AppBlocks,
    
    balances: Option<Vec<CoinMetadata>>,
    txns: Option<Vec<TxnListResponse>>,
    fee_coin: Option<CoinMetadata>,
    tx_gas: Option<cel_wallet::CosmosGas>,
    
    send_promise: send::SendPromise,
    buy_blob_promise: buy_blob::BuyBlobPromise,
    balance_promise: balance::BalancePromise,
    txn_list_promise: TxnListPromise,
    
    buy_blob_detail: BuyBlobDetail,
    send_detail: SendDetail,
    account_select_detail: AccountData,
    add_account_detail: AddAccountDetail,
    delete_account_detail: DeleteAccountDetail,
    mnemonic_detail: AccountMnemonicDetail,
}

pub struct AppBlocks{
    //New Wallet
    pub new_wallet: NewWalletBlock,
    //Existing Wallet
    pub existing_wallet: ExistingWalletBlock,
    //Top bar 
    pub top_panel: TopPanelBlock,
    //Main Balance
    pub balance: BalanceBlock,
    //home button
    pub home_buttons: HomeButtonsBlock,
    //Gas
    pub gas: GasBlock,
    //Success/Failure
    pub message: MessageBlock,
    //Bottom bar
    pub bottom_panel: BottomPanelBlock,
    //Send panel
    pub send: SendBlock,
    pub buy_blob: BuyBlobBlock,
    pub account_list: AccountListBlock,
    pub add_account: AddAccountBlock,
    pub txn_list: TxnListBlock,
}

impl AppBlocks{
    pub fn new()->Self{
        AppBlocks{
            new_wallet: NewWalletBlock::new(),
            existing_wallet: ExistingWalletBlock::new(),
            message: MessageBlock::new(),
            balance: BalanceBlock::new(),
            gas: GasBlock::new(),
            top_panel: TopPanelBlock::new(),
            bottom_panel: BottomPanelBlock::new(),
            home_buttons: HomeButtonsBlock::new(),
            send: SendBlock::new(),
            buy_blob: BuyBlobBlock::new(),
            account_list: AccountListBlock::new(),
            add_account: AddAccountBlock::new(),
            txn_list: TxnListBlock::new(),
            
        }
    }

    // pub fn ui(&mut self, ui: &egui::Ui, width: f32, height: f32){

    //     self.frame_update();
    // }

    // fn frame_update(&mut self){

    // }
}


pub fn is_mobile(ctx: &Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}

impl App{
    pub fn new(language: &str)->Self{
        let mut user = User::default();
        user.new(Some(language)).unwrap();
        let view= match user.get_state() {
            //new or load from mnemoic
            UserState::New =>{
                AppView::SplashNewP1
            },
            UserState::Existing =>{
                AppView::SplashExisting
            },
            _=>{AppView::SplashExisting}
        };
        let state=AppState::None;
        let mut app=App{
            
            view,
            user: user,
            balances: None,
            txns: None,
            
            app_state : state,
            send_promise: send::SendPromise::new(),
            buy_blob_promise: buy_blob::BuyBlobPromise::new(),
            balance_promise: balance::BalancePromise::new(),
            txn_list_promise: TxnListPromise::new(),

            tx_gas: None,
            password: "".to_string(),
            blocks: AppBlocks::new(),
            buy_blob_detail: BuyBlobDetail::default(),
            send_detail: SendDetail::default(),
            account_select_detail: AccountData::default(),
            delete_account_detail: DeleteAccountDetail::default(),
            fee_coin: None,
            timer: Timer::new(1),
            add_account_detail: AddAccountDetail::new(),
            mnemonic_detail: AccountMnemonicDetail::new(),
        };
        app.blocks.existing_wallet.set_visible(true);
        app
    } 

    // pub fn update_visibility(&mut self){
    //     match self.app_state {

    //         _={},
    //     }
    // }

    pub fn ui(&mut self,ctx: &Context){
        self.mobile_ui(ctx);
    }

    fn check_password(&mut self){
        //if self.splash.get_state() == crate::screens::splash::SplashState::PasswordCheck {
        if self.app_state == AppState::SplashPasswordCheck{
            self.app_state=AppState::None;
            let _ = self.user.load_wallet(self.password.as_str())
            .map(|_| {
            
               self.load_user();
                
            })
            .map_err(|_e| {
                //self.splash.update_password_result(Some("Password incorrect!".to_string()));
                //Show error
                self.blocks.message.update_tx_response(TxState::Failure,"Password incorrect!".to_string());
            });
            self.app_state = AppState::None;
        };

        if self.app_state == AppState::SplashPasswordReset {
            self.app_state = AppState::None;
            self.blocks.message.clear_state();

        }
    }

    fn load_user(&mut self){
        let account_data_list=self.user.get_accounts().unwrap().iter().map(|val| 
            AccountData::new( val.value.clone() ,val.account_id.clone(), val.title.clone(),))
        .collect::<Vec<AccountData>>();
        //println!("account_data_list={:?}",account_data_list);
        self.blocks.account_list.add_accounts(account_data_list);
        
        let current_account=self.user.get_current_account_address_detail();
        
        self.account_select_detail.update(self.user.get_current_account_name(),
        current_account.0.clone(),current_account.1.clone());
        self.update_current_account(); 
        self.blocks.message.clear_state();
        self.view=AppView::Home;
    }

    fn update_current_account(&mut self){
        //balances
        self.balance_promise.init(&self.user,&self.account_select_detail.name);
        //txns
        self.txn_list_promise.init(&self.user,&self.account_select_detail.name);

        self.blocks.message.clear_state();
        self.blocks.top_panel.set_address_value(self.account_select_detail.name.clone(),
        self.account_select_detail.address.clone(),self.account_select_detail.title.clone());

    }

    fn check_send(&mut self){
        //if self.send.get_action() == crate::screens::send::SendAction::SendClick {
        if self.app_state == AppState::SendClick{
            self.app_state= AppState::None;
            // self.send.update_action(crate::screens::send::SendAction::SendClickWait);
            //if self.active_account_name.is_some() {
            self.send_promise.init_tx(&self.user,
                //&self.active_account_name.clone().unwrap(),
                &self.account_select_detail.name,
                self.send_detail.clone(),
                self.tx_gas);
            //};
            self.blocks.message.update_tx_response(TxState::Pending,"Pending!".to_owned());
        }
    }

    fn check_active_account(&mut self){
        if self.app_state== AppState::ActiveAccountUpdate {
            self.app_state=AppState::None;
            // self.balance_promise.init(&self.user,&self.account_select_detail.name);
            // self.blocks.message.clear_state();
            // self.blocks.top_panel.set_address_value(self.account_select_detail.name.clone(),
            //     self.account_select_detail.address.clone(),self.account_select_detail.title.clone());
            self.update_current_account();
        }

        if self.app_state == AppState::AccountDeleteClick {
            self.app_state=AppState::None;
            if self.delete_account_detail.name.is_some() {
                let name=self.delete_account_detail.name.clone().unwrap();
                self.user.remove_account(&name);
                self.blocks.account_list.remove_account(&name)
            }
            
            
            //get root account
            self.user.set_current_account("account-0".to_owned());
            let root_account=self.user.get_current_account_address_detail();
                
            self.account_select_detail.update(self.user.get_current_account_name(),
            root_account.0.clone(),root_account.1.clone());
            //update view
            self.update_current_account();
            self.view=AppView::Home;
        }
    }

    fn check_add_account(&mut self){
        if self.app_state == AppState::AddAccountPubkey{
            self.app_state=AppState::None;
            if self.view != AppView::AddAccountScreen{
                self.blocks.message.tx_response.clear_state();
            };
            match self.user.get_new_account_address(){
                Ok((name,address))=>{
                    //println!("update account {} {} ",address,name);
                    self.add_account_detail.address=Some(address.clone());
                    self.add_account_detail.name=Some(name.clone());
                    self.add_account_detail.title=Some(name.clone());
                    //Set this as current account
                    self.account_select_detail.update_self(self.add_account_detail.clone().into());
                    self.update_current_account();
                }
                Err(_)=>{
                    //todo Add message on screen
                }
            };
            
        }
        if self.app_state == AppState::AddAccountClick {
            self.app_state=AppState::None;
            self.user.add_account(self.add_account_detail.title.clone()).unwrap();
            //add to account list
            self.blocks.account_list.add_accounts(
                vec![AccountData::new( self.add_account_detail.name.clone().unwrap() ,
                self.add_account_detail.address.clone().unwrap(), self.add_account_detail.title.clone().unwrap(),)]);
            
            //set current account
            self.account_select_detail.update_add_account(&self.add_account_detail);
            
            self.update_current_account();
            if self.view != AppView::AddAccountScreen{
                self.blocks.message.tx_response.clear_state();
            };
            self.view=AppView::Home;
        }

        if self.app_state == AppState::SettingsClick{
            self.app_state=AppState::None;

            if self.view!=AppView::Home {
                self.blocks.message.tx_response.clear_state();
            };
            self.view=AppView::Home;
        }

        if self.app_state == AppState::HomeClick{
            self.app_state=AppState::None;
            if self.view!=AppView::Home {
                self.blocks.message.tx_response.clear_state();
            };
            self.view=AppView::Home;
        }

        if self.app_state == AppState::TxnListClick{
            self.app_state=AppState::None;
            if self.view!=AppView::TxnList {
                self.blocks.message.tx_response.clear_state();
            };
            self.view=AppView::TxnList;
        }
    }

    fn check_splash(&mut self){
        if self.app_state == AppState::SplashReset {
            //create mnemonic
            self.app_state=AppState::None;
            self.view=AppView::SplashNewP1;
            self.mnemonic_detail.set_phrase(&User::get_mnemonic(None));
        }
        if self.app_state == AppState::VerifyMnemonic{
            self.app_state = AppState::None;
            self.mnemonic_detail.set_verification_phrase();
            self.view=AppView::SplashNewP3;
           
        }

        if self.app_state == AppState::MatchMnemonic {
            self.app_state = AppState::None;
            if self.mnemonic_detail.match_verify() {
                //Load user 
                match self.user.load_wallet_from_mnemoic(&self.mnemonic_detail.get_phrase(),None,&self.password){
                    Ok(_)=>{
                        self.load_user()
                        
                    },
                    Err(_)=>{
                        self.blocks.message.update_tx_response(TxState::Failure,
                            "Error: Could not load Wallet".to_string());
                    }
                }
                //
                
            }else{
                self.blocks.message.update_tx_response(TxState::Failure,
                    "Error: Mnemonic did not match".to_string());
            };
        }
        if self.app_state == AppState::ImportMnemonic {
            self.app_state=AppState::None;
            //Check if mnemoic is value
            if self.mnemonic_detail.is_mnemonic_valid() == false{
                self.blocks.message.update_tx_response(TxState::Failure,
                    "Mnemonic invalid".to_string());
            }else{
                match self.user.load_wallet_from_mnemoic(&self.mnemonic_detail.get_phrase(),None,&self.password){
                    Ok(_)=>{
                        self.load_user()
                        
                    },
                    Err(_)=>{
                        self.blocks.message.update_tx_response(TxState::Failure,
                            "Error: Could not load Wallet".to_string());
                    }
                }
            }
            //import mnemonic
        }
    }
    

    fn check_buy_blob(&mut self){
        if self.app_state == AppState::BuyBlobClick {
            //self.blob.update_action(crate::screens::blob::BuyBlobAction::BuyBlobClickWait);
            self.app_state= AppState::None;
            // println!("check buy blob");
         
            //if self.active_account_name.is_some() {
                if self.buy_blob_detail.namespace.len()>0 
                 && self.buy_blob_detail.file_path.len() > 0 {
                    let bytes=self.buy_blob_detail.namespace.as_bytes();
                        if bytes.len()>9 {
                            self.blocks.message.update_tx_response(TxState::Failure,
                                "Error: Namespace should be less than 9 bytes".to_string());
                            
                        }
                        match cel_wallet::utils::load_file(self.buy_blob_detail.file_path.clone()){
                            Ok(data)=>{
                                self.buy_blob_detail.data = data;
                                self.blocks.message.update_tx_response(TxState::Pending, 
                                    "Pending".to_string());
                                self.buy_blob_promise.init_tx(&self.user,
                                    //&self.active_account_name.clone().unwrap(),
                                    &self.account_select_detail.name,
                                    self.buy_blob_detail.clone(),
                                    self.tx_gas);
                            },
                            Err(_)=>{
                                self.blocks.message.update_tx_response(TxState::Failure,
                                    "Error in loading file".to_string());
                            }
                        }
                    
                    }else{
                        if self.buy_blob_detail.namespace.len() == 0 { 
                            self.blocks.message.update_tx_response(TxState::Failure,
                                "Set namespace".to_string());
                        }
                        else if self.buy_blob_detail.file_path.len() == 0 { 
                            self.blocks.message.update_tx_response(TxState::Failure,
                                "Select file to upload".to_string());
                        };
                    };
                };
                        
                
           // };

            
        }
    


    fn is_splash(&self)->bool{
        self.view == AppView::SplashNewP1 ||
        self.view == AppView::SplashNewP2 ||
        self.view == AppView::SplashNewP3 ||
        self.view == AppView::SplashExisting ||
        self.view == AppView::SplashImport
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let width=screen_size.x-16.0;
        let height=screen_size.y;
        if !self.is_splash() {
            egui::TopBottomPanel::top("top").show(ctx, |ui| {
                self.blocks.top_panel.ui(ctx,ui,width,height,&mut self.app_state,&mut self.view);
             });
             egui::TopBottomPanel::bottom("buttom").show(ctx, |ui| {
                self.blocks.bottom_panel.ui(ui,width,height,&mut self.app_state,&mut self.view);
             });
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view {
                AppView::SplashExisting=>{
                    self.splash_existing_view(ui,width,height);
                } ,
                AppView::SplashNewP1 | AppView::SplashNewP2 | AppView::SplashNewP3 | AppView::SplashImport=>{
                    self.splash_new_view(ui,width,height);
                } ,
                AppView::Home=>{
                    self.home_view(ui,width,height);
                },
                AppView::SendScreen=>{
                    self.send_view(ui,width,height);
                },
                AppView::BuyBlobScreen=>{
                    self.buy_blob_view(ctx,ui,width,height);
                },
                AppView::AccountListScreen=>{
                    self.account_list_view(ui,width,height);
                },
                AppView::AddAccountScreen =>{
                    self.account_add_view(ui,width,height);
                },
                AppView::TxnList =>{
                    self.txn_list_view(ui,width,height);
                }
                _=>{},
            };
            
            self.blocks.message.ui(ui,width,height);
        });
        

        self.frame_update();
        // self.show_windows(ctx);
    }

    fn splash_existing_view(&mut self,ui: &mut egui::Ui, width: f32, height :f32){
        // let margin_top: f32 = height*0.5;
        ui.vertical(|ui| {
            // ui.add_space(margin_top);
            self.blocks.existing_wallet.ui(ui,width,height,&mut self.password,&mut self.app_state,&mut self.view);
        });
        self.check_splash();
    }

    fn splash_new_view(&mut self,ui: &mut egui::Ui, width: f32, height :f32){
        //let margin_top: f32 = height*0.3;
        ui.vertical(|ui| {
            
            self.blocks.new_wallet.ui(ui,width,height,&mut self.app_state,&mut self.view,
            &mut self.mnemonic_detail,&mut self.password );
        });
        self.check_splash();
    }

     

    fn splash_new_import_view(&mut self,ui: &mut egui::Ui, width: f32, height :f32){
        let margin_top: f32 = height*0.5;
        ui.vertical(|ui| {
            ui.add_space(margin_top);
            // self.blocks.existing_wallet.ui(ui,width,height,&mut self.password,&mut self.app_state);
            
        });
        self.check_splash();
    }
    

    fn home_view(&mut self,ui: &mut egui::Ui, width: f32, height :f32){
        ui.vertical(|ui| {
            //balance
            self.blocks.balance.ui(ui,width,height,&mut self.app_state);
            //buttons 
            
            if self.fee_coin.is_some() {
                self.blocks.home_buttons.ui(ui,width,height,&self.fee_coin.clone().unwrap(),
                &mut self.send_detail, &mut self.app_state,&mut self.view);
            }
        });
        
    }

    fn send_view(&mut self,ui: &mut egui::Ui, width: f32, height :f32){
        ui.vertical(|ui| {
            //balance
            self.blocks.balance.ui(ui,width,height,&mut self.app_state);
            //buttons 
            self.blocks.send.ui(ui,width,height,&mut self.send_detail, &mut self.app_state,&mut self.timer);
            //gas
            self.blocks.gas.ui(ui,width,height,&mut self.tx_gas);
        });

        self.check_send();
        
    }

    fn buy_blob_view(&mut self,ctx: &egui::Context, ui: &mut egui::Ui, width: f32, height :f32){
        ui.vertical(|ui| {
            //balance
            self.blocks.balance.ui(ui,width,height,&mut self.app_state);
            //buttons 
            self.blocks.buy_blob.ui(ctx,ui,width,height,&mut self.buy_blob_detail, &mut self.app_state,&mut self.timer);
            //gas
            self.blocks.gas.ui(ui,width,height,&mut self.tx_gas);
        });

        self.check_buy_blob();
        
    }

    fn account_list_view(&mut self, ui: &mut egui::Ui, width: f32, height :f32){
        ui.vertical(|ui| {
            self.blocks.account_list.ui(ui,width,height,&mut self.app_state, &mut self.view, &mut self.account_select_detail,
            &mut self.delete_account_detail);
        });
        self.check_active_account();
    }

    fn txn_list_view(&mut self, ui: &mut egui::Ui, width: f32, height :f32){
        ui.vertical(|ui| {
            self.blocks.txn_list.ui(ui,width,height,&mut self.app_state, &mut self.view);
        });
    }

    fn account_add_view(&mut self, ui: &mut egui::Ui, width: f32, height :f32){
        ui.vertical(|ui| {
            //balance
            self.blocks.add_account.ui(ui,width,height,&mut self.app_state, &mut self.view, &mut self.add_account_detail);
        });
        
    }
    fn frame_update(&mut self){
        
        self.check_password();
        self.check_add_account();

        match &self.app_state {
            AppState::ShowError(val)=>self.blocks.message.update_tx_response(TxState::Failure,val.clone()),
            _=>{}
        };
        
        self.send_promise.check_tx_result();
        self.send_promise.check_gas_result();
        self.send_promise.consume_tx_result().map(|m| {
            //self.send.update_tx_response(Some(m.to_owned()),None);
            self.blocks.message.update_tx_response(TxState::Success,"Sucecss!".to_owned());
            //get updated after 5 seconds
            std::thread::sleep(tokio::time::Duration::from_millis(5000));
            
            self.balance_promise.init(&self.user,
                //&self.active_account_name.clone().unwrap()
                &self.account_select_detail.name,
            );
            
        });
        self.send_promise.consume_gas_result().map(|m| {
            self.tx_gas=Some(m);
            self.blocks.message.update_tx_response(TxState::None,
                "".to_string());
            //Update gas block
            self.blocks.gas.set_gas(m);
        });
        self.send_promise.consume_failure().map(|_|{
            // self.send.update_tx_response(None,Some("Tx failed".to_string()));
            self.blocks.message.update_tx_response(TxState::Failure,"Tx failed!".to_string());

            std::thread::sleep(tokio::time::Duration::from_millis(5000));
            //self.balance_promise.init(&self.user,self.home.get_active_account_name().clone());
            self.balance_promise.init(&self.user,
                    //&self.active_account_name.clone().unwrap()
                &self.account_select_detail.name,
            );
            
        });



        self.buy_blob_promise.check_tx_result();
        self.buy_blob_promise.check_gas_result();
        self.buy_blob_promise.consume_tx_result().map(|m| {
            //self.blob.update_tx_response(Some(m.to_owned()),None);
            self.blocks.message.update_tx_response(TxState::Success,"Success!".to_owned());
            //get updated after 5 seconds
            std::thread::sleep(tokio::time::Duration::from_millis(5000));
            //self.balance_promise.init(&self.user,self.home.get_active_account_name().clone());
            //if self.active_account_name.is_some() {
                self.balance_promise.init(&self.user,
                    //&self.active_account_name.clone().unwrap()
                    &self.account_select_detail.name,
                );
            //}
        });

        self.txn_list_promise.check_result();
        self.txn_list_promise.consume_result().as_ref().map(|m| {
            //self.home.update_balances_from_app(m);
            self.txns=Some(m.clone());
            self.blocks.txn_list.set_txns(m.clone());
            
            
        });
        self.txn_list_promise.consume_failure().map(|_|{});

        
        self.buy_blob_promise.consume_gas_result().map(|m| {
            self.tx_gas=Some(m);
            self.blocks.message.update_tx_response(TxState::None,
                "".to_string());
            //Update gas block
            self.blocks.gas.set_gas(m);
        });
        self.buy_blob_promise.consume_failure().map(|_|{
           // self.blob.update_tx_response(None,Some("Tx failed".to_string()));
           self.blocks.message.update_tx_response(TxState::Failure,"Tx failed!".to_string());
        });

        
        self.balance_promise.check_result();
        self.balance_promise.consume_result().as_ref().map(|m| {
            //self.home.update_balances_from_app(m);
            self.balances=Some(m.clone());
            let fee_coin=self.balances.clone().as_ref().map(|m| m.first().clone()).unwrap().cloned();
            fee_coin.as_ref().map(|fee_coin| {
                self.blocks.balance.set_fee_coin(fee_coin.symbol.clone(), 
                    helper::get_rounded_value(fee_coin.balance,fee_coin.exponent),
                    None
                );
            });
            if fee_coin.is_none() {
                //Clear
                self.blocks.balance.set_fee_coin(DEFAULT_SYMBOL.to_owned(), 
                    helper::get_rounded_value(0,DEFAULT_EXPONENT),
                    None
                );
            }
            if self.fee_coin.is_none() {
                fee_coin.as_ref().map(|fee_coin| self.blocks.gas.update_fee_coin(fee_coin));
                self.fee_coin=fee_coin;
            }
            //Get txns
            std::thread::sleep(tokio::time::Duration::from_millis(2000));
            self.txn_list_promise.init(&self.user,
                //&self.active_account_name.clone().unwrap()
                &self.account_select_detail.name,
            );
            
        });
        self.balance_promise.consume_failure().map(|_|{});

        self.timer.update();
        if self.timer.consume() == true {
            
            //get gas 
            if self.view == AppView::SendScreen {
                //use receiver detail
                //if self.active_account_name.is_some() {
                    self.send_promise.init_gas(&self.user,
                        //&self.active_account_name.clone().unwrap(),
                        &self.account_select_detail.name,
                        self.send_detail.clone());
                //}
            };
            if self.view == AppView::BuyBlobScreen {
                //use buy blob detail
                // if self.active_account_name.is_some() {
                    self.buy_blob_promise.init_gas(&self.user,
                        //&self.active_account_name.clone().unwrap(),
                        &self.account_select_detail.name,
                        self.buy_blob_detail.clone());
                // }
            };
        }
    }
 
}