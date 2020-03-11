use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=firebird_stock", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    for row in client.query("SELECT code, name FROM public.stock_base_info;", &[]).await? {
        let mut col1:&str = row.get(0);
        let mut code=col1.to_string();
        capitalize(&mut code);
        let mut col2:&str=row.get(1);
        let mut name=col2.to_string();
        let mut web_url="http://data.gtimg.cn/flashdata/hushen/weekly/".to_string()+&code;
        web_url.push_str(".js?maxage=43201");
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
        if t_pos>0 {continue;}
        println!("body={:?}",body);
        let mut tmp_body = body.get(0..);
        let mut pos;
        let mut date;
        let mut tmp_str;
        let mut iter;
        let mut tmp_date;
        let mut start_pri;
        let mut end_pri;
        let mut max_pri;
        let mut min_pri;
        loop {
            pos = tmp_body.unwrap().find("\\\n");
            if pos.unwrap()==0 {break;}
            date = tmp_body.unwrap().get(pos.unwrap() + 2..pos.unwrap() + 8);
            if date==None {break;}
            tmp_str = tmp_body.unwrap().get(pos.unwrap() + 1..pos.unwrap() + 40);
            iter = tmp_str.unwrap().split_whitespace();
            tmp_date=iter.next();
            start_pri = iter.next();
            end_pri = iter.next();
            max_pri = iter.next();
            min_pri = iter.next();
            tmp_body = tmp_body.unwrap().get(pos.unwrap()+10..);
            let tmp_code=col1.to_string();
            let tmp_name=col2.to_string();
            let tmp_start_pri=start_pri.unwrap().to_string();
            let tmp_end_pri=end_pri.unwrap().to_string();
            let tmp_max_pri=max_pri.unwrap().to_string();
            let tmp_min_pri=min_pri.unwrap().to_string();
            let tmp_start_pri=tmp_start_pri.parse::<f32>().unwrap();
            let tmp_end_pri=tmp_end_pri.parse::<f32>().unwrap();
            let tmp_max_pri=tmp_max_pri.parse::<f32>().unwrap();
            let tmp_min_pri=tmp_min_pri.parse::<f32>().unwrap();
            let tmp_date=date.unwrap();
            let rows = client
                .query("SELECT code,name FROM weekly_data where code=$1::TEXT and date=$2::TEXT", &[&tmp_code,&tmp_date])
                .await?;
            let count  = rows.len();
            if count.eq(&0) {
                client.execute(
                    "INSERT INTO weekly_data (code,name,\"StartPri\",\"EndPri\",\"MaxPri\",\"MinPri\",date) VALUES ($1, $2,$3,$4,$5,$6,$7)",
                    &[&tmp_code, &tmp_name, &tmp_start_pri, &tmp_end_pri, &tmp_max_pri, &tmp_min_pri, &tmp_date],
                ).await?;
            };
        }
    }

    println!("Yay!");
    Ok(())
}

fn capitalize(s:&mut str){
    s.make_ascii_lowercase();
}