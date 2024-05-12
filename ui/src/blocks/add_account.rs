use crate::helper;
use crate::state::AddAccountDetail;
use crate::app::{AppState,AppView};
use crate::components::button::WalletButton;

pub struct AddAccountBlock{
    add_account:  WalletButton,
    import_private_key:  WalletButton,
}

impl AddAccountBlock{

    pub fn new()->Self{
        AddAccountBlock{
            add_account: super::get_action_button("Add"),
            import_private_key:super::get_action_button("Import"),
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui, width : f32, height: f32,
        state: &mut AppState, view: &mut AppView, add_account_detail: &mut AddAccountDetail){
        let text_height=super::ADD_ACCOUNT_WIDGET_HEIGHT * height;
        let text_width=super::ADD_ACCOUNT_TEXT_WIDTH * width;
        //heading
        ui.heading(helper::get_label("New Account",18.0,false));
        //title
        ui.label(helper::get_label("Name",12.0,false));
        
        if add_account_detail.title.is_some() {
            let title=egui::TextEdit::singleline(add_account_detail.title.as_mut().unwrap())
                .desired_width(text_width)
                .min_size(egui::Vec2::new(text_width,text_height))
                .interactive(false)
                .vertical_align(egui::Align::Min);
            // ui.add_space(10.0);
            ui.add(title);
            
        };
        
        //public key (Read only)
        ui.label(helper::get_label("Address",12.0,false));
        add_account_detail.address.as_ref().map(|address| ui.label(helper::get_label(address,12.0,false)));
        
        //Add 
        self.add_account.ui(ui,width,height);
        if self.add_account.get_is_clicked() {
            *state=AppState::AddAccountClick;
        }
        //Import from private key
        ui.label(helper::get_label("Import using Private Key",12.0, false));
        //Private key
        let private_key_hex_input=egui::TextEdit::singleline(&mut add_account_detail.private_key_hex)
            .desired_width(text_width)
            .min_size(egui::Vec2::new(text_width,text_height))
            .vertical_align(egui::Align::Min);
        // ui.add_space(10.0);
        ui.add(private_key_hex_input);
        //button
        self.import_private_key.ui(ui,width,height);
        if self.import_private_key.get_is_clicked() {
            *state = AppState::ImportPrivateKeyClick;
        }

    }
}