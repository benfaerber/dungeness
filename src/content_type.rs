use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum ContentType {
    TextCss,
    TextCsv,
    TextHtml,
    TextJavascript,
    TextPlain,
    TextXml,

    ApplicationJavaArchive,
    ApplicationEdiX12,
    ApplicationEdiFact,
    ApplicationJavascript,
    ApplicationOctetStream,
    ApplicationOgg,
    ApplicationPdf,
    ApplicationXhtmlXml,
    ApplicationXShockwaveFlash,
    ApplicationJson,
    ApplicationLdJson,
    ApplicationXml,
    ApplicationZip,
    ApplicationXWwwFormUrlencoded,

    AudioMpeg,
    AudioXMsWma,
    AudioVndRnRealAudio,
    AudioXWav,

    ImageGif,
    ImageJpeg,
    ImagePng,
    ImageTiff,
    ImageVndMicrosoftIcon,
    ImageXIcon,
    ImageVndDjvu,
    ImageSvgXml,

    MultipartMixed,
    MultipartAlternative,
    MultipartRelated,
    MultipartFormData,

    VideoMpeg,
    VideoMp4,
    VideoQuicktime,
    VideoXMsWmv,
    VideoMsvideo,
    VideoXFlv,
    VideoWebm,

    ApplicationVndAndroidPackageArchive,
    ApplicationVndOasisOpenDocumentText,
    ApplicationVndOasisOpenDocumentSpreadsheet,
    ApplicationVndOasisOpenDocumentPresentation,
    ApplicationVndOasisOpenDocumentGraphics,
    ApplicationVndMsExcel,
    ApplicationVndOpenXmlFormatsOfficeDocumentSpreadsheetmlSheet,
    ApplicationVndMsPowerpoint,
    ApplicationVndOpenXmlFormatsOfficeDocumentPresentationmlPresentation,
    ApplicationMsWord,
    ApplicationVndOpenXmlFormatsOfficeDocumentWordProcessingmlDocument,
    ApplicationVndMozillaAulXml,
}

