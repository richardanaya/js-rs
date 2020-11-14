#![no_std]
use callback::*;
use js::*;

pub fn get_element_by_id(id: &str) -> JSObject {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(strPtr,strLen){
                    let el = document.getElementById(this.readUtf8FromMemory(strPtr,strLen)); 
                    return this.storeObject(el);
            }",
        )
    })
    .invoke_2(id.as_ptr() as u32, id.len() as u32)
    .into()
}

pub fn add_event_listener(
    dom: impl Into<f64>,
    event: &str,
    handler: impl FnMut(f64) -> () + Send + 'static,
) {
    let cb = create_callback_1(handler);
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(el,strPtr,strLen,callback){
                    el = this.getObject(el);
                    const event = this.readUtf8FromMemory(strPtr,strLen);
                    el.addEventListener(event,this.createCallback(callback));
            }",
        )
    })
    .invoke_4(dom.into(), event.as_ptr() as u32, event.len() as u32, cb);
}

pub struct KeyDownEvent {
    pub handle: JSObject,
}

impl KeyDownEvent {
    pub fn from_event(ev: impl Into<JSObject>) -> KeyDownEvent {
        KeyDownEvent { handle: ev.into() }
    }

    pub fn key_code(&self) -> u32 {
        static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
        FN.get_or_init(|| {
            register_function(
                "function(ev){
                        ev = this.getObject(ev);
                        return ev.keyCode;
                }",
            )
        })
        .invoke_1(&self.handle) as u32
    }
}