use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct StockBaseData {
    code: Option<String>,
    name: Option<String>,
}

fn main() {
    let url = "mysql://root:root@127.0.0.1:3306/firebird_stock";
    let pool = Pool::new(url).unwrap();
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
        println!("{:?},{:?}",i.code,i.name);
    }

    println!("Yay!");
}