use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let colors = vec!["#F4908E", "#F2F097", "#88B0DC", "#F7B5D1", "#53C4AF", "#FDE38C"];
    let sizes = vec![3.0, 6.0, 9.0, 12.0, 15.0, 20.0];

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;

    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    { mouse_down(&context, &pressed, &canvas); }
    { mouse_move(&context, &pressed, &canvas); }
    { mouse_up(&context, &pressed, &canvas); }
    { touch_up(&context, &pressed, &canvas); }

    // Create divs for color picker
    for c in colors {
        let div = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlElement>()?;
        div.set_class_name("color");
        {
            click(&context, &div, c.clone());
        }
        div.style().set_property("background-color", c);
        let div = div.dyn_into::<web_sys::Node>()?;
        document.body().unwrap().append_child(&div)?;
    }
    for ss in sizes {
        let div = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlElement>()?;
        
        {
            clickSize(&context, &div, ss.clone());
        }
        div.set_class_name("size");
        div.set_id(&ss.to_string());
        let div = div.dyn_into::<web_sys::Node>()?;
        document.body().unwrap().append_child(&div)?;
    }

    Ok(())
}

fn click(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, div: &web_sys::HtmlElement, c: &str) {
    let context = context.clone();
    let c = JsValue::from(String::from(c));
    let closure = Closure::wrap(Box::new(move || {
        context.set_stroke_style(&c);            
    }) as Box<dyn FnMut()>);

    div.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
fn clickSize(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, div: &web_sys::HtmlElement, c:f64) {
    let context = context.clone();
    let closure = Closure::wrap(Box::new(move || {
        context.set_line_width(c);
    }) as Box<dyn FnMut()>);

    div.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
    //http://bencentra.com/code/2014/12/05/html5-canvas-touch-events.html
    // fake events
    //
fn touch_up(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        pressed.set(false);
        context.line_to(event.offset_x() as f64, event.offset_y() as f64);
        context.stroke();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref()).unwrap();
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
fn mouse_up(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        pressed.set(false);
        context.line_to(event.offset_x() as f64, event.offset_y() as f64);
        context.stroke();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref()).unwrap();
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn mouse_move(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement){
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if pressed.get() {
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
            context.begin_path();
            context.set_line_cap("round");
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        }
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn mouse_down(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement){
    let context = context.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        context.begin_path();
        context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        pressed.set(true);
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    canvas.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}