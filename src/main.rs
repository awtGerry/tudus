use iced::widget::{
    column,
    row,
    container,
    text_editor,
    text,
    button,
    horizontal_space,
};
use iced::theme::Button;
use iced::Application;
use iced::{Theme, Command, Settings, Element, Font};

fn main() -> iced::Result {
    TodoApp::run(Settings {
        fonts: vec![include_bytes!("../fonts/icon-font.ttf")
            .as_slice()
            .into()],
        ..Settings::default()
    })
}

struct TodoApp {
    content: iced::widget::text_editor::Content,
}

#[derive(Debug, Clone)]
enum Todo {
    Edit(iced::widget::text_editor::Action),
    Calendar,
    Reminder,
    New
}

impl Application for TodoApp {
    type Message = Todo; // Messages that can be sent to the app

    type Theme = Theme; // Custom theme (use default dark for now)
    type Executor = iced::executor::Default; // engine to run async tasks
    type Flags = (); // data passed to the app on startup

    fn new(_flags: Self::Flags) -> (Self, Command<Todo>) {
        (
            Self {
                content: text_editor::Content::new()
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
            Todo::Calendar => {
                Command::none()
            }
            Todo::Reminder => {
                Command::none()
            }
            Todo::New => {
                let todo = self.content.text();
                println!("Todo: {}", todo);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let todos_title = text("New todo".to_string()).size(25).height(50);

        let new_todo = {
            let input = text_editor(&self.content).on_action(Todo::Edit).padding(10);
            column![
                input,
                row![
                    button(calendar_icon()).on_press(Todo::Calendar)
                        .padding([6, 12])
                        .style(Button::Text),
                    button(reminder_icon()).on_press(Todo::Reminder)
                        .padding([6, 12])
                        .style(Button::Text),
                    horizontal_space(),
                    button("Add").on_press(Todo::New).padding([6, 12]),
                ].spacing(20)
            ].spacing(10)
        };

        container(
            column![
                todos_title,
                new_todo,
            ],
        ).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightLight
    }
}

fn calendar_icon<'a>() -> Element<'a, Todo> {
    icon('\u{f133}')
}

fn reminder_icon<'a>() -> Element<'a, Todo> {
    icon('\u{e802}')
}

fn icon<'a>(codepoint: char) -> Element<'a, Todo> {
    const ICON_FONT: Font = Font::with_name("icon-font");
    text(codepoint).font(ICON_FONT).into()
}
