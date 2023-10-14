use iced::{
    widget::{container, text, Container},
    Length, Renderer, alignment::{Horizontal, Vertical},
};

use super::main_ui::{Message, MyTools};

pub fn orderview(_state: &MyTools) -> Container<'static, Message, Renderer> {
    let text = text(format!("TODO")).size(100);

    container(text)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