impl ContentType {
    pub fn to_string(&self) -> String {
        match self {
            Self::TextCss => "text/css",
            Self::TextCsv => "text/csv",
            Self::TextHtml => "text/html",
            Self::TextJavascript => "text/javascript",
            Self::TextPlain => "text/plain",
            Self::TextXml => "text/xml",

            Self::ApplicationJavaArchive => "application/java-archive",
            Self::ApplicationEdiX12 => "application/EDI-X12",
            Self::ApplicationEdiFact => "application/EDIFACT",
            Self::ApplicationJavascript => "application/javascript",
            Self::ApplicationOctetStream => "application/octet-stream",
            Self::ApplicationOgg => "application/ogg",
            Self::ApplicationPdf => "application/pdf",
            Self::ApplicationXhtmlXml => "application/xhtml+xml",
            Self::ApplicationXShockwaveFlash => "application/x-shockwave-flash",
            Self::ApplicationJson => "application/json",
            Self::ApplicationLdJson => "application/ld+json",
            Self::ApplicationXml => "application/xml",
            Self::ApplicationZip => "application/zip",
            Self::ApplicationXWwwFormUrlencoded => "application/x-www-form-urlencoded",

            Self::AudioMpeg => "audio/mpeg",
            Self::AudioXMsWma => "audio/x-ms-wma",
            Self::AudioVndRnRealAudio => "audio/vnd.rn-realaudio",
            Self::AudioXWav => "audio/x-wav",

            Self::ImageGif => "image/gif",
            Self::ImageJpeg => "image/jpeg",
            Self::ImagePng => "image/png",
            Self::ImageTiff => "image/tiff",
            Self::ImageVndMicrosoftIcon => "image/vnd.microsoft.icon",
            Self::ImageXIcon => "image/x-icon",
            Self::ImageVndDjvu => "image/vnd.djvu",
            Self::ImageSvgXml => "image/svg+xml",

            Self::MultipartMixed => "multipart/mixed ",
            Self::MultipartAlternative => "multipart/alternative",
            Self::MultipartRelated => "multipart/related",
            Self::MultipartFormData => "multipart/form-data",

            Self::VideoMpeg => "video/mpeg",
            Self::VideoMp4 => "video/mp4",
            Self::VideoQuicktime => "video/quicktime",
            Self::VideoXMsWmv => "video/x-ms-wmv",
            Self::VideoMsvideo => "video/x-msvideo",
            Self::VideoXFlv => "video/x-flv",
            Self::VideoWebm => "video/webm",

            Self::ApplicationVndAndroidPackageArchive => "application/vnd.android.package-archive",
            Self::ApplicationVndOasisOpenDocumentText => "application/vnd.oasis.opendocument.text",
            Self::ApplicationVndOasisOpenDocumentSpreadsheet => {
                "application/vnd.oasis.opendocument.spreadsheet"
            }
            Self::ApplicationVndOasisOpenDocumentPresentation => {
                "application/vnd.oasis.opendocument.presentation"
            }
            Self::ApplicationVndOasisOpenDocumentGraphics => {
                "application/vnd.oasis.opendocument.graphics"
            }
            Self::ApplicationVndMsExcel => "application/vnd.ms-excel",
            Self::ApplicationVndOpenXmlFormatsOfficeDocumentSpreadsheetmlSheet => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            Self::ApplicationVndMsPowerpoint => "application/vnd.ms-powerpoint",
            Self::ApplicationVndOpenXmlFormatsOfficeDocumentPresentationmlPresentation => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            Self::ApplicationMsWord => "application/msword",
            Self::ApplicationVndOpenXmlFormatsOfficeDocumentWordProcessingmlDocument => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            Self::ApplicationVndMozillaAulXml => "application/vnd.mozilla.xul+xml",
            _ => "text/plain",
        }
        .to_string()
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "text/css" => Self::TextCss,
            "text/csv" => Self::TextCsv,
            "text/html" => Self::TextHtml,
            "text/javascript" => Self::TextJavascript,
            "text/plain" => Self::TextPlain,
            "text/xml" => Self::TextXml,

            "application/java-archive" => Self::ApplicationJavaArchive,
            "application/EDI-X12" => Self::ApplicationEdiX12,
            "application/EDIFACT" => Self::ApplicationEdiFact,
            "application/javascript" => Self::ApplicationJavascript,
            "application/octet-stream" => Self::ApplicationOctetStream,
            "application/ogg" => Self::ApplicationOgg,
            "application/pdf" => Self::ApplicationPdf,
            "application/xhtml+xml" => Self::ApplicationXhtmlXml,
            "application/x-shockwave-flash" => Self::ApplicationXShockwaveFlash,
            "application/json" => Self::ApplicationJson,
            "application/ld+json" => Self::ApplicationLdJson,
            "application/xml" => Self::ApplicationXml,
            "application/zip" => Self::ApplicationZip,
            "application/x-www-form-urlencoded" => Self::ApplicationXWwwFormUrlencoded,

            "audio/mpeg" => Self::AudioMpeg,
            "audio/x-ms-wma" => Self::AudioXMsWma,
            "audio/vnd.rn-realaudio" => Self::AudioVndRnRealAudio,
            "audio/x-wav" => Self::AudioXWav,

            "image/gif" => Self::ImageGif,
            "image/jpeg" => Self::ImageJpeg,
            "image/png" => Self::ImagePng,
            "image/tiff" => Self::ImageTiff,
            "image/vnd.microsoft.icon" => Self::ImageVndMicrosoftIcon,
            "image/x-icon" => Self::ImageXIcon,
            "image/vnd.djvu" => Self::ImageVndDjvu,
            "image/svg+xml" => Self::ImageSvgXml,

            "multipart/mixed " => Self::MultipartMixed,
            "multipart/alternative" => Self::MultipartAlternative,
            "multipart/related" => Self::MultipartRelated,
            "multipart/form-data" => Self::MultipartFormData,

            "video/mpeg" => Self::VideoMpeg,
            "video/mp4" => Self::VideoMp4,
            "video/quicktime" => Self::VideoQuicktime,
            "video/x-ms-wmv" => Self::VideoXMsWmv,
            "video/x-msvideo" => Self::VideoMsvideo,
            "video/x-flv" => Self::VideoXFlv,
            "video/webm" => Self::VideoWebm,

            "application/vnd.android.package-archive" => Self::ApplicationVndAndroidPackageArchive,
            "application/vnd.oasis.opendocument.text" => Self::ApplicationVndOasisOpenDocumentText,
            "application/vnd.oasis.opendocument.spreadsheet" => {
                Self::ApplicationVndOasisOpenDocumentSpreadsheet
            }
            "application/vnd.oasis.opendocument.presentation" => {
                Self::ApplicationVndOasisOpenDocumentPresentation
            }
            "application/vnd.oasis.opendocument.graphics" => {
                Self::ApplicationVndOasisOpenDocumentGraphics
            }
            "application/vnd.ms-excel" => Self::ApplicationVndMsExcel,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                Self::ApplicationVndOpenXmlFormatsOfficeDocumentSpreadsheetmlSheet
            }
            "application/vnd.ms-powerpoint" => Self::ApplicationVndMsPowerpoint,
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
                Self::ApplicationVndOpenXmlFormatsOfficeDocumentPresentationmlPresentation
            }
            "application/msword" => Self::ApplicationMsWord,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                Self::ApplicationVndOpenXmlFormatsOfficeDocumentWordProcessingmlDocument
            }
            "application/vnd.mozilla.xul+xml" => Self::ApplicationVndMozillaAulXml,
            _ => Self::TextPlain,
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        let chunks: Vec<&str> = filename.split(".").collect();
        let ext = chunks[chunks.len() - 1];

        match ext {
            // TODO: Add more common files
            "gif" => Self::ImageGif,
            "jpeg" | "jpg" => Self::ImageJpeg,
            "png" => Self::ImagePng,
            "tiff" => Self::ImageTiff,
            "ico" => Self::ImageXIcon,
            "svg" => Self::ImageSvgXml,

            "css" => Self::TextCss,
            "csv" => Self::TextCsv,
            "html" | "htm" | "php" => Self::TextHtml,
            "xml" => Self::TextXml,

            "json" => Self::ApplicationJson,
            "js" => Self::ApplicationJavascript,
            "pdf" => Self::ApplicationPdf,

            _ => Self::TextPlain,
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
