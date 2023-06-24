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
pub enum Video {
    #[doc = "video/1d-interleaved-parityfec"]
    #[serde(rename = "video/1d-interleaved-parityfec")]
    _1DInterleavedParityfec,
    #[doc = "video/3gpp"]
    #[serde(rename = "video/3gpp")]
    #[serde(alias = "3gp")]
    #[serde(alias = "3gpp")]
    _3Gpp,
    #[doc = "video/3gpp2"]
    #[serde(rename = "video/3gpp2")]
    #[serde(alias = "3g2")]
    #[serde(alias = "3gpp2")]
    _3Gpp2,
    #[doc = "video/3gpp-tt"]
    #[serde(rename = "video/3gpp-tt")]
    _3GppTt,
    #[doc = "video/AV1"]
    #[serde(rename = "video/AV1")]
    Av1,
    #[doc = "video/BMPEG"]
    #[serde(rename = "video/BMPEG")]
    Bmpeg,
    #[doc = "video/BT656"]
    #[serde(rename = "video/BT656")]
    Bt656,
    #[doc = "video/CelB"]
    #[serde(rename = "video/CelB")]
    CelB,
    #[doc = "video/DV"]
    #[serde(rename = "video/DV")]
    Dv,
    #[doc = "video/encaprtp"]
    #[serde(rename = "video/encaprtp")]
    Encaprtp,
    #[doc = "video/example"]
    #[serde(rename = "video/example")]
    Example,
    #[doc = "video/FFV1"]
    #[serde(rename = "video/FFV1")]
    Ffv1,
    #[doc = "video/flexfec"]
    #[serde(rename = "video/flexfec")]
    Flexfec,
    #[doc = "video/H261"]
    #[serde(rename = "video/H261")]
    H261,
    #[doc = "video/H263"]
    #[serde(rename = "video/H263")]
    H263,
    #[doc = "video/H263-1998"]
    #[serde(rename = "video/H263-1998")]
    H2631998,
    #[doc = "video/H263-2000"]
    #[serde(rename = "video/H263-2000")]
    H2632000,
    #[doc = "video/H264"]
    #[serde(rename = "video/H264")]
    H264,
    #[doc = "video/H264-RCDO"]
    #[serde(rename = "video/H264-RCDO")]
    H264Rcdo,
    #[doc = "video/H264-SVC"]
    #[serde(rename = "video/H264-SVC")]
    H264Svc,
    #[doc = "video/H265"]
    #[serde(rename = "video/H265")]
    H265,
    #[doc = "video/H266"]
    #[serde(rename = "video/H266")]
    H266,
    #[doc = "video/iso.segment"]
    #[serde(rename = "video/iso.segment")]
    #[serde(alias = "m4s")]
    IsoSegment,
    #[doc = "video/JPEG"]
    #[serde(rename = "video/JPEG")]
    Jpeg,
    #[doc = "video/jpeg2000"]
    #[serde(rename = "video/jpeg2000")]
    Jpeg2000,
    #[doc = "video/jxsv"]
    #[serde(rename = "video/jxsv")]
    Jxsv,
    #[doc = "video/mj2"]
    #[serde(rename = "video/mj2")]
    #[serde(alias = "mj2")]
    #[serde(alias = "mjp2")]
    Mj2,
    #[doc = "video/MP1S"]
    #[serde(rename = "video/MP1S")]
    Mp1S,
    #[doc = "video/MP2P"]
    #[serde(rename = "video/MP2P")]
    Mp2P,
    #[doc = "video/MP2T"]
    #[serde(rename = "video/MP2T")]
    Mp2T,
    #[doc = "video/mp4"]
    #[serde(rename = "video/mp4")]
    #[serde(alias = "mp4")]
    #[serde(alias = "mpg4")]
    #[serde(alias = "m4v")]
    Mp4,
    #[doc = "video/MP4V-ES"]
    #[serde(rename = "video/MP4V-ES")]
    Mp4VEs,
    #[doc = "video/MPV"]
    #[serde(rename = "video/MPV")]
    Mpv,
    #[doc = "video/mpeg4-generic"]
    #[serde(rename = "video/mpeg4-generic")]
    Mpeg,
    #[doc = "video/nv"]
    #[serde(rename = "video/nv")]
    Mpeg4Generic,
    #[doc = "video/ogg"]
    #[serde(rename = "video/ogg")]
    #[serde(alias = "ogv")]
    Nv,
    #[doc = "video/parityfec"]
    #[serde(rename = "video/parityfec")]
    Ogg,
    #[doc = "video/pointer"]
    #[serde(rename = "video/pointer")]
    Parityfec,
    #[doc = "video/quicktime"]
    #[serde(rename = "video/quicktime")]
    #[serde(alias = "mov")]
    #[serde(alias = "qt")]
    Pointer,
    #[doc = "video/raptorfec"]
    #[serde(rename = "video/raptorfec")]
    Quicktime,
    #[doc = "video/raw"]
    #[serde(rename = "video/raw")]
    Raptorfec,
    #[doc = "video/rtp-enc-aescm128"]
    #[serde(rename = "video/rtp-enc-aescm128")]
    Raw,
    #[doc = "video/rtploopback"]
    #[serde(rename = "video/rtploopback")]
    RtpEncAescm128,
    #[doc = "video/rtx"]
    #[serde(rename = "video/rtx")]
    Rtploopback,
    #[doc = "video/scip"]
    #[serde(rename = "video/scip")]
    Rtx,
    #[doc = "video/smpte291"]
    #[serde(rename = "video/smpte291")]
    Scip,
    #[doc = "video/SMPTE292M"]
    #[serde(rename = "video/SMPTE292M")]
    Smpte291,
    #[doc = "video/ulpfec"]
    #[serde(rename = "video/ulpfec")]
    Smpte292M,
    #[doc = "video/vc1"]
    #[serde(rename = "video/vc1")]
    Ulpfec,
    #[doc = "video/vc2"]
    #[serde(rename = "video/vc2")]
    Vc1,
    #[doc = "video/vnd.CCTV"]
    #[serde(rename = "video/vnd.CCTV")]
    Vc2,
    #[doc = "video/vnd.dece.hd"]
    #[serde(rename = "video/vnd.dece.hd")]
    #[serde(alias = "uvh")]
    #[serde(alias = "uvvh")]
    VndCCTV,
    #[doc = "video/vnd.dece.mobile"]
    #[serde(rename = "video/vnd.dece.mobile")]
    #[serde(alias = "uvm")]
    #[serde(alias = "uvvm")]
    VndDeceHd,
    #[doc = "video/vnd.dece.mp4"]
    #[serde(rename = "video/vnd.dece.mp4")]
    #[serde(alias = "uvu")]
    #[serde(alias = "uvvu")]
    VndDeceMobile,
    #[doc = "video/vnd.dece.pd"]
    #[serde(rename = "video/vnd.dece.pd")]
    #[serde(alias = "uvp")]
    #[serde(alias = "uvvp")]
    VndDeceMp4,
    #[doc = "video/vnd.dece.sd"]
    #[serde(rename = "video/vnd.dece.sd")]
    #[serde(alias = "uvs")]
    #[serde(alias = "uvvs")]
    VndDecePd,
    #[doc = "video/vnd.dece.video"]
    #[serde(rename = "video/vnd.dece.video")]
    #[serde(alias = "uvv")]
    #[serde(alias = "uvvv")]
    VndDeceSd,
    #[doc = "video/vnd.directv.mpeg"]
    #[serde(rename = "video/vnd.directv.mpeg")]
    VndDeceVideo,
    #[doc = "video/vnd.directv.mpeg-tts"]
    #[serde(rename = "video/vnd.directv.mpeg-tts")]
    VndDirectvMpeg,
    #[doc = "video/vnd.dlna.mpeg-tts"]
    #[serde(rename = "video/vnd.dlna.mpeg-tts")]
    VndDirectvMpegTts,
    #[doc = "video/vnd.dvb.file"]
    #[serde(rename = "video/vnd.dvb.file")]
    #[serde(alias = "dvb")]
    VndDlnaMpegTts,
    #[doc = "video/vnd.fvt"]
    #[serde(rename = "video/vnd.fvt")]
    #[serde(alias = "fvt")]
    VndDvbFile,
    #[doc = "video/vnd.hns.video"]
    #[serde(rename = "video/vnd.hns.video")]
    VndFvt,
    #[doc = "video/vnd.iptvforum.1dparityfec-1010"]
    #[serde(rename = "video/vnd.iptvforum.1dparityfec-1010")]
    VndHnsVideo,
    #[doc = "video/vnd.iptvforum.1dparityfec-2005"]
    #[serde(rename = "video/vnd.iptvforum.1dparityfec-2005")]
    VndIptvforum1Dparityfec1010,
    #[doc = "video/vnd.iptvforum.2dparityfec-1010"]
    #[serde(rename = "video/vnd.iptvforum.2dparityfec-1010")]
    VndIptvforum1Dparityfec2005,
    #[doc = "video/vnd.iptvforum.2dparityfec-2005"]
    #[serde(rename = "video/vnd.iptvforum.2dparityfec-2005")]
    VndIptvforum2Dparityfec1010,
    #[doc = "video/vnd.iptvforum.ttsavc"]
    #[serde(rename = "video/vnd.iptvforum.ttsavc")]
    VndIptvforum2Dparityfec2005,
    #[doc = "video/vnd.iptvforum.ttsmpeg2"]
    #[serde(rename = "video/vnd.iptvforum.ttsmpeg2")]
    VndIptvforumTtsavc,
    #[doc = "video/vnd.motorola.video"]
    #[serde(rename = "video/vnd.motorola.video")]
    VndIptvforumTtsmpeg2,
    #[doc = "video/vnd.motorola.videop"]
    #[serde(rename = "video/vnd.motorola.videop")]
    VndMotorolaVideo,
    #[doc = "video/vnd.mpegurl"]
    #[serde(rename = "video/vnd.mpegurl")]
    #[serde(alias = "mxu")]
    #[serde(alias = "m4u")]
    VndMotorolaVideop,
    #[doc = "video/vnd.ms-playready.media.pyv"]
    #[serde(rename = "video/vnd.ms-playready.media.pyv")]
    #[serde(alias = "pyv")]
    VndMpegurl,
    #[doc = "video/vnd.nokia.interleaved-multimedia"]
    #[serde(rename = "video/vnd.nokia.interleaved-multimedia")]
    #[serde(alias = "nim")]
    VndMsPlayreadyMediaPyv,
    #[doc = "video/vnd.nokia.mp4vr"]
    #[serde(rename = "video/vnd.nokia.mp4vr")]
    VndNokiaInterleavedMultimedia,
    #[doc = "video/vnd.nokia.videovoip"]
    #[serde(rename = "video/vnd.nokia.videovoip")]
    VndNokiaMp4Vr,
    #[doc = "video/vnd.objectvideo"]
    #[serde(rename = "video/vnd.objectvideo")]
    VndNokiaVideovoip,
    #[doc = "video/vnd.radgamettools.bink"]
    #[serde(rename = "video/vnd.radgamettools.bink")]
    #[serde(alias = "bik")]
    #[serde(alias = "bk2")]
    VndObjectvideo,
    #[doc = "video/vnd.radgamettools.smacker"]
    #[serde(rename = "video/vnd.radgamettools.smacker")]
    #[serde(alias = "smk")]
    VndRadgamettoolsBink,
    #[doc = "video/vnd.sealed.mpeg1"]
    #[serde(rename = "video/vnd.sealed.mpeg1")]
    #[serde(alias = "smpg")]
    #[serde(alias = "s11")]
    VndRadgamettoolsSmacker,
    #[doc = "video/vnd.sealed.mpeg4"]
    #[serde(rename = "video/vnd.sealed.mpeg4")]
    #[serde(alias = "s14")]
    VndSealedMpeg1,
    #[doc = "video/vnd.sealed.swf"]
    #[serde(rename = "video/vnd.sealed.swf")]
    #[serde(alias = "sswf")]
    #[serde(alias = "ssw")]
    VndSealedMpeg4,
    #[doc = "video/vnd.sealedmedia.softseal.mov"]
    #[serde(rename = "video/vnd.sealedmedia.softseal.mov")]
    #[serde(alias = "smov")]
    #[serde(alias = "smo")]
    #[serde(alias = "s1q")]
    VndSealedSwf,
    #[doc = "video/vnd.uvvu.mp4"]
    #[serde(rename = "video/vnd.uvvu.mp4")]
    VndSealedmediaSoftsealMov,
    #[doc = "video/vnd.youtube.yt"]
    #[serde(rename = "video/vnd.youtube.yt")]
    #[serde(alias = "yt")]
    VndUvvuMp4,
    #[doc = "video/vnd.vivo"]
    #[serde(rename = "video/vnd.vivo")]
    #[serde(alias = "viv")]
    VndYoutubeYt,
    #[doc = "video/VP8"]
    #[serde(rename = "video/VP8")]
    VndVivo,
    #[doc = "video/VP9"]
    #[serde(rename = "video/VP9")]
    Vp8,
}
impl ::std::fmt::Display for Video {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Video::_1DInterleavedParityfec => write!(f, "video/1d-interleaved-parityfec")?,
            Video::_3Gpp => write!(f, "video/3gpp")?,
            Video::_3Gpp2 => write!(f, "video/3gpp2")?,
            Video::_3GppTt => write!(f, "video/3gpp-tt")?,
            Video::Av1 => write!(f, "video/AV1")?,
            Video::Bmpeg => write!(f, "video/BMPEG")?,
            Video::Bt656 => write!(f, "video/BT656")?,
            Video::CelB => write!(f, "video/CelB")?,
            Video::Dv => write!(f, "video/DV")?,
            Video::Encaprtp => write!(f, "video/encaprtp")?,
            Video::Example => write!(f, "video/example")?,
            Video::Ffv1 => write!(f, "video/FFV1")?,
            Video::Flexfec => write!(f, "video/flexfec")?,
            Video::H261 => write!(f, "video/H261")?,
            Video::H263 => write!(f, "video/H263")?,
            Video::H2631998 => write!(f, "video/H263-1998")?,
            Video::H2632000 => write!(f, "video/H263-2000")?,
            Video::H264 => write!(f, "video/H264")?,
            Video::H264Rcdo => write!(f, "video/H264-RCDO")?,
            Video::H264Svc => write!(f, "video/H264-SVC")?,
            Video::H265 => write!(f, "video/H265")?,
            Video::H266 => write!(f, "video/H266")?,
            Video::IsoSegment => write!(f, "video/iso.segment")?,
            Video::Jpeg => write!(f, "video/JPEG")?,
            Video::Jpeg2000 => write!(f, "video/jpeg2000")?,
            Video::Jxsv => write!(f, "video/jxsv")?,
            Video::Mj2 => write!(f, "video/mj2")?,
            Video::Mp1S => write!(f, "video/MP1S")?,
            Video::Mp2P => write!(f, "video/MP2P")?,
            Video::Mp2T => write!(f, "video/MP2T")?,
            Video::Mp4 => write!(f, "video/mp4")?,
            Video::Mp4VEs => write!(f, "video/MP4V-ES")?,
            Video::Mpv => write!(f, "video/MPV")?,
            Video::Mpeg => write!(f, "video/mpeg4-generic")?,
            Video::Mpeg4Generic => write!(f, "video/nv")?,
            Video::Nv => write!(f, "video/ogg")?,
            Video::Ogg => write!(f, "video/parityfec")?,
            Video::Parityfec => write!(f, "video/pointer")?,
            Video::Pointer => write!(f, "video/quicktime")?,
            Video::Quicktime => write!(f, "video/raptorfec")?,
            Video::Raptorfec => write!(f, "video/raw")?,
            Video::Raw => write!(f, "video/rtp-enc-aescm128")?,
            Video::RtpEncAescm128 => write!(f, "video/rtploopback")?,
            Video::Rtploopback => write!(f, "video/rtx")?,
            Video::Rtx => write!(f, "video/scip")?,
            Video::Scip => write!(f, "video/smpte291")?,
            Video::Smpte291 => write!(f, "video/SMPTE292M")?,
            Video::Smpte292M => write!(f, "video/ulpfec")?,
            Video::Ulpfec => write!(f, "video/vc1")?,
            Video::Vc1 => write!(f, "video/vc2")?,
            Video::Vc2 => write!(f, "video/vnd.CCTV")?,
            Video::VndCCTV => write!(f, "video/vnd.dece.hd")?,
            Video::VndDeceHd => write!(f, "video/vnd.dece.mobile")?,
            Video::VndDeceMobile => write!(f, "video/vnd.dece.mp4")?,
            Video::VndDeceMp4 => write!(f, "video/vnd.dece.pd")?,
            Video::VndDecePd => write!(f, "video/vnd.dece.sd")?,
            Video::VndDeceSd => write!(f, "video/vnd.dece.video")?,
            Video::VndDeceVideo => write!(f, "video/vnd.directv.mpeg")?,
            Video::VndDirectvMpeg => write!(f, "video/vnd.directv.mpeg-tts")?,
            Video::VndDirectvMpegTts => write!(f, "video/vnd.dlna.mpeg-tts")?,
            Video::VndDlnaMpegTts => write!(f, "video/vnd.dvb.file")?,
            Video::VndDvbFile => write!(f, "video/vnd.fvt")?,
            Video::VndFvt => write!(f, "video/vnd.hns.video")?,
            Video::VndHnsVideo => write!(f, "video/vnd.iptvforum.1dparityfec-1010")?,
            Video::VndIptvforum1Dparityfec1010 => {
                write!(f, "video/vnd.iptvforum.1dparityfec-2005")?
            }
            Video::VndIptvforum1Dparityfec2005 => {
                write!(f, "video/vnd.iptvforum.2dparityfec-1010")?
            }
            Video::VndIptvforum2Dparityfec1010 => {
                write!(f, "video/vnd.iptvforum.2dparityfec-2005")?
            }
            Video::VndIptvforum2Dparityfec2005 => write!(f, "video/vnd.iptvforum.ttsavc")?,
            Video::VndIptvforumTtsavc => write!(f, "video/vnd.iptvforum.ttsmpeg2")?,
            Video::VndIptvforumTtsmpeg2 => write!(f, "video/vnd.motorola.video")?,
            Video::VndMotorolaVideo => write!(f, "video/vnd.motorola.videop")?,
            Video::VndMotorolaVideop => write!(f, "video/vnd.mpegurl")?,
            Video::VndMpegurl => write!(f, "video/vnd.ms-playready.media.pyv")?,
            Video::VndMsPlayreadyMediaPyv => write!(f, "video/vnd.nokia.interleaved-multimedia")?,
            Video::VndNokiaInterleavedMultimedia => write!(f, "video/vnd.nokia.mp4vr")?,
            Video::VndNokiaMp4Vr => write!(f, "video/vnd.nokia.videovoip")?,
            Video::VndNokiaVideovoip => write!(f, "video/vnd.objectvideo")?,
            Video::VndObjectvideo => write!(f, "video/vnd.radgamettools.bink")?,
            Video::VndRadgamettoolsBink => write!(f, "video/vnd.radgamettools.smacker")?,
            Video::VndRadgamettoolsSmacker => write!(f, "video/vnd.sealed.mpeg1")?,
            Video::VndSealedMpeg1 => write!(f, "video/vnd.sealed.mpeg4")?,
            Video::VndSealedMpeg4 => write!(f, "video/vnd.sealed.swf")?,
            Video::VndSealedSwf => write!(f, "video/vnd.sealedmedia.softseal.mov")?,
            Video::VndSealedmediaSoftsealMov => write!(f, "video/vnd.uvvu.mp4")?,
            Video::VndUvvuMp4 => write!(f, "video/vnd.youtube.yt")?,
            Video::VndYoutubeYt => write!(f, "video/vnd.vivo")?,
            Video::VndVivo => write!(f, "video/VP8")?,
            Video::Vp8 => write!(f, "video/VP9")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Video {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "video/1d-interleaved-parityfec" => Ok(Video::_1DInterleavedParityfec),
            "video/3gpp" | "3gp" | "3gpp" => Ok(Video::_3Gpp),
            "video/3gpp2" | "3g2" | "3gpp2" => Ok(Video::_3Gpp2),
            "video/3gpp-tt" => Ok(Video::_3GppTt),
            "video/AV1" => Ok(Video::Av1),
            "video/BMPEG" => Ok(Video::Bmpeg),
            "video/BT656" => Ok(Video::Bt656),
            "video/CelB" => Ok(Video::CelB),
            "video/DV" => Ok(Video::Dv),
            "video/encaprtp" => Ok(Video::Encaprtp),
            "video/example" => Ok(Video::Example),
            "video/FFV1" => Ok(Video::Ffv1),
            "video/flexfec" => Ok(Video::Flexfec),
            "video/H261" => Ok(Video::H261),
            "video/H263" => Ok(Video::H263),
            "video/H263-1998" => Ok(Video::H2631998),
            "video/H263-2000" => Ok(Video::H2632000),
            "video/H264" => Ok(Video::H264),
            "video/H264-RCDO" => Ok(Video::H264Rcdo),
            "video/H264-SVC" => Ok(Video::H264Svc),
            "video/H265" => Ok(Video::H265),
            "video/H266" => Ok(Video::H266),
            "video/iso.segment" | "m4s" => Ok(Video::IsoSegment),
            "video/JPEG" => Ok(Video::Jpeg),
            "video/jpeg2000" => Ok(Video::Jpeg2000),
            "video/jxsv" => Ok(Video::Jxsv),
            "video/mj2" | "mj2" | "mjp2" => Ok(Video::Mj2),
            "video/MP1S" => Ok(Video::Mp1S),
            "video/MP2P" => Ok(Video::Mp2P),
            "video/MP2T" => Ok(Video::Mp2T),
            "video/mp4" | "mp4" | "mpg4" | "m4v" => Ok(Video::Mp4),
            "video/MP4V-ES" => Ok(Video::Mp4VEs),
            "video/MPV" => Ok(Video::Mpv),
            "video/mpeg4-generic" => Ok(Video::Mpeg),
            "video/nv" => Ok(Video::Mpeg4Generic),
            "video/ogg" | "ogv" => Ok(Video::Nv),
            "video/parityfec" => Ok(Video::Ogg),
            "video/pointer" => Ok(Video::Parityfec),
            "video/quicktime" | "mov" | "qt" => Ok(Video::Pointer),
            "video/raptorfec" => Ok(Video::Quicktime),
            "video/raw" => Ok(Video::Raptorfec),
            "video/rtp-enc-aescm128" => Ok(Video::Raw),
            "video/rtploopback" => Ok(Video::RtpEncAescm128),
            "video/rtx" => Ok(Video::Rtploopback),
            "video/scip" => Ok(Video::Rtx),
            "video/smpte291" => Ok(Video::Scip),
            "video/SMPTE292M" => Ok(Video::Smpte291),
            "video/ulpfec" => Ok(Video::Smpte292M),
            "video/vc1" => Ok(Video::Ulpfec),
            "video/vc2" => Ok(Video::Vc1),
            "video/vnd.CCTV" => Ok(Video::Vc2),
            "video/vnd.dece.hd" | "uvh" | "uvvh" => Ok(Video::VndCCTV),
            "video/vnd.dece.mobile" | "uvm" | "uvvm" => Ok(Video::VndDeceHd),
            "video/vnd.dece.mp4" | "uvu" | "uvvu" => Ok(Video::VndDeceMobile),
            "video/vnd.dece.pd" | "uvp" | "uvvp" => Ok(Video::VndDeceMp4),
            "video/vnd.dece.sd" | "uvs" | "uvvs" => Ok(Video::VndDecePd),
            "video/vnd.dece.video" | "uvv" | "uvvv" => Ok(Video::VndDeceSd),
            "video/vnd.directv.mpeg" => Ok(Video::VndDeceVideo),
            "video/vnd.directv.mpeg-tts" => Ok(Video::VndDirectvMpeg),
            "video/vnd.dlna.mpeg-tts" => Ok(Video::VndDirectvMpegTts),
            "video/vnd.dvb.file" | "dvb" => Ok(Video::VndDlnaMpegTts),
            "video/vnd.fvt" | "fvt" => Ok(Video::VndDvbFile),
            "video/vnd.hns.video" => Ok(Video::VndFvt),
            "video/vnd.iptvforum.1dparityfec-1010" => Ok(Video::VndHnsVideo),
            "video/vnd.iptvforum.1dparityfec-2005" => Ok(Video::VndIptvforum1Dparityfec1010),
            "video/vnd.iptvforum.2dparityfec-1010" => Ok(Video::VndIptvforum1Dparityfec2005),
            "video/vnd.iptvforum.2dparityfec-2005" => Ok(Video::VndIptvforum2Dparityfec1010),
            "video/vnd.iptvforum.ttsavc" => Ok(Video::VndIptvforum2Dparityfec2005),
            "video/vnd.iptvforum.ttsmpeg2" => Ok(Video::VndIptvforumTtsavc),
            "video/vnd.motorola.video" => Ok(Video::VndIptvforumTtsmpeg2),
            "video/vnd.motorola.videop" => Ok(Video::VndMotorolaVideo),
            "video/vnd.mpegurl" | "mxu" | "m4u" => Ok(Video::VndMotorolaVideop),
            "video/vnd.ms-playready.media.pyv" | "pyv" => Ok(Video::VndMpegurl),
            "video/vnd.nokia.interleaved-multimedia" | "nim" => Ok(Video::VndMsPlayreadyMediaPyv),
            "video/vnd.nokia.mp4vr" => Ok(Video::VndNokiaInterleavedMultimedia),
            "video/vnd.nokia.videovoip" => Ok(Video::VndNokiaMp4Vr),
            "video/vnd.objectvideo" => Ok(Video::VndNokiaVideovoip),
            "video/vnd.radgamettools.bink" | "bik" | "bk2" => Ok(Video::VndObjectvideo),
            "video/vnd.radgamettools.smacker" | "smk" => Ok(Video::VndRadgamettoolsBink),
            "video/vnd.sealed.mpeg1" | "smpg" | "s11" => Ok(Video::VndRadgamettoolsSmacker),
            "video/vnd.sealed.mpeg4" | "s14" => Ok(Video::VndSealedMpeg1),
            "video/vnd.sealed.swf" | "sswf" | "ssw" => Ok(Video::VndSealedMpeg4),
            "video/vnd.sealedmedia.softseal.mov" | "smov" | "smo" | "s1q" => {
                Ok(Video::VndSealedSwf)
            }
            "video/vnd.uvvu.mp4" => Ok(Video::VndSealedmediaSoftsealMov),
            "video/vnd.youtube.yt" | "yt" => Ok(Video::VndUvvuMp4),
            "video/vnd.vivo" | "viv" => Ok(Video::VndYoutubeYt),
            "video/VP8" => Ok(Video::VndVivo),
            "video/VP9" => Ok(Video::Vp8),
            _ => Err(()),
        }
    }
}
