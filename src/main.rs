mod tudus;
mod db;

use tudus::Tudu;

use iced::widget::{
    Column,
    column,
    row,
    scrollable,
    radio,
    container,
    TextInput,
    text,
    button,
    horizontal_space,
    tooltip,
};

use iced::theme::Button;
use iced::theme;
use iced::window;
use iced::Application;
use iced::{Theme, Command, Settings, Element, Font};

/* TODO:
 * Stop making the same queries multiple times it's inefficient
 * Add due date and reminder
 * Add a settings page (maybe)
*/

fn main() -> iced::Result {
    TudusApp::run(Settings {
        window: window::Settings {
            size: iced::Size {
                width: 1000.0,
                height: 600.0,
            },
            resizable: false,
            ..window::Settings::default()
        },
        fonts: vec![include_bytes!("../fonts/icon-font.ttf")
            .as_slice()
            .into()
        ],
        ..Settings::default()
    })
}

struct TudusApp {
    tudu_input: String,
    tudus_list: Vec<Tudu>,
    state_show: TudusShow,
    theme: u8,
}

#[allow(unused)]
#[derive(Debug, Clone)]
enum TudusShow {
    All,
    Active,
    Completed,
}

#[derive(Debug, Clone)]
enum App {
    InputChanged(String),
    CheckTudu(i64),
    UncheckTudu(i64),
    ChangeTudusShow(TudusShow),
    Calendar,
    Reminder,
    New,
    ThemeSwitcher,
}

impl Application for TudusApp {
    type Message = App; // Messages that can be sent to the app

    type Theme = Theme; // Custom theme (use default dark for now)
    type Executor = iced::executor::Default; // engine to run async tasks
    type Flags = (); // data passed to the app on startup

    fn new(_flags: Self::Flags) -> (Self, Command<App>) {
        // for now we will read the theme from a file, later we will use a settings file or geting it from the db
        let theme = std::fs::read_to_string("theme.txt").unwrap_or("1".to_string());
        (
            Self {
                // content: text_editor::Content::new(),
                tudu_input: String::new(),
                tudus_list: Tudu::get_all(),
                state_show: TudusShow::All,
                theme: theme.parse().unwrap(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Tudus")
    }

    fn update(&mut self, message: Self::Message) -> Command<App> {
        match message {
            App::InputChanged(input) => {
                self.tudu_input = input;
                Command::none()
            }
            App::Calendar => {
                Command::none()
            }
            App::Reminder => {
                Command::none()
            }
            App::New => {
                let new_tudu = Tudu::new(self.tudu_input.clone().to_string(), "".to_string());
                new_tudu.save();
                println!("Tudu saved");
                self.tudus_list = Tudu::get_all();
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
            App::CheckTudu(id) => {
                Tudu::complete_tudu(id);
                self.tudus_list = Tudu::get_all();
                Command::none()
            }
            App::UncheckTudu(id) => {
                Tudu::uncomplete_tudu(id);
                self.tudus_list = Tudu::get_all();
                Command::none()
            }
            App::ChangeTudusShow(state) => {
                self.state_show = state;
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
            let input = TextInput::new("Add a new todo", &self.tudu_input)
                .on_input(App::InputChanged)
                .padding(10);
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

        let toggle_tudus = {
            row![
                button("All").on_press(App::ChangeTudusShow(TudusShow::All)),
                button("Active").on_press(App::ChangeTudusShow(TudusShow::Active)),
                button("Completed").on_press(App::ChangeTudusShow(TudusShow::Completed)),
            ].spacing(20)
        };

        let spc1 = {
            column![
                new_todo,
                toggle_tudus
            ].spacing(20)
        };

        /* Check TudusShow and filter the list of tudus */
        #[allow(unused)]
        let mut tudus: Vec<Element<'_, App>> = Vec::new();
        match self.state_show {
            TudusShow::All => {
                tudus = self.tudus_list
                    .iter()
                    .map(|tudu| {
                        let id = match &tudu.id {
                            Some(id) => id,
                            None => panic!("No id found"),
                        };
                        if !tudu.completed {
                            row![
                                radio(
                                    "",
                                    *id,
                                    None,
                                    App::CheckTudu
                                ),
                                text(&tudu.name).size(20)
                            ]
                            .spacing(10)
                            .into()
                        } else {
                            row![
                                radio(
                                    "",
                                    *id,
                                    Some(*id),
                                    App::UncheckTudu
                                ),
                                text(&tudu.name).size(20)
                            ]
                            .spacing(10)
                            .into()
                        }
                    })
                    .collect();
            }
            TudusShow::Active => {
                tudus = self.tudus_list
                    .iter()
                    .filter(|tudu| !tudu.completed)
                    .map(|tudu| {
                        let id = match &tudu.id {
                            Some(id) => id,
                            None => panic!("No id found"),
                        };
                        row![
                            radio(
                                "",
                                *id,
                                None,
                                App::CheckTudu
                            ),

                            text(&tudu.name).size(20)
                        ]
                        .spacing(10)
                        .into()
                    })
                    .collect();
            }
            TudusShow::Completed => {
                tudus = self.tudus_list
                    .iter()
                    .filter(|tudu| tudu.completed)
                    .map(|tudu| {
                        let id = match &tudu.id {
                            Some(id) => id,
                            None => panic!("No id found"),
                        };
                        row![
                            radio(
                                "",
                                *id,
                                Some(*id),
                                App::UncheckTudu
                            ),
                            text(&tudu.name).size(20)
                        ]
                        .spacing(10)
                        .into()
                    })
                    .collect();
            }
        };

        let tudus = if tudus.is_empty() {
            scrollable(
                text("No tudus yet, YAY!")
            )
        } else {
            let tudus = Column::with_children(tudus)
                .spacing(20)
                .width(iced::Length::Fill);

            scrollable(
                column![
                    tudus
                ]
            )
        };

        /* TODO: Investigate if there is a better way to do the y+ spacing*/
        /* Space between new todo and todo */
        let spc2 = {
            column![
                spc1,
                tudus
            ].spacing(30)
        };

        container(
            column![
                header,
                spc2
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

fn theme_icon<'a>(theme: u8) -> Element<'a, App> {
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
