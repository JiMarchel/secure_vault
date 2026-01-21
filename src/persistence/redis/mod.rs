pub mod rate_limiter;
pub mod token_store;
pub mod otp;

use redis::aio::ConnectionManager;

pub struct RedisPersistence {
    conn: ConnectionManager,
}

impl RedisPersistence {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    pub fn conn(&self) -> ConnectionManager {
        self.conn.clone()
    }
}
