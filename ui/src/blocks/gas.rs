use crate::components::text::{WalletText,DefaultHyperlinkFunction};
use crate::helper;
use cel_wallet::utils::CoinMetadata;

const GAS_LABEL_FONT_SIZE: f32=12.0;
const GAS_TEXT_FONT_SIZE: f32=12.0;
const GAS_LABEL_COLOR: egui::Color32 = egui::Color32::WHITE;
const GAS_TEXT_COLOR: egui::Color32 = egui::Color32::WHITE;
const GAS_FRAME_COLOR: egui::Color32 = egui::Color32::TRANSPARENT;
const FRAME_SELECTED_COLOR: egui::Color32 = egui::Color32::DARK_GRAY;

struct GasText{
    pub frame_color: egui::Color32,
    pub text: WalletText<DefaultHyperlinkFunction>,
    pub value: u64,
}

impl GasText{

    pub fn new()->Self{
        GasText{
            text:  WalletText::new(
                vec![], vec![GAS_LABEL_FONT_SIZE,GAS_TEXT_FONT_SIZE],
            vec![GAS_LABEL_COLOR,GAS_TEXT_COLOR],
            None,egui::Align::Min,false,true,
            None,
            move |s| format!("{}",s)
            ),
            frame_color: GAS_FRAME_COLOR,
            value: 0
        }
    }
 
    pub fn ui(&mut self, ui: &mut egui::Ui,width: f32, height: f32){
        let f_width=width*0.2;
        let f_height=height*0.2;
        ui.allocate_ui_with_layout(egui::Vec2::new(f_width,f_height),egui::Layout::default(),
            |ui|{  
                egui::Frame::default().fill(self.frame_color).show(ui,|ui| {
                self.text.ui(ui,width,height);
            });
        });
        
    }

    pub fn set_texts(&mut self,val: Vec<String>){
        self.text.set_texts(val);
    }
 

    pub fn get_is_clicked(&self)->bool{
        self.text.get_is_clicked()
    }

    pub fn set_color(&mut self,color: egui::Color32){
        self.frame_color=color;
    }


}


pub struct GasBlock{
    gas: Option<cel_wallet::CosmosGas>,
    //
    label_font: egui::FontId,
    low: GasText,
    mid: GasText,
    high: GasText,
    exponent: u32,
    pub is_hidden: bool,
}

impl GasBlock{

    pub fn new()->Self{
        GasBlock{
            gas : None,
            label_font: egui::FontId::monospace(GAS_LABEL_FONT_SIZE),
            low :GasText::new(),
            mid: GasText::new(),
            high: GasText::new(),
            exponent: 0,
            is_hidden:true,
        }
    }

    pub fn update_fee_coin(&mut self, coin: &CoinMetadata){
        self.exponent=coin.exponent;
    }

    pub fn clear_gas(&mut self){
        self.gas=None;
    }

    pub fn set_gas(&mut self,gas: cel_wallet::CosmosGas){
        self.gas=Some(gas);
        self.low.value=gas;
        let low_value=helper::get_rounded_value(gas.into(),self.exponent);

        self.mid.value=helper::add_pct_to_u64(gas,0.15);
        let mid_value=helper::get_rounded_value(self.mid.value.into(),self.exponent);

        self.high.value=helper::add_pct_to_u64(gas,0.3);
        let high_value=helper::get_rounded_value(self.high.value.into(),self.exponent);
        
        self.low.set_texts(vec!["Low".to_string(),format!("{}",low_value)]);
        self.mid.set_texts(vec!["Mid".to_string(),format!("{}",mid_value)]);
        self.high.set_texts(vec!["High".to_string(),format!("{}",high_value)]);

        self.clear_selected();
        self.low.set_color(FRAME_SELECTED_COLOR);
        
    }

    pub fn get_gas(&self)->Option<cel_wallet::CosmosGas>{
        self.gas
    }

    pub fn clear_selected(&mut self){
        self.low.frame_color=GAS_FRAME_COLOR;
        self.mid.frame_color=GAS_FRAME_COLOR;
        self.high.frame_color=GAS_FRAME_COLOR;
    }
 
    pub fn hide(&mut self){
        self.gas=None;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, width: f32, height: f32,gas_value: &mut Option<cel_wallet::CosmosGas>){

        if self.gas.is_some() { 
            let margin_height=0.025*height;
            // let margin_width=0.02*width;
            ui.add_space(margin_height);
            // ui.horizontal(|ui| {
            egui::Grid::new("gas_grid")
            .num_columns(4)
            .spacing([10.0, 2.0])
            // .striped(true)
            .show(ui, |ui| {
                //label
                let label=egui::Label::new( egui::widget_text::RichText::new("Gas")
                .font(self.label_font.clone()));
                //.color(self.text_color.get(i).unwrap().clone())).sense(egui::Sense::click());
                ui.add(label);
                //ui.add_space(margin_width);
                //
                self.low.ui(ui,width,height);
                self.mid.ui(ui,width,height);
                self.high.ui(ui,width,height);
                ui.end_row();
            });
            // });
            ui.add_space(margin_height);
            self.frame_update(gas_value);
        }
    }

    fn frame_update(&mut self,gas_value: &mut Option<cel_wallet::CosmosGas>){
        if self.low.get_is_clicked() {
            self.clear_selected();
            self.low.set_color(FRAME_SELECTED_COLOR);
            *gas_value=Some(self.low.value);
        }
        if self.mid.get_is_clicked() {
            self.clear_selected();
            self.mid.set_color(FRAME_SELECTED_COLOR);
            *gas_value=Some(self.mid.value);
        }
        if self.high.get_is_clicked() {
            self.clear_selected();
            self.high.set_color(FRAME_SELECTED_COLOR);
            *gas_value=Some(self.high.value);
        }
    }
}