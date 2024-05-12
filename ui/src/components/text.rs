use crate::helper;
use std::{fmt,format};
use chrono::{Utc,DateTime};
use cel_wallet::tx::TxnListResponse;

#[derive(Clone)]
pub struct WalletText<F>
where F: FnOnce(String) -> String + Clone{
    text: Vec<String>,
    font_id: Vec<egui::FontId>,
    text_color:  Vec<egui::Color32>,
    height: Option<f32>,
    //center_align: bool,
    is_dim_pct: bool,
    alignment: egui::Align,
    is_vertical: bool,
    back_color: egui::Color32,
    clicked: bool,
    width: Option<f32>,
    is_hyperlink: bool,
    hyperlink_fn: Option<F> 
}

impl<F> fmt::Debug for WalletText<F> 
where F: FnOnce(String) -> String  + Clone{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"text={:?}\r\n",self.text)
    }
}

impl<F> WalletText<F>
where F: FnOnce(String) -> String + Clone{
    pub fn new(text: Vec<String>, font_size: Vec<f32>,
        text_color: Vec<egui::Color32>, height: Option<f32>,alignment: egui::Align,
        is_vertical:bool,is_dim_pct: bool, hyperlink_fn : F)->Self{
         
        WalletText{
            text,
            alignment,
            text_color,
            height,
            font_id: font_size.iter().map(|m| egui::FontId::monospace(*m)).collect::<Vec<egui::FontId>>(),
            is_dim_pct,
            is_vertical,
            back_color: egui::Color32::TRANSPARENT,
            clicked: false,
            width: None,
            is_hyperlink: false,
            hyperlink_fn: Some(hyperlink_fn),
        }
    }

    pub fn set_size(&mut self, width: Option<f32>,height: Option<f32>){
        self.width=width;
        self.height=height;
    }
    pub fn get_texts(&self)->Vec<String>{
        self.text.clone()
    }

    pub fn set_back_color(&mut self,color : egui::Color32){
        self.back_color=color;
    }

    pub fn set_texts(&mut self, texts: Vec<String>){
        self.text=texts;
    }

    fn calculate_margin(&self, screen_width: f32, screen_height: f32)->f32{
        let height=self.height.unwrap_or(0.0);
        if !self.is_dim_pct {
            return height;
        };
        let margin = if self.is_vertical == true {
            height*screen_height
        }else{
            height*screen_width
        };
        margin
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        
        self.clicked=false;
        if self.is_vertical == true{
            ui.with_layout(egui::Layout::top_down(self.alignment),|ui|{
                egui::Frame::default().fill(self.back_color).show(ui,|ui| 
                {
                    ui.vertical_centered(|ui|{
                        self.text_ui(ui,screen_width,screen_height);
                    });
                });
            });
        }else{
            // ui.with_layout(egui::Layout::top_down(self.alignment),|ui|{
                
            // });
            ui.vertical(|ui|{
                egui::Frame::default().fill(self.back_color).show(ui,|ui| 
                    {
                        
                        self.text_ui(ui,screen_width,screen_height);
                        
                    });
            });
        };
    }

    fn text_ui(&mut self,ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        let margin = self.calculate_margin(screen_width,screen_height);
        ui.add_space(margin/2.0);
        if self.text.len() ==0 {
            ui.label("");
        }else{
            for i in 0..self.text.len(){
                let value=self.text.get(i).unwrap();
                
                 if self.is_hyperlink == false{
                    let label= egui::Label::new( egui::widget_text::RichText::new(value.as_str())
                    .font(self.font_id.get(i).unwrap().clone())
                    .color(self.text_color.get(i).unwrap().clone())).sense(egui::Sense::click());
                    if self.width.is_some() && self.height.is_some() {
                        if ui.add_sized(
                            [self.width.unwrap()*screen_width,self.height.unwrap()*screen_height],
                            label
                        ).clicked() {
                            self.clicked=true;
                            //println!("Text clicked");
                        }
                    }else{
                        if ui.add(
                            label
                        ).clicked() {
                            self.clicked=true;
                            //println!("Text clicked");
                        }
                    }
                }else{
                    self.hyperlink_fn.as_ref().map(|func| {
                        let hyperlink= egui::Hyperlink::from_label_and_url(
                            egui::widget_text::RichText::new(value.as_str())
                        .font(self.font_id.get(i).unwrap().clone())
                        .color(self.text_color.get(i).unwrap().clone()),
                        (func.clone())(value.clone()),
                        );
                        if self.width.is_some() && self.height.is_some() {
                            ui.add_sized(
                                [self.width.unwrap()*screen_width,self.height.unwrap()*screen_height],
                                hyperlink
                            );
                        }else{
                            ui.add(
                                hyperlink
                            );
                        };
                    });
                };
                
            }
        };
        ui.add_space(margin/2.0);
    }

    pub fn get_is_clicked(&self)->bool{
        self.clicked
    }

    pub fn enable_hyperlink(&mut self,val: bool){
        self.is_hyperlink=val;
    }
 
}


pub type DefaultHyperlinkFunction = fn(String)->String;

