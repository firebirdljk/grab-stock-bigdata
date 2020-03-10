use crate::utils::db_source::init_mysql;
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StockBaseData {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[allow(dead_code)]
pub fn select() {
    let pool = init_mysql();
    let mut conn = pool.get_conn().unwrap();
    // Let's select payments from database. Type inference should do the trick here.
    let selected_stock_bases = conn
        .query_map(
            "SELECT code, name from stock_base_info",
            |(code, name)| {
                StockBaseData { code, name }
            },
        ).unwrap();

   // for i in selected_stock_bases{
   //     println!("{:?},{:?}",i.code,i.name);
   // }
    selected_stock_bases
}