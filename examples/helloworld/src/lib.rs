use wui::*;

#[no_mangle]
pub fn main() -> () {
    render();
}

fn render(){
    let wui = globals::get::<Wui>().lock();
    wui.render(view!{
        Text("Hello World!")
    });
}