use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Conn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL expected in .env");
    let manager = ConnectionManager::<PgConnection>::new(connspec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Fatal error: Failed to create pool")
}
