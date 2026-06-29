use csscolorparser::Color;
use image::ImageFormat;
use image::Rgba;
use qrcode::EcLevel;
use qrcode::QrCode;
use qrcode::QrResult;
use qrcode::Version;
use qrcode::render::svg;
use qrcode::render::unicode;
use qrcode::types::QrError;
use std::io::Cursor;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Svg,
    Html,
    Unicode,
    PlainText,
    Png,
    Jpeg,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    OpenExr,
    Farbfeld,
    Avif,
    Qoi,
}

impl Default for Format {
    fn default() -> Self {
        Self::Unicode
    }
}

impl From<&str> for Format {
    fn from(headerval: &str) -> Self {
        match headerval.to_lowercase().as_str() {
            "text/html" => Self::Html,
            "image/svg+xml" => Self::Svg,
            "text/plain" => Self::PlainText,
            "image/png" => Self::Png,
            "image/jpeg" | "image/jpg" => Self::Jpeg,
            "image/gif" => Self::Gif,
            "image/webp" => Self::WebP,
            "image/x-portable-anymap" => Self::Pnm,
            "image/tiff" => Self::Tiff,
            "image/x-tga" => Self::Tga,
            // "image/vnd.ms-dds" => Self::Dds,
            "image/bmp" => Self::Bmp,
            // "image/vnd.microsoft.icon" => Self::Ico,
            // "image/vnd.radiance" => Self::Hdr,
            // "image/x-exr" => Self::OpenExr,
            // "image/farbfeld" => Self::Farbfeld,
            // "image/avif" => Self::Avif,
            "image/qoi" => Self::Qoi,
            _ => Self::default(),
        }
    }
}

impl Format {
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Svg => "image/svg+xml",
            Self::Html => "text/html",
            Self::PlainText => "text/plain",
            Self::Unicode => "text/plain",
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::WebP => "image/webp",
            Self::Pnm => "image/x-portable-anymap",
            Self::Tiff => "image/tiff",
            Self::Tga => "image/x-tga",
            Self::Dds => "image/vnd.ms-dds",
            Self::Bmp => "image/bmp",
            Self::Ico => "image/vnd.microsoft.icon",
            Self::Hdr => "image/vnd.radiance",
            Self::OpenExr => "image/x-exr",
            Self::Farbfeld => "image/farbfeld",
            Self::Avif => "image/avif",
            Self::Qoi => "image/qoi",
        }
    }
}

impl From<&String> for Format {
    fn from(headerval: &String) -> Self {
        headerval.as_str().into()
    }
}

impl From<Format> for ImageFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Png => Self::Png,
            Format::Jpeg => Self::Jpeg,
            Format::Gif => Self::Gif,
            Format::WebP => Self::WebP,
            Format::Pnm => Self::Pnm,
            Format::Tiff => Self::Tiff,
            Format::Tga => Self::Tga,
            Format::Dds => Self::Dds,
            Format::Bmp => Self::Bmp,
            Format::Ico => Self::Ico,
            Format::Hdr => Self::Hdr,
            Format::OpenExr => Self::OpenExr,
            Format::Farbfeld => Self::Farbfeld,
            Format::Avif => Self::Avif,
            Format::Qoi => Self::Qoi,
            _ => Self::Png, // Default to PNG for unsupported formats
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VersionType {
    NormalVersion,
    MicroVersion,
}

impl Default for VersionType {
    fn default() -> Self {
        Self::MicroVersion
    }
}

