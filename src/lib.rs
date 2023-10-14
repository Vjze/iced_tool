use std::fmt::{Display, Formatter};
pub mod util;
pub mod views;
pub mod styles;
pub mod logos {
    pub static IMG_LOGO_DARK: &[u8] = include_bytes!("../resources/logo/logo_dark.png");
    pub static IMG_LOGO_LIGHT: &[u8] = include_bytes!("../resources/logo/logo_light.png");
}
// #[derive(Debug, Clone, Copy, Default)]
// pub enum  WindowState{
//     Max,
//     #[default]
//     Default,
// }

#[derive(Debug, Clone, Copy, Default)]
pub enum TipType {
    #[default]
    EmptyTip,
    LoginTip,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Selected {
    #[default]
    Sn,
    Box,
    Carton,
    Workid,
}

impl Selected {
    const ALL: [Selected; 4] = [
        Selected::Sn,
        Selected::Box,
        Selected::Carton,
        Selected::Workid,
    ];
}

impl Display for Selected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Selected::Sn => "Sn查询",
                Selected::Box => "盒号查询",
                Selected::Carton => "箱号查询",
                Selected::Workid => "工号查询",
            }
        )
    }
}
#[derive(Debug, Clone, Default)]
pub struct SnDataInfo {
    sn: String,
    product_bill: String,
    test_type: String,
    result: String,
    ith: String,
    pf: String,
    vop: String,
    im: String,
    rs: String,
    sen: String,
    res: String,
    icc: String,
    idark: String,
    vbr: String,
    ixtalk: String,
    kink: String,
    testdate: String,
}
impl SnDataInfo {
    pub fn sn(&self) -> String {
        self.sn.clone()
    }

    pub fn productbill(&self) -> String {
        self.product_bill.clone()
    }

    pub fn testtype(&self) -> String {
        self.test_type.clone()
    }

    pub fn result(&self) -> String {
        self.result.clone()
    }

    pub fn ith(&self) -> String {
        self.ith.clone()
    }
    pub fn pf(&self) -> String {
        self.pf.clone()
    }
    pub fn vop(&self) -> String {
        self.vop.clone()
    }

    pub fn im(&self) -> String {
        self.im.clone()
    }

    pub fn rs(&self) -> String {
        self.rs.clone()
    }

    pub fn sen(&self) -> String {
        self.sen.clone()
    }

    pub fn res(&self) -> String {
        self.res.clone()
    }
    pub fn icc(&self) -> String {
        self.icc.clone()
    }
    pub fn idark(&self) -> String {
        self.idark.clone()
    }

    pub fn vbr(&self) -> String {
        self.vbr.clone()
    }

    pub fn ixtalk(&self) -> String {
        self.ixtalk.clone()
    }
    pub fn kink(&self) -> String {
        self.kink.clone()
    }
    pub fn testdate(&self) -> String {
        self.testdate.clone()
    }
}
#[derive(Debug, Clone, Default)]
pub struct BoxDataInfo {
    pno: String,
    sn: String,
    pn: String,
    workorder: String,
    creator: String,
    createtime: String,
}

impl BoxDataInfo {

    pub fn pno(&self) -> String {
        self.pno.clone()
    }

    pub fn sn(&self) -> String {
        self.sn.clone()
    }

    pub fn pn(&self) -> String {
        self.pn.clone()
    }

    pub fn workorder(&self) -> String {
        self.workorder.clone()
    }

    pub fn creator(&self) -> String {
        self.creator.clone()
    }
    pub fn createtime(&self) -> String {
        self.createtime.clone()
    }
}
