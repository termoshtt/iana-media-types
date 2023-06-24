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
pub enum Audio {
    #[doc = "audio/1d-interleaved-parityfec"]
    #[serde(alias = "audio/1d-interleaved-parityfec")]
    _1DInterleavedParityfec,
    #[doc = "audio/32kadpcm"]
    #[serde(alias = "audio/32kadpcm")]
    _32Kadpcm,
    #[doc = "audio/3gpp"]
    #[serde(alias = "audio/3gpp")]
    _3Gpp,
    #[doc = "audio/3gpp2"]
    #[serde(alias = "audio/3gpp2")]
    _3Gpp2,
    #[doc = "audio/aac"]
    #[serde(alias = "audio/aac")]
    Aac,
    #[doc = "audio/ac3"]
    #[serde(alias = "audio/ac3")]
    Ac3,
    #[doc = "audio/AMR"]
    #[serde(alias = "audio/AMR")]
    Amr,
    #[doc = "audio/AMR-WB"]
    #[serde(alias = "audio/AMR-WB")]
    AmrWb,
    #[doc = "audio/amr-wb+"]
    #[serde(alias = "audio/amr-wb+")]
    AmrWb_,
    #[doc = "audio/aptx"]
    #[serde(alias = "audio/aptx")]
    Aptx,
    #[doc = "audio/asc"]
    #[serde(alias = "audio/asc")]
    Asc,
    #[doc = "audio/ATRAC-ADVANCED-LOSSLESS"]
    #[serde(alias = "audio/ATRAC-ADVANCED-LOSSLESS")]
    AtracAdvancedLossless,
    #[doc = "audio/ATRAC-X"]
    #[serde(alias = "audio/ATRAC-X")]
    AtracX,
    #[doc = "audio/ATRAC3"]
    #[serde(alias = "audio/ATRAC3")]
    Atrac3,
    #[doc = "audio/basic"]
    #[serde(alias = "audio/basic")]
    Basic,
    #[doc = "audio/BV16"]
    #[serde(alias = "audio/BV16")]
    Bv16,
    #[doc = "audio/BV32"]
    #[serde(alias = "audio/BV32")]
    Bv32,
    #[doc = "audio/clearmode"]
    #[serde(alias = "audio/clearmode")]
    Clearmode,
    #[doc = "audio/CN"]
    #[serde(alias = "audio/CN")]
    Cn,
    #[doc = "audio/DAT12"]
    #[serde(alias = "audio/DAT12")]
    Dat12,
    #[doc = "audio/dls"]
    #[serde(alias = "audio/dls")]
    Dls,
    #[doc = "audio/dsr-es201108"]
    #[serde(alias = "audio/dsr-es201108")]
    DsrEs201108,
    #[doc = "audio/dsr-es202050"]
    #[serde(alias = "audio/dsr-es202050")]
    DsrEs202050,
    #[doc = "audio/dsr-es202211"]
    #[serde(alias = "audio/dsr-es202211")]
    DsrEs202211,
    #[doc = "audio/dsr-es202212"]
    #[serde(alias = "audio/dsr-es202212")]
    DsrEs202212,
    #[doc = "audio/DV"]
    #[serde(alias = "audio/DV")]
    Dv,
    #[doc = "audio/DVI4"]
    #[serde(alias = "audio/DVI4")]
    Dvi4,
    #[doc = "audio/eac3"]
    #[serde(alias = "audio/eac3")]
    Eac3,
    #[doc = "audio/encaprtp"]
    #[serde(alias = "audio/encaprtp")]
    Encaprtp,
    #[doc = "audio/EVRC"]
    #[serde(alias = "audio/EVRC")]
    Evrc,
    #[doc = "audio/EVRC-QCP"]
    #[serde(alias = "audio/EVRC-QCP")]
    EvrcQcp,
    #[doc = "audio/EVRC0"]
    #[serde(alias = "audio/EVRC0")]
    Evrc0,
    #[doc = "audio/EVRC1"]
    #[serde(alias = "audio/EVRC1")]
    Evrc1,
    #[doc = "audio/EVRCB"]
    #[serde(alias = "audio/EVRCB")]
    Evrcb,
    #[doc = "audio/EVRCB0"]
    #[serde(alias = "audio/EVRCB0")]
    Evrcb0,
    #[doc = "audio/EVRCB1"]
    #[serde(alias = "audio/EVRCB1")]
    Evrcb1,
    #[doc = "audio/EVRCNW"]
    #[serde(alias = "audio/EVRCNW")]
    Evrcnw,
    #[doc = "audio/EVRCNW0"]
    #[serde(alias = "audio/EVRCNW0")]
    Evrcnw0,
    #[doc = "audio/EVRCNW1"]
    #[serde(alias = "audio/EVRCNW1")]
    Evrcnw1,
    #[doc = "audio/EVRCWB"]
    #[serde(alias = "audio/EVRCWB")]
    Evrcwb,
    #[doc = "audio/EVRCWB0"]
    #[serde(alias = "audio/EVRCWB0")]
    Evrcwb0,
    #[doc = "audio/EVRCWB1"]
    #[serde(alias = "audio/EVRCWB1")]
    Evrcwb1,
    #[doc = "audio/EVS"]
    #[serde(alias = "audio/EVS")]
    Evs,
    #[doc = "audio/example"]
    #[serde(alias = "audio/example")]
    Example,
    #[doc = "audio/flexfec"]
    #[serde(alias = "audio/flexfec")]
    Flexfec,
    #[doc = "audio/fwdred"]
    #[serde(alias = "audio/fwdred")]
    Fwdred,
    #[doc = "audio/G711-0"]
    #[serde(alias = "audio/G711-0")]
    G7110,
    #[doc = "audio/G719"]
    #[serde(alias = "audio/G719")]
    G719,
    #[doc = "audio/G7221"]
    #[serde(alias = "audio/G7221")]
    G7221,
    #[doc = "audio/G722"]
    #[serde(alias = "audio/G722")]
    G722,
    #[doc = "audio/G723"]
    #[serde(alias = "audio/G723")]
    G723,
    #[doc = "audio/G726-16"]
    #[serde(alias = "audio/G726-16")]
    G72616,
    #[doc = "audio/G726-24"]
    #[serde(alias = "audio/G726-24")]
    G72624,
    #[doc = "audio/G726-32"]
    #[serde(alias = "audio/G726-32")]
    G72632,
    #[doc = "audio/G726-40"]
    #[serde(alias = "audio/G726-40")]
    G72640,
    #[doc = "audio/G728"]
    #[serde(alias = "audio/G728")]
    G728,
    #[doc = "audio/G729"]
    #[serde(alias = "audio/G729")]
    G729,
    #[doc = "audio/G7291"]
    #[serde(alias = "audio/G7291")]
    G7291,
    #[doc = "audio/G729D"]
    #[serde(alias = "audio/G729D")]
    G729D,
    #[doc = "audio/G729E"]
    #[serde(alias = "audio/G729E")]
    G729E,
    #[doc = "audio/GSM"]
    #[serde(alias = "audio/GSM")]
    Gsm,
    #[doc = "audio/GSM-EFR"]
    #[serde(alias = "audio/GSM-EFR")]
    GsmEfr,
    #[doc = "audio/GSM-HR-08"]
    #[serde(alias = "audio/GSM-HR-08")]
    GsmHr08,
    #[doc = "audio/iLBC"]
    #[serde(alias = "audio/iLBC")]
    Ilbc,
    #[doc = "audio/ip-mr_v2.5"]
    #[serde(alias = "audio/ip-mr_v2.5")]
    IpMrV25,
    #[doc = "audio/L8"]
    #[serde(alias = "audio/L8")]
    L8,
    #[doc = "audio/L16"]
    #[serde(alias = "audio/L16")]
    L16,
    #[doc = "audio/L20"]
    #[serde(alias = "audio/L20")]
    L20,
    #[doc = "audio/L24"]
    #[serde(alias = "audio/L24")]
    L24,
    #[doc = "audio/LPC"]
    #[serde(alias = "audio/LPC")]
    Lpc,
    #[doc = "audio/MELP"]
    #[serde(alias = "audio/MELP")]
    Melp,
    #[doc = "audio/MELP600"]
    #[serde(alias = "audio/MELP600")]
    Melp600,
    #[doc = "audio/MELP1200"]
    #[serde(alias = "audio/MELP1200")]
    Melp1200,
    #[doc = "audio/MELP2400"]
    #[serde(alias = "audio/MELP2400")]
    Melp2400,
    #[doc = "audio/mhas"]
    #[serde(alias = "audio/mhas")]
    Mhas,
    #[doc = "audio/mobile-xmf"]
    #[serde(alias = "audio/mobile-xmf")]
    MobileXmf,
    #[doc = "audio/MPA"]
    #[serde(alias = "audio/MPA")]
    Mpa,
    #[doc = "audio/mp4"]
    #[serde(alias = "audio/mp4")]
    Mp4,
    #[doc = "audio/MP4A-LATM"]
    #[serde(alias = "audio/MP4A-LATM")]
    Mp4ALatm,
    #[doc = "audio/mpa-robust"]
    #[serde(alias = "audio/mpa-robust")]
    MpaRobust,
    #[doc = "audio/mpeg"]
    #[serde(alias = "audio/mpeg")]
    Mpeg,
    #[doc = "audio/mpeg4-generic"]
    #[serde(alias = "audio/mpeg4-generic")]
    Mpeg4Generic,
    #[doc = "audio/ogg"]
    #[serde(alias = "audio/ogg")]
    Ogg,
    #[doc = "audio/opus"]
    #[serde(alias = "audio/opus")]
    Opus,
    #[doc = "audio/parityfec"]
    #[serde(alias = "audio/parityfec")]
    Parityfec,
    #[doc = "audio/PCMA"]
    #[serde(alias = "audio/PCMA")]
    Pcma,
    #[doc = "audio/PCMA-WB"]
    #[serde(alias = "audio/PCMA-WB")]
    PcmaWb,
    #[doc = "audio/PCMU"]
    #[serde(alias = "audio/PCMU")]
    Pcmu,
    #[doc = "audio/PCMU-WB"]
    #[serde(alias = "audio/PCMU-WB")]
    PcmuWb,
    #[doc = "audio/prs.sid"]
    #[serde(alias = "audio/prs.sid")]
    PrsSid,
    #[doc = "audio/QCELP"]
    #[serde(alias = "audio/QCELP")]
    Qcelp,
    #[doc = "audio/raptorfec"]
    #[serde(alias = "audio/raptorfec")]
    Raptorfec,
    #[doc = "audio/RED"]
    #[serde(alias = "audio/RED")]
    Red,
    #[doc = "audio/rtp-enc-aescm128"]
    #[serde(alias = "audio/rtp-enc-aescm128")]
    RtpEncAescm128,
    #[doc = "audio/rtploopback"]
    #[serde(alias = "audio/rtploopback")]
    Rtploopback,
    #[doc = "audio/rtp-midi"]
    #[serde(alias = "audio/rtp-midi")]
    RtpMidi,
    #[doc = "audio/rtx"]
    #[serde(alias = "audio/rtx")]
    Rtx,
    #[doc = "audio/scip"]
    #[serde(alias = "audio/scip")]
    Scip,
    #[doc = "audio/SMV"]
    #[serde(alias = "audio/SMV")]
    Smv,
    #[doc = "audio/SMV0"]
    #[serde(alias = "audio/SMV0")]
    Smv0,
    #[doc = "audio/SMV-QCP"]
    #[serde(alias = "audio/SMV-QCP")]
    SmvQcp,
    #[doc = "audio/sofa"]
    #[serde(alias = "audio/sofa")]
    Sofa,
    #[doc = "audio/sp-midi"]
    #[serde(alias = "audio/sp-midi")]
    SpMidi,
    #[doc = "audio/speex"]
    #[serde(alias = "audio/speex")]
    Speex,
    #[doc = "audio/t140c"]
    #[serde(alias = "audio/t140c")]
    T140C,
    #[doc = "audio/t38"]
    #[serde(alias = "audio/t38")]
    T38,
    #[doc = "audio/telephone-event"]
    #[serde(alias = "audio/telephone-event")]
    TelephoneEvent,
    #[doc = "audio/TETRA_ACELP"]
    #[serde(alias = "audio/TETRA_ACELP")]
    TetraAcelp,
    #[doc = "audio/TETRA_ACELP_BB"]
    #[serde(alias = "audio/TETRA_ACELP_BB")]
    TetraAcelpBb,
    #[doc = "audio/tone"]
    #[serde(alias = "audio/tone")]
    Tone,
    #[doc = "audio/TSVCIS"]
    #[serde(alias = "audio/TSVCIS")]
    Tsvcis,
    #[doc = "audio/UEMCLIP"]
    #[serde(alias = "audio/UEMCLIP")]
    Uemclip,
    #[doc = "audio/ulpfec"]
    #[serde(alias = "audio/ulpfec")]
    Ulpfec,
    #[doc = "audio/usac"]
    #[serde(alias = "audio/usac")]
    Usac,
    #[doc = "audio/VDVI"]
    #[serde(alias = "audio/VDVI")]
    Vdvi,
    #[doc = "audio/VMR-WB"]
    #[serde(alias = "audio/VMR-WB")]
    VmrWb,
    #[doc = "audio/vnd.3gpp.iufp"]
    #[serde(alias = "audio/vnd.3gpp.iufp")]
    Vnd3GppIufp,
    #[doc = "audio/vnd.4SB"]
    #[serde(alias = "audio/vnd.4SB")]
    Vnd4SB,
    #[doc = "audio/vnd.audiokoz"]
    #[serde(alias = "audio/vnd.audiokoz")]
    VndAudiokoz,
    #[doc = "audio/vnd.CELP"]
    #[serde(alias = "audio/vnd.CELP")]
    VndCELP,
    #[doc = "audio/vnd.cisco.nse"]
    #[serde(alias = "audio/vnd.cisco.nse")]
    VndCiscoNse,
    #[doc = "audio/vnd.cmles.radio-events"]
    #[serde(alias = "audio/vnd.cmles.radio-events")]
    VndCmlesRadioEvents,
    #[doc = "audio/vnd.cns.anp1"]
    #[serde(alias = "audio/vnd.cns.anp1")]
    VndCnsAnp1,
    #[doc = "audio/vnd.cns.inf1"]
    #[serde(alias = "audio/vnd.cns.inf1")]
    VndCnsInf1,
    #[doc = "audio/vnd.dece.audio"]
    #[serde(alias = "audio/vnd.dece.audio")]
    VndDeceAudio,
    #[doc = "audio/vnd.digital-winds"]
    #[serde(alias = "audio/vnd.digital-winds")]
    VndDigitalWinds,
    #[doc = "audio/vnd.dlna.adts"]
    #[serde(alias = "audio/vnd.dlna.adts")]
    VndDlnaAdts,
    #[doc = "audio/vnd.dolby.heaac.1"]
    #[serde(alias = "audio/vnd.dolby.heaac.1")]
    VndDolbyHeaac1,
    #[doc = "audio/vnd.dolby.heaac.2"]
    #[serde(alias = "audio/vnd.dolby.heaac.2")]
    VndDolbyHeaac2,
    #[doc = "audio/vnd.dolby.mlp"]
    #[serde(alias = "audio/vnd.dolby.mlp")]
    VndDolbyMlp,
    #[doc = "audio/vnd.dolby.mps"]
    #[serde(alias = "audio/vnd.dolby.mps")]
    VndDolbyMps,
    #[doc = "audio/vnd.dolby.pl2"]
    #[serde(alias = "audio/vnd.dolby.pl2")]
    VndDolbyPl2,
    #[doc = "audio/vnd.dolby.pl2x"]
    #[serde(alias = "audio/vnd.dolby.pl2x")]
    VndDolbyPl2X,
    #[doc = "audio/vnd.dolby.pl2z"]
    #[serde(alias = "audio/vnd.dolby.pl2z")]
    VndDolbyPl2Z,
    #[doc = "audio/vnd.dolby.pulse.1"]
    #[serde(alias = "audio/vnd.dolby.pulse.1")]
    VndDolbyPulse1,
    #[doc = "audio/vnd.dra"]
    #[serde(alias = "audio/vnd.dra")]
    VndDra,
    #[doc = "audio/vnd.dts"]
    #[serde(alias = "audio/vnd.dts")]
    VndDts,
    #[doc = "audio/vnd.dts.hd"]
    #[serde(alias = "audio/vnd.dts.hd")]
    VndDtsHd,
    #[doc = "audio/vnd.dts.uhd"]
    #[serde(alias = "audio/vnd.dts.uhd")]
    VndDtsUhd,
    #[doc = "audio/vnd.dvb.file"]
    #[serde(alias = "audio/vnd.dvb.file")]
    VndDvbFile,
    #[doc = "audio/vnd.everad.plj"]
    #[serde(alias = "audio/vnd.everad.plj")]
    VndEveradPlj,
    #[doc = "audio/vnd.hns.audio"]
    #[serde(alias = "audio/vnd.hns.audio")]
    VndHnsAudio,
    #[doc = "audio/vnd.lucent.voice"]
    #[serde(alias = "audio/vnd.lucent.voice")]
    VndLucentVoice,
    #[doc = "audio/vnd.ms-playready.media.pya"]
    #[serde(alias = "audio/vnd.ms-playready.media.pya")]
    VndMsPlayreadyMediaPya,
    #[doc = "audio/vnd.nokia.mobile-xmf"]
    #[serde(alias = "audio/vnd.nokia.mobile-xmf")]
    VndNokiaMobileXmf,
    #[doc = "audio/vnd.nortel.vbk"]
    #[serde(alias = "audio/vnd.nortel.vbk")]
    VndNortelVbk,
    #[doc = "audio/vnd.nuera.ecelp4800"]
    #[serde(alias = "audio/vnd.nuera.ecelp4800")]
    VndNueraEcelp4800,
    #[doc = "audio/vnd.nuera.ecelp7470"]
    #[serde(alias = "audio/vnd.nuera.ecelp7470")]
    VndNueraEcelp7470,
    #[doc = "audio/vnd.nuera.ecelp9600"]
    #[serde(alias = "audio/vnd.nuera.ecelp9600")]
    VndNueraEcelp9600,
    #[doc = "audio/vnd.octel.sbc"]
    #[serde(alias = "audio/vnd.octel.sbc")]
    VndOctelSbc,
    #[doc = "audio/vnd.presonus.multitrack"]
    #[serde(alias = "audio/vnd.presonus.multitrack")]
    VndPresonusMultitrack,
    #[doc = "audio/vnd.rhetorex.32kadpcm"]
    #[serde(alias = "audio/vnd.rhetorex.32kadpcm")]
    VndRhetorex32Kadpcm,
    #[doc = "audio/vnd.rip"]
    #[serde(alias = "audio/vnd.rip")]
    VndRip,
    #[doc = "audio/vnd.sealedmedia.softseal.mpeg"]
    #[serde(alias = "audio/vnd.sealedmedia.softseal.mpeg")]
    VndSealedmediaSoftsealMpeg,
    #[doc = "audio/vnd.vmx.cvsd"]
    #[serde(alias = "audio/vnd.vmx.cvsd")]
    VndVmxCvsd,
    #[doc = "audio/vorbis"]
    #[serde(alias = "audio/vorbis")]
    Vorbis,
    #[doc = "audio/vorbis-config"]
    #[serde(alias = "audio/vorbis-config")]
    VorbisConfig,
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
            Audio::VndRhetorex32Kadpcm => write!(f, "audio/vnd.rhetorex.32kadpcm")?,
            Audio::VndRip => write!(f, "audio/vnd.rip")?,
            Audio::VndSealedmediaSoftsealMpeg => write!(f, "audio/vnd.sealedmedia.softseal.mpeg")?,
            Audio::VndVmxCvsd => write!(f, "audio/vnd.vmx.cvsd")?,
            Audio::Vorbis => write!(f, "audio/vorbis")?,
            Audio::VorbisConfig => write!(f, "audio/vorbis-config")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Audio {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "audio/1d-interleaved-parityfec" => Ok(Audio::_1DInterleavedParityfec),
            "audio/32kadpcm" => Ok(Audio::_32Kadpcm),
            "audio/3gpp" => Ok(Audio::_3Gpp),
            "audio/3gpp2" => Ok(Audio::_3Gpp2),
            "audio/aac" => Ok(Audio::Aac),
            "audio/ac3" => Ok(Audio::Ac3),
            "audio/AMR" => Ok(Audio::Amr),
            "audio/AMR-WB" => Ok(Audio::AmrWb),
            "audio/amr-wb+" => Ok(Audio::AmrWb_),
            "audio/aptx" => Ok(Audio::Aptx),
            "audio/asc" => Ok(Audio::Asc),
            "audio/ATRAC-ADVANCED-LOSSLESS" => Ok(Audio::AtracAdvancedLossless),
            "audio/ATRAC-X" => Ok(Audio::AtracX),
            "audio/ATRAC3" => Ok(Audio::Atrac3),
            "audio/basic" => Ok(Audio::Basic),
            "audio/BV16" => Ok(Audio::Bv16),
            "audio/BV32" => Ok(Audio::Bv32),
            "audio/clearmode" => Ok(Audio::Clearmode),
            "audio/CN" => Ok(Audio::Cn),
            "audio/DAT12" => Ok(Audio::Dat12),
            "audio/dls" => Ok(Audio::Dls),
            "audio/dsr-es201108" => Ok(Audio::DsrEs201108),
            "audio/dsr-es202050" => Ok(Audio::DsrEs202050),
            "audio/dsr-es202211" => Ok(Audio::DsrEs202211),
            "audio/dsr-es202212" => Ok(Audio::DsrEs202212),
            "audio/DV" => Ok(Audio::Dv),
            "audio/DVI4" => Ok(Audio::Dvi4),
            "audio/eac3" => Ok(Audio::Eac3),
            "audio/encaprtp" => Ok(Audio::Encaprtp),
            "audio/EVRC" => Ok(Audio::Evrc),
            "audio/EVRC-QCP" => Ok(Audio::EvrcQcp),
            "audio/EVRC0" => Ok(Audio::Evrc0),
            "audio/EVRC1" => Ok(Audio::Evrc1),
            "audio/EVRCB" => Ok(Audio::Evrcb),
            "audio/EVRCB0" => Ok(Audio::Evrcb0),
            "audio/EVRCB1" => Ok(Audio::Evrcb1),
            "audio/EVRCNW" => Ok(Audio::Evrcnw),
            "audio/EVRCNW0" => Ok(Audio::Evrcnw0),
            "audio/EVRCNW1" => Ok(Audio::Evrcnw1),
            "audio/EVRCWB" => Ok(Audio::Evrcwb),
            "audio/EVRCWB0" => Ok(Audio::Evrcwb0),
            "audio/EVRCWB1" => Ok(Audio::Evrcwb1),
            "audio/EVS" => Ok(Audio::Evs),
            "audio/example" => Ok(Audio::Example),
            "audio/flexfec" => Ok(Audio::Flexfec),
            "audio/fwdred" => Ok(Audio::Fwdred),
            "audio/G711-0" => Ok(Audio::G7110),
            "audio/G719" => Ok(Audio::G719),
            "audio/G7221" => Ok(Audio::G7221),
            "audio/G722" => Ok(Audio::G722),
            "audio/G723" => Ok(Audio::G723),
            "audio/G726-16" => Ok(Audio::G72616),
            "audio/G726-24" => Ok(Audio::G72624),
            "audio/G726-32" => Ok(Audio::G72632),
            "audio/G726-40" => Ok(Audio::G72640),
            "audio/G728" => Ok(Audio::G728),
            "audio/G729" => Ok(Audio::G729),
            "audio/G7291" => Ok(Audio::G7291),
            "audio/G729D" => Ok(Audio::G729D),
            "audio/G729E" => Ok(Audio::G729E),
            "audio/GSM" => Ok(Audio::Gsm),
            "audio/GSM-EFR" => Ok(Audio::GsmEfr),
            "audio/GSM-HR-08" => Ok(Audio::GsmHr08),
            "audio/iLBC" => Ok(Audio::Ilbc),
            "audio/ip-mr_v2.5" => Ok(Audio::IpMrV25),
            "audio/L8" => Ok(Audio::L8),
            "audio/L16" => Ok(Audio::L16),
            "audio/L20" => Ok(Audio::L20),
            "audio/L24" => Ok(Audio::L24),
            "audio/LPC" => Ok(Audio::Lpc),
            "audio/MELP" => Ok(Audio::Melp),
            "audio/MELP600" => Ok(Audio::Melp600),
            "audio/MELP1200" => Ok(Audio::Melp1200),
            "audio/MELP2400" => Ok(Audio::Melp2400),
            "audio/mhas" => Ok(Audio::Mhas),
            "audio/mobile-xmf" => Ok(Audio::MobileXmf),
            "audio/MPA" => Ok(Audio::Mpa),
            "audio/mp4" => Ok(Audio::Mp4),
            "audio/MP4A-LATM" => Ok(Audio::Mp4ALatm),
            "audio/mpa-robust" => Ok(Audio::MpaRobust),
            "audio/mpeg" => Ok(Audio::Mpeg),
            "audio/mpeg4-generic" => Ok(Audio::Mpeg4Generic),
            "audio/ogg" => Ok(Audio::Ogg),
            "audio/opus" => Ok(Audio::Opus),
            "audio/parityfec" => Ok(Audio::Parityfec),
            "audio/PCMA" => Ok(Audio::Pcma),
            "audio/PCMA-WB" => Ok(Audio::PcmaWb),
            "audio/PCMU" => Ok(Audio::Pcmu),
            "audio/PCMU-WB" => Ok(Audio::PcmuWb),
            "audio/prs.sid" => Ok(Audio::PrsSid),
            "audio/QCELP" => Ok(Audio::Qcelp),
            "audio/raptorfec" => Ok(Audio::Raptorfec),
            "audio/RED" => Ok(Audio::Red),
            "audio/rtp-enc-aescm128" => Ok(Audio::RtpEncAescm128),
            "audio/rtploopback" => Ok(Audio::Rtploopback),
            "audio/rtp-midi" => Ok(Audio::RtpMidi),
            "audio/rtx" => Ok(Audio::Rtx),
            "audio/scip" => Ok(Audio::Scip),
            "audio/SMV" => Ok(Audio::Smv),
            "audio/SMV0" => Ok(Audio::Smv0),
            "audio/SMV-QCP" => Ok(Audio::SmvQcp),
            "audio/sofa" => Ok(Audio::Sofa),
            "audio/sp-midi" => Ok(Audio::SpMidi),
            "audio/speex" => Ok(Audio::Speex),
            "audio/t140c" => Ok(Audio::T140C),
            "audio/t38" => Ok(Audio::T38),
            "audio/telephone-event" => Ok(Audio::TelephoneEvent),
            "audio/TETRA_ACELP" => Ok(Audio::TetraAcelp),
            "audio/TETRA_ACELP_BB" => Ok(Audio::TetraAcelpBb),
            "audio/tone" => Ok(Audio::Tone),
            "audio/TSVCIS" => Ok(Audio::Tsvcis),
            "audio/UEMCLIP" => Ok(Audio::Uemclip),
            "audio/ulpfec" => Ok(Audio::Ulpfec),
            "audio/usac" => Ok(Audio::Usac),
            "audio/VDVI" => Ok(Audio::Vdvi),
            "audio/VMR-WB" => Ok(Audio::VmrWb),
            "audio/vnd.3gpp.iufp" => Ok(Audio::Vnd3GppIufp),
            "audio/vnd.4SB" => Ok(Audio::Vnd4SB),
            "audio/vnd.audiokoz" => Ok(Audio::VndAudiokoz),
            "audio/vnd.CELP" => Ok(Audio::VndCELP),
            "audio/vnd.cisco.nse" => Ok(Audio::VndCiscoNse),
            "audio/vnd.cmles.radio-events" => Ok(Audio::VndCmlesRadioEvents),
            "audio/vnd.cns.anp1" => Ok(Audio::VndCnsAnp1),
            "audio/vnd.cns.inf1" => Ok(Audio::VndCnsInf1),
            "audio/vnd.dece.audio" => Ok(Audio::VndDeceAudio),
            "audio/vnd.digital-winds" => Ok(Audio::VndDigitalWinds),
            "audio/vnd.dlna.adts" => Ok(Audio::VndDlnaAdts),
            "audio/vnd.dolby.heaac.1" => Ok(Audio::VndDolbyHeaac1),
            "audio/vnd.dolby.heaac.2" => Ok(Audio::VndDolbyHeaac2),
            "audio/vnd.dolby.mlp" => Ok(Audio::VndDolbyMlp),
            "audio/vnd.dolby.mps" => Ok(Audio::VndDolbyMps),
            "audio/vnd.dolby.pl2" => Ok(Audio::VndDolbyPl2),
            "audio/vnd.dolby.pl2x" => Ok(Audio::VndDolbyPl2X),
            "audio/vnd.dolby.pl2z" => Ok(Audio::VndDolbyPl2Z),
            "audio/vnd.dolby.pulse.1" => Ok(Audio::VndDolbyPulse1),
            "audio/vnd.dra" => Ok(Audio::VndDra),
            "audio/vnd.dts" => Ok(Audio::VndDts),
            "audio/vnd.dts.hd" => Ok(Audio::VndDtsHd),
            "audio/vnd.dts.uhd" => Ok(Audio::VndDtsUhd),
            "audio/vnd.dvb.file" => Ok(Audio::VndDvbFile),
            "audio/vnd.everad.plj" => Ok(Audio::VndEveradPlj),
            "audio/vnd.hns.audio" => Ok(Audio::VndHnsAudio),
            "audio/vnd.lucent.voice" => Ok(Audio::VndLucentVoice),
            "audio/vnd.ms-playready.media.pya" => Ok(Audio::VndMsPlayreadyMediaPya),
            "audio/vnd.nokia.mobile-xmf" => Ok(Audio::VndNokiaMobileXmf),
            "audio/vnd.nortel.vbk" => Ok(Audio::VndNortelVbk),
            "audio/vnd.nuera.ecelp4800" => Ok(Audio::VndNueraEcelp4800),
            "audio/vnd.nuera.ecelp7470" => Ok(Audio::VndNueraEcelp7470),
            "audio/vnd.nuera.ecelp9600" => Ok(Audio::VndNueraEcelp9600),
            "audio/vnd.octel.sbc" => Ok(Audio::VndOctelSbc),
            "audio/vnd.presonus.multitrack" => Ok(Audio::VndPresonusMultitrack),
            "audio/vnd.rhetorex.32kadpcm" => Ok(Audio::VndRhetorex32Kadpcm),
            "audio/vnd.rip" => Ok(Audio::VndRip),
            "audio/vnd.sealedmedia.softseal.mpeg" => Ok(Audio::VndSealedmediaSoftsealMpeg),
            "audio/vnd.vmx.cvsd" => Ok(Audio::VndVmxCvsd),
            "audio/vorbis" => Ok(Audio::Vorbis),
            "audio/vorbis-config" => Ok(Audio::VorbisConfig),
            _ => Err(()),
        }
    }
}
