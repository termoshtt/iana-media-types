#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Image {
    #[doc = "image/aces"]
    Aces,
    #[doc = "image/avci"]
    Avci,
    #[doc = "image/avcs"]
    Avcs,
    #[doc = "image/avif"]
    Avif,
    #[doc = "image/bmp"]
    Bmp,
    #[doc = "image/cgm"]
    Cgm,
    #[doc = "image/dicom-rle"]
    DicomRle,
    #[doc = "image/dpx"]
    Dpx,
    #[doc = "image/emf"]
    Emf,
    #[doc = "image/example"]
    Example,
    #[doc = "image/fits"]
    Fits,
    #[doc = "image/g3fax"]
    G3Fax,
    #[doc = "image/heic"]
    Heic,
    #[doc = "image/heic-sequence"]
    HeicSequence,
    #[doc = "image/heif"]
    Heif,
    #[doc = "image/heif-sequence"]
    HeifSequence,
    #[doc = "image/hej2k"]
    Hej2K,
    #[doc = "image/hsj2"]
    Hsj2,
    #[doc = "image/jls"]
    Jls,
    #[doc = "image/jp2"]
    Jp2,
    #[doc = "image/jph"]
    Jph,
    #[doc = "image/jphc"]
    Jphc,
    #[doc = "image/jpm"]
    Jpm,
    #[doc = "image/jpx"]
    Jpx,
    #[doc = "image/jxr"]
    Jxr,
    #[doc = "image/jxrA"]
    JxrA,
    #[doc = "image/jxrS"]
    JxrS,
    #[doc = "image/jxs"]
    Jxs,
    #[doc = "image/jxsc"]
    Jxsc,
    #[doc = "image/jxsi"]
    Jxsi,
    #[doc = "image/jxss"]
    Jxss,
    #[doc = "image/ktx"]
    Ktx,
    #[doc = "image/ktx2"]
    Ktx2,
    #[doc = "image/naplps"]
    Naplps,
    #[doc = "image/png"]
    Png,
    #[doc = "image/prs.btif"]
    PrsBtif,
    #[doc = "image/prs.pti"]
    PrsPti,
    #[doc = "image/pwg-raster"]
    PwgRaster,
    #[doc = "image/svg+xml"]
    SvgXml,
    #[doc = "image/t38"]
    T38,
    #[doc = "image/tiff"]
    Tiff,
    #[doc = "image/tiff-fx"]
    TiffFx,
    #[doc = "image/vnd.adobe.photoshop"]
    VndAdobePhotoshop,
    #[doc = "image/vnd.airzip.accelerator.azv"]
    VndAirzipAcceleratorAzv,
    #[doc = "image/vnd.cns.inf2"]
    VndCnsInf2,
    #[doc = "image/vnd.dece.graphic"]
    VndDeceGraphic,
    #[doc = "image/vnd.djvu"]
    VndDjvu,
    #[doc = "image/vnd.dwg"]
    VndDwg,
    #[doc = "image/vnd.dxf"]
    VndDxf,
    #[doc = "image/vnd.dvb.subtitle"]
    VndDvbSubtitle,
    #[doc = "image/vnd.fastbidsheet"]
    VndFastbidsheet,
    #[doc = "image/vnd.fpx"]
    VndFpx,
    #[doc = "image/vnd.fst"]
    VndFst,
    #[doc = "image/vnd.fujixerox.edmics-mmr"]
    VndFujixeroxEdmicsMmr,
    #[doc = "image/vnd.fujixerox.edmics-rlc"]
    VndFujixeroxEdmicsRlc,
    #[doc = "image/vnd.globalgraphics.pgb"]
    VndGlobalgraphicsPgb,
    #[doc = "image/vnd.microsoft.icon"]
    VndMicrosoftIcon,
    #[doc = "image/vnd.mix"]
    VndMix,
    #[doc = "image/vnd.ms-modi"]
    VndMsModi,
    #[doc = "image/vnd.mozilla.apng"]
    VndMozillaApng,
    #[doc = "image/vnd.net-fpx"]
    VndNetFpx,
    #[doc = "image/vnd.pco.b16"]
    VndPcoB16,
    #[doc = "image/vnd.radiance"]
    VndRadiance,
    #[doc = "image/vnd.sealed.png"]
    VndSealedPng,
    #[doc = "image/vnd.sealedmedia.softseal.gif"]
    VndSealedmediaSoftsealGif,
    #[doc = "image/vnd.sealedmedia.softseal.jpg"]
    VndSealedmediaSoftsealJpg,
    #[doc = "image/vnd.svf"]
    VndSvf,
    #[doc = "image/vnd.tencent.tap"]
    VndTencentTap,
    #[doc = "image/vnd.valve.source.texture"]
    VndValveSourceTexture,
    #[doc = "image/vnd.wap.wbmp"]
    VndWapWbmp,
    #[doc = "image/vnd.xiff"]
    VndXiff,
    #[doc = "image/vnd.zbrush.pcx"]
    VndZbrushPcx,
    #[doc = "image/wmf"]
    Wmf,
    #[doc = "image/emf"]
    XEmfDEPRECATEDInFavorOfImageEmf,
    #[doc = "image/wmf"]
    XWmfDEPRECATEDInFavorOfImageWmf,
    Other(String),
}
impl ::std::fmt::Display for Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Image::Aces => write!(f, "image/aces")?,
            Image::Avci => write!(f, "image/avci")?,
            Image::Avcs => write!(f, "image/avcs")?,
            Image::Avif => write!(f, "image/avif")?,
            Image::Bmp => write!(f, "image/bmp")?,
            Image::Cgm => write!(f, "image/cgm")?,
            Image::DicomRle => write!(f, "image/dicom-rle")?,
            Image::Dpx => write!(f, "image/dpx")?,
            Image::Emf => write!(f, "image/emf")?,
            Image::Example => write!(f, "image/example")?,
            Image::Fits => write!(f, "image/fits")?,
            Image::G3Fax => write!(f, "image/g3fax")?,
            Image::Heic => write!(f, "image/heic")?,
            Image::HeicSequence => write!(f, "image/heic-sequence")?,
            Image::Heif => write!(f, "image/heif")?,
            Image::HeifSequence => write!(f, "image/heif-sequence")?,
            Image::Hej2K => write!(f, "image/hej2k")?,
            Image::Hsj2 => write!(f, "image/hsj2")?,
            Image::Jls => write!(f, "image/jls")?,
            Image::Jp2 => write!(f, "image/jp2")?,
            Image::Jph => write!(f, "image/jph")?,
            Image::Jphc => write!(f, "image/jphc")?,
            Image::Jpm => write!(f, "image/jpm")?,
            Image::Jpx => write!(f, "image/jpx")?,
            Image::Jxr => write!(f, "image/jxr")?,
            Image::JxrA => write!(f, "image/jxrA")?,
            Image::JxrS => write!(f, "image/jxrS")?,
            Image::Jxs => write!(f, "image/jxs")?,
            Image::Jxsc => write!(f, "image/jxsc")?,
            Image::Jxsi => write!(f, "image/jxsi")?,
            Image::Jxss => write!(f, "image/jxss")?,
            Image::Ktx => write!(f, "image/ktx")?,
            Image::Ktx2 => write!(f, "image/ktx2")?,
            Image::Naplps => write!(f, "image/naplps")?,
            Image::Png => write!(f, "image/png")?,
            Image::PrsBtif => write!(f, "image/prs.btif")?,
            Image::PrsPti => write!(f, "image/prs.pti")?,
            Image::PwgRaster => write!(f, "image/pwg-raster")?,
            Image::SvgXml => write!(f, "image/svg+xml")?,
            Image::T38 => write!(f, "image/t38")?,
            Image::Tiff => write!(f, "image/tiff")?,
            Image::TiffFx => write!(f, "image/tiff-fx")?,
            Image::VndAdobePhotoshop => write!(f, "image/vnd.adobe.photoshop")?,
            Image::VndAirzipAcceleratorAzv => write!(f, "image/vnd.airzip.accelerator.azv")?,
            Image::VndCnsInf2 => write!(f, "image/vnd.cns.inf2")?,
            Image::VndDeceGraphic => write!(f, "image/vnd.dece.graphic")?,
            Image::VndDjvu => write!(f, "image/vnd.djvu")?,
            Image::VndDwg => write!(f, "image/vnd.dwg")?,
            Image::VndDxf => write!(f, "image/vnd.dxf")?,
            Image::VndDvbSubtitle => write!(f, "image/vnd.dvb.subtitle")?,
            Image::VndFastbidsheet => write!(f, "image/vnd.fastbidsheet")?,
            Image::VndFpx => write!(f, "image/vnd.fpx")?,
            Image::VndFst => write!(f, "image/vnd.fst")?,
            Image::VndFujixeroxEdmicsMmr => write!(f, "image/vnd.fujixerox.edmics-mmr")?,
            Image::VndFujixeroxEdmicsRlc => write!(f, "image/vnd.fujixerox.edmics-rlc")?,
            Image::VndGlobalgraphicsPgb => write!(f, "image/vnd.globalgraphics.pgb")?,
            Image::VndMicrosoftIcon => write!(f, "image/vnd.microsoft.icon")?,
            Image::VndMix => write!(f, "image/vnd.mix")?,
            Image::VndMsModi => write!(f, "image/vnd.ms-modi")?,
            Image::VndMozillaApng => write!(f, "image/vnd.mozilla.apng")?,
            Image::VndNetFpx => write!(f, "image/vnd.net-fpx")?,
            Image::VndPcoB16 => write!(f, "image/vnd.pco.b16")?,
            Image::VndRadiance => write!(f, "image/vnd.radiance")?,
            Image::VndSealedPng => write!(f, "image/vnd.sealed.png")?,
            Image::VndSealedmediaSoftsealGif => write!(f, "image/vnd.sealedmedia.softseal.gif")?,
            Image::VndSealedmediaSoftsealJpg => write!(f, "image/vnd.sealedmedia.softseal.jpg")?,
            Image::VndSvf => write!(f, "image/vnd.svf")?,
            Image::VndTencentTap => write!(f, "image/vnd.tencent.tap")?,
            Image::VndValveSourceTexture => write!(f, "image/vnd.valve.source.texture")?,
            Image::VndWapWbmp => write!(f, "image/vnd.wap.wbmp")?,
            Image::VndXiff => write!(f, "image/vnd.xiff")?,
            Image::VndZbrushPcx => write!(f, "image/vnd.zbrush.pcx")?,
            Image::Wmf => write!(f, "image/wmf")?,
            Image::XEmfDEPRECATEDInFavorOfImageEmf => write!(f, "image/emf")?,
            Image::XWmfDEPRECATEDInFavorOfImageWmf => write!(f, "image/wmf")?,
            Image::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Image {
    fn from(input: &str) -> Self {
        match input {
            "image/aces" => Image::Aces,
            "image/avci" => Image::Avci,
            "image/avcs" => Image::Avcs,
            "image/avif" => Image::Avif,
            "image/bmp" => Image::Bmp,
            "image/cgm" => Image::Cgm,
            "image/dicom-rle" => Image::DicomRle,
            "image/dpx" => Image::Dpx,
            "image/emf" => Image::Emf,
            "image/example" => Image::Example,
            "image/fits" => Image::Fits,
            "image/g3fax" => Image::G3Fax,
            "image/heic" => Image::Heic,
            "image/heic-sequence" => Image::HeicSequence,
            "image/heif" => Image::Heif,
            "image/heif-sequence" => Image::HeifSequence,
            "image/hej2k" => Image::Hej2K,
            "image/hsj2" => Image::Hsj2,
            "image/jls" => Image::Jls,
            "image/jp2" => Image::Jp2,
            "image/jph" => Image::Jph,
            "image/jphc" => Image::Jphc,
            "image/jpm" => Image::Jpm,
            "image/jpx" => Image::Jpx,
            "image/jxr" => Image::Jxr,
            "image/jxrA" => Image::JxrA,
            "image/jxrS" => Image::JxrS,
            "image/jxs" => Image::Jxs,
            "image/jxsc" => Image::Jxsc,
            "image/jxsi" => Image::Jxsi,
            "image/jxss" => Image::Jxss,
            "image/ktx" => Image::Ktx,
            "image/ktx2" => Image::Ktx2,
            "image/naplps" => Image::Naplps,
            "image/png" => Image::Png,
            "image/prs.btif" => Image::PrsBtif,
            "image/prs.pti" => Image::PrsPti,
            "image/pwg-raster" => Image::PwgRaster,
            "image/svg+xml" => Image::SvgXml,
            "image/t38" => Image::T38,
            "image/tiff" => Image::Tiff,
            "image/tiff-fx" => Image::TiffFx,
            "image/vnd.adobe.photoshop" => Image::VndAdobePhotoshop,
            "image/vnd.airzip.accelerator.azv" => Image::VndAirzipAcceleratorAzv,
            "image/vnd.cns.inf2" => Image::VndCnsInf2,
            "image/vnd.dece.graphic" => Image::VndDeceGraphic,
            "image/vnd.djvu" => Image::VndDjvu,
            "image/vnd.dwg" => Image::VndDwg,
            "image/vnd.dxf" => Image::VndDxf,
            "image/vnd.dvb.subtitle" => Image::VndDvbSubtitle,
            "image/vnd.fastbidsheet" => Image::VndFastbidsheet,
            "image/vnd.fpx" => Image::VndFpx,
            "image/vnd.fst" => Image::VndFst,
            "image/vnd.fujixerox.edmics-mmr" => Image::VndFujixeroxEdmicsMmr,
            "image/vnd.fujixerox.edmics-rlc" => Image::VndFujixeroxEdmicsRlc,
            "image/vnd.globalgraphics.pgb" => Image::VndGlobalgraphicsPgb,
            "image/vnd.microsoft.icon" => Image::VndMicrosoftIcon,
            "image/vnd.mix" => Image::VndMix,
            "image/vnd.ms-modi" => Image::VndMsModi,
            "image/vnd.mozilla.apng" => Image::VndMozillaApng,
            "image/vnd.net-fpx" => Image::VndNetFpx,
            "image/vnd.pco.b16" => Image::VndPcoB16,
            "image/vnd.radiance" => Image::VndRadiance,
            "image/vnd.sealed.png" => Image::VndSealedPng,
            "image/vnd.sealedmedia.softseal.gif" => Image::VndSealedmediaSoftsealGif,
            "image/vnd.sealedmedia.softseal.jpg" => Image::VndSealedmediaSoftsealJpg,
            "image/vnd.svf" => Image::VndSvf,
            "image/vnd.tencent.tap" => Image::VndTencentTap,
            "image/vnd.valve.source.texture" => Image::VndValveSourceTexture,
            "image/vnd.wap.wbmp" => Image::VndWapWbmp,
            "image/vnd.xiff" => Image::VndXiff,
            "image/vnd.zbrush.pcx" => Image::VndZbrushPcx,
            "image/wmf" => Image::Wmf,
            "image/emf" => Image::XEmfDEPRECATEDInFavorOfImageEmf,
            "image/wmf" => Image::XWmfDEPRECATEDInFavorOfImageWmf,
            _ => Image::Other(input.to_string()),
        }
    }
}
