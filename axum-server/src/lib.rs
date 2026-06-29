use axum::{
    Router,
    body::{Body, Bytes},
    extract::FromRequestParts,
    extract::OriginalUri,
    http::StatusCode,
    http::header,
    http::header::HeaderName,
    response::Response,
    routing::get,
};

use libs::EcLevel;
use libs::Format;
use libs::Generator;
use libs::HELP;
use libs::HTML_HELP;
use libs::TEMPLATE;

fn get_first_header_value(
    headers: &header::HeaderMap,
    key: header::HeaderName,
) -> Option<String> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.split(';').next())
        .map(String::from)
}

struct QRGenerator(Generator);

impl<S> FromRequestParts<S> for QRGenerator
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let mut generator = Generator::default();

        if let Some(val) =
            get_first_header_value(&parts.headers, header::ACCEPT)
        {
            generator.format = Format::from(&val);
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-width"),
        ) {
            generator.width =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-height"),
        ) {
            generator.height =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-min-width"),
        ) {
            generator.min_width =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-min-height"),
        ) {
            generator.min_height =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-max-width"),
        ) {
            generator.max_width =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-max-height"),
        ) {
            generator.max_height =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-dark-color"),
        ) {
            generator.dark_color = Some(format!("#{}", val));
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-light-color"),
        ) {
            generator.light_color = Some(format!("#{}", val));
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-version-type"),
        ) {
            generator.version_type = val.as_str().into();
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-version-number"),
        ) {
            generator.version_number =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-ec-level"),
        ) {
            generator.error_correction_level = match val.as_str() {
                "L" => Ok(Some(EcLevel::L)),
                "M" => Ok(Some(EcLevel::M)),
                "Q" => Ok(Some(EcLevel::Q)),
                "H" => Ok(Some(EcLevel::H)),
                _ => Err(StatusCode::BAD_REQUEST),
            }?;
        }

        if let Some(val) = get_first_header_value(
            &parts.headers,
            HeaderName::from_static("x-qr-quiet-zone"),
        ) {
            generator.quiet_zone =
                val.parse().map(Some).map_err(|_| StatusCode::BAD_REQUEST)?;
        }

        Ok(QRGenerator(generator))
    }
}

fn generate(
    bytes: &[u8],
    generator: &Generator,
) -> Result<Response, StatusCode> {
    let image = generator
        .generate(bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let body = Body::from(image);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, generator.format.content_type())
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn options_handler() -> Result<Response, StatusCode> {
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            "HEAD, POST, GET, OPTIONS",
        )
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .body(Body::empty())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(resp)
}

async fn post_handler(
    OriginalUri(uri): OriginalUri,
    QRGenerator(generator): QRGenerator,
    bytes: Bytes,
) -> Result<Response, StatusCode> {
    let (_, path) = uri.path().split_once('/').unwrap_or_default();
    if !path.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    };

    if bytes.is_empty() {
        Err(StatusCode::BAD_REQUEST)
    } else {
        generate(&bytes, &generator)
    }
}

async fn get_handler(
    OriginalUri(uri): OriginalUri,
    QRGenerator(generator): QRGenerator,
) -> Result<Response, StatusCode> {
    let (_, path) = uri.path().split_once('/').unwrap_or_default();

    if path.is_empty() {
        match generator.format {
            Format::Html => {
                let html = TEMPLATE
                    .trim()
                    .replace("{{ content }}", "")
                    .replace("{{ help }}", &HTML_HELP.trim());
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body(Body::from(html))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
            }
            Format::PlainText | Format::Unicode => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
                .body(Body::from(HELP.trim()))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
            _ => Err(StatusCode::BAD_REQUEST),
        }
    } else {
        let input = uri
            .query()
            .map(|q| format!("{}?{}", path, q))
            .unwrap_or_else(|| path.to_string());

        let bytes = input.as_bytes();
        generate(bytes, &generator)
    }
}

pub fn router() -> Router {
    Router::new()
        .fallback(get(get_handler).post(post_handler).options(options_handler))
}
