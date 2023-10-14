use iced::{
    alignment::Horizontal,
    widget::{button, checkbox, column, container, pick_list, row, text, text_input, Container},
    Length, Renderer,
};
use iced_aw::date_picker;

use crate::{
    styles::{
        self, button_styles::transparent_button_with_rounded_border_theme,
        container_styles::second_class_container_rounded_theme,
    },
    Selected,
};

use super::{
    listview::{box_list_view, sn_list_view},
    main_ui::{Message, MyTools},
};

pub fn queryview(state: &MyTools) -> Container<'static, Message, Renderer> {
    let input = text_input("输入要查询的数据...", &state.input_value)
        .on_input(Message::Textchange)
        .on_submit(Message::Button)
        .size(16);

    let select = pick_list(&Selected::ALL[..], state.selected, Message::Selected)
        .placeholder("选择查询项....");
    let check = checkbox("以日期查询", state.datequery, Message::Datequery);
    let pick_date_1 = button(text(format!("{}", state.date1))).on_press(Message::Open1);
    let pick_date_2 = button(text(format!("{}", state.date2))).on_press(Message::Open1);
    let datepicker_1 = date_picker(
        state.show_picker_1,
        state.date1,
        pick_date_1,
        Message::Cancel1,
        Message::Submit1,
    );
    let datepicker_2 = date_picker(
        state.show_picker_2,
        state.date2,
        pick_date_2,
        Message::Cancel2,
        Message::Submit2,
    );
    let text_much = if state.quantity == 0 {
        text(format!("数量:{}", state.quantity))
            .size(18)
            .style(styles::text_styles::red_text_theme())
    } else {
        text(format!("数量:{}", state.quantity))
            .size(18)
            .style(styles::text_styles::green_text_theme())
    };
    let but = button("查 询")
        .on_press(Message::Button)
        .style(transparent_button_with_rounded_border_theme());
    let main_row = row![input, select, check, text_much]
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .align_items(Horizontal::Center.into());

    let sele = state.selected.clone();

    let con: Container<'_, Message> = match sele {
        Some(value) => match value {
            Selected::Sn => {
                let list = state.snlists.clone();
                let l = sn_list_view(list);
                let col = column![main_row.push(but), l];
                container(col).width(Length::Fill).height(Length::Fill)
                // .into()
            }
            Selected::Box => {
                let dataquery_is_check = match state.datequery {
                    true => container(row![datepicker_1, datepicker_2].spacing(10)),
                    false => container(""),
                };
                let list = state.boxlists.clone();
                let l = box_list_view(list);
                let col = column![main_row.push(dataquery_is_check).push(but), l];
                container(col).width(Length::Fill).height(Length::Fill)
                // .into()
            }
            Selected::Carton => todo!(),
            Selected::Workid => todo!(),
        },
        None => {
            let text = text("本工具由纯Rust语言编写，包括UI（Iced）也是纯Rust！").size(40);
            let col = column![main_row.push(but), text].align_items(Horizontal::Center.into());
            container(col)
                .center_x()
                .width(Length::Fill)
                .height(Length::Fill)
            // .into()
        }
    };

    container(con)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(second_class_container_rounded_theme())
        .into()
}
