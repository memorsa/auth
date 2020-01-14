use std::io;
use std::sync::Arc;

use actix::prelude::*;
use bytes::{BufMut, Bytes, BytesMut};
use futures::FutureExt;
use tokio_postgres::{connect, Client, NoTls};

/// Postgres interface
pub struct PgConnection {
    cl: Arc<Client>,
}

pub struct RandomWorld;

impl Message for RandomWorld {
    type Result = io::Result<Bytes>;
}

impl Handler<RandomWorld> for PgConnection {
    type Result = ResponseFuture<Result<Bytes, io::Error>>;

    fn handle(&mut self, _: RandomWorld, _: &mut Self::Context) -> Self::Result {
        let cl = self.cl.clone();
        Box::pin(async move {
            let fut = cl.query("SELECT $1::TEXT", &[&"hello world Actor"]);
            let rows = fut
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
            let value: &str = rows[0].get(0);

            let mut body = BytesMut::with_capacity(40);

            body.put(value.clone().as_bytes());

            Ok(body.freeze())
        })
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

        Ok(PgConnection::create(move |_| PgConnection {
            cl: Arc::new(cl),
        }))
    }
}
