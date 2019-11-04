#![no_std]
extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use js_ffi::*;

#[derive(Default)]
pub struct Text {
    content: String
}

impl Text {
    pub fn new(t:&str) -> Self{
        Text {
            content:t.to_string()
        }
    }

    pub fn construct(&self,_children:Option<Vec<View>>){

    }
}

pub struct Wui {
    fn_logger:JSValue
}

pub enum View {
    Text(Text)
}

impl Default for Wui {
    fn default() -> Self {
        Wui{
            fn_logger:register("console.log")
        }
    }
}

impl Wui {
    pub fn render(&self, v:View){
        call_1(UNDEFINED, self.fn_logger, TYPE_STRING, to_js_string("Hello World"));
    }
}

pub use view::view;