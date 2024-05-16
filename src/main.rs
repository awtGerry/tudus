use iced::widget::{
    column,
    row,
    container,
    text_editor,
    text,
    button,
    horizontal_space,
    tooltip
};
use iced::theme::Button;
use iced::theme;
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
    theme: usize, // 1 for light, 0 for dark
}

#[derive(Debug, Clone)]
enum App {
    Edit(iced::widget::text_editor::Action),
    Calendar,
    Reminder,
    New,
    ThemeSwitcher,
}

impl Application for TodoApp {
    type Message = App; // Messages that can be sent to the app

    type Theme = Theme; // Custom theme (use default dark for now)
    type Executor = iced::executor::Default; // engine to run async tasks
    type Flags = (); // data passed to the app on startup

    fn new(_flags: Self::Flags) -> (Self, Command<App>) {
        let theme = std::fs::read_to_string("theme.txt").unwrap_or("1".to_string());
        (
            Self {
                content: text_editor::Content::new(),
                theme: theme.parse().unwrap(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Todo's App")
    }

    fn update(&mut self, message: Self::Message) -> Command<App> {
        match message {
            App::Edit(action) => {
                self.content.perform(action);
                Command::none()
            }
            App::Calendar => {
                Command::none()
            }
            App::Reminder => {
                Command::none()
            }
            App::New => {
                let todo = self.content.text();
                println!("Todo: {}", todo);
                Command::none()
            }
            App::ThemeSwitcher => {
                if self.theme == 1 {
                    std::fs::write("theme.txt", "0").unwrap();
                    self.theme = 0;
                } else {
                    std::fs::write("theme.txt", "1").unwrap();
                    self.theme = 1;
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header = {
            let new_todo_title = text("Tudus").size(25).height(50);
            row![
                new_todo_title,
                horizontal_space(),
                tooltip(
                    button(theme_icon(self.theme))
                        .on_press(App::ThemeSwitcher)
                        .style(Button::Text) // TODO: change font color
                        .padding([6, 12]),
                    "Switch theme",
                    tooltip::Position::Left
                ).style(theme::Container::Box),
            ].spacing(20)
        };

        let new_todo = {
            let input = text_editor(&self.content).on_action(App::Edit).padding(10);
            column![
                input,
                row![
                    tooltip(
                        button(calendar_icon())
                            .on_press(App::Calendar)
                            .padding([6, 12])
                            .style(Button::Text),
                        "Set due date",
                        tooltip::Position::Bottom
                    ).style(theme::Container::Box),
                    tooltip(
                        button(reminder_icon()).on_press(App::Reminder)
                            .padding([6, 12])
                            .style(Button::Text),
                        "Remind me",
                        tooltip::Position::Bottom
                    ).style(theme::Container::Box),
                    horizontal_space(),
                    button("Add").on_press(App::New).padding([6, 12]),
                ].spacing(20)
            ].spacing(10)
        };

        container(
            column![
                header,
                new_todo,
            ],
        ).padding(20).into()
    }

    fn theme(&self) -> Theme {
        let theme = if self.theme == 1 {
            Theme::TokyoNightLight
        } else {
            Theme::TokyoNight
        };
        theme
    }
}

fn calendar_icon<'a>() -> Element<'a, App> {
    icon('\u{f133}')
}

fn reminder_icon<'a>() -> Element<'a, App> {
    icon('\u{e802}')
}

fn theme_icon<'a>(theme: usize) -> Element<'a, App> {
    if theme == 1 {
        icon('\u{E803}') // sun
    } else {
        icon('\u{E804}') // moon
    }
}

fn icon<'a>(codepoint: char) -> Element<'a, App> {
    const ICON_FONT: Font = Font::with_name("icon-font");
    text(codepoint).font(ICON_FONT).into()
}
