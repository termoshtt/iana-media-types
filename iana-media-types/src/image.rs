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
    #[serde(alias = "exr")]
    Aces,
    #[doc = "image/apng"]
    #[serde(alias = "image/apng")]
    Apng,
    #[doc = "image/avci"]
    #[serde(alias = "image/avci")]
    #[serde(alias = "avci")]
    Avci,
    #[doc = "image/avcs"]
    #[serde(alias = "image/avcs")]
    #[serde(alias = "avcs")]
    Avcs,
    #[doc = "image/avif"]
    #[serde(alias = "image/avif")]
    #[serde(alias = "avif")]
    #[serde(alias = "hif")]
    Avif,
    #[doc = "image/bmp"]
    #[serde(alias = "image/bmp")]
    #[serde(alias = "bmp")]
    #[serde(alias = "dib")]
    Bmp,
    #[doc = "image/cgm"]
    #[serde(alias = "image/cgm")]
    #[serde(alias = "cgm")]
    Cgm,
    #[doc = "image/dicom-rle"]
    #[serde(alias = "image/dicom-rle")]
    #[serde(alias = "drle")]
    DicomRle,
    #[doc = "image/dpx"]
    #[serde(alias = "image/dpx")]
    #[serde(alias = "dpx")]
    Dpx,
    #[doc = "image/emf"]
    #[serde(alias = "image/emf")]
    #[serde(alias = "emf")]
    Emf,
    #[doc = "image/example"]
    #[serde(alias = "image/example")]
    Example,
    #[doc = "image/fits"]
    #[serde(alias = "image/fits")]
    #[serde(alias = "fits")]
    #[serde(alias = "fit")]
    #[serde(alias = "fts")]
    Fits,
    #[doc = "image/g3fax"]
    #[serde(alias = "image/g3fax")]
    G3Fax,
    #[doc = "image/heic"]
    #[serde(alias = "image/heic")]
    #[serde(alias = "heic")]
    Gif,
    #[doc = "image/heic-sequence"]
    #[serde(alias = "image/heic-sequence")]
    #[serde(alias = "heics")]
    Heic,
    #[doc = "image/heif"]
    #[serde(alias = "image/heif")]
    #[serde(alias = "heif")]
    HeicSequence,
    #[doc = "image/heif-sequence"]
    #[serde(alias = "image/heif-sequence")]
    #[serde(alias = "heifs")]
    Heif,
    #[doc = "image/hej2k"]
    #[serde(alias = "image/hej2k")]
    #[serde(alias = "hej2")]
    HeifSequence,
    #[doc = "image/hsj2"]
    #[serde(alias = "image/hsj2")]
    #[serde(alias = "hsj2")]
    Hej2K,
    #[doc = "image/j2c"]
    #[serde(alias = "image/j2c")]
    Hsj2,
    #[doc = "image/jls"]
    #[serde(alias = "image/jls")]
    #[serde(alias = "jls")]
    Ief,
    #[doc = "image/jp2"]
    #[serde(alias = "image/jp2")]
    #[serde(alias = "jp2")]
    #[serde(alias = "jpg2")]
    J2C,
    #[doc = "image/jph"]
    #[serde(alias = "image/jph")]
    #[serde(alias = "jph")]
    Jls,
    #[doc = "image/jphc"]
    #[serde(alias = "image/jphc")]
    #[serde(alias = "jhc")]
    Jp2,
    #[doc = "image/jpm"]
    #[serde(alias = "image/jpm")]
    #[serde(alias = "jpm")]
    #[serde(alias = "jpgm")]
    Jpeg,
    #[doc = "image/jpx"]
    #[serde(alias = "image/jpx")]
    #[serde(alias = "jpx")]
    #[serde(alias = "jpf")]
    Jph,
    #[doc = "image/jxr"]
    #[serde(alias = "image/jxr")]
    #[serde(alias = "jxr")]
    Jphc,
    #[doc = "image/jxrA"]
    #[serde(alias = "image/jxrA")]
    #[serde(alias = "jxra")]
    Jpm,
    #[doc = "image/jxrS"]
    #[serde(alias = "image/jxrS")]
    #[serde(alias = "jxrs")]
    Jpx,
    #[doc = "image/jxs"]
    #[serde(alias = "image/jxs")]
    #[serde(alias = "jxs")]
    Jxr,
    #[doc = "image/jxsc"]
    #[serde(alias = "image/jxsc")]
    #[serde(alias = "jxsc")]
    JxrA,
    #[doc = "image/jxsi"]
    #[serde(alias = "image/jxsi")]
    #[serde(alias = "jxsi")]
    JxrS,
    #[doc = "image/jxss"]
    #[serde(alias = "image/jxss")]
    #[serde(alias = "jxss")]
    Jxs,
    #[doc = "image/ktx"]
    #[serde(alias = "image/ktx")]
    #[serde(alias = "ktx")]
    Jxsc,
    #[doc = "image/ktx2"]
    #[serde(alias = "image/ktx2")]
    #[serde(alias = "ktx2")]
    Jxsi,
    #[doc = "image/naplps"]
    #[serde(alias = "image/naplps")]
    Jxss,
    #[doc = "image/png"]
    #[serde(alias = "image/png")]
    #[serde(alias = "png")]
    Ktx,
    #[doc = "image/prs.btif"]
    #[serde(alias = "image/prs.btif")]
    #[serde(alias = "btif")]
    #[serde(alias = "btf")]
    Ktx2,
    #[doc = "image/prs.pti"]
    #[serde(alias = "image/prs.pti")]
    #[serde(alias = "pti")]
    Naplps,
    #[doc = "image/pwg-raster"]
    #[serde(alias = "image/pwg-raster")]
    Png,
    #[doc = "image/svg+xml"]
    #[serde(alias = "image/svg+xml")]
    #[serde(alias = "svg")]
    #[serde(alias = "svgz")]
    PrsBtif,
    #[doc = "image/t38"]
    #[serde(alias = "image/t38")]
    #[serde(alias = "t38")]
    PrsPti,
    #[doc = "image/tiff"]
    #[serde(alias = "image/tiff")]
    #[serde(alias = "tiff")]
    #[serde(alias = "tif")]
    PwgRaster,
    #[doc = "image/tiff-fx"]
    #[serde(alias = "image/tiff-fx")]
    #[serde(alias = "tfx")]
    SvgXml,
    #[doc = "image/vnd.adobe.photoshop"]
    #[serde(alias = "image/vnd.adobe.photoshop")]
    #[serde(alias = "psd")]
    T38,
    #[doc = "image/vnd.airzip.accelerator.azv"]
    #[serde(alias = "image/vnd.airzip.accelerator.azv")]
    #[serde(alias = "azv")]
    Tiff,
    #[doc = "image/vnd.cns.inf2"]
    #[serde(alias = "image/vnd.cns.inf2")]
    TiffFx,
    #[doc = "image/vnd.dece.graphic"]
    #[serde(alias = "image/vnd.dece.graphic")]
    #[serde(alias = "uvi")]
    #[serde(alias = "uvvi")]
    #[serde(alias = "uvg")]
    #[serde(alias = "uvvg")]
    VndAdobePhotoshop,
    #[doc = "image/vnd.djvu"]
    #[serde(alias = "image/vnd.djvu")]
    #[serde(alias = "djvu")]
    #[serde(alias = "djv")]
    VndAirzipAcceleratorAzv,
    #[doc = "image/vnd.dwg"]
    #[serde(alias = "image/vnd.dwg")]
    #[serde(alias = "dwg")]
    VndCnsInf2,
    #[doc = "image/vnd.dxf"]
    #[serde(alias = "image/vnd.dxf")]
    #[serde(alias = "dxf")]
    VndDeceGraphic,
    #[doc = "image/vnd.dvb.subtitle"]
    #[serde(alias = "image/vnd.dvb.subtitle")]
    VndDjvu,
    #[doc = "image/vnd.fastbidsheet"]
    #[serde(alias = "image/vnd.fastbidsheet")]
    #[serde(alias = "fbs")]
    VndDwg,
    #[doc = "image/vnd.fpx"]
    #[serde(alias = "image/vnd.fpx")]
    #[serde(alias = "fpx")]
    VndDxf,
    #[doc = "image/vnd.fst"]
    #[serde(alias = "image/vnd.fst")]
    #[serde(alias = "fst")]
    VndDvbSubtitle,
    #[doc = "image/vnd.fujixerox.edmics-mmr"]
    #[serde(alias = "image/vnd.fujixerox.edmics-mmr")]
    #[serde(alias = "mmr")]
    VndFastbidsheet,
    #[doc = "image/vnd.fujixerox.edmics-rlc"]
    #[serde(alias = "image/vnd.fujixerox.edmics-rlc")]
    #[serde(alias = "rlc")]
    VndFpx,
    #[doc = "image/vnd.globalgraphics.pgb"]
    #[serde(alias = "image/vnd.globalgraphics.pgb")]
    #[serde(alias = "pgb")]
    VndFst,
    #[doc = "image/vnd.microsoft.icon"]
    #[serde(alias = "image/vnd.microsoft.icon")]
    #[serde(alias = "ico")]
    VndFujixeroxEdmicsMmr,
    #[doc = "image/vnd.mix"]
    #[serde(alias = "image/vnd.mix")]
    VndFujixeroxEdmicsRlc,
    #[doc = "image/vnd.ms-modi"]
    #[serde(alias = "image/vnd.ms-modi")]
    #[serde(alias = "mdi")]
    VndGlobalgraphicsPgb,
    #[doc = "image/vnd.mozilla.apng"]
    #[serde(alias = "image/vnd.mozilla.apng")]
    #[serde(alias = "apng")]
    VndMicrosoftIcon,
    #[doc = "image/vnd.net-fpx"]
    #[serde(alias = "image/vnd.net-fpx")]
    VndMix,
    #[doc = "image/vnd.pco.b16"]
    #[serde(alias = "image/vnd.pco.b16")]
    #[serde(alias = "b16")]
    VndMsModi,
    #[doc = "image/vnd.radiance"]
    #[serde(alias = "image/vnd.radiance")]
    #[serde(alias = "hdr")]
    #[serde(alias = "rgbe")]
    #[serde(alias = "xyze")]
    VndMozillaApng,
    #[doc = "image/vnd.sealed.png"]
    #[serde(alias = "image/vnd.sealed.png")]
    #[serde(alias = "spng")]
    #[serde(alias = "spn")]
    #[serde(alias = "s1n")]
    VndNetFpx,
    #[doc = "image/vnd.sealedmedia.softseal.gif"]
    #[serde(alias = "image/vnd.sealedmedia.softseal.gif")]
    #[serde(alias = "sgif")]
    #[serde(alias = "sgi")]
    #[serde(alias = "s1g")]
    VndPcoB16,
    #[doc = "image/vnd.sealedmedia.softseal.jpg"]
    #[serde(alias = "image/vnd.sealedmedia.softseal.jpg")]
    #[serde(alias = "sjpg")]
    #[serde(alias = "sjp")]
    #[serde(alias = "s1j")]
    VndRadiance,
    #[doc = "image/vnd.svf"]
    #[serde(alias = "image/vnd.svf")]
    VndSealedPng,
    #[doc = "image/vnd.tencent.tap"]
    #[serde(alias = "image/vnd.tencent.tap")]
    #[serde(alias = "tap")]
    VndSealedmediaSoftsealGif,
    #[doc = "image/vnd.valve.source.texture"]
    #[serde(alias = "image/vnd.valve.source.texture")]
    #[serde(alias = "vtf")]
    VndSealedmediaSoftsealJpg,
    #[doc = "image/vnd.wap.wbmp"]
    #[serde(alias = "image/vnd.wap.wbmp")]
    #[serde(alias = "wbmp")]
    VndSvf,
    #[doc = "image/vnd.xiff"]
    #[serde(alias = "image/vnd.xiff")]
    #[serde(alias = "xif")]
    VndTencentTap,
    #[doc = "image/vnd.zbrush.pcx"]
    #[serde(alias = "image/vnd.zbrush.pcx")]
    #[serde(alias = "pcx")]
    VndValveSourceTexture,
    #[doc = "image/webp"]
    #[serde(alias = "image/webp")]
    #[serde(alias = "webp")]
    VndWapWbmp,
    #[doc = "image/wmf"]
    #[serde(alias = "image/wmf")]
    #[serde(alias = "wmf")]
    VndXiff,
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
            Image::Gif => write!(f, "image/heic")?,
            Image::Heic => write!(f, "image/heic-sequence")?,
            Image::HeicSequence => write!(f, "image/heif")?,
            Image::Heif => write!(f, "image/heif-sequence")?,
            Image::HeifSequence => write!(f, "image/hej2k")?,
            Image::Hej2K => write!(f, "image/hsj2")?,
            Image::Hsj2 => write!(f, "image/j2c")?,
            Image::Ief => write!(f, "image/jls")?,
            Image::J2C => write!(f, "image/jp2")?,
            Image::Jls => write!(f, "image/jph")?,
            Image::Jp2 => write!(f, "image/jphc")?,
            Image::Jpeg => write!(f, "image/jpm")?,
            Image::Jph => write!(f, "image/jpx")?,
            Image::Jphc => write!(f, "image/jxr")?,
            Image::Jpm => write!(f, "image/jxrA")?,
            Image::Jpx => write!(f, "image/jxrS")?,
            Image::Jxr => write!(f, "image/jxs")?,
            Image::JxrA => write!(f, "image/jxsc")?,
            Image::JxrS => write!(f, "image/jxsi")?,
            Image::Jxs => write!(f, "image/jxss")?,
            Image::Jxsc => write!(f, "image/ktx")?,
            Image::Jxsi => write!(f, "image/ktx2")?,
            Image::Jxss => write!(f, "image/naplps")?,
            Image::Ktx => write!(f, "image/png")?,
            Image::Ktx2 => write!(f, "image/prs.btif")?,
            Image::Naplps => write!(f, "image/prs.pti")?,
            Image::Png => write!(f, "image/pwg-raster")?,
            Image::PrsBtif => write!(f, "image/svg+xml")?,
            Image::PrsPti => write!(f, "image/t38")?,
            Image::PwgRaster => write!(f, "image/tiff")?,
            Image::SvgXml => write!(f, "image/tiff-fx")?,
            Image::T38 => write!(f, "image/vnd.adobe.photoshop")?,
            Image::Tiff => write!(f, "image/vnd.airzip.accelerator.azv")?,
            Image::TiffFx => write!(f, "image/vnd.cns.inf2")?,
            Image::VndAdobePhotoshop => write!(f, "image/vnd.dece.graphic")?,
            Image::VndAirzipAcceleratorAzv => write!(f, "image/vnd.djvu")?,
            Image::VndCnsInf2 => write!(f, "image/vnd.dwg")?,
            Image::VndDeceGraphic => write!(f, "image/vnd.dxf")?,
            Image::VndDjvu => write!(f, "image/vnd.dvb.subtitle")?,
            Image::VndDwg => write!(f, "image/vnd.fastbidsheet")?,
            Image::VndDxf => write!(f, "image/vnd.fpx")?,
            Image::VndDvbSubtitle => write!(f, "image/vnd.fst")?,
            Image::VndFastbidsheet => write!(f, "image/vnd.fujixerox.edmics-mmr")?,
            Image::VndFpx => write!(f, "image/vnd.fujixerox.edmics-rlc")?,
            Image::VndFst => write!(f, "image/vnd.globalgraphics.pgb")?,
            Image::VndFujixeroxEdmicsMmr => write!(f, "image/vnd.microsoft.icon")?,
            Image::VndFujixeroxEdmicsRlc => write!(f, "image/vnd.mix")?,
            Image::VndGlobalgraphicsPgb => write!(f, "image/vnd.ms-modi")?,
            Image::VndMicrosoftIcon => write!(f, "image/vnd.mozilla.apng")?,
            Image::VndMix => write!(f, "image/vnd.net-fpx")?,
            Image::VndMsModi => write!(f, "image/vnd.pco.b16")?,
            Image::VndMozillaApng => write!(f, "image/vnd.radiance")?,
            Image::VndNetFpx => write!(f, "image/vnd.sealed.png")?,
            Image::VndPcoB16 => write!(f, "image/vnd.sealedmedia.softseal.gif")?,
            Image::VndRadiance => write!(f, "image/vnd.sealedmedia.softseal.jpg")?,
            Image::VndSealedPng => write!(f, "image/vnd.svf")?,
            Image::VndSealedmediaSoftsealGif => write!(f, "image/vnd.tencent.tap")?,
            Image::VndSealedmediaSoftsealJpg => write!(f, "image/vnd.valve.source.texture")?,
            Image::VndSvf => write!(f, "image/vnd.wap.wbmp")?,
            Image::VndTencentTap => write!(f, "image/vnd.xiff")?,
            Image::VndValveSourceTexture => write!(f, "image/vnd.zbrush.pcx")?,
            Image::VndWapWbmp => write!(f, "image/webp")?,
            Image::VndXiff => write!(f, "image/wmf")?,
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
            "image/heic" => Ok(Image::Gif),
            "image/heic-sequence" => Ok(Image::Heic),
            "image/heif" => Ok(Image::HeicSequence),
            "image/heif-sequence" => Ok(Image::Heif),
            "image/hej2k" => Ok(Image::HeifSequence),
            "image/hsj2" => Ok(Image::Hej2K),
            "image/j2c" => Ok(Image::Hsj2),
            "image/jls" => Ok(Image::Ief),
            "image/jp2" => Ok(Image::J2C),
            "image/jph" => Ok(Image::Jls),
            "image/jphc" => Ok(Image::Jp2),
            "image/jpm" => Ok(Image::Jpeg),
            "image/jpx" => Ok(Image::Jph),
            "image/jxr" => Ok(Image::Jphc),
            "image/jxrA" => Ok(Image::Jpm),
            "image/jxrS" => Ok(Image::Jpx),
            "image/jxs" => Ok(Image::Jxr),
            "image/jxsc" => Ok(Image::JxrA),
            "image/jxsi" => Ok(Image::JxrS),
            "image/jxss" => Ok(Image::Jxs),
            "image/ktx" => Ok(Image::Jxsc),
            "image/ktx2" => Ok(Image::Jxsi),
            "image/naplps" => Ok(Image::Jxss),
            "image/png" => Ok(Image::Ktx),
            "image/prs.btif" => Ok(Image::Ktx2),
            "image/prs.pti" => Ok(Image::Naplps),
            "image/pwg-raster" => Ok(Image::Png),
            "image/svg+xml" => Ok(Image::PrsBtif),
            "image/t38" => Ok(Image::PrsPti),
            "image/tiff" => Ok(Image::PwgRaster),
            "image/tiff-fx" => Ok(Image::SvgXml),
            "image/vnd.adobe.photoshop" => Ok(Image::T38),
            "image/vnd.airzip.accelerator.azv" => Ok(Image::Tiff),
            "image/vnd.cns.inf2" => Ok(Image::TiffFx),
            "image/vnd.dece.graphic" => Ok(Image::VndAdobePhotoshop),
            "image/vnd.djvu" => Ok(Image::VndAirzipAcceleratorAzv),
            "image/vnd.dwg" => Ok(Image::VndCnsInf2),
            "image/vnd.dxf" => Ok(Image::VndDeceGraphic),
            "image/vnd.dvb.subtitle" => Ok(Image::VndDjvu),
            "image/vnd.fastbidsheet" => Ok(Image::VndDwg),
            "image/vnd.fpx" => Ok(Image::VndDxf),
            "image/vnd.fst" => Ok(Image::VndDvbSubtitle),
            "image/vnd.fujixerox.edmics-mmr" => Ok(Image::VndFastbidsheet),
            "image/vnd.fujixerox.edmics-rlc" => Ok(Image::VndFpx),
            "image/vnd.globalgraphics.pgb" => Ok(Image::VndFst),
            "image/vnd.microsoft.icon" => Ok(Image::VndFujixeroxEdmicsMmr),
            "image/vnd.mix" => Ok(Image::VndFujixeroxEdmicsRlc),
            "image/vnd.ms-modi" => Ok(Image::VndGlobalgraphicsPgb),
            "image/vnd.mozilla.apng" => Ok(Image::VndMicrosoftIcon),
            "image/vnd.net-fpx" => Ok(Image::VndMix),
            "image/vnd.pco.b16" => Ok(Image::VndMsModi),
            "image/vnd.radiance" => Ok(Image::VndMozillaApng),
            "image/vnd.sealed.png" => Ok(Image::VndNetFpx),
            "image/vnd.sealedmedia.softseal.gif" => Ok(Image::VndPcoB16),
            "image/vnd.sealedmedia.softseal.jpg" => Ok(Image::VndRadiance),
            "image/vnd.svf" => Ok(Image::VndSealedPng),
            "image/vnd.tencent.tap" => Ok(Image::VndSealedmediaSoftsealGif),
            "image/vnd.valve.source.texture" => Ok(Image::VndSealedmediaSoftsealJpg),
            "image/vnd.wap.wbmp" => Ok(Image::VndSvf),
            "image/vnd.xiff" => Ok(Image::VndTencentTap),
            "image/vnd.zbrush.pcx" => Ok(Image::VndValveSourceTexture),
            "image/webp" => Ok(Image::VndWapWbmp),
            "image/wmf" => Ok(Image::VndXiff),
            _ => Err(()),
        }
    }
}
