use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};

use iced::futures::SinkExt;
use iced::widget::{container, row, text};
use iced::{executor, Application, Command, Element, Subscription, Theme};

fn main() -> iced::Result {
    Example::run(iced::Settings::default())
}

struct Example {
    last_pressed: String,
    // store the global manager otherwise it will be dropped and events will not be emitted
    _manager: GlobalHotKeyManager,
}

#[derive(Debug, Clone)]
enum ProgramCommands {
    // message received when the subscription calls back to the main gui thread
    Received(String),
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = ProgramCommands;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Example, iced::Command<Self::Message>) {
        let manager = GlobalHotKeyManager::new().unwrap();
        let hotkey_1 = HotKey::new(Some(Modifiers::CONTROL), Code::ArrowRight);
        let hotkey_2 = HotKey::new(None, Code::ArrowUp);

        manager.register(hotkey_1).unwrap();
        manager.register(hotkey_2).unwrap();
        (
            Example {
                last_pressed: "".to_string(),
                _manager: manager,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Iced example!")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark // dark theme :D
    }
    fn update(&mut self, msg: Self::Message) -> iced::Command<ProgramCommands> {
        match msg {
            Self::Message::Received(code) => {
                // update the text widget
                self.last_pressed = code.to_string();

                Command::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        container(row![text("You pressed: "), text(self.last_pressed.clone())]).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.hotkey_sub()
    }
}

impl Example {
    pub fn hotkey_sub(&self) -> Subscription<ProgramCommands> {
        iced::subscription::channel(0, 32, |mut sender| async move {
            let receiver = GlobalHotKeyEvent::receiver();
            // poll for global hotkey events every 50ms
            loop {
                if let Ok(event) = receiver.try_recv() {
                    sender
                        .send(ProgramCommands::Received(format!("{:?}", event)))
                        .await
                        .unwrap();
                }
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }
}
