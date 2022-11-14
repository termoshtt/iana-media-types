#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Audio {
    #[doc = "audio/1d-interleaved-parityfec"]
    _1DInterleavedParityfec,
    #[doc = "audio/32kadpcm"]
    _32Kadpcm,
    #[doc = "audio/3gpp"]
    _3Gpp,
    #[doc = "audio/3gpp2"]
    _3Gpp2,
    #[doc = "audio/aac"]
    Aac,
    #[doc = "audio/ac3"]
    Ac3,
    #[doc = "audio/AMR"]
    Amr,
    #[doc = "audio/AMR-WB"]
    AmrWb,
    #[doc = "audio/amr-wb+"]
    AmrWb_,
    #[doc = "audio/aptx"]
    Aptx,
    #[doc = "audio/asc"]
    Asc,
    #[doc = "audio/ATRAC-ADVANCED-LOSSLESS"]
    AtracAdvancedLossless,
    #[doc = "audio/ATRAC-X"]
    AtracX,
    #[doc = "audio/ATRAC3"]
    Atrac3,
    #[doc = "audio/basic"]
    Basic,
    #[doc = "audio/BV16"]
    Bv16,
    #[doc = "audio/BV32"]
    Bv32,
    #[doc = "audio/clearmode"]
    Clearmode,
    #[doc = "audio/CN"]
    Cn,
    #[doc = "audio/DAT12"]
    Dat12,
    #[doc = "audio/dls"]
    Dls,
    #[doc = "audio/dsr-es201108"]
    DsrEs201108,
    #[doc = "audio/dsr-es202050"]
    DsrEs202050,
    #[doc = "audio/dsr-es202211"]
    DsrEs202211,
    #[doc = "audio/dsr-es202212"]
    DsrEs202212,
    #[doc = "audio/DV"]
    Dv,
    #[doc = "audio/DVI4"]
    Dvi4,
    #[doc = "audio/eac3"]
    Eac3,
    #[doc = "audio/encaprtp"]
    Encaprtp,
    #[doc = "audio/EVRC"]
    Evrc,
    #[doc = "audio/EVRC-QCP"]
    EvrcQcp,
    #[doc = "audio/EVRC0"]
    Evrc0,
    #[doc = "audio/EVRC1"]
    Evrc1,
    #[doc = "audio/EVRCB"]
    Evrcb,
    #[doc = "audio/EVRCB0"]
    Evrcb0,
    #[doc = "audio/EVRCB1"]
    Evrcb1,
    #[doc = "audio/EVRCNW"]
    Evrcnw,
    #[doc = "audio/EVRCNW0"]
    Evrcnw0,
    #[doc = "audio/EVRCNW1"]
    Evrcnw1,
    #[doc = "audio/EVRCWB"]
    Evrcwb,
    #[doc = "audio/EVRCWB0"]
    Evrcwb0,
    #[doc = "audio/EVRCWB1"]
    Evrcwb1,
    #[doc = "audio/EVS"]
    Evs,
    #[doc = "audio/example"]
    Example,
    #[doc = "audio/flexfec"]
    Flexfec,
    #[doc = "audio/fwdred"]
    Fwdred,
    #[doc = "audio/G711-0"]
    G7110,
    #[doc = "audio/G719"]
    G719,
    #[doc = "audio/G7221"]
    G7221,
    #[doc = "audio/G722"]
    G722,
    #[doc = "audio/G723"]
    G723,
    #[doc = "audio/G726-16"]
    G72616,
    #[doc = "audio/G726-24"]
    G72624,
    #[doc = "audio/G726-32"]
    G72632,
    #[doc = "audio/G726-40"]
    G72640,
    #[doc = "audio/G728"]
    G728,
    #[doc = "audio/G729"]
    G729,
    #[doc = "audio/G7291"]
    G7291,
    #[doc = "audio/G729D"]
    G729D,
    #[doc = "audio/G729E"]
    G729E,
    #[doc = "audio/GSM"]
    Gsm,
    #[doc = "audio/GSM-EFR"]
    GsmEfr,
    #[doc = "audio/GSM-HR-08"]
    GsmHr08,
    #[doc = "audio/iLBC"]
    Ilbc,
    #[doc = "audio/ip-mr_v2.5"]
    IpMrV25,
    #[doc = "audio/L8"]
    L8,
    #[doc = "audio/L16"]
    L16,
    #[doc = "audio/L20"]
    L20,
    #[doc = "audio/L24"]
    L24,
    #[doc = "audio/LPC"]
    Lpc,
    #[doc = "audio/MELP"]
    Melp,
    #[doc = "audio/MELP600"]
    Melp600,
    #[doc = "audio/MELP1200"]
    Melp1200,
    #[doc = "audio/MELP2400"]
    Melp2400,
    #[doc = "audio/mhas"]
    Mhas,
    #[doc = "audio/mobile-xmf"]
    MobileXmf,
    #[doc = "audio/MPA"]
    Mpa,
    #[doc = "audio/mp4"]
    Mp4,
    #[doc = "audio/MP4A-LATM"]
    Mp4ALatm,
    #[doc = "audio/mpa-robust"]
    MpaRobust,
    #[doc = "audio/mpeg"]
    Mpeg,
    #[doc = "audio/mpeg4-generic"]
    Mpeg4Generic,
    #[doc = "audio/ogg"]
    Ogg,
    #[doc = "audio/opus"]
    Opus,
    #[doc = "audio/parityfec"]
    Parityfec,
    #[doc = "audio/PCMA"]
    Pcma,
    #[doc = "audio/PCMA-WB"]
    PcmaWb,
    #[doc = "audio/PCMU"]
    Pcmu,
    #[doc = "audio/PCMU-WB"]
    PcmuWb,
    #[doc = "audio/prs.sid"]
    PrsSid,
    #[doc = "audio/QCELP"]
    Qcelp,
    #[doc = "audio/raptorfec"]
    Raptorfec,
    #[doc = "audio/RED"]
    Red,
    #[doc = "audio/rtp-enc-aescm128"]
    RtpEncAescm128,
    #[doc = "audio/rtploopback"]
    Rtploopback,
    #[doc = "audio/rtp-midi"]
    RtpMidi,
    #[doc = "audio/rtx"]
    Rtx,
    #[doc = "audio/scip"]
    Scip,
    #[doc = "audio/SMV"]
    Smv,
    #[doc = "audio/SMV0"]
    Smv0,
    #[doc = "audio/SMV-QCP"]
    SmvQcp,
    #[doc = "audio/sofa"]
    Sofa,
    #[doc = "audio/sp-midi"]
    SpMidi,
    #[doc = "audio/speex"]
    Speex,
    #[doc = "audio/t140c"]
    T140C,
    #[doc = "audio/t38"]
    T38,
    #[doc = "audio/telephone-event"]
    TelephoneEvent,
    #[doc = "audio/TETRA_ACELP"]
    TetraAcelp,
    #[doc = "audio/TETRA_ACELP_BB"]
    TetraAcelpBb,
    #[doc = "audio/tone"]
    Tone,
    #[doc = "audio/TSVCIS"]
    Tsvcis,
    #[doc = "audio/UEMCLIP"]
    Uemclip,
    #[doc = "audio/ulpfec"]
    Ulpfec,
    #[doc = "audio/usac"]
    Usac,
    #[doc = "audio/VDVI"]
    Vdvi,
    #[doc = "audio/VMR-WB"]
    VmrWb,
    #[doc = "audio/vnd.3gpp.iufp"]
    Vnd3GppIufp,
    #[doc = "audio/vnd.4SB"]
    Vnd4SB,
    #[doc = "audio/vnd.audiokoz"]
    VndAudiokoz,
    #[doc = "audio/vnd.CELP"]
    VndCELP,
    #[doc = "audio/vnd.cisco.nse"]
    VndCiscoNse,
    #[doc = "audio/vnd.cmles.radio-events"]
    VndCmlesRadioEvents,
    #[doc = "audio/vnd.cns.anp1"]
    VndCnsAnp1,
    #[doc = "audio/vnd.cns.inf1"]
    VndCnsInf1,
    #[doc = "audio/vnd.dece.audio"]
    VndDeceAudio,
    #[doc = "audio/vnd.digital-winds"]
    VndDigitalWinds,
    #[doc = "audio/vnd.dlna.adts"]
    VndDlnaAdts,
    #[doc = "audio/vnd.dolby.heaac.1"]
    VndDolbyHeaac1,
    #[doc = "audio/vnd.dolby.heaac.2"]
    VndDolbyHeaac2,
    #[doc = "audio/vnd.dolby.mlp"]
    VndDolbyMlp,
    #[doc = "audio/vnd.dolby.mps"]
    VndDolbyMps,
    #[doc = "audio/vnd.dolby.pl2"]
    VndDolbyPl2,
    #[doc = "audio/vnd.dolby.pl2x"]
    VndDolbyPl2X,
    #[doc = "audio/vnd.dolby.pl2z"]
    VndDolbyPl2Z,
    #[doc = "audio/vnd.dolby.pulse.1"]
    VndDolbyPulse1,
    #[doc = "audio/vnd.dra"]
    VndDra,
    #[doc = "audio/vnd.dts"]
    VndDts,
    #[doc = "audio/vnd.dts.hd"]
    VndDtsHd,
    #[doc = "audio/vnd.dts.uhd"]
    VndDtsUhd,
    #[doc = "audio/vnd.dvb.file"]
    VndDvbFile,
    #[doc = "audio/vnd.everad.plj"]
    VndEveradPlj,
    #[doc = "audio/vnd.hns.audio"]
    VndHnsAudio,
    #[doc = "audio/vnd.lucent.voice"]
    VndLucentVoice,
    #[doc = "audio/vnd.ms-playready.media.pya"]
    VndMsPlayreadyMediaPya,
    #[doc = "audio/vnd.nokia.mobile-xmf"]
    VndNokiaMobileXmf,
    #[doc = "audio/vnd.nortel.vbk"]
    VndNortelVbk,
    #[doc = "audio/vnd.nuera.ecelp4800"]
    VndNueraEcelp4800,
    #[doc = "audio/vnd.nuera.ecelp7470"]
    VndNueraEcelp7470,
    #[doc = "audio/vnd.nuera.ecelp9600"]
    VndNueraEcelp9600,
    #[doc = "audio/vnd.octel.sbc"]
    VndOctelSbc,
    #[doc = "audio/vnd.presonus.multitrack"]
    VndPresonusMultitrack,
    #[doc = "audio/vnd.qcelp"]
    VndQcelpDEPRECATEDInFavorOfAudioQcelp,
    #[doc = "audio/vnd.rhetorex.32kadpcm"]
    VndRhetorex32Kadpcm,
    #[doc = "audio/vnd.rip"]
    VndRip,
    #[doc = "audio/vnd.sealedmedia.softseal.mpeg"]
    VndSealedmediaSoftsealMpeg,
    #[doc = "audio/vnd.vmx.cvsd"]
    VndVmxCvsd,
    #[doc = "audio/vorbis"]
    Vorbis,
    #[doc = "audio/vorbis-config"]
    VorbisConfig,
    Other(String),
}
impl ::std::fmt::Display for Audio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Audio::_1DInterleavedParityfec => write!(f, "audio/1d-interleaved-parityfec")?,
            Audio::_32Kadpcm => write!(f, "audio/32kadpcm")?,
            Audio::_3Gpp => write!(f, "audio/3gpp")?,
            Audio::_3Gpp2 => write!(f, "audio/3gpp2")?,
            Audio::Aac => write!(f, "audio/aac")?,
            Audio::Ac3 => write!(f, "audio/ac3")?,
            Audio::Amr => write!(f, "audio/AMR")?,
            Audio::AmrWb => write!(f, "audio/AMR-WB")?,
            Audio::AmrWb_ => write!(f, "audio/amr-wb+")?,
            Audio::Aptx => write!(f, "audio/aptx")?,
            Audio::Asc => write!(f, "audio/asc")?,
            Audio::AtracAdvancedLossless => write!(f, "audio/ATRAC-ADVANCED-LOSSLESS")?,
            Audio::AtracX => write!(f, "audio/ATRAC-X")?,
            Audio::Atrac3 => write!(f, "audio/ATRAC3")?,
            Audio::Basic => write!(f, "audio/basic")?,
            Audio::Bv16 => write!(f, "audio/BV16")?,
            Audio::Bv32 => write!(f, "audio/BV32")?,
            Audio::Clearmode => write!(f, "audio/clearmode")?,
            Audio::Cn => write!(f, "audio/CN")?,
            Audio::Dat12 => write!(f, "audio/DAT12")?,
            Audio::Dls => write!(f, "audio/dls")?,
            Audio::DsrEs201108 => write!(f, "audio/dsr-es201108")?,
            Audio::DsrEs202050 => write!(f, "audio/dsr-es202050")?,
            Audio::DsrEs202211 => write!(f, "audio/dsr-es202211")?,
            Audio::DsrEs202212 => write!(f, "audio/dsr-es202212")?,
            Audio::Dv => write!(f, "audio/DV")?,
            Audio::Dvi4 => write!(f, "audio/DVI4")?,
            Audio::Eac3 => write!(f, "audio/eac3")?,
            Audio::Encaprtp => write!(f, "audio/encaprtp")?,
            Audio::Evrc => write!(f, "audio/EVRC")?,
            Audio::EvrcQcp => write!(f, "audio/EVRC-QCP")?,
            Audio::Evrc0 => write!(f, "audio/EVRC0")?,
            Audio::Evrc1 => write!(f, "audio/EVRC1")?,
            Audio::Evrcb => write!(f, "audio/EVRCB")?,
            Audio::Evrcb0 => write!(f, "audio/EVRCB0")?,
            Audio::Evrcb1 => write!(f, "audio/EVRCB1")?,
            Audio::Evrcnw => write!(f, "audio/EVRCNW")?,
            Audio::Evrcnw0 => write!(f, "audio/EVRCNW0")?,
            Audio::Evrcnw1 => write!(f, "audio/EVRCNW1")?,
            Audio::Evrcwb => write!(f, "audio/EVRCWB")?,
            Audio::Evrcwb0 => write!(f, "audio/EVRCWB0")?,
            Audio::Evrcwb1 => write!(f, "audio/EVRCWB1")?,
            Audio::Evs => write!(f, "audio/EVS")?,
            Audio::Example => write!(f, "audio/example")?,
            Audio::Flexfec => write!(f, "audio/flexfec")?,
            Audio::Fwdred => write!(f, "audio/fwdred")?,
            Audio::G7110 => write!(f, "audio/G711-0")?,
            Audio::G719 => write!(f, "audio/G719")?,
            Audio::G7221 => write!(f, "audio/G7221")?,
            Audio::G722 => write!(f, "audio/G722")?,
            Audio::G723 => write!(f, "audio/G723")?,
            Audio::G72616 => write!(f, "audio/G726-16")?,
            Audio::G72624 => write!(f, "audio/G726-24")?,
            Audio::G72632 => write!(f, "audio/G726-32")?,
            Audio::G72640 => write!(f, "audio/G726-40")?,
            Audio::G728 => write!(f, "audio/G728")?,
            Audio::G729 => write!(f, "audio/G729")?,
            Audio::G7291 => write!(f, "audio/G7291")?,
            Audio::G729D => write!(f, "audio/G729D")?,
            Audio::G729E => write!(f, "audio/G729E")?,
            Audio::Gsm => write!(f, "audio/GSM")?,
            Audio::GsmEfr => write!(f, "audio/GSM-EFR")?,
            Audio::GsmHr08 => write!(f, "audio/GSM-HR-08")?,
            Audio::Ilbc => write!(f, "audio/iLBC")?,
            Audio::IpMrV25 => write!(f, "audio/ip-mr_v2.5")?,
            Audio::L8 => write!(f, "audio/L8")?,
            Audio::L16 => write!(f, "audio/L16")?,
            Audio::L20 => write!(f, "audio/L20")?,
            Audio::L24 => write!(f, "audio/L24")?,
            Audio::Lpc => write!(f, "audio/LPC")?,
            Audio::Melp => write!(f, "audio/MELP")?,
            Audio::Melp600 => write!(f, "audio/MELP600")?,
            Audio::Melp1200 => write!(f, "audio/MELP1200")?,
            Audio::Melp2400 => write!(f, "audio/MELP2400")?,
            Audio::Mhas => write!(f, "audio/mhas")?,
            Audio::MobileXmf => write!(f, "audio/mobile-xmf")?,
            Audio::Mpa => write!(f, "audio/MPA")?,
            Audio::Mp4 => write!(f, "audio/mp4")?,
            Audio::Mp4ALatm => write!(f, "audio/MP4A-LATM")?,
            Audio::MpaRobust => write!(f, "audio/mpa-robust")?,
            Audio::Mpeg => write!(f, "audio/mpeg")?,
            Audio::Mpeg4Generic => write!(f, "audio/mpeg4-generic")?,
            Audio::Ogg => write!(f, "audio/ogg")?,
            Audio::Opus => write!(f, "audio/opus")?,
            Audio::Parityfec => write!(f, "audio/parityfec")?,
            Audio::Pcma => write!(f, "audio/PCMA")?,
            Audio::PcmaWb => write!(f, "audio/PCMA-WB")?,
            Audio::Pcmu => write!(f, "audio/PCMU")?,
            Audio::PcmuWb => write!(f, "audio/PCMU-WB")?,
            Audio::PrsSid => write!(f, "audio/prs.sid")?,
            Audio::Qcelp => write!(f, "audio/QCELP")?,
            Audio::Raptorfec => write!(f, "audio/raptorfec")?,
            Audio::Red => write!(f, "audio/RED")?,
            Audio::RtpEncAescm128 => write!(f, "audio/rtp-enc-aescm128")?,
            Audio::Rtploopback => write!(f, "audio/rtploopback")?,
            Audio::RtpMidi => write!(f, "audio/rtp-midi")?,
            Audio::Rtx => write!(f, "audio/rtx")?,
            Audio::Scip => write!(f, "audio/scip")?,
            Audio::Smv => write!(f, "audio/SMV")?,
            Audio::Smv0 => write!(f, "audio/SMV0")?,
            Audio::SmvQcp => write!(f, "audio/SMV-QCP")?,
            Audio::Sofa => write!(f, "audio/sofa")?,
            Audio::SpMidi => write!(f, "audio/sp-midi")?,
            Audio::Speex => write!(f, "audio/speex")?,
            Audio::T140C => write!(f, "audio/t140c")?,
            Audio::T38 => write!(f, "audio/t38")?,
            Audio::TelephoneEvent => write!(f, "audio/telephone-event")?,
            Audio::TetraAcelp => write!(f, "audio/TETRA_ACELP")?,
            Audio::TetraAcelpBb => write!(f, "audio/TETRA_ACELP_BB")?,
            Audio::Tone => write!(f, "audio/tone")?,
            Audio::Tsvcis => write!(f, "audio/TSVCIS")?,
            Audio::Uemclip => write!(f, "audio/UEMCLIP")?,
            Audio::Ulpfec => write!(f, "audio/ulpfec")?,
            Audio::Usac => write!(f, "audio/usac")?,
            Audio::Vdvi => write!(f, "audio/VDVI")?,
            Audio::VmrWb => write!(f, "audio/VMR-WB")?,
            Audio::Vnd3GppIufp => write!(f, "audio/vnd.3gpp.iufp")?,
            Audio::Vnd4SB => write!(f, "audio/vnd.4SB")?,
            Audio::VndAudiokoz => write!(f, "audio/vnd.audiokoz")?,
            Audio::VndCELP => write!(f, "audio/vnd.CELP")?,
            Audio::VndCiscoNse => write!(f, "audio/vnd.cisco.nse")?,
            Audio::VndCmlesRadioEvents => write!(f, "audio/vnd.cmles.radio-events")?,
            Audio::VndCnsAnp1 => write!(f, "audio/vnd.cns.anp1")?,
            Audio::VndCnsInf1 => write!(f, "audio/vnd.cns.inf1")?,
            Audio::VndDeceAudio => write!(f, "audio/vnd.dece.audio")?,
            Audio::VndDigitalWinds => write!(f, "audio/vnd.digital-winds")?,
            Audio::VndDlnaAdts => write!(f, "audio/vnd.dlna.adts")?,
            Audio::VndDolbyHeaac1 => write!(f, "audio/vnd.dolby.heaac.1")?,
            Audio::VndDolbyHeaac2 => write!(f, "audio/vnd.dolby.heaac.2")?,
            Audio::VndDolbyMlp => write!(f, "audio/vnd.dolby.mlp")?,
            Audio::VndDolbyMps => write!(f, "audio/vnd.dolby.mps")?,
            Audio::VndDolbyPl2 => write!(f, "audio/vnd.dolby.pl2")?,
            Audio::VndDolbyPl2X => write!(f, "audio/vnd.dolby.pl2x")?,
            Audio::VndDolbyPl2Z => write!(f, "audio/vnd.dolby.pl2z")?,
            Audio::VndDolbyPulse1 => write!(f, "audio/vnd.dolby.pulse.1")?,
            Audio::VndDra => write!(f, "audio/vnd.dra")?,
            Audio::VndDts => write!(f, "audio/vnd.dts")?,
            Audio::VndDtsHd => write!(f, "audio/vnd.dts.hd")?,
            Audio::VndDtsUhd => write!(f, "audio/vnd.dts.uhd")?,
            Audio::VndDvbFile => write!(f, "audio/vnd.dvb.file")?,
            Audio::VndEveradPlj => write!(f, "audio/vnd.everad.plj")?,
            Audio::VndHnsAudio => write!(f, "audio/vnd.hns.audio")?,
            Audio::VndLucentVoice => write!(f, "audio/vnd.lucent.voice")?,
            Audio::VndMsPlayreadyMediaPya => write!(f, "audio/vnd.ms-playready.media.pya")?,
            Audio::VndNokiaMobileXmf => write!(f, "audio/vnd.nokia.mobile-xmf")?,
            Audio::VndNortelVbk => write!(f, "audio/vnd.nortel.vbk")?,
            Audio::VndNueraEcelp4800 => write!(f, "audio/vnd.nuera.ecelp4800")?,
            Audio::VndNueraEcelp7470 => write!(f, "audio/vnd.nuera.ecelp7470")?,
            Audio::VndNueraEcelp9600 => write!(f, "audio/vnd.nuera.ecelp9600")?,
            Audio::VndOctelSbc => write!(f, "audio/vnd.octel.sbc")?,
            Audio::VndPresonusMultitrack => write!(f, "audio/vnd.presonus.multitrack")?,
            Audio::VndQcelpDEPRECATEDInFavorOfAudioQcelp => write!(f, "audio/vnd.qcelp")?,
            Audio::VndRhetorex32Kadpcm => write!(f, "audio/vnd.rhetorex.32kadpcm")?,
            Audio::VndRip => write!(f, "audio/vnd.rip")?,
            Audio::VndSealedmediaSoftsealMpeg => write!(f, "audio/vnd.sealedmedia.softseal.mpeg")?,
            Audio::VndVmxCvsd => write!(f, "audio/vnd.vmx.cvsd")?,
            Audio::Vorbis => write!(f, "audio/vorbis")?,
            Audio::VorbisConfig => write!(f, "audio/vorbis-config")?,
            Audio::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Audio {
    fn from(input: &str) -> Self {
        match input {
            "audio/1d-interleaved-parityfec" => Audio::_1DInterleavedParityfec,
            "audio/32kadpcm" => Audio::_32Kadpcm,
            "audio/3gpp" => Audio::_3Gpp,
            "audio/3gpp2" => Audio::_3Gpp2,
            "audio/aac" => Audio::Aac,
            "audio/ac3" => Audio::Ac3,
            "audio/AMR" => Audio::Amr,
            "audio/AMR-WB" => Audio::AmrWb,
            "audio/amr-wb+" => Audio::AmrWb_,
            "audio/aptx" => Audio::Aptx,
            "audio/asc" => Audio::Asc,
            "audio/ATRAC-ADVANCED-LOSSLESS" => Audio::AtracAdvancedLossless,
            "audio/ATRAC-X" => Audio::AtracX,
            "audio/ATRAC3" => Audio::Atrac3,
            "audio/basic" => Audio::Basic,
            "audio/BV16" => Audio::Bv16,
            "audio/BV32" => Audio::Bv32,
            "audio/clearmode" => Audio::Clearmode,
            "audio/CN" => Audio::Cn,
            "audio/DAT12" => Audio::Dat12,
            "audio/dls" => Audio::Dls,
            "audio/dsr-es201108" => Audio::DsrEs201108,
            "audio/dsr-es202050" => Audio::DsrEs202050,
            "audio/dsr-es202211" => Audio::DsrEs202211,
            "audio/dsr-es202212" => Audio::DsrEs202212,
            "audio/DV" => Audio::Dv,
            "audio/DVI4" => Audio::Dvi4,
            "audio/eac3" => Audio::Eac3,
            "audio/encaprtp" => Audio::Encaprtp,
            "audio/EVRC" => Audio::Evrc,
            "audio/EVRC-QCP" => Audio::EvrcQcp,
            "audio/EVRC0" => Audio::Evrc0,
            "audio/EVRC1" => Audio::Evrc1,
            "audio/EVRCB" => Audio::Evrcb,
            "audio/EVRCB0" => Audio::Evrcb0,
            "audio/EVRCB1" => Audio::Evrcb1,
            "audio/EVRCNW" => Audio::Evrcnw,
            "audio/EVRCNW0" => Audio::Evrcnw0,
            "audio/EVRCNW1" => Audio::Evrcnw1,
            "audio/EVRCWB" => Audio::Evrcwb,
            "audio/EVRCWB0" => Audio::Evrcwb0,
            "audio/EVRCWB1" => Audio::Evrcwb1,
            "audio/EVS" => Audio::Evs,
            "audio/example" => Audio::Example,
            "audio/flexfec" => Audio::Flexfec,
            "audio/fwdred" => Audio::Fwdred,
            "audio/G711-0" => Audio::G7110,
            "audio/G719" => Audio::G719,
            "audio/G7221" => Audio::G7221,
            "audio/G722" => Audio::G722,
            "audio/G723" => Audio::G723,
            "audio/G726-16" => Audio::G72616,
            "audio/G726-24" => Audio::G72624,
            "audio/G726-32" => Audio::G72632,
            "audio/G726-40" => Audio::G72640,
            "audio/G728" => Audio::G728,
            "audio/G729" => Audio::G729,
            "audio/G7291" => Audio::G7291,
            "audio/G729D" => Audio::G729D,
            "audio/G729E" => Audio::G729E,
            "audio/GSM" => Audio::Gsm,
            "audio/GSM-EFR" => Audio::GsmEfr,
            "audio/GSM-HR-08" => Audio::GsmHr08,
            "audio/iLBC" => Audio::Ilbc,
            "audio/ip-mr_v2.5" => Audio::IpMrV25,
            "audio/L8" => Audio::L8,
            "audio/L16" => Audio::L16,
            "audio/L20" => Audio::L20,
            "audio/L24" => Audio::L24,
            "audio/LPC" => Audio::Lpc,
            "audio/MELP" => Audio::Melp,
            "audio/MELP600" => Audio::Melp600,
            "audio/MELP1200" => Audio::Melp1200,
            "audio/MELP2400" => Audio::Melp2400,
            "audio/mhas" => Audio::Mhas,
            "audio/mobile-xmf" => Audio::MobileXmf,
            "audio/MPA" => Audio::Mpa,
            "audio/mp4" => Audio::Mp4,
            "audio/MP4A-LATM" => Audio::Mp4ALatm,
            "audio/mpa-robust" => Audio::MpaRobust,
            "audio/mpeg" => Audio::Mpeg,
            "audio/mpeg4-generic" => Audio::Mpeg4Generic,
            "audio/ogg" => Audio::Ogg,
            "audio/opus" => Audio::Opus,
            "audio/parityfec" => Audio::Parityfec,
            "audio/PCMA" => Audio::Pcma,
            "audio/PCMA-WB" => Audio::PcmaWb,
            "audio/PCMU" => Audio::Pcmu,
            "audio/PCMU-WB" => Audio::PcmuWb,
            "audio/prs.sid" => Audio::PrsSid,
            "audio/QCELP" => Audio::Qcelp,
            "audio/raptorfec" => Audio::Raptorfec,
            "audio/RED" => Audio::Red,
            "audio/rtp-enc-aescm128" => Audio::RtpEncAescm128,
            "audio/rtploopback" => Audio::Rtploopback,
            "audio/rtp-midi" => Audio::RtpMidi,
            "audio/rtx" => Audio::Rtx,
            "audio/scip" => Audio::Scip,
            "audio/SMV" => Audio::Smv,
            "audio/SMV0" => Audio::Smv0,
            "audio/SMV-QCP" => Audio::SmvQcp,
            "audio/sofa" => Audio::Sofa,
            "audio/sp-midi" => Audio::SpMidi,
            "audio/speex" => Audio::Speex,
            "audio/t140c" => Audio::T140C,
            "audio/t38" => Audio::T38,
            "audio/telephone-event" => Audio::TelephoneEvent,
            "audio/TETRA_ACELP" => Audio::TetraAcelp,
            "audio/TETRA_ACELP_BB" => Audio::TetraAcelpBb,
            "audio/tone" => Audio::Tone,
            "audio/TSVCIS" => Audio::Tsvcis,
            "audio/UEMCLIP" => Audio::Uemclip,
            "audio/ulpfec" => Audio::Ulpfec,
            "audio/usac" => Audio::Usac,
            "audio/VDVI" => Audio::Vdvi,
            "audio/VMR-WB" => Audio::VmrWb,
            "audio/vnd.3gpp.iufp" => Audio::Vnd3GppIufp,
            "audio/vnd.4SB" => Audio::Vnd4SB,
            "audio/vnd.audiokoz" => Audio::VndAudiokoz,
            "audio/vnd.CELP" => Audio::VndCELP,
            "audio/vnd.cisco.nse" => Audio::VndCiscoNse,
            "audio/vnd.cmles.radio-events" => Audio::VndCmlesRadioEvents,
            "audio/vnd.cns.anp1" => Audio::VndCnsAnp1,
            "audio/vnd.cns.inf1" => Audio::VndCnsInf1,
            "audio/vnd.dece.audio" => Audio::VndDeceAudio,
            "audio/vnd.digital-winds" => Audio::VndDigitalWinds,
            "audio/vnd.dlna.adts" => Audio::VndDlnaAdts,
            "audio/vnd.dolby.heaac.1" => Audio::VndDolbyHeaac1,
            "audio/vnd.dolby.heaac.2" => Audio::VndDolbyHeaac2,
            "audio/vnd.dolby.mlp" => Audio::VndDolbyMlp,
            "audio/vnd.dolby.mps" => Audio::VndDolbyMps,
            "audio/vnd.dolby.pl2" => Audio::VndDolbyPl2,
            "audio/vnd.dolby.pl2x" => Audio::VndDolbyPl2X,
            "audio/vnd.dolby.pl2z" => Audio::VndDolbyPl2Z,
            "audio/vnd.dolby.pulse.1" => Audio::VndDolbyPulse1,
            "audio/vnd.dra" => Audio::VndDra,
            "audio/vnd.dts" => Audio::VndDts,
            "audio/vnd.dts.hd" => Audio::VndDtsHd,
            "audio/vnd.dts.uhd" => Audio::VndDtsUhd,
            "audio/vnd.dvb.file" => Audio::VndDvbFile,
            "audio/vnd.everad.plj" => Audio::VndEveradPlj,
            "audio/vnd.hns.audio" => Audio::VndHnsAudio,
            "audio/vnd.lucent.voice" => Audio::VndLucentVoice,
            "audio/vnd.ms-playready.media.pya" => Audio::VndMsPlayreadyMediaPya,
            "audio/vnd.nokia.mobile-xmf" => Audio::VndNokiaMobileXmf,
            "audio/vnd.nortel.vbk" => Audio::VndNortelVbk,
            "audio/vnd.nuera.ecelp4800" => Audio::VndNueraEcelp4800,
            "audio/vnd.nuera.ecelp7470" => Audio::VndNueraEcelp7470,
            "audio/vnd.nuera.ecelp9600" => Audio::VndNueraEcelp9600,
            "audio/vnd.octel.sbc" => Audio::VndOctelSbc,
            "audio/vnd.presonus.multitrack" => Audio::VndPresonusMultitrack,
            "audio/vnd.qcelp" => Audio::VndQcelpDEPRECATEDInFavorOfAudioQcelp,
            "audio/vnd.rhetorex.32kadpcm" => Audio::VndRhetorex32Kadpcm,
            "audio/vnd.rip" => Audio::VndRip,
            "audio/vnd.sealedmedia.softseal.mpeg" => Audio::VndSealedmediaSoftsealMpeg,
            "audio/vnd.vmx.cvsd" => Audio::VndVmxCvsd,
            "audio/vorbis" => Audio::Vorbis,
            "audio/vorbis-config" => Audio::VorbisConfig,
            _ => Audio::Other(input.to_string()),
        }
    }
}
