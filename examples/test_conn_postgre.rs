extern crate postgres;
use postgres::{Client, NoTls};
static URI:&str =  "host=127.0.0.1 dbname=firebird_stock user=postgres password=postgres port=5432";

fn main() {
    let mut client = Client::connect(URI, NoTls).unwrap();
    for row in client.query("SELECT code, name FROM public.stock_base_info;", &[]).unwrap() {
        let code: &str = row.get(0);
        let name:&str =row.get(1);
        println!("found person: {}  {:?}", code, name);
    }
}