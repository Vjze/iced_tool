use crate::logos::{IMG_LOGO_DARK, IMG_LOGO_LIGHT};
use crate::styles::button_styles::{
    transparent_button_theme, transparent_button_with_rounded_border_theme,
};

use super::main_ui::{Message, MyTools};
use iced::widget::image;
use iced::Font;
use iced::{
    widget::{button, container, image::Handle, row, text, Container, Space},
    Length, Renderer,
};
pub fn bar(state: &MyTools) -> Container<'static, Message, Renderer> {
    let theme = if state.theme {
        IMG_LOGO_DARK
    } else {
        IMG_LOGO_LIGHT
    };
    let min = button("-")
        .on_press(Message::Minwindow)
        .style(transparent_button_theme());
    // let max = button("口")
    //     .on_press(Message::Maxwindow)
    //     .style(transparent_button_theme());
    let close = button("X")
        .on_press(Message::Closewindow)
        .style(transparent_button_theme());
    let logo = image(Handle::from_memory(theme)).height(Length::Shrink);
    let title = text("Iced_Tools").size(40.0).font(Font::MONOSPACE);
    let query_button = button("查询功能合集")
        .on_press(Message::QueryAction)
        .style(transparent_button_with_rounded_border_theme());
    let bandin_button = button("条码绑定")
        .on_press(Message::BandinAction)
        .style(transparent_button_with_rounded_border_theme());
    let order_button = button("工单打印")
        .on_press(Message::OrderAction)
        .style(transparent_button_with_rounded_border_theme());

    let bar = row![
        logo,
        title,
        Space::new(Length::Fill, Length::Shrink),
        query_button,
        bandin_button,
        order_button,
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        min,
        // max,
        close,
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center);
    container(bar).max_height(60.0).padding(5).into()
}
