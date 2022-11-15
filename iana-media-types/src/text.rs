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
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Text {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "text/1d-interleaved-parityfec" => Ok(Text::_1DInterleavedParityfec),
            "text/cache-manifest" => Ok(Text::CacheManifest),
            "text/calendar" => Ok(Text::Calendar),
            "text/cql" => Ok(Text::Cql),
            "text/cql-expression" => Ok(Text::CqlExpression),
            "text/cql-identifier" => Ok(Text::CqlIdentifier),
            "text/css" => Ok(Text::Css),
            "text/csv" => Ok(Text::Csv),
            "text/csv-schema" => Ok(Text::CsvSchema),
            "text/dns" => Ok(Text::Dns),
            "text/encaprtp" => Ok(Text::Encaprtp),
            "text/example" => Ok(Text::Example),
            "text/fhirpath" => Ok(Text::Fhirpath),
            "text/flexfec" => Ok(Text::Flexfec),
            "text/fwdred" => Ok(Text::Fwdred),
            "text/gff3" => Ok(Text::Gff3),
            "text/grammar-ref-list" => Ok(Text::GrammarRefList),
            "text/hl7v2" => Ok(Text::Hl7V2),
            "text/html" => Ok(Text::Html),
            "text/javascript" => Ok(Text::Javascript),
            "text/jcr-cnd" => Ok(Text::JcrCnd),
            "text/markdown" => Ok(Text::Markdown),
            "text/mizar" => Ok(Text::Mizar),
            "text/n3" => Ok(Text::N3),
            "text/parameters" => Ok(Text::Parameters),
            "text/parityfec" => Ok(Text::Parityfec),
            "text/provenance-notation" => Ok(Text::ProvenanceNotation),
            "text/prs.fallenstein.rst" => Ok(Text::PrsFallensteinRst),
            "text/prs.lines.tag" => Ok(Text::PrsLinesTag),
            "text/prs.prop.logic" => Ok(Text::PrsPropLogic),
            "text/raptorfec" => Ok(Text::Raptorfec),
            "text/RED" => Ok(Text::Red),
            "text/rfc822-headers" => Ok(Text::Rfc822Headers),
            "text/rtf" => Ok(Text::Rtf),
            "text/rtp-enc-aescm128" => Ok(Text::RtpEncAescm128),
            "text/rtploopback" => Ok(Text::Rtploopback),
            "text/rtx" => Ok(Text::Rtx),
            "text/SGML" => Ok(Text::Sgml),
            "text/shaclc" => Ok(Text::Shaclc),
            "text/shex" => Ok(Text::Shex),
            "text/spdx" => Ok(Text::Spdx),
            "text/strings" => Ok(Text::Strings),
            "text/t140" => Ok(Text::T140),
            "text/tab-separated-values" => Ok(Text::TabSeparatedValues),
            "text/troff" => Ok(Text::Troff),
            "text/turtle" => Ok(Text::Turtle),
            "text/ulpfec" => Ok(Text::Ulpfec),
            "text/uri-list" => Ok(Text::UriList),
            "text/vcard" => Ok(Text::Vcard),
            "text/vnd.a" => Ok(Text::VndA),
            "text/vnd.abc" => Ok(Text::VndAbc),
            "text/vnd.ascii-art" => Ok(Text::VndAsciiArt),
            "text/vnd.curl" => Ok(Text::VndCurl),
            "text/vnd.debian.copyright" => Ok(Text::VndDebianCopyright),
            "text/vnd.DMClientScript" => Ok(Text::VndDMClientScript),
            "text/vnd.dvb.subtitle" => Ok(Text::VndDvbSubtitle),
            "text/vnd.esmertec.theme-descriptor" => Ok(Text::VndEsmertecThemeDescriptor),
            "text/vnd.exchangeable" => Ok(Text::VndExchangeable),
            "text/vnd.familysearch.gedcom" => Ok(Text::VndFamilysearchGedcom),
            "text/vnd.ficlab.flt" => Ok(Text::VndFiclabFlt),
            "text/vnd.fly" => Ok(Text::VndFly),
            "text/vnd.fmi.flexstor" => Ok(Text::VndFmiFlexstor),
            "text/vnd.gml" => Ok(Text::VndGml),
            "text/vnd.graphviz" => Ok(Text::VndGraphviz),
            "text/vnd.hans" => Ok(Text::VndHans),
            "text/vnd.hgl" => Ok(Text::VndHgl),
            "text/vnd.in3d.3dml" => Ok(Text::VndIn3D3Dml),
            "text/vnd.in3d.spot" => Ok(Text::VndIn3DSpot),
            "text/vnd.IPTC.NewsML" => Ok(Text::VndIPTCNewsML),
            "text/vnd.IPTC.NITF" => Ok(Text::VndIPTCNITF),
            "text/vnd.latex-z" => Ok(Text::VndLatexZ),
            "text/vnd.motorola.reflex" => Ok(Text::VndMotorolaReflex),
            "text/vnd.ms-mediapackage" => Ok(Text::VndMsMediapackage),
            "text/vnd.net2phone.commcenter.command" => Ok(Text::VndNet2PhoneCommcenterCommand),
            "text/vnd.radisys.msml-basic-layout" => Ok(Text::VndRadisysMsmlBasicLayout),
            "text/vnd.senx.warpscript" => Ok(Text::VndSenxWarpscript),
            "text/vnd.sun.j2me.app-descriptor" => Ok(Text::VndSunJ2MeAppDescriptor),
            "text/vnd.sosi" => Ok(Text::VndSosi),
            "text/vnd.trolltech.linguist" => Ok(Text::VndTrolltechLinguist),
            "text/vnd.wap.si" => Ok(Text::VndWapSi),
            "text/vnd.wap.sl" => Ok(Text::VndWapSl),
            "text/vnd.wap.wml" => Ok(Text::VndWapWml),
            "text/vnd.wap.wmlscript" => Ok(Text::VndWapWmlscript),
            "text/vtt" => Ok(Text::Vtt),
            "text/xml" => Ok(Text::Xml),
            "text/xml-external-parsed-entity" => Ok(Text::XmlExternalParsedEntity),
            _ => Err(()),
        }
    }
}
