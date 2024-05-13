use crate::state::SendDetail;
use crate::app::AppState;
use crate::components::{timer::Timer,button::WalletButton};
use crate::helper;


pub struct SendBlock{
    send_button: WalletButton
}

impl SendBlock{

    pub fn new()->Self{
        SendBlock{
            send_button: WalletButton::new(0.0,egui::Align::Max, 14.0,
                "Send".to_string(), super::BUTTON_WIDTH, super::BUTTON_HEIGHT,false,true),
        }
    }

    pub fn ui(&mut self,ui: &mut egui::Ui, width: f32, height: f32,
        receiver_detail: &mut SendDetail, state : &mut AppState, timer: &mut Timer){
        let margin_top=height * 0.02;
        let address_width=0.6*width;
        let text_height=super::BUTTON_HEIGHT*height;
        let amount_width=address_width;
        
        ui.heading("Send Coins");
        ui.add_space(margin_top);
        egui::Grid::new("send_grid")
        .num_columns(3)
        .spacing([10.0, 2.0])
        // .striped(true)
        .show(ui, |ui| {
         
                //address
            
            ui.label("Receiver");
            let receiver_text=egui::TextEdit::singleline(&mut receiver_detail.to)
                .desired_width(address_width)
                .min_size(egui::Vec2::new(address_width,text_height))
                .vertical_align(egui::Align::Center);

            // ui.add_space(10.0);
            ui.add(receiver_text);
            ui.end_row();
          
             
            //amount
           
            ui.label("Amount");
            let receiver_text=egui::TextEdit::singleline(&mut receiver_detail.amount)
                .desired_width(amount_width)
                .min_size(egui::Vec2::new(amount_width,text_height))
                .vertical_align(egui::Align::Center);
                //ui.add_space(10.0);
            let response=ui.add(receiver_text);
            if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || response.changed() {
                if receiver_detail.amount.len() > 0 {
                    let val=helper::convert_amount(&receiver_detail.amount,receiver_detail.exponent).unwrap_or(0);
                    if val>0 {
                        //check gas
                        timer.start();
                    }
                };
            };

            self.send_button.ui(ui,width,height);
            if self.send_button.get_is_clicked() {
                //self.labels.tx_response.set_state(crate::components::tx_response::TxState::Pending, "Pending".to_string());
                *state=AppState::SendClick;
            };
            ui.end_row();
           
        });
     
    

    }
}