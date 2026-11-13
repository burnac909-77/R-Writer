use iced::widget::{button, column, container, pick_list, row, text, text_input, Space};
use iced::{Alignment, Background, Color, Element, Length, Sandbox, Settings, Theme};

pub fn main() -> iced::Result {
    RWriterApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    FileOpened,
    FileSaved,
    NameChanged(String),
    PageSizeChanged(PageSize),
    FontChanged(FontChoice),
    SizeChanged(String),
    TextChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageSize {
    A4,
    Letter,
}

impl std::fmt::Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PageSize::A4 => "A4",
                PageSize::Letter => "Letter",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FontChoice {
    Ubuntu,
    Hack,
    LiberationSerif,
}

impl std::fmt::Display for FontChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FontChoice::Ubuntu => "Ubuntu Light",
                FontChoice::Hack => "Hack Mono",
                FontChoice::LiberationSerif => "Liberation Serif",
            }
        )
    }
}

struct RWriterApp {
    content: String,
    file_name: String,
    page_size: PageSize,
    font_choice: FontChoice,
    font_size: String,
}

impl Sandbox for RWriterApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: String::from("Start typing your document here..."),
            file_name: String::from("document"),
            page_size: PageSize::A4,
            font_choice: FontChoice::Ubuntu,
            font_size: String::from("14"),
        }
    }

    fn title(&self) -> String {
        format!("{} - R-Writer", self.file_name)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FileOpened => {}
            Message::FileSaved => {}
            Message::NameChanged(name) => self.file_name = name,
            Message::PageSizeChanged(size) => self.page_size = size,
            Message::FontChanged(font) => self.font_choice = font,
            Message::SizeChanged(sz) => self.font_size = sz,
            Message::TextChanged(txt) => self.content = txt,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let toolbar = row![
            button("Open...").on_press(Message::FileOpened),
            button("Save").on_press(Message::FileSaved),
            text("Name:").size(13),
            text_input("File name", &self.file_name)
                .on_input(Message::NameChanged)
                .width(130),
            Space::with_width(Length::Fixed(10.0)),
            pick_list(
                vec![PageSize::A4, PageSize::Letter],
                Some(self.page_size),
                Message::PageSizeChanged
            ),
            pick_list(
                vec![
                    FontChoice::Ubuntu,
                    FontChoice::Hack,
                    FontChoice::LiberationSerif
                ],
                Some(self.font_choice),
                Message::FontChanged
            ),
            text("Size:").size(13),
            text_input("14", &self.font_size)
                .on_input(Message::SizeChanged)
                .width(60),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let top_container = container(toolbar)
            .padding(10)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
                border: iced::Border {
                    color: Color::from_rgb8(212, 212, 208),
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            });

        let editor_sheet = container(text(""))
            .padding(48)
            .width(Length::Fixed(600.0))
            .height(Length::Fixed(800.0))
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                border: iced::Border {
                    radius: 2.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

        let canvas_area = container(editor_sheet)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgb8(106, 106, 106))),
                ..Default::default()
            });

        let word_count = self.content.split_whitespace().count();
        let status_bar = container(text(format!(
            "Page 1 of 1 · {} words · Ready",
            word_count
        )))
        .padding(8)
        .width(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
            text_color: Some(Color::from_rgb8(95, 95, 95)),
            border: iced::Border {
                color: Color::from_rgb8(212, 212, 208),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        });

        column![top_container, canvas_area, status_bar].into()
    }
}
