use tokio_postgres::{connect as db_connect, Client, Error, NoTls};

pub async fn connect(db_url: &str) -> Result<Client, Error> {
    let (cl, conn) = db_connect(db_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(cl)
}
