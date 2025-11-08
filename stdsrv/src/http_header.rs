use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct HttpHeaders {
    _inner: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        HttpHeaders {
            _inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, k: &str, v: &str) {
        self._inner.insert(k.to_string(), v.to_string());
    }

    #[cfg(test)]
    pub fn get(&self, k: &str) -> Option<&String> {
        self._inner.get(k)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self._inner.len()
    }
}

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self._inner.iter() {
            write!(f, "{}: {}\r\n", k, v)?;
        }
        Ok(())
    }
}
