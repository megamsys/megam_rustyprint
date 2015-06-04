extern crate term_painter;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use std::fmt;
use std::vec::Vec;

pub struct Printer {
    pub header    : Vec<String>,
    pub body			: Vec<Vec<String>>,
}

impl Printer {

  pub fn new() -> Printer {
     Printer {
					header: Vec::new(),
          body: Vec::new(),
     }
  }

  pub fn set_header(&mut self, head: Vec<String>) {
     self.header = head;
   }

  pub fn set_body(&mut self, tbody: Vec<Vec<String>>) {
     self.body = tbody;
  }  

  fn get_size(&self, column: usize) -> usize {
		 let mut size = 0;
		if size < self.header[column].len() {
			size = self.header[column].len();
		}
		for b in self.body.iter() {
				if size < b[column].len() {
							size = b[column].len();
				}
	  }
   size
  }
}



impl fmt::Debug for Printer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
     let mut flag = true;
		 let mut headcol = 0;
		 let mut bodycol = 0; 
     
     for y in self.body.iter() {
        if self.body[0].len() != y.len() || self.header.len() != y.len() {
						 headcol = self.header.len();
						 bodycol = y.len();
             flag = false;
         }
      }    
		
     if flag == false {
 				try!(write!(fmt, "{}", Plain.bold().fg(Red).paint(format!("Error: Unmatched header cols: {} and body cols: {}", headcol, bodycol))));
     } else {   
					
				let mut i = 0;
					for x in self.header.iter() {  
							if i == 0 {
									try!(write!(fmt, "{}", Blue.paint(" * ")));	
							}
							for s in 0..self.get_size(i) {
  							try!(write!(fmt, "{}", Blue.paint("-")));
							}	
							i = i + 1;		
          	try!(write!(fmt, "{}", Blue.paint(" * ")));			
					}
					println!("{}","");

          let mut k = 0;  
					for x in self.header.iter() {  
							if k == 0 {
									try!(write!(fmt, "{}", Blue.paint(" | ")));	
							}
							try!(write!(fmt, "{}", Plain.bold().fg(BrightWhite).paint(x)));
							for s in 0..self.get_size(k) - x.len() {
  								try!(write!(fmt, "{}", " "));								
							}	 
							k = k + 1;
							try!(write!(fmt, "{}", Blue.paint(" | ")));
					}  
       		println!("{}","");
          
					let mut j = 0;
					for x in self.header.iter() {  
					    if j == 0 {
									try!(write!(fmt, "{}", Blue.paint(" * ")));	
							}				
							for s in 0..self.get_size(j) {
  							try!(write!(fmt, "{}", Blue.paint("-")));
							}	
							j = j + 1;		
          	try!(write!(fmt, "{}", Blue.paint(" * ")));			
					}
					println!("{}","");

				let mut q = 0;
        for z in self.body.iter() {
						q = 0;
						for b in z.iter() {
								if q == 0 {
									try!(write!(fmt, "{}", Blue.paint(" | ")));	
							  }
								try!(write!(fmt, "{}", Plain.fg(White).paint(b)));
								for s in 0..self.get_size(q) - b.len() {
  								try!(write!(fmt, "{}", " "));								
							   }	 
							   q = q + 1;
							   try!(write!(fmt, "{}", Blue.paint(" | ")));
						}
						println!("{}","");
				}

       let mut i = 0;
			 for x in self.header.iter() { 
				 if i == 0 {
									try!(write!(fmt, "{}", Blue.paint(" * ")));	
					}				
					for s in 0..self.get_size(i) {
  					try!(write!(fmt, "{}", Blue.paint("-")));
					}	
					i = i + 1;		
          try!(write!(fmt, "{}", Blue.paint(" * ")));			
				}
				println!("{}","");
			}

     Ok(())
    }
}