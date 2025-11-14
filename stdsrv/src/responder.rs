//! Helper trait(s).

use crate::request::HttpRequest;
use crate::response::HttpResponse;

/// Responder trait. Just a respond method that turns a [`HttpRequest`] to a [`HttpResponse`].
///
/// [`HttpRequest`]: ../request/struct.HttpRequest.html
/// [`HttpResponse`]: ../response/struct.HttpResponse.html
pub trait Responder {
    fn respond(&self, req: HttpRequest) -> HttpResponse;
}
