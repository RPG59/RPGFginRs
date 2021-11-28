use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

trait ISubscriber {
    fn exec(&self);
}

#[derive(PartialEq, Eq, Hash)]
enum EventType {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
    MouseMove,
}

struct MouseEvent {
    eventType: EventType,
    x: i32,
    y: i32,
}

struct KeyboardEvent {
    eventType: EventType,
    key: String,
}

trait Event {
    fn getEventType(&self) -> EventType;
}

impl Event for MouseEvent {
    fn getEventType(&self) -> EventType {
        self.eventType
    }
}

impl Event for KeyboardEvent {
    fn getEventType(&self) -> EventType {
        self.eventType
    }
}

// struct Ref<'a, T> {
//     field: &'a T,
// }

struct Ref {
    data: std::cell::Cell<f32>,
}

#[derive(Default)]
struct EventBus {
    // subscribers: std::collections::HashMap<EventType, Vec<dyn ISubscriber>>,
    test: u32,
}

impl EventBus {
    pub fn init(&self) {
        // let (sender, receiver) = futures::sync::mpsc::unbounded();
        // let foo = Ref {
        //     data: std::cell::Cell::new(12.0),
        // };

        // println!("{}", foo.data.get());

        // let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        //     self.publish(&MouseEvent {
        //         eventType: EventType::MouseDown,
        //         x: event.offset_x(),
        //         y: event.offset_y(),
        //     });
        // }) as Box<dyn FnMut(_)>);

        // let window = web_sys::window().unwrap();
        // window.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
    }

    // pub fn subscribe(&mut self, eventType: EventType, subscriber: &T) {
    //     if let Some(handlerList) = self.subscribers.get(&eventType) {
    //         handlerList.push(subscriber);
    //     } else {
    //         self.subscribers.insert(eventType, vec![subscriber]);
    //     }
    // }

    // pub fn publish(&self, event: &Event) {
    //     if let Some(handlerList) = self.subscribers.get(&event.getEventType()) {
    //         for handler in handlerList.iter() {
    //             handler.exec();
    //         }
    //     }
    // }

    // pub fn detach(&mut self, subscriber: &'a T) {
    //   let index = self.subscribers.iter().position(|x| *x == subscriber );
    // }
}
