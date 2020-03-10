use crate::utils::db_source::init_mysql;
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MonthlyData {
    pub code:Option<String>,
    pub name:Option<String>,
    pub month_start_pri:Option<String>,
    pub month_end_pri:Option<String>,
    pub month_max:Option<String>,
    pub month_min:Option<String>,
    pub date:Option<String>,
}

#[allow(dead_code)]
pub fn save(data: MonthlyData) -> u64 {
    //print!("save start");
    let pool = init_mysql();
    let mut conn = pool.get_conn().unwrap();
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
    let sql = "INSERT INTO firebird_stock.monthly_data(code,name,monthStartPri,monthEndPri,monthMax,monthMin,date) VALUES (?,?,?,?,?,?,?)";
    //tx.exec_drop("INSERT INTO tmp (a) VALUES (?)", ("foo",))?;
   // println!("code=>{:?}",data.code);
    let selSql:String="SELECT code from firebird_stock.monthly_data where code=".to_string();
    let code:String=data.code.unwrap();
    let date:String=data.date.unwrap();
    let mid:String=" and date=".to_string();
    let querySql=selSql+&code+&mid+&date;
    println!("{}",querySql);
    let val: Option<String> = tx.query_first(querySql).unwrap();
    if val.is_none() {
        tx.exec_drop(sql, (code, data.name.unwrap(), data.month_start_pri.unwrap(),
                               data.month_end_pri.unwrap(), data.month_max.unwrap(), data.month_min.unwrap(),
                               date, )).unwrap();
        tx.commit();
        // let result = p.prep_exec(sql, (monthly_data.code,monthly_data.name,monthly_data.month_start_pri,monthly_data.month_end_pri,
        //  monthly_data.month_max,monthly_data.month_min,monthly_data.date, )).unwrap().last_insert_id();
        1
    }else{
        0
    }
}