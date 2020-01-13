use std::io;

use actix::prelude::*;
use futures::FutureExt;
use tokio_postgres::{connect, Client, NoTls};

/// Postgres interface
pub struct PgConnection {
    cl: Client,
}

pub struct RandomWorld;

impl Message for RandomWorld {
    type Result = String;
}

impl Handler<RandomWorld> for PgConnection {
    type Result = String;

    fn handle(&mut self, _: RandomWorld, _: &mut Self::Context) -> Self::Result {
        "Hello Tokio Postgres".to_string()
    }
}

impl Actor for PgConnection {
    type Context = Context<Self>;
}

impl PgConnection {
    pub async fn connect(db_url: &str) -> Result<Addr<PgConnection>, io::Error> {
        let (cl, conn) = connect(db_url, NoTls)
            .await
            .expect("can not connect to postgresql");
        actix_rt::spawn(conn.map(|_| ()));

        Ok(PgConnection::create(move |_| PgConnection { cl }))
    }
}
