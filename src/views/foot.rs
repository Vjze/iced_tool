use iced::{
    widget::{container, row, text, toggler, Container, Space},
    Length,
};

use super::main_ui::{Message, MyTools};

pub fn foot(state: &MyTools, t: String, rt: f32, v: String, b: bool) -> Container<'static, Message> {
    let tt = &t[..19];
    let rtt = rt.to_string();
    let mut rt_1 = "";
    if rtt.len() < 2 {
        rt_1 = "0";
    } else {
        rt_1 = &rtt[..6];
    }
    let theme = if state.theme {
        String::from("夜间")
    } else {
        String::from("白天")
    };
    let time = text(format!("{}", tt));
    let run_time = text(format!("本次查询时间：{} 秒 ", rt_1));
    let version = text(format!("当前版本：{}", v));
    let toggler = toggler(theme, b, Message::SwitchChanged);
    let row = row![
        time,
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        run_time,
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        Space::new(Length::Fill, Length::Shrink),
        version,
        Space::new(Length::Fill, Length::Shrink),
        toggler
    ]
    .width(Length::Fill);
    container(row).into()
}
