use tokio_postgres::{NoTls, Error};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=firebird_stock", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

  //  let rows = client
  //      .query("SELECT code,name FROM stock_base_info;", &[])
   //     .await?;

    for row in client.query("SELECT code,name FROM stock_base_info;", &[]).unwrap() {
        let code = row.get(0).to_string();
        let name=row.get(0).to_string();
        println!("found person: {}  {:?}", code, name);
    }

    let code="000001";
    let name="abcd";
    let start_pri:f32=32.23;
    let end_pri:f32=23.44;
    let max_pri:f32=31.11;
    let min_pri:f32=11.22;
    let date="201115";
    println!("{} {} {} {}",start_pri,end_pri,max_pri,min_pri);

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT code,name FROM monthly_data where code=$1::TEXT and date=$2::TEXT", &[&code,&date])
        .await?;

    // And then check that we got back the same string we sent over.
    let value  = rows.len();
    println!("{}",value);
    if value.eq(&0) {
        client.execute(
            "INSERT INTO monthly_data (code,name,\"StartPri\",\"EndPri\",\"MaxPri\",\"MinPri\",date) VALUES ($1, $2,$3,$4,$5,$6,$7)",
            &[&code, &name, &start_pri, &end_pri, &max_pri, &min_pri, &date],
        ).await?;
    };

    Ok(())
}