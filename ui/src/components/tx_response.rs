use crate::components::text::MsgText;

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum TxState{
    None,
    Pending,
    Success,
    Failure,
}
pub struct TxResponse{
    result: Option<String>,
    success_label: MsgText,
    error_label: MsgText,
    tx_state: TxState,
}

impl TxResponse{
    pub fn new(margin: f32)->Self{
        TxResponse{
            result : None,
            success_label: MsgText::new(12.0,
                egui::Color32::WHITE,Some(margin)),
            error_label: MsgText::new(12.0,
                egui::Color32::RED,Some(margin)),
            tx_state: TxState::None,
        }
    }

    pub fn clear_state(&mut self){
        self.tx_state=TxState::None;
    }
    
    pub fn set_state(&mut self, state: TxState, msg: String){
        self.tx_state=state;
        self.result=Some(msg.clone());
        if self.tx_state == TxState::Success ||
        self.tx_state == TxState::Pending  { 
            self.success_label.label.set_texts(vec![msg]);
        }else if self.tx_state == TxState::Failure  {
            self.error_label.label.set_texts(vec![msg]);
        };
    }

    pub fn get_state(&self)->TxState{
        self.tx_state
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, width: f32, height: f32){
        if self.tx_state == TxState::Success {
            self.success_label.ui(ui,width,height);
        }else if self.tx_state == TxState::Failure {
            self.error_label.ui(ui,width,height);
        }else if self.tx_state == TxState::Pending {
            self.success_label.ui(ui,width,height);
        }
    }
}