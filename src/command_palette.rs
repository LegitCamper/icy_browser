use crate::Bookmark;
use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::keyboard::{self, Key};
use iced::mouse::Cursor;
use iced::widget::canvas::event::Status;
use iced::widget::{center, column, container, mouse_area, opaque, stack};
use iced::widget::{scrollable, text, Column};
use iced::{border, Color, Element, Length, Theme};
use iced::{Event, Rectangle, Renderer, Size};
use strum_macros::Display;

#[derive(Clone, Debug, Display, PartialEq)]
/// The entry types for command palette
pub enum PaletteEntry<Message> {
    None,
    #[strum(to_string = "Commands")]
    Command(Message),
    #[strum(to_string = "Bookmarks")]
    Bookmark(Bookmark),
    #[strum(to_string = "Tabs")]
    /// title, url
    Tab(String, String),
}

impl<Message: ToString> PaletteEntry<Message> {
    pub fn inner_name(&self) -> String {
        match self {
            PaletteEntry::Command(command) => command.to_string(),
            PaletteEntry::Bookmark(bookmark) => {
                format!("{} -> {}", bookmark.name(), bookmark.url())
            }
            PaletteEntry::Tab(title, url) => format!("{}: {}", title, url),
            PaletteEntry::None => String::new(),
        }
    }
}

fn results_list<'a, Message: ToString + 'a>(
    results: &[PaletteEntry<Message>],
    selected_item: Option<String>,
) -> Element<'a, Message> {
    let mut list = Vec::new();
    let mut result_types = Vec::new();

    for result in results {
        if !result_types.contains(&result.to_string()) {
            result_types.push(result.to_string());
            list.push(text(result.to_string()).size(20).into())
        }

        let mut text = container(text(format!("   {}", result.inner_name())).size(16));
        if let Some(selected_item) = selected_item.as_ref() {
            if result.inner_name() == *selected_item {
                text = text.style(|theme: &Theme| {
                    container::Style::default().background(theme.palette().primary)
                })
            }
        }
        list.push(text.into())
    }

    scrollable(Column::from_vec(list))
        .width(Length::Fill)
        .spacing(10)
        .into()
}

pub fn command_palette<'a, Message: Clone + ToString>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    state: &'a mut CommandPaletteState<Message>,
    on_hide_command_palette: Message,
) -> CommandPalette<'a, Message> {
    CommandPalette::new(content, state, on_hide_command_palette)
}

pub struct CommandPaletteState<Message: ToString> {
    query: String,
    possible_results: Vec<PaletteEntry<Message>>,
    filtered_results: Vec<PaletteEntry<Message>>,
    selected_item: Option<String>,
}

impl<Message: ToString + Clone> CommandPaletteState<Message> {
    pub fn new(commands: Vec<Message>, bookmarks: Option<Vec<Bookmark>>) -> Self {
        let mut results: Vec<PaletteEntry<Message>> = Vec::new();
        // This may need to be extended in the future
        results.extend(commands.into_iter().map(PaletteEntry::Command));
        if let Some(bookmarks) = bookmarks {
            results.extend(bookmarks.into_iter().map(PaletteEntry::Bookmark));
        };

        Self {
            query: String::new(),
            possible_results: results.clone(),
            filtered_results: results,
            selected_item: None,
        }
    }

    pub fn submitted(&self) -> PaletteEntry<Message> {
        match &self.selected_item {
            Some(selected) => {
                for result in self.filtered_results.iter() {
                    if result.inner_name() == *selected {
                        return result.clone();
                    }
                }
                panic!("Selected item was not found in filtered results")
            }
            None => panic!("Item was not selected before submitting"),
        }
    }

    pub fn reset(&mut self) {
        self.query = String::new();
        self.filtered_results = self.possible_results.clone();
        self.selected_item = None;
    }

    pub fn first_item(&mut self) {
        self.selected_item = self
            .filtered_results
            .first()
            .map(|res| res.inner_name())
            .or(None)
    }

    pub fn next_item(&mut self) {
        match &self.selected_item {
            None => {
                self.selected_item = self
                    .filtered_results
                    .first()
                    .map(|res| res.inner_name())
                    .or(None)
            }
            Some(selected_item) => {
                if let Some(last) = self.filtered_results.last() {
                    if *selected_item != last.inner_name() {
                        if let Some(pos) = self
                            .filtered_results
                            .iter()
                            .position(|res| res.inner_name() == *selected_item)
                        {
                            self.selected_item = Some(self.filtered_results[pos + 1].inner_name());
                        } else {
                            self.selected_item = None
                        }
                    }
                }
            }
        }
    }

    pub fn previous_item(&mut self) {
        match &self.selected_item {
            None => {
                self.selected_item = self
                    .filtered_results
                    .first()
                    .map(|res| res.inner_name())
                    .or(None)
            }
            Some(selected_item) => {
                if let Some(first) = self.filtered_results.first() {
                    if *selected_item != first.inner_name() {
                        if let Some(pos) = self
                            .filtered_results
                            .iter()
                            .position(|res| res.inner_name() == *selected_item)
                        {
                            self.selected_item = Some(self.filtered_results[pos - 1].inner_name());
                        } else {
                            self.selected_item = None;
                        }
                    }
                }
            }
        }
    }
}

pub struct CommandPalette<'a, Message: Clone + ToString> {
    webview: Element<'a, Message, Theme, Renderer>,
    state: &'a mut CommandPaletteState<Message>,
    on_hide_command_palette: Message,
}

impl<'a, Message: Clone + ToString> CommandPalette<'a, Message> {
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        state: &'a mut CommandPaletteState<Message>,
        on_hide_command_palette: Message,
    ) -> Self {
        CommandPalette {
            webview: content.into(),
            state,
            on_hide_command_palette,
        }
    }
}
impl<Message: 'static + ToString + Clone> Widget<Message, Theme, Renderer>
    for CommandPalette<'_, Message>
where
    Renderer: iced::advanced::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.webview)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.webview]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let child_layout = self
            .webview
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits);

        Node::with_children(child_layout.size(), vec![child_layout])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let search = container(text(&self.state.query).size(25))
            .style(|theme: &Theme| container::bordered_box(theme))
            .padding(5)
            .width(Length::Fill);

        let mut window = container(column![
            search,
            container(results_list(
                self.state.filtered_results.as_slice(),
                self.state.selected_item.clone(),
            ))
            .width(Length::Fill)
            .height(Length::Fill)
        ])
        .padding(10)
        .center(600);

        window = window.style(|theme: &Theme| container::Style {
            background: Some(theme.palette().background.into()),
            border: border::rounded(10),
            ..container::Style::default()
        });

        stack![
            self.webview,
            opaque(
                mouse_area(center(opaque(window)).style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                }))
                .on_press(self.on_hide_command_palette.clone()),
            )
        ]
        .draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        )
    }

    fn on_event(
        &mut self,
        _tree: &mut Tree,
        event: Event,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> Status {
        match event {
            Event::Keyboard(event) => match event {
                keyboard::Event::KeyPressed {
                    key,
                    modified_key: _,
                    physical_key: _,
                    location: _,
                    modifiers: _,
                    text,
                } => {
                    if key == Key::Named(keyboard::key::Named::ArrowUp) {
                        self.state.previous_item();
                        Status::Captured
                    } else if key == Key::Named(keyboard::key::Named::ArrowDown) {
                        self.state.next_item();
                        Status::Captured
                    } else {
                        if let Some(text) = text {
                            if !text.is_empty() {
                                self.state.query.push_str(text.as_str());
                                return Status::Captured;
                            }
                            return Status::Ignored;
                        }
                        Status::Ignored
                    }
                }
                _ => Status::Ignored,
            },
            Event::Mouse(_event) => Status::Ignored,
            Event::Window(_event) => Status::Ignored,
            Event::Touch(_event) => Status::Ignored,
        }
    }
}

impl<'a, Message: 'a + Clone + ToString, Theme, Renderer> From<CommandPalette<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
    CommandPalette<'a, Message>: Widget<Message, Theme, Renderer>,
{
    fn from(command_palette: CommandPalette<'a, Message>) -> Self {
        Self::new(command_palette)
    }
}
