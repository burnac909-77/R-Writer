use iced::widget::{button, column, container, pick_list, row, text, text_input, text_editor, Space, scrollable, stack, Column};
use iced::{Alignment, Background, Color, Element, Length, Theme, window};

pub fn main() -> iced::Result {
    // Load custom app logo icon for Linux window & taskbar at runtime
    let icon = window::icon::from_file_data(
        include_bytes!("../icon.png"),
        Some(image::ImageFormat::Png),
    ).ok();

    iced::application("R-Writer", RWriterApp::update, RWriterApp::view)
        .window(window::Settings {
            icon,
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    OpenClicked,
    SaveClicked,
    CloseSaveDialog,
    ConfirmSave,
    FormatSelected(SaveFormat),
    NameChanged(String),
    PageSizeChanged(PageSize),
    FontChanged(FontChoice),
    SizeChanged(String),
    PageAction(usize, text_editor::Action),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SaveFormat {
    Docx,
    Odt,
    Txt,
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
    UbuntuLight,
    HackMono,
    LiberationSerif,
    LiberationSans,
    LiberationMono,
    Carlito,
    Caladea,
    DejaVuSans,
    DejaVuSerif,
}

impl std::fmt::Display for FontChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FontChoice::UbuntuLight => "Ubuntu Light (default sans)",
                FontChoice::HackMono => "Hack (default mono)",
                FontChoice::LiberationSerif => "Liberation Serif (Times-like)",
                FontChoice::LiberationSans => "Liberation Sans (Arial-like)",
                FontChoice::LiberationMono => "Liberation Mono (Courier-like)",
                FontChoice::Carlito => "Carlito (Calibri-like)",
                FontChoice::Caladea => "Caladea (Cambria-like)",
                FontChoice::DejaVuSans => "DejaVu Sans",
                FontChoice::DejaVuSerif => "DejaVu Serif",
            }
        )
    }
}

struct RWriterApp {
    pages: Vec<text_editor::Content>,
    file_name: String,
    page_size: PageSize,
    current_font: FontChoice,
    current_size_str: String,
    current_size: u16,
    show_save_dialog: bool,
    save_format: SaveFormat,
}

impl Default for RWriterApp {
    fn default() -> Self {
        Self {
            pages: vec![text_editor::Content::new()], // Starts completely empty with no placeholder text
            file_name: String::from("document"),
            page_size: PageSize::A4,
            current_font: FontChoice::UbuntuLight,
            current_size_str: String::from("14"),
            current_size: 14,
            show_save_dialog: false,
            save_format: SaveFormat::Docx,
        }
    }
}

