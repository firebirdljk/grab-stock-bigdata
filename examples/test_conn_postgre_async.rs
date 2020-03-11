use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main()-> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=firebird_stock", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    for row in client.query("SELECT code, name FROM public.stock_base_info", &[]).await? {
        let code: &str = row.get(0);
        let name:&str =row.get(1);
        println!("found stock: {}  {:?}", code, name);
    }
    Ok(())
}