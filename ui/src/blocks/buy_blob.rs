use crate::state::BuyBlobDetail;
use crate::app::AppState;
use crate::components::button::WalletButton;
use crate::components::timer::Timer;


pub struct BuyBlobBlock{
    buy_blob_button: WalletButton,
    file_dialog: WalletButton,
    opened_file: Option<std::path::PathBuf>,
    open_file_dialog: Option<egui_file::FileDialog>,
}

impl BuyBlobBlock{

    pub fn new()->BuyBlobBlock{
        BuyBlobBlock{
            buy_blob_button:  super::get_action_button("Buy"),
            file_dialog: super::get_action_button("..."),
            opened_file: None,
            open_file_dialog : None,
        }
    }

    pub fn ui(&mut self,ctx: &egui::Context,ui: &mut egui::Ui, width: f32, height: f32,
         buy_blob_detail: &mut BuyBlobDetail,
         state : &mut AppState, timer: &mut Timer){
            let margin_top=height * 0.02;
            let namespace_width=0.5*width;
            let text_height=super::BUTTON_HEIGHT*height;
            let file_path_width=namespace_width;

            ui.add_space(margin_top);
                    
            egui::Grid::new("buy_blob_grid")
            .num_columns(3)
            .spacing([10.0, 2.0])
            // .striped(true)
            .show(ui, |ui| {
                 //

                //  ui.vertical(|ui| {
                     //name space
                    // ui.horizontal(|ui| {
                    ui.label("Namespace");
                    let namespace_text=egui::TextEdit::singleline(&mut buy_blob_detail.namespace)
                        //.desired_width(namespace_width)
                        .min_size(egui::Vec2::new(namespace_width,text_height))
                        .vertical_align(egui::Align::Center);
                    // ui.add_space(10.0);
                    let response= ui.add(namespace_text);
                //   println!("self.receiver_detail.to={}",self.receiver_detail.to);
                    if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || response.changed() {
                        if buy_blob_detail.namespace.len()>0 && buy_blob_detail.file_path.len()>0 {
                            timer.start();
                        };
                    };
                    ui.end_row();
                    // });
                    //file
                    // ui.horizontal(|ui|{
                    ui.label("File");
                    let file_path_text=egui::TextEdit::singleline(&mut buy_blob_detail.file_path)
                        .desired_width(file_path_width)
                        .min_size(egui::Vec2::new(file_path_width,text_height))
                        .vertical_align(egui::Align::Min);
                    // ui.add_space(10.0);
                    ui.add(file_path_text);

                    self.file_dialog.ui(ui,width,height);
                    if self.file_dialog.get_is_clicked() && self.open_file_dialog.is_none() {
                        let mut dialog = egui_file::FileDialog::open_file(self.opened_file.clone())
                        .default_size(egui::Vec2::new(width,height-64.0));
                        dialog.open();
                        self.open_file_dialog = Some(dialog);
                    }
                    // });
                    ui.end_row();
                    
                    
                // });
            });
            
            ui.horizontal(|ui|{
                ui.add_space(10.0);
                self.buy_blob_button.ui(ui,width,height);
                if self.buy_blob_button.get_is_clicked() {
                    *state=AppState::BuyBlobClick;
                }
               
            });
        
        self.ctx_update(ctx,buy_blob_detail, timer, width,height);
        

    }

    fn ctx_update(&mut self,ctx: &egui::Context,buy_blob_detail: &mut BuyBlobDetail,timer: &mut Timer,  width: f32, _height: f32){
        if let Some(dialog) = &mut self.open_file_dialog {
            match dialog.show(ctx).state() {
                egui_file::State::Selected => {
                    if let Some(file) = dialog.path() {
                        self.opened_file = Some(file.to_path_buf());
                        // println!("opened_file={:?}",self.opened_file);
                        self.opened_file.clone().map(|m| {
                                buy_blob_detail.file_path=m.into_os_string().into_string().unwrap();
                                if buy_blob_detail.namespace.len()>0 {
                                    timer.start();
                                }
                            }
                        );
                    };
                },
                egui_file::State::Closed =>{
                    self.open_file_dialog=None;
                },
                egui_file::State::Cancelled =>{
                    self.open_file_dialog=None;
                },
                _ =>{ }
            };
        };
    }
}

