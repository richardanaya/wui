use wui::*;

#[no_mangle]
pub fn main() -> () {
    let mut wui = globals::get::<Wui>().lock();
    wui.initialize();
    wui.render(view!{
        App {
            Text("Hello World!")
        }
    });
}