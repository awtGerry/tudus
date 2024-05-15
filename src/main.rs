use iced::widget::{column, row, container, text_editor, text, button};
use iced::Application;
use iced::{Theme, Command};

fn main() -> iced::Result {
    TodoApp::run(iced::Settings::default())
}

#[allow(unused)]
struct TodoApp {
    content: iced::widget::text_editor::Content,
    error: Option<std::io::Error>,
}

#[derive(Debug, Clone)]
enum Todo {
    Edit(iced::widget::text_editor::Action),
    Register
}

impl Application for TodoApp {
    type Message = Todo; // Messages that can be sent to the app

    type Theme = Theme; // Custom theme (use default dark for now)
    type Executor = iced::executor::Default; // engine to run async tasks
    type Flags = (); // data passed to the app on startup

    fn new(_flags: Self::Flags) -> (Self, Command<Todo>) {
        (
            Self {
                content: text_editor::Content::new(),
                error: None,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Todo's App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Todo> {
        match message {
            Todo::Edit(action) => {
                self.content.perform(action);
                Command::none()
            }
            Todo::Register => {
                let todo = self.content.text();
                println!("Todo: {}", todo);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let todos_title = text("New todo".to_string());
        let input = text_editor(&self.content).on_action(Todo::Edit);
        let register_btn = button("create todo").on_press(Todo::Register);
        container(
            column![
                todos_title,
                row![input, register_btn]
            ],
        ).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
