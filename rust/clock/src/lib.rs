use std::fmt;
use std::char;
#[derive(Debug)]
pub struct Clock{
    hours:i32,
    minutes:i32
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut  hours = hours + minutes/60 as i32;
        let mut minutes = minutes%60;
       
        if minutes < 0 {
            minutes = 60+minutes;
            hours =hours-1;

        }

         if hours < 0 {
            hours = 24+hours%24;
        }
       
        Clock{hours:hours%24,minutes}
        
    }

    pub fn to_string(self)->String{
        let width =2 ;
        format!("{:0width$}",self,width=width)
    }


    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_min = minutes+self.minutes;
        let  add_hrs =  new_min/60 as i32 ;
        let  add_min = new_min%60 ;
        let mut new_hrs = self.hours;
        println!("add min = {}. add hr = {}",new_min,new_hrs);
       if add_min<0{
          
        new_min = 60+add_min ;
        new_hrs = new_hrs-1

       }else{
           new_min = add_min;
       }

       if add_hrs <0 || new_hrs <0{
           new_hrs = new_hrs +add_hrs;
           new_hrs = (24+new_hrs)%24; 
       }else {
           new_hrs = new_hrs+add_hrs;
           new_hrs = new_hrs%24;
       }
       println!("min = {}.hr = {}",new_min,new_hrs);

        
         

         Clock{hours:new_hrs,minutes:new_min}
        
        
        
    }
    
}


impl fmt::Display for Clock{
    fn fmt(&self,f: &mut fmt::Formatter<'_>)->fmt::Result{

        //let s = format!("{}{}","0",self.hours.to_string())  ;
        //let s_len  =s.len();
        //let m = format!("{}{}","0",self.minutes.to_string());
       // let m_len  = m.len();
        let width = 2;
        write!(f,"{:0width$}:{:0width$}",self.hours,self.minutes,width=width)
       
    

    }
}

impl PartialEq for Clock{
    fn eq(&self,other: &Clock)->bool{
        self.hours == other.hours  && self.minutes == other.minutes
    }

}