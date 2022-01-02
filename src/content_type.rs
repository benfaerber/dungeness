use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ContentType {
  TextCss,
  TextCsc,
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
  ApplicationVndMozillaAulXml
}

impl ContentType {
  fn to_string(&self) -> String {
    match self {
      Self::TextCss => "text/css",
      Self::TextCsc => "text/csv",
      Self::TextHtml => "text/html",
      Self::TextJavascript => "text/javascript",
      Self::TextPlain => "text/plain",
      Self::TextXml => "text/xml",
      _ => "text/plain",
    }.to_string()
  }
}

impl fmt::Display for ContentType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}