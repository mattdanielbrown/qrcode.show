mod core;

pub use core::Format;
pub use core::Generator;
pub use core::VersionType;
pub use qrcode::EcLevel;
pub use qrcode::QrCode;
pub use qrcode::QrResult;
pub use qrcode::Version;

pub const TEMPLATE: &str = include_str!("../../templates/base.html");
pub const HELP: &str = include_str!("../../README.txt");
pub const HTML_HELP: &str = include_str!("./qrcode.show.html");
