//! Wrapper type of a `HashMap` to contain HTTP headers and format them correctly.

use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct HttpHeaders {
    inner: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        HttpHeaders {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, k: &str, v: &str) {
        self.inner.insert(k.to_string(), v.to_string());
    }

    #[cfg(test)]
    pub fn get(&self, k: &str) -> Option<&String> {
        self.inner.get(k)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.inner {
            write!(f, "{k}: {v}\r\n")?;
        }
        Ok(())
    }
}
