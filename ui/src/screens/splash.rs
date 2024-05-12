use cel_wallet::menu::user::{User,UserState};
use crate::components::button::WalletButton;

#[derive(Clone)]
pub struct Splash{
    show_mnemonic: bool,
    pub mnemonic: String,
    pub password: String,
    pub active: bool,
    state: SplashState,
    last_error: Option<String>,
    continue_button: WalletButton,
}

#[derive(PartialEq,Copy,Clone,Debug)]
pub enum SplashState{
    Select,
    NewWallet,
    ExistingWallet,
    PasswordCheck,
}

impl Splash{

    pub fn new(user: &User)->Self{
        let mut state= SplashState::Select;
        match user.get_state() {
            //new or load from mnemoic
            UserState::New =>{
                state=SplashState::NewWallet;
            },
            UserState::Existing =>{
                state=SplashState::ExistingWallet;
            },
            _=>{}
        }
        Splash{
            show_mnemonic: false,
            mnemonic: "".to_string(),
            password: "".to_string(),
            active: true,
            state,
            last_error: None,
            continue_button: WalletButton::new(0.05,egui::Align::Center, 20.0,
                "Continue".to_string(), 0.3,0.05,true,true),
        }
    }

    pub fn get_state(&self)->SplashState{
        self.state
    }
 
    //pub fn ui(&mut self,ui: &mut egui::Ui){
    pub fn ui(&mut self,ctx: & egui::Context,user: &User){
        // if crate::app::is_mobile(ctx) {
        //     self.desktop_ui(ctx);
        // } else {
        //     self.desktop_ui(ctx);
        // }
        match self.state {
            SplashState::Select=>{self.select_ui(ctx);},
            SplashState::NewWallet=>{self.new_wallet_ui(ctx)},
            SplashState::ExistingWallet=>{self.existing_wallet_ui(ctx,user)},
            SplashState::PasswordCheck=>{self.existing_wallet_ui(ctx,user)},
        }
    }

    pub fn update_password_result(&mut self,error: Option<String>){
        self.last_error=error.clone();
        if self.last_error.is_none(){
            self.last_error=None;
            self.active=false;
        }else{
            self.state=SplashState::ExistingWallet;
        };
    }

    fn select_ui(&mut self, ctx: &egui::Context){
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let width=screen_size.x;
        let height=screen_size.y;
        //60pct down
        //50pct across
        let margin_top=height*60.0/100.0;
        let margin_left=width*50.0/100.0;
        let button_width=width * 30.0/100.0;
        let button_height=height * 5.0/100.0;
        egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(margin_top);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                    //ui.label("Hello World!");
                    if ui.add_sized([button_width,button_height],egui::Button::new("New Wallet")).clicked() {
                        println!("New wallet");
                        self.state=SplashState::NewWallet;
                    };
                    
                });
            // ui.horizontal(|ui| {
            //     ui.add_space(margin_left);
                
            // });
            let margin_top=height*5.0/100.0;
            ui.add_space(margin_top);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                //ui.add_space(margin_left);
                if ui.add_sized([button_width,button_height],egui::Button::new("Existing Wallet")).clicked() {
                    //println!("Existing wallet");
                    //self.show_mnemonic=!self.show_mnemonic;
                    self.state=SplashState::ExistingWallet;
                };
            });
            
        });
        });
        
    }

    fn new_wallet_ui(&mut self, ctx: &egui::Context){
        let screen_size = ctx.input(|i| i.screen_rect.size());
    }

    fn existing_wallet_ui(&mut self, ctx: &egui::Context, user: &User){
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let width=screen_size.x;
        let height=screen_size.y;
        //60pct down
        //50pct across
        let margin_top=height*60.0/100.0;
        let margin_left=width*50.0/100.0;
        let button_width=width * 30.0/100.0;
        let button_height=height * 5.0/100.0;
        egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            
            ui.add_space(margin_top);
            let password_text=egui::TextEdit::singleline(&mut self.password)
                .password(true)
                .desired_width(button_width)
                .vertical_align(egui::Align::Center);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                    //ui.label("Hello World!");
                    ui.add(password_text);
                    
                });
            
            self.continue_button.ui(ui,width,height);
            if self.continue_button.get_is_clicked() {
                self.state=SplashState::PasswordCheck;
            }
        
            if let Some(error)=&self.last_error {
                let margin_top=height*5.0/100.0;
                ui.add_space(margin_top);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                    ui.label(format!("Error: {}",error));      
                });
            };
        });
        });
    }

 
}