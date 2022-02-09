use seed::{prelude::*, *};
use itertools::Itertools;
use std::collections::VecDeque;
use std::collections::HashSet;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let n_gram_size = 5;
    let mut set: HashSet<u32> = HashSet::new();
    set.insert(68);
    set.insert(70);
    Model {
        event_streams: Vec::new(),
        n_gram_size: n_gram_size,
        pressed_keys: SlidingWindow::new(n_gram_size),
        allowed_keycodes: set,
    }
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    allowed_keycodes: HashSet<u32>,
    n_gram_size: usize,
    pressed_keys: SlidingWindow<u32>,
    event_streams: Vec<StreamHandle>,
}

#[derive(Default)]
struct SlidingWindow<T> {
    window: VecDeque<T>,
    capacity: usize, 
}

impl<T> SlidingWindow<T> {
    fn new(capacity: usize) -> SlidingWindow<T> {
        SlidingWindow {
            capacity: capacity,
            window: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, element: T) -> Option<T> {
        self.window.push_back(element);
        if self.window.len() == self.capacity {
            self.window.pop_front()
        } else {
            None
        }   
    }

    fn last(&self) -> Option<&T> {
        self.window.back()
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ToggleWatching,
    KeyPressed(web_sys::KeyboardEvent),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleWatching => {
            if model.event_streams.is_empty() {
                model.event_streams = vec![
                    orders.stream_with_handle(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    })),
                ];
            } else {
                model.event_streams.clear();
            }
        }
        Msg::KeyPressed(ev) => {
            let key_code = ev.key_code();
            if model.allowed_keycodes.contains(&key_code) {
                model.pressed_keys.push(ev.key_code());
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h2![format!("Last combination pressed: {:?}", model.pressed_keys.last())],
        h2![format!("Last key pressed: {:?}", model.pressed_keys.last())],
        button![
            ev(Ev::Click, |_| Msg::ToggleWatching),
            if model.event_streams.is_empty() {
                "Start watching"
            } else {
                "Stop watching"
            }
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
