use crate::{
    styles::{self, container_styles::release_time_container_theme},
    util::sql::{box_work_test, login_user, sn_work},
    BoxDataInfo, 
    // WindowState,
};
use crate::{Selected, SnDataInfo, TipType};

use iced::{
    widget::{column, container},
    window, Application, Command, Element, Subscription, Theme,
};
use iced_aw::date_picker::Date;
use time::Instant;

use super::{
    bandin_view::bandinview, bar::bar, foot::foot, login::login, order_view::orderview,
    query_view::queryview, tip::tip,
};
pub struct MyTools {
    pub view: TabId,
    pub input_value: String,
    pub boxlists: Vec<BoxDataInfo>,
    pub snlists: Vec<SnDataInfo>,
    pub text: String,
    pub selected: Option<Selected>,
    pub quantity: usize,
    pub show_picker_1: bool,
    pub show_picker_2: bool,
    pub datequery: bool,
    pub date1: Date,
    pub date2: Date,
    pub show_modal: bool,
    pub logined: bool,
    pub name: String,
    pub password: String,
    pub tip_type: TipType,
    pub now: time::OffsetDateTime,
    pub starttime: Instant,
    pub endtime: f32,
    pub theme: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabId {
    #[default]
    Query,
    Bandin,
    Order,
}

#[derive(Clone, Debug)]
pub enum Message {
    Button,
    Addboxresult(Vec<BoxDataInfo>),
    Addsnresult(Vec<SnDataInfo>),
    Textchange(String),
    Selected(Selected),
    Open1,
    Cancel1,
    Submit1(Date),
    Open2,
    Cancel2,
    Submit2(Date),
    Datequery(bool),
    OpenModal,
    CloseModal,
    Logined(bool),
    Namechange(String),
    Passchange(String),
    Logingpre,
    LoginCence,
    Loginbool(bool),
    Datetime(time::OffsetDateTime),
    SwitchChanged(bool),
    QueryAction,
    BandinAction,
    OrderAction,
    Minwindow,
    // Maxwindow(WindowState),
    Closewindow,
}

impl Application for MyTools {
    type Message = Message;
    type Theme = Theme;
    type Executor = tokio::runtime::Runtime;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                view: Default::default(),
                boxlists: vec![],
                input_value: Default::default(),
                text: Default::default(),
                selected: Default::default(),
                snlists: vec![],
                quantity: Default::default(),
                show_picker_1: false,
                show_picker_2: false,
                datequery: false,
                date1: Date::today(),
                date2: Date::today(),
                show_modal: false,
                logined: false,
                name: Default::default(),
                password: Default::default(),
                tip_type: Default::default(),
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                starttime: Instant::now(),
                endtime: 0.0,
                theme: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from(format!(
            "{} - V {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Button => {
                self.starttime = Instant::now();
                if self.input_value.len() == 0 {
                    self.show_modal = true;
                    self.tip_type = TipType::EmptyTip;
                    Command::none()
                } else {
                    let s = self.input_value.clone();
                    self.text = s.clone();
                    match self.selected {
                        Some(value) => match value {
                            Selected::Sn => Command::perform(sn_work(s), Message::Addsnresult),
                            Selected::Box => {
                                Command::perform(box_work_test(s), Message::Addboxresult)
                            }
                            Selected::Carton => return Command::none(),
                            Selected::Workid => return Command::none(),
                        },
                        None => return Command::none(),
                    }
                }
            }
            Message::Textchange(value) => {
                self.input_value = value;
                Command::none()
            }
            Message::Addsnresult(value) => {
                self.snlists = value;
                self.quantity = self.snlists.len();
                let start = self.starttime.clone();
                self.endtime = start.elapsed().as_seconds_f32();
                Command::none()
            }
            Message::Selected(check) => {
                self.selected = Some(check);
                Command::none()
            }
            Message::Addboxresult(value) => {
                self.boxlists = value;
                self.quantity = self.boxlists.len();
                let start = self.starttime.clone();
                self.endtime = start.elapsed().as_seconds_f32();
                Command::none()
            }
            Message::Open1 => {
                self.show_picker_1 = true;
                Command::none()
            }
            Message::Cancel1 => {
                self.show_picker_1 = false;
                Command::none()
            }
            Message::Submit1(value) => {
                self.date1 = value;
                self.show_picker_1 = false;
                Command::none()
            }
            Message::Datequery(bool) => {
                self.datequery = bool;
                Command::none()
            }
            Message::Open2 => {
                self.show_picker_2 = true;
                Command::none()
            }
            Message::Cancel2 => {
                self.show_picker_2 = false;
                Command::none()
            }
            Message::Submit2(value) => {
                self.date2 = value;
                self.show_picker_2 = false;
                Command::none()
            }
            Message::OpenModal => {
                self.show_modal = true;
                Command::none()
            }
            Message::CloseModal => {
                self.show_modal = false;
                Command::none()
            }
            Message::Logined(value) => {
                self.logined = value;
                Command::none()
            }
            Message::Namechange(value) => {
                self.name = value;
                Command::none()
            }
            Message::Passchange(value) => {
                self.password = value;
                Command::none()
            }
            Message::Logingpre => {
                let name = self.name.clone();
                let password = self.password.clone();
                Command::perform(login_user(name, password), Message::Loginbool)
            }
            Message::LoginCence => {
                self.logined = false;
                window::close()
            }
            Message::Loginbool(bool) => {
                self.logined = bool;
                if !self.logined {
                    self.show_modal = true;
                    self.tip_type = TipType::LoginTip;
                }
                Command::none()
            }
            Message::Datetime(local_time) => {
                let now = local_time;
                if now != self.now {
                    self.now = now;
                }
                Command::none()
            }
            Message::SwitchChanged(value) => {
                self.theme = value;
                Command::none()
            }
            Message::QueryAction => {
                self.view = TabId::Query;
                Command::none()
            }
            Message::BandinAction => {
                self.view = TabId::Bandin;
                Command::none()
            }
            Message::OrderAction => {
                self.view = TabId::Order;
                Command::none()
            }
            Message::Minwindow => window::minimize(true),
            // Message::Maxwindow(value) => match value {
            //     WindowState::Max => {
            //         window::resize(iced::Size{width: 1650, height: 800 })
            //     },
            //     WindowState::Default => {
            //         window::maximize(true)
            //     },
            // },
            Message::Closewindow => window::close(),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000)).map(|_| {
            Message::Datetime(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }
    fn view(&self) -> Element<'static, Self::Message> {
        let bar = bar(self);
        let mian_view = match self.view {
            TabId::Query => queryview(self),
            TabId::Bandin => bandinview(self),
            TabId::Order => orderview(self),
        };
        let foot = foot(
            self,
            self.now.to_string(),
            self.endtime,
            env!("CARGO_PKG_VERSION").to_string(),
            self.theme,
        );
        let login = login(self);
        let body = match self.tip_type {
            TipType::EmptyTip => "输入框不能为空，请输入要查询的内容再进行查询！",
            TipType::LoginTip => "登录异常，工号不存在或者密码错误，请确认后再进行登录！",
        };
        let all_tip = tip(self, login, body);
        let col = column![bar, mian_view, foot, all_tip];
        container(col)
            .padding(5)
            .style(release_time_container_theme())
            .into()
    }
    fn theme(&self) -> Self::Theme {
        let custom_theme = Box::new(
            match self.theme {
                true => styles::theme::TroxideTheme::Dark,
                false => styles::theme::TroxideTheme::Light,
            }
            .get_custom_theme(),
        );
        iced::Theme::Custom(custom_theme)
    }
}
