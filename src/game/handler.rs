use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{Event, EventTarget};

pub struct EventHandler {
    listener: Closure<dyn FnMut(Event)>,
    target: Rc<dyn AsRef<EventTarget>>,
    type_: String,
}

impl EventHandler {
    pub fn new(
        target: Rc<dyn AsRef<EventTarget>>,
        type_: &str,
        handler: impl Fn(&Event) + 'static,
    ) -> Self {
        let handler = Rc::new(handler);
        let listener = Closure::new(move |event: Event| handler(&event));

        target
            .as_ref()
            .as_ref()
            .add_event_listener_with_callback(&type_, listener.as_ref().unchecked_ref())
            .unwrap_throw();

        Self {
            listener,
            target,
            type_: type_.to_string(),
        }
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.target
            .as_ref()
            .as_ref()
            .remove_event_listener_with_callback(
                &self.type_,
                self.listener.as_ref().unchecked_ref(),
            )
            .unwrap_throw();
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_event_handler() {
        let test_target = Rc::new(EventTarget::new().unwrap_throw());
        let count = Rc::new(RefCell::new(0));
        let count2 = count.clone();
        let _handler = EventHandler::new(test_target.clone(), "test", move |event| {
            event.prevent_default();
            *count.borrow_mut() += 1;
        });

        let e1 = Event::new("test").unwrap_throw();
        let e2 = Event::new("test").unwrap_throw();

        test_target.dispatch_event(&e1).unwrap_throw();
        test_target.dispatch_event(&e2).unwrap_throw();

        assert_eq!(*count2.borrow(), 2, "expected 2");
    }

    #[wasm_bindgen_test]
    fn test_remove_callback() {
        let count = Rc::new(RefCell::new(0));
        let test_target = Rc::new(EventTarget::new().unwrap_throw());

        {
            let count = count.clone();

            let _handler = EventHandler::new(test_target.clone(), "test", move |event| {
                event.prevent_default();
                *count.borrow_mut() += 1;
            });

            let event = Event::new("test").unwrap_throw();

            test_target.dispatch_event(&event).unwrap_throw();
        }

        let event = Event::new("test").unwrap_throw();

        test_target.dispatch_event(&event).unwrap_throw();

        assert_eq!(*count.borrow(), 1, "expected 1");
    }
}
