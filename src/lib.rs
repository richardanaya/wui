#![no_std]
extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;
use js_ffi::*;

#[derive(Default)]
pub struct App {
}

impl App {
    pub fn construct(&self,_children:Option<Vec<View>>){}
}

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

#[derive(Debug)]
struct NodeModification{

}

pub struct Wui {
    fn_initialize:JSValue,
    fn_logger:JSValue,
    current:Option<View>,
    changes:Vec<NodeModification>
}

pub enum View {
    App(App),
    Text(Text)
}

impl Default for Wui {
    fn default() -> Self {
        Wui{
            fn_initialize:register(r#"()=>{
                console.log("initializing");
                window.wui_nodes = [];
            }"#),
            fn_logger:register("console.log"),
            current:None,
            changes:Vec::new()
        }
    }
}

impl Wui {
    pub fn initialize(&self){
        call_0(UNDEFINED, self.fn_initialize);
    }

    pub fn log(&self,m:&str){
        call_1(UNDEFINED, self.fn_logger, TYPE_STRING, to_js_string(m));
    }

    pub fn render(&mut self, v:View){
        self.current = Some(v);
        self.log("rendering");
        self.flush_changes();
    }

    pub fn flush_changes(&mut self){
        // send changes to native side
        for c in self.changes.iter() {
            self.log(&format!("sending change {:?}",c));
        }
        self.changes.clear()
    }
}

pub use view::view;