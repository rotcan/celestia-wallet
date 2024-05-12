use crate::components::button::WalletButton;
use crate::app::{AppState,AppView};
use crate::state::SendDetail;
use cel_wallet::utils::CoinMetadata;
pub struct HomeButtonsBlock{
    pub send_button: WalletButton,
    pub buy_blob_button: WalletButton,
    visible: bool,
}


impl HomeButtonsBlock{
    pub fn new()->Self{
        HomeButtonsBlock{
            send_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Send".to_string(), 0.3, 0.1,false,true),
            buy_blob_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Buy Blob".to_string(), 0.3, 0.1,false,true),
                visible: true,
        }
    }

    pub fn set_visible(&mut self, val: bool){
        self.visible=val;
    }

    pub fn get_visible(&self)->bool{
        self.visible
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, width: f32, height: f32,transfer_coin: &CoinMetadata,
        send_detail: &mut SendDetail, _state: &mut AppState, view: &mut AppView){
        if self.visible == true {
            ui.horizontal(|ui|{
                ui.add_space(width*0.18);
                self.send_button.ui(ui,width,height);
                if self.send_button.get_is_clicked() {
                    send_detail.set_value(transfer_coin);
                    //Set main coin for now
                    *view=AppView::SendScreen;
                    
                }
                self.buy_blob_button.ui(ui,width,height);
                if self.buy_blob_button.get_is_clicked() {
                    *view=AppView::BuyBlobScreen;
                }
            });
        };
    }

}