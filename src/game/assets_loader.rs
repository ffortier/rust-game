use std::{cell::RefCell, rc::Rc};

use js_sys::{Array, Function, Promise};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlImageElement};

use super::error::InitError;

pub struct AssetsLoader {
    pub base_path: String,
}

impl AssetsLoader {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            ..Default::default()
        }
    }

    pub async fn load_image(&self, name: &str) -> Result<HtmlImageElement, InitError> {
        let document = window()
            .ok_or(InitError::NoWindow)?
            .document()
            .ok_or(InitError::NoDocument)?;

        let image = document
            .create_element("img")
            .map_err(|err| InitError::ElementCreationFailed(format!("{err:?}")))?
            .unchecked_into::<HtmlImageElement>();

        let src = format!("{}{}", self.base_path, name);

        log::debug!("loading image {}", &src);

        image.set_src(&src);

        let image = Rc::new(RefCell::new(Some(image)));
        let p = create_load_promise(image.clone());

        JsFuture::from(p).await.unwrap_throw();

        Ok(image.take().unwrap())
    }
}

impl Default for AssetsLoader {
    fn default() -> Self {
        Self {
            base_path: "/assets/".to_string(),
        }
    }
}

fn create_load_promise(image: Rc<RefCell<Option<HtmlImageElement>>>) -> Promise {
    Promise::new(&mut move |resolve, reject| {
        let load_cb_ref = Rc::new(RefCell::new(None));
        let error_cb_ref = Rc::new(RefCell::new(None));

        let load_cb = create_promise_callback(
            resolve,
            load_cb_ref.clone(),
            error_cb_ref.clone(),
            image.clone(),
            "load",
        );

        let error_cb = create_promise_callback(
            reject,
            load_cb_ref.clone(),
            error_cb_ref.clone(),
            image.clone(),
            "error",
        );

        load_cb_ref.replace(Some(load_cb));
        error_cb_ref.replace(Some(error_cb));

        add_event_listener_with_callback(&image, "load", &load_cb_ref);
        add_event_listener_with_callback(&image, "error", &error_cb_ref);
    })
}

fn create_promise_callback(
    func: Function,
    load_cb_ref: Rc<RefCell<Option<Function>>>,
    error_cb_ref: Rc<RefCell<Option<Function>>>,
    image: Rc<RefCell<Option<HtmlImageElement>>>,
    name: &'static str,
) -> Function {
    Closure::once_into_js(move || {
        log::debug!("promise callback {name}");

        func.apply(&JsValue::NULL, &Array::new()).unwrap();

        remove_event_listener_with_callback(&image, "load", load_cb_ref);
        remove_event_listener_with_callback(&image, "error", error_cb_ref);
    })
    .dyn_into::<Function>()
    .unwrap()
}

fn remove_event_listener_with_callback(
    image: &Rc<RefCell<Option<HtmlImageElement>>>,
    event_type: &str,
    cb: Rc<RefCell<Option<Function>>>,
) {
    image
        .borrow()
        .as_ref()
        .unwrap()
        .remove_event_listener_with_callback(event_type, cb.borrow().as_ref().unwrap())
        .unwrap();

    cb.take();
}

fn add_event_listener_with_callback(
    image: &Rc<RefCell<Option<HtmlImageElement>>>,
    event_type: &str,
    cb: &Rc<RefCell<Option<Function>>>,
) {
    image
        .borrow()
        .as_ref()
        .unwrap()
        .add_event_listener_with_callback(event_type, cb.borrow().as_ref().unwrap())
        .unwrap();
}