impl RWriterApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpenClicked => {}
            Message::SaveClicked => {
                self.show_save_dialog = true;
            }
            Message::CloseSaveDialog => {
                self.show_save_dialog = false;
            }
            Message::ConfirmSave => {
                if let Some(doc_dir) = dirs::document_dir() {
                    let extension = match self.save_format {
                        SaveFormat::Docx => "docx",
                        SaveFormat::Odt => "odt",
                        SaveFormat::Txt => "txt",
                    };
                    let file_name = format!("{}.{}", self.file_name.trim(), extension);
                    let full_path = doc_dir.join(file_name);

                    let full_text: String = self.pages
                        .iter()
                        .map(|p| p.text())
                        .collect::<Vec<String>>()
                        .join("\n");

                    let _ = std::fs::write(&full_path, full_text);
                }
                self.show_save_dialog = false;
            }
            Message::FormatSelected(fmt) => {
                self.save_format = fmt;
            }
            Message::NameChanged(name) => self.file_name = name,
            Message::PageSizeChanged(size) => self.page_size = size,
            Message::FontChanged(font) => {
                self.current_font = font;
            }
            Message::SizeChanged(sz) => {
                self.current_size_str = sz.clone();
                if let Ok(parsed) = sz.parse::<u16>() {
                    self.current_size = parsed.clamp(8, 48);
                }
            }
            Message::PageAction(idx, action) => {
                // Enter creates a new line on the active page instead of spawning a new physical page
                if let Some(content) = self.pages.get_mut(idx) {
                    content.perform(action);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        fn red_button_style(_theme: &Theme, status: button::Status) -> button::Style {
            let bg = match status {
                button::Status::Hovered => Color::from_rgb8(210, 50, 50),
                _ => Color::from_rgb8(180, 40, 40),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: Color::WHITE,
                border: iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }

        let blank_editor_style = |_theme: &Theme, _status: text_editor::Status| {
            text_editor::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: Color::from_rgb8(150, 150, 150),
                value: Color::from_rgb8(17, 17, 17),
                selection: Color::from_rgb8(180, 200, 255),
            }
        };

        // 1. TOP TOOLBAR
        let toolbar = row![
            button("Open...").on_press(Message::OpenClicked).style(red_button_style),
            button("Save").on_press(Message::SaveClicked).style(red_button_style),
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
                    FontChoice::UbuntuLight,
                    FontChoice::HackMono,
                    FontChoice::LiberationSerif,
                    FontChoice::LiberationSans,
                    FontChoice::LiberationMono,
                    FontChoice::Carlito,
                    FontChoice::Caladea,
                    FontChoice::DejaVuSans,
                    FontChoice::DejaVuSerif,
                ],
                Some(self.current_font),
                Message::FontChanged
            ),
            text("Size:").size(13),
            text_input("14", &self.current_size_str)
                .on_input(Message::SizeChanged)
                .width(60),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let top_container = container(toolbar)
            .padding(10)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
                border: iced::Border {
                    color: Color::from_rgb8(212, 212, 208),
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            });

        // 2. MULTI-PAGE WORKSPACE CONTAINER
        let mut workspace_col = Column::new().spacing(25).align_x(Alignment::Center);
        let total_pages = self.pages.len();

        for (i, page_content) in self.pages.iter().enumerate() {
            let editor = text_editor(page_content)
                .on_action(move |action| Message::PageAction(i, action))
                .size(self.current_size)
                .style(blank_editor_style);

            let sheet_inner = container(editor)
                .padding(iced::Padding {
                    top: 40.0,
                    right: 40.0,
                    bottom: 64.0,
                    left: 10.0, // 1-character left margin spacing
                })
                .width(Length::Fixed(650.0))
                .height(Length::Fixed(800.0));

            let editor_sheet = container(sheet_inner)
                .width(Length::Fixed(650.0))
                .height(Length::Fixed(800.0))
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::WHITE)),
                    border: iced::Border {
                        radius: 2.0.into(),
                        color: Color::from_rgb8(200, 200, 200),
                        width: 1.0,
                    },
                    ..Default::default()
                });

            workspace_col = workspace_col.push(editor_sheet);
        }

        // Clean gap under the toolbar before the first page
        let padded_workspace = column![
            Space::with_height(Length::Fixed(20.0)),
            workspace_col,
            Space::with_height(Length::Fixed(30.0)),
        ].align_x(Alignment::Center);

        let canvas_area = container(scrollable(padded_workspace))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(106, 106, 106))),
                ..Default::default()
            });

        // 3. STATUS BAR
        let full_doc_text: String = self.pages.iter().map(|p| p.text()).collect::<Vec<String>>().join("\n");
        let total_chars = full_doc_text.chars().count();
        let total_words = full_doc_text.split_whitespace().count();

        let status_bar = container(text(format!(
            "Page 1 of {} · {} characters · {} words · Ready",
            total_pages, total_chars, total_words
        )))
        .padding(8)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
            text_color: Some(Color::from_rgb8(95, 95, 95)),
            border: iced::Border {
                color: Color::from_rgb8(212, 212, 208),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        });

        let main_app = column![top_container, canvas_area, status_bar];

        // 4. SAVE DIALOG MODAL OVERLAY
        if self.show_save_dialog {
            let dialog_box = container(
                column![
                    text("Save document").size(18),
                    Space::with_height(Length::Fixed(10.0)),
                    button(text(if self.save_format == SaveFormat::Docx { "● Word document (.docx)" } else { "○ Word document (.docx)" }))
                        .on_press(Message::FormatSelected(SaveFormat::Docx))
                        .style(red_button_style),
                    button(text(if self.save_format == SaveFormat::Odt { "● OpenDocument text (.odt)" } else { "○ OpenDocument text (.odt)" }))
                        .on_press(Message::FormatSelected(SaveFormat::Odt))
                        .style(red_button_style),
                    button(text(if self.save_format == SaveFormat::Txt { "● Plain Text (.txt)" } else { "○ Plain Text (.txt)" }))
                        .on_press(Message::FormatSelected(SaveFormat::Txt))
                        .style(red_button_style),
                    Space::with_height(Length::Fixed(15.0)),
                    row![
                        button("Cancel").on_press(Message::CloseSaveDialog),
                        button("Save").on_press(Message::ConfirmSave).style(red_button_style),
                    ].spacing(10)
                ].padding(20)
            )
            .width(Length::Fixed(320.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
                border: iced::Border {
                    radius: 12.0.into(),
                    color: Color::from_rgb8(212, 212, 208),
                    width: 1.0,
                },
                ..Default::default()
            });

            let modal_overlay = container(dialog_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.45))),
                    ..Default::default()
                });

            stack![main_app, modal_overlay].into()
        } else {
            main_app.into()
        }
    }
}
