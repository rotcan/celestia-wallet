use crate::components::tx_response::{TxResponse,TxState};


pub struct MessageBlock{
    pub tx_response: TxResponse,
    visible : bool,
}

impl MessageBlock{
    pub fn new()->Self{
        MessageBlock{
            tx_response: TxResponse::new(0.1),
            visible: true,
        }
    }

}

impl MessageBlock{

    pub fn clear_state(&mut self){
        self.tx_response.clear_state();
    }
    
    pub fn set_visible(&mut self, val: bool){
        self.visible=val;
    }

    pub fn get_visible(&self)->bool{
        self.visible
    }

    pub fn update_tx_response(&mut self,state: TxState, message: String){
        self.tx_response.set_state(state, message);
    }
   
    pub fn ui(&mut self, ui: &mut egui::Ui,width: f32, height: f32){
        self.tx_response.ui(ui,width,height);
    }
}