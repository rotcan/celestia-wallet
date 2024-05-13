use crate::app::{AppState,AppView};
use crate::components::text::AddressText;
use crate::helper;

pub struct TopPanelBlock{
    address: AddressText,
}

impl TopPanelBlock{
    pub fn new()->TopPanelBlock{
        TopPanelBlock{
            address: AddressText::new(vec![12.0,6.0],
                vec![egui::Color32::WHITE,egui::Color32::WHITE],None),
        }
    }

    pub fn set_address_value(&mut self,name: String, address : String, title: String){
        self.address.set_value(name, address,title, true);
        
    }

    pub fn ui(&mut self,ctx: &egui::Context, ui: &mut egui::Ui, width: f32, height: f32, _state : &mut AppState,
    view: &mut AppView){
        ui.horizontal(|ui|{
            
            let account_list = egui::Button::new(egui::RichText::new("☰".to_string())
            .font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
            if ui.add(account_list).clicked() {
                *view= AppView::AccountListScreen;
            };

            ui.add_space(super::TOP_PANEL_MARGIN_ADDRESS*width);
            self.address.ui(ui,width,height);
            if self.address.get_is_clicked(){
                ctx.output_mut(|o| o.copied_text = self.address.address.clone());
                //println!("copied address");
            };
            if view != &AppView::Home {
                ui.add_space(super::TOP_PANEL_MARGIN_BACK*width);
                let back = egui::Button::new(egui::RichText::new("⮈".to_string()).font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
                if ui.add(back).clicked() {
                    *view= AppView::Home;
                }
            }
        });
    }
}