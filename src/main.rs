use iced::widget::{column, text, button};
use iced::{Sandbox, Settings};
use iced::Element;

use std::env;

fn main() -> iced::Result {
    env::set_var("RUST_BACKTRACE", "1");
    Test::run(Settings::default())
}

struct Test {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Test {
    type Message = Message;
    fn new() -> Test {
        Test { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Test")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = column![
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
            text(self.value.to_string())
        ].into();

        content
    }
}
