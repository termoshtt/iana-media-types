#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Video {
    #[doc = "video/1d-interleaved-parityfec"]
    _1DInterleavedParityfec,
    #[doc = "video/3gpp"]
    _3Gpp,
    #[doc = "video/3gpp2"]
    _3Gpp2,
    #[doc = "video/3gpp-tt"]
    _3GppTt,
    #[doc = "video/AV1"]
    Av1,
    #[doc = "video/BMPEG"]
    Bmpeg,
    #[doc = "video/BT656"]
    Bt656,
    #[doc = "video/CelB"]
    CelB,
    #[doc = "video/DV"]
    Dv,
    #[doc = "video/encaprtp"]
    Encaprtp,
    #[doc = "video/example"]
    Example,
    #[doc = "video/FFV1"]
    Ffv1,
    #[doc = "video/flexfec"]
    Flexfec,
    #[doc = "video/H261"]
    H261,
    #[doc = "video/H263"]
    H263,
    #[doc = "video/H263-1998"]
    H2631998,
    #[doc = "video/H263-2000"]
    H2632000,
    #[doc = "video/H264"]
    H264,
    #[doc = "video/H264-RCDO"]
    H264Rcdo,
    #[doc = "video/H264-SVC"]
    H264Svc,
    #[doc = "video/H265"]
    H265,
    #[doc = "video/H266"]
    H266,
    #[doc = "video/iso.segment"]
    IsoSegment,
    #[doc = "video/JPEG"]
    Jpeg,
    #[doc = "video/jpeg2000"]
    Jpeg2000,
    #[doc = "video/jxsv"]
    Jxsv,
    #[doc = "video/mj2"]
    Mj2,
    #[doc = "video/MP1S"]
    Mp1S,
    #[doc = "video/MP2P"]
    Mp2P,
    #[doc = "video/MP2T"]
    Mp2T,
    #[doc = "video/mp4"]
    Mp4,
    #[doc = "video/MP4V-ES"]
    Mp4VEs,
    #[doc = "video/MPV"]
    Mpv,
    #[doc = "video/mpeg4-generic"]
    Mpeg4Generic,
    #[doc = "video/nv"]
    Nv,
    #[doc = "video/ogg"]
    Ogg,
    #[doc = "video/parityfec"]
    Parityfec,
    #[doc = "video/pointer"]
    Pointer,
    #[doc = "video/quicktime"]
    Quicktime,
    #[doc = "video/raptorfec"]
    Raptorfec,
    #[doc = "video/raw"]
    Raw,
    #[doc = "video/rtp-enc-aescm128"]
    RtpEncAescm128,
    #[doc = "video/rtploopback"]
    Rtploopback,
    #[doc = "video/rtx"]
    Rtx,
    #[doc = "video/scip"]
    Scip,
    #[doc = "video/smpte291"]
    Smpte291,
    #[doc = "video/SMPTE292M"]
    Smpte292M,
    #[doc = "video/ulpfec"]
    Ulpfec,
    #[doc = "video/vc1"]
    Vc1,
    #[doc = "video/vc2"]
    Vc2,
    #[doc = "video/vnd.CCTV"]
    VndCCTV,
    #[doc = "video/vnd.dece.hd"]
    VndDeceHd,
    #[doc = "video/vnd.dece.mobile"]
    VndDeceMobile,
    #[doc = "video/vnd.dece.mp4"]
    VndDeceMp4,
    #[doc = "video/vnd.dece.pd"]
    VndDecePd,
    #[doc = "video/vnd.dece.sd"]
    VndDeceSd,
    #[doc = "video/vnd.dece.video"]
    VndDeceVideo,
    #[doc = "video/vnd.directv.mpeg"]
    VndDirectvMpeg,
    #[doc = "video/vnd.directv.mpeg-tts"]
    VndDirectvMpegTts,
    #[doc = "video/vnd.dlna.mpeg-tts"]
    VndDlnaMpegTts,
    #[doc = "video/vnd.dvb.file"]
    VndDvbFile,
    #[doc = "video/vnd.fvt"]
    VndFvt,
    #[doc = "video/vnd.hns.video"]
    VndHnsVideo,
    #[doc = "video/vnd.iptvforum.1dparityfec-1010"]
    VndIptvforum1Dparityfec1010,
    #[doc = "video/vnd.iptvforum.1dparityfec-2005"]
    VndIptvforum1Dparityfec2005,
    #[doc = "video/vnd.iptvforum.2dparityfec-1010"]
    VndIptvforum2Dparityfec1010,
    #[doc = "video/vnd.iptvforum.2dparityfec-2005"]
    VndIptvforum2Dparityfec2005,
    #[doc = "video/vnd.iptvforum.ttsavc"]
    VndIptvforumTtsavc,
    #[doc = "video/vnd.iptvforum.ttsmpeg2"]
    VndIptvforumTtsmpeg2,
    #[doc = "video/vnd.motorola.video"]
    VndMotorolaVideo,
    #[doc = "video/vnd.motorola.videop"]
    VndMotorolaVideop,
    #[doc = "video/vnd.mpegurl"]
    VndMpegurl,
    #[doc = "video/vnd.ms-playready.media.pyv"]
    VndMsPlayreadyMediaPyv,
    #[doc = "video/vnd.nokia.interleaved-multimedia"]
    VndNokiaInterleavedMultimedia,
    #[doc = "video/vnd.nokia.mp4vr"]
    VndNokiaMp4Vr,
    #[doc = "video/vnd.nokia.videovoip"]
    VndNokiaVideovoip,
    #[doc = "video/vnd.objectvideo"]
    VndObjectvideo,
    #[doc = "video/vnd.radgamettools.bink"]
    VndRadgamettoolsBink,
    #[doc = "video/vnd.radgamettools.smacker"]
    VndRadgamettoolsSmacker,
    #[doc = "video/vnd.sealed.mpeg1"]
    VndSealedMpeg1,
    #[doc = "video/vnd.sealed.mpeg4"]
    VndSealedMpeg4,
    #[doc = "video/vnd.sealed.swf"]
    VndSealedSwf,
    #[doc = "video/vnd.sealedmedia.softseal.mov"]
    VndSealedmediaSoftsealMov,
    #[doc = "video/vnd.uvvu.mp4"]
    VndUvvuMp4,
    #[doc = "video/vnd.youtube.yt"]
    VndYoutubeYt,
    #[doc = "video/vnd.vivo"]
    VndVivo,
    #[doc = "video/VP8"]
    Vp8,
    #[doc = "video/VP9"]
    Vp9,
    Other(String),
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
            Video::Mpeg4Generic => write!(f, "video/mpeg4-generic")?,
            Video::Nv => write!(f, "video/nv")?,
            Video::Ogg => write!(f, "video/ogg")?,
            Video::Parityfec => write!(f, "video/parityfec")?,
            Video::Pointer => write!(f, "video/pointer")?,
            Video::Quicktime => write!(f, "video/quicktime")?,
            Video::Raptorfec => write!(f, "video/raptorfec")?,
            Video::Raw => write!(f, "video/raw")?,
            Video::RtpEncAescm128 => write!(f, "video/rtp-enc-aescm128")?,
            Video::Rtploopback => write!(f, "video/rtploopback")?,
            Video::Rtx => write!(f, "video/rtx")?,
            Video::Scip => write!(f, "video/scip")?,
            Video::Smpte291 => write!(f, "video/smpte291")?,
            Video::Smpte292M => write!(f, "video/SMPTE292M")?,
            Video::Ulpfec => write!(f, "video/ulpfec")?,
            Video::Vc1 => write!(f, "video/vc1")?,
            Video::Vc2 => write!(f, "video/vc2")?,
            Video::VndCCTV => write!(f, "video/vnd.CCTV")?,
            Video::VndDeceHd => write!(f, "video/vnd.dece.hd")?,
            Video::VndDeceMobile => write!(f, "video/vnd.dece.mobile")?,
            Video::VndDeceMp4 => write!(f, "video/vnd.dece.mp4")?,
            Video::VndDecePd => write!(f, "video/vnd.dece.pd")?,
            Video::VndDeceSd => write!(f, "video/vnd.dece.sd")?,
            Video::VndDeceVideo => write!(f, "video/vnd.dece.video")?,
            Video::VndDirectvMpeg => write!(f, "video/vnd.directv.mpeg")?,
            Video::VndDirectvMpegTts => write!(f, "video/vnd.directv.mpeg-tts")?,
            Video::VndDlnaMpegTts => write!(f, "video/vnd.dlna.mpeg-tts")?,
            Video::VndDvbFile => write!(f, "video/vnd.dvb.file")?,
            Video::VndFvt => write!(f, "video/vnd.fvt")?,
            Video::VndHnsVideo => write!(f, "video/vnd.hns.video")?,
            Video::VndIptvforum1Dparityfec1010 => {
                write!(f, "video/vnd.iptvforum.1dparityfec-1010")?
            }
            Video::VndIptvforum1Dparityfec2005 => {
                write!(f, "video/vnd.iptvforum.1dparityfec-2005")?
            }
            Video::VndIptvforum2Dparityfec1010 => {
                write!(f, "video/vnd.iptvforum.2dparityfec-1010")?
            }
            Video::VndIptvforum2Dparityfec2005 => {
                write!(f, "video/vnd.iptvforum.2dparityfec-2005")?
            }
            Video::VndIptvforumTtsavc => write!(f, "video/vnd.iptvforum.ttsavc")?,
            Video::VndIptvforumTtsmpeg2 => write!(f, "video/vnd.iptvforum.ttsmpeg2")?,
            Video::VndMotorolaVideo => write!(f, "video/vnd.motorola.video")?,
            Video::VndMotorolaVideop => write!(f, "video/vnd.motorola.videop")?,
            Video::VndMpegurl => write!(f, "video/vnd.mpegurl")?,
            Video::VndMsPlayreadyMediaPyv => write!(f, "video/vnd.ms-playready.media.pyv")?,
            Video::VndNokiaInterleavedMultimedia => {
                write!(f, "video/vnd.nokia.interleaved-multimedia")?
            }
            Video::VndNokiaMp4Vr => write!(f, "video/vnd.nokia.mp4vr")?,
            Video::VndNokiaVideovoip => write!(f, "video/vnd.nokia.videovoip")?,
            Video::VndObjectvideo => write!(f, "video/vnd.objectvideo")?,
            Video::VndRadgamettoolsBink => write!(f, "video/vnd.radgamettools.bink")?,
            Video::VndRadgamettoolsSmacker => write!(f, "video/vnd.radgamettools.smacker")?,
            Video::VndSealedMpeg1 => write!(f, "video/vnd.sealed.mpeg1")?,
            Video::VndSealedMpeg4 => write!(f, "video/vnd.sealed.mpeg4")?,
            Video::VndSealedSwf => write!(f, "video/vnd.sealed.swf")?,
            Video::VndSealedmediaSoftsealMov => write!(f, "video/vnd.sealedmedia.softseal.mov")?,
            Video::VndUvvuMp4 => write!(f, "video/vnd.uvvu.mp4")?,
            Video::VndYoutubeYt => write!(f, "video/vnd.youtube.yt")?,
            Video::VndVivo => write!(f, "video/vnd.vivo")?,
            Video::Vp8 => write!(f, "video/VP8")?,
            Video::Vp9 => write!(f, "video/VP9")?,
            Video::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Video {
    fn from(input: &str) -> Self {
        match input {
            "video/1d-interleaved-parityfec" => Video::_1DInterleavedParityfec,
            "video/3gpp" => Video::_3Gpp,
            "video/3gpp2" => Video::_3Gpp2,
            "video/3gpp-tt" => Video::_3GppTt,
            "video/AV1" => Video::Av1,
            "video/BMPEG" => Video::Bmpeg,
            "video/BT656" => Video::Bt656,
            "video/CelB" => Video::CelB,
            "video/DV" => Video::Dv,
            "video/encaprtp" => Video::Encaprtp,
            "video/example" => Video::Example,
            "video/FFV1" => Video::Ffv1,
            "video/flexfec" => Video::Flexfec,
            "video/H261" => Video::H261,
            "video/H263" => Video::H263,
            "video/H263-1998" => Video::H2631998,
            "video/H263-2000" => Video::H2632000,
            "video/H264" => Video::H264,
            "video/H264-RCDO" => Video::H264Rcdo,
            "video/H264-SVC" => Video::H264Svc,
            "video/H265" => Video::H265,
            "video/H266" => Video::H266,
            "video/iso.segment" => Video::IsoSegment,
            "video/JPEG" => Video::Jpeg,
            "video/jpeg2000" => Video::Jpeg2000,
            "video/jxsv" => Video::Jxsv,
            "video/mj2" => Video::Mj2,
            "video/MP1S" => Video::Mp1S,
            "video/MP2P" => Video::Mp2P,
            "video/MP2T" => Video::Mp2T,
            "video/mp4" => Video::Mp4,
            "video/MP4V-ES" => Video::Mp4VEs,
            "video/MPV" => Video::Mpv,
            "video/mpeg4-generic" => Video::Mpeg4Generic,
            "video/nv" => Video::Nv,
            "video/ogg" => Video::Ogg,
            "video/parityfec" => Video::Parityfec,
            "video/pointer" => Video::Pointer,
            "video/quicktime" => Video::Quicktime,
            "video/raptorfec" => Video::Raptorfec,
            "video/raw" => Video::Raw,
            "video/rtp-enc-aescm128" => Video::RtpEncAescm128,
            "video/rtploopback" => Video::Rtploopback,
            "video/rtx" => Video::Rtx,
            "video/scip" => Video::Scip,
            "video/smpte291" => Video::Smpte291,
            "video/SMPTE292M" => Video::Smpte292M,
            "video/ulpfec" => Video::Ulpfec,
            "video/vc1" => Video::Vc1,
            "video/vc2" => Video::Vc2,
            "video/vnd.CCTV" => Video::VndCCTV,
            "video/vnd.dece.hd" => Video::VndDeceHd,
            "video/vnd.dece.mobile" => Video::VndDeceMobile,
            "video/vnd.dece.mp4" => Video::VndDeceMp4,
            "video/vnd.dece.pd" => Video::VndDecePd,
            "video/vnd.dece.sd" => Video::VndDeceSd,
            "video/vnd.dece.video" => Video::VndDeceVideo,
            "video/vnd.directv.mpeg" => Video::VndDirectvMpeg,
            "video/vnd.directv.mpeg-tts" => Video::VndDirectvMpegTts,
            "video/vnd.dlna.mpeg-tts" => Video::VndDlnaMpegTts,
            "video/vnd.dvb.file" => Video::VndDvbFile,
            "video/vnd.fvt" => Video::VndFvt,
            "video/vnd.hns.video" => Video::VndHnsVideo,
            "video/vnd.iptvforum.1dparityfec-1010" => Video::VndIptvforum1Dparityfec1010,
            "video/vnd.iptvforum.1dparityfec-2005" => Video::VndIptvforum1Dparityfec2005,
            "video/vnd.iptvforum.2dparityfec-1010" => Video::VndIptvforum2Dparityfec1010,
            "video/vnd.iptvforum.2dparityfec-2005" => Video::VndIptvforum2Dparityfec2005,
            "video/vnd.iptvforum.ttsavc" => Video::VndIptvforumTtsavc,
            "video/vnd.iptvforum.ttsmpeg2" => Video::VndIptvforumTtsmpeg2,
            "video/vnd.motorola.video" => Video::VndMotorolaVideo,
            "video/vnd.motorola.videop" => Video::VndMotorolaVideop,
            "video/vnd.mpegurl" => Video::VndMpegurl,
            "video/vnd.ms-playready.media.pyv" => Video::VndMsPlayreadyMediaPyv,
            "video/vnd.nokia.interleaved-multimedia" => Video::VndNokiaInterleavedMultimedia,
            "video/vnd.nokia.mp4vr" => Video::VndNokiaMp4Vr,
            "video/vnd.nokia.videovoip" => Video::VndNokiaVideovoip,
            "video/vnd.objectvideo" => Video::VndObjectvideo,
            "video/vnd.radgamettools.bink" => Video::VndRadgamettoolsBink,
            "video/vnd.radgamettools.smacker" => Video::VndRadgamettoolsSmacker,
            "video/vnd.sealed.mpeg1" => Video::VndSealedMpeg1,
            "video/vnd.sealed.mpeg4" => Video::VndSealedMpeg4,
            "video/vnd.sealed.swf" => Video::VndSealedSwf,
            "video/vnd.sealedmedia.softseal.mov" => Video::VndSealedmediaSoftsealMov,
            "video/vnd.uvvu.mp4" => Video::VndUvvuMp4,
            "video/vnd.youtube.yt" => Video::VndYoutubeYt,
            "video/vnd.vivo" => Video::VndVivo,
            "video/VP8" => Video::Vp8,
            "video/VP9" => Video::Vp9,
            _ => Video::Other(input.to_string()),
        }
    }
}