impl From<&str> for VersionType {
    fn from(string: &str) -> Self {
        match string {
            "n" | "normal" => Self::NormalVersion,
            "m" | "micro" => Self::MicroVersion,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Generator {
    pub format: Format,

    pub width: Option<u32>,
    pub height: Option<u32>,

    pub min_width: Option<u32>,
    pub min_height: Option<u32>,

    pub max_width: Option<u32>,
    pub max_height: Option<u32>,

    pub dark_color: Option<String>,
    pub light_color: Option<String>,

    pub version_type: VersionType,
    pub version_number: Option<i16>,

    pub error_correction_level: Option<EcLevel>,

    pub quiet_zone: Option<bool>,
}

impl Generator {
    pub fn generate(&self, input: &[u8]) -> QrResult<Vec<u8>> {
        use EcLevel::*;
        use Version::*;
        use VersionType::*;

        let code = match (
            self.version_type,
            self.version_number,
            self.error_correction_level,
        ) {
            (MicroVersion, Some(n), Some(e)) => {
                QrCode::with_version(input, Micro(n), e)
            }
            (MicroVersion, Some(n), None) => {
                QrCode::with_version(input, Micro(n), L)
            }
            (NormalVersion, Some(n), Some(e)) => {
                QrCode::with_version(input, Normal(n), e)
            }
            (NormalVersion, Some(n), None) => {
                QrCode::with_version(input, Normal(n), L)
            }
            (_, _, Some(e)) => QrCode::with_error_correction_level(input, e),
            (_, _, _) => QrCode::new(input),
        };

        let code = code?;

        let min_height = self.height.or(self.min_height).unwrap_or_else(|| {
            if matches!(self.format, Format::Unicode) {
                20
            } else {
                360
            }
        });
        let min_width = self.width.or(self.min_width).unwrap_or_else(|| {
            if matches!(self.format, Format::Unicode) {
                20
            } else {
                360
            }
        });

        let max_height = self
            .height
            .or(self.max_height)
            .unwrap_or_default()
            .max(min_height);

        let max_width = self
            .width
            .or(self.max_width)
            .unwrap_or_default()
            .max(min_width);

        let image = match self.format {
            Format::Svg | Format::Html => {
                let mut bytes = code
                    .render()
                    .min_dimensions(min_width, min_height)
                    .max_dimensions(max_width, max_height)
                    .dark_color(svg::Color(
                        self.dark_color.as_deref().unwrap_or("#000"),
                    ))
                    .light_color(svg::Color(
                        self.light_color.as_deref().unwrap_or("#fff"),
                    ))
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build()
                    .into_bytes();
                bytes.push(b'\n');
                bytes
            }

            Format::PlainText => {
                let mut bytes = code
                    .render::<char>()
                    .module_dimensions(2, 1)
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build()
                    .into_bytes();
                bytes.push(b'\n');
                bytes
            }

            Format::Unicode => {
                let mut bytes = code
                    .render::<unicode::Dense1x2>()
                    .min_dimensions(min_width, min_height)
                    .max_dimensions(max_width, max_height)
                    .dark_color(unicode::Dense1x2::Light)
                    .light_color(unicode::Dense1x2::Dark)
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build()
                    .into_bytes();
                bytes.push(b'\n');
                bytes
            }

            Format::Png
            | Format::Jpeg
            | Format::Gif
            | Format::WebP
            | Format::Pnm
            | Format::Tiff
            | Format::Tga
            | Format::Dds
            | Format::Bmp
            | Format::Ico
            | Format::Hdr
            | Format::OpenExr
            | Format::Farbfeld
            | Format::Avif
            | Format::Qoi => {
                let [dr, dg, db, da] = self
                    .dark_color
                    .as_deref()
                    .unwrap_or("#000")
                    .parse::<Color>()
                    .unwrap_or(Color::from_rgba8(0, 0, 0, 0))
                    .to_linear_rgba_u8();

                let [lr, lg, lb, la] = self
                    .light_color
                    .as_deref()
                    .unwrap_or("#fff")
                    .parse::<Color>()
                    .unwrap_or(Color::from_rgba8(255, 255, 255, 0))
                    .to_linear_rgba_u8();

                let image = code
                    .render::<Rgba<u8>>()
                    .dark_color(Rgba([dr, dg, db, da]))
                    .light_color(Rgba([lr, lg, lb, la]))
                    .min_dimensions(min_width, min_height)
                    .max_dimensions(max_width, max_height)
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build();

                let mut result = vec![];

                image
                    .write_to(&mut Cursor::new(&mut result), self.format.into())
                    .map_err(|_| QrError::UnsupportedCharacterSet)?;

                result
            }
        };

        Ok(image)
    }
}