#[derive(Clone)]
pub struct CoinText{
    pub symbol: String,
    pub value: String,
    pub usd: String,
    pub label: WalletText<DefaultHyperlinkFunction>,
}

impl CoinText{
    pub fn new(font_sizes:Vec<f32>,colors: Vec<egui::Color32>,margin: Option<f32>)->Self{
        CoinText{
            symbol: "".to_string(),
            value: "0.0".to_string(),
            usd: "0.0".to_string(),
            label: WalletText::new(
                vec![], font_sizes,
            colors,
            margin,egui::Align::Center,true,true,
            move |s| format!("{}",s)
            )
        }
    }

    pub fn set_value(&mut self, symbol: String, value: String, usd: String){
        self.symbol=symbol;
        self.value=value;
        self.usd=usd;
        self.label.set_texts(vec![format!("{} {}",self.value,self.symbol),format!("${}",self.usd)]);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        self.label.ui(ui,screen_width,screen_height);
    }

    pub fn set_back_color(&mut self,color : egui::Color32){
        self.label.set_back_color(color);
    }
}



#[derive(Clone)]
pub struct AddressText{
    pub name: String,
    pub address: String,
    pub title: String,
    pub label: WalletText<DefaultHyperlinkFunction>,
}


impl AddressText{
    pub fn new(font_sizes:Vec<f32>,colors: Vec<egui::Color32>,margin: Option<f32>)->Self{
        AddressText{
            name: "".to_string(),
            address: "".to_string(),
            title: "".to_string(),
            label: WalletText::new(
                vec![], font_sizes,
            colors,
            margin,egui::Align::Center,false,true,
            move |s| format!("{}",s)
            )
        }
    }


    pub fn set_value(&mut self,name: String, address: String, title: String, is_truncated:bool){
        self.address=address.clone();
        self.title=title.clone();
        self.name=name.clone();
        if is_truncated == true {
            self.label.set_texts(vec![helper::get_wrapped_text(title,4),helper::get_wrapped_text(address,4)]);
        }else{
            self.label.set_texts(vec![title,address]);
        }
    }
 
    pub fn get_value(&self)->(String,String,String){
        (self.name.clone(),self.address.clone(),self.title.clone())
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        self.label.ui(ui,screen_width,screen_height);
    }

    
    pub fn get_is_clicked(&self)->bool{
        self.label.clicked
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>){
        self.label.set_size(width,height);
    }
}

#[derive(Clone)]
pub struct MsgText{
    pub value: String,
    pub label: WalletText<DefaultHyperlinkFunction>,
}


impl MsgText{
    pub fn new(font_size:f32,color: egui::Color32,margin: Option<f32>)->Self{
        MsgText{
            value: "".to_string(),
            label: WalletText::new(
                vec![], vec![font_size],
            vec![color],
            margin,egui::Align::Min,true,true,
            move |s| format!("{}",s)
            )
        }
    }


    pub fn set_value(&mut self, value: String){
        self.value=value.clone();
        self.label.set_texts(vec![value]);
    }


    pub fn ui(&mut self, ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        self.label.ui(ui,screen_width,screen_height);
    }


}
 
#[derive(Clone,Debug)]
pub struct TxnText{
    pub hash: String,
    pub url: Option<String>,
    pub is_success: bool,
    pub timestamp: DateTime<Utc>,
    pub timestamp_text: WalletText<DefaultHyperlinkFunction>,
    pub label: WalletText<DefaultHyperlinkFunction>,
}

impl TxnText{
    
    pub fn ui(&mut self, ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        //ui.label(self.hash.clone());
           
            self.timestamp_text.ui(ui,screen_width,screen_height);
            
            self.label.ui(ui,screen_width,screen_height);
        
    }
}

impl From<&TxnListResponse> for TxnText {
    fn from(a: &TxnListResponse)->TxnText{
        let is_success = if a.status == "success" {
            true
        }else{
            false
        };
        let hash=helper::get_wrapped_text(a.hash.clone(),12);
        
        let day= format!("{}", a.time.format("%d/%m/%Y"));
        let time= format!("{}", a.time.format("%H:%M:%S"));

        let mut txn_text= TxnText{
            hash: a.hash.clone()  ,
            url : None,
            is_success,
            timestamp: a.time,
            timestamp_text: WalletText::new(
                vec![day,time], vec![super::TX_LIST_HASH_FONT_SIZE,super::TX_LIST_HASH_FONT_SIZE],
                vec![super::TX_LIST_HASH_COLOR,super::TX_LIST_HASH_COLOR],
                None,egui::Align::Min,false,true,
                move |s| format!("{}",s)
            ),
            label: WalletText::new(
                vec![ hash.clone()], vec![super::TX_LIST_HASH_FONT_SIZE],
                vec![super::TX_LIST_HASH_COLOR],
                Some(0.02),egui::Align::Min,false,true,
                move |s| format!("https://arabica.celenium.io/tx/{}?tab=messages",s)
            ),
        };
        txn_text.label.set_size(Some(0.78),Some(0.05));
        txn_text.timestamp_text.set_size(Some(0.22),Some(0.025));
        txn_text.label.enable_hyperlink(true);
        txn_text
    }
}

