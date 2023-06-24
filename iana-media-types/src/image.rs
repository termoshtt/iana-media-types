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
    #[serde(rename = "image/aces")]
    #[serde(alias = "exr")]
    Aces,
    #[doc = "image/apng"]
    #[serde(rename = "image/apng")]
    Apng,
    #[doc = "image/avci"]
    #[serde(rename = "image/avci")]
    #[serde(alias = "avci")]
    Avci,
    #[doc = "image/avcs"]
    #[serde(rename = "image/avcs")]
    #[serde(alias = "avcs")]
    Avcs,
    #[doc = "image/avif"]
    #[serde(rename = "image/avif")]
    #[serde(alias = "avif")]
    #[serde(alias = "hif")]
    Avif,
    #[doc = "image/bmp"]
    #[serde(rename = "image/bmp")]
    #[serde(alias = "bmp")]
    #[serde(alias = "dib")]
    Bmp,
    #[doc = "image/cgm"]
    #[serde(rename = "image/cgm")]
    #[serde(alias = "cgm")]
    Cgm,
    #[doc = "image/dicom-rle"]
    #[serde(rename = "image/dicom-rle")]
    #[serde(alias = "drle")]
    DicomRle,
    #[doc = "image/dpx"]
    #[serde(rename = "image/dpx")]
    #[serde(alias = "dpx")]
    Dpx,
    #[doc = "image/emf"]
    #[serde(rename = "image/emf")]
    #[serde(alias = "emf")]
    Emf,
    #[doc = "image/example"]
    #[serde(rename = "image/example")]
    Example,
    #[doc = "image/fits"]
    #[serde(rename = "image/fits")]
    #[serde(alias = "fits")]
    #[serde(alias = "fit")]
    #[serde(alias = "fts")]
    Fits,
    #[doc = "image/g3fax"]
    #[serde(rename = "image/g3fax")]
    G3Fax,
    #[doc = "image/heic"]
    #[serde(rename = "image/heic")]
    #[serde(alias = "heic")]
    Gif,
    #[doc = "image/heic-sequence"]
    #[serde(rename = "image/heic-sequence")]
    #[serde(alias = "heics")]
    Heic,
    #[doc = "image/heif"]
    #[serde(rename = "image/heif")]
    #[serde(alias = "heif")]
    HeicSequence,
    #[doc = "image/heif-sequence"]
    #[serde(rename = "image/heif-sequence")]
    #[serde(alias = "heifs")]
    Heif,
    #[doc = "image/hej2k"]
    #[serde(rename = "image/hej2k")]
    #[serde(alias = "hej2")]
    HeifSequence,
    #[doc = "image/hsj2"]
    #[serde(rename = "image/hsj2")]
    #[serde(alias = "hsj2")]
    Hej2K,
    #[doc = "image/j2c"]
    #[serde(rename = "image/j2c")]
    Hsj2,
    #[doc = "image/jls"]
    #[serde(rename = "image/jls")]
    #[serde(alias = "jls")]
    Ief,
    #[doc = "image/jp2"]
    #[serde(rename = "image/jp2")]
    #[serde(alias = "jp2")]
    #[serde(alias = "jpg2")]
    J2C,
    #[doc = "image/jph"]
    #[serde(rename = "image/jph")]
    #[serde(alias = "jph")]
    Jls,
    #[doc = "image/jphc"]
    #[serde(rename = "image/jphc")]
    #[serde(alias = "jhc")]
    Jp2,
    #[doc = "image/jpm"]
    #[serde(rename = "image/jpm")]
    #[serde(alias = "jpm")]
    #[serde(alias = "jpgm")]
    Jpeg,
    #[doc = "image/jpx"]
    #[serde(rename = "image/jpx")]
    #[serde(alias = "jpx")]
    #[serde(alias = "jpf")]
    Jph,
    #[doc = "image/jxr"]
    #[serde(rename = "image/jxr")]
    #[serde(alias = "jxr")]
    Jphc,
    #[doc = "image/jxrA"]
    #[serde(rename = "image/jxrA")]
    #[serde(alias = "jxra")]
    Jpm,
    #[doc = "image/jxrS"]
    #[serde(rename = "image/jxrS")]
    #[serde(alias = "jxrs")]
    Jpx,
    #[doc = "image/jxs"]
    #[serde(rename = "image/jxs")]
    #[serde(alias = "jxs")]
    Jxr,
    #[doc = "image/jxsc"]
    #[serde(rename = "image/jxsc")]
    #[serde(alias = "jxsc")]
    JxrA,
    #[doc = "image/jxsi"]
    #[serde(rename = "image/jxsi")]
    #[serde(alias = "jxsi")]
    JxrS,
    #[doc = "image/jxss"]
    #[serde(rename = "image/jxss")]
    #[serde(alias = "jxss")]
    Jxs,
    #[doc = "image/ktx"]
    #[serde(rename = "image/ktx")]
    #[serde(alias = "ktx")]
    Jxsc,
    #[doc = "image/ktx2"]
    #[serde(rename = "image/ktx2")]
    #[serde(alias = "ktx2")]
    Jxsi,
    #[doc = "image/naplps"]
    #[serde(rename = "image/naplps")]
    Jxss,
    #[doc = "image/png"]
    #[serde(rename = "image/png")]
    #[serde(alias = "png")]
    Ktx,
    #[doc = "image/prs.btif"]
    #[serde(rename = "image/prs.btif")]
    #[serde(alias = "btif")]
    #[serde(alias = "btf")]
    Ktx2,
    #[doc = "image/prs.pti"]
    #[serde(rename = "image/prs.pti")]
    #[serde(alias = "pti")]
    Naplps,
    #[doc = "image/pwg-raster"]
    #[serde(rename = "image/pwg-raster")]
    Png,
    #[doc = "image/svg+xml"]
    #[serde(rename = "image/svg+xml")]
    #[serde(alias = "svg")]
    #[serde(alias = "svgz")]
    PrsBtif,
    #[doc = "image/t38"]
    #[serde(rename = "image/t38")]
    #[serde(alias = "t38")]
    PrsPti,
    #[doc = "image/tiff"]
    #[serde(rename = "image/tiff")]
    #[serde(alias = "tiff")]
    #[serde(alias = "tif")]
    PwgRaster,
    #[doc = "image/tiff-fx"]
    #[serde(rename = "image/tiff-fx")]
    #[serde(alias = "tfx")]
    SvgXml,
    #[doc = "image/vnd.adobe.photoshop"]
    #[serde(rename = "image/vnd.adobe.photoshop")]
    #[serde(alias = "psd")]
    T38,
    #[doc = "image/vnd.airzip.accelerator.azv"]
    #[serde(rename = "image/vnd.airzip.accelerator.azv")]
    #[serde(alias = "azv")]
    Tiff,
    #[doc = "image/vnd.cns.inf2"]
    #[serde(rename = "image/vnd.cns.inf2")]
    TiffFx,
    #[doc = "image/vnd.dece.graphic"]
    #[serde(rename = "image/vnd.dece.graphic")]
    #[serde(alias = "uvi")]
    #[serde(alias = "uvvi")]
    #[serde(alias = "uvg")]
    #[serde(alias = "uvvg")]
    VndAdobePhotoshop,
    #[doc = "image/vnd.djvu"]
    #[serde(rename = "image/vnd.djvu")]
    #[serde(alias = "djvu")]
    #[serde(alias = "djv")]
    VndAirzipAcceleratorAzv,
    #[doc = "image/vnd.dwg"]
    #[serde(rename = "image/vnd.dwg")]
    #[serde(alias = "dwg")]
    VndCnsInf2,
    #[doc = "image/vnd.dxf"]
    #[serde(rename = "image/vnd.dxf")]
    #[serde(alias = "dxf")]
    VndDeceGraphic,
    #[doc = "image/vnd.dvb.subtitle"]
    #[serde(rename = "image/vnd.dvb.subtitle")]
    VndDjvu,
    #[doc = "image/vnd.fastbidsheet"]
    #[serde(rename = "image/vnd.fastbidsheet")]
    #[serde(alias = "fbs")]
    VndDwg,
    #[doc = "image/vnd.fpx"]
    #[serde(rename = "image/vnd.fpx")]
    #[serde(alias = "fpx")]
    VndDxf,
    #[doc = "image/vnd.fst"]
    #[serde(rename = "image/vnd.fst")]
    #[serde(alias = "fst")]
    VndDvbSubtitle,
    #[doc = "image/vnd.fujixerox.edmics-mmr"]
    #[serde(rename = "image/vnd.fujixerox.edmics-mmr")]
    #[serde(alias = "mmr")]
    VndFastbidsheet,
    #[doc = "image/vnd.fujixerox.edmics-rlc"]
    #[serde(rename = "image/vnd.fujixerox.edmics-rlc")]
    #[serde(alias = "rlc")]
    VndFpx,
    #[doc = "image/vnd.globalgraphics.pgb"]
    #[serde(rename = "image/vnd.globalgraphics.pgb")]
    #[serde(alias = "pgb")]
    VndFst,
    #[doc = "image/vnd.microsoft.icon"]
    #[serde(rename = "image/vnd.microsoft.icon")]
    #[serde(alias = "ico")]
    VndFujixeroxEdmicsMmr,
    #[doc = "image/vnd.mix"]
    #[serde(rename = "image/vnd.mix")]
    VndFujixeroxEdmicsRlc,
    #[doc = "image/vnd.ms-modi"]
    #[serde(rename = "image/vnd.ms-modi")]
    #[serde(alias = "mdi")]
    VndGlobalgraphicsPgb,
    #[doc = "image/vnd.mozilla.apng"]
    #[serde(rename = "image/vnd.mozilla.apng")]
    #[serde(alias = "apng")]
    VndMicrosoftIcon,
    #[doc = "image/vnd.net-fpx"]
    #[serde(rename = "image/vnd.net-fpx")]
    VndMix,
    #[doc = "image/vnd.pco.b16"]
    #[serde(rename = "image/vnd.pco.b16")]
    #[serde(alias = "b16")]
    VndMsModi,
    #[doc = "image/vnd.radiance"]
    #[serde(rename = "image/vnd.radiance")]
    #[serde(alias = "hdr")]
    #[serde(alias = "rgbe")]
    #[serde(alias = "xyze")]
    VndMozillaApng,
    #[doc = "image/vnd.sealed.png"]
    #[serde(rename = "image/vnd.sealed.png")]
    #[serde(alias = "spng")]
    #[serde(alias = "spn")]
    #[serde(alias = "s1n")]
    VndNetFpx,
    #[doc = "image/vnd.sealedmedia.softseal.gif"]
    #[serde(rename = "image/vnd.sealedmedia.softseal.gif")]
    #[serde(alias = "sgif")]
    #[serde(alias = "sgi")]
    #[serde(alias = "s1g")]
    VndPcoB16,
    #[doc = "image/vnd.sealedmedia.softseal.jpg"]
    #[serde(rename = "image/vnd.sealedmedia.softseal.jpg")]
    #[serde(alias = "sjpg")]
    #[serde(alias = "sjp")]
    #[serde(alias = "s1j")]
    VndRadiance,
    #[doc = "image/vnd.svf"]
    #[serde(rename = "image/vnd.svf")]
    VndSealedPng,
    #[doc = "image/vnd.tencent.tap"]
    #[serde(rename = "image/vnd.tencent.tap")]
    #[serde(alias = "tap")]
    VndSealedmediaSoftsealGif,
    #[doc = "image/vnd.valve.source.texture"]
    #[serde(rename = "image/vnd.valve.source.texture")]
    #[serde(alias = "vtf")]
    VndSealedmediaSoftsealJpg,
    #[doc = "image/vnd.wap.wbmp"]
    #[serde(rename = "image/vnd.wap.wbmp")]
    #[serde(alias = "wbmp")]
    VndSvf,
    #[doc = "image/vnd.xiff"]
    #[serde(rename = "image/vnd.xiff")]
    #[serde(alias = "xif")]
    VndTencentTap,
    #[doc = "image/vnd.zbrush.pcx"]
    #[serde(rename = "image/vnd.zbrush.pcx")]
    #[serde(alias = "pcx")]
    VndValveSourceTexture,
    #[doc = "image/webp"]
    #[serde(rename = "image/webp")]
    #[serde(alias = "webp")]
    VndWapWbmp,
    #[doc = "image/wmf"]
    #[serde(rename = "image/wmf")]
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
            "image/aces" | "exr" => Ok(Image::Aces),
            "image/apng" => Ok(Image::Apng),
            "image/avci" | "avci" => Ok(Image::Avci),
            "image/avcs" | "avcs" => Ok(Image::Avcs),
            "image/avif" | "avif" | "hif" => Ok(Image::Avif),
            "image/bmp" | "bmp" | "dib" => Ok(Image::Bmp),
            "image/cgm" | "cgm" => Ok(Image::Cgm),
            "image/dicom-rle" | "drle" => Ok(Image::DicomRle),
            "image/dpx" | "dpx" => Ok(Image::Dpx),
            "image/emf" | "emf" => Ok(Image::Emf),
            "image/example" => Ok(Image::Example),
            "image/fits" | "fits" | "fit" | "fts" => Ok(Image::Fits),
            "image/g3fax" => Ok(Image::G3Fax),
            "image/heic" | "heic" => Ok(Image::Gif),
            "image/heic-sequence" | "heics" => Ok(Image::Heic),
            "image/heif" | "heif" => Ok(Image::HeicSequence),
            "image/heif-sequence" | "heifs" => Ok(Image::Heif),
            "image/hej2k" | "hej2" => Ok(Image::HeifSequence),
            "image/hsj2" | "hsj2" => Ok(Image::Hej2K),
            "image/j2c" => Ok(Image::Hsj2),
            "image/jls" | "jls" => Ok(Image::Ief),
            "image/jp2" | "jp2" | "jpg2" => Ok(Image::J2C),
            "image/jph" | "jph" => Ok(Image::Jls),
            "image/jphc" | "jhc" => Ok(Image::Jp2),
            "image/jpm" | "jpm" | "jpgm" => Ok(Image::Jpeg),
            "image/jpx" | "jpx" | "jpf" => Ok(Image::Jph),
            "image/jxr" | "jxr" => Ok(Image::Jphc),
            "image/jxrA" | "jxra" => Ok(Image::Jpm),
            "image/jxrS" | "jxrs" => Ok(Image::Jpx),
            "image/jxs" | "jxs" => Ok(Image::Jxr),
            "image/jxsc" | "jxsc" => Ok(Image::JxrA),
            "image/jxsi" | "jxsi" => Ok(Image::JxrS),
            "image/jxss" | "jxss" => Ok(Image::Jxs),
            "image/ktx" | "ktx" => Ok(Image::Jxsc),
            "image/ktx2" | "ktx2" => Ok(Image::Jxsi),
            "image/naplps" => Ok(Image::Jxss),
            "image/png" | "png" => Ok(Image::Ktx),
            "image/prs.btif" | "btif" | "btf" => Ok(Image::Ktx2),
            "image/prs.pti" | "pti" => Ok(Image::Naplps),
            "image/pwg-raster" => Ok(Image::Png),
            "image/svg+xml" | "svg" | "svgz" => Ok(Image::PrsBtif),
            "image/t38" | "t38" => Ok(Image::PrsPti),
            "image/tiff" | "tiff" | "tif" => Ok(Image::PwgRaster),
            "image/tiff-fx" | "tfx" => Ok(Image::SvgXml),
            "image/vnd.adobe.photoshop" | "psd" => Ok(Image::T38),
            "image/vnd.airzip.accelerator.azv" | "azv" => Ok(Image::Tiff),
            "image/vnd.cns.inf2" => Ok(Image::TiffFx),
            "image/vnd.dece.graphic" | "uvi" | "uvvi" | "uvg" | "uvvg" => {
                Ok(Image::VndAdobePhotoshop)
            }
            "image/vnd.djvu" | "djvu" | "djv" => Ok(Image::VndAirzipAcceleratorAzv),
            "image/vnd.dwg" | "dwg" => Ok(Image::VndCnsInf2),
            "image/vnd.dxf" | "dxf" => Ok(Image::VndDeceGraphic),
            "image/vnd.dvb.subtitle" => Ok(Image::VndDjvu),
            "image/vnd.fastbidsheet" | "fbs" => Ok(Image::VndDwg),
            "image/vnd.fpx" | "fpx" => Ok(Image::VndDxf),
            "image/vnd.fst" | "fst" => Ok(Image::VndDvbSubtitle),
            "image/vnd.fujixerox.edmics-mmr" | "mmr" => Ok(Image::VndFastbidsheet),
            "image/vnd.fujixerox.edmics-rlc" | "rlc" => Ok(Image::VndFpx),
            "image/vnd.globalgraphics.pgb" | "pgb" => Ok(Image::VndFst),
            "image/vnd.microsoft.icon" | "ico" => Ok(Image::VndFujixeroxEdmicsMmr),
            "image/vnd.mix" => Ok(Image::VndFujixeroxEdmicsRlc),
            "image/vnd.ms-modi" | "mdi" => Ok(Image::VndGlobalgraphicsPgb),
            "image/vnd.mozilla.apng" | "apng" => Ok(Image::VndMicrosoftIcon),
            "image/vnd.net-fpx" => Ok(Image::VndMix),
            "image/vnd.pco.b16" | "b16" => Ok(Image::VndMsModi),
            "image/vnd.radiance" | "hdr" | "rgbe" | "xyze" => Ok(Image::VndMozillaApng),
            "image/vnd.sealed.png" | "spng" | "spn" | "s1n" => Ok(Image::VndNetFpx),
            "image/vnd.sealedmedia.softseal.gif" | "sgif" | "sgi" | "s1g" => Ok(Image::VndPcoB16),
            "image/vnd.sealedmedia.softseal.jpg" | "sjpg" | "sjp" | "s1j" => Ok(Image::VndRadiance),
            "image/vnd.svf" => Ok(Image::VndSealedPng),
            "image/vnd.tencent.tap" | "tap" => Ok(Image::VndSealedmediaSoftsealGif),
            "image/vnd.valve.source.texture" | "vtf" => Ok(Image::VndSealedmediaSoftsealJpg),
            "image/vnd.wap.wbmp" | "wbmp" => Ok(Image::VndSvf),
            "image/vnd.xiff" | "xif" => Ok(Image::VndTencentTap),
            "image/vnd.zbrush.pcx" | "pcx" => Ok(Image::VndValveSourceTexture),
            "image/webp" | "webp" => Ok(Image::VndWapWbmp),
            "image/wmf" | "wmf" => Ok(Image::VndXiff),
            _ => Err(()),
        }
    }
}
