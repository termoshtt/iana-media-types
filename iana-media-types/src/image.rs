#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    :: serde :: Serialize,
    :: serde :: Deserialize,
)]
pub enum Image {
    #[doc = "image/aces"]
    #[serde(alias = "image/aces")]
    Aces,
    #[doc = "image/apng"]
    #[serde(alias = "image/apng")]
    Apng,
    #[doc = "image/avci"]
    #[serde(alias = "image/avci")]
    Avci,
    #[doc = "image/avcs"]
    #[serde(alias = "image/avcs")]
    Avcs,
    #[doc = "image/avif"]
    #[serde(alias = "image/avif")]
    Avif,
    #[doc = "image/bmp"]
    #[serde(alias = "image/bmp")]
    Bmp,
    #[doc = "image/cgm"]
    #[serde(alias = "image/cgm")]
    Cgm,
    #[doc = "image/dicom-rle"]
    #[serde(alias = "image/dicom-rle")]
    DicomRle,
    #[doc = "image/dpx"]
    #[serde(alias = "image/dpx")]
    Dpx,
    #[doc = "image/emf"]
    #[serde(alias = "image/emf")]
    Emf,
    #[doc = "image/example"]
    #[serde(alias = "image/example")]
    Example,
    #[doc = "image/fits"]
    #[serde(alias = "image/fits")]
    Fits,
    #[doc = "image/g3fax"]
    #[serde(alias = "image/g3fax")]
    G3Fax,
    #[doc = "image/heic"]
    #[serde(alias = "image/heic")]
    Heic,
    #[doc = "image/heic-sequence"]
    #[serde(alias = "image/heic-sequence")]
    HeicSequence,
    #[doc = "image/heif"]
    #[serde(alias = "image/heif")]
    Heif,
    #[doc = "image/heif-sequence"]
    #[serde(alias = "image/heif-sequence")]
    HeifSequence,
    #[doc = "image/hej2k"]
    #[serde(alias = "image/hej2k")]
    Hej2K,
    #[doc = "image/hsj2"]
    #[serde(alias = "image/hsj2")]
    Hsj2,
    #[doc = "image/j2c"]
    #[serde(alias = "image/j2c")]
    J2C,
    #[doc = "image/jls"]
    #[serde(alias = "image/jls")]
    Jls,
    #[doc = "image/jp2"]
    #[serde(alias = "image/jp2")]
    Jp2,
    #[doc = "image/jph"]
    #[serde(alias = "image/jph")]
    Jph,
    #[doc = "image/jphc"]
    #[serde(alias = "image/jphc")]
    Jphc,
    #[doc = "image/jpm"]
    #[serde(alias = "image/jpm")]
    Jpm,
    #[doc = "image/jpx"]
    #[serde(alias = "image/jpx")]
    Jpx,
    #[doc = "image/jxr"]
    #[serde(alias = "image/jxr")]
    Jxr,
    #[doc = "image/jxrA"]
    #[serde(alias = "image/jxrA")]
    JxrA,
    #[doc = "image/jxrS"]
    #[serde(alias = "image/jxrS")]
    JxrS,
    #[doc = "image/jxs"]
    #[serde(alias = "image/jxs")]
    Jxs,
    #[doc = "image/jxsc"]
    #[serde(alias = "image/jxsc")]
    Jxsc,
    #[doc = "image/jxsi"]
    #[serde(alias = "image/jxsi")]
    Jxsi,
    #[doc = "image/jxss"]
    #[serde(alias = "image/jxss")]
    Jxss,
    #[doc = "image/ktx"]
    #[serde(alias = "image/ktx")]
    Ktx,
    #[doc = "image/ktx2"]
    #[serde(alias = "image/ktx2")]
    Ktx2,
    #[doc = "image/naplps"]
    #[serde(alias = "image/naplps")]
    Naplps,
    #[doc = "image/png"]
    #[serde(alias = "image/png")]
    Png,
    #[doc = "image/prs.btif"]
    #[serde(alias = "image/prs.btif")]
    PrsBtif,
    #[doc = "image/prs.pti"]
    #[serde(alias = "image/prs.pti")]
    PrsPti,
    #[doc = "image/pwg-raster"]
    #[serde(alias = "image/pwg-raster")]
    PwgRaster,
    #[doc = "image/svg+xml"]
    #[serde(alias = "image/svg+xml")]
    SvgXml,
    #[doc = "image/t38"]
    #[serde(alias = "image/t38")]
    T38,
    #[doc = "image/tiff"]
    #[serde(alias = "image/tiff")]
    Tiff,
    #[doc = "image/tiff-fx"]
    #[serde(alias = "image/tiff-fx")]
    TiffFx,
    #[doc = "image/vnd.adobe.photoshop"]
    #[serde(alias = "image/vnd.adobe.photoshop")]
    VndAdobePhotoshop,
    #[doc = "image/vnd.airzip.accelerator.azv"]
    #[serde(alias = "image/vnd.airzip.accelerator.azv")]
    VndAirzipAcceleratorAzv,
    #[doc = "image/vnd.cns.inf2"]
    #[serde(alias = "image/vnd.cns.inf2")]
    VndCnsInf2,
    #[doc = "image/vnd.dece.graphic"]
    #[serde(alias = "image/vnd.dece.graphic")]
    VndDeceGraphic,
    #[doc = "image/vnd.djvu"]
    #[serde(alias = "image/vnd.djvu")]
    VndDjvu,
    #[doc = "image/vnd.dwg"]
    #[serde(alias = "image/vnd.dwg")]
    VndDwg,
    #[doc = "image/vnd.dxf"]
    #[serde(alias = "image/vnd.dxf")]
    VndDxf,
    #[doc = "image/vnd.dvb.subtitle"]
    #[serde(alias = "image/vnd.dvb.subtitle")]
    VndDvbSubtitle,
    #[doc = "image/vnd.fastbidsheet"]
    #[serde(alias = "image/vnd.fastbidsheet")]
    VndFastbidsheet,
    #[doc = "image/vnd.fpx"]
    #[serde(alias = "image/vnd.fpx")]
    VndFpx,
    #[doc = "image/vnd.fst"]
    #[serde(alias = "image/vnd.fst")]
    VndFst,
    #[doc = "image/vnd.fujixerox.edmics-mmr"]
    #[serde(alias = "image/vnd.fujixerox.edmics-mmr")]
    VndFujixeroxEdmicsMmr,
    #[doc = "image/vnd.fujixerox.edmics-rlc"]
    #[serde(alias = "image/vnd.fujixerox.edmics-rlc")]
    VndFujixeroxEdmicsRlc,
    #[doc = "image/vnd.globalgraphics.pgb"]
    #[serde(alias = "image/vnd.globalgraphics.pgb")]
    VndGlobalgraphicsPgb,
    #[doc = "image/vnd.microsoft.icon"]
    #[serde(alias = "image/vnd.microsoft.icon")]
    VndMicrosoftIcon,
    #[doc = "image/vnd.mix"]
    #[serde(alias = "image/vnd.mix")]
    VndMix,
    #[doc = "image/vnd.ms-modi"]
    #[serde(alias = "image/vnd.ms-modi")]
    VndMsModi,
    #[doc = "image/vnd.mozilla.apng"]
    #[serde(alias = "image/vnd.mozilla.apng")]
    VndMozillaApng,
    #[doc = "image/vnd.net-fpx"]
    #[serde(alias = "image/vnd.net-fpx")]
    VndNetFpx,
    #[doc = "image/vnd.pco.b16"]
    #[serde(alias = "image/vnd.pco.b16")]
    VndPcoB16,
    #[doc = "image/vnd.radiance"]
    #[serde(alias = "image/vnd.radiance")]
    VndRadiance,
    #[doc = "image/vnd.sealed.png"]
    #[serde(alias = "image/vnd.sealed.png")]
    VndSealedPng,
    #[doc = "image/vnd.sealedmedia.softseal.gif"]
    #[serde(alias = "image/vnd.sealedmedia.softseal.gif")]
    VndSealedmediaSoftsealGif,
    #[doc = "image/vnd.sealedmedia.softseal.jpg"]
    #[serde(alias = "image/vnd.sealedmedia.softseal.jpg")]
    VndSealedmediaSoftsealJpg,
    #[doc = "image/vnd.svf"]
    #[serde(alias = "image/vnd.svf")]
    VndSvf,
    #[doc = "image/vnd.tencent.tap"]
    #[serde(alias = "image/vnd.tencent.tap")]
    VndTencentTap,
    #[doc = "image/vnd.valve.source.texture"]
    #[serde(alias = "image/vnd.valve.source.texture")]
    VndValveSourceTexture,
    #[doc = "image/vnd.wap.wbmp"]
    #[serde(alias = "image/vnd.wap.wbmp")]
    VndWapWbmp,
    #[doc = "image/vnd.xiff"]
    #[serde(alias = "image/vnd.xiff")]
    VndXiff,
    #[doc = "image/vnd.zbrush.pcx"]
    #[serde(alias = "image/vnd.zbrush.pcx")]
    VndZbrushPcx,
    #[doc = "image/webp"]
    #[serde(alias = "image/webp")]
    Webp,
    #[doc = "image/wmf"]
    #[serde(alias = "image/wmf")]
    Wmf,
}
impl ::std::fmt::Display for Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Image::Aces => write!(f, "image/aces")?,
            Image::Apng => write!(f, "image/apng")?,
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
            Image::J2C => write!(f, "image/j2c")?,
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
            Image::Webp => write!(f, "image/webp")?,
            Image::Wmf => write!(f, "image/wmf")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Image {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "image/aces" => Ok(Image::Aces),
            "image/apng" => Ok(Image::Apng),
            "image/avci" => Ok(Image::Avci),
            "image/avcs" => Ok(Image::Avcs),
            "image/avif" => Ok(Image::Avif),
            "image/bmp" => Ok(Image::Bmp),
            "image/cgm" => Ok(Image::Cgm),
            "image/dicom-rle" => Ok(Image::DicomRle),
            "image/dpx" => Ok(Image::Dpx),
            "image/emf" => Ok(Image::Emf),
            "image/example" => Ok(Image::Example),
            "image/fits" => Ok(Image::Fits),
            "image/g3fax" => Ok(Image::G3Fax),
            "image/heic" => Ok(Image::Heic),
            "image/heic-sequence" => Ok(Image::HeicSequence),
            "image/heif" => Ok(Image::Heif),
            "image/heif-sequence" => Ok(Image::HeifSequence),
            "image/hej2k" => Ok(Image::Hej2K),
            "image/hsj2" => Ok(Image::Hsj2),
            "image/j2c" => Ok(Image::J2C),
            "image/jls" => Ok(Image::Jls),
            "image/jp2" => Ok(Image::Jp2),
            "image/jph" => Ok(Image::Jph),
            "image/jphc" => Ok(Image::Jphc),
            "image/jpm" => Ok(Image::Jpm),
            "image/jpx" => Ok(Image::Jpx),
            "image/jxr" => Ok(Image::Jxr),
            "image/jxrA" => Ok(Image::JxrA),
            "image/jxrS" => Ok(Image::JxrS),
            "image/jxs" => Ok(Image::Jxs),
            "image/jxsc" => Ok(Image::Jxsc),
            "image/jxsi" => Ok(Image::Jxsi),
            "image/jxss" => Ok(Image::Jxss),
            "image/ktx" => Ok(Image::Ktx),
            "image/ktx2" => Ok(Image::Ktx2),
            "image/naplps" => Ok(Image::Naplps),
            "image/png" => Ok(Image::Png),
            "image/prs.btif" => Ok(Image::PrsBtif),
            "image/prs.pti" => Ok(Image::PrsPti),
            "image/pwg-raster" => Ok(Image::PwgRaster),
            "image/svg+xml" => Ok(Image::SvgXml),
            "image/t38" => Ok(Image::T38),
            "image/tiff" => Ok(Image::Tiff),
            "image/tiff-fx" => Ok(Image::TiffFx),
            "image/vnd.adobe.photoshop" => Ok(Image::VndAdobePhotoshop),
            "image/vnd.airzip.accelerator.azv" => Ok(Image::VndAirzipAcceleratorAzv),
            "image/vnd.cns.inf2" => Ok(Image::VndCnsInf2),
            "image/vnd.dece.graphic" => Ok(Image::VndDeceGraphic),
            "image/vnd.djvu" => Ok(Image::VndDjvu),
            "image/vnd.dwg" => Ok(Image::VndDwg),
            "image/vnd.dxf" => Ok(Image::VndDxf),
            "image/vnd.dvb.subtitle" => Ok(Image::VndDvbSubtitle),
            "image/vnd.fastbidsheet" => Ok(Image::VndFastbidsheet),
            "image/vnd.fpx" => Ok(Image::VndFpx),
            "image/vnd.fst" => Ok(Image::VndFst),
            "image/vnd.fujixerox.edmics-mmr" => Ok(Image::VndFujixeroxEdmicsMmr),
            "image/vnd.fujixerox.edmics-rlc" => Ok(Image::VndFujixeroxEdmicsRlc),
            "image/vnd.globalgraphics.pgb" => Ok(Image::VndGlobalgraphicsPgb),
            "image/vnd.microsoft.icon" => Ok(Image::VndMicrosoftIcon),
            "image/vnd.mix" => Ok(Image::VndMix),
            "image/vnd.ms-modi" => Ok(Image::VndMsModi),
            "image/vnd.mozilla.apng" => Ok(Image::VndMozillaApng),
            "image/vnd.net-fpx" => Ok(Image::VndNetFpx),
            "image/vnd.pco.b16" => Ok(Image::VndPcoB16),
            "image/vnd.radiance" => Ok(Image::VndRadiance),
            "image/vnd.sealed.png" => Ok(Image::VndSealedPng),
            "image/vnd.sealedmedia.softseal.gif" => Ok(Image::VndSealedmediaSoftsealGif),
            "image/vnd.sealedmedia.softseal.jpg" => Ok(Image::VndSealedmediaSoftsealJpg),
            "image/vnd.svf" => Ok(Image::VndSvf),
            "image/vnd.tencent.tap" => Ok(Image::VndTencentTap),
            "image/vnd.valve.source.texture" => Ok(Image::VndValveSourceTexture),
            "image/vnd.wap.wbmp" => Ok(Image::VndWapWbmp),
            "image/vnd.xiff" => Ok(Image::VndXiff),
            "image/vnd.zbrush.pcx" => Ok(Image::VndZbrushPcx),
            "image/webp" => Ok(Image::Webp),
            "image/wmf" => Ok(Image::Wmf),
            _ => Err(()),
        }
    }
}
