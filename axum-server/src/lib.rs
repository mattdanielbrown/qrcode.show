use axum::{
    Router,
    body::{Body, Bytes},
    extract::FromRequestParts,
    extract::OriginalUri,
    http::StatusCode,
    http::header,
    http::header::HeaderName,
    http::header::HeaderValue,
    response::IntoResponse,
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

enum QRResponse {
    Cors,
    Plain(String),
    Html(String),
    Svg(String),
    Png(Vec<u8>),
    Jpeg(Vec<u8>),
    Unicode(Vec<u8>),
}

fn cors(mut res: Response) -> Response {
    res.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("HEAD, POST, GET, OPTIONS"),
    );

    res.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );

    res.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
    res
}

impl IntoResponse for QRResponse {
    fn into_response(self) -> Response {
        let resp = match self {
            Self::Cors => Response::new(Body::empty()),

            Self::Plain(text) => {
                let mut res = Response::new(Body::from(text));
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain"),
                );

                res
            }

            Self::Svg(svg) => {
                let mut res = Response::new(Body::from(svg));
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("image/svg+xml"),
                );

                res
            }

            Self::Html(html) => {
                let mut res = Response::new(Body::from(html));
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/html"),
                );

                res
            }

            Self::Unicode(data) => {
                let mut res = Response::new(Body::from(data));
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static(
                        "application/octet-stream",
                    ),
                );

                res
            }

            Self::Png(png) => {
                let mut res = Response::new(Body::from(png));
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("image/png"),
                );

                res
            }

            Self::Jpeg(jpeg) => {
                let mut res = Response::new(Body::from(jpeg));
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("image/jpeg"),
                );

                res
            }
        };
        cors(resp)
    }
}

fn generate(
    bytes: &[u8],
    generator: &Generator,
) -> Result<QRResponse, StatusCode> {
    let image = generator
        .generate(bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match generator.format {
        Format::Svg => {
            Ok(QRResponse::Svg(String::from_utf8_lossy(&image).to_string()))
        }

        Format::Png => Ok(QRResponse::Png(image)),

        Format::Jpeg => Ok(QRResponse::Jpeg(image)),

        Format::Html => {
            let html = TEMPLATE
                .replace("{{ content }}", &String::from_utf8_lossy(&image))
                .replace("{{ help }}", &HTML_HELP);
            Ok(QRResponse::Html(html))
        }

        Format::Unicode => Ok(QRResponse::Unicode(image)),

        Format::PlainText => Ok(QRResponse::Plain(
            String::from_utf8_lossy(&image).to_string(),
        )),
    }
}

async fn options_handler() -> Result<QRResponse, StatusCode> {
    Ok(QRResponse::Cors)
}

async fn post_handler(
    OriginalUri(uri): OriginalUri,
    QRGenerator(generator): QRGenerator,
    bytes: Bytes,
) -> Result<QRResponse, StatusCode> {
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
) -> Result<QRResponse, StatusCode> {
    let (_, path) = uri.path().split_once('/').unwrap_or_default();

    if path.is_empty() {
        match generator.format {
            Format::Html => {
                let html = TEMPLATE
                    .replace("{{ content }}", "")
                    .replace("{{ help }}", &HTML_HELP);
                Ok(QRResponse::Html(html))
            }
            Format::PlainText | Format::Unicode => {
                Ok(QRResponse::Plain(HELP.to_string()))
            }
            Format::Jpeg | Format::Png | Format::Svg => {
                Err(StatusCode::BAD_REQUEST)
            }
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
