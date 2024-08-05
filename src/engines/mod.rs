use iced::keyboard;
use iced::mouse;
use iced::widget::image::{Handle, Image};
use iced::{event::Status, Point};
use std::sync::mpsc;
use std::thread;

#[cfg(feature = "webkit")]
pub mod ultralight;

#[allow(unused)]
pub trait BrowserEngine {
    fn new(width: u32, height: u32) -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn render(&mut self);

mod ultralight;

/// Commands to control browser engines
pub enum Commands {
    DoWork,
    Render,
    NeedRender,
    Size,
    Resize(u32, u32),
    PixelBuffer,
    Url,
    GotoUrl(String),
    HasLoaded,
    NewTab(String),
    GotoTab(String),
    Refresh,
    GoForward,
    GoBackward,
    Focus,
    Unfocus,
    Scroll(mouse::ScrollDelta),
    Keyboard(keyboard::Event),
    Mouse(Point, mouse::Event),
}

/// The response to Commands
pub enum CommandsRecv {
    Size(u32, u32),
    NeedRender(bool),
    PixelBuffer(Vec<u8>),
    UrlRecv(String),
    HasLoaded(bool),
    Scroll(Status),
    Keyboard(Status),
    Mouse(Status),
}

/// Holds sender and receiver channels for controlling browser engines
pub struct Engine {
    sender: mpsc::Sender<Commands>,
    receiver: mpsc::Receiver<CommandsRecv>,
}

impl Engine {
    pub fn new() -> Self {
        let (send_commands, read_commands) = mpsc::channel::<Commands>();
        let (send_response, read_response) = mpsc::channel::<CommandsRecv>();
        // Spawns new thread for browser engines to operate in
        // this frees the main thread for ui actions
        thread::spawn(move || {
            #[cfg(feature = "webkit")]
            let mut engine = ultralight::Ultralight::new(800, 800);

            while let Ok(command) = read_commands.recv() {
                match command {
                    Commands::DoWork => engine.do_work(),
                    Commands::Render => engine.render(),
                    Commands::NeedRender => {
                        send_response
                            .send(CommandsRecv::NeedRender(engine.needs_render()))
                            .unwrap();
                    }
                    Commands::Resize(w, h) => engine.resize(w, h),
                    Commands::Size => {
                        let size = engine.size();
                        send_response
                            .send(CommandsRecv::Size(size.0, size.1))
                            .unwrap()
                    }
                    Commands::PixelBuffer => {
                        if let Some(buffer) = engine.pixel_buffer() {
                            send_response
                                .send(CommandsRecv::PixelBuffer(buffer))
                                .unwrap()
                        }
                    }
                    Commands::Url => {
                        if let Some(url) = engine.get_url() {
                            send_response.send(CommandsRecv::UrlRecv(url)).unwrap()
                        }
                    }
                    Commands::GotoUrl(url) => engine.goto_url(url.as_str()),
                    Commands::HasLoaded => {
                        let has_loaded = engine.has_loaded();
                        send_response
                            .send(CommandsRecv::HasLoaded(has_loaded))
                            .unwrap();
                    }

                    Commands::NewTab(url) => engine.new_tab(url.as_str()),
                    Commands::GotoTab(url) => engine.goto_url(url.as_str()),
                    Commands::Refresh => engine.refresh(),
                    Commands::GoForward => engine.go_forward(),
                    Commands::GoBackward => engine.go_back(),
                    Commands::Focus => engine.focus(),
                    Commands::Unfocus => engine.unfocus(),
                    Commands::Scroll(delta) => {
                        send_response
                            .send(CommandsRecv::Scroll(engine.scroll(delta)))
                            .unwrap();
                    }
                    Commands::Keyboard(event) => {
                        send_response
                            .send(CommandsRecv::Scroll(engine.handle_keyboard_event(event)))
                            .unwrap();
                    }
                    Commands::Mouse(point, event) => {
                        send_response
                            .send(CommandsRecv::Scroll(
                                engine.handle_mouse_event(point, event),
                            ))
                            .unwrap();
                    }
                }
            }
        });

        Self {
            sender: send_commands,
            receiver: read_response,
        }
    }

    pub fn send(&self, command: Commands) {
        self.sender.send(command).unwrap()
    }

    pub fn recv(&self, command: Commands) -> CommandsRecv {
        self.sender.send(command).unwrap();
        self.receiver.recv().unwrap()
    }
}

trait BrowserEngine {
    fn new(width: u32, height: u32) -> Self;

    fn do_work(&self);
    fn render(&self);
    fn needs_render(&self) -> bool;
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;
    fn get_image(&mut self) -> Option<Image<Handle>>;

    fn get_title(&self) -> Option<String>;
    fn get_url(&self) -> Option<String>;
    fn goto_url(&self, url: &str);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: &str);
    fn goto_tab(&mut self, url: &str) -> Option<()>;

    fn refresh(&self);
    fn go_forward(&self);
    fn go_back(&self);
    fn focus(&self);
    fn unfocus(&self);

    fn scroll(&self, delta: mouse::ScrollDelta) -> Status;
    fn handle_keyboard_event(&self, event: keyboard::Event) -> Status;
    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) -> Status;
}

fn bgr_to_rgb(image: Vec<u8>) -> Vec<u8> {
    image
        .chunks(4)
        .map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
        .flatten()
        .collect()
}

pub fn create_image(image: Vec<u8>, w: u32, h: u32, bgr: bool) -> Image<Handle> {
    let image = if bgr { bgr_to_rgb(image) } else { image };
    let handle = Handle::from_pixels(w, h, image);
    Image::new(handle)
}

pub fn create_empty_view(w: u32, h: u32) -> Image<Handle> {
    let mut image: Vec<u8> = Vec::new();
    for _ in 0..(w * h) {
        image.push(255);
    }
    create_image(image, w, h, false)
}
