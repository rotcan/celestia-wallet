use crate::app::{AppState, AppView};
use crate::components::text::TxnText;
use cel_wallet::tx::TxnListResponse;

#[derive(Clone,Debug)]
pub struct TxnListBlock{
    txns: Vec<TxnText>,
}

impl TxnListBlock{
    pub fn new()->Self{
        TxnListBlock{
            txns: vec![],
        }
    }
    
    pub fn clear(&mut self){
        self.txns.clear();
    }

    pub fn add_txns(&mut self, list: Vec<TxnListResponse>){
        list.iter().for_each(|txn| {
            let txn_text: TxnText = txn.into();
            self.txns.push(txn_text);
        });
    }

    pub fn set_txns(&mut self, list: Vec<TxnListResponse>){
        self.clear();
        self.add_txns(list);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, width: f32, height : f32, state: &mut AppState, view: &mut AppView){

        egui::ScrollArea::vertical()
            .max_height(f32::INFINITY)
            .show(ui, |ui| {

           
                egui::Grid::new("txn_list_grid")
            // .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    
                    for txn in self.txns.iter_mut(){
                        let color = if txn.is_success == false {
                            egui::Color32::DARK_RED
                        }else{
                            egui::Color32::DARK_GREEN
                        };
                        egui::Frame::none()
                        .fill(color)
                        .show(ui,|ui|{   
                            //ui.horizontal(|ui| {
                                txn.ui(ui,width,height);    
                            //});
                        });
                        ui.end_row();
                    }
                });
           
        });
    }   
}