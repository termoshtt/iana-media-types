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
pub enum Text {
    #[doc = "text/1d-interleaved-parityfec"]
    #[serde(rename = "text/1d-interleaved-parityfec")]
    _1DInterleavedParityfec,
    #[doc = "text/cache-manifest"]
    #[serde(rename = "text/cache-manifest")]
    #[serde(alias = "appcache")]
    #[serde(alias = "manifest")]
    CacheManifest,
    #[doc = "text/calendar"]
    #[serde(rename = "text/calendar")]
    #[serde(alias = "ics")]
    #[serde(alias = "ifb")]
    Calendar,
    #[doc = "text/cql"]
    #[serde(rename = "text/cql")]
    #[serde(alias = "CQL")]
    Cql,
    #[doc = "text/cql-expression"]
    #[serde(rename = "text/cql-expression")]
    CqlExpression,
    #[doc = "text/cql-identifier"]
    #[serde(rename = "text/cql-identifier")]
    CqlIdentifier,
    #[doc = "text/css"]
    #[serde(rename = "text/css")]
    #[serde(alias = "css")]
    Css,
    #[doc = "text/csv"]
    #[serde(rename = "text/csv")]
    #[serde(alias = "csv")]
    Csv,
    #[doc = "text/csv-schema"]
    #[serde(rename = "text/csv-schema")]
    #[serde(alias = "csvs")]
    CsvSchema,
    #[doc = "text/dns"]
    #[serde(rename = "text/dns")]
    #[serde(alias = "soa")]
    #[serde(alias = "zone")]
    Dns,
    #[doc = "text/encaprtp"]
    #[serde(rename = "text/encaprtp")]
    Encaprtp,
    #[doc = "text/example"]
    #[serde(rename = "text/example")]
    Enriched,
    #[doc = "text/fhirpath"]
    #[serde(rename = "text/fhirpath")]
    Example,
    #[doc = "text/flexfec"]
    #[serde(rename = "text/flexfec")]
    Fhirpath,
    #[doc = "text/fwdred"]
    #[serde(rename = "text/fwdred")]
    Flexfec,
    #[doc = "text/gff3"]
    #[serde(rename = "text/gff3")]
    #[serde(alias = "gff3")]
    Fwdred,
    #[doc = "text/grammar-ref-list"]
    #[serde(rename = "text/grammar-ref-list")]
    Gff3,
    #[doc = "text/hl7v2"]
    #[serde(rename = "text/hl7v2")]
    GrammarRefList,
    #[doc = "text/html"]
    #[serde(rename = "text/html")]
    #[serde(alias = "html")]
    #[serde(alias = "htm")]
    Hl7V2,
    #[doc = "text/javascript"]
    #[serde(rename = "text/javascript")]
    #[serde(alias = "js")]
    #[serde(alias = "mjs")]
    Html,
    #[doc = "text/jcr-cnd"]
    #[serde(rename = "text/jcr-cnd")]
    #[serde(alias = "cnd")]
    Javascript,
    #[doc = "text/markdown"]
    #[serde(rename = "text/markdown")]
    #[serde(alias = "markdown")]
    #[serde(alias = "md")]
    JcrCnd,
    #[doc = "text/mizar"]
    #[serde(rename = "text/mizar")]
    #[serde(alias = "miz")]
    Markdown,
    #[doc = "text/n3"]
    #[serde(rename = "text/n3")]
    #[serde(alias = "n3")]
    Mizar,
    #[doc = "text/parameters"]
    #[serde(rename = "text/parameters")]
    N3,
    #[doc = "text/parityfec"]
    #[serde(rename = "text/parityfec")]
    Parameters,
    #[doc = "text/provenance-notation"]
    #[serde(rename = "text/provenance-notation")]
    #[serde(alias = "provn")]
    Parityfec,
    #[doc = "text/prs.fallenstein.rst"]
    #[serde(rename = "text/prs.fallenstein.rst")]
    #[serde(alias = "rst")]
    Plain,
    #[doc = "text/prs.lines.tag"]
    #[serde(rename = "text/prs.lines.tag")]
    #[serde(alias = "tag")]
    #[serde(alias = "dsc")]
    ProvenanceNotation,
    #[doc = "text/prs.prop.logic"]
    #[serde(rename = "text/prs.prop.logic")]
    PrsFallensteinRst,
    #[doc = "text/raptorfec"]
    #[serde(rename = "text/raptorfec")]
    PrsLinesTag,
    #[doc = "text/RED"]
    #[serde(rename = "text/RED")]
    PrsPropLogic,
    #[doc = "text/rfc822-headers"]
    #[serde(rename = "text/rfc822-headers")]
    Raptorfec,
    #[doc = "text/rtf"]
    #[serde(rename = "text/rtf")]
    Red,
    #[doc = "text/rtp-enc-aescm128"]
    #[serde(rename = "text/rtp-enc-aescm128")]
    Rfc822Headers,
    #[doc = "text/rtploopback"]
    #[serde(rename = "text/rtploopback")]
    Richtext,
    #[doc = "text/rtx"]
    #[serde(rename = "text/rtx")]
    Rtf,
    #[doc = "text/SGML"]
    #[serde(rename = "text/SGML")]
    #[serde(alias = "sgml")]
    #[serde(alias = "sgm")]
    RtpEncAescm128,
    #[doc = "text/shaclc"]
    #[serde(rename = "text/shaclc")]
    #[serde(alias = "shaclc")]
    #[serde(alias = "shc")]
    Rtploopback,
    #[doc = "text/shex"]
    #[serde(rename = "text/shex")]
    #[serde(alias = "shex")]
    Rtx,
    #[doc = "text/spdx"]
    #[serde(rename = "text/spdx")]
    #[serde(alias = "spdx")]
    Sgml,
    #[doc = "text/strings"]
    #[serde(rename = "text/strings")]
    Shaclc,
    #[doc = "text/t140"]
    #[serde(rename = "text/t140")]
    Shex,
    #[doc = "text/tab-separated-values"]
    #[serde(rename = "text/tab-separated-values")]
    #[serde(alias = "tsv")]
    Spdx,
    #[doc = "text/troff"]
    #[serde(rename = "text/troff")]
    #[serde(alias = "t")]
    #[serde(alias = "tr")]
    #[serde(alias = "roff")]
    Strings,
    #[doc = "text/turtle"]
    #[serde(rename = "text/turtle")]
    #[serde(alias = "ttl")]
    T140,
    #[doc = "text/ulpfec"]
    #[serde(rename = "text/ulpfec")]
    TabSeparatedValues,
    #[doc = "text/uri-list"]
    #[serde(rename = "text/uri-list")]
    #[serde(alias = "uris")]
    #[serde(alias = "uri")]
    Troff,
    #[doc = "text/vcard"]
    #[serde(rename = "text/vcard")]
    #[serde(alias = "vcf")]
    #[serde(alias = "vcard")]
    Turtle,
    #[doc = "text/vnd.a"]
    #[serde(rename = "text/vnd.a")]
    #[serde(alias = "a")]
    Ulpfec,
    #[doc = "text/vnd.abc"]
    #[serde(rename = "text/vnd.abc")]
    #[serde(alias = "abc")]
    UriList,
    #[doc = "text/vnd.ascii-art"]
    #[serde(rename = "text/vnd.ascii-art")]
    #[serde(alias = "ascii")]
    Vcard,
    #[doc = "text/vnd.curl"]
    #[serde(rename = "text/vnd.curl")]
    VndA,
    #[doc = "text/vnd.debian.copyright"]
    #[serde(rename = "text/vnd.debian.copyright")]
    #[serde(alias = "copyright")]
    VndAbc,
    #[doc = "text/vnd.DMClientScript"]
    #[serde(rename = "text/vnd.DMClientScript")]
    #[serde(alias = "dms")]
    VndAsciiArt,
    #[doc = "text/vnd.dvb.subtitle"]
    #[serde(rename = "text/vnd.dvb.subtitle")]
    #[serde(alias = "sub")]
    VndCurl,
    #[doc = "text/vnd.esmertec.theme-descriptor"]
    #[serde(rename = "text/vnd.esmertec.theme-descriptor")]
    #[serde(alias = "jtd")]
    VndDebianCopyright,
    #[doc = "text/vnd.exchangeable"]
    #[serde(rename = "text/vnd.exchangeable")]
    #[serde(alias = "vfk")]
    VndDMClientScript,
    #[doc = "text/vnd.familysearch.gedcom"]
    #[serde(rename = "text/vnd.familysearch.gedcom")]
    #[serde(alias = "ged")]
    VndDvbSubtitle,
    #[doc = "text/vnd.ficlab.flt"]
    #[serde(rename = "text/vnd.ficlab.flt")]
    #[serde(alias = "flt")]
    VndEsmertecThemeDescriptor,
    #[doc = "text/vnd.fly"]
    #[serde(rename = "text/vnd.fly")]
    #[serde(alias = "fly")]
    VndExchangeable,
    #[doc = "text/vnd.fmi.flexstor"]
    #[serde(rename = "text/vnd.fmi.flexstor")]
    #[serde(alias = "flx")]
    VndFamilysearchGedcom,
    #[doc = "text/vnd.gml"]
    #[serde(rename = "text/vnd.gml")]
    VndFiclabFlt,
    #[doc = "text/vnd.graphviz"]
    #[serde(rename = "text/vnd.graphviz")]
    #[serde(alias = "gv")]
    #[serde(alias = "dot")]
    VndFly,
    #[doc = "text/vnd.hans"]
    #[serde(rename = "text/vnd.hans")]
    #[serde(alias = "hans")]
    VndFmiFlexstor,
    #[doc = "text/vnd.hgl"]
    #[serde(rename = "text/vnd.hgl")]
    #[serde(alias = "hgl")]
    VndGml,
    #[doc = "text/vnd.in3d.3dml"]
    #[serde(rename = "text/vnd.in3d.3dml")]
    #[serde(alias = "3dml")]
    #[serde(alias = "3dm")]
    VndGraphviz,
    #[doc = "text/vnd.in3d.spot"]
    #[serde(rename = "text/vnd.in3d.spot")]
    #[serde(alias = "spot")]
    #[serde(alias = "spo")]
    VndHans,
    #[doc = "text/vnd.IPTC.NewsML"]
    #[serde(rename = "text/vnd.IPTC.NewsML")]
    VndHgl,
    #[doc = "text/vnd.IPTC.NITF"]
    #[serde(rename = "text/vnd.IPTC.NITF")]
    VndIn3D3Dml,
    #[doc = "text/vnd.latex-z"]
    #[serde(rename = "text/vnd.latex-z")]
    VndIn3DSpot,
    #[doc = "text/vnd.motorola.reflex"]
    #[serde(rename = "text/vnd.motorola.reflex")]
    VndIPTCNewsML,
    #[doc = "text/vnd.ms-mediapackage"]
    #[serde(rename = "text/vnd.ms-mediapackage")]
    #[serde(alias = "mpf")]
    VndIPTCNITF,
    #[doc = "text/vnd.net2phone.commcenter.command"]
    #[serde(rename = "text/vnd.net2phone.commcenter.command")]
    #[serde(alias = "ccc")]
    VndLatexZ,
    #[doc = "text/vnd.radisys.msml-basic-layout"]
    #[serde(rename = "text/vnd.radisys.msml-basic-layout")]
    VndMotorolaReflex,
    #[doc = "text/vnd.senx.warpscript"]
    #[serde(rename = "text/vnd.senx.warpscript")]
    #[serde(alias = "mc2")]
    VndMsMediapackage,
    #[doc = "text/vnd.sun.j2me.app-descriptor"]
    #[serde(rename = "text/vnd.sun.j2me.app-descriptor")]
    #[serde(alias = "jad")]
    VndNet2PhoneCommcenterCommand,
    #[doc = "text/vnd.sosi"]
    #[serde(rename = "text/vnd.sosi")]
    #[serde(alias = "sos")]
    VndRadisysMsmlBasicLayout,
    #[doc = "text/vnd.trolltech.linguist"]
    #[serde(rename = "text/vnd.trolltech.linguist")]
    #[serde(alias = "ts")]
    VndSenxWarpscript,
    #[doc = "text/vnd.wap.si"]
    #[serde(rename = "text/vnd.wap.si")]
    #[serde(alias = "si")]
    VndSunJ2MeAppDescriptor,
    #[doc = "text/vnd.wap.sl"]
    #[serde(rename = "text/vnd.wap.sl")]
    #[serde(alias = "sl")]
    VndSosi,
    #[doc = "text/vnd.wap.wml"]
    #[serde(rename = "text/vnd.wap.wml")]
    #[serde(alias = "wml")]
    VndTrolltechLinguist,
    #[doc = "text/vnd.wap.wmlscript"]
    #[serde(rename = "text/vnd.wap.wmlscript")]
    #[serde(alias = "wmls")]
    VndWapSi,
    #[doc = "text/vtt"]
    #[serde(rename = "text/vtt")]
    #[serde(alias = "vtt")]
    VndWapSl,
    #[doc = "text/wgsl"]
    #[serde(rename = "text/wgsl")]
    #[serde(alias = "wgsl")]
    VndWapWml,
    #[doc = "text/xml"]
    #[serde(rename = "text/xml")]
    #[serde(alias = "xml")]
    #[serde(alias = "xsd")]
    #[serde(alias = "rng")]
    VndWapWmlscript,
    #[doc = "text/xml-external-parsed-entity"]
    #[serde(rename = "text/xml-external-parsed-entity")]
    #[serde(alias = "ent")]
    Vtt,
}
impl ::std::fmt::Display for Text {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Text::_1DInterleavedParityfec => write!(f, "text/1d-interleaved-parityfec")?,
            Text::CacheManifest => write!(f, "text/cache-manifest")?,
            Text::Calendar => write!(f, "text/calendar")?,
            Text::Cql => write!(f, "text/cql")?,
            Text::CqlExpression => write!(f, "text/cql-expression")?,
            Text::CqlIdentifier => write!(f, "text/cql-identifier")?,
            Text::Css => write!(f, "text/css")?,
            Text::Csv => write!(f, "text/csv")?,
            Text::CsvSchema => write!(f, "text/csv-schema")?,
            Text::Dns => write!(f, "text/dns")?,
            Text::Encaprtp => write!(f, "text/encaprtp")?,
            Text::Enriched => write!(f, "text/example")?,
            Text::Example => write!(f, "text/fhirpath")?,
            Text::Fhirpath => write!(f, "text/flexfec")?,
            Text::Flexfec => write!(f, "text/fwdred")?,
            Text::Fwdred => write!(f, "text/gff3")?,
            Text::Gff3 => write!(f, "text/grammar-ref-list")?,
            Text::GrammarRefList => write!(f, "text/hl7v2")?,
            Text::Hl7V2 => write!(f, "text/html")?,
            Text::Html => write!(f, "text/javascript")?,
            Text::Javascript => write!(f, "text/jcr-cnd")?,
            Text::JcrCnd => write!(f, "text/markdown")?,
            Text::Markdown => write!(f, "text/mizar")?,
            Text::Mizar => write!(f, "text/n3")?,
            Text::N3 => write!(f, "text/parameters")?,
            Text::Parameters => write!(f, "text/parityfec")?,
            Text::Parityfec => write!(f, "text/provenance-notation")?,
            Text::Plain => write!(f, "text/prs.fallenstein.rst")?,
            Text::ProvenanceNotation => write!(f, "text/prs.lines.tag")?,
            Text::PrsFallensteinRst => write!(f, "text/prs.prop.logic")?,
            Text::PrsLinesTag => write!(f, "text/raptorfec")?,
            Text::PrsPropLogic => write!(f, "text/RED")?,
            Text::Raptorfec => write!(f, "text/rfc822-headers")?,
            Text::Red => write!(f, "text/rtf")?,
            Text::Rfc822Headers => write!(f, "text/rtp-enc-aescm128")?,
            Text::Richtext => write!(f, "text/rtploopback")?,
            Text::Rtf => write!(f, "text/rtx")?,
            Text::RtpEncAescm128 => write!(f, "text/SGML")?,
            Text::Rtploopback => write!(f, "text/shaclc")?,
            Text::Rtx => write!(f, "text/shex")?,
            Text::Sgml => write!(f, "text/spdx")?,
            Text::Shaclc => write!(f, "text/strings")?,
            Text::Shex => write!(f, "text/t140")?,
            Text::Spdx => write!(f, "text/tab-separated-values")?,
            Text::Strings => write!(f, "text/troff")?,
            Text::T140 => write!(f, "text/turtle")?,
            Text::TabSeparatedValues => write!(f, "text/ulpfec")?,
            Text::Troff => write!(f, "text/uri-list")?,
            Text::Turtle => write!(f, "text/vcard")?,
            Text::Ulpfec => write!(f, "text/vnd.a")?,
            Text::UriList => write!(f, "text/vnd.abc")?,
            Text::Vcard => write!(f, "text/vnd.ascii-art")?,
            Text::VndA => write!(f, "text/vnd.curl")?,
            Text::VndAbc => write!(f, "text/vnd.debian.copyright")?,
            Text::VndAsciiArt => write!(f, "text/vnd.DMClientScript")?,
            Text::VndCurl => write!(f, "text/vnd.dvb.subtitle")?,
            Text::VndDebianCopyright => write!(f, "text/vnd.esmertec.theme-descriptor")?,
            Text::VndDMClientScript => write!(f, "text/vnd.exchangeable")?,
            Text::VndDvbSubtitle => write!(f, "text/vnd.familysearch.gedcom")?,
            Text::VndEsmertecThemeDescriptor => write!(f, "text/vnd.ficlab.flt")?,
            Text::VndExchangeable => write!(f, "text/vnd.fly")?,
            Text::VndFamilysearchGedcom => write!(f, "text/vnd.fmi.flexstor")?,
            Text::VndFiclabFlt => write!(f, "text/vnd.gml")?,
            Text::VndFly => write!(f, "text/vnd.graphviz")?,
            Text::VndFmiFlexstor => write!(f, "text/vnd.hans")?,
            Text::VndGml => write!(f, "text/vnd.hgl")?,
            Text::VndGraphviz => write!(f, "text/vnd.in3d.3dml")?,
            Text::VndHans => write!(f, "text/vnd.in3d.spot")?,
            Text::VndHgl => write!(f, "text/vnd.IPTC.NewsML")?,
            Text::VndIn3D3Dml => write!(f, "text/vnd.IPTC.NITF")?,
            Text::VndIn3DSpot => write!(f, "text/vnd.latex-z")?,
            Text::VndIPTCNewsML => write!(f, "text/vnd.motorola.reflex")?,
            Text::VndIPTCNITF => write!(f, "text/vnd.ms-mediapackage")?,
            Text::VndLatexZ => write!(f, "text/vnd.net2phone.commcenter.command")?,
            Text::VndMotorolaReflex => write!(f, "text/vnd.radisys.msml-basic-layout")?,
            Text::VndMsMediapackage => write!(f, "text/vnd.senx.warpscript")?,
            Text::VndNet2PhoneCommcenterCommand => write!(f, "text/vnd.sun.j2me.app-descriptor")?,
            Text::VndRadisysMsmlBasicLayout => write!(f, "text/vnd.sosi")?,
            Text::VndSenxWarpscript => write!(f, "text/vnd.trolltech.linguist")?,
            Text::VndSunJ2MeAppDescriptor => write!(f, "text/vnd.wap.si")?,
            Text::VndSosi => write!(f, "text/vnd.wap.sl")?,
            Text::VndTrolltechLinguist => write!(f, "text/vnd.wap.wml")?,
            Text::VndWapSi => write!(f, "text/vnd.wap.wmlscript")?,
            Text::VndWapSl => write!(f, "text/vtt")?,
            Text::VndWapWml => write!(f, "text/wgsl")?,
            Text::VndWapWmlscript => write!(f, "text/xml")?,
            Text::Vtt => write!(f, "text/xml-external-parsed-entity")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Text {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "text/1d-interleaved-parityfec" => Ok(Text::_1DInterleavedParityfec),
            "text/cache-manifest" | "appcache" | "manifest" => Ok(Text::CacheManifest),
            "text/calendar" | "ics" | "ifb" => Ok(Text::Calendar),
            "text/cql" | "CQL" => Ok(Text::Cql),
            "text/cql-expression" => Ok(Text::CqlExpression),
            "text/cql-identifier" => Ok(Text::CqlIdentifier),
            "text/css" | "css" => Ok(Text::Css),
            "text/csv" | "csv" => Ok(Text::Csv),
            "text/csv-schema" | "csvs" => Ok(Text::CsvSchema),
            "text/dns" | "soa" | "zone" => Ok(Text::Dns),
            "text/encaprtp" => Ok(Text::Encaprtp),
            "text/example" => Ok(Text::Enriched),
            "text/fhirpath" => Ok(Text::Example),
            "text/flexfec" => Ok(Text::Fhirpath),
            "text/fwdred" => Ok(Text::Flexfec),
            "text/gff3" | "gff3" => Ok(Text::Fwdred),
            "text/grammar-ref-list" => Ok(Text::Gff3),
            "text/hl7v2" => Ok(Text::GrammarRefList),
            "text/html" | "html" | "htm" => Ok(Text::Hl7V2),
            "text/javascript" | "js" | "mjs" => Ok(Text::Html),
            "text/jcr-cnd" | "cnd" => Ok(Text::Javascript),
            "text/markdown" | "markdown" | "md" => Ok(Text::JcrCnd),
            "text/mizar" | "miz" => Ok(Text::Markdown),
            "text/n3" | "n3" => Ok(Text::Mizar),
            "text/parameters" => Ok(Text::N3),
            "text/parityfec" => Ok(Text::Parameters),
            "text/provenance-notation" | "provn" => Ok(Text::Parityfec),
            "text/prs.fallenstein.rst" | "rst" => Ok(Text::Plain),
            "text/prs.lines.tag" | "tag" | "dsc" => Ok(Text::ProvenanceNotation),
            "text/prs.prop.logic" => Ok(Text::PrsFallensteinRst),
            "text/raptorfec" => Ok(Text::PrsLinesTag),
            "text/RED" => Ok(Text::PrsPropLogic),
            "text/rfc822-headers" => Ok(Text::Raptorfec),
            "text/rtf" => Ok(Text::Red),
            "text/rtp-enc-aescm128" => Ok(Text::Rfc822Headers),
            "text/rtploopback" => Ok(Text::Richtext),
            "text/rtx" => Ok(Text::Rtf),
            "text/SGML" | "sgml" | "sgm" => Ok(Text::RtpEncAescm128),
            "text/shaclc" | "shaclc" | "shc" => Ok(Text::Rtploopback),
            "text/shex" | "shex" => Ok(Text::Rtx),
            "text/spdx" | "spdx" => Ok(Text::Sgml),
            "text/strings" => Ok(Text::Shaclc),
            "text/t140" => Ok(Text::Shex),
            "text/tab-separated-values" | "tsv" => Ok(Text::Spdx),
            "text/troff" | "t" | "tr" | "roff" => Ok(Text::Strings),
            "text/turtle" | "ttl" => Ok(Text::T140),
            "text/ulpfec" => Ok(Text::TabSeparatedValues),
            "text/uri-list" | "uris" | "uri" => Ok(Text::Troff),
            "text/vcard" | "vcf" | "vcard" => Ok(Text::Turtle),
            "text/vnd.a" | "a" => Ok(Text::Ulpfec),
            "text/vnd.abc" | "abc" => Ok(Text::UriList),
            "text/vnd.ascii-art" | "ascii" => Ok(Text::Vcard),
            "text/vnd.curl" => Ok(Text::VndA),
            "text/vnd.debian.copyright" | "copyright" => Ok(Text::VndAbc),
            "text/vnd.DMClientScript" | "dms" => Ok(Text::VndAsciiArt),
            "text/vnd.dvb.subtitle" | "sub" => Ok(Text::VndCurl),
            "text/vnd.esmertec.theme-descriptor" | "jtd" => Ok(Text::VndDebianCopyright),
            "text/vnd.exchangeable" | "vfk" => Ok(Text::VndDMClientScript),
            "text/vnd.familysearch.gedcom" | "ged" => Ok(Text::VndDvbSubtitle),
            "text/vnd.ficlab.flt" | "flt" => Ok(Text::VndEsmertecThemeDescriptor),
            "text/vnd.fly" | "fly" => Ok(Text::VndExchangeable),
            "text/vnd.fmi.flexstor" | "flx" => Ok(Text::VndFamilysearchGedcom),
            "text/vnd.gml" => Ok(Text::VndFiclabFlt),
            "text/vnd.graphviz" | "gv" | "dot" => Ok(Text::VndFly),
            "text/vnd.hans" | "hans" => Ok(Text::VndFmiFlexstor),
            "text/vnd.hgl" | "hgl" => Ok(Text::VndGml),
            "text/vnd.in3d.3dml" | "3dml" | "3dm" => Ok(Text::VndGraphviz),
            "text/vnd.in3d.spot" | "spot" | "spo" => Ok(Text::VndHans),
            "text/vnd.IPTC.NewsML" => Ok(Text::VndHgl),
            "text/vnd.IPTC.NITF" => Ok(Text::VndIn3D3Dml),
            "text/vnd.latex-z" => Ok(Text::VndIn3DSpot),
            "text/vnd.motorola.reflex" => Ok(Text::VndIPTCNewsML),
            "text/vnd.ms-mediapackage" | "mpf" => Ok(Text::VndIPTCNITF),
            "text/vnd.net2phone.commcenter.command" | "ccc" => Ok(Text::VndLatexZ),
            "text/vnd.radisys.msml-basic-layout" => Ok(Text::VndMotorolaReflex),
            "text/vnd.senx.warpscript" | "mc2" => Ok(Text::VndMsMediapackage),
            "text/vnd.sun.j2me.app-descriptor" | "jad" => Ok(Text::VndNet2PhoneCommcenterCommand),
            "text/vnd.sosi" | "sos" => Ok(Text::VndRadisysMsmlBasicLayout),
            "text/vnd.trolltech.linguist" | "ts" => Ok(Text::VndSenxWarpscript),
            "text/vnd.wap.si" | "si" => Ok(Text::VndSunJ2MeAppDescriptor),
            "text/vnd.wap.sl" | "sl" => Ok(Text::VndSosi),
            "text/vnd.wap.wml" | "wml" => Ok(Text::VndTrolltechLinguist),
            "text/vnd.wap.wmlscript" | "wmls" => Ok(Text::VndWapSi),
            "text/vtt" | "vtt" => Ok(Text::VndWapSl),
            "text/wgsl" | "wgsl" => Ok(Text::VndWapWml),
            "text/xml" | "xml" | "xsd" | "rng" => Ok(Text::VndWapWmlscript),
            "text/xml-external-parsed-entity" | "ent" => Ok(Text::Vtt),
            _ => Err(()),
        }
    }
}
