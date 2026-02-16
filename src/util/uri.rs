use headers_core::Error;
use http::{HeaderValue, Uri};

pub trait UriExt: Sized {
    fn to_uri(&self) -> Result<Uri, Error>;

    fn from_uri(uri: &Uri) -> Option<Self>;
}

impl UriExt for HeaderValue {
    fn to_uri(&self) -> Result<Uri, Error> {
        self.to_str()
            .map_err(|_| Error::invalid())
            .and_then(|s| s.parse().map_err(|_| Error::invalid()))
    }

    fn from_uri(uri: &Uri) -> Option<Self> {
        HeaderValue::try_from(uri.to_string()).ok()
    }
}
