#[derive(Debug, Clone)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub attempts: u32,
    pub remaining: u32,
    pub retry_after: Option<u64>,
}
