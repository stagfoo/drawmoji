
#[cfg(feature = "wee_alloc")]
extern crate wasm_bindgen;
extern crate stdweb;

use wasm_bindgen::prelude::*;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d
};

use stdweb::web::event::{
    MouseMoveEvent,
    ResizeEvent,
};

use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::InputElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[wasm_bindgen]
extern "C" {
    //pull js function or "shims"
    fn console_log(s: &str);
    type HTMLDocument;
    type Element;
    //create marcos in rust
}

#[wasm_bindgen]
pub fn set_color(){
    //Set color
}

#[wasm_bindgen]
pub fn set_size(){
    //Set color
}

#[wasm_bindgen]
pub fn create_size_slider(){
    let slider: InputElement = document().create_element("input").unwrap().try_into().unwrap();
    slider.set_attribute("type", "range");
    slider.set_attribute("step", "25");
    document().query_selector( "body" ).unwrap().unwrap().append_child(&slider);
}


#[wasm_bindgen]
pub fn create_canvas(){
    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);
    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));
    canvas.add_event_listener( enclose!( (context) move |event: MouseMoveEvent| {
        context.set_fill_style_color("#abd2e4");
        context.fill_rect(f64::from(event.client_x()), f64::from(event.client_y())
                          , 10.0, 10.0);
    }));
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    stdweb::initialize();

    create_canvas();
    create_size_slider();
    stdweb::event_loop();
    Ok(())
}