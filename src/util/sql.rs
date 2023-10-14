use tiberius::time::chrono;

use crate::{BoxDataInfo, SnDataInfo};

use super::util::{client, sn_client, login_check};
pub async fn sn_work(s: String) -> Vec<SnDataInfo> {
    let mut row_data = Vec::new();
    let mut client = sn_client().await;
    let query_ty = format!("where SN = '{}'", s);
    // let testtype = "SN,ProductBill,TestType,Result,Ith,Pf,Vop,Im,Rs,Sen,Res,ICC,Idark,Vbr,IXtalk,Kink,TestDate";
    let testtype_12 = "SN,ProductBill,TestType,Result,Ith,Po,Vf,Im,Rs,Sen,Res,ICC,Idark,Vbr,Xtalk,Kink_I,TestDate";
    // let stream  = client.query(format!("
    // SELECT {3} FROM {1}MAC_10GBOSADATA  {0} UNION all SELECT {4} FROM {2}MAC_f07959df8122  {0}
    // UNION all SELECT {4} FROM {2}MAC_00e04c686dd1  {0}  UNION all SELECT {4} FROM {2}MAC_00e04c3105fb  {0}
    // UNION all SELECT {4} FROM {2}MAC_70e400a2c0d4  {0}  UNION all SELECT {4} FROM {2}MAC_00262da5e778  {0}
    // UNION all SELECT {4} FROM {2}MAC_408d5cb2d04a  {0}  UNION all SELECT {4} FROM {2}MAC_408d5cb712a3  {0}
    // UNION all SELECT {4} FROM {2}MAC_408d5cb72176  {0}  UNION all SELECT {4} FROM {2}MAC_10634b007c4b  {0}
    // UNION all SELECT {4} FROM {2}MAC_10634b0966f0  {0}  UNION all SELECT {4} FROM {2}MAC_08626683922a  {0}
    // UNION all SELECT {4} FROM {2}MAC_c8e7d8e187a7  {0}  UNION all SELECT {4} FROM {2}MAC_f0b42937e924  {0}
    // UNION all SELECT {4} FROM {2}MAC_fcaa14afca45  {0}  UNION all SELECT {4} FROM {2}MAC_F07959DF968E  {0}
    // UNION all SELECT {4} FROM {2}MAC_f07959df9218  {0}  UNION all SELECT {4} FROM {2}MAC_f07959df9694  {0}
    // UNION all SELECT {4} FROM {2}MAC_f07959dfa742  {0}  UNION all SELECT {4} FROM {2}MAC_fcaa14db2983  {0}  ",
    let stream = client
        .simple_query(format!(
            "SELECT {1} FROM MAC_10GBOSADATA  {0}",
            query_ty, testtype_12
        ))
        .await
        .unwrap();
    let rowsets = stream.into_results().await.unwrap();
    for i in 0..rowsets.len() {
        let rows = rowsets.get(i).unwrap();
        for row in rows {
            let sn = row.get::<&str, _>(0) .unwrap().to_string();
            let workid = row.get::<&str, _>(1) .unwrap().to_string();
            let pn = row.get::<&str, _>(2) .unwrap().to_string();
            let result = row.get::<&str, _>(3) .unwrap().to_string();
            let ith = row.get::<&str, _>(4) .unwrap().to_string();
            let pf = row.get::<&str, _>(5) .unwrap().to_string();
            let vop = row.get::<&str, _>(6) .unwrap().to_string();
            let im = row.get::<&str, _>(7) .unwrap().to_string();
            let rs = row.get::<&str, _>(8) .unwrap().to_string();
            let sen = row.get::<&str, _>(9) .unwrap().to_string();
            let res = row.get::<&str, _>(10) .unwrap().to_string();
            let icc = row.get::<&str, _>(11) .unwrap().to_string();
            let idark = row.get::<&str, _>(12) .unwrap().to_string();
            let vbr = row.get::<&str, _>(13) .unwrap().to_string();
            let ixtalk = row.get::<&str, _>(14) .unwrap().to_string();
            let kink = row.get::<&str, _>(15) .unwrap().to_string();
            let datatime = row.get::<chrono::NaiveDateTime, _>(16)
                    .unwrap()
                    .format("%Y/%m/%d %H:%M:%S").to_string();
            row_data.push(SnDataInfo {
                sn,
                product_bill: workid,
                test_type: pn,
                result: result,
                ith,
                pf,
                vop,
                im,
                rs,
                sen,
                res,
                icc,
                idark,
                vbr,
                ixtalk,
                kink,
                testdate: datatime,
            });
        }
    }
    row_data
}
pub async fn box_work_test(s: String) -> Vec<BoxDataInfo>{
    let mut datas = Vec::new();
    let mut client = client().await;
    let query_ty = format!(
        "where Pack_no = '{}' ORDER BY [CreateTime] DESC OFFSET 0 ROWS ",
        s
    );
    let testtype_none = "Pack_no,Sn,PN,WorkOrder,Creator,CreateTime";
    let stream = client
        .simple_query(
            format!(
                "select {0} from MaterialPackSn {1} ",
                testtype_none, query_ty
            )
        )
        .await
        .unwrap();
    let rowsets = stream.into_results().await.unwrap();
    // let mut rv: Vec<Vec<String>> = vec![];
    for i in 0..rowsets.len() {
        let rows = rowsets.get(i).unwrap();
        for row in rows {
            let pno = row.get::<&str, _>(0).unwrap().to_string();
            let sn = row.get::<&str, _>(1).unwrap().to_string();
            let pn = row.get::<&str, _>(2).unwrap().to_string();
            let workorder = row.get::<&str, _>(3).unwrap().to_string();
            let creator = row.get::<&str, _>(4).unwrap().to_string();
            let createtime = row.get::<chrono::NaiveDateTime, _>(5).unwrap().format("%Y/%m/%d %H:%M:%S").to_string();
            let data = BoxDataInfo { 
                pno, 
                sn, 
                pn, 
                workorder, 
                creator, 
                createtime 
            };
            // let data = DataInfo::new()
            // .set_pno(row.get::<&str, _>(0))
            // .set_sn(row.get::<&str, _>(1))
            // .set_pn(row.get::<&str, _>(2))
            // .set_workorder(row.get::<&str, _>(3))
            // .set_creator(row.get::<&str, _>(4))
            // .set_createtime(Some(row.get::<chrono::NaiveDateTime, _>(5).unwrap().format("%Y/%m/%d %H:%M:%S").to_string().as_str()));
            // let mut v: Vec<String> = vec![];
            // v.push(row.get::<&str, _>(0).unwrap().to_string());
            // v.push(row.get::<&str, _>(1).unwrap().to_string());
            // v.push(row.get::<&str, _>(2).unwrap().to_string());
            // v.push(row.get::<&str, _>(3).unwrap().to_string());
            // v.push(row.get::<&str, _>(4).unwrap().to_string());
            // v.push(row.get::<chrono::NaiveDateTime, _>(5).unwrap().to_string());
            datas.push(data)
        }
    }
    // let quantity = datas.len();
    // (datas, quantity)
    datas
}

pub async fn login_user(id: String, pa: String) -> bool {
    let mut client = login_check().await;
    let i = id.clone();
    let stream = client
        .simple_query(
            format!("select name from login where name = '{}'", i),
        )
        .await
        .unwrap();
    let row = stream.into_results().await.unwrap();
    let r = row.get(0).unwrap();

    if r.is_empty() {
        return false;
    } else {
        let x = login_pass(id, pa).await;
        return x;
    }
}

pub async fn login_pass(id: String, p: String) -> bool {
    let mut client = login_check().await;
    let i = id.clone();
    let stream = client
        .simple_query(
            format!("select password from login where name = '{}'", i),
        )
        .await
        .unwrap();
    let row = stream.into_row().await.unwrap().unwrap();
    let r = row.get::<&str, _>(0).unwrap().to_string();
    if r != p {
        return false;
    } else {
        return true;
    }
}