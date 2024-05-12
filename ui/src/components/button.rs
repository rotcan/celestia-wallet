#[derive(Clone)]
pub struct WalletButton{
    margin: f32,
    label: String,
    width: f32,
    height :f32,
    alignment: egui::Align,
    is_clicked: bool,
    font_id: egui::FontId,
    is_vertical: bool,
    is_dim_pct: bool,
    is_visible: bool,
}

impl WalletButton{
    pub fn new(margin: f32,alignment: egui::Align, font_size: f32,
    label: String, width: f32, height: f32,is_vertical: bool,is_dim_pct: bool)->Self{
        
     
        WalletButton{
            margin,
            label,
            alignment,
            width,
            height,
            is_clicked: false,
            font_id: egui::FontId::monospace(font_size),
            is_vertical,
            is_dim_pct,
            is_visible: true,
        }
    }

    fn calculate_margin(&self, screen_width: f32, screen_height: f32)->f32{
        if !self.is_dim_pct {
            return self.margin;
        };
        let margin = if self.is_vertical == true {
            self.margin*screen_height
        }else{
            self.margin*screen_width
        };
        margin
    }

    pub fn set_visible(&mut self,v: bool){
        self.is_visible=v;
    }
 
    fn calculate_dimension(&self,value: f32, full_value :f32)->f32{
        if !self.is_dim_pct {
            return value;
        };
        value * full_value
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, screen_width: f32, screen_height: f32){
        self.is_clicked=false;
        
        if self.is_visible {
            let margin = self.calculate_margin(screen_width,screen_height);
            let width=self.calculate_dimension(self.width, screen_width);
            let height=self.calculate_dimension(self.height, screen_height);
            ui.add_space(margin);
            if self.is_vertical==false {
                self.button_ui(ui,width,height);
            }else{
                ui.with_layout(egui::Layout::top_down(self.alignment),|ui|{
                    //ui.add_space(margin_left);
                    self.button_ui(ui,width,height);
                });
            };
        };
    }

    fn button_ui(&mut self, ui: &mut egui::Ui,width: f32,height: f32){
        if ui.add_sized([width,height],
            egui::Button::new(
                //self.label.as_str()
                egui::widget_text::RichText::new(self.label.as_str()).font(self.font_id.clone())
            )).clicked() {
            self.is_clicked=true;
        };
    }

    pub fn get_is_clicked(&self)->bool{
        self.is_clicked
    }

}