use hyper::header::Host;
use hyper::status::StatusCode;

pub fn get_name_by_http_code(code: u16) -> Option<&'static str> {
    get_status_from_code(code).canonical_reason()
}

pub fn get_content_type(mimetype: &str, charset: &str) -> String {
    if (mimetype.starts_with("text/") || (mimetype == "application/xml") ||
       (mimetype.starts_with("application/") && mimetype.ends_with("+xml"))) &&
       !mimetype.contains("charset") {
        mimetype.to_string() + "; charset=" + charset
    } else {
        mimetype.to_string()
    }
}

pub fn get_host_value(host: &Host) -> String {
    match host.port {
        None | Some(80) | Some(443) => host.hostname.clone(),
        Some(port) => format!("{}:{}", host.hostname, port),
    }
}

pub fn get_status_from_code(code: u16) -> StatusCode {
    match code {
        100 => StatusCode::Continue,
        101 => StatusCode::SwitchingProtocols,
        102 => StatusCode::Processing,
        200 => StatusCode::Ok,
        201 => StatusCode::Created,
        202 => StatusCode::Accepted,
        203 => StatusCode::NonAuthoritativeInformation,
        204 => StatusCode::NoContent,
        205 => StatusCode::ResetContent,
        206 => StatusCode::PartialContent,
        207 => StatusCode::MultiStatus,
        208 => StatusCode::AlreadyReported,
        226 => StatusCode::ImUsed,
        300 => StatusCode::MultipleChoices,
        301 => StatusCode::MovedPermanently,
        302 => StatusCode::Found,
        303 => StatusCode::SeeOther,
        304 => StatusCode::NotModified,
        305 => StatusCode::UseProxy,
        307 => StatusCode::TemporaryRedirect,
        308 => StatusCode::PermanentRedirect,
        400 => StatusCode::BadRequest,
        401 => StatusCode::Unauthorized,
        402 => StatusCode::PaymentRequired,
        403 => StatusCode::Forbidden,
        404 => StatusCode::NotFound,
        405 => StatusCode::MethodNotAllowed,
        406 => StatusCode::NotAcceptable,
        407 => StatusCode::ProxyAuthenticationRequired,
        408 => StatusCode::RequestTimeout,
        409 => StatusCode::Conflict,
        410 => StatusCode::Gone,
        411 => StatusCode::LengthRequired,
        412 => StatusCode::PreconditionFailed,
        413 => StatusCode::PayloadTooLarge,
        414 => StatusCode::UriTooLong,
        415 => StatusCode::UnsupportedMediaType,
        416 => StatusCode::RangeNotSatisfiable,
        417 => StatusCode::ExpectationFailed,
        418 => StatusCode::ImATeapot,
        422 => StatusCode::UnprocessableEntity,
        423 => StatusCode::Locked,
        424 => StatusCode::FailedDependency,
        426 => StatusCode::UpgradeRequired,
        428 => StatusCode::PreconditionRequired,
        429 => StatusCode::TooManyRequests,
        431 => StatusCode::RequestHeaderFieldsTooLarge,
        500 => StatusCode::InternalServerError,
        501 => StatusCode::NotImplemented,
        502 => StatusCode::BadGateway,
        503 => StatusCode::ServiceUnavailable,
        504 => StatusCode::GatewayTimeout,
        505 => StatusCode::HttpVersionNotSupported,
        506 => StatusCode::VariantAlsoNegotiates,
        507 => StatusCode::InsufficientStorage,
        508 => StatusCode::LoopDetected,
        510 => StatusCode::NotExtended,
        511 => StatusCode::NetworkAuthenticationRequired,
        _ => StatusCode::Unregistered(code),
    }
}
