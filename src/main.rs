use iced::widget::{button, column, row, text, text_editor, Container, Space};
use iced::{Element, Length, Task};
use rfd::FileDialog;

pub fn main() -> iced::Result {
    iced::application("R-Writer (Pure Rust)", RWriterApp::update, RWriterApp::view)
        .run()
}

struct RWriterApp {
    content: text_editor::Content,
    file_path: Option<String>,
    status_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    OpenClicked,
    SaveClicked,
}

impl Default for RWriterApp {
    fn default() -> Self {
        Self {
            content: text_editor::Content::new(),
            file_path: None,
            status_message: "Ready - Pure Rust Native Mode".into(),
        }
    }
}

impl RWriterApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            }
            Message::OpenClicked => {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        self.file_path = Some(path.to_string_lossy().to_string());
                        self.content = text_editor::Content::with_text(&contents);
                        self.status_message = format!("Loaded {}", path.to_string_lossy());
                    }
                }
                Task::none()
            }
            Message::SaveClicked => {
                if let Some(path) = &self.file_path {
                    let text = self.content.text();
                    let _ = std::fs::write(path, text);
                    self.status_message = format!("Saved to {}", path);
                } else {
                    if let Some(path) = FileDialog::new().save_file() {
                        let path_str = path.to_string_lossy().to_string();
                        let text = self.content.text();
                        let _ = std::fs::write(&path_str, text);
                        self.file_path = Some(path_str.clone());
                        self.status_message = format!("Saved to {}", path_str);
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let open_btn = button("Open File").on_press(Message::OpenClicked);
        let save_btn = button("Save File").on_press(Message::SaveClicked);
        
        let toolbar = row![open_btn, save_btn, Space::with_width(Length::Fill)]
            .spacing(10)
            .padding(10);

        let editor = text_editor(&self.content)
            .on_action(Message::Edit)
            .padding(20);

        let status = text(&self.status_message).size(14);
        let status_bar = Container::new(status).padding(8);

        let layout = column![toolbar, editor, status_bar]
            .spacing(5)
            .height(Length::Fill);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
