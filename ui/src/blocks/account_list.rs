use crate::app::{AppState,AppView};
use crate::components::text::AddressText;
use crate::state::{AccountData,DeleteAccountDetail};
use cel_wallet::EXTENSION;
use crate::helper;

pub struct AccountListBlock{
    accounts: Vec<AddressText>,
}

impl AccountListBlock{
    pub fn new()->Self{
        AccountListBlock{
            accounts: vec![],
        }
    }
 

    pub fn add_accounts(&mut self, val: Vec<AccountData>){
        
        val.iter().for_each(|val| {
            let mut account=AddressText::new(
                //vec![val.name.clone(),val.address.clone()], 
                vec![super::ACCOUNT_LIST_NAME_FONTSIZE,
                super::ACCOUNT_LIST_ADDRESS_FONTSIZE],
                vec![super::ACCOUNT_LIST_COLOR,super::ACCOUNT_LIST_COLOR],
                None,
                //None,egui::Align::Min,false,true,
            );
            account.set_value(val.name.clone(),val.address.clone(),val.title.clone(),false);

            account.set_size(Some(super::ACCOUNT_LIST_ADDRESS_WIDTH),Some(super::ACCOUNT_LIST_ADDRESS_HEIGHT));
            self.accounts.push(account);
        });
    }

    pub fn remove_account(&mut self, name: &String){
        if let Some(index) = self.accounts.iter().position(|value| &value.name==name) {
            self.accounts.remove(index);
        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui,width: f32, height: f32,
         state: &mut AppState, view: &mut AppView,account_data: &mut AccountData, account_delete: &mut DeleteAccountDetail){
        // ui.vertical(|ui| {
            egui::ScrollArea::vertical()
            .max_height(f32::INFINITY)
            .show(ui, |ui| {
            egui::Grid::new("account_list_grid")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                for account in self.accounts.iter_mut() {
                    
                    account.ui(ui,width,height);
                    let values=account.get_value();
                        
                    if account.get_is_clicked(){
                        account_data.update(values.0.clone().replace(EXTENSION,""),values.1,values.2);
                        *view=AppView::Home;
                        *state=AppState::ActiveAccountUpdate;
                    }
                    //Do not delete root account
                    if values.0 != format!("account-0{}",EXTENSION) {
                        let delete = egui::Button::new(egui::RichText::new("❎".to_string())
                            .font(helper::get_font_id(super::CHAR_BUTTON_FONTSIZE)));
                        if ui.add(delete).clicked() {
                            account_delete.name=Some(values.0);
                            *state=AppState::AccountDeleteClick;
                        }
                    }
                        
                    ui.end_row();
                    // ui.separator();
                    // ui.separator();
                    // ui.end_row();
                }
                 
            });
        });
        // })
    }
}