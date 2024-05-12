use crate::app::{AppState,AppView};
use crate::helper;
pub struct BottomPanelBlock{
    
}

impl BottomPanelBlock{
    pub fn new()->Self{
        BottomPanelBlock{
            
        }
    }

    pub fn ui(&mut self,ui: &mut egui::Ui, width: f32, height: f32, state : &mut AppState, view: &mut AppView){
        //User details
        let col_width=width*0.24;
        let col_height=height*super::PANEL_HEIGHT;
        egui::Grid::new("bottom_panel_grid")
        //.num_columns(4)
        //.min_col_width(col_width)
        //.spacing([10.0, 5.0])
        // .striped(true)
        .show(ui, |ui| {
            let user = egui::Button::new(egui::RichText::new("🐶".to_string()).font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
            if ui.add_sized([col_width,col_height],user).clicked() {
                *view= AppView::Home;
            };
            //Add Account
            let add_account = egui::Button::new(egui::RichText::new("➕".to_string()).font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
            if ui.add_sized([col_width,col_height],add_account).clicked() {
                *view= AppView::AddAccountScreen;
                *state= AppState::AddAccountPubkey;
            };
            
            //Settings
            let settings = egui::Button::new(egui::RichText::new("⚙".to_string()).font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
            if ui.add_sized([col_width,col_height],settings).clicked() {
                *view= AppView::Home;
            };
            
            //List
            let list = egui::Button::new(egui::RichText::new("📖".to_string()).font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
            if ui.add_sized([col_width,col_height],list).clicked() {
                *view= AppView::TxnList;
            };
            ui.end_row();
        });
    }
}