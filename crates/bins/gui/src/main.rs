use iced::{Sandbox, Element, Settings, button, Column, Align, Button, Text};

fn main() {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment: button::State,
    decrement: button::State
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => { self.value += 1; },
            Message::Decrement => { self.value -= 1; },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Button::new(&mut self.increment, Text::new("Increment"))
                .on_press(Message::Increment))
            .push(Button::new(&mut self.decrement, Text::new("Decrement"))
                .on_press(Message::Decrement))
            .into()
    }
}
