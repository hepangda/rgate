
enum HttpFlags {
    Nothing,
    KeepAlive,
}

enum HttpParserState {
    Begin,

    MethodOptions,
    MethodHead,
    MethodGet,
    MethodPost,
    MethodAlmostDone,
    MethodDone,

    UriStart,
    Uri,
    UriDone,

    VerHttpSlash,
    VerMajor,
    VerDot,
    VerMinor,
    VerEnd,

    Cr,
    CrLf,
    CrLfCr,

    Header,
    Colon,
    Value,
    Died,
}
enum HttpParserErrors {
    Fine,
    NotAvailable,

    Parsed,
    InvalidMethod,
    InvalidUri,
    InvalidVersion,
    InvalidContentLength,
    UnsupportedVersion,
    CrLf,
    InvalidHeader,
    UnrecognizedChar,
    UnexpectedEnd,
    EntityTooLarge,
    Other,
}

struct HttpParser<'a> {
    stream: &'a String,
    request: &'a String,
    field: &'a String,
}

impl<'a> HttpParser<'a> {
    pub fn from_string(s: &'a String) -> HttpParser<'a> {
        HttpParser {
            stream: s,
            request: s,
            field: s,
        }
    }
}