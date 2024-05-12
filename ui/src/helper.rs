use std::format;
use std::ops::{MulAssign,DivAssign};

pub fn get_rounded_value(val: u128, exponent: u32)->String{
    let base: u32=10;
    let f_val=val as f32/base.pow(exponent) as f32;
    //println!("f_val={}",val,exp);
    format!("{:.2}", f_val)    
}

pub fn get_wrapped_text(val: String,char_count: u32)->String{
    let char_count=char_count as usize;
    if val.len() < char_count * 2 {
        return val;
    };
    let (first,_)=val.split_at(char_count);
    let (_,end)=val.split_at(val.len()-char_count);
    format!("{}..{}",first,end)
}

pub fn convert_amount(val: &str, exponent: u32)->Result<u128,std::num::ParseFloatError>{
    let float_amount=val.parse::<f32>()?;
    let base: u32=10;
    let amount: u128 = (float_amount * base.pow(exponent) as f32).round() as u128;
    Ok(amount)
}
 

pub fn add_pct_to_u64(val: u64, pct: f32)->u64{
    let mut val=val;
    let base: u64=10;
    let exponent: u32=4;
    let numerator= (pct * base.pow(exponent) as f32).round() as u64;
    val.mul_assign( base.pow(exponent) as u64 + numerator);
    val.div_assign( base.pow(exponent) as u64);
    val
}

pub fn get_font_id(font_size: f32)->egui::FontId{
    egui::FontId::monospace(font_size)
}

pub fn get_label(text: &str, font_size: f32, underline: bool )->egui::widget_text::RichText{
    let font_id=egui::FontId::monospace(font_size);
    let mut widget=egui::widget_text::RichText::new(text).font(font_id);
    if underline == true{
        return widget.underline();
    };
    widget

}

pub fn password_validation(password: &str)->bool{
    //just length for now
    if password.len() >5 {
        return true;
    };
    false
}