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
    // since 0.4.0, this needs to be inside the struct. for version before, it does it.
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
        manager.register(hotkey_2).unwrap();
        manager.register(hotkey_1).unwrap();
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
        // if you want to have multiple subscriptions, do it like this:
        // iced::Subscription::batch(vec![self.hotkey_sub(), self.test_sub()])
        self.hotkey_sub()
    }
}

impl Example {
    pub fn hotkey_sub(&self) -> Subscription<ProgramCommands> {
        iced::subscription::channel(0, 32, |mut sender| async move {
            let receiver = GlobalHotKeyEvent::receiver();
            loop {
                match receiver.try_recv() {
                    Err(_e) => {
                        // nothing, this happens when nothing happens
                    }
                    Ok(t) => {
                        // if you want to differenciate between the different codes, you can match them:
                        // match t {
                        //     GlobalHotKeyEvent { id: 267073199 } => { // this is for up arrow, no modifiers
                        //         do whatever
                        //     }
                        // }
                        sender
                            .send(ProgramCommands::Received(format!("{:?}", t)))
                            .await
                            .unwrap()
                    }
                }
                // so this will occur infinitely, i would really recommend using the `async_std` crate to put a sleep here
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }
}
