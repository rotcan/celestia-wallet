use crate::app::{AppState,AppView};
use crate::state::{AccountMnemonicDetail,PASSWORD_ERROR};
use crate::components::button::WalletButton;
use crate::helper;
pub struct NewWalletBlock{
    next:  WalletButton,
    copy: WalletButton ,
    create: WalletButton ,
    import: WalletButton ,
    back: WalletButton ,
}

impl NewWalletBlock{
    pub fn new()->Self{
        NewWalletBlock{
            next: super::get_splash_button("Next"),
            copy: super::get_splash_button("Copy"),
            create: super::get_splash_button("Create New"),
            import: super::get_splash_button("Import Existing"),
            back:  super::get_splash_button("Back"),
        }
    }

   

    pub fn ui(&mut self, ui: &mut egui::Ui,width: f32, height: f32,
         state: &mut AppState,view: &mut AppView,mnemonic_detail: &mut AccountMnemonicDetail,
         password: &mut String){
            let text_height=super::ADD_ACCOUNT_WIDGET_HEIGHT * height;
        let text_width=super::MNEMONIC_TEXT_WIDTH * width;
        let vertical_margin=super::MNEMONIC_TEXT_VERTICAL_MARGIN * height;
        ui.add_space(0.2*height);
        let password_width=width * 30.0/100.0;
        if view == &AppView::SplashNewP1{
            
            ui.add_space(0.2*height);
            self.create.ui(ui,width,height);
            if self.create.get_is_clicked() {
                *view=AppView::SplashNewP2;
            };

            ui.add_space(0.025*height);
            self.import.ui(ui,width,height);
            if self.import.get_is_clicked() {
                mnemonic_detail.clear();            
                *view=AppView::SplashImport;
            }

            ui.add_space(0.025*height);
            self.back.ui(ui,width,height);
            if self.back.get_is_clicked() {
                mnemonic_detail.clear();            
                *view=AppView::SplashExisting;
            }
        }
        else if view == &AppView::SplashNewP2{
                //Part 1
                //Show 12 words mnemonic
            ui.label(helper::get_label("Store the mnemonic carefully",12.0,false));
            ui.add_space(0.025*height);
            //if let Some(mut mnemonic) = mnemonic_detail.mnemonic{ 
            egui::Grid::new("account_list_grid")
                    .num_columns(3)
                    .show(ui, |ui| {
                        ui.vertical(|ui|{
                    for i in 0..4{
                        ui.horizontal(|ui|{
                        for j in 0..3{
                            
                                let title=egui::TextEdit::singleline(&mut mnemonic_detail.mnemonic[i*3+j])
                                .desired_width(text_width)
                                .min_size(egui::Vec2::new(text_width,text_height))
                                .vertical_align(egui::Align::Min);
                                // ui.add_space(10.0);
                                ui.add(title);
                                
                                

                        }
                    });
                        ui.end_row();
                        ui.add_space(vertical_margin);
                }
                });
            });

            //Copy
            self.copy.ui(ui,width,height);
            if self.copy.get_is_clicked() {
                let phrase: String= mnemonic_detail.mnemonic.join(" ");
                ui.output_mut(|o| o.copied_text = phrase );
            }
            ui.add_space(0.025*height);
            //Next
            self.next.ui(ui,width,height);
            if self.next.get_is_clicked() {
                *state=AppState::VerifyMnemonic;
            }

            ui.add_space(0.025*height);
            self.back.ui(ui,width,height);
            if self.back.get_is_clicked() {
                mnemonic_detail.clear();            
                *state=AppState::SplashReset;
            }

        }
        else if view == &AppView::SplashNewP3{
            //Part 2
            //Verify mnemonic
            ui.label(helper::get_label("Verify the mnemonic ",12.0,false));
            ui.add_space(0.025*height);
            //if let Some(mut mnemonic) = mnemonic_detail.mnemonic{ 
            egui::Grid::new("account_list_grid")
                    .num_columns(3)
                    .show(ui, |ui| {
                        ui.vertical(|ui|{
                    for i in 0..4{
                        ui.horizontal(|ui|{
                        for j in 0..3{
                                let counter=i*3+j;
                                let title=egui::TextEdit::singleline(&mut mnemonic_detail.verify[counter])
                                .desired_width(text_width)
                                .min_size(egui::Vec2::new(text_width,text_height))
                                .vertical_align(egui::Align::Min);
                                // ui.add_space(10.0);
                                ui.add(title);
                                

                        }
                    });
                        ui.end_row();
                        ui.add_space(vertical_margin);
                }
                });
            });

            ui.vertical_centered(|ui| {
                ui.label(helper::get_label("Password",14.0,false));
            });
            
            //Password
            let password_text=egui::TextEdit::singleline(password)
                .password(true)
                .desired_width(password_width)
                .vertical_align(egui::Align::Center);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                //ui.label("Hello World!");
                ui.add(password_text);
                
            });
            ui.add_space(0.025*height);
            //Next
            self.next.ui(ui,width,height);
         
            if self.next.get_is_clicked() {
                if helper::password_validation(password) == true {
                    *state=AppState::MatchMnemonic;
                }else{
                    *state=AppState::ShowError(PASSWORD_ERROR.to_owned());
                };
            }

            
        }else if view == &AppView::SplashImport{  
            ui.label(helper::get_label("Import existing mnemonic",12.0,false));
            ui.add_space(0.025*height);
            //if let Some(mut mnemonic) = mnemonic_detail.mnemonic{ 
            egui::Grid::new("account_list_grid")
                    .num_columns(3)
                    .show(ui, |ui| {
                        ui.vertical(|ui|{
                    for i in 0..4{
                        ui.horizontal(|ui|{
                        for j in 0..3{
                                let counter= i*3+j;
                                let title=egui::TextEdit::singleline(&mut mnemonic_detail.mnemonic[counter])
                                .desired_width(text_width)
                                .min_size(egui::Vec2::new(text_width,text_height))
                                .vertical_align(egui::Align::Min);
                                // ui.add_space(10.0);
                                let response=ui.add(title);
                                if counter ==0  && response.changed()  {
                                    let values=AccountMnemonicDetail::break_phrase_into_mnemonic( &mnemonic_detail.mnemonic[counter]);
                                    if values.len() == 12 {
                                        for c in 0..12 {
                                            mnemonic_detail.mnemonic[c]=values[c].clone();
                                        }
                                    }
                                };
                                
                                

                        }
                    });
                        ui.end_row();
                        ui.add_space(vertical_margin);
                }
                });
            });


            ui.vertical_centered(|ui| {
                ui.label(helper::get_label("Password",14.0,false));
            });
            //Password
            let password_text=egui::TextEdit::singleline(password)
                .password(true)
                .desired_width(password_width)
                .vertical_align(egui::Align::Center);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center),|ui|{
                //ui.label("Hello World!");
                ui.add(password_text);
                
            });
            ui.add_space(0.025*height);
            //Next
            self.next.ui(ui,width,height);
            
            if self.next.get_is_clicked() {
                if helper::password_validation(password) == true  {
                    *state=AppState::ImportMnemonic;
                } else if helper::password_validation(password) == false {
                    *state=AppState::ShowError(PASSWORD_ERROR.to_owned());
                };
            }

            ui.add_space(0.025*height);
            self.back.ui(ui,width,height);
            if self.back.get_is_clicked() {
                *state=AppState::SplashReset;
            }

        } 
    }
}
