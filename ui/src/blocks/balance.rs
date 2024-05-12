use crate::components::text::CoinText;
use crate::app::AppState;

pub struct BalanceBlock{
    text: CoinText
}

impl BalanceBlock{
    pub fn new()->Self{
        BalanceBlock{
            text: CoinText::new(vec![18.0,9.0],
                vec![egui::Color32::WHITE,egui::Color32::WHITE],Some(0.2)),
        }
    }

    
    pub fn set_fee_coin(&mut self,symbol: String, value: String, usd: String ){
        self.text.set_value(symbol,value,usd);
        self.text.set_back_color(egui::Color32::DARK_GRAY);
    }


    pub fn ui(&mut self, ui: &mut egui::Ui, width: f32, height: f32, _state: &mut AppState){

        ui.allocate_ui_with_layout(egui::Vec2::new(width,height*0.3),egui::Layout::default(),
            |ui|{  egui::Frame::default().fill(egui::Color32::DARK_GRAY).show(ui,|ui| 
            {
                self.text.ui(ui,width,height);
                
            })
        }
        );
    }
}