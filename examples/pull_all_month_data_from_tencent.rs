use mysql::prelude::* ;
use mysql::TxOpts as mto;

#[derive(Debug, PartialEq, Eq)]
struct StockBaseData {
    code: Option<String>,
    name: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct MonthlyData {
     code:Option<String>,
     name:Option<String>,
     month_start_pri:Option<String>,
     month_end_pri:Option<String>,
     month_max:Option<String>,
     month_min:Option<String>,
     date:Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let url = "mysql://root:root@127.0.0.1:3306/firebird_stock";
    let pool = mysql::Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Let's select payments from database. Type inference should do the trick here.
    let selected_stock_bases = conn
        .query_map(
            "SELECT code, name from stock_base_info",
            |(code, name)| {
                StockBaseData { code, name }
            },
        ).unwrap();
    for i in selected_stock_bases{
        //println!("{:?},{:?}",i.code.unwrap(),i.name);
        let mut code:String=i.code.unwrap();
        capitalize(&mut code);
        let mut name:String=i.name.unwrap();
        let end_Url=".js?maxage=43201".to_string();
        let mut web_url="http://data.gtimg.cn/flashdata/hushen/monthly/".to_string()+&code+&end_Url;
        println!("{}",web_url);
        let body = reqwest::get(&web_url)
            .await?
            .text()
            .await?;
        let mut t_body = body.get(0..);
        let mut t_pos;
        match t_body.unwrap().find("404 Not Found") {
            None => t_pos=0,
            Some(i) => t_pos=i,
        }
        println!("{:?}",t_pos);
        if t_pos>0 {continue;}
        println!("body={:?}",body);
        let mut tmp_body = body.get(0..);
        let mut pos;
        let mut date;
        let mut tmp_str;
        let mut iter;
        let mut tmp_date;
        let mut month_start_pri;
        let mut month_end_pri;
        let mut month_max;
        let mut month_min;
        loop {
            pos = tmp_body.unwrap().find("\\\n");
            if pos.unwrap()==0 {break;}
            date = tmp_body.unwrap().get(pos.unwrap() + 2..pos.unwrap() + 8);
            if date==None {break;}
            tmp_str = tmp_body.unwrap().get(pos.unwrap() + 1..pos.unwrap() + 40);
            iter = tmp_str.unwrap().split_whitespace();
            tmp_date=iter.next();
            month_start_pri = iter.next();
            month_end_pri = iter.next();
            month_max = iter.next();
            month_min = iter.next();
            tmp_body = tmp_body.unwrap().get(pos.unwrap()+10..);
            let tmp_code=&code;
            let tmp_name=&name;
            let data=MonthlyData{code:Some(tmp_code.into()),name:Some(tmp_name.into()),month_start_pri:Some(month_start_pri.unwrap().to_string().into()),
                month_end_pri: Some(month_end_pri.unwrap().to_string().into()),
                month_max:Some(month_max.unwrap().to_string().into()),
                month_min:Some(month_min.unwrap().to_string().into()),date:Some(date.unwrap().to_string().into())};
            //save_monthly_data(data);

            let mut tx = conn.start_transaction(mto::default()).unwrap();
            let sql = "INSERT INTO firebird_stock.monthly_data(code,name,monthStartPri,monthEndPri,monthMax,monthMin,date) VALUES (?,?,?,?,?,?,?)";
            let sel_Sql:String="SELECT code from firebird_stock.monthly_data where code='".to_string();
            let stock_code:String=data.code.unwrap();
            let stock_date:String=data.date.unwrap();
            let mid:String="' and date=".to_string();
            let query_Sql=sel_Sql+&stock_code+&mid+&stock_date;
            println!("{:?}",query_Sql);
            let val: Option<String> = tx.query_first(query_Sql).unwrap();
            let mut is_ok;
            match val {
                None => is_ok=0,
                Some(i) => is_ok=1,
            }
            if is_ok==0 {
                tx.exec_drop(sql, (stock_code, data.name.unwrap(), data.month_start_pri.unwrap(),
                                   data.month_end_pri.unwrap(), data.month_max.unwrap(), data.month_min.unwrap(),
                                   stock_date, )).unwrap();
                tx.commit();
            }
        }
    }

    println!("Yay!");
    Ok(())
}

fn capitalize(s:&mut str){
    s.make_ascii_lowercase();
}