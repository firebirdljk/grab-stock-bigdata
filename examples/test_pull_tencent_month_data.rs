//use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let  body = reqwest::get("http://data.gtimg.cn/flashdata/hushen/monthly/sz002955.js?maxage=43201")
        .await?
        .text()
        .await?;
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
       // println!("tmpbody={:?}",tmpBody.unwrap());
        pos = tmp_body.unwrap().find("\\\n");
        println!("{:?}", pos);
        if pos.unwrap()==0 {break;}
        date = tmp_body.unwrap().get(pos.unwrap() + 2..pos.unwrap() + 8);
        println!("{:?}", date);
        if date==None {break;}
        //let count = body.chars().count();
        tmp_str = tmp_body.unwrap().get(pos.unwrap() + 1..pos.unwrap() + 40);
        iter = tmp_str.unwrap().split_whitespace();
        tmp_date=iter.next();
        month_start_pri = iter.next();
        month_end_pri = iter.next();
        month_max = iter.next();
        month_min = iter.next();
        println!("{:?}", tmp_date);
        println!("{:?}", month_start_pri);
        println!("{:?}", month_end_pri);
        println!("{:?}", month_max);
        println!("{:?}", month_min);
        tmp_body = tmp_body.unwrap().get(pos.unwrap()+10..);
       // println!("{:?}", tmpBody);
    }
    Ok(())
  //  assert_eq!(body.find("\\\n"),Some(4));
}
