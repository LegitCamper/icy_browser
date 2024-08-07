// use iced::advanced::{graphics::core::Element, widget::Operation};
// use iced::widget::{component, container, text_input, text_input::Id, Component, TextInput};
// use iced::{theme::Theme, Renderer};

// pub struct HoverTextInput<'a, Message> {
//     placeholder: String,
//     text: String,
//     // id: Id,
//     text_input: TextInput<'a, Message>,
// }

// impl<Message: Clone> HoverTextInput<'_, Message> {
//     pub fn new() -> Self {
//         HoverTextInput {
//             placeholder: String::from("hello"),
//             text: String::from("there"),
//             // id: Id::unique(),
//             text_input: text_input("fasdf", "here"),
//         }
//     }
// }

// #[derive(Clone)]
// pub enum Event {}

// impl<Message: Clone> Component<Message> for HoverTextInput<'_, Message> {
//     type State = ();
//     type Event = Event;

//     fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
//         match event {};
//         None
//     }

//     fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme, Renderer> {
//         container(self.text_input).into()
//     }

//     fn operate(&self, state: &mut Self::State, operation: &mut dyn Operation<Message>) {
//         operation.text_input(self.text_input, None)
//         // operation.focusable(state, self.id)
//     }
// }

// impl<'a, Message: 'a + Clone> From<HoverTextInput<'_, Message>>
//     for Element<'a, Message, Theme, Renderer>
// {
//     fn from(widget: HoverTextInput<Message>) -> Self {
//         component(widget)
//     }
// }
