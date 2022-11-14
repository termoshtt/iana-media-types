#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Text {
    #[doc = "text/1d-interleaved-parityfec"]
    _1DInterleavedParityfec,
    #[doc = "text/cache-manifest"]
    CacheManifest,
    #[doc = "text/calendar"]
    Calendar,
    #[doc = "text/cql"]
    Cql,
    #[doc = "text/cql-expression"]
    CqlExpression,
    #[doc = "text/cql-identifier"]
    CqlIdentifier,
    #[doc = "text/css"]
    Css,
    #[doc = "text/csv"]
    Csv,
    #[doc = "text/csv-schema"]
    CsvSchema,
    #[doc = "text/directory"]
    DirectoryDEPRECATEDByRFC6350,
    #[doc = "text/dns"]
    Dns,
    #[doc = "text/encaprtp"]
    Encaprtp,
    #[doc = "text/example"]
    Example,
    #[doc = "text/fhirpath"]
    Fhirpath,
    #[doc = "text/flexfec"]
    Flexfec,
    #[doc = "text/fwdred"]
    Fwdred,
    #[doc = "text/gff3"]
    Gff3,
    #[doc = "text/grammar-ref-list"]
    GrammarRefList,
    #[doc = "text/hl7v2"]
    Hl7V2,
    #[doc = "text/html"]
    Html,
    #[doc = "text/javascript"]
    Javascript,
    #[doc = "text/jcr-cnd"]
    JcrCnd,
    #[doc = "text/markdown"]
    Markdown,
    #[doc = "text/mizar"]
    Mizar,
    #[doc = "text/n3"]
    N3,
    #[doc = "text/parameters"]
    Parameters,
    #[doc = "text/parityfec"]
    Parityfec,
    #[doc = "text/provenance-notation"]
    ProvenanceNotation,
    #[doc = "text/prs.fallenstein.rst"]
    PrsFallensteinRst,
    #[doc = "text/prs.lines.tag"]
    PrsLinesTag,
    #[doc = "text/prs.prop.logic"]
    PrsPropLogic,
    #[doc = "text/raptorfec"]
    Raptorfec,
    #[doc = "text/RED"]
    Red,
    #[doc = "text/rfc822-headers"]
    Rfc822Headers,
    #[doc = "text/rtf"]
    Rtf,
    #[doc = "text/rtp-enc-aescm128"]
    RtpEncAescm128,
    #[doc = "text/rtploopback"]
    Rtploopback,
    #[doc = "text/rtx"]
    Rtx,
    #[doc = "text/SGML"]
    Sgml,
    #[doc = "text/shaclc"]
    Shaclc,
    #[doc = "text/shex"]
    Shex,
    #[doc = "text/spdx"]
    Spdx,
    #[doc = "text/strings"]
    Strings,
    #[doc = "text/t140"]
    T140,
    #[doc = "text/tab-separated-values"]
    TabSeparatedValues,
    #[doc = "text/troff"]
    Troff,
    #[doc = "text/turtle"]
    Turtle,
    #[doc = "text/ulpfec"]
    Ulpfec,
    #[doc = "text/uri-list"]
    UriList,
    #[doc = "text/vcard"]
    Vcard,
    #[doc = "text/vnd.a"]
    VndA,
    #[doc = "text/vnd.abc"]
    VndAbc,
    #[doc = "text/vnd.ascii-art"]
    VndAsciiArt,
    #[doc = "text/vnd.curl"]
    VndCurl,
    #[doc = "text/vnd.debian.copyright"]
    VndDebianCopyright,
    #[doc = "text/vnd.DMClientScript"]
    VndDMClientScript,
    #[doc = "text/vnd.dvb.subtitle"]
    VndDvbSubtitle,
    #[doc = "text/vnd.esmertec.theme-descriptor"]
    VndEsmertecThemeDescriptor,
    #[doc = "text/vnd.exchangeable"]
    VndExchangeable,
    #[doc = "text/vnd.familysearch.gedcom"]
    VndFamilysearchGedcom,
    #[doc = "text/vnd.ficlab.flt"]
    VndFiclabFlt,
    #[doc = "text/vnd.fly"]
    VndFly,
    #[doc = "text/vnd.fmi.flexstor"]
    VndFmiFlexstor,
    #[doc = "text/vnd.gml"]
    VndGml,
    #[doc = "text/vnd.graphviz"]
    VndGraphviz,
    #[doc = "text/vnd.hans"]
    VndHans,
    #[doc = "text/vnd.hgl"]
    VndHgl,
    #[doc = "text/vnd.in3d.3dml"]
    VndIn3D3Dml,
    #[doc = "text/vnd.in3d.spot"]
    VndIn3DSpot,
    #[doc = "text/vnd.IPTC.NewsML"]
    VndIPTCNewsML,
    #[doc = "text/vnd.IPTC.NITF"]
    VndIPTCNITF,
    #[doc = "text/vnd.latex-z"]
    VndLatexZ,
    #[doc = "text/vnd.motorola.reflex"]
    VndMotorolaReflex,
    #[doc = "text/vnd.ms-mediapackage"]
    VndMsMediapackage,
    #[doc = "text/vnd.net2phone.commcenter.command"]
    VndNet2PhoneCommcenterCommand,
    #[doc = "text/vnd.radisys.msml-basic-layout"]
    VndRadisysMsmlBasicLayout,
    #[doc = "text/vnd.senx.warpscript"]
    VndSenxWarpscript,
    #[doc = "text/vnd.sun.j2me.app-descriptor"]
    VndSunJ2MeAppDescriptor,
    #[doc = "text/vnd.sosi"]
    VndSosi,
    #[doc = "text/vnd.trolltech.linguist"]
    VndTrolltechLinguist,
    #[doc = "text/vnd.wap.si"]
    VndWapSi,
    #[doc = "text/vnd.wap.sl"]
    VndWapSl,
    #[doc = "text/vnd.wap.wml"]
    VndWapWml,
    #[doc = "text/vnd.wap.wmlscript"]
    VndWapWmlscript,
    #[doc = "text/vtt"]
    Vtt,
    #[doc = "text/xml"]
    Xml,
    #[doc = "text/xml-external-parsed-entity"]
    XmlExternalParsedEntity,
    Other(String),
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
            Text::DirectoryDEPRECATEDByRFC6350 => write!(f, "text/directory")?,
            Text::Dns => write!(f, "text/dns")?,
            Text::Encaprtp => write!(f, "text/encaprtp")?,
            Text::Example => write!(f, "text/example")?,
            Text::Fhirpath => write!(f, "text/fhirpath")?,
            Text::Flexfec => write!(f, "text/flexfec")?,
            Text::Fwdred => write!(f, "text/fwdred")?,
            Text::Gff3 => write!(f, "text/gff3")?,
            Text::GrammarRefList => write!(f, "text/grammar-ref-list")?,
            Text::Hl7V2 => write!(f, "text/hl7v2")?,
            Text::Html => write!(f, "text/html")?,
            Text::Javascript => write!(f, "text/javascript")?,
            Text::JcrCnd => write!(f, "text/jcr-cnd")?,
            Text::Markdown => write!(f, "text/markdown")?,
            Text::Mizar => write!(f, "text/mizar")?,
            Text::N3 => write!(f, "text/n3")?,
            Text::Parameters => write!(f, "text/parameters")?,
            Text::Parityfec => write!(f, "text/parityfec")?,
            Text::ProvenanceNotation => write!(f, "text/provenance-notation")?,
            Text::PrsFallensteinRst => write!(f, "text/prs.fallenstein.rst")?,
            Text::PrsLinesTag => write!(f, "text/prs.lines.tag")?,
            Text::PrsPropLogic => write!(f, "text/prs.prop.logic")?,
            Text::Raptorfec => write!(f, "text/raptorfec")?,
            Text::Red => write!(f, "text/RED")?,
            Text::Rfc822Headers => write!(f, "text/rfc822-headers")?,
            Text::Rtf => write!(f, "text/rtf")?,
            Text::RtpEncAescm128 => write!(f, "text/rtp-enc-aescm128")?,
            Text::Rtploopback => write!(f, "text/rtploopback")?,
            Text::Rtx => write!(f, "text/rtx")?,
            Text::Sgml => write!(f, "text/SGML")?,
            Text::Shaclc => write!(f, "text/shaclc")?,
            Text::Shex => write!(f, "text/shex")?,
            Text::Spdx => write!(f, "text/spdx")?,
            Text::Strings => write!(f, "text/strings")?,
            Text::T140 => write!(f, "text/t140")?,
            Text::TabSeparatedValues => write!(f, "text/tab-separated-values")?,
            Text::Troff => write!(f, "text/troff")?,
            Text::Turtle => write!(f, "text/turtle")?,
            Text::Ulpfec => write!(f, "text/ulpfec")?,
            Text::UriList => write!(f, "text/uri-list")?,
            Text::Vcard => write!(f, "text/vcard")?,
            Text::VndA => write!(f, "text/vnd.a")?,
            Text::VndAbc => write!(f, "text/vnd.abc")?,
            Text::VndAsciiArt => write!(f, "text/vnd.ascii-art")?,
            Text::VndCurl => write!(f, "text/vnd.curl")?,
            Text::VndDebianCopyright => write!(f, "text/vnd.debian.copyright")?,
            Text::VndDMClientScript => write!(f, "text/vnd.DMClientScript")?,
            Text::VndDvbSubtitle => write!(f, "text/vnd.dvb.subtitle")?,
            Text::VndEsmertecThemeDescriptor => write!(f, "text/vnd.esmertec.theme-descriptor")?,
            Text::VndExchangeable => write!(f, "text/vnd.exchangeable")?,
            Text::VndFamilysearchGedcom => write!(f, "text/vnd.familysearch.gedcom")?,
            Text::VndFiclabFlt => write!(f, "text/vnd.ficlab.flt")?,
            Text::VndFly => write!(f, "text/vnd.fly")?,
            Text::VndFmiFlexstor => write!(f, "text/vnd.fmi.flexstor")?,
            Text::VndGml => write!(f, "text/vnd.gml")?,
            Text::VndGraphviz => write!(f, "text/vnd.graphviz")?,
            Text::VndHans => write!(f, "text/vnd.hans")?,
            Text::VndHgl => write!(f, "text/vnd.hgl")?,
            Text::VndIn3D3Dml => write!(f, "text/vnd.in3d.3dml")?,
            Text::VndIn3DSpot => write!(f, "text/vnd.in3d.spot")?,
            Text::VndIPTCNewsML => write!(f, "text/vnd.IPTC.NewsML")?,
            Text::VndIPTCNITF => write!(f, "text/vnd.IPTC.NITF")?,
            Text::VndLatexZ => write!(f, "text/vnd.latex-z")?,
            Text::VndMotorolaReflex => write!(f, "text/vnd.motorola.reflex")?,
            Text::VndMsMediapackage => write!(f, "text/vnd.ms-mediapackage")?,
            Text::VndNet2PhoneCommcenterCommand => {
                write!(f, "text/vnd.net2phone.commcenter.command")?
            }
            Text::VndRadisysMsmlBasicLayout => write!(f, "text/vnd.radisys.msml-basic-layout")?,
            Text::VndSenxWarpscript => write!(f, "text/vnd.senx.warpscript")?,
            Text::VndSunJ2MeAppDescriptor => write!(f, "text/vnd.sun.j2me.app-descriptor")?,
            Text::VndSosi => write!(f, "text/vnd.sosi")?,
            Text::VndTrolltechLinguist => write!(f, "text/vnd.trolltech.linguist")?,
            Text::VndWapSi => write!(f, "text/vnd.wap.si")?,
            Text::VndWapSl => write!(f, "text/vnd.wap.sl")?,
            Text::VndWapWml => write!(f, "text/vnd.wap.wml")?,
            Text::VndWapWmlscript => write!(f, "text/vnd.wap.wmlscript")?,
            Text::Vtt => write!(f, "text/vtt")?,
            Text::Xml => write!(f, "text/xml")?,
            Text::XmlExternalParsedEntity => write!(f, "text/xml-external-parsed-entity")?,
            Text::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Text {
    fn from(input: &str) -> Self {
        match input {
            "text/1d-interleaved-parityfec" => Text::_1DInterleavedParityfec,
            "text/cache-manifest" => Text::CacheManifest,
            "text/calendar" => Text::Calendar,
            "text/cql" => Text::Cql,
            "text/cql-expression" => Text::CqlExpression,
            "text/cql-identifier" => Text::CqlIdentifier,
            "text/css" => Text::Css,
            "text/csv" => Text::Csv,
            "text/csv-schema" => Text::CsvSchema,
            "text/directory" => Text::DirectoryDEPRECATEDByRFC6350,
            "text/dns" => Text::Dns,
            "text/encaprtp" => Text::Encaprtp,
            "text/example" => Text::Example,
            "text/fhirpath" => Text::Fhirpath,
            "text/flexfec" => Text::Flexfec,
            "text/fwdred" => Text::Fwdred,
            "text/gff3" => Text::Gff3,
            "text/grammar-ref-list" => Text::GrammarRefList,
            "text/hl7v2" => Text::Hl7V2,
            "text/html" => Text::Html,
            "text/javascript" => Text::Javascript,
            "text/jcr-cnd" => Text::JcrCnd,
            "text/markdown" => Text::Markdown,
            "text/mizar" => Text::Mizar,
            "text/n3" => Text::N3,
            "text/parameters" => Text::Parameters,
            "text/parityfec" => Text::Parityfec,
            "text/provenance-notation" => Text::ProvenanceNotation,
            "text/prs.fallenstein.rst" => Text::PrsFallensteinRst,
            "text/prs.lines.tag" => Text::PrsLinesTag,
            "text/prs.prop.logic" => Text::PrsPropLogic,
            "text/raptorfec" => Text::Raptorfec,
            "text/RED" => Text::Red,
            "text/rfc822-headers" => Text::Rfc822Headers,
            "text/rtf" => Text::Rtf,
            "text/rtp-enc-aescm128" => Text::RtpEncAescm128,
            "text/rtploopback" => Text::Rtploopback,
            "text/rtx" => Text::Rtx,
            "text/SGML" => Text::Sgml,
            "text/shaclc" => Text::Shaclc,
            "text/shex" => Text::Shex,
            "text/spdx" => Text::Spdx,
            "text/strings" => Text::Strings,
            "text/t140" => Text::T140,
            "text/tab-separated-values" => Text::TabSeparatedValues,
            "text/troff" => Text::Troff,
            "text/turtle" => Text::Turtle,
            "text/ulpfec" => Text::Ulpfec,
            "text/uri-list" => Text::UriList,
            "text/vcard" => Text::Vcard,
            "text/vnd.a" => Text::VndA,
            "text/vnd.abc" => Text::VndAbc,
            "text/vnd.ascii-art" => Text::VndAsciiArt,
            "text/vnd.curl" => Text::VndCurl,
            "text/vnd.debian.copyright" => Text::VndDebianCopyright,
            "text/vnd.DMClientScript" => Text::VndDMClientScript,
            "text/vnd.dvb.subtitle" => Text::VndDvbSubtitle,
            "text/vnd.esmertec.theme-descriptor" => Text::VndEsmertecThemeDescriptor,
            "text/vnd.exchangeable" => Text::VndExchangeable,
            "text/vnd.familysearch.gedcom" => Text::VndFamilysearchGedcom,
            "text/vnd.ficlab.flt" => Text::VndFiclabFlt,
            "text/vnd.fly" => Text::VndFly,
            "text/vnd.fmi.flexstor" => Text::VndFmiFlexstor,
            "text/vnd.gml" => Text::VndGml,
            "text/vnd.graphviz" => Text::VndGraphviz,
            "text/vnd.hans" => Text::VndHans,
            "text/vnd.hgl" => Text::VndHgl,
            "text/vnd.in3d.3dml" => Text::VndIn3D3Dml,
            "text/vnd.in3d.spot" => Text::VndIn3DSpot,
            "text/vnd.IPTC.NewsML" => Text::VndIPTCNewsML,
            "text/vnd.IPTC.NITF" => Text::VndIPTCNITF,
            "text/vnd.latex-z" => Text::VndLatexZ,
            "text/vnd.motorola.reflex" => Text::VndMotorolaReflex,
            "text/vnd.ms-mediapackage" => Text::VndMsMediapackage,
            "text/vnd.net2phone.commcenter.command" => Text::VndNet2PhoneCommcenterCommand,
            "text/vnd.radisys.msml-basic-layout" => Text::VndRadisysMsmlBasicLayout,
            "text/vnd.senx.warpscript" => Text::VndSenxWarpscript,
            "text/vnd.sun.j2me.app-descriptor" => Text::VndSunJ2MeAppDescriptor,
            "text/vnd.sosi" => Text::VndSosi,
            "text/vnd.trolltech.linguist" => Text::VndTrolltechLinguist,
            "text/vnd.wap.si" => Text::VndWapSi,
            "text/vnd.wap.sl" => Text::VndWapSl,
            "text/vnd.wap.wml" => Text::VndWapWml,
            "text/vnd.wap.wmlscript" => Text::VndWapWmlscript,
            "text/vtt" => Text::Vtt,
            "text/xml" => Text::Xml,
            "text/xml-external-parsed-entity" => Text::XmlExternalParsedEntity,
            _ => Text::Other(input.to_string()),
        }
    }
}
