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
    #[serde(rename = "audio/1d-interleaved-parityfec")]
    _1DInterleavedParityfec,
    #[doc = "audio/32kadpcm"]
    #[serde(rename = "audio/32kadpcm")]
    #[serde(alias = "726")]
    _32Kadpcm,
    #[doc = "audio/3gpp"]
    #[serde(rename = "audio/3gpp")]
    _3Gpp,
    #[doc = "audio/3gpp2"]
    #[serde(rename = "audio/3gpp2")]
    _3Gpp2,
    #[doc = "audio/aac"]
    #[serde(rename = "audio/aac")]
    #[serde(alias = "adts")]
    #[serde(alias = "aac")]
    #[serde(alias = "ass")]
    Aac,
    #[doc = "audio/ac3"]
    #[serde(rename = "audio/ac3")]
    #[serde(alias = "ac3")]
    Ac3,
    #[doc = "audio/AMR"]
    #[serde(rename = "audio/AMR")]
    #[serde(alias = "amr")]
    Amr,
    #[doc = "audio/AMR-WB"]
    #[serde(rename = "audio/AMR-WB")]
    #[serde(alias = "awb")]
    AmrWb,
    #[doc = "audio/amr-wb+"]
    #[serde(rename = "audio/amr-wb+")]
    AmrWb_,
    #[doc = "audio/aptx"]
    #[serde(rename = "audio/aptx")]
    Aptx,
    #[doc = "audio/asc"]
    #[serde(rename = "audio/asc")]
    #[serde(alias = "acn")]
    Asc,
    #[doc = "audio/ATRAC-ADVANCED-LOSSLESS"]
    #[serde(rename = "audio/ATRAC-ADVANCED-LOSSLESS")]
    #[serde(alias = "aal")]
    AtracAdvancedLossless,
    #[doc = "audio/ATRAC-X"]
    #[serde(rename = "audio/ATRAC-X")]
    #[serde(alias = "atx")]
    AtracX,
    #[doc = "audio/ATRAC3"]
    #[serde(rename = "audio/ATRAC3")]
    #[serde(alias = "at3")]
    #[serde(alias = "aa3")]
    #[serde(alias = "omg")]
    Atrac3,
    #[doc = "audio/basic"]
    #[serde(rename = "audio/basic")]
    #[serde(alias = "au")]
    #[serde(alias = "snd")]
    Basic,
    #[doc = "audio/BV16"]
    #[serde(rename = "audio/BV16")]
    Bv16,
    #[doc = "audio/BV32"]
    #[serde(rename = "audio/BV32")]
    Bv32,
    #[doc = "audio/clearmode"]
    #[serde(rename = "audio/clearmode")]
    Clearmode,
    #[doc = "audio/CN"]
    #[serde(rename = "audio/CN")]
    Cn,
    #[doc = "audio/DAT12"]
    #[serde(rename = "audio/DAT12")]
    Dat12,
    #[doc = "audio/dls"]
    #[serde(rename = "audio/dls")]
    #[serde(alias = "dls")]
    Dls,
    #[doc = "audio/dsr-es201108"]
    #[serde(rename = "audio/dsr-es201108")]
    DsrEs201108,
    #[doc = "audio/dsr-es202050"]
    #[serde(rename = "audio/dsr-es202050")]
    DsrEs202050,
    #[doc = "audio/dsr-es202211"]
    #[serde(rename = "audio/dsr-es202211")]
    DsrEs202211,
    #[doc = "audio/dsr-es202212"]
    #[serde(rename = "audio/dsr-es202212")]
    DsrEs202212,
    #[doc = "audio/DV"]
    #[serde(rename = "audio/DV")]
    Dv,
    #[doc = "audio/DVI4"]
    #[serde(rename = "audio/DVI4")]
    Dvi4,
    #[doc = "audio/eac3"]
    #[serde(rename = "audio/eac3")]
    Eac3,
    #[doc = "audio/encaprtp"]
    #[serde(rename = "audio/encaprtp")]
    Encaprtp,
    #[doc = "audio/EVRC"]
    #[serde(rename = "audio/EVRC")]
    #[serde(alias = "evc")]
    Evrc,
    #[doc = "audio/EVRC-QCP"]
    #[serde(rename = "audio/EVRC-QCP")]
    EvrcQcp,
    #[doc = "audio/EVRC0"]
    #[serde(rename = "audio/EVRC0")]
    Evrc0,
    #[doc = "audio/EVRC1"]
    #[serde(rename = "audio/EVRC1")]
    Evrc1,
    #[doc = "audio/EVRCB"]
    #[serde(rename = "audio/EVRCB")]
    #[serde(alias = "evb")]
    Evrcb,
    #[doc = "audio/EVRCB0"]
    #[serde(rename = "audio/EVRCB0")]
    Evrcb0,
    #[doc = "audio/EVRCB1"]
    #[serde(rename = "audio/EVRCB1")]
    Evrcb1,
    #[doc = "audio/EVRCNW"]
    #[serde(rename = "audio/EVRCNW")]
    #[serde(alias = "enw")]
    Evrcnw,
    #[doc = "audio/EVRCNW0"]
    #[serde(rename = "audio/EVRCNW0")]
    Evrcnw0,
    #[doc = "audio/EVRCNW1"]
    #[serde(rename = "audio/EVRCNW1")]
    Evrcnw1,
    #[doc = "audio/EVRCWB"]
    #[serde(rename = "audio/EVRCWB")]
    #[serde(alias = "evw")]
    Evrcwb,
    #[doc = "audio/EVRCWB0"]
    #[serde(rename = "audio/EVRCWB0")]
    Evrcwb0,
    #[doc = "audio/EVRCWB1"]
    #[serde(rename = "audio/EVRCWB1")]
    Evrcwb1,
    #[doc = "audio/EVS"]
    #[serde(rename = "audio/EVS")]
    Evs,
    #[doc = "audio/example"]
    #[serde(rename = "audio/example")]
    Example,
    #[doc = "audio/flexfec"]
    #[serde(rename = "audio/flexfec")]
    Flexfec,
    #[doc = "audio/fwdred"]
    #[serde(rename = "audio/fwdred")]
    Fwdred,
    #[doc = "audio/G711-0"]
    #[serde(rename = "audio/G711-0")]
    G7110,
    #[doc = "audio/G719"]
    #[serde(rename = "audio/G719")]
    G719,
    #[doc = "audio/G7221"]
    #[serde(rename = "audio/G7221")]
    G7221,
    #[doc = "audio/G722"]
    #[serde(rename = "audio/G722")]
    G722,
    #[doc = "audio/G723"]
    #[serde(rename = "audio/G723")]
    G723,
    #[doc = "audio/G726-16"]
    #[serde(rename = "audio/G726-16")]
    G72616,
    #[doc = "audio/G726-24"]
    #[serde(rename = "audio/G726-24")]
    G72624,
    #[doc = "audio/G726-32"]
    #[serde(rename = "audio/G726-32")]
    G72632,
    #[doc = "audio/G726-40"]
    #[serde(rename = "audio/G726-40")]
    G72640,
    #[doc = "audio/G728"]
    #[serde(rename = "audio/G728")]
    G728,
    #[doc = "audio/G729"]
    #[serde(rename = "audio/G729")]
    G729,
    #[doc = "audio/G7291"]
    #[serde(rename = "audio/G7291")]
    G7291,
    #[doc = "audio/G729D"]
    #[serde(rename = "audio/G729D")]
    G729D,
    #[doc = "audio/G729E"]
    #[serde(rename = "audio/G729E")]
    G729E,
    #[doc = "audio/GSM"]
    #[serde(rename = "audio/GSM")]
    Gsm,
    #[doc = "audio/GSM-EFR"]
    #[serde(rename = "audio/GSM-EFR")]
    GsmEfr,
    #[doc = "audio/GSM-HR-08"]
    #[serde(rename = "audio/GSM-HR-08")]
    GsmHr08,
    #[doc = "audio/iLBC"]
    #[serde(rename = "audio/iLBC")]
    #[serde(alias = "lbc")]
    Ilbc,
    #[doc = "audio/ip-mr_v2.5"]
    #[serde(rename = "audio/ip-mr_v2.5")]
    IpMrV25,
    #[doc = "audio/L8"]
    #[serde(rename = "audio/L8")]
    L8,
    #[doc = "audio/L16"]
    #[serde(rename = "audio/L16")]
    #[serde(alias = "l16")]
    L16,
    #[doc = "audio/L20"]
    #[serde(rename = "audio/L20")]
    L20,
    #[doc = "audio/L24"]
    #[serde(rename = "audio/L24")]
    L24,
    #[doc = "audio/LPC"]
    #[serde(rename = "audio/LPC")]
    Lpc,
    #[doc = "audio/MELP"]
    #[serde(rename = "audio/MELP")]
    Melp,
    #[doc = "audio/MELP600"]
    #[serde(rename = "audio/MELP600")]
    Melp600,
    #[doc = "audio/MELP1200"]
    #[serde(rename = "audio/MELP1200")]
    Melp1200,
    #[doc = "audio/MELP2400"]
    #[serde(rename = "audio/MELP2400")]
    Melp2400,
    #[doc = "audio/mhas"]
    #[serde(rename = "audio/mhas")]
    #[serde(alias = "mhas")]
    Mhas,
    #[doc = "audio/mobile-xmf"]
    #[serde(rename = "audio/mobile-xmf")]
    #[serde(alias = "mxmf")]
    MobileXmf,
    #[doc = "audio/MPA"]
    #[serde(rename = "audio/MPA")]
    Mpa,
    #[doc = "audio/mp4"]
    #[serde(rename = "audio/mp4")]
    #[serde(alias = "m4a")]
    Mp4,
    #[doc = "audio/MP4A-LATM"]
    #[serde(rename = "audio/MP4A-LATM")]
    Mp4ALatm,
    #[doc = "audio/mpa-robust"]
    #[serde(rename = "audio/mpa-robust")]
    MpaRobust,
    #[doc = "audio/mpeg"]
    #[serde(rename = "audio/mpeg")]
    #[serde(alias = "mp3")]
    #[serde(alias = "mpga")]
    #[serde(alias = "mp1")]
    #[serde(alias = "mp2")]
    Mpeg,
    #[doc = "audio/mpeg4-generic"]
    #[serde(rename = "audio/mpeg4-generic")]
    Mpeg4Generic,
    #[doc = "audio/ogg"]
    #[serde(rename = "audio/ogg")]
    #[serde(alias = "oga")]
    #[serde(alias = "ogg")]
    #[serde(alias = "opus")]
    #[serde(alias = "spx")]
    Ogg,
    #[doc = "audio/opus"]
    #[serde(rename = "audio/opus")]
    Opus,
    #[doc = "audio/parityfec"]
    #[serde(rename = "audio/parityfec")]
    Parityfec,
    #[doc = "audio/PCMA"]
    #[serde(rename = "audio/PCMA")]
    Pcma,
    #[doc = "audio/PCMA-WB"]
    #[serde(rename = "audio/PCMA-WB")]
    PcmaWb,
    #[doc = "audio/PCMU"]
    #[serde(rename = "audio/PCMU")]
    Pcmu,
    #[doc = "audio/PCMU-WB"]
    #[serde(rename = "audio/PCMU-WB")]
    PcmuWb,
    #[doc = "audio/prs.sid"]
    #[serde(rename = "audio/prs.sid")]
    #[serde(alias = "sid")]
    #[serde(alias = "psid")]
    PrsSid,
    #[doc = "audio/QCELP"]
    #[serde(rename = "audio/QCELP")]
    #[serde(alias = "qcp")]
    Qcelp,
    #[doc = "audio/raptorfec"]
    #[serde(rename = "audio/raptorfec")]
    Raptorfec,
    #[doc = "audio/RED"]
    #[serde(rename = "audio/RED")]
    Red,
    #[doc = "audio/rtp-enc-aescm128"]
    #[serde(rename = "audio/rtp-enc-aescm128")]
    RtpEncAescm128,
    #[doc = "audio/rtploopback"]
    #[serde(rename = "audio/rtploopback")]
    Rtploopback,
    #[doc = "audio/rtp-midi"]
    #[serde(rename = "audio/rtp-midi")]
    RtpMidi,
    #[doc = "audio/rtx"]
    #[serde(rename = "audio/rtx")]
    Rtx,
    #[doc = "audio/scip"]
    #[serde(rename = "audio/scip")]
    Scip,
    #[doc = "audio/SMV"]
    #[serde(rename = "audio/SMV")]
    #[serde(alias = "smv")]
    Smv,
    #[doc = "audio/SMV0"]
    #[serde(rename = "audio/SMV0")]
    Smv0,
    #[doc = "audio/SMV-QCP"]
    #[serde(rename = "audio/SMV-QCP")]
    SmvQcp,
    #[doc = "audio/sofa"]
    #[serde(rename = "audio/sofa")]
    #[serde(alias = "sofa")]
    Sofa,
    #[doc = "audio/sp-midi"]
    #[serde(rename = "audio/sp-midi")]
    SpMidi,
    #[doc = "audio/speex"]
    #[serde(rename = "audio/speex")]
    Speex,
    #[doc = "audio/t140c"]
    #[serde(rename = "audio/t140c")]
    T140C,
    #[doc = "audio/t38"]
    #[serde(rename = "audio/t38")]
    T38,
    #[doc = "audio/telephone-event"]
    #[serde(rename = "audio/telephone-event")]
    TelephoneEvent,
    #[doc = "audio/TETRA_ACELP"]
    #[serde(rename = "audio/TETRA_ACELP")]
    TetraAcelp,
    #[doc = "audio/TETRA_ACELP_BB"]
    #[serde(rename = "audio/TETRA_ACELP_BB")]
    TetraAcelpBb,
    #[doc = "audio/tone"]
    #[serde(rename = "audio/tone")]
    Tone,
    #[doc = "audio/TSVCIS"]
    #[serde(rename = "audio/TSVCIS")]
    Tsvcis,
    #[doc = "audio/UEMCLIP"]
    #[serde(rename = "audio/UEMCLIP")]
    Uemclip,
    #[doc = "audio/ulpfec"]
    #[serde(rename = "audio/ulpfec")]
    Ulpfec,
    #[doc = "audio/usac"]
    #[serde(rename = "audio/usac")]
    #[serde(alias = "loas")]
    #[serde(alias = "xhe")]
    Usac,
    #[doc = "audio/VDVI"]
    #[serde(rename = "audio/VDVI")]
    Vdvi,
    #[doc = "audio/VMR-WB"]
    #[serde(rename = "audio/VMR-WB")]
    VmrWb,
    #[doc = "audio/vnd.3gpp.iufp"]
    #[serde(rename = "audio/vnd.3gpp.iufp")]
    Vnd3GppIufp,
    #[doc = "audio/vnd.4SB"]
    #[serde(rename = "audio/vnd.4SB")]
    Vnd4SB,
    #[doc = "audio/vnd.audiokoz"]
    #[serde(rename = "audio/vnd.audiokoz")]
    #[serde(alias = "koz")]
    VndAudiokoz,
    #[doc = "audio/vnd.CELP"]
    #[serde(rename = "audio/vnd.CELP")]
    VndCELP,
    #[doc = "audio/vnd.cisco.nse"]
    #[serde(rename = "audio/vnd.cisco.nse")]
    VndCiscoNse,
    #[doc = "audio/vnd.cmles.radio-events"]
    #[serde(rename = "audio/vnd.cmles.radio-events")]
    VndCmlesRadioEvents,
    #[doc = "audio/vnd.cns.anp1"]
    #[serde(rename = "audio/vnd.cns.anp1")]
    VndCnsAnp1,
    #[doc = "audio/vnd.cns.inf1"]
    #[serde(rename = "audio/vnd.cns.inf1")]
    VndCnsInf1,
    #[doc = "audio/vnd.dece.audio"]
    #[serde(rename = "audio/vnd.dece.audio")]
    #[serde(alias = "uva")]
    #[serde(alias = "uvva")]
    VndDeceAudio,
    #[doc = "audio/vnd.digital-winds"]
    #[serde(rename = "audio/vnd.digital-winds")]
    #[serde(alias = "eol")]
    VndDigitalWinds,
    #[doc = "audio/vnd.dlna.adts"]
    #[serde(rename = "audio/vnd.dlna.adts")]
    VndDlnaAdts,
    #[doc = "audio/vnd.dolby.heaac.1"]
    #[serde(rename = "audio/vnd.dolby.heaac.1")]
    VndDolbyHeaac1,
    #[doc = "audio/vnd.dolby.heaac.2"]
    #[serde(rename = "audio/vnd.dolby.heaac.2")]
    VndDolbyHeaac2,
    #[doc = "audio/vnd.dolby.mlp"]
    #[serde(rename = "audio/vnd.dolby.mlp")]
    #[serde(alias = "mlp")]
    VndDolbyMlp,
    #[doc = "audio/vnd.dolby.mps"]
    #[serde(rename = "audio/vnd.dolby.mps")]
    VndDolbyMps,
    #[doc = "audio/vnd.dolby.pl2"]
    #[serde(rename = "audio/vnd.dolby.pl2")]
    VndDolbyPl2,
    #[doc = "audio/vnd.dolby.pl2x"]
    #[serde(rename = "audio/vnd.dolby.pl2x")]
    VndDolbyPl2X,
    #[doc = "audio/vnd.dolby.pl2z"]
    #[serde(rename = "audio/vnd.dolby.pl2z")]
    VndDolbyPl2Z,
    #[doc = "audio/vnd.dolby.pulse.1"]
    #[serde(rename = "audio/vnd.dolby.pulse.1")]
    VndDolbyPulse1,
    #[doc = "audio/vnd.dra"]
    #[serde(rename = "audio/vnd.dra")]
    VndDra,
    #[doc = "audio/vnd.dts"]
    #[serde(rename = "audio/vnd.dts")]
    #[serde(alias = "dts")]
    VndDts,
    #[doc = "audio/vnd.dts.hd"]
    #[serde(rename = "audio/vnd.dts.hd")]
    #[serde(alias = "dtshd")]
    VndDtsHd,
    #[doc = "audio/vnd.dts.uhd"]
    #[serde(rename = "audio/vnd.dts.uhd")]
    VndDtsUhd,
    #[doc = "audio/vnd.dvb.file"]
    #[serde(rename = "audio/vnd.dvb.file")]
    VndDvbFile,
    #[doc = "audio/vnd.everad.plj"]
    #[serde(rename = "audio/vnd.everad.plj")]
    #[serde(alias = "plj")]
    VndEveradPlj,
    #[doc = "audio/vnd.hns.audio"]
    #[serde(rename = "audio/vnd.hns.audio")]
    VndHnsAudio,
    #[doc = "audio/vnd.lucent.voice"]
    #[serde(rename = "audio/vnd.lucent.voice")]
    #[serde(alias = "lvp")]
    VndLucentVoice,
    #[doc = "audio/vnd.ms-playready.media.pya"]
    #[serde(rename = "audio/vnd.ms-playready.media.pya")]
    #[serde(alias = "pya")]
    VndMsPlayreadyMediaPya,
    #[doc = "audio/vnd.nokia.mobile-xmf"]
    #[serde(rename = "audio/vnd.nokia.mobile-xmf")]
    VndNokiaMobileXmf,
    #[doc = "audio/vnd.nortel.vbk"]
    #[serde(rename = "audio/vnd.nortel.vbk")]
    #[serde(alias = "vbk")]
    VndNortelVbk,
    #[doc = "audio/vnd.nuera.ecelp4800"]
    #[serde(rename = "audio/vnd.nuera.ecelp4800")]
    #[serde(alias = "ecelp4800")]
    VndNueraEcelp4800,
    #[doc = "audio/vnd.nuera.ecelp7470"]
    #[serde(rename = "audio/vnd.nuera.ecelp7470")]
    #[serde(alias = "ecelp7470")]
    VndNueraEcelp7470,
    #[doc = "audio/vnd.nuera.ecelp9600"]
    #[serde(rename = "audio/vnd.nuera.ecelp9600")]
    #[serde(alias = "ecelp9600")]
    VndNueraEcelp9600,
    #[doc = "audio/vnd.octel.sbc"]
    #[serde(rename = "audio/vnd.octel.sbc")]
    VndOctelSbc,
    #[doc = "audio/vnd.presonus.multitrack"]
    #[serde(rename = "audio/vnd.presonus.multitrack")]
    #[serde(alias = "multitrack")]
    VndPresonusMultitrack,
    #[doc = "audio/vnd.rhetorex.32kadpcm"]
    #[serde(rename = "audio/vnd.rhetorex.32kadpcm")]
    VndRhetorex32Kadpcm,
    #[doc = "audio/vnd.rip"]
    #[serde(rename = "audio/vnd.rip")]
    #[serde(alias = "rip")]
    VndRip,
    #[doc = "audio/vnd.sealedmedia.softseal.mpeg"]
    #[serde(rename = "audio/vnd.sealedmedia.softseal.mpeg")]
    #[serde(alias = "smp3")]
    #[serde(alias = "smp")]
    #[serde(alias = "s1m")]
    VndSealedmediaSoftsealMpeg,
    #[doc = "audio/vnd.vmx.cvsd"]
    #[serde(rename = "audio/vnd.vmx.cvsd")]
    VndVmxCvsd,
    #[doc = "audio/vorbis"]
    #[serde(rename = "audio/vorbis")]
    Vorbis,
    #[doc = "audio/vorbis-config"]
    #[serde(rename = "audio/vorbis-config")]
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
            "audio/32kadpcm" | "726" => Ok(Audio::_32Kadpcm),
            "audio/3gpp" => Ok(Audio::_3Gpp),
            "audio/3gpp2" => Ok(Audio::_3Gpp2),
            "audio/aac" | "adts" | "aac" | "ass" => Ok(Audio::Aac),
            "audio/ac3" | "ac3" => Ok(Audio::Ac3),
            "audio/AMR" | "amr" => Ok(Audio::Amr),
            "audio/AMR-WB" | "awb" => Ok(Audio::AmrWb),
            "audio/amr-wb+" => Ok(Audio::AmrWb_),
            "audio/aptx" => Ok(Audio::Aptx),
            "audio/asc" | "acn" => Ok(Audio::Asc),
            "audio/ATRAC-ADVANCED-LOSSLESS" | "aal" => Ok(Audio::AtracAdvancedLossless),
            "audio/ATRAC-X" | "atx" => Ok(Audio::AtracX),
            "audio/ATRAC3" | "at3" | "aa3" | "omg" => Ok(Audio::Atrac3),
            "audio/basic" | "au" | "snd" => Ok(Audio::Basic),
            "audio/BV16" => Ok(Audio::Bv16),
            "audio/BV32" => Ok(Audio::Bv32),
            "audio/clearmode" => Ok(Audio::Clearmode),
            "audio/CN" => Ok(Audio::Cn),
            "audio/DAT12" => Ok(Audio::Dat12),
            "audio/dls" | "dls" => Ok(Audio::Dls),
            "audio/dsr-es201108" => Ok(Audio::DsrEs201108),
            "audio/dsr-es202050" => Ok(Audio::DsrEs202050),
            "audio/dsr-es202211" => Ok(Audio::DsrEs202211),
            "audio/dsr-es202212" => Ok(Audio::DsrEs202212),
            "audio/DV" => Ok(Audio::Dv),
            "audio/DVI4" => Ok(Audio::Dvi4),
            "audio/eac3" => Ok(Audio::Eac3),
            "audio/encaprtp" => Ok(Audio::Encaprtp),
            "audio/EVRC" | "evc" => Ok(Audio::Evrc),
            "audio/EVRC-QCP" => Ok(Audio::EvrcQcp),
            "audio/EVRC0" => Ok(Audio::Evrc0),
            "audio/EVRC1" => Ok(Audio::Evrc1),
            "audio/EVRCB" | "evb" => Ok(Audio::Evrcb),
            "audio/EVRCB0" => Ok(Audio::Evrcb0),
            "audio/EVRCB1" => Ok(Audio::Evrcb1),
            "audio/EVRCNW" | "enw" => Ok(Audio::Evrcnw),
            "audio/EVRCNW0" => Ok(Audio::Evrcnw0),
            "audio/EVRCNW1" => Ok(Audio::Evrcnw1),
            "audio/EVRCWB" | "evw" => Ok(Audio::Evrcwb),
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
            "audio/iLBC" | "lbc" => Ok(Audio::Ilbc),
            "audio/ip-mr_v2.5" => Ok(Audio::IpMrV25),
            "audio/L8" => Ok(Audio::L8),
            "audio/L16" | "l16" => Ok(Audio::L16),
            "audio/L20" => Ok(Audio::L20),
            "audio/L24" => Ok(Audio::L24),
            "audio/LPC" => Ok(Audio::Lpc),
            "audio/MELP" => Ok(Audio::Melp),
            "audio/MELP600" => Ok(Audio::Melp600),
            "audio/MELP1200" => Ok(Audio::Melp1200),
            "audio/MELP2400" => Ok(Audio::Melp2400),
            "audio/mhas" | "mhas" => Ok(Audio::Mhas),
            "audio/mobile-xmf" | "mxmf" => Ok(Audio::MobileXmf),
            "audio/MPA" => Ok(Audio::Mpa),
            "audio/mp4" | "m4a" => Ok(Audio::Mp4),
            "audio/MP4A-LATM" => Ok(Audio::Mp4ALatm),
            "audio/mpa-robust" => Ok(Audio::MpaRobust),
            "audio/mpeg" | "mp3" | "mpga" | "mp1" | "mp2" => Ok(Audio::Mpeg),
            "audio/mpeg4-generic" => Ok(Audio::Mpeg4Generic),
            "audio/ogg" | "oga" | "ogg" | "opus" | "spx" => Ok(Audio::Ogg),
            "audio/opus" => Ok(Audio::Opus),
            "audio/parityfec" => Ok(Audio::Parityfec),
            "audio/PCMA" => Ok(Audio::Pcma),
            "audio/PCMA-WB" => Ok(Audio::PcmaWb),
            "audio/PCMU" => Ok(Audio::Pcmu),
            "audio/PCMU-WB" => Ok(Audio::PcmuWb),
            "audio/prs.sid" | "sid" | "psid" => Ok(Audio::PrsSid),
            "audio/QCELP" | "qcp" => Ok(Audio::Qcelp),
            "audio/raptorfec" => Ok(Audio::Raptorfec),
            "audio/RED" => Ok(Audio::Red),
            "audio/rtp-enc-aescm128" => Ok(Audio::RtpEncAescm128),
            "audio/rtploopback" => Ok(Audio::Rtploopback),
            "audio/rtp-midi" => Ok(Audio::RtpMidi),
            "audio/rtx" => Ok(Audio::Rtx),
            "audio/scip" => Ok(Audio::Scip),
            "audio/SMV" | "smv" => Ok(Audio::Smv),
            "audio/SMV0" => Ok(Audio::Smv0),
            "audio/SMV-QCP" => Ok(Audio::SmvQcp),
            "audio/sofa" | "sofa" => Ok(Audio::Sofa),
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
            "audio/usac" | "loas" | "xhe" => Ok(Audio::Usac),
            "audio/VDVI" => Ok(Audio::Vdvi),
            "audio/VMR-WB" => Ok(Audio::VmrWb),
            "audio/vnd.3gpp.iufp" => Ok(Audio::Vnd3GppIufp),
            "audio/vnd.4SB" => Ok(Audio::Vnd4SB),
            "audio/vnd.audiokoz" | "koz" => Ok(Audio::VndAudiokoz),
            "audio/vnd.CELP" => Ok(Audio::VndCELP),
            "audio/vnd.cisco.nse" => Ok(Audio::VndCiscoNse),
            "audio/vnd.cmles.radio-events" => Ok(Audio::VndCmlesRadioEvents),
            "audio/vnd.cns.anp1" => Ok(Audio::VndCnsAnp1),
            "audio/vnd.cns.inf1" => Ok(Audio::VndCnsInf1),
            "audio/vnd.dece.audio" | "uva" | "uvva" => Ok(Audio::VndDeceAudio),
            "audio/vnd.digital-winds" | "eol" => Ok(Audio::VndDigitalWinds),
            "audio/vnd.dlna.adts" => Ok(Audio::VndDlnaAdts),
            "audio/vnd.dolby.heaac.1" => Ok(Audio::VndDolbyHeaac1),
            "audio/vnd.dolby.heaac.2" => Ok(Audio::VndDolbyHeaac2),
            "audio/vnd.dolby.mlp" | "mlp" => Ok(Audio::VndDolbyMlp),
            "audio/vnd.dolby.mps" => Ok(Audio::VndDolbyMps),
            "audio/vnd.dolby.pl2" => Ok(Audio::VndDolbyPl2),
            "audio/vnd.dolby.pl2x" => Ok(Audio::VndDolbyPl2X),
            "audio/vnd.dolby.pl2z" => Ok(Audio::VndDolbyPl2Z),
            "audio/vnd.dolby.pulse.1" => Ok(Audio::VndDolbyPulse1),
            "audio/vnd.dra" => Ok(Audio::VndDra),
            "audio/vnd.dts" | "dts" => Ok(Audio::VndDts),
            "audio/vnd.dts.hd" | "dtshd" => Ok(Audio::VndDtsHd),
            "audio/vnd.dts.uhd" => Ok(Audio::VndDtsUhd),
            "audio/vnd.dvb.file" => Ok(Audio::VndDvbFile),
            "audio/vnd.everad.plj" | "plj" => Ok(Audio::VndEveradPlj),
            "audio/vnd.hns.audio" => Ok(Audio::VndHnsAudio),
            "audio/vnd.lucent.voice" | "lvp" => Ok(Audio::VndLucentVoice),
            "audio/vnd.ms-playready.media.pya" | "pya" => Ok(Audio::VndMsPlayreadyMediaPya),
            "audio/vnd.nokia.mobile-xmf" => Ok(Audio::VndNokiaMobileXmf),
            "audio/vnd.nortel.vbk" | "vbk" => Ok(Audio::VndNortelVbk),
            "audio/vnd.nuera.ecelp4800" | "ecelp4800" => Ok(Audio::VndNueraEcelp4800),
            "audio/vnd.nuera.ecelp7470" | "ecelp7470" => Ok(Audio::VndNueraEcelp7470),
            "audio/vnd.nuera.ecelp9600" | "ecelp9600" => Ok(Audio::VndNueraEcelp9600),
            "audio/vnd.octel.sbc" => Ok(Audio::VndOctelSbc),
            "audio/vnd.presonus.multitrack" | "multitrack" => Ok(Audio::VndPresonusMultitrack),
            "audio/vnd.rhetorex.32kadpcm" => Ok(Audio::VndRhetorex32Kadpcm),
            "audio/vnd.rip" | "rip" => Ok(Audio::VndRip),
            "audio/vnd.sealedmedia.softseal.mpeg" | "smp3" | "smp" | "s1m" => {
                Ok(Audio::VndSealedmediaSoftsealMpeg)
            }
            "audio/vnd.vmx.cvsd" => Ok(Audio::VndVmxCvsd),
            "audio/vorbis" => Ok(Audio::Vorbis),
            "audio/vorbis-config" => Ok(Audio::VorbisConfig),
            _ => Err(()),
        }
    }
}
