use std::time::{Duration,Instant};

pub struct Timer{
    active : bool,
    once: bool,
    limit: Duration,
    current_time: Instant,
    is_finish: bool,
}

impl Timer{
    pub fn new(seconds: u64)->Self{
        Timer{
            active: false,
            once: true,
            limit: Duration::new(seconds, 0),
            current_time: Instant::now(),
            is_finish: false,
        }
    }

    pub fn set_once(&mut self,once: bool){
        self.once=once;
    }

    pub fn start(&mut self){
        self.active=true;
        self.current_time=Instant::now();
    }

    pub fn update(&mut self){
        // if self.active == true {
        //     println!("time diff = {:?}",Instant::now().checked_duration_since(self.current_time));
        // };
        if self.active == true && Instant::now().checked_duration_since(self.current_time) > Some(self.limit) {
            self.is_finish = true;
        }
    }

    pub fn consume(&mut self)->bool{
        if self.active == true && self.is_finish == true{
            self.is_finish = false;
            if self.once {
                self.active=false;
            }else{
                //todo decrease by limit
                self.start();
            };
            return true;
        };
        false
    }
}