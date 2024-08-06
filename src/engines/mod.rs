use iced::keyboard;
use iced::mouse;
use iced::{event::Status, Point};
use kanal::{unbounded, Receiver, Sender};
use std::thread;

#[cfg(feature = "webkit")]
pub mod ultralight;

#[allow(unused)]
pub trait BrowserEngine {
    fn new() -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn render(&mut self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;

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

// This enum needs to match all the avalible methods in BrowserEngine
#[derive(Debug)]
enum SendCommand {
    DoWork,
    NeedRenderer,
    Render,
    Size,
    Resize(u32, u32),
    PixelBuffer,
    GetTitle,
    GetUrl,
    GotoUrl(String),
    GotoTab(String),
    HasLoaded,
    NewTab(String),
    Refresh,
    GoForward,
    GoBackward,
    Focus,
    Unfocus,
    Scroll(mouse::ScrollDelta),
    HandleKeyboardEvent(keyboard::Event),
    HandleMouseEvent(Point, mouse::Event),
}

// This is all the responses BrowserEngine can have
#[derive(Debug)]
enum RecvCommand {
    NeedRenderer(bool),
    Size(u32, u32),
    PixelBuffer(Vec<u8>),
    GetTitle(String),
    GetUrl(String),
    HasLoaded(bool),
    Scroll(Status),
    HandleKeyboardEvent(Status),
    HandleMouseEvent(Status),
}

// methods to pass calls to web engine
pub struct Engine {
    sender: Sender<SendCommand>,
    reciever: Receiver<RecvCommand>,
}

impl Engine {
    pub fn new<E: BrowserEngine>() -> Self {
        let (sender, thread_reciever) = unbounded();
        let (thread_sender, reciever) = unbounded();

        // create new thread to run browser engine in
        thread::spawn(move || {
            let mut engine = E::new();

            loop {
                engine.do_work();

                if let Ok(command) = thread_reciever.recv() {
                    match command {
                        SendCommand::DoWork => engine.do_work(),
                        SendCommand::NeedRenderer => thread_sender
                            .send(RecvCommand::NeedRenderer(engine.need_render()))
                            .unwrap(),
                        SendCommand::Render => engine.render(),
                        SendCommand::Size => {
                            let size = engine.size();
                            thread_sender
                                .send(RecvCommand::Size(size.0, size.1))
                                .unwrap()
                        }
                        SendCommand::Resize(w, h) => engine.resize(w, h),
                        SendCommand::PixelBuffer => thread_sender
                            .send(RecvCommand::PixelBuffer(engine.pixel_buffer().unwrap()))
                            .unwrap(),
                        SendCommand::GetTitle => thread_sender
                            .send(RecvCommand::GetTitle(engine.get_title().unwrap()))
                            .unwrap(),
                        SendCommand::GetUrl => thread_sender
                            .send(RecvCommand::GetUrl(engine.get_url().unwrap()))
                            .unwrap(),
                        SendCommand::GotoUrl(url) => engine.goto_url(&url),
                        SendCommand::GotoTab(url) => engine.goto_tab(&url).unwrap(),
                        SendCommand::HasLoaded => thread_sender
                            .send(RecvCommand::HasLoaded(engine.has_loaded()))
                            .unwrap(),
                        SendCommand::NewTab(url) => engine.new_tab(&url),
                        SendCommand::Refresh => engine.refresh(),
                        SendCommand::GoForward => engine.go_forward(),
                        SendCommand::GoBackward => engine.go_back(),
                        SendCommand::Focus => engine.focus(),
                        SendCommand::Unfocus => engine.unfocus(),
                        SendCommand::Scroll(delta) => thread_sender
                            .send(RecvCommand::Scroll(engine.scroll(delta)))
                            .unwrap(),
                        SendCommand::HandleKeyboardEvent(event) => thread_sender
                            .send(RecvCommand::HandleKeyboardEvent(
                                engine.handle_keyboard_event(event),
                            ))
                            .unwrap(),
                        SendCommand::HandleMouseEvent(point, event) => thread_sender
                            .send(RecvCommand::HandleMouseEvent(
                                engine.handle_mouse_event(point, event),
                            ))
                            .unwrap(),
                    }
                }
            }
        });

        Self { sender, reciever }
    }

    fn send(&self, command: SendCommand) {
        // ensure command is expecting a response
        match command {
            SendCommand::NeedRenderer => panic!("This Command requires a response"),
            SendCommand::Size => panic!("This Command requires a response"),
            SendCommand::PixelBuffer => panic!("This Command requires a response"),
            SendCommand::GetTitle => panic!("This Command requires a response"),
            SendCommand::GetUrl => panic!("This Command requires a response"),
            SendCommand::HasLoaded => panic!("This Command requires a response"),
            SendCommand::Scroll(_) => panic!("This Command requires a response"),
            SendCommand::HandleKeyboardEvent(_) => panic!("This Command requires a response"),
            SendCommand::HandleMouseEvent(_, _) => panic!("This Command requires a response"),
            _ => (),
        };

        self.sender.send(command).unwrap()
    }

    fn recv(&self, command: SendCommand) -> Option<RecvCommand> {
        // ensure command is not expecting a response
        match command {
            SendCommand::DoWork => panic!("This Command has no response"),
            SendCommand::Render => panic!("This Command has no response"),
            SendCommand::Resize(_, _) => panic!("This Command has no response"),
            SendCommand::GotoUrl(_) => panic!("This Command has no response"),
            SendCommand::NewTab(_) => panic!("This Command has no response"),
            SendCommand::Refresh => panic!("This Command has no response"),
            SendCommand::GoForward => panic!("This Command has no response"),
            SendCommand::GoBackward => panic!("This Command has no response"),
            SendCommand::Focus => panic!("This Command has no response"),
            SendCommand::Unfocus => panic!("This Command has no response"),
            _ => (),
        };

        self.sender.send(command).ok()?;
        self.reciever.recv().ok()
    }
}

impl BrowserEngine for Engine {
    fn new() -> Self {
        #[cfg(feature = "webkit")]
        Engine::new::<ultralight::Ultralight>()
    }

    fn do_work(&self) {
        self.send(SendCommand::DoWork)
    }

    fn need_render(&self) -> bool {
        if let RecvCommand::NeedRenderer(need_render) =
            self.recv(SendCommand::NeedRenderer).unwrap()
        {
            need_render
        } else {
            unreachable!()
        }
    }

    fn render(&mut self) {
        self.send(SendCommand::Render)
    }

    fn size(&self) -> (u32, u32) {
        if let RecvCommand::Size(w, h) = self.recv(SendCommand::Size).unwrap() {
            (w, h)
        } else {
            unreachable!()
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.send(SendCommand::Resize(width, height))
    }

    fn pixel_buffer(&mut self) -> Option<Vec<u8>> {
        if let RecvCommand::PixelBuffer(buffer) = self.recv(SendCommand::PixelBuffer).unwrap() {
            Some(buffer)
        } else {
            unreachable!()
        }
    }

    fn get_title(&self) -> Option<String> {
        if let RecvCommand::GetTitle(title) = self.recv(SendCommand::GetTitle).unwrap() {
            Some(title)
        } else {
            unreachable!()
        }
    }

    fn get_url(&self) -> Option<String> {
        if let RecvCommand::GetUrl(url) = self.recv(SendCommand::GetUrl).unwrap() {
            Some(url)
        } else {
            unreachable!()
        }
    }

    fn goto_url(&self, url: &str) {
        self.send(SendCommand::GotoUrl(url.to_string()))
    }

    fn has_loaded(&self) -> bool {
        if let RecvCommand::HasLoaded(loaded) = self.recv(SendCommand::HasLoaded).unwrap() {
            loaded
        } else {
            unreachable!()
        }
    }

    fn new_tab(&mut self, url: &str) {
        self.send(SendCommand::NewTab(url.to_string()))
    }

    fn goto_tab(&mut self, url: &str) -> Option<()> {
        Some(self.send(SendCommand::GotoTab(url.to_string())))
    }

    fn refresh(&self) {
        self.send(SendCommand::Refresh)
    }

    fn go_forward(&self) {
        self.send(SendCommand::GoForward)
    }

    fn go_back(&self) {
        self.send(SendCommand::GoBackward)
    }

    fn focus(&self) {
        self.send(SendCommand::Focus)
    }

    fn unfocus(&self) {
        self.send(SendCommand::Unfocus)
    }

    fn scroll(&self, delta: mouse::ScrollDelta) -> Status {
        if let RecvCommand::Scroll(status) = self.recv(SendCommand::Scroll(delta)).unwrap() {
            status
        } else {
            unreachable!()
        }
    }

    fn handle_keyboard_event(&self, event: keyboard::Event) -> Status {
        if let RecvCommand::HandleKeyboardEvent(status) =
            self.recv(SendCommand::HandleKeyboardEvent(event)).unwrap()
        {
            status
        } else {
            unreachable!()
        }
    }

    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) -> Status {
        if let RecvCommand::HandleMouseEvent(status) = self
            .recv(SendCommand::HandleMouseEvent(point, event))
            .unwrap()
        {
            status
        } else {
            unreachable!()
        }
    }
}
