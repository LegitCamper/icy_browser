use iced::{widget::Column, Command, Element};
use url::Url;

use super::{browser_view, nav_bar, tab_bar, BrowserView, NavBar, TabBar};
use crate::{engines::BrowserEngine, to_url};

#[cfg(feature = "ultralight")]
use crate::engines::ultralight::Ultralight;

// #[derive(Debug)]
// enum View {
//     Loading,
//     Loaded,
// }

pub enum Action {
    None,
    // This is for downloading resources
    Init(Command<Message>),
}

#[derive(Debug, Clone)]
pub enum Message {
    TabBar(tab_bar::Message),
    NavBar(nav_bar::Message),
    BrowserView(browser_view::Message),
    DoWork,
    // New,
}

pub struct BrowserWidget<Engine: BrowserEngine> {
    engine: Option<Engine>,
    command: Command<Message>,
    // view: View,
    home: Url,
    tab_bar: Option<TabBar>,
    nav_bar: Option<NavBar>,
    browser_view: Option<BrowserView>,
}

impl<Engine: BrowserEngine> BrowserWidget<Engine> {
    const HOME: &'static str = "https://duckduckgo.com";

    pub fn new() -> Self {
        let home = Url::parse(Self::HOME).unwrap();
        Self {
            engine: Some(Engine::new()),
            // view: View::Loading,
            home,
            tab_bar: None,
            nav_bar: None,
            browser_view: None,
            command: Command::none(),
        }
    }

    pub fn with_homepage(mut self, homepage: &str) -> Self {
        self.home = Url::parse(homepage).unwrap();
        self
    }

    pub fn with_tab_bar(mut self) -> Self {
        self.tab_bar = Some(tab_bar());
        self
    }

    pub fn with_nav_bar(mut self) -> Self {
        self.nav_bar = Some(nav_bar());
        self
    }

    pub fn with_browsesr_view(mut self) -> Self {
        self.browser_view = Some(browser_view());
        self
    }

    pub fn build(self) -> (Self, Command<Message>) {
        (
            Self {
                engine: self.engine,
                // view: self.view,
                home: self.home,
                tab_bar: self.tab_bar,
                nav_bar: self.nav_bar,
                browser_view: self.browser_view,
                command: Command::none(),
            },
            self.command,
        )
    }

    /// Some stuff needs to be done regularly, so is done in a subscription
    pub fn do_work(&self) {
        if let Some(engine) = self.engine.as_ref() {
            engine.do_work();
        }
    }

    // fn init_command() -> Command<Message> {
    //     Command::future(async {
    //         // download zip and extract it.
    //         // copy resources and .so/.dll to home
    //         // let downloads = [
    //         //     ""
    //         // ]

    //         // // Return the joke as a message
    //         // Message::Initialize(joke.to_owned())
    //     })
    // }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        if let Some(engine) = self.engine.as_mut() {
            match message {
                Message::DoWork => engine.do_work(),
                Message::TabBar(msg) => {
                    if let Some(tab_bar) = self.tab_bar.as_mut() {
                        match tab_bar.update(msg) {
                            tab_bar::Message::TabSelected(index) => {
                                engine.goto_tab(index as u32).unwrap()
                            }
                            tab_bar::Message::TabClosed(index) => engine.close_tab(index as u32),
                            tab_bar::Message::NewTab => {
                                engine.new_tab(&Url::parse(self.home.as_str()).unwrap())
                            }
                        }
                    }
                }
                Message::NavBar(msg) => {
                    if let Some(nav_bar) = self.nav_bar.as_mut() {
                        match nav_bar.update(msg) {
                            nav_bar::Action::GoBackward => engine.go_back(),
                            nav_bar::Action::GoForward => engine.go_forward(),
                            nav_bar::Action::Refresh => engine.refresh(),
                            nav_bar::Action::GoHome => engine.goto_url(&self.home),
                            nav_bar::Action::GoUrl(url) => engine.goto_url(&to_url(&url).unwrap()),
                            nav_bar::Action::None => (),
                        }
                    }
                }
                Message::BrowserView(msg) => {
                    if let Some(browser_view) = self.browser_view.as_mut() {
                        match browser_view.update(msg) {
                            browser_view::Action::SendKeyboardEvent(event) => {
                                engine.handle_keyboard_event(event);
                            }
                            browser_view::Action::SendMouseEvent(point, event) => {
                                engine.handle_mouse_event(point, event);
                            }
                            // browser_view::Action::UpdateImage(bounds) => {
                            //     let (current_size, allowed_size) =
                            //         (engine.size(), bounds.size());
                            //     if current_size.0 != allowed_size.width as u32
                            //         || current_size.1 != allowed_size.height as u32
                            //     {
                            //         engine.resize(
                            //             allowed_size.width as u32,
                            //             allowed_size.height as u32,
                            //         );
                            //     }

                            //     let image_data = engine.pixel_buffer().unwrap();
                            //     let image = create_image(
                            //         image_data,
                            //         current_size.0,
                            //         current_size.1,
                            //         true,
                            //     );
                            // }
                            browser_view::Action::None => (),
                        }
                    }
                } // Message::New => ,
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let mut view = Vec::new();
        if let Some(tab_bar) = &self.tab_bar {
            view.push(tab_bar.view().map(Message::TabBar))
        }
        if let Some(nav_bar) = &self.nav_bar {
            view.push(nav_bar.view().map(Message::NavBar))
        }
        if let Some(browser_view) = &self.browser_view {
            view.push(browser_view.view().map(Message::BrowserView))
        }

        Column::from_vec(view).into()
    }
}

impl BrowserWidget<Ultralight> {
    #[cfg(feature = "ultralight")]
    pub fn new_ultralight() -> BrowserWidget<Ultralight> {
        let engine = Ultralight::new();
        let home = Url::parse(Self::HOME).unwrap();
        BrowserWidget {
            engine: Some(engine),
            // view: View::Loading,
            home,
            tab_bar: None,
            nav_bar: None,
            browser_view: None,
            command: Command::none(),
        }
    }
}
