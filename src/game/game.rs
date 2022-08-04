use std::{cell::RefCell, rc::Rc};

use super::{
    error::InitError, handler::EventHandler, logic::GameLogic, renderer::Renderer, GameOptions,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    window, CanvasRenderingContext2d, Event, EventTarget, HtmlCanvasElement, HtmlElement,
};

#[wasm_bindgen]
pub struct Game {
    canvas: Rc<HtmlCanvasElement>,
    event_target: Rc<RefCell<Option<EventTarget>>>,
    request_id: Rc<RefCell<Option<i32>>>,
    game_logic: Rc<RefCell<GameLogic>>,
    renderer: Rc<RefCell<Renderer>>,
    _keydown_handler: EventHandler,
    _keyup_handler: EventHandler,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(options: GameOptions) -> Self {
        let canvas = create_canvas(options.container()).unwrap_throw();
        let rendering_context = canvas
            .get_context("2d")
            .unwrap_throw()
            .unwrap_throw()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap_throw();

        let renderer = Rc::new(RefCell::new(Renderer::new(
            rendering_context,
            canvas.width(),
            canvas.height(),
        )));
        
        let canvas = Rc::new(canvas);
        let game_logic = Rc::new(RefCell::new(GameLogic::default()));

        let keydown_handler = {
            let game_logic = game_logic.clone();
            let window = Rc::new(window().unwrap_throw());

            EventHandler::new(window, "keydown", move |event| {
                game_logic.borrow_mut().on_keydown(event.unchecked_ref());
            })
        };

        let keyup_handler = {
            let game_logic = game_logic.clone();
            let window = Rc::new(window().unwrap_throw());

            EventHandler::new(window, "keyup", move |event| {
                game_logic.borrow_mut().on_keyup(event.unchecked_ref());
            })
        };

        Self {
            canvas,
            event_target: Rc::new(RefCell::new(None)),
            request_id: Rc::new(RefCell::new(None)),
            game_logic,
            renderer,
            _keydown_handler: keydown_handler,
            _keyup_handler: keyup_handler,
        }
    }

    pub fn run(&mut self) {
        if self.request_id.borrow().is_some() {
            return;
        }

        self.game_logic.borrow_mut().setup();

        let cb = Rc::new(RefCell::new(None));

        cb.replace(Some({
            let cb = cb.clone();
            let game_logic = self.game_logic.clone();
            let renderer = self.renderer.clone();
            let request_id = self.request_id.clone();
            let event_target = self.event_target.clone();

            Closure::new(move || {
                if let Some(event_target) = event_target.borrow().as_ref() {
                    event_target
                        .dispatch_event(&Event::new("frame").unwrap_throw())
                        .unwrap_throw();
                }

                game_logic.borrow_mut().update();
                game_logic.borrow_mut().draw(&mut renderer.borrow_mut());

                if game_logic.borrow().is_running() {
                    request_id
                        .borrow_mut()
                        .replace(request_animation_frame(cb.borrow().as_ref().unwrap()));
                } else {
                    request_id.take();
                }
            })
        }));

        self.request_id
            .borrow_mut()
            .replace(request_animation_frame(cb.borrow().as_ref().unwrap()));

        if let Some(event_target) = self.event_target.borrow().as_ref() {
            event_target
                .dispatch_event(&Event::new("running").unwrap_throw())
                .unwrap_throw();
        }
    }

    pub fn reset(&mut self) {
        self.stop();
        self.run();
    }

    pub fn stop(&mut self) {
        if let Some(request_id) = self.request_id.borrow_mut().take() {
            cancel_animation_frame(request_id);

            if let Some(event_target) = self.event_target.borrow().as_ref() {
                event_target
                    .dispatch_event(&Event::new("running").unwrap_throw())
                    .unwrap_throw();
            }
        }
    }

    #[wasm_bindgen(skip_typescript, js_name = __postConstruct)]
    pub fn post_construct(&mut self, js_this: JsValue) {
        self.event_target
            .borrow_mut()
            .replace(js_this.unchecked_into::<EventTarget>());
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.canvas.remove();
    }
}

fn create_canvas(container: Option<HtmlElement>) -> Result<HtmlCanvasElement, InitError> {
    let window = window().ok_or(InitError::NoWindow)?;
    let document = window.document().ok_or(InitError::NoDocument)?;
    let canvas = document
        .create_element("canvas")
        .map_err(|err| InitError::CanvasCreationFailed(format!("{err:?}")))?
        .unchecked_into::<HtmlCanvasElement>();

    canvas.set_width(480);
    canvas.set_height(360);

    let container = container.or(document.body());
    let container = container.as_ref().ok_or(InitError::NoContainer)?;

    container
        .append_child(&canvas)
        .map_err(|err| InitError::AppendChildFailed(format!("{err:?}")))?;

    Ok(canvas)
}

fn request_animation_frame(callback: &Closure<dyn FnMut()>) -> i32 {
    window()
        .unwrap_throw()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap_throw()
}

fn cancel_animation_frame(request_id: i32) {
    window()
        .unwrap_throw()
        .cancel_animation_frame(request_id)
        .unwrap_throw();
}
