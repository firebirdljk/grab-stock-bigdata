use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct MonthData {
    code: Option<String>,
    name: Option<String>,
    month_start_pri:Option<String>,
}

fn main() {
    let url = "mysql://root:root@127.0.0.1:3306/firebird_stock";
    //let pool = Pool::new(url)?;
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let datas = vec![
        MonthData { code: Some("002899".into()),  name: None ,month_start_pri:Some("32.33".into())},
        MonthData { code: Some("639821".into()),  name: Some("foo".into()),month_start_pri:Some("35.00".into()) },
    ];
    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO monthly_data (code, name,monthStartPri)
      VALUES (:code,  :name,:month_start_pri)",
        datas.iter().map(|p| params! {
        "code" => &p.code,
        "name" => &p.name,
        "month_start_pri"=>&p.month_start_pri,
    })
    ).unwrap();

    println!("Yay!");
}