use iced::widget::{container, text_editor};
use iced::Sandbox;

fn main() -> iced::Result {
    TodoApp::run(iced::Settings::default())
}

struct TodoApp {
    content: iced::widget::text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(iced::widget::text_editor::Action),
}

impl Sandbox for TodoApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Todo's App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action)
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let input = text_editor(&self.content).on_action(Message::Edit);
        container(input).padding(10).into()
    }
}
