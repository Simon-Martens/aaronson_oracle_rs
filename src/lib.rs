use keycodes;
use rand::Rng;
use seed::{prelude::*, *};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut set: HashSet<u32> = HashSet::new();
    set.insert(68);
    set.insert(70);
    Model {
        event_streams: Vec::new(),
        pressed_keys: SlidingWindow::new(5),
        allowed_keycodes: set,
        guesses: Guesses::new(100),
    }
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    allowed_keycodes: HashSet<u32>,
    pressed_keys: SlidingWindow,
    event_streams: Vec<StreamHandle>,
    guesses: Guesses,
}

#[derive(Default)]
struct Guesses {
    window: VecDeque<bool>,
    presses: Vec<u32>,
    guesses: Vec<u32>,
    window_size: usize,
    right_guesses: u32,
    wrong_guesses: u32,
    press_cnt: u32,
}

impl Guesses {
    fn new(window_size: usize) -> Guesses {
        Guesses {
            window: VecDeque::with_capacity(window_size),
            window_size: window_size,
            guesses: Vec::new(),
            presses: Vec::new(),
            right_guesses: 0,
            wrong_guesses: 0,
            press_cnt: 0,
        }
    }

    fn push_press(&mut self, press: u32) {
        self.presses.push(press);
        let right;
        if press == *self.guesses.last().unwrap() {
            self.right_guesses += 1;
            right = true;
        } else {
            self.wrong_guesses += 1;
            right = false;
        }
        self.window.push_back(right);
        if self.window.len() > self.window_size {
            if self.window.pop_front().unwrap() {
                self.right_guesses -= 1;
            } else {
                self.wrong_guesses -= 1;
            }
        }
        self.press_cnt += 1;
    }

    fn push_guess(&mut self, guess: u32) {
        self.guesses.push(guess);
    }

    fn get_percentage(&self) -> f64 {
        let g: f64 = (self.right_guesses + self.wrong_guesses) as f64 / 100.0;
        let mut p = 0f64;
        if g > 0.0 {
            self.right_guesses as f64 / g
        } else {
            g
        }
    }

    fn get_guess(&self) -> u32 {
        *self.guesses.last().unwrap_or(&0)
    }
}

#[derive(Default)]
struct SlidingWindow {
    window: VecDeque<u32>,
    n_gram_size: usize,
    ngrams: HashMap<String, u32>,
}

impl SlidingWindow {
    fn new(n_gram_size: usize) -> SlidingWindow {
        SlidingWindow {
            n_gram_size: n_gram_size,
            window: VecDeque::with_capacity(n_gram_size),
            ngrams: Default::default(),
        }
    }

    fn push(&mut self, element: u32) {
        self.window.push_back(element);
        if self.window.len() > self.n_gram_size {
            self.window.pop_front();
        }
        if self.window.len() == self.n_gram_size {
            let id = self.get_id();
            match self.ngrams.get_mut(&id) {
                Some(n) => *n += 1,
                None => {
                    self.ngrams.insert(id, 1);
                }
            }
        }
    }

    fn guess(&self, values: &HashSet<u32>) -> u32 {
        let id4 = self.get_id_less_one();
        let mut winner = (0u32, 0u32);
        values.iter().for_each(|x| {
            let newstring = id4.clone() + &(x.to_string());
            if let Some(n) = self.ngrams.get(&newstring) {
                if *n > winner.1 {
                    winner.0 = *x;
                    winner.1 = *n;
                }
            } else {
                if winner.1 == 0 {
                    winner.0 = *x;
                }
            }
        });
        winner.0
    }

    fn last(&self) -> Option<&u32> {
        self.window.back()
    }

    fn get_id(&self) -> String {
        let mut string = String::new();
        self.window.iter().for_each(|x| string += &(x.to_string()));
        string
    }

    fn get_id_less_one(&self) -> String {
        let mut string = String::new();
        self.window
            .iter()
            .skip(1)
            .for_each(|x| string += &(x.to_string()));
        string
    }

    fn get_guesses(&self) -> &HashMap<String, u32> {
        &self.ngrams
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
                model.event_streams = vec![orders
                    .stream_with_handle(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    }))];
                    model.guesses.push_guess((model.pressed_keys.guess(&model.allowed_keycodes)));
            } else {
                model.event_streams.clear();
            }
        }
        Msg::KeyPressed(ev) => {
            let key_code = ev.key_code();
            if model.allowed_keycodes.contains(&key_code) {
                model.guesses.push_press(key_code);
                model.pressed_keys.push(key_code);
            }
            model.guesses.push_guess((model.pressed_keys.guess(&model.allowed_keycodes)));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        // p![format!(
        //     "Combinations pressed: {:?}",
        //     model.pressed_keys.get_guesses()
        // )],
        // p![format!("Guess: {:?}", model.guesses.get_guess())],
        // p![format!("Last key pressed: {:?}", model.pressed_keys.last())],
        h2![format!("Right guesses: {:?}%", model.guesses.get_percentage())],
        button![
            C!["btn btn-primary mb-3 mt-2"],
            ev(Ev::Click, |_| Msg::ToggleWatching),
            if model.event_streams.is_empty() {
                "Start"
            } else {
                "Stop"
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
