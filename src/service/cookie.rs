use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
use chrono::Duration;

pub struct CookieBuilder {
    name: String,
    value: String,
    max_age: Duration,
    http_only: bool,
    secure: bool,
    same_site: SameSite,
    path: String,
}

pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl CookieBuilder {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            max_age: Duration::hours(1),
            http_only: true,
            secure: true,
            same_site: SameSite::Lax,
            path: "/".to_string(),
        }
    }

    pub fn max_age(mut self, duration: Duration) -> Self {
        self.max_age = duration;
        self
    }

    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = same_site;
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn build(self) -> String {
        let same_site_str = match self.same_site {
            SameSite::Strict => "Strict",
            SameSite::Lax => "Lax",
            SameSite::None => "None",
        };

        format!(
            "{}={}; Path={}; Max-Age={}; SameSite={}{}{}",
            self.name,
            self.value,
            self.path,
            self.max_age.num_seconds(),
            same_site_str,
            if self.http_only { "; HttpOnly" } else { "" },
            if self.secure { "; Secure" } else { "" }
        )
    }
}

pub fn set_cookie(headers: &mut HeaderMap, cookie: String) {
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        headers.append(SET_COOKIE, value);
    }
}

pub fn delete_cookie(name: &str) -> String {
    format!(
        "{}=; Path=/; Max-Age=0; HttpOnly; Secure; SameSite=Lax",
        name
    )
}