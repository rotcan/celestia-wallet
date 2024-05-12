use crate::components::button::WalletButton;
pub mod send;
pub mod gas;
pub mod new_wallet;
pub mod existing_wallet;
pub mod message;
pub mod balance;
pub mod bottom_panel;
pub mod top_panel;
pub mod home_buttons;
pub mod buy_blob;
pub mod account_list;
pub mod add_account;
pub mod txn_list;

pub const BUTTON_WIDTH: f32=0.2;
pub const BUTTON_HEIGHT : f32=0.05;
pub const CHAR_BUTTON_FONTSIZE: f32=18.0;
pub const TOP_PANEL_MARGIN_ADDRESS: f32=0.3;
pub const TOP_PANEL_MARGIN_BACK: f32=0.3;
pub const PANEL_HEIGHT: f32=0.075;

pub const ACCOUNT_LIST_NAME_FONTSIZE: f32=12.0;
pub const ACCOUNT_LIST_ADDRESS_FONTSIZE: f32=10.0;
pub const ACCOUNT_LIST_COLOR: egui::Color32 = egui::Color32::WHITE;
pub const ACCOUNT_LIST_ADDRESS_WIDTH: f32=0.90;
pub const ACCOUNT_LIST_ADDRESS_HEIGHT: f32=0.03;

pub const ADD_ACCOUNT_WIDGET_HEIGHT: f32=0.05;
pub const ADD_ACCOUNT_TEXT_WIDTH: f32=0.95;

pub const MNEMONIC_TEXT_WIDTH: f32=0.3;
pub const MNEMONIC_TEXT_VERTICAL_MARGIN: f32=0.025;

pub fn get_action_button(title: &str)->WalletButton{
    WalletButton::new(0.0,egui::Align::Max, 14.0,
        title.to_owned(),  BUTTON_WIDTH, BUTTON_HEIGHT,true,true)
}

pub const SPLASH_BUTTON_WIDTH: f32=0.4;
pub fn get_splash_button(title: &str)->WalletButton{
    WalletButton::new(0.0,egui::Align::Center, 14.0,
        title.to_owned(),  SPLASH_BUTTON_WIDTH, BUTTON_HEIGHT,true,true)
}
// pub fn doc_link_label_with_crate<'a>(
//     crate_name: &'a str,
//     title: &'a str,
//     search_term: &'a str,
// ) -> impl egui::Widget + 'a {
//     let label = format!("{title}:");
//     let url = format!("https://docs.rs/{crate_name}?search={search_term}");
//     move |ui: &mut egui::Ui| {
//         ui.hyperlink_to(label, url).on_hover_ui(|ui| {
//             ui.horizontal_wrapped(|ui| {
//                 ui.label("Search egui docs for");
//                 ui.code(search_term);
//             });
//         })
//     }
// }