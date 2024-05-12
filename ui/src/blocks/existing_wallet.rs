use crate::components::button::WalletButton;
use crate::app::{AppState,AppView};
pub struct ExistingWalletBlock{
    continue_button: WalletButton,
    reset_button: WalletButton,
    visible: bool,
}
use crate::helper;

impl ExistingWalletBlock{
    pub fn new()->Self{
        ExistingWalletBlock{
            continue_button :WalletButton::new(0.0,egui::Align::Center, 14.0,
                "Continue".to_string(), 0.3,0.05,true,true),
                visible: false,
            reset_button: WalletButton::new(0.0,egui::Align::Center, 14.0,
                "Reset".to_string(), 0.3,0.05,true,true),
        }
    }

}
impl  ExistingWalletBlock{

    pub fn set_visible(&mut self, val: bool){
        self.visible=val;
    }

    pub fn get_visible(&self)->bool{
        self.visible
    }
   
    pub fn ui(&mut self, ui: &mut egui::Ui,width: f32, height: f32,
     password: &mut String, state: &mut AppState, view: &mut AppView){
        // let screen_size = ctx.input(|i| i.screen_rect.size());
        // let width=screen_size.x;
        // let height=screen_size.y;
        //60pct down
        //50pct across
        // let margin_top=height*60.0/100.0;
        // let margin_left=width*50.0/100.0;
        if self.visible == true {
            let button_width=width * 30.0/100.0;
            let button_height=height * 5.0/100.0;
            ui.add_space(0.4*height);
            //egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                
                ui.vertical_centered(|ui| {
                    ui.label(helper::get_label("Password",14.0,false));
                });
                //ui.add_space(margin_top);
                let password_text=egui::TextEdit::singleline(password)
                    .password(true)
                    .desired_width(button_width)
                    .vertical_align(egui::Align::Center);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                        //ui.label("Hello World!");
                        ui.add(password_text);
                        
                    });
                
                ui.add_space(0.05*height);
                self.continue_button.ui(ui,width,height);
                if self.continue_button.get_is_clicked() {
                    *state=AppState::SplashPasswordCheck;
                }
                ui.add_space(0.025*height);
                self.reset_button.ui(ui,width,height);
                if self.reset_button.get_is_clicked() {
                    *state= AppState::SplashReset;
                }
            
                // if let Some(error)=&self.last_error {
                //     let margin_top=height*5.0/100.0;
                //     ui.add_space(margin_top);
                //     ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                //         ui.label(format!("Error: {}",error));      
                //     });
                // };
            });
            //});
        };
        
    }
}