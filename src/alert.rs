use wasm_bindgen::{prelude::*, JsCast};

/// Display an alert message to the user
pub fn alert(message: &str){
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let error_container = document.get_element_by_id("errors").unwrap();
    let alert = document.create_element("div").unwrap();
    alert.set_inner_html(message);
    alert.set_class_name("alert");
    
    error_container.append_child(&alert).unwrap();
    
    let on_click = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let src  =  event.target().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
        if src.class_name() == "alert" {
            src.remove();
        }
    }) as Box<dyn FnMut(_)>);

    error_container.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref()).unwrap();
    on_click.forget();
}