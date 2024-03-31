use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env::var;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn establish_pooled_connection() -> DBPool {
    dotenv().ok();

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    // let conn = pool.clone().get().unwrap();
    return pool;
}

pub struct PgDbPool {
    pub db_pool: DBPool,
}

impl PgDbPool {
    pub fn new() -> Self {
        PgDbPool {
            db_pool: establish_pooled_connection(),
        }
    }

    pub fn get_connection(&self) -> DBConnection {
        self.db_pool.get().unwrap()
    }
}
