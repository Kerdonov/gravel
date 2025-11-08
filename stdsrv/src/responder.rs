//! Traits helping HTTP connections

use crate::request::HttpRequest;
use crate::response::HttpResponse;

/// Responder trait. Just a respond method that turns a HttpRequest to a HttpResponse.
pub trait Responder {
    fn respond(&self, req: HttpRequest) -> HttpResponse;
}

/*
/// Size trait. Number of bytes when encoded.
pub trait Size {
    fn size(&self) -> usize;
}

// Standard implementations for Size trait

impl Size for u8 {
    fn size(&self) -> usize {
        1
    }
}

impl<T> Size for Vec<T>
where
    T: Size,
{
    fn size(&self) -> usize {
        if let Some(elem) = self.first() {
            elem.size() * self.len()
        } else {
            0
        }
    }
}

impl<T> Size for Option<T>
where
    T: Size,
{
    fn size(&self) -> usize {
        match self {
            Some(t) => t.size(),
            None => 0,
        }
    }
}

impl Size for String {
    fn size(&self) -> usize {
        self.len()
    }
}
*/
