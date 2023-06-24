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
pub enum Application {
    #[doc = "application/1d-interleaved-parityfec"]
    #[serde(rename = "application/1d-interleaved-parityfec")]
    _1DInterleavedParityfec,
    #[doc = "application/3gpdash-qoe-report+xml"]
    #[serde(rename = "application/3gpdash-qoe-report+xml")]
    _3GpdashQoeReportXml,
    #[doc = "application/3gppHal+json"]
    #[serde(rename = "application/3gppHal+json")]
    _3GppHalJson,
    #[doc = "application/3gppHalForms+json"]
    #[serde(rename = "application/3gppHalForms+json")]
    _3GppHalFormsJson,
    #[doc = "application/3gpp-ims+xml"]
    #[serde(rename = "application/3gpp-ims+xml")]
    _3GppImsXml,
    #[doc = "application/A2L"]
    #[serde(rename = "application/A2L")]
    #[serde(alias = "a2l")]
    A2L,
    #[doc = "application/ace+cbor"]
    #[serde(rename = "application/ace+cbor")]
    AceCbor,
    #[doc = "application/ace+json"]
    #[serde(rename = "application/ace+json")]
    AceJson,
    #[doc = "application/activemessage"]
    #[serde(rename = "application/activemessage")]
    Activemessage,
    #[doc = "application/activity+json"]
    #[serde(rename = "application/activity+json")]
    ActivityJson,
    #[doc = "application/aif+cbor"]
    #[serde(rename = "application/aif+cbor")]
    AifCbor,
    #[doc = "application/aif+json"]
    #[serde(rename = "application/aif+json")]
    AifJson,
    #[doc = "application/alto-cdni+json"]
    #[serde(rename = "application/alto-cdni+json")]
    AltoCdniJson,
    #[doc = "application/alto-cdnifilter+json"]
    #[serde(rename = "application/alto-cdnifilter+json")]
    AltoCdnifilterJson,
    #[doc = "application/alto-costmap+json"]
    #[serde(rename = "application/alto-costmap+json")]
    AltoCostmapJson,
    #[doc = "application/alto-costmapfilter+json"]
    #[serde(rename = "application/alto-costmapfilter+json")]
    AltoCostmapfilterJson,
    #[doc = "application/alto-directory+json"]
    #[serde(rename = "application/alto-directory+json")]
    AltoDirectoryJson,
    #[doc = "application/alto-endpointprop+json"]
    #[serde(rename = "application/alto-endpointprop+json")]
    AltoEndpointpropJson,
    #[doc = "application/alto-endpointpropparams+json"]
    #[serde(rename = "application/alto-endpointpropparams+json")]
    AltoEndpointpropparamsJson,
    #[doc = "application/alto-endpointcost+json"]
    #[serde(rename = "application/alto-endpointcost+json")]
    AltoEndpointcostJson,
    #[doc = "application/alto-endpointcostparams+json"]
    #[serde(rename = "application/alto-endpointcostparams+json")]
    AltoEndpointcostparamsJson,
    #[doc = "application/alto-error+json"]
    #[serde(rename = "application/alto-error+json")]
    AltoErrorJson,
    #[doc = "application/alto-networkmapfilter+json"]
    #[serde(rename = "application/alto-networkmapfilter+json")]
    AltoNetworkmapfilterJson,
    #[doc = "application/alto-networkmap+json"]
    #[serde(rename = "application/alto-networkmap+json")]
    AltoNetworkmapJson,
    #[doc = "application/alto-propmap+json"]
    #[serde(rename = "application/alto-propmap+json")]
    AltoPropmapJson,
    #[doc = "application/alto-propmapparams+json"]
    #[serde(rename = "application/alto-propmapparams+json")]
    AltoPropmapparamsJson,
    #[doc = "application/alto-updatestreamcontrol+json"]
    #[serde(rename = "application/alto-updatestreamcontrol+json")]
    AltoUpdatestreamcontrolJson,
    #[doc = "application/alto-updatestreamparams+json"]
    #[serde(rename = "application/alto-updatestreamparams+json")]
    AltoUpdatestreamparamsJson,
    #[doc = "application/AML"]
    #[serde(rename = "application/AML")]
    #[serde(alias = "aml")]
    Aml,
    #[doc = "application/andrew-inset"]
    #[serde(rename = "application/andrew-inset")]
    #[serde(alias = "ez")]
    AndrewInset,
    #[doc = "application/applefile"]
    #[serde(rename = "application/applefile")]
    Applefile,
    #[doc = "application/at+jwt"]
    #[serde(rename = "application/at+jwt")]
    AtJwt,
    #[doc = "application/ATF"]
    #[serde(rename = "application/ATF")]
    #[serde(alias = "atf")]
    Atf,
    #[doc = "application/ATFX"]
    #[serde(rename = "application/ATFX")]
    #[serde(alias = "atfx")]
    Atfx,
    #[doc = "application/atom+xml"]
    #[serde(rename = "application/atom+xml")]
    #[serde(alias = "atom")]
    AtomXml,
    #[doc = "application/atomcat+xml"]
    #[serde(rename = "application/atomcat+xml")]
    #[serde(alias = "atomcat")]
    AtomcatXml,
    #[doc = "application/atomdeleted+xml"]
    #[serde(rename = "application/atomdeleted+xml")]
    #[serde(alias = "atomdeleted")]
    AtomdeletedXml,
    #[doc = "application/atomicmail"]
    #[serde(rename = "application/atomicmail")]
    Atomicmail,
    #[doc = "application/atomsvc+xml"]
    #[serde(rename = "application/atomsvc+xml")]
    #[serde(alias = "atomsvc")]
    AtomsvcXml,
    #[doc = "application/atsc-dwd+xml"]
    #[serde(rename = "application/atsc-dwd+xml")]
    #[serde(alias = "dwd")]
    AtscDwdXml,
    #[doc = "application/atsc-dynamic-event-message"]
    #[serde(rename = "application/atsc-dynamic-event-message")]
    AtscDynamicEventMessage,
    #[doc = "application/atsc-held+xml"]
    #[serde(rename = "application/atsc-held+xml")]
    #[serde(alias = "held")]
    AtscHeldXml,
    #[doc = "application/atsc-rdt+json"]
    #[serde(rename = "application/atsc-rdt+json")]
    AtscRdtJson,
    #[doc = "application/atsc-rsat+xml"]
    #[serde(rename = "application/atsc-rsat+xml")]
    #[serde(alias = "rsat")]
    AtscRsatXml,
    #[doc = "application/ATXML"]
    #[serde(rename = "application/ATXML")]
    #[serde(alias = "atxml")]
    Atxml,
    #[doc = "application/auth-policy+xml"]
    #[serde(rename = "application/auth-policy+xml")]
    #[serde(alias = "apxml")]
    AuthPolicyXml,
    #[doc = "application/automationml-aml+xml"]
    #[serde(rename = "application/automationml-aml+xml")]
    AutomationmlAmlXml,
    #[doc = "application/automationml-amlx+zip"]
    #[serde(rename = "application/automationml-amlx+zip")]
    #[serde(alias = "amlx")]
    AutomationmlAmlxZip,
    #[doc = "application/bacnet-xdd+zip"]
    #[serde(rename = "application/bacnet-xdd+zip")]
    #[serde(alias = "xdd")]
    BacnetXddZip,
    #[doc = "application/batch-SMTP"]
    #[serde(rename = "application/batch-SMTP")]
    BatchSMTP,
    #[doc = "application/beep+xml"]
    #[serde(rename = "application/beep+xml")]
    BeepXml,
    #[doc = "application/calendar+json"]
    #[serde(rename = "application/calendar+json")]
    CalendarJson,
    #[doc = "application/calendar+xml"]
    #[serde(rename = "application/calendar+xml")]
    #[serde(alias = "xcs")]
    CalendarXml,
    #[doc = "application/call-completion"]
    #[serde(rename = "application/call-completion")]
    CallCompletion,
    #[doc = "application/CALS-1840"]
    #[serde(rename = "application/CALS-1840")]
    Cals1840,
    #[doc = "application/captive+json"]
    #[serde(rename = "application/captive+json")]
    CaptiveJson,
    #[doc = "application/cbor"]
    #[serde(rename = "application/cbor")]
    #[serde(alias = "cbor")]
    Cbor,
    #[doc = "application/cbor-seq"]
    #[serde(rename = "application/cbor-seq")]
    CborSeq,
    #[doc = "application/cccex"]
    #[serde(rename = "application/cccex")]
    #[serde(alias = "c3ex")]
    Cccex,
    #[doc = "application/ccmp+xml"]
    #[serde(rename = "application/ccmp+xml")]
    #[serde(alias = "ccmp")]
    CcmpXml,
    #[doc = "application/ccxml+xml"]
    #[serde(rename = "application/ccxml+xml")]
    #[serde(alias = "ccxml")]
    CcxmlXml,
    #[doc = "application/cda+xml"]
    #[serde(rename = "application/cda+xml")]
    CdaXml,
    #[doc = "application/CDFX+XML"]
    #[serde(rename = "application/CDFX+XML")]
    #[serde(alias = "cdfx")]
    CdfxXml,
    #[doc = "application/cdmi-capability"]
    #[serde(rename = "application/cdmi-capability")]
    #[serde(alias = "cdmia")]
    CdmiCapability,
    #[doc = "application/cdmi-container"]
    #[serde(rename = "application/cdmi-container")]
    #[serde(alias = "cdmic")]
    CdmiContainer,
    #[doc = "application/cdmi-domain"]
    #[serde(rename = "application/cdmi-domain")]
    #[serde(alias = "cdmid")]
    CdmiDomain,
    #[doc = "application/cdmi-object"]
    #[serde(rename = "application/cdmi-object")]
    #[serde(alias = "cdmio")]
    CdmiObject,
    #[doc = "application/cdmi-queue"]
    #[serde(rename = "application/cdmi-queue")]
    #[serde(alias = "cdmiq")]
    CdmiQueue,
    #[doc = "application/cdni"]
    #[serde(rename = "application/cdni")]
    Cdni,
    #[doc = "application/CEA"]
    #[serde(rename = "application/CEA")]
    #[serde(alias = "cea")]
    Cea,
    #[doc = "application/cea-2018+xml"]
    #[serde(rename = "application/cea-2018+xml")]
    Cea2018Xml,
    #[doc = "application/cellml+xml"]
    #[serde(rename = "application/cellml+xml")]
    #[serde(alias = "cellml")]
    #[serde(alias = "cml")]
    CellmlXml,
    #[doc = "application/cfw"]
    #[serde(rename = "application/cfw")]
    Cfw,
    #[doc = "application/city+json"]
    #[serde(rename = "application/city+json")]
    CityJson,
    #[doc = "application/clr"]
    #[serde(rename = "application/clr")]
    #[serde(alias = "1clr")]
    Clr,
    #[doc = "application/clue_info+xml"]
    #[serde(rename = "application/clue_info+xml")]
    #[serde(alias = "clue")]
    ClueInfoXml,
    #[doc = "application/clue+xml"]
    #[serde(rename = "application/clue+xml")]
    ClueXml,
    #[doc = "application/cms"]
    #[serde(rename = "application/cms")]
    #[serde(alias = "cmsc")]
    Cms,
    #[doc = "application/cnrp+xml"]
    #[serde(rename = "application/cnrp+xml")]
    CnrpXml,
    #[doc = "application/coap-group+json"]
    #[serde(rename = "application/coap-group+json")]
    CoapGroupJson,
    #[doc = "application/coap-payload"]
    #[serde(rename = "application/coap-payload")]
    CoapPayload,
    #[doc = "application/commonground"]
    #[serde(rename = "application/commonground")]
    Commonground,
    #[doc = "application/concise-problem-details+cbor"]
    #[serde(rename = "application/concise-problem-details+cbor")]
    ConciseProblemDetailsCbor,
    #[doc = "application/conference-info+xml"]
    #[serde(rename = "application/conference-info+xml")]
    ConferenceInfoXml,
    #[doc = "application/cpl+xml"]
    #[serde(rename = "application/cpl+xml")]
    #[serde(alias = "cpl")]
    CplXml,
    #[doc = "application/cose"]
    #[serde(rename = "application/cose")]
    Cose,
    #[doc = "application/cose-key"]
    #[serde(rename = "application/cose-key")]
    CoseKey,
    #[doc = "application/cose-key-set"]
    #[serde(rename = "application/cose-key-set")]
    CoseKeySet,
    #[doc = "application/cose-x509"]
    #[serde(rename = "application/cose-x509")]
    CoseX509,
    #[doc = "application/csrattrs"]
    #[serde(rename = "application/csrattrs")]
    #[serde(alias = "csrattrs")]
    Csrattrs,
    #[doc = "application/csta+xml"]
    #[serde(rename = "application/csta+xml")]
    CstaXml,
    #[doc = "application/CSTAdata+xml"]
    #[serde(rename = "application/CSTAdata+xml")]
    CstadataXml,
    #[doc = "application/csvm+json"]
    #[serde(rename = "application/csvm+json")]
    CsvmJson,
    #[doc = "application/cwl"]
    #[serde(rename = "application/cwl")]
    #[serde(alias = "cwl")]
    Cwl,
    #[doc = "application/cwl+json"]
    #[serde(rename = "application/cwl+json")]
    #[serde(alias = "cwl.json")]
    CwlJson,
    #[doc = "application/cwt"]
    #[serde(rename = "application/cwt")]
    Cwt,
    #[doc = "application/cybercash"]
    #[serde(rename = "application/cybercash")]
    Cybercash,
    #[doc = "application/dash+xml"]
    #[serde(rename = "application/dash+xml")]
    #[serde(alias = "mpd")]
    DashXml,
    #[doc = "application/dash-patch+xml"]
    #[serde(rename = "application/dash-patch+xml")]
    DashPatchXml,
    #[doc = "application/dashdelta"]
    #[serde(rename = "application/dashdelta")]
    #[serde(alias = "mpdd")]
    Dashdelta,
    #[doc = "application/davmount+xml"]
    #[serde(rename = "application/davmount+xml")]
    #[serde(alias = "davmount")]
    DavmountXml,
    #[doc = "application/dca-rft"]
    #[serde(rename = "application/dca-rft")]
    DcaRft,
    #[doc = "application/DCD"]
    #[serde(rename = "application/DCD")]
    #[serde(alias = "dcd")]
    Dcd,
    #[doc = "application/dec-dx"]
    #[serde(rename = "application/dec-dx")]
    DecDx,
    #[doc = "application/dialog-info+xml"]
    #[serde(rename = "application/dialog-info+xml")]
    DialogInfoXml,
    #[doc = "application/dicom"]
    #[serde(rename = "application/dicom")]
    #[serde(alias = "dcm")]
    Dicom,
    #[doc = "application/dicom+json"]
    #[serde(rename = "application/dicom+json")]
    DicomJson,
    #[doc = "application/dicom+xml"]
    #[serde(rename = "application/dicom+xml")]
    DicomXml,
    #[doc = "application/DII"]
    #[serde(rename = "application/DII")]
    #[serde(alias = "dii")]
    Dii,
    #[doc = "application/DIT"]
    #[serde(rename = "application/DIT")]
    #[serde(alias = "dit")]
    Dit,
    #[doc = "application/dns"]
    #[serde(rename = "application/dns")]
    Dns,
    #[doc = "application/dns+json"]
    #[serde(rename = "application/dns+json")]
    DnsJson,
    #[doc = "application/dns-message"]
    #[serde(rename = "application/dns-message")]
    DnsMessage,
    #[doc = "application/dots+cbor"]
    #[serde(rename = "application/dots+cbor")]
    DotsCbor,
    #[doc = "application/dpop+jwt"]
    #[serde(rename = "application/dpop+jwt")]
    DpopJwt,
    #[doc = "application/dskpp+xml"]
    #[serde(rename = "application/dskpp+xml")]
    #[serde(alias = "xmls")]
    DskppXml,
    #[doc = "application/dssc+der"]
    #[serde(rename = "application/dssc+der")]
    #[serde(alias = "dssc")]
    DsscDer,
    #[doc = "application/dssc+xml"]
    #[serde(rename = "application/dssc+xml")]
    #[serde(alias = "xdssc")]
    DsscXml,
    #[doc = "application/dvcs"]
    #[serde(rename = "application/dvcs")]
    #[serde(alias = "dvc")]
    Dvcs,
    #[doc = "application/EDI-consent"]
    #[serde(rename = "application/EDI-consent")]
    EdiConsent,
    #[doc = "application/EDIFACT"]
    #[serde(rename = "application/EDIFACT")]
    Edifact,
    #[doc = "application/EDI-X12"]
    #[serde(rename = "application/EDI-X12")]
    EdiX12,
    #[doc = "application/efi"]
    #[serde(rename = "application/efi")]
    #[serde(alias = "efi")]
    Efi,
    #[doc = "application/elm+json"]
    #[serde(rename = "application/elm+json")]
    ElmJson,
    #[doc = "application/elm+xml"]
    #[serde(rename = "application/elm+xml")]
    ElmXml,
    #[doc = "application/EmergencyCallData.cap+xml"]
    #[serde(rename = "application/EmergencyCallData.cap+xml")]
    EmergencyCallDataCapXml,
    #[doc = "application/EmergencyCallData.Comment+xml"]
    #[serde(rename = "application/EmergencyCallData.Comment+xml")]
    EmergencyCallDataCommentXml,
    #[doc = "application/EmergencyCallData.Control+xml"]
    #[serde(rename = "application/EmergencyCallData.Control+xml")]
    EmergencyCallDataControlXml,
    #[doc = "application/EmergencyCallData.DeviceInfo+xml"]
    #[serde(rename = "application/EmergencyCallData.DeviceInfo+xml")]
    EmergencyCallDataDeviceInfoXml,
    #[doc = "application/EmergencyCallData.eCall.MSD"]
    #[serde(rename = "application/EmergencyCallData.eCall.MSD")]
    EmergencyCallDataECallMSD,
    #[doc = "application/EmergencyCallData.LegacyESN+json"]
    #[serde(rename = "application/EmergencyCallData.LegacyESN+json")]
    EmergencyCallDataLegacyESNJson,
    #[doc = "application/EmergencyCallData.ProviderInfo+xml"]
    #[serde(rename = "application/EmergencyCallData.ProviderInfo+xml")]
    EmergencyCallDataProviderInfoXml,
    #[doc = "application/EmergencyCallData.ServiceInfo+xml"]
    #[serde(rename = "application/EmergencyCallData.ServiceInfo+xml")]
    EmergencyCallDataServiceInfoXml,
    #[doc = "application/EmergencyCallData.SubscriberInfo+xml"]
    #[serde(rename = "application/EmergencyCallData.SubscriberInfo+xml")]
    EmergencyCallDataSubscriberInfoXml,
    #[doc = "application/EmergencyCallData.VEDS+xml"]
    #[serde(rename = "application/EmergencyCallData.VEDS+xml")]
    EmergencyCallDataVEDSXml,
    #[doc = "application/emma+xml"]
    #[serde(rename = "application/emma+xml")]
    #[serde(alias = "emma")]
    EmmaXml,
    #[doc = "application/emotionml+xml"]
    #[serde(rename = "application/emotionml+xml")]
    #[serde(alias = "emotionml")]
    EmotionmlXml,
    #[doc = "application/encaprtp"]
    #[serde(rename = "application/encaprtp")]
    Encaprtp,
    #[doc = "application/epp+xml"]
    #[serde(rename = "application/epp+xml")]
    EppXml,
    #[doc = "application/epub+zip"]
    #[serde(rename = "application/epub+zip")]
    #[serde(alias = "epub")]
    EpubZip,
    #[doc = "application/eshop"]
    #[serde(rename = "application/eshop")]
    Eshop,
    #[doc = "application/example"]
    #[serde(rename = "application/example")]
    Example,
    #[doc = "application/exi"]
    #[serde(rename = "application/exi")]
    #[serde(alias = "exi")]
    Exi,
    #[doc = "application/expect-ct-report+json"]
    #[serde(rename = "application/expect-ct-report+json")]
    ExpectCtReportJson,
    #[doc = "application/express"]
    #[serde(rename = "application/express")]
    #[serde(alias = "exp")]
    Express,
    #[doc = "application/fastinfoset"]
    #[serde(rename = "application/fastinfoset")]
    #[serde(alias = "finf")]
    Fastinfoset,
    #[doc = "application/fastsoap"]
    #[serde(rename = "application/fastsoap")]
    Fastsoap,
    #[doc = "application/fdf"]
    #[serde(rename = "application/fdf")]
    Fdf,
    #[doc = "application/fdt+xml"]
    #[serde(rename = "application/fdt+xml")]
    #[serde(alias = "fdt")]
    FdtXml,
    #[doc = "application/fhir+json"]
    #[serde(rename = "application/fhir+json")]
    FhirJson,
    #[doc = "application/fhir+xml"]
    #[serde(rename = "application/fhir+xml")]
    FhirXml,
    #[doc = "application/fits"]
    #[serde(rename = "application/fits")]
    Fits,
    #[doc = "application/flexfec"]
    #[serde(rename = "application/flexfec")]
    Flexfec,
    #[doc = "application/font-tdpfr"]
    #[serde(rename = "application/font-tdpfr")]
    #[serde(alias = "pfr")]
    FontTdpfr,
    #[doc = "application/framework-attributes+xml"]
    #[serde(rename = "application/framework-attributes+xml")]
    FrameworkAttributesXml,
    #[doc = "application/geo+json"]
    #[serde(rename = "application/geo+json")]
    #[serde(alias = "geojson")]
    GeoJson,
    #[doc = "application/geo+json-seq"]
    #[serde(rename = "application/geo+json-seq")]
    GeoJsonSeq,
    #[doc = "application/geopackage+sqlite3"]
    #[serde(rename = "application/geopackage+sqlite3")]
    #[serde(alias = "gpkg")]
    GeopackageSqlite3,
    #[doc = "application/geoxacml+xml"]
    #[serde(rename = "application/geoxacml+xml")]
    GeoxacmlXml,
    #[doc = "application/gltf-buffer"]
    #[serde(rename = "application/gltf-buffer")]
    #[serde(alias = "glbin")]
    #[serde(alias = "glbuf")]
    GltfBuffer,
    #[doc = "application/gml+xml"]
    #[serde(rename = "application/gml+xml")]
    #[serde(alias = "gml")]
    GmlXml,
    #[doc = "application/gzip"]
    #[serde(rename = "application/gzip")]
    #[serde(alias = "gz")]
    #[serde(alias = "tgz")]
    Gzip,
    #[doc = "application/H224"]
    #[serde(rename = "application/H224")]
    H224,
    #[doc = "application/held+xml"]
    #[serde(rename = "application/held+xml")]
    HeldXml,
    #[doc = "application/hl7v2+xml"]
    #[serde(rename = "application/hl7v2+xml")]
    Hl7V2Xml,
    #[doc = "application/http"]
    #[serde(rename = "application/http")]
    Http,
    #[doc = "application/hyperstudio"]
    #[serde(rename = "application/hyperstudio")]
    #[serde(alias = "stk")]
    Hyperstudio,
    #[doc = "application/ibe-key-request+xml"]
    #[serde(rename = "application/ibe-key-request+xml")]
    IbeKeyRequestXml,
    #[doc = "application/ibe-pkg-reply+xml"]
    #[serde(rename = "application/ibe-pkg-reply+xml")]
    IbePkgReplyXml,
    #[doc = "application/ibe-pp-data"]
    #[serde(rename = "application/ibe-pp-data")]
    IbePpData,
    #[doc = "application/iges"]
    #[serde(rename = "application/iges")]
    Iges,
    #[doc = "application/im-iscomposing+xml"]
    #[serde(rename = "application/im-iscomposing+xml")]
    ImIscomposingXml,
    #[doc = "application/index"]
    #[serde(rename = "application/index")]
    Index,
    #[doc = "application/index.cmd"]
    #[serde(rename = "application/index.cmd")]
    IndexCmd,
    #[doc = "application/index.obj"]
    #[serde(rename = "application/index.obj")]
    IndexObj,
    #[doc = "application/index.response"]
    #[serde(rename = "application/index.response")]
    IndexResponse,
    #[doc = "application/index.vnd"]
    #[serde(rename = "application/index.vnd")]
    IndexVnd,
    #[doc = "application/inkml+xml"]
    #[serde(rename = "application/inkml+xml")]
    #[serde(alias = "ink")]
    #[serde(alias = "inkml")]
    InkmlXml,
    #[doc = "application/IOTP"]
    #[serde(rename = "application/IOTP")]
    Iotp,
    #[doc = "application/ipfix"]
    #[serde(rename = "application/ipfix")]
    #[serde(alias = "ipfix")]
    Ipfix,
    #[doc = "application/ipp"]
    #[serde(rename = "application/ipp")]
    Ipp,
    #[doc = "application/ISUP"]
    #[serde(rename = "application/ISUP")]
    Isup,
    #[doc = "application/its+xml"]
    #[serde(rename = "application/its+xml")]
    #[serde(alias = "its")]
    ItsXml,
    #[doc = "application/java-archive"]
    #[serde(rename = "application/java-archive")]
    #[serde(alias = "jar")]
    JavaArchive,
    #[doc = "application/jf2feed+json"]
    #[serde(rename = "application/jf2feed+json")]
    Jf2FeedJson,
    #[doc = "application/jose"]
    #[serde(rename = "application/jose")]
    Jose,
    #[doc = "application/jose+json"]
    #[serde(rename = "application/jose+json")]
    JoseJson,
    #[doc = "application/jrd+json"]
    #[serde(rename = "application/jrd+json")]
    #[serde(alias = "jrd")]
    JrdJson,
    #[doc = "application/jscalendar+json"]
    #[serde(rename = "application/jscalendar+json")]
    JscalendarJson,
    #[doc = "application/json"]
    #[serde(rename = "application/json")]
    #[serde(alias = "json")]
    Json,
    #[doc = "application/json-patch+json"]
    #[serde(rename = "application/json-patch+json")]
    #[serde(alias = "json-patch")]
    JsonPatchJson,
    #[doc = "application/json-seq"]
    #[serde(rename = "application/json-seq")]
    JsonSeq,
    #[doc = "application/jwk+json"]
    #[serde(rename = "application/jwk+json")]
    JwkJson,
    #[doc = "application/jwk-set+json"]
    #[serde(rename = "application/jwk-set+json")]
    JwkSetJson,
    #[doc = "application/jwt"]
    #[serde(rename = "application/jwt")]
    Jwt,
    #[doc = "application/kpml-request+xml"]
    #[serde(rename = "application/kpml-request+xml")]
    KpmlRequestXml,
    #[doc = "application/kpml-response+xml"]
    #[serde(rename = "application/kpml-response+xml")]
    KpmlResponseXml,
    #[doc = "application/ld+json"]
    #[serde(rename = "application/ld+json")]
    #[serde(alias = "jsonld")]
    LdJson,
    #[doc = "application/lgr+xml"]
    #[serde(rename = "application/lgr+xml")]
    #[serde(alias = "lgr")]
    LgrXml,
    #[doc = "application/link-format"]
    #[serde(rename = "application/link-format")]
    #[serde(alias = "wlnk")]
    LinkFormat,
    #[doc = "application/linkset"]
    #[serde(rename = "application/linkset")]
    Linkset,
    #[doc = "application/linkset+json"]
    #[serde(rename = "application/linkset+json")]
    LinksetJson,
    #[doc = "application/load-control+xml"]
    #[serde(rename = "application/load-control+xml")]
    LoadControlXml,
    #[doc = "application/logout+jwt"]
    #[serde(rename = "application/logout+jwt")]
    LogoutJwt,
    #[doc = "application/lost+xml"]
    #[serde(rename = "application/lost+xml")]
    #[serde(alias = "lostxml")]
    LostXml,
    #[doc = "application/lostsync+xml"]
    #[serde(rename = "application/lostsync+xml")]
    #[serde(alias = "lostsyncxml")]
    LostsyncXml,
    #[doc = "application/lpf+zip"]
    #[serde(rename = "application/lpf+zip")]
    #[serde(alias = "lpf")]
    LpfZip,
    #[doc = "application/LXF"]
    #[serde(rename = "application/LXF")]
    #[serde(alias = "lxf")]
    Lxf,
    #[doc = "application/mac-binhex40"]
    #[serde(rename = "application/mac-binhex40")]
    #[serde(alias = "hqx")]
    MacBinhex40,
    #[doc = "application/macwriteii"]
    #[serde(rename = "application/macwriteii")]
    Macwriteii,
    #[doc = "application/mads+xml"]
    #[serde(rename = "application/mads+xml")]
    #[serde(alias = "mads")]
    MadsXml,
    #[doc = "application/manifest+json"]
    #[serde(rename = "application/manifest+json")]
    #[serde(alias = "webmanifest")]
    ManifestJson,
    #[doc = "application/marc"]
    #[serde(rename = "application/marc")]
    #[serde(alias = "mrc")]
    Marc,
    #[doc = "application/marcxml+xml"]
    #[serde(rename = "application/marcxml+xml")]
    #[serde(alias = "mrcx")]
    MarcxmlXml,
    #[doc = "application/mathematica"]
    #[serde(rename = "application/mathematica")]
    #[serde(alias = "nb")]
    #[serde(alias = "ma")]
    #[serde(alias = "mb")]
    Mathematica,
    #[doc = "application/mathml+xml"]
    #[serde(rename = "application/mathml+xml")]
    #[serde(alias = "mml")]
    MathmlXml,
    #[doc = "application/mathml-content+xml"]
    #[serde(rename = "application/mathml-content+xml")]
    MathmlContentXml,
    #[doc = "application/mathml-presentation+xml"]
    #[serde(rename = "application/mathml-presentation+xml")]
    MathmlPresentationXml,
    #[doc = "application/mbms-associated-procedure-description+xml"]
    #[serde(rename = "application/mbms-associated-procedure-description+xml")]
    MbmsAssociatedProcedureDescriptionXml,
    #[doc = "application/mbms-deregister+xml"]
    #[serde(rename = "application/mbms-deregister+xml")]
    MbmsDeregisterXml,
    #[doc = "application/mbms-envelope+xml"]
    #[serde(rename = "application/mbms-envelope+xml")]
    MbmsEnvelopeXml,
    #[doc = "application/mbms-msk-response+xml"]
    #[serde(rename = "application/mbms-msk-response+xml")]
    MbmsMskResponseXml,
    #[doc = "application/mbms-msk+xml"]
    #[serde(rename = "application/mbms-msk+xml")]
    MbmsMskXml,
    #[doc = "application/mbms-protection-description+xml"]
    #[serde(rename = "application/mbms-protection-description+xml")]
    MbmsProtectionDescriptionXml,
    #[doc = "application/mbms-reception-report+xml"]
    #[serde(rename = "application/mbms-reception-report+xml")]
    MbmsReceptionReportXml,
    #[doc = "application/mbms-register-response+xml"]
    #[serde(rename = "application/mbms-register-response+xml")]
    MbmsRegisterResponseXml,
    #[doc = "application/mbms-register+xml"]
    #[serde(rename = "application/mbms-register+xml")]
    MbmsRegisterXml,
    #[doc = "application/mbms-schedule+xml"]
    #[serde(rename = "application/mbms-schedule+xml")]
    MbmsScheduleXml,
    #[doc = "application/mbms-user-service-description+xml"]
    #[serde(rename = "application/mbms-user-service-description+xml")]
    MbmsUserServiceDescriptionXml,
    #[doc = "application/mbox"]
    #[serde(rename = "application/mbox")]
    #[serde(alias = "mbox")]
    Mbox,
    #[doc = "application/media_control+xml"]
    #[serde(rename = "application/media_control+xml")]
    MediaControlXml,
    #[doc = "application/media-policy-dataset+xml"]
    #[serde(rename = "application/media-policy-dataset+xml")]
    MediaPolicyDatasetXml,
    #[doc = "application/mediaservercontrol+xml"]
    #[serde(rename = "application/mediaservercontrol+xml")]
    MediaservercontrolXml,
    #[doc = "application/merge-patch+json"]
    #[serde(rename = "application/merge-patch+json")]
    MergePatchJson,
    #[doc = "application/metalink4+xml"]
    #[serde(rename = "application/metalink4+xml")]
    #[serde(alias = "meta4")]
    Metalink4Xml,
    #[doc = "application/mets+xml"]
    #[serde(rename = "application/mets+xml")]
    #[serde(alias = "mets")]
    MetsXml,
    #[doc = "application/MF4"]
    #[serde(rename = "application/MF4")]
    #[serde(alias = "mf4")]
    Mf4,
    #[doc = "application/mikey"]
    #[serde(rename = "application/mikey")]
    Mikey,
    #[doc = "application/mipc"]
    #[serde(rename = "application/mipc")]
    #[serde(alias = "h5")]
    Mipc,
    #[doc = "application/missing-blocks+cbor-seq"]
    #[serde(rename = "application/missing-blocks+cbor-seq")]
    MissingBlocksCborSeq,
    #[doc = "application/mmt-aei+xml"]
    #[serde(rename = "application/mmt-aei+xml")]
    #[serde(alias = "maei")]
    MmtAeiXml,
    #[doc = "application/mmt-usd+xml"]
    #[serde(rename = "application/mmt-usd+xml")]
    #[serde(alias = "musd")]
    MmtUsdXml,
    #[doc = "application/mods+xml"]
    #[serde(rename = "application/mods+xml")]
    #[serde(alias = "mods")]
    ModsXml,
    #[doc = "application/moss-keys"]
    #[serde(rename = "application/moss-keys")]
    MossKeys,
    #[doc = "application/moss-signature"]
    #[serde(rename = "application/moss-signature")]
    MossSignature,
    #[doc = "application/mosskey-data"]
    #[serde(rename = "application/mosskey-data")]
    MosskeyData,
    #[doc = "application/mosskey-request"]
    #[serde(rename = "application/mosskey-request")]
    MosskeyRequest,
    #[doc = "application/mp21"]
    #[serde(rename = "application/mp21")]
    #[serde(alias = "m21")]
    #[serde(alias = "mp21")]
    Mp21,
    #[doc = "application/mp4"]
    #[serde(rename = "application/mp4")]
    Mp4,
    #[doc = "application/mpeg4-generic"]
    #[serde(rename = "application/mpeg4-generic")]
    Mpeg4Generic,
    #[doc = "application/mpeg4-iod"]
    #[serde(rename = "application/mpeg4-iod")]
    Mpeg4Iod,
    #[doc = "application/mpeg4-iod-xmt"]
    #[serde(rename = "application/mpeg4-iod-xmt")]
    Mpeg4IodXmt,
    #[doc = "application/mrb-consumer+xml"]
    #[serde(rename = "application/mrb-consumer+xml")]
    MrbConsumerXml,
    #[doc = "application/mrb-publish+xml"]
    #[serde(rename = "application/mrb-publish+xml")]
    MrbPublishXml,
    #[doc = "application/msc-ivr+xml"]
    #[serde(rename = "application/msc-ivr+xml")]
    MscIvrXml,
    #[doc = "application/msc-mixer+xml"]
    #[serde(rename = "application/msc-mixer+xml")]
    MscMixerXml,
    #[doc = "application/msword"]
    #[serde(rename = "application/msword")]
    #[serde(alias = "doc")]
    Msword,
    #[doc = "application/mud+json"]
    #[serde(rename = "application/mud+json")]
    MudJson,
    #[doc = "application/multipart-core"]
    #[serde(rename = "application/multipart-core")]
    MultipartCore,
    #[doc = "application/mxf"]
    #[serde(rename = "application/mxf")]
    #[serde(alias = "mxf")]
    Mxf,
    #[doc = "application/n-quads"]
    #[serde(rename = "application/n-quads")]
    #[serde(alias = "nq")]
    NQuads,
    #[doc = "application/n-triples"]
    #[serde(rename = "application/n-triples")]
    #[serde(alias = "nt")]
    NTriples,
    #[doc = "application/nasdata"]
    #[serde(rename = "application/nasdata")]
    Nasdata,
    #[doc = "application/news-checkgroups"]
    #[serde(rename = "application/news-checkgroups")]
    NewsCheckgroups,
    #[doc = "application/news-groupinfo"]
    #[serde(rename = "application/news-groupinfo")]
    NewsGroupinfo,
    #[doc = "application/news-transmission"]
    #[serde(rename = "application/news-transmission")]
    NewsTransmission,
    #[doc = "application/nlsml+xml"]
    #[serde(rename = "application/nlsml+xml")]
    NlsmlXml,
    #[doc = "application/node"]
    #[serde(rename = "application/node")]
    Node,
    #[doc = "application/nss"]
    #[serde(rename = "application/nss")]
    Nss,
    #[doc = "application/oauth-authz-req+jwt"]
    #[serde(rename = "application/oauth-authz-req+jwt")]
    OauthAuthzReqJwt,
    #[doc = "application/oblivious-dns-message"]
    #[serde(rename = "application/oblivious-dns-message")]
    ObliviousDnsMessage,
    #[doc = "application/ocsp-request"]
    #[serde(rename = "application/ocsp-request")]
    #[serde(alias = "orq")]
    OcspRequest,
    #[doc = "application/ocsp-response"]
    #[serde(rename = "application/ocsp-response")]
    #[serde(alias = "ors")]
    OcspResponse,
    #[doc = "application/octet-stream"]
    #[serde(rename = "application/octet-stream")]
    #[serde(alias = "bin")]
    #[serde(alias = "lha")]
    #[serde(alias = "lzh")]
    #[serde(alias = "exe")]
    #[serde(alias = "class")]
    #[serde(alias = "so")]
    #[serde(alias = "dll")]
    #[serde(alias = "img")]
    #[serde(alias = "iso")]
    OctetStream,
    #[doc = "application/ODA"]
    #[serde(rename = "application/ODA")]
    #[serde(alias = "oda")]
    Oda,
    #[doc = "application/odm+xml"]
    #[serde(rename = "application/odm+xml")]
    OdmXml,
    #[doc = "application/ODX"]
    #[serde(rename = "application/ODX")]
    #[serde(alias = "odx")]
    Odx,
    #[doc = "application/oebps-package+xml"]
    #[serde(rename = "application/oebps-package+xml")]
    #[serde(alias = "opf")]
    OebpsPackageXml,
    #[doc = "application/ogg"]
    #[serde(rename = "application/ogg")]
    #[serde(alias = "ogx")]
    Ogg,
    #[doc = "application/ohttp-keys"]
    #[serde(rename = "application/ohttp-keys")]
    OhttpKeys,
    #[doc = "application/opc-nodeset+xml"]
    #[serde(rename = "application/opc-nodeset+xml")]
    OpcNodesetXml,
    #[doc = "application/oscore"]
    #[serde(rename = "application/oscore")]
    Oscore,
    #[doc = "application/oxps"]
    #[serde(rename = "application/oxps")]
    #[serde(alias = "oxps")]
    Oxps,
    #[doc = "application/p21"]
    #[serde(rename = "application/p21")]
    #[serde(alias = "p21")]
    #[serde(alias = "stp")]
    #[serde(alias = "step")]
    #[serde(alias = "stpnc")]
    #[serde(alias = "210")]
    #[serde(alias = "ifc")]
    P21,
    #[doc = "application/p21+zip"]
    #[serde(rename = "application/p21+zip")]
    #[serde(alias = "stpz")]
    P21Zip,
    #[doc = "application/p2p-overlay+xml"]
    #[serde(rename = "application/p2p-overlay+xml")]
    #[serde(alias = "relo")]
    P2POverlayXml,
    #[doc = "application/parityfec"]
    #[serde(rename = "application/parityfec")]
    Parityfec,
    #[doc = "application/passport"]
    #[serde(rename = "application/passport")]
    Passport,
    #[doc = "application/patch-ops-error+xml"]
    #[serde(rename = "application/patch-ops-error+xml")]
    PatchOpsErrorXml,
    #[doc = "application/pdf"]
    #[serde(rename = "application/pdf")]
    #[serde(alias = "pdf")]
    Pdf,
    #[doc = "application/PDX"]
    #[serde(rename = "application/PDX")]
    #[serde(alias = "pdx")]
    Pdx,
    #[doc = "application/pem-certificate-chain"]
    #[serde(rename = "application/pem-certificate-chain")]
    #[serde(alias = "pem")]
    PemCertificateChain,
    #[doc = "application/pgp-encrypted"]
    #[serde(rename = "application/pgp-encrypted")]
    #[serde(alias = "pgp")]
    PgpEncrypted,
    #[doc = "application/pgp-keys"]
    #[serde(rename = "application/pgp-keys")]
    PgpKeys,
    #[doc = "application/pgp-signature"]
    #[serde(rename = "application/pgp-signature")]
    #[serde(alias = "sig")]
    PgpSignature,
    #[doc = "application/pidf-diff+xml"]
    #[serde(rename = "application/pidf-diff+xml")]
    PidfDiffXml,
    #[doc = "application/pidf+xml"]
    #[serde(rename = "application/pidf+xml")]
    PidfXml,
    #[doc = "application/pkcs10"]
    #[serde(rename = "application/pkcs10")]
    #[serde(alias = "p10")]
    Pkcs10,
    #[doc = "application/pkcs7-mime"]
    #[serde(rename = "application/pkcs7-mime")]
    #[serde(alias = "p7m")]
    #[serde(alias = "p7c")]
    Pkcs7Mime,
    #[doc = "application/pkcs7-signature"]
    #[serde(rename = "application/pkcs7-signature")]
    #[serde(alias = "p7s")]
    Pkcs7Signature,
    #[doc = "application/pkcs8"]
    #[serde(rename = "application/pkcs8")]
    #[serde(alias = "p8")]
    Pkcs8,
    #[doc = "application/pkcs8-encrypted"]
    #[serde(rename = "application/pkcs8-encrypted")]
    #[serde(alias = "p8e")]
    Pkcs8Encrypted,
    #[doc = "application/pkcs12"]
    #[serde(rename = "application/pkcs12")]
    #[serde(alias = "p12")]
    #[serde(alias = "pfx")]
    Pkcs12,
    #[doc = "application/pkix-attr-cert"]
    #[serde(rename = "application/pkix-attr-cert")]
    PkixAttrCert,
    #[doc = "application/pkix-cert"]
    #[serde(rename = "application/pkix-cert")]
    #[serde(alias = "cer")]
    PkixCert,
    #[doc = "application/pkix-crl"]
    #[serde(rename = "application/pkix-crl")]
    #[serde(alias = "crl")]
    PkixCrl,
    #[doc = "application/pkix-pkipath"]
    #[serde(rename = "application/pkix-pkipath")]
    #[serde(alias = "pkipath")]
    PkixPkipath,
    #[doc = "application/pkixcmp"]
    #[serde(rename = "application/pkixcmp")]
    #[serde(alias = "pki")]
    Pkixcmp,
    #[doc = "application/pls+xml"]
    #[serde(rename = "application/pls+xml")]
    #[serde(alias = "pls")]
    PlsXml,
    #[doc = "application/poc-settings+xml"]
    #[serde(rename = "application/poc-settings+xml")]
    PocSettingsXml,
    #[doc = "application/postscript"]
    #[serde(rename = "application/postscript")]
    #[serde(alias = "ps")]
    #[serde(alias = "eps")]
    #[serde(alias = "ai")]
    Postscript,
    #[doc = "application/ppsp-tracker+json"]
    #[serde(rename = "application/ppsp-tracker+json")]
    PpspTrackerJson,
    #[doc = "application/problem+json"]
    #[serde(rename = "application/problem+json")]
    ProblemJson,
    #[doc = "application/problem+xml"]
    #[serde(rename = "application/problem+xml")]
    ProblemXml,
    #[doc = "application/provenance+xml"]
    #[serde(rename = "application/provenance+xml")]
    #[serde(alias = "provx")]
    ProvenanceXml,
    #[doc = "application/prs.alvestrand.titrax-sheet"]
    #[serde(rename = "application/prs.alvestrand.titrax-sheet")]
    PrsAlvestrandTitraxSheet,
    #[doc = "application/prs.cww"]
    #[serde(rename = "application/prs.cww")]
    #[serde(alias = "cw")]
    #[serde(alias = "cww")]
    PrsCww,
    #[doc = "application/prs.cyn"]
    #[serde(rename = "application/prs.cyn")]
    PrsCyn,
    #[doc = "application/prs.hpub+zip"]
    #[serde(rename = "application/prs.hpub+zip")]
    #[serde(alias = "hpub")]
    PrsHpubZip,
    #[doc = "application/prs.implied-document+xml"]
    #[serde(rename = "application/prs.implied-document+xml")]
    PrsImpliedDocumentXml,
    #[doc = "application/prs.implied-executable"]
    #[serde(rename = "application/prs.implied-executable")]
    PrsImpliedExecutable,
    #[doc = "application/prs.implied-structure"]
    #[serde(rename = "application/prs.implied-structure")]
    PrsImpliedStructure,
    #[doc = "application/prs.nprend"]
    #[serde(rename = "application/prs.nprend")]
    #[serde(alias = "rnd")]
    #[serde(alias = "rct")]
    PrsNprend,
    #[doc = "application/prs.plucker"]
    #[serde(rename = "application/prs.plucker")]
    PrsPlucker,
    #[doc = "application/prs.rdf-xml-crypt"]
    #[serde(rename = "application/prs.rdf-xml-crypt")]
    #[serde(alias = "rdf-crypt")]
    PrsRdfXmlCrypt,
    #[doc = "application/prs.xsf+xml"]
    #[serde(rename = "application/prs.xsf+xml")]
    #[serde(alias = "xsf")]
    PrsXsfXml,
    #[doc = "application/pskc+xml"]
    #[serde(rename = "application/pskc+xml")]
    #[serde(alias = "pskcxml")]
    PskcXml,
    #[doc = "application/pvd+json"]
    #[serde(rename = "application/pvd+json")]
    PvdJson,
    #[doc = "application/rdf+xml"]
    #[serde(rename = "application/rdf+xml")]
    #[serde(alias = "rdf")]
    RdfXml,
    #[doc = "application/route-apd+xml"]
    #[serde(rename = "application/route-apd+xml")]
    #[serde(alias = "rapd")]
    RouteApdXml,
    #[doc = "application/route-s-tsid+xml"]
    #[serde(rename = "application/route-s-tsid+xml")]
    #[serde(alias = "sls")]
    RouteSTsidXml,
    #[doc = "application/route-usd+xml"]
    #[serde(rename = "application/route-usd+xml")]
    #[serde(alias = "rusd")]
    RouteUsdXml,
    #[doc = "application/QSIG"]
    #[serde(rename = "application/QSIG")]
    Qsig,
    #[doc = "application/raptorfec"]
    #[serde(rename = "application/raptorfec")]
    Raptorfec,
    #[doc = "application/rdap+json"]
    #[serde(rename = "application/rdap+json")]
    RdapJson,
    #[doc = "application/reginfo+xml"]
    #[serde(rename = "application/reginfo+xml")]
    #[serde(alias = "rif")]
    ReginfoXml,
    #[doc = "application/relax-ng-compact-syntax"]
    #[serde(rename = "application/relax-ng-compact-syntax")]
    #[serde(alias = "rnc")]
    RelaxNgCompactSyntax,
    #[doc = "application/reputon+json"]
    #[serde(rename = "application/reputon+json")]
    ReputonJson,
    #[doc = "application/resource-lists-diff+xml"]
    #[serde(rename = "application/resource-lists-diff+xml")]
    #[serde(alias = "rld")]
    ResourceListsDiffXml,
    #[doc = "application/resource-lists+xml"]
    #[serde(rename = "application/resource-lists+xml")]
    #[serde(alias = "rl")]
    ResourceListsXml,
    #[doc = "application/rfc+xml"]
    #[serde(rename = "application/rfc+xml")]
    #[serde(alias = "rfcxml")]
    RfcXml,
    #[doc = "application/riscos"]
    #[serde(rename = "application/riscos")]
    Riscos,
    #[doc = "application/rlmi+xml"]
    #[serde(rename = "application/rlmi+xml")]
    RlmiXml,
    #[doc = "application/rls-services+xml"]
    #[serde(rename = "application/rls-services+xml")]
    #[serde(alias = "rs")]
    RlsServicesXml,
    #[doc = "application/rpki-checklist"]
    #[serde(rename = "application/rpki-checklist")]
    RpkiChecklist,
    #[doc = "application/rpki-ghostbusters"]
    #[serde(rename = "application/rpki-ghostbusters")]
    #[serde(alias = "gbr")]
    RpkiGhostbusters,
    #[doc = "application/rpki-manifest"]
    #[serde(rename = "application/rpki-manifest")]
    #[serde(alias = "mft")]
    RpkiManifest,
    #[doc = "application/rpki-publication"]
    #[serde(rename = "application/rpki-publication")]
    RpkiPublication,
    #[doc = "application/rpki-roa"]
    #[serde(rename = "application/rpki-roa")]
    #[serde(alias = "roa")]
    RpkiRoa,
    #[doc = "application/rpki-updown"]
    #[serde(rename = "application/rpki-updown")]
    RpkiUpdown,
    #[doc = "application/rtf"]
    #[serde(rename = "application/rtf")]
    #[serde(alias = "rtf")]
    Rtf,
    #[doc = "application/rtploopback"]
    #[serde(rename = "application/rtploopback")]
    Rtploopback,
    #[doc = "application/rtx"]
    #[serde(rename = "application/rtx")]
    Rtx,
    #[doc = "application/samlassertion+xml"]
    #[serde(rename = "application/samlassertion+xml")]
    SamlassertionXml,
    #[doc = "application/samlmetadata+xml"]
    #[serde(rename = "application/samlmetadata+xml")]
    SamlmetadataXml,
    #[doc = "application/sarif-external-properties+json"]
    #[serde(rename = "application/sarif-external-properties+json")]
    #[serde(alias = "sarif-external-properties")]
    #[serde(alias = "sarif-external-properties.json")]
    SarifExternalPropertiesJson,
    #[doc = "application/sarif+json"]
    #[serde(rename = "application/sarif+json")]
    #[serde(alias = "sarif")]
    #[serde(alias = "sarif.json")]
    SarifJson,
    #[doc = "application/sbe"]
    #[serde(rename = "application/sbe")]
    Sbe,
    #[doc = "application/sbml+xml"]
    #[serde(rename = "application/sbml+xml")]
    SbmlXml,
    #[doc = "application/scaip+xml"]
    #[serde(rename = "application/scaip+xml")]
    ScaipXml,
    #[doc = "application/scim+json"]
    #[serde(rename = "application/scim+json")]
    #[serde(alias = "scim")]
    ScimJson,
    #[doc = "application/scvp-cv-request"]
    #[serde(rename = "application/scvp-cv-request")]
    #[serde(alias = "scq")]
    ScvpCvRequest,
    #[doc = "application/scvp-cv-response"]
    #[serde(rename = "application/scvp-cv-response")]
    #[serde(alias = "scs")]
    ScvpCvResponse,
    #[doc = "application/scvp-vp-request"]
    #[serde(rename = "application/scvp-vp-request")]
    #[serde(alias = "spq")]
    ScvpVpRequest,
    #[doc = "application/scvp-vp-response"]
    #[serde(rename = "application/scvp-vp-response")]
    #[serde(alias = "spp")]
    ScvpVpResponse,
    #[doc = "application/sdp"]
    #[serde(rename = "application/sdp")]
    #[serde(alias = "sdp")]
    Sdp,
    #[doc = "application/secevent+jwt"]
    #[serde(rename = "application/secevent+jwt")]
    SeceventJwt,
    #[doc = "application/senml-etch+cbor"]
    #[serde(rename = "application/senml-etch+cbor")]
    #[serde(alias = "senml-etchc")]
    SenmlEtchCbor,
    #[doc = "application/senml-etch+json"]
    #[serde(rename = "application/senml-etch+json")]
    #[serde(alias = "senml-etchj")]
    SenmlEtchJson,
    #[doc = "application/senml-exi"]
    #[serde(rename = "application/senml-exi")]
    #[serde(alias = "senmle")]
    SenmlExi,
    #[doc = "application/senml+cbor"]
    #[serde(rename = "application/senml+cbor")]
    #[serde(alias = "senmlc")]
    SenmlCbor,
    #[doc = "application/senml+json"]
    #[serde(rename = "application/senml+json")]
    #[serde(alias = "senml")]
    SenmlJson,
    #[doc = "application/senml+xml"]
    #[serde(rename = "application/senml+xml")]
    #[serde(alias = "senmlx")]
    SenmlXml,
    #[doc = "application/sensml-exi"]
    #[serde(rename = "application/sensml-exi")]
    #[serde(alias = "sensmle")]
    SensmlExi,
    #[doc = "application/sensml+cbor"]
    #[serde(rename = "application/sensml+cbor")]
    #[serde(alias = "sensmlc")]
    SensmlCbor,
    #[doc = "application/sensml+json"]
    #[serde(rename = "application/sensml+json")]
    #[serde(alias = "sensml")]
    SensmlJson,
    #[doc = "application/sensml+xml"]
    #[serde(rename = "application/sensml+xml")]
    #[serde(alias = "sensmlx")]
    SensmlXml,
    #[doc = "application/sep-exi"]
    #[serde(rename = "application/sep-exi")]
    SepExi,
    #[doc = "application/sep+xml"]
    #[serde(rename = "application/sep+xml")]
    SepXml,
    #[doc = "application/session-info"]
    #[serde(rename = "application/session-info")]
    SessionInfo,
    #[doc = "application/set-payment"]
    #[serde(rename = "application/set-payment")]
    SetPayment,
    #[doc = "application/set-payment-initiation"]
    #[serde(rename = "application/set-payment-initiation")]
    SetPaymentInitiation,
    #[doc = "application/set-registration"]
    #[serde(rename = "application/set-registration")]
    SetRegistration,
    #[doc = "application/set-registration-initiation"]
    #[serde(rename = "application/set-registration-initiation")]
    SetRegistrationInitiation,
    #[doc = "application/SGML"]
    #[serde(rename = "application/SGML")]
    Sgml,
    #[doc = "application/sgml-open-catalog"]
    #[serde(rename = "application/sgml-open-catalog")]
    #[serde(alias = "soc")]
    SgmlOpenCatalog,
    #[doc = "application/shf+xml"]
    #[serde(rename = "application/shf+xml")]
    #[serde(alias = "shf")]
    ShfXml,
    #[doc = "application/sieve"]
    #[serde(rename = "application/sieve")]
    #[serde(alias = "siv")]
    #[serde(alias = "sieve")]
    Sieve,
    #[doc = "application/simple-filter+xml"]
    #[serde(rename = "application/simple-filter+xml")]
    #[serde(alias = "cl")]
    SimpleFilterXml,
    #[doc = "application/simple-message-summary"]
    #[serde(rename = "application/simple-message-summary")]
    SimpleMessageSummary,
    #[doc = "application/simpleSymbolContainer"]
    #[serde(rename = "application/simpleSymbolContainer")]
    SimpleSymbolContainer,
    #[doc = "application/sipc"]
    #[serde(rename = "application/sipc")]
    Sipc,
    #[doc = "application/slate"]
    #[serde(rename = "application/slate")]
    Slate,
    #[doc = "application/smil+xml"]
    #[serde(rename = "application/smil+xml")]
    #[serde(alias = "smil")]
    #[serde(alias = "smi")]
    #[serde(alias = "sml")]
    SmilXml,
    #[doc = "application/smpte336m"]
    #[serde(rename = "application/smpte336m")]
    Smpte336M,
    #[doc = "application/soap+fastinfoset"]
    #[serde(rename = "application/soap+fastinfoset")]
    SoapFastinfoset,
    #[doc = "application/soap+xml"]
    #[serde(rename = "application/soap+xml")]
    SoapXml,
    #[doc = "application/sparql-query"]
    #[serde(rename = "application/sparql-query")]
    #[serde(alias = "rq")]
    SparqlQuery,
    #[doc = "application/spdx+json"]
    #[serde(rename = "application/spdx+json")]
    #[serde(alias = "spdx.json")]
    SpdxJson,
    #[doc = "application/sparql-results+xml"]
    #[serde(rename = "application/sparql-results+xml")]
    #[serde(alias = "srx")]
    SparqlResultsXml,
    #[doc = "application/spirits-event+xml"]
    #[serde(rename = "application/spirits-event+xml")]
    SpiritsEventXml,
    #[doc = "application/sql"]
    #[serde(rename = "application/sql")]
    #[serde(alias = "sql")]
    Sql,
    #[doc = "application/srgs"]
    #[serde(rename = "application/srgs")]
    #[serde(alias = "gram")]
    Srgs,
    #[doc = "application/srgs+xml"]
    #[serde(rename = "application/srgs+xml")]
    #[serde(alias = "grxml")]
    SrgsXml,
    #[doc = "application/sru+xml"]
    #[serde(rename = "application/sru+xml")]
    #[serde(alias = "sru")]
    SruXml,
    #[doc = "application/ssml+xml"]
    #[serde(rename = "application/ssml+xml")]
    #[serde(alias = "ssml")]
    SsmlXml,
    #[doc = "application/stix+json"]
    #[serde(rename = "application/stix+json")]
    #[serde(alias = "stix")]
    StixJson,
    #[doc = "application/swid+cbor"]
    #[serde(rename = "application/swid+cbor")]
    #[serde(alias = "coswid")]
    SwidCbor,
    #[doc = "application/swid+xml"]
    #[serde(rename = "application/swid+xml")]
    #[serde(alias = "swidtag")]
    SwidXml,
    #[doc = "application/tamp-apex-update"]
    #[serde(rename = "application/tamp-apex-update")]
    #[serde(alias = "tau")]
    TampApexUpdate,
    #[doc = "application/tamp-apex-update-confirm"]
    #[serde(rename = "application/tamp-apex-update-confirm")]
    #[serde(alias = "auc")]
    TampApexUpdateConfirm,
    #[doc = "application/tamp-community-update"]
    #[serde(rename = "application/tamp-community-update")]
    #[serde(alias = "tcu")]
    TampCommunityUpdate,
    #[doc = "application/tamp-community-update-confirm"]
    #[serde(rename = "application/tamp-community-update-confirm")]
    #[serde(alias = "cuc")]
    TampCommunityUpdateConfirm,
    #[doc = "application/tamp-error"]
    #[serde(rename = "application/tamp-error")]
    #[serde(alias = "ter")]
    TampError,
    #[doc = "application/tamp-sequence-adjust"]
    #[serde(rename = "application/tamp-sequence-adjust")]
    #[serde(alias = "tsa")]
    TampSequenceAdjust,
    #[doc = "application/tamp-sequence-adjust-confirm"]
    #[serde(rename = "application/tamp-sequence-adjust-confirm")]
    #[serde(alias = "sac")]
    TampSequenceAdjustConfirm,
    #[doc = "application/tamp-status-query"]
    #[serde(rename = "application/tamp-status-query")]
    TampStatusQuery,
    #[doc = "application/tamp-status-response"]
    #[serde(rename = "application/tamp-status-response")]
    TampStatusResponse,
    #[doc = "application/tamp-update"]
    #[serde(rename = "application/tamp-update")]
    #[serde(alias = "tur")]
    TampUpdate,
    #[doc = "application/tamp-update-confirm"]
    #[serde(rename = "application/tamp-update-confirm")]
    #[serde(alias = "tuc")]
    TampUpdateConfirm,
    #[doc = "application/taxii+json"]
    #[serde(rename = "application/taxii+json")]
    TaxiiJson,
    #[doc = "application/td+json"]
    #[serde(rename = "application/td+json")]
    #[serde(alias = "jsontd")]
    TdJson,
    #[doc = "application/tei+xml"]
    #[serde(rename = "application/tei+xml")]
    #[serde(alias = "tei")]
    #[serde(alias = "teiCorpus")]
    #[serde(alias = "odd")]
    TeiXml,
    #[doc = "application/TETRA_ISI"]
    #[serde(rename = "application/TETRA_ISI")]
    TetraIsi,
    #[doc = "application/thraud+xml"]
    #[serde(rename = "application/thraud+xml")]
    #[serde(alias = "tfi")]
    ThraudXml,
    #[doc = "application/timestamp-query"]
    #[serde(rename = "application/timestamp-query")]
    #[serde(alias = "tsq")]
    TimestampQuery,
    #[doc = "application/timestamp-reply"]
    #[serde(rename = "application/timestamp-reply")]
    #[serde(alias = "tsr")]
    TimestampReply,
    #[doc = "application/timestamped-data"]
    #[serde(rename = "application/timestamped-data")]
    #[serde(alias = "tsd")]
    TimestampedData,
    #[doc = "application/tlsrpt+gzip"]
    #[serde(rename = "application/tlsrpt+gzip")]
    TlsrptGzip,
    #[doc = "application/tlsrpt+json"]
    #[serde(rename = "application/tlsrpt+json")]
    TlsrptJson,
    #[doc = "application/tm+json"]
    #[serde(rename = "application/tm+json")]
    #[serde(alias = "jsontm")]
    #[serde(alias = "tm.json")]
    #[serde(alias = "tm.jsonld")]
    TmJson,
    #[doc = "application/tnauthlist"]
    #[serde(rename = "application/tnauthlist")]
    Tnauthlist,
    #[doc = "application/token-introspection+jwt"]
    #[serde(rename = "application/token-introspection+jwt")]
    TokenIntrospectionJwt,
    #[doc = "application/trickle-ice-sdpfrag"]
    #[serde(rename = "application/trickle-ice-sdpfrag")]
    TrickleIceSdpfrag,
    #[doc = "application/trig"]
    #[serde(rename = "application/trig")]
    #[serde(alias = "trig")]
    Trig,
    #[doc = "application/ttml+xml"]
    #[serde(rename = "application/ttml+xml")]
    #[serde(alias = "ttml")]
    TtmlXml,
    #[doc = "application/tve-trigger"]
    #[serde(rename = "application/tve-trigger")]
    TveTrigger,
    #[doc = "application/tzif"]
    #[serde(rename = "application/tzif")]
    Tzif,
    #[doc = "application/tzif-leap"]
    #[serde(rename = "application/tzif-leap")]
    TzifLeap,
    #[doc = "application/ulpfec"]
    #[serde(rename = "application/ulpfec")]
    Ulpfec,
    #[doc = "application/urc-grpsheet+xml"]
    #[serde(rename = "application/urc-grpsheet+xml")]
    #[serde(alias = "gsheet")]
    UrcGrpsheetXml,
    #[doc = "application/urc-ressheet+xml"]
    #[serde(rename = "application/urc-ressheet+xml")]
    #[serde(alias = "rsheet")]
    UrcRessheetXml,
    #[doc = "application/urc-targetdesc+xml"]
    #[serde(rename = "application/urc-targetdesc+xml")]
    #[serde(alias = "td")]
    UrcTargetdescXml,
    #[doc = "application/urc-uisocketdesc+xml"]
    #[serde(rename = "application/urc-uisocketdesc+xml")]
    #[serde(alias = "uis")]
    UrcUisocketdescXml,
    #[doc = "application/vcard+json"]
    #[serde(rename = "application/vcard+json")]
    VcardJson,
    #[doc = "application/vcard+xml"]
    #[serde(rename = "application/vcard+xml")]
    VcardXml,
    #[doc = "application/vemmi"]
    #[serde(rename = "application/vemmi")]
    Vemmi,
    #[doc = "application/vnd.1000minds.decision-model+xml"]
    #[serde(rename = "application/vnd.1000minds.decision-model+xml")]
    #[serde(alias = "1km")]
    Vnd1000MindsDecisionModelXml,
    #[doc = "application/vnd.1ob"]
    #[serde(rename = "application/vnd.1ob")]
    #[serde(alias = "ob")]
    Vnd1Ob,
    #[doc = "application/vnd.3gpp.5gnas"]
    #[serde(rename = "application/vnd.3gpp.5gnas")]
    Vnd3Gpp5Gnas,
    #[doc = "application/vnd.3gpp.access-transfer-events+xml"]
    #[serde(rename = "application/vnd.3gpp.access-transfer-events+xml")]
    Vnd3GppAccessTransferEventsXml,
    #[doc = "application/vnd.3gpp.bsf+xml"]
    #[serde(rename = "application/vnd.3gpp.bsf+xml")]
    Vnd3GppBsfXml,
    #[doc = "application/vnd.3gpp.crs+xml"]
    #[serde(rename = "application/vnd.3gpp.crs+xml")]
    Vnd3GppCrsXml,
    #[doc = "application/vnd.3gpp.current-location-discovery+xml"]
    #[serde(rename = "application/vnd.3gpp.current-location-discovery+xml")]
    Vnd3GppCurrentLocationDiscoveryXml,
    #[doc = "application/vnd.3gpp.GMOP+xml"]
    #[serde(rename = "application/vnd.3gpp.GMOP+xml")]
    Vnd3GppGMOPXml,
    #[doc = "application/vnd.3gpp.gtpc"]
    #[serde(rename = "application/vnd.3gpp.gtpc")]
    Vnd3GppGtpc,
    #[doc = "application/vnd.3gpp.interworking-data"]
    #[serde(rename = "application/vnd.3gpp.interworking-data")]
    Vnd3GppInterworkingData,
    #[doc = "application/vnd.3gpp.lpp"]
    #[serde(rename = "application/vnd.3gpp.lpp")]
    Vnd3GppLpp,
    #[doc = "application/vnd.3gpp.mc-signalling-ear"]
    #[serde(rename = "application/vnd.3gpp.mc-signalling-ear")]
    Vnd3GppMcSignallingEar,
    #[doc = "application/vnd.3gpp.mcdata-affiliation-command+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-affiliation-command+xml")]
    Vnd3GppMcdataAffiliationCommandXml,
    #[doc = "application/vnd.3gpp.mcdata-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-info+xml")]
    Vnd3GppMcdataInfoXml,
    #[doc = "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml")]
    Vnd3GppMcdataMsgstoreCtrlRequestXml,
    #[doc = "application/vnd.3gpp.mcdata-payload"]
    #[serde(rename = "application/vnd.3gpp.mcdata-payload")]
    Vnd3GppMcdataPayload,
    #[doc = "application/vnd.3gpp.mcdata-regroup+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-regroup+xml")]
    Vnd3GppMcdataRegroupXml,
    #[doc = "application/vnd.3gpp.mcdata-service-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-service-config+xml")]
    Vnd3GppMcdataServiceConfigXml,
    #[doc = "application/vnd.3gpp.mcdata-signalling"]
    #[serde(rename = "application/vnd.3gpp.mcdata-signalling")]
    Vnd3GppMcdataSignalling,
    #[doc = "application/vnd.3gpp.mcdata-ue-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-ue-config+xml")]
    Vnd3GppMcdataUeConfigXml,
    #[doc = "application/vnd.3gpp.mcdata-user-profile+xml"]
    #[serde(rename = "application/vnd.3gpp.mcdata-user-profile+xml")]
    Vnd3GppMcdataUserProfileXml,
    #[doc = "application/vnd.3gpp.mcptt-affiliation-command+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-affiliation-command+xml")]
    Vnd3GppMcpttAffiliationCommandXml,
    #[doc = "application/vnd.3gpp.mcptt-floor-request+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-floor-request+xml")]
    Vnd3GppMcpttFloorRequestXml,
    #[doc = "application/vnd.3gpp.mcptt-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-info+xml")]
    Vnd3GppMcpttInfoXml,
    #[doc = "application/vnd.3gpp.mcptt-location-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-location-info+xml")]
    Vnd3GppMcpttLocationInfoXml,
    #[doc = "application/vnd.3gpp.mcptt-mbms-usage-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-mbms-usage-info+xml")]
    Vnd3GppMcpttMbmsUsageInfoXml,
    #[doc = "application/vnd.3gpp.mcptt-regroup+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-regroup+xml")]
    Vnd3GppMcpttRegroupXml,
    #[doc = "application/vnd.3gpp.mcptt-service-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-service-config+xml")]
    Vnd3GppMcpttServiceConfigXml,
    #[doc = "application/vnd.3gpp.mcptt-signed+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-signed+xml")]
    Vnd3GppMcpttSignedXml,
    #[doc = "application/vnd.3gpp.mcptt-ue-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-ue-config+xml")]
    Vnd3GppMcpttUeConfigXml,
    #[doc = "application/vnd.3gpp.mcptt-ue-init-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-ue-init-config+xml")]
    Vnd3GppMcpttUeInitConfigXml,
    #[doc = "application/vnd.3gpp.mcptt-user-profile+xml"]
    #[serde(rename = "application/vnd.3gpp.mcptt-user-profile+xml")]
    Vnd3GppMcpttUserProfileXml,
    #[doc = "application/vnd.3gpp.mcvideo-affiliation-command+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-affiliation-command+xml")]
    Vnd3GppMcvideoAffiliationCommandXml,
    #[doc = "application/vnd.3gpp.mcvideo-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-info+xml")]
    Vnd3GppMcvideoInfoXml,
    #[doc = "application/vnd.3gpp.mcvideo-location-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-location-info+xml")]
    Vnd3GppMcvideoLocationInfoXml,
    #[doc = "application/vnd.3gpp.mcvideo-mbms-usage-info+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-mbms-usage-info+xml")]
    Vnd3GppMcvideoMbmsUsageInfoXml,
    #[doc = "application/vnd.3gpp.mcvideo-regroup+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-regroup+xml")]
    Vnd3GppMcvideoRegroupXml,
    #[doc = "application/vnd.3gpp.mcvideo-service-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-service-config+xml")]
    Vnd3GppMcvideoServiceConfigXml,
    #[doc = "application/vnd.3gpp.mcvideo-transmission-request+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-transmission-request+xml")]
    Vnd3GppMcvideoTransmissionRequestXml,
    #[doc = "application/vnd.3gpp.mcvideo-ue-config+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-ue-config+xml")]
    Vnd3GppMcvideoUeConfigXml,
    #[doc = "application/vnd.3gpp.mcvideo-user-profile+xml"]
    #[serde(rename = "application/vnd.3gpp.mcvideo-user-profile+xml")]
    Vnd3GppMcvideoUserProfileXml,
    #[doc = "application/vnd.3gpp.mid-call+xml"]
    #[serde(rename = "application/vnd.3gpp.mid-call+xml")]
    Vnd3GppMidCallXml,
    #[doc = "application/vnd.3gpp.ngap"]
    #[serde(rename = "application/vnd.3gpp.ngap")]
    Vnd3GppNgap,
    #[doc = "application/vnd.3gpp.pfcp"]
    #[serde(rename = "application/vnd.3gpp.pfcp")]
    Vnd3GppPfcp,
    #[doc = "application/vnd.3gpp.pic-bw-large"]
    #[serde(rename = "application/vnd.3gpp.pic-bw-large")]
    #[serde(alias = "plb")]
    Vnd3GppPicBwLarge,
    #[doc = "application/vnd.3gpp.pic-bw-small"]
    #[serde(rename = "application/vnd.3gpp.pic-bw-small")]
    #[serde(alias = "psb")]
    Vnd3GppPicBwSmall,
    #[doc = "application/vnd.3gpp.pic-bw-var"]
    #[serde(rename = "application/vnd.3gpp.pic-bw-var")]
    #[serde(alias = "pvb")]
    Vnd3GppPicBwVar,
    #[doc = "application/vnd.3gpp-prose-pc3a+xml"]
    #[serde(rename = "application/vnd.3gpp-prose-pc3a+xml")]
    Vnd3GppProsePc3AXml,
    #[doc = "application/vnd.3gpp-prose-pc3ach+xml"]
    #[serde(rename = "application/vnd.3gpp-prose-pc3ach+xml")]
    Vnd3GppProsePc3AchXml,
    #[doc = "application/vnd.3gpp-prose-pc3ch+xml"]
    #[serde(rename = "application/vnd.3gpp-prose-pc3ch+xml")]
    Vnd3GppProsePc3ChXml,
    #[doc = "application/vnd.3gpp-prose-pc8+xml"]
    #[serde(rename = "application/vnd.3gpp-prose-pc8+xml")]
    Vnd3GppProsePc8Xml,
    #[doc = "application/vnd.3gpp-prose+xml"]
    #[serde(rename = "application/vnd.3gpp-prose+xml")]
    Vnd3GppProseXml,
    #[doc = "application/vnd.3gpp.s1ap"]
    #[serde(rename = "application/vnd.3gpp.s1ap")]
    Vnd3GppS1Ap,
    #[doc = "application/vnd.3gpp.seal-group-doc+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-group-doc+xml")]
    Vnd3GppSealGroupDocXml,
    #[doc = "application/vnd.3gpp.seal-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-info+xml")]
    Vnd3GppSealInfoXml,
    #[doc = "application/vnd.3gpp.seal-location-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-location-info+xml")]
    Vnd3GppSealLocationInfoXml,
    #[doc = "application/vnd.3gpp.seal-mbms-usage-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-mbms-usage-info+xml")]
    Vnd3GppSealMbmsUsageInfoXml,
    #[doc = "application/vnd.3gpp.seal-network-QoS-management-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-network-QoS-management-info+xml")]
    Vnd3GppSealNetworkQoSManagementInfoXml,
    #[doc = "application/vnd.3gpp.seal-ue-config-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-ue-config-info+xml")]
    Vnd3GppSealUeConfigInfoXml,
    #[doc = "application/vnd.3gpp.seal-unicast-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-unicast-info+xml")]
    Vnd3GppSealUnicastInfoXml,
    #[doc = "application/vnd.3gpp.seal-user-profile-info+xml"]
    #[serde(rename = "application/vnd.3gpp.seal-user-profile-info+xml")]
    Vnd3GppSealUserProfileInfoXml,
    #[doc = "application/vnd.3gpp.sms"]
    #[serde(rename = "application/vnd.3gpp.sms")]
    Vnd3GppSms,
    #[doc = "application/vnd.3gpp.sms+xml"]
    #[serde(rename = "application/vnd.3gpp.sms+xml")]
    Vnd3GppSmsXml,
    #[doc = "application/vnd.3gpp.srvcc-ext+xml"]
    #[serde(rename = "application/vnd.3gpp.srvcc-ext+xml")]
    Vnd3GppSrvccExtXml,
    #[doc = "application/vnd.3gpp.SRVCC-info+xml"]
    #[serde(rename = "application/vnd.3gpp.SRVCC-info+xml")]
    Vnd3GppSRVCCInfoXml,
    #[doc = "application/vnd.3gpp.state-and-event-info+xml"]
    #[serde(rename = "application/vnd.3gpp.state-and-event-info+xml")]
    Vnd3GppStateAndEventInfoXml,
    #[doc = "application/vnd.3gpp.ussd+xml"]
    #[serde(rename = "application/vnd.3gpp.ussd+xml")]
    Vnd3GppUssdXml,
    #[doc = "application/vnd.3gpp.vae-info+xml"]
    #[serde(rename = "application/vnd.3gpp.vae-info+xml")]
    Vnd3GppVaeInfoXml,
    #[doc = "application/vnd.3gpp-v2x-local-service-information"]
    #[serde(rename = "application/vnd.3gpp-v2x-local-service-information")]
    Vnd3GppV2XLocalServiceInformation,
    #[doc = "application/vnd.3gpp2.bcmcsinfo+xml"]
    #[serde(rename = "application/vnd.3gpp2.bcmcsinfo+xml")]
    Vnd3Gpp2BcmcsinfoXml,
    #[doc = "application/vnd.3gpp2.sms"]
    #[serde(rename = "application/vnd.3gpp2.sms")]
    #[serde(alias = "sms")]
    Vnd3Gpp2Sms,
    #[doc = "application/vnd.3gpp2.tcap"]
    #[serde(rename = "application/vnd.3gpp2.tcap")]
    #[serde(alias = "tcap")]
    Vnd3Gpp2Tcap,
    #[doc = "application/vnd.3gpp.v2x"]
    #[serde(rename = "application/vnd.3gpp.v2x")]
    Vnd3GppV2X,
    #[doc = "application/vnd.3lightssoftware.imagescal"]
    #[serde(rename = "application/vnd.3lightssoftware.imagescal")]
    #[serde(alias = "imgcal")]
    Vnd3LightssoftwareImagescal,
    #[doc = "application/vnd.3M.Post-it-Notes"]
    #[serde(rename = "application/vnd.3M.Post-it-Notes")]
    #[serde(alias = "pwn")]
    Vnd3MPostItNotes,
    #[doc = "application/vnd.accpac.simply.aso"]
    #[serde(rename = "application/vnd.accpac.simply.aso")]
    #[serde(alias = "aso")]
    VndAccpacSimplyAso,
    #[doc = "application/vnd.accpac.simply.imp"]
    #[serde(rename = "application/vnd.accpac.simply.imp")]
    #[serde(alias = "imp")]
    VndAccpacSimplyImp,
    #[doc = "application/vnd.acm.addressxfer+json"]
    #[serde(rename = "application/vnd.acm.addressxfer+json")]
    VndAcmAddressxferJson,
    #[doc = "application/vnd.acucobol"]
    #[serde(rename = "application/vnd.acucobol")]
    #[serde(alias = "acu")]
    VndAcucobol,
    #[doc = "application/vnd.acucorp"]
    #[serde(rename = "application/vnd.acucorp")]
    #[serde(alias = "atc")]
    #[serde(alias = "acutc")]
    VndAcucorp,
    #[doc = "application/vnd.adobe.flash.movie"]
    #[serde(rename = "application/vnd.adobe.flash.movie")]
    #[serde(alias = "swf")]
    VndAdobeFlashMovie,
    #[doc = "application/vnd.adobe.formscentral.fcdt"]
    #[serde(rename = "application/vnd.adobe.formscentral.fcdt")]
    #[serde(alias = "fcdt")]
    VndAdobeFormscentralFcdt,
    #[doc = "application/vnd.adobe.fxp"]
    #[serde(rename = "application/vnd.adobe.fxp")]
    #[serde(alias = "fxp")]
    #[serde(alias = "fxpl")]
    VndAdobeFxp,
    #[doc = "application/vnd.adobe.partial-upload"]
    #[serde(rename = "application/vnd.adobe.partial-upload")]
    VndAdobePartialUpload,
    #[doc = "application/vnd.adobe.xdp+xml"]
    #[serde(rename = "application/vnd.adobe.xdp+xml")]
    #[serde(alias = "xdp")]
    VndAdobeXdpXml,
    #[doc = "application/vnd.aether.imp"]
    #[serde(rename = "application/vnd.aether.imp")]
    VndAetherImp,
    #[doc = "application/vnd.afpc.afplinedata"]
    #[serde(rename = "application/vnd.afpc.afplinedata")]
    VndAfpcAfplinedata,
    #[doc = "application/vnd.afpc.afplinedata-pagedef"]
    #[serde(rename = "application/vnd.afpc.afplinedata-pagedef")]
    VndAfpcAfplinedataPagedef,
    #[doc = "application/vnd.afpc.cmoca-cmresource"]
    #[serde(rename = "application/vnd.afpc.cmoca-cmresource")]
    VndAfpcCmocaCmresource,
    #[doc = "application/vnd.afpc.foca-charset"]
    #[serde(rename = "application/vnd.afpc.foca-charset")]
    VndAfpcFocaCharset,
    #[doc = "application/vnd.afpc.foca-codedfont"]
    #[serde(rename = "application/vnd.afpc.foca-codedfont")]
    VndAfpcFocaCodedfont,
    #[doc = "application/vnd.afpc.foca-codepage"]
    #[serde(rename = "application/vnd.afpc.foca-codepage")]
    VndAfpcFocaCodepage,
    #[doc = "application/vnd.afpc.modca"]
    #[serde(rename = "application/vnd.afpc.modca")]
    #[serde(alias = "list3820")]
    #[serde(alias = "listafp")]
    #[serde(alias = "afp")]
    #[serde(alias = "pseg3820")]
    VndAfpcModca,
    #[doc = "application/vnd.afpc.modca-cmtable"]
    #[serde(rename = "application/vnd.afpc.modca-cmtable")]
    VndAfpcModcaCmtable,
    #[doc = "application/vnd.afpc.modca-formdef"]
    #[serde(rename = "application/vnd.afpc.modca-formdef")]
    VndAfpcModcaFormdef,
    #[doc = "application/vnd.afpc.modca-mediummap"]
    #[serde(rename = "application/vnd.afpc.modca-mediummap")]
    VndAfpcModcaMediummap,
    #[doc = "application/vnd.afpc.modca-objectcontainer"]
    #[serde(rename = "application/vnd.afpc.modca-objectcontainer")]
    VndAfpcModcaObjectcontainer,
    #[doc = "application/vnd.afpc.modca-overlay"]
    #[serde(rename = "application/vnd.afpc.modca-overlay")]
    #[serde(alias = "ovl")]
    VndAfpcModcaOverlay,
    #[doc = "application/vnd.afpc.modca-pagesegment"]
    #[serde(rename = "application/vnd.afpc.modca-pagesegment")]
    #[serde(alias = "psg")]
    VndAfpcModcaPagesegment,
    #[doc = "application/vnd.age"]
    #[serde(rename = "application/vnd.age")]
    #[serde(alias = "age")]
    VndAge,
    #[doc = "application/vnd.ah-barcode"]
    #[serde(rename = "application/vnd.ah-barcode")]
    VndAhBarcode,
    #[doc = "application/vnd.ahead.space"]
    #[serde(rename = "application/vnd.ahead.space")]
    #[serde(alias = "ahead")]
    VndAheadSpace,
    #[doc = "application/vnd.airzip.filesecure.azf"]
    #[serde(rename = "application/vnd.airzip.filesecure.azf")]
    #[serde(alias = "azf")]
    VndAirzipFilesecureAzf,
    #[doc = "application/vnd.airzip.filesecure.azs"]
    #[serde(rename = "application/vnd.airzip.filesecure.azs")]
    #[serde(alias = "azs")]
    VndAirzipFilesecureAzs,
    #[doc = "application/vnd.amadeus+json"]
    #[serde(rename = "application/vnd.amadeus+json")]
    VndAmadeusJson,
    #[doc = "application/vnd.amazon.mobi8-ebook"]
    #[serde(rename = "application/vnd.amazon.mobi8-ebook")]
    #[serde(alias = "azw3")]
    VndAmazonMobi8Ebook,
    #[doc = "application/vnd.americandynamics.acc"]
    #[serde(rename = "application/vnd.americandynamics.acc")]
    #[serde(alias = "acc")]
    VndAmericandynamicsAcc,
    #[doc = "application/vnd.amiga.ami"]
    #[serde(rename = "application/vnd.amiga.ami")]
    #[serde(alias = "ami")]
    VndAmigaAmi,
    #[doc = "application/vnd.amundsen.maze+xml"]
    #[serde(rename = "application/vnd.amundsen.maze+xml")]
    VndAmundsenMazeXml,
    #[doc = "application/vnd.android.ota"]
    #[serde(rename = "application/vnd.android.ota")]
    #[serde(alias = "ota")]
    VndAndroidOta,
    #[doc = "application/vnd.anki"]
    #[serde(rename = "application/vnd.anki")]
    #[serde(alias = "apkg")]
    VndAnki,
    #[doc = "application/vnd.anser-web-certificate-issue-initiation"]
    #[serde(rename = "application/vnd.anser-web-certificate-issue-initiation")]
    #[serde(alias = "cii")]
    VndAnserWebCertificateIssueInitiation,
    #[doc = "application/vnd.antix.game-component"]
    #[serde(rename = "application/vnd.antix.game-component")]
    VndAntixGameComponent,
    #[doc = "application/vnd.apache.arrow.file"]
    #[serde(rename = "application/vnd.apache.arrow.file")]
    #[serde(alias = "arrow")]
    VndApacheArrowFile,
    #[doc = "application/vnd.apache.arrow.stream"]
    #[serde(rename = "application/vnd.apache.arrow.stream")]
    #[serde(alias = "arrows")]
    VndApacheArrowStream,
    #[doc = "application/vnd.apache.thrift.binary"]
    #[serde(rename = "application/vnd.apache.thrift.binary")]
    VndApacheThriftBinary,
    #[doc = "application/vnd.apache.thrift.compact"]
    #[serde(rename = "application/vnd.apache.thrift.compact")]
    VndApacheThriftCompact,
    #[doc = "application/vnd.apache.thrift.json"]
    #[serde(rename = "application/vnd.apache.thrift.json")]
    VndApacheThriftJson,
    #[doc = "application/vnd.apexlang"]
    #[serde(rename = "application/vnd.apexlang")]
    #[serde(alias = "apexland")]
    #[serde(alias = "apex")]
    #[serde(alias = "axdl")]
    VndApexlang,
    #[doc = "application/vnd.api+json"]
    #[serde(rename = "application/vnd.api+json")]
    VndApiJson,
    #[doc = "application/vnd.aplextor.warrp+json"]
    #[serde(rename = "application/vnd.aplextor.warrp+json")]
    VndAplextorWarrpJson,
    #[doc = "application/vnd.apothekende.reservation+json"]
    #[serde(rename = "application/vnd.apothekende.reservation+json")]
    VndApothekendeReservationJson,
    #[doc = "application/vnd.apple.installer+xml"]
    #[serde(rename = "application/vnd.apple.installer+xml")]
    #[serde(alias = "dist")]
    #[serde(alias = "distz")]
    #[serde(alias = "pkg")]
    #[serde(alias = "mpkg")]
    VndAppleInstallerXml,
    #[doc = "application/vnd.apple.keynote"]
    #[serde(rename = "application/vnd.apple.keynote")]
    #[serde(alias = "keynote")]
    VndAppleKeynote,
    #[doc = "application/vnd.apple.mpegurl"]
    #[serde(rename = "application/vnd.apple.mpegurl")]
    #[serde(alias = "m3u8")]
    VndAppleMpegurl,
    #[doc = "application/vnd.apple.numbers"]
    #[serde(rename = "application/vnd.apple.numbers")]
    #[serde(alias = "numbers")]
    VndAppleNumbers,
    #[doc = "application/vnd.apple.pages"]
    #[serde(rename = "application/vnd.apple.pages")]
    #[serde(alias = "pages")]
    VndApplePages,
    #[doc = "application/vnd.aristanetworks.swi"]
    #[serde(rename = "application/vnd.aristanetworks.swi")]
    #[serde(alias = "swi")]
    VndAristanetworksSwi,
    #[doc = "application/vnd.artisan+json"]
    #[serde(rename = "application/vnd.artisan+json")]
    #[serde(alias = "artisan")]
    VndArtisanJson,
    #[doc = "application/vnd.artsquare"]
    #[serde(rename = "application/vnd.artsquare")]
    VndArtsquare,
    #[doc = "application/vnd.astraea-software.iota"]
    #[serde(rename = "application/vnd.astraea-software.iota")]
    #[serde(alias = "iota")]
    VndAstraeaSoftwareIota,
    #[doc = "application/vnd.audiograph"]
    #[serde(rename = "application/vnd.audiograph")]
    #[serde(alias = "aep")]
    VndAudiograph,
    #[doc = "application/vnd.autopackage"]
    #[serde(rename = "application/vnd.autopackage")]
    #[serde(alias = "package")]
    VndAutopackage,
    #[doc = "application/vnd.avalon+json"]
    #[serde(rename = "application/vnd.avalon+json")]
    VndAvalonJson,
    #[doc = "application/vnd.avistar+xml"]
    #[serde(rename = "application/vnd.avistar+xml")]
    VndAvistarXml,
    #[doc = "application/vnd.balsamiq.bmml+xml"]
    #[serde(rename = "application/vnd.balsamiq.bmml+xml")]
    #[serde(alias = "bmml")]
    VndBalsamiqBmmlXml,
    #[doc = "application/vnd.banana-accounting"]
    #[serde(rename = "application/vnd.banana-accounting")]
    #[serde(alias = "ac2")]
    VndBananaAccounting,
    #[doc = "application/vnd.bbf.usp.error"]
    #[serde(rename = "application/vnd.bbf.usp.error")]
    VndBbfUspError,
    #[doc = "application/vnd.bbf.usp.msg"]
    #[serde(rename = "application/vnd.bbf.usp.msg")]
    VndBbfUspMsg,
    #[doc = "application/vnd.bbf.usp.msg+json"]
    #[serde(rename = "application/vnd.bbf.usp.msg+json")]
    VndBbfUspMsgJson,
    #[doc = "application/vnd.balsamiq.bmpr"]
    #[serde(rename = "application/vnd.balsamiq.bmpr")]
    #[serde(alias = "bmpr")]
    VndBalsamiqBmpr,
    #[doc = "application/vnd.bekitzur-stech+json"]
    #[serde(rename = "application/vnd.bekitzur-stech+json")]
    VndBekitzurStechJson,
    #[doc = "application/vnd.belightsoft.lhzd+zip"]
    #[serde(rename = "application/vnd.belightsoft.lhzd+zip")]
    #[serde(alias = "lhzd")]
    VndBelightsoftLhzdZip,
    #[doc = "application/vnd.belightsoft.lhzl+zip"]
    #[serde(rename = "application/vnd.belightsoft.lhzl+zip")]
    #[serde(alias = "lhzl")]
    VndBelightsoftLhzlZip,
    #[doc = "application/vnd.bint.med-content"]
    #[serde(rename = "application/vnd.bint.med-content")]
    VndBintMedContent,
    #[doc = "application/vnd.biopax.rdf+xml"]
    #[serde(rename = "application/vnd.biopax.rdf+xml")]
    VndBiopaxRdfXml,
    #[doc = "application/vnd.blink-idb-value-wrapper"]
    #[serde(rename = "application/vnd.blink-idb-value-wrapper")]
    VndBlinkIdbValueWrapper,
    #[doc = "application/vnd.blueice.multipass"]
    #[serde(rename = "application/vnd.blueice.multipass")]
    #[serde(alias = "mpm")]
    VndBlueiceMultipass,
    #[doc = "application/vnd.bluetooth.ep.oob"]
    #[serde(rename = "application/vnd.bluetooth.ep.oob")]
    #[serde(alias = "ep")]
    VndBluetoothEpOob,
    #[doc = "application/vnd.bluetooth.le.oob"]
    #[serde(rename = "application/vnd.bluetooth.le.oob")]
    #[serde(alias = "le")]
    VndBluetoothLeOob,
    #[doc = "application/vnd.bmi"]
    #[serde(rename = "application/vnd.bmi")]
    #[serde(alias = "bmi")]
    VndBmi,
    #[doc = "application/vnd.bpf"]
    #[serde(rename = "application/vnd.bpf")]
    VndBpf,
    #[doc = "application/vnd.bpf3"]
    #[serde(rename = "application/vnd.bpf3")]
    VndBpf3,
    #[doc = "application/vnd.businessobjects"]
    #[serde(rename = "application/vnd.businessobjects")]
    #[serde(alias = "rep")]
    VndBusinessobjects,
    #[doc = "application/vnd.byu.uapi+json"]
    #[serde(rename = "application/vnd.byu.uapi+json")]
    VndByuUapiJson,
    #[doc = "application/vnd.cab-jscript"]
    #[serde(rename = "application/vnd.cab-jscript")]
    VndCabJscript,
    #[doc = "application/vnd.canon-cpdl"]
    #[serde(rename = "application/vnd.canon-cpdl")]
    VndCanonCpdl,
    #[doc = "application/vnd.canon-lips"]
    #[serde(rename = "application/vnd.canon-lips")]
    VndCanonLips,
    #[doc = "application/vnd.capasystems-pg+json"]
    #[serde(rename = "application/vnd.capasystems-pg+json")]
    VndCapasystemsPgJson,
    #[doc = "application/vnd.cendio.thinlinc.clientconf"]
    #[serde(rename = "application/vnd.cendio.thinlinc.clientconf")]
    #[serde(alias = "tlclient")]
    VndCendioThinlincClientconf,
    #[doc = "application/vnd.century-systems.tcp_stream"]
    #[serde(rename = "application/vnd.century-systems.tcp_stream")]
    VndCenturySystemsTcpStream,
    #[doc = "application/vnd.chemdraw+xml"]
    #[serde(rename = "application/vnd.chemdraw+xml")]
    #[serde(alias = "cdxml")]
    VndChemdrawXml,
    #[doc = "application/vnd.chess-pgn"]
    #[serde(rename = "application/vnd.chess-pgn")]
    #[serde(alias = "pgn")]
    VndChessPgn,
    #[doc = "application/vnd.chipnuts.karaoke-mmd"]
    #[serde(rename = "application/vnd.chipnuts.karaoke-mmd")]
    #[serde(alias = "mmd")]
    VndChipnutsKaraokeMmd,
    #[doc = "application/vnd.ciedi"]
    #[serde(rename = "application/vnd.ciedi")]
    VndCiedi,
    #[doc = "application/vnd.cinderella"]
    #[serde(rename = "application/vnd.cinderella")]
    #[serde(alias = "cdy")]
    VndCinderella,
    #[doc = "application/vnd.cirpack.isdn-ext"]
    #[serde(rename = "application/vnd.cirpack.isdn-ext")]
    VndCirpackIsdnExt,
    #[doc = "application/vnd.citationstyles.style+xml"]
    #[serde(rename = "application/vnd.citationstyles.style+xml")]
    #[serde(alias = "csl")]
    VndCitationstylesStyleXml,
    #[doc = "application/vnd.claymore"]
    #[serde(rename = "application/vnd.claymore")]
    #[serde(alias = "cla")]
    VndClaymore,
    #[doc = "application/vnd.cloanto.rp9"]
    #[serde(rename = "application/vnd.cloanto.rp9")]
    #[serde(alias = "rp9")]
    VndCloantoRp9,
    #[doc = "application/vnd.clonk.c4group"]
    #[serde(rename = "application/vnd.clonk.c4group")]
    #[serde(alias = "c4g")]
    #[serde(alias = "c4d")]
    #[serde(alias = "c4f")]
    #[serde(alias = "c4p")]
    #[serde(alias = "c4u")]
    VndClonkC4Group,
    #[doc = "application/vnd.cluetrust.cartomobile-config"]
    #[serde(rename = "application/vnd.cluetrust.cartomobile-config")]
    #[serde(alias = "c11amc")]
    VndCluetrustCartomobileConfig,
    #[doc = "application/vnd.cluetrust.cartomobile-config-pkg"]
    #[serde(rename = "application/vnd.cluetrust.cartomobile-config-pkg")]
    #[serde(alias = "c11amz")]
    VndCluetrustCartomobileConfigPkg,
    #[doc = "application/vnd.cncf.helm.chart.content.v1.tar+gzip"]
    #[serde(rename = "application/vnd.cncf.helm.chart.content.v1.tar+gzip")]
    VndCncfHelmChartContentV1TarGzip,
    #[doc = "application/vnd.cncf.helm.chart.provenance.v1.prov"]
    #[serde(rename = "application/vnd.cncf.helm.chart.provenance.v1.prov")]
    VndCncfHelmChartProvenanceV1Prov,
    #[doc = "application/vnd.cncf.helm.config.v1+json"]
    #[serde(rename = "application/vnd.cncf.helm.config.v1+json")]
    VndCncfHelmConfigV1Json,
    #[doc = "application/vnd.coffeescript"]
    #[serde(rename = "application/vnd.coffeescript")]
    #[serde(alias = "coffee")]
    VndCoffeescript,
    #[doc = "application/vnd.collabio.xodocuments.document"]
    #[serde(rename = "application/vnd.collabio.xodocuments.document")]
    #[serde(alias = "xodt")]
    VndCollabioXodocumentsDocument,
    #[doc = "application/vnd.collabio.xodocuments.document-template"]
    #[serde(rename = "application/vnd.collabio.xodocuments.document-template")]
    #[serde(alias = "xott")]
    VndCollabioXodocumentsDocumentTemplate,
    #[doc = "application/vnd.collabio.xodocuments.presentation"]
    #[serde(rename = "application/vnd.collabio.xodocuments.presentation")]
    #[serde(alias = "xodp")]
    VndCollabioXodocumentsPresentation,
    #[doc = "application/vnd.collabio.xodocuments.presentation-template"]
    #[serde(rename = "application/vnd.collabio.xodocuments.presentation-template")]
    #[serde(alias = "xotp")]
    VndCollabioXodocumentsPresentationTemplate,
    #[doc = "application/vnd.collabio.xodocuments.spreadsheet"]
    #[serde(rename = "application/vnd.collabio.xodocuments.spreadsheet")]
    #[serde(alias = "xods")]
    VndCollabioXodocumentsSpreadsheet,
    #[doc = "application/vnd.collabio.xodocuments.spreadsheet-template"]
    #[serde(rename = "application/vnd.collabio.xodocuments.spreadsheet-template")]
    #[serde(alias = "xots")]
    VndCollabioXodocumentsSpreadsheetTemplate,
    #[doc = "application/vnd.collection.doc+json"]
    #[serde(rename = "application/vnd.collection.doc+json")]
    VndCollectionDocJson,
    #[doc = "application/vnd.collection+json"]
    #[serde(rename = "application/vnd.collection+json")]
    VndCollectionJson,
    #[doc = "application/vnd.collection.next+json"]
    #[serde(rename = "application/vnd.collection.next+json")]
    VndCollectionNextJson,
    #[doc = "application/vnd.comicbook-rar"]
    #[serde(rename = "application/vnd.comicbook-rar")]
    #[serde(alias = "cbr")]
    VndComicbookRar,
    #[doc = "application/vnd.comicbook+zip"]
    #[serde(rename = "application/vnd.comicbook+zip")]
    #[serde(alias = "cbz")]
    VndComicbookZip,
    #[doc = "application/vnd.commerce-battelle"]
    #[serde(rename = "application/vnd.commerce-battelle")]
    #[serde(alias = "ica")]
    #[serde(alias = "icf")]
    #[serde(alias = "icd")]
    #[serde(alias = "ic0")]
    #[serde(alias = "ic1")]
    #[serde(alias = "ic2")]
    #[serde(alias = "ic3")]
    #[serde(alias = "ic4")]
    #[serde(alias = "ic5")]
    #[serde(alias = "ic6")]
    #[serde(alias = "ic7")]
    #[serde(alias = "ic8")]
    VndCommerceBattelle,
    #[doc = "application/vnd.commonspace"]
    #[serde(rename = "application/vnd.commonspace")]
    #[serde(alias = "csp")]
    #[serde(alias = "cst")]
    VndCommonspace,
    #[doc = "application/vnd.coreos.ignition+json"]
    #[serde(rename = "application/vnd.coreos.ignition+json")]
    #[serde(alias = "ign")]
    #[serde(alias = "ignition")]
    VndCoreosIgnitionJson,
    #[doc = "application/vnd.cosmocaller"]
    #[serde(rename = "application/vnd.cosmocaller")]
    #[serde(alias = "cmc")]
    VndCosmocaller,
    #[doc = "application/vnd.contact.cmsg"]
    #[serde(rename = "application/vnd.contact.cmsg")]
    #[serde(alias = "cdbcmsg")]
    VndContactCmsg,
    #[doc = "application/vnd.crick.clicker"]
    #[serde(rename = "application/vnd.crick.clicker")]
    #[serde(alias = "clkx")]
    VndCrickClicker,
    #[doc = "application/vnd.crick.clicker.keyboard"]
    #[serde(rename = "application/vnd.crick.clicker.keyboard")]
    #[serde(alias = "clkk")]
    VndCrickClickerKeyboard,
    #[doc = "application/vnd.crick.clicker.palette"]
    #[serde(rename = "application/vnd.crick.clicker.palette")]
    #[serde(alias = "clkp")]
    VndCrickClickerPalette,
    #[doc = "application/vnd.crick.clicker.template"]
    #[serde(rename = "application/vnd.crick.clicker.template")]
    #[serde(alias = "clkt")]
    VndCrickClickerTemplate,
    #[doc = "application/vnd.crick.clicker.wordbank"]
    #[serde(rename = "application/vnd.crick.clicker.wordbank")]
    #[serde(alias = "clkw")]
    VndCrickClickerWordbank,
    #[doc = "application/vnd.criticaltools.wbs+xml"]
    #[serde(rename = "application/vnd.criticaltools.wbs+xml")]
    #[serde(alias = "wbs")]
    VndCriticaltoolsWbsXml,
    #[doc = "application/vnd.cryptii.pipe+json"]
    #[serde(rename = "application/vnd.cryptii.pipe+json")]
    VndCryptiiPipeJson,
    #[doc = "application/vnd.crypto-shade-file"]
    #[serde(rename = "application/vnd.crypto-shade-file")]
    #[serde(alias = "ssvc")]
    VndCryptoShadeFile,
    #[doc = "application/vnd.cryptomator.encrypted"]
    #[serde(rename = "application/vnd.cryptomator.encrypted")]
    #[serde(alias = "c9r")]
    #[serde(alias = "c9s")]
    VndCryptomatorEncrypted,
    #[doc = "application/vnd.cryptomator.vault"]
    #[serde(rename = "application/vnd.cryptomator.vault")]
    #[serde(alias = "cryptomator")]
    VndCryptomatorVault,
    #[doc = "application/vnd.ctc-posml"]
    #[serde(rename = "application/vnd.ctc-posml")]
    #[serde(alias = "pml")]
    VndCtcPosml,
    #[doc = "application/vnd.ctct.ws+xml"]
    #[serde(rename = "application/vnd.ctct.ws+xml")]
    VndCtctWsXml,
    #[doc = "application/vnd.cups-pdf"]
    #[serde(rename = "application/vnd.cups-pdf")]
    VndCupsPdf,
    #[doc = "application/vnd.cups-postscript"]
    #[serde(rename = "application/vnd.cups-postscript")]
    VndCupsPostscript,
    #[doc = "application/vnd.cups-ppd"]
    #[serde(rename = "application/vnd.cups-ppd")]
    #[serde(alias = "ppd")]
    VndCupsPpd,
    #[doc = "application/vnd.cups-raster"]
    #[serde(rename = "application/vnd.cups-raster")]
    VndCupsRaster,
    #[doc = "application/vnd.cups-raw"]
    #[serde(rename = "application/vnd.cups-raw")]
    VndCupsRaw,
    #[doc = "application/vnd.curl"]
    #[serde(rename = "application/vnd.curl")]
    #[serde(alias = "curl")]
    VndCurl,
    #[doc = "application/vnd.cyan.dean.root+xml"]
    #[serde(rename = "application/vnd.cyan.dean.root+xml")]
    VndCyanDeanRootXml,
    #[doc = "application/vnd.cybank"]
    #[serde(rename = "application/vnd.cybank")]
    VndCybank,
    #[doc = "application/vnd.cyclonedx+json"]
    #[serde(rename = "application/vnd.cyclonedx+json")]
    VndCyclonedxJson,
    #[doc = "application/vnd.cyclonedx+xml"]
    #[serde(rename = "application/vnd.cyclonedx+xml")]
    VndCyclonedxXml,
    #[doc = "application/vnd.d2l.coursepackage1p0+zip"]
    #[serde(rename = "application/vnd.d2l.coursepackage1p0+zip")]
    VndD2LCoursepackage1P0Zip,
    #[doc = "application/vnd.d3m-dataset"]
    #[serde(rename = "application/vnd.d3m-dataset")]
    VndD3MDataset,
    #[doc = "application/vnd.d3m-problem"]
    #[serde(rename = "application/vnd.d3m-problem")]
    VndD3MProblem,
    #[doc = "application/vnd.dart"]
    #[serde(rename = "application/vnd.dart")]
    #[serde(alias = "dart")]
    VndDart,
    #[doc = "application/vnd.data-vision.rdz"]
    #[serde(rename = "application/vnd.data-vision.rdz")]
    #[serde(alias = "rdz")]
    VndDataVisionRdz,
    #[doc = "application/vnd.datalog"]
    #[serde(rename = "application/vnd.datalog")]
    #[serde(alias = "dl")]
    VndDatalog,
    #[doc = "application/vnd.datapackage+json"]
    #[serde(rename = "application/vnd.datapackage+json")]
    VndDatapackageJson,
    #[doc = "application/vnd.dataresource+json"]
    #[serde(rename = "application/vnd.dataresource+json")]
    VndDataresourceJson,
    #[doc = "application/vnd.dbf"]
    #[serde(rename = "application/vnd.dbf")]
    #[serde(alias = "dbf")]
    VndDbf,
    #[doc = "application/vnd.debian.binary-package"]
    #[serde(rename = "application/vnd.debian.binary-package")]
    #[serde(alias = "deb")]
    #[serde(alias = "udeb")]
    VndDebianBinaryPackage,
    #[doc = "application/vnd.dece.data"]
    #[serde(rename = "application/vnd.dece.data")]
    #[serde(alias = "uvf")]
    #[serde(alias = "uvvf")]
    #[serde(alias = "uvd")]
    #[serde(alias = "uvvd")]
    VndDeceData,
    #[doc = "application/vnd.dece.ttml+xml"]
    #[serde(rename = "application/vnd.dece.ttml+xml")]
    #[serde(alias = "uvt")]
    #[serde(alias = "uvvt")]
    VndDeceTtmlXml,
    #[doc = "application/vnd.dece.unspecified"]
    #[serde(rename = "application/vnd.dece.unspecified")]
    #[serde(alias = "uvx")]
    #[serde(alias = "uvvx")]
    VndDeceUnspecified,
    #[doc = "application/vnd.dece.zip"]
    #[serde(rename = "application/vnd.dece.zip")]
    #[serde(alias = "uvz")]
    #[serde(alias = "uvvz")]
    VndDeceZip,
    #[doc = "application/vnd.denovo.fcselayout-link"]
    #[serde(rename = "application/vnd.denovo.fcselayout-link")]
    #[serde(alias = "fe_launch")]
    VndDenovoFcselayoutLink,
    #[doc = "application/vnd.desmume.movie"]
    #[serde(rename = "application/vnd.desmume.movie")]
    #[serde(alias = "dsm")]
    VndDesmumeMovie,
    #[doc = "application/vnd.dir-bi.plate-dl-nosuffix"]
    #[serde(rename = "application/vnd.dir-bi.plate-dl-nosuffix")]
    VndDirBiPlateDlNosuffix,
    #[doc = "application/vnd.dm.delegation+xml"]
    #[serde(rename = "application/vnd.dm.delegation+xml")]
    VndDmDelegationXml,
    #[doc = "application/vnd.dna"]
    #[serde(rename = "application/vnd.dna")]
    #[serde(alias = "dna")]
    VndDna,
    #[doc = "application/vnd.document+json"]
    #[serde(rename = "application/vnd.document+json")]
    #[serde(alias = "docjson")]
    VndDocumentJson,
    #[doc = "application/vnd.dolby.mobile.1"]
    #[serde(rename = "application/vnd.dolby.mobile.1")]
    VndDolbyMobile1,
    #[doc = "application/vnd.dolby.mobile.2"]
    #[serde(rename = "application/vnd.dolby.mobile.2")]
    VndDolbyMobile2,
    #[doc = "application/vnd.doremir.scorecloud-binary-document"]
    #[serde(rename = "application/vnd.doremir.scorecloud-binary-document")]
    #[serde(alias = "scld")]
    VndDoremirScorecloudBinaryDocument,
    #[doc = "application/vnd.dpgraph"]
    #[serde(rename = "application/vnd.dpgraph")]
    #[serde(alias = "dpg")]
    #[serde(alias = "mwc")]
    #[serde(alias = "dpgraph")]
    VndDpgraph,
    #[doc = "application/vnd.dreamfactory"]
    #[serde(rename = "application/vnd.dreamfactory")]
    #[serde(alias = "dfac")]
    VndDreamfactory,
    #[doc = "application/vnd.drive+json"]
    #[serde(rename = "application/vnd.drive+json")]
    VndDriveJson,
    #[doc = "application/vnd.dtg.local"]
    #[serde(rename = "application/vnd.dtg.local")]
    VndDtgLocal,
    #[doc = "application/vnd.dtg.local.flash"]
    #[serde(rename = "application/vnd.dtg.local.flash")]
    #[serde(alias = "fla")]
    VndDtgLocalFlash,
    #[doc = "application/vnd.dtg.local.html"]
    #[serde(rename = "application/vnd.dtg.local.html")]
    VndDtgLocalHtml,
    #[doc = "application/vnd.dvb.ait"]
    #[serde(rename = "application/vnd.dvb.ait")]
    #[serde(alias = "ait")]
    VndDvbAit,
    #[doc = "application/vnd.dvb.dvbisl+xml"]
    #[serde(rename = "application/vnd.dvb.dvbisl+xml")]
    VndDvbDvbislXml,
    #[doc = "application/vnd.dvb.dvbj"]
    #[serde(rename = "application/vnd.dvb.dvbj")]
    VndDvbDvbj,
    #[doc = "application/vnd.dvb.esgcontainer"]
    #[serde(rename = "application/vnd.dvb.esgcontainer")]
    VndDvbEsgcontainer,
    #[doc = "application/vnd.dvb.ipdcdftnotifaccess"]
    #[serde(rename = "application/vnd.dvb.ipdcdftnotifaccess")]
    VndDvbIpdcdftnotifaccess,
    #[doc = "application/vnd.dvb.ipdcesgaccess"]
    #[serde(rename = "application/vnd.dvb.ipdcesgaccess")]
    VndDvbIpdcesgaccess,
    #[doc = "application/vnd.dvb.ipdcesgaccess2"]
    #[serde(rename = "application/vnd.dvb.ipdcesgaccess2")]
    VndDvbIpdcesgaccess2,
    #[doc = "application/vnd.dvb.ipdcesgpdd"]
    #[serde(rename = "application/vnd.dvb.ipdcesgpdd")]
    VndDvbIpdcesgpdd,
    #[doc = "application/vnd.dvb.ipdcroaming"]
    #[serde(rename = "application/vnd.dvb.ipdcroaming")]
    VndDvbIpdcroaming,
    #[doc = "application/vnd.dvb.iptv.alfec-base"]
    #[serde(rename = "application/vnd.dvb.iptv.alfec-base")]
    VndDvbIptvAlfecBase,
    #[doc = "application/vnd.dvb.iptv.alfec-enhancement"]
    #[serde(rename = "application/vnd.dvb.iptv.alfec-enhancement")]
    VndDvbIptvAlfecEnhancement,
    #[doc = "application/vnd.dvb.notif-aggregate-root+xml"]
    #[serde(rename = "application/vnd.dvb.notif-aggregate-root+xml")]
    VndDvbNotifAggregateRootXml,
    #[doc = "application/vnd.dvb.notif-container+xml"]
    #[serde(rename = "application/vnd.dvb.notif-container+xml")]
    VndDvbNotifContainerXml,
    #[doc = "application/vnd.dvb.notif-generic+xml"]
    #[serde(rename = "application/vnd.dvb.notif-generic+xml")]
    VndDvbNotifGenericXml,
    #[doc = "application/vnd.dvb.notif-ia-msglist+xml"]
    #[serde(rename = "application/vnd.dvb.notif-ia-msglist+xml")]
    VndDvbNotifIaMsglistXml,
    #[doc = "application/vnd.dvb.notif-ia-registration-request+xml"]
    #[serde(rename = "application/vnd.dvb.notif-ia-registration-request+xml")]
    VndDvbNotifIaRegistrationRequestXml,
    #[doc = "application/vnd.dvb.notif-ia-registration-response+xml"]
    #[serde(rename = "application/vnd.dvb.notif-ia-registration-response+xml")]
    VndDvbNotifIaRegistrationResponseXml,
    #[doc = "application/vnd.dvb.notif-init+xml"]
    #[serde(rename = "application/vnd.dvb.notif-init+xml")]
    VndDvbNotifInitXml,
    #[doc = "application/vnd.dvb.pfr"]
    #[serde(rename = "application/vnd.dvb.pfr")]
    VndDvbPfr,
    #[doc = "application/vnd.dvb.service"]
    #[serde(rename = "application/vnd.dvb.service")]
    #[serde(alias = "svc")]
    VndDvbService,
    #[doc = "application/vnd.dxr"]
    #[serde(rename = "application/vnd.dxr")]
    VndDxr,
    #[doc = "application/vnd.dynageo"]
    #[serde(rename = "application/vnd.dynageo")]
    #[serde(alias = "geo")]
    VndDynageo,
    #[doc = "application/vnd.dzr"]
    #[serde(rename = "application/vnd.dzr")]
    #[serde(alias = "dzr")]
    VndDzr,
    #[doc = "application/vnd.easykaraoke.cdgdownload"]
    #[serde(rename = "application/vnd.easykaraoke.cdgdownload")]
    VndEasykaraokeCdgdownload,
    #[doc = "application/vnd.ecip.rlp"]
    #[serde(rename = "application/vnd.ecip.rlp")]
    VndEcipRlp,
    #[doc = "application/vnd.ecdis-update"]
    #[serde(rename = "application/vnd.ecdis-update")]
    VndEcdisUpdate,
    #[doc = "application/vnd.eclipse.ditto+json"]
    #[serde(rename = "application/vnd.eclipse.ditto+json")]
    VndEclipseDittoJson,
    #[doc = "application/vnd.ecowin.chart"]
    #[serde(rename = "application/vnd.ecowin.chart")]
    #[serde(alias = "mag")]
    VndEcowinChart,
    #[doc = "application/vnd.ecowin.filerequest"]
    #[serde(rename = "application/vnd.ecowin.filerequest")]
    VndEcowinFilerequest,
    #[doc = "application/vnd.ecowin.fileupdate"]
    #[serde(rename = "application/vnd.ecowin.fileupdate")]
    VndEcowinFileupdate,
    #[doc = "application/vnd.ecowin.series"]
    #[serde(rename = "application/vnd.ecowin.series")]
    VndEcowinSeries,
    #[doc = "application/vnd.ecowin.seriesrequest"]
    #[serde(rename = "application/vnd.ecowin.seriesrequest")]
    VndEcowinSeriesrequest,
    #[doc = "application/vnd.ecowin.seriesupdate"]
    #[serde(rename = "application/vnd.ecowin.seriesupdate")]
    VndEcowinSeriesupdate,
    #[doc = "application/vnd.efi.img"]
    #[serde(rename = "application/vnd.efi.img")]
    VndEfiImg,
    #[doc = "application/vnd.efi.iso"]
    #[serde(rename = "application/vnd.efi.iso")]
    VndEfiIso,
    #[doc = "application/vnd.eln+zip"]
    #[serde(rename = "application/vnd.eln+zip")]
    #[serde(alias = "eln")]
    VndElnZip,
    #[doc = "application/vnd.emclient.accessrequest+xml"]
    #[serde(rename = "application/vnd.emclient.accessrequest+xml")]
    VndEmclientAccessrequestXml,
    #[doc = "application/vnd.enliven"]
    #[serde(rename = "application/vnd.enliven")]
    #[serde(alias = "nml")]
    VndEnliven,
    #[doc = "application/vnd.enphase.envoy"]
    #[serde(rename = "application/vnd.enphase.envoy")]
    VndEnphaseEnvoy,
    #[doc = "application/vnd.eprints.data+xml"]
    #[serde(rename = "application/vnd.eprints.data+xml")]
    VndEprintsDataXml,
    #[doc = "application/vnd.epson.esf"]
    #[serde(rename = "application/vnd.epson.esf")]
    #[serde(alias = "esf")]
    VndEpsonEsf,
    #[doc = "application/vnd.epson.msf"]
    #[serde(rename = "application/vnd.epson.msf")]
    #[serde(alias = "msf")]
    VndEpsonMsf,
    #[doc = "application/vnd.epson.quickanime"]
    #[serde(rename = "application/vnd.epson.quickanime")]
    #[serde(alias = "qam")]
    VndEpsonQuickanime,
    #[doc = "application/vnd.epson.salt"]
    #[serde(rename = "application/vnd.epson.salt")]
    #[serde(alias = "slt")]
    VndEpsonSalt,
    #[doc = "application/vnd.epson.ssf"]
    #[serde(rename = "application/vnd.epson.ssf")]
    #[serde(alias = "ssf")]
    VndEpsonSsf,
    #[doc = "application/vnd.ericsson.quickcall"]
    #[serde(rename = "application/vnd.ericsson.quickcall")]
    #[serde(alias = "qcall")]
    #[serde(alias = "qca")]
    VndEricssonQuickcall,
    #[doc = "application/vnd.espass-espass+zip"]
    #[serde(rename = "application/vnd.espass-espass+zip")]
    #[serde(alias = "espass")]
    VndEspassEspassZip,
    #[doc = "application/vnd.eszigno3+xml"]
    #[serde(rename = "application/vnd.eszigno3+xml")]
    #[serde(alias = "es3")]
    #[serde(alias = "et3")]
    VndEszigno3Xml,
    #[doc = "application/vnd.etsi.aoc+xml"]
    #[serde(rename = "application/vnd.etsi.aoc+xml")]
    VndEtsiAocXml,
    #[doc = "application/vnd.etsi.asic-s+zip"]
    #[serde(rename = "application/vnd.etsi.asic-s+zip")]
    #[serde(alias = "asics")]
    VndEtsiAsicSZip,
    #[doc = "application/vnd.etsi.asic-e+zip"]
    #[serde(rename = "application/vnd.etsi.asic-e+zip")]
    #[serde(alias = "asice")]
    #[serde(alias = "sce")]
    VndEtsiAsicEZip,
    #[doc = "application/vnd.etsi.cug+xml"]
    #[serde(rename = "application/vnd.etsi.cug+xml")]
    VndEtsiCugXml,
    #[doc = "application/vnd.etsi.iptvcommand+xml"]
    #[serde(rename = "application/vnd.etsi.iptvcommand+xml")]
    VndEtsiIptvcommandXml,
    #[doc = "application/vnd.etsi.iptvdiscovery+xml"]
    #[serde(rename = "application/vnd.etsi.iptvdiscovery+xml")]
    VndEtsiIptvdiscoveryXml,
    #[doc = "application/vnd.etsi.iptvprofile+xml"]
    #[serde(rename = "application/vnd.etsi.iptvprofile+xml")]
    VndEtsiIptvprofileXml,
    #[doc = "application/vnd.etsi.iptvsad-bc+xml"]
    #[serde(rename = "application/vnd.etsi.iptvsad-bc+xml")]
    VndEtsiIptvsadBcXml,
    #[doc = "application/vnd.etsi.iptvsad-cod+xml"]
    #[serde(rename = "application/vnd.etsi.iptvsad-cod+xml")]
    VndEtsiIptvsadCodXml,
    #[doc = "application/vnd.etsi.iptvsad-npvr+xml"]
    #[serde(rename = "application/vnd.etsi.iptvsad-npvr+xml")]
    VndEtsiIptvsadNpvrXml,
    #[doc = "application/vnd.etsi.iptvservice+xml"]
    #[serde(rename = "application/vnd.etsi.iptvservice+xml")]
    VndEtsiIptvserviceXml,
    #[doc = "application/vnd.etsi.iptvsync+xml"]
    #[serde(rename = "application/vnd.etsi.iptvsync+xml")]
    VndEtsiIptvsyncXml,
    #[doc = "application/vnd.etsi.iptvueprofile+xml"]
    #[serde(rename = "application/vnd.etsi.iptvueprofile+xml")]
    VndEtsiIptvueprofileXml,
    #[doc = "application/vnd.etsi.mcid+xml"]
    #[serde(rename = "application/vnd.etsi.mcid+xml")]
    VndEtsiMcidXml,
    #[doc = "application/vnd.etsi.mheg5"]
    #[serde(rename = "application/vnd.etsi.mheg5")]
    VndEtsiMheg5,
    #[doc = "application/vnd.etsi.overload-control-policy-dataset+xml"]
    #[serde(rename = "application/vnd.etsi.overload-control-policy-dataset+xml")]
    VndEtsiOverloadControlPolicyDatasetXml,
    #[doc = "application/vnd.etsi.pstn+xml"]
    #[serde(rename = "application/vnd.etsi.pstn+xml")]
    VndEtsiPstnXml,
    #[doc = "application/vnd.etsi.sci+xml"]
    #[serde(rename = "application/vnd.etsi.sci+xml")]
    VndEtsiSciXml,
    #[doc = "application/vnd.etsi.simservs+xml"]
    #[serde(rename = "application/vnd.etsi.simservs+xml")]
    VndEtsiSimservsXml,
    #[doc = "application/vnd.etsi.timestamp-token"]
    #[serde(rename = "application/vnd.etsi.timestamp-token")]
    #[serde(alias = "tst")]
    VndEtsiTimestampToken,
    #[doc = "application/vnd.etsi.tsl+xml"]
    #[serde(rename = "application/vnd.etsi.tsl+xml")]
    VndEtsiTslXml,
    #[doc = "application/vnd.etsi.tsl.der"]
    #[serde(rename = "application/vnd.etsi.tsl.der")]
    VndEtsiTslDer,
    #[doc = "application/vnd.eu.kasparian.car+json"]
    #[serde(rename = "application/vnd.eu.kasparian.car+json")]
    #[serde(alias = "carjson")]
    VndEuKasparianCarJson,
    #[doc = "application/vnd.eudora.data"]
    #[serde(rename = "application/vnd.eudora.data")]
    VndEudoraData,
    #[doc = "application/vnd.evolv.ecig.profile"]
    #[serde(rename = "application/vnd.evolv.ecig.profile")]
    #[serde(alias = "ecigprofile")]
    VndEvolvEcigProfile,
    #[doc = "application/vnd.evolv.ecig.settings"]
    #[serde(rename = "application/vnd.evolv.ecig.settings")]
    #[serde(alias = "ecig")]
    VndEvolvEcigSettings,
    #[doc = "application/vnd.evolv.ecig.theme"]
    #[serde(rename = "application/vnd.evolv.ecig.theme")]
    #[serde(alias = "ecigtheme")]
    VndEvolvEcigTheme,
    #[doc = "application/vnd.exstream-empower+zip"]
    #[serde(rename = "application/vnd.exstream-empower+zip")]
    #[serde(alias = "mpw")]
    VndExstreamEmpowerZip,
    #[doc = "application/vnd.exstream-package"]
    #[serde(rename = "application/vnd.exstream-package")]
    #[serde(alias = "pub")]
    VndExstreamPackage,
    #[doc = "application/vnd.ezpix-album"]
    #[serde(rename = "application/vnd.ezpix-album")]
    #[serde(alias = "ez2")]
    VndEzpixAlbum,
    #[doc = "application/vnd.ezpix-package"]
    #[serde(rename = "application/vnd.ezpix-package")]
    #[serde(alias = "ez3")]
    VndEzpixPackage,
    #[doc = "application/vnd.f-secure.mobile"]
    #[serde(rename = "application/vnd.f-secure.mobile")]
    VndFSecureMobile,
    #[doc = "application/vnd.fastcopy-disk-image"]
    #[serde(rename = "application/vnd.fastcopy-disk-image")]
    #[serde(alias = "dim")]
    VndFastcopyDiskImage,
    #[doc = "application/vnd.familysearch.gedcom+zip"]
    #[serde(rename = "application/vnd.familysearch.gedcom+zip")]
    #[serde(alias = "gdz")]
    VndFamilysearchGedcomZip,
    #[doc = "application/vnd.fdsn.mseed"]
    #[serde(rename = "application/vnd.fdsn.mseed")]
    #[serde(alias = "msd")]
    #[serde(alias = "mseed")]
    VndFdsnMseed,
    #[doc = "application/vnd.fdsn.seed"]
    #[serde(rename = "application/vnd.fdsn.seed")]
    #[serde(alias = "seed")]
    #[serde(alias = "dataless")]
    VndFdsnSeed,
    #[doc = "application/vnd.ffsns"]
    #[serde(rename = "application/vnd.ffsns")]
    VndFfsns,
    #[doc = "application/vnd.ficlab.flb+zip"]
    #[serde(rename = "application/vnd.ficlab.flb+zip")]
    #[serde(alias = "flb")]
    VndFiclabFlbZip,
    #[doc = "application/vnd.filmit.zfc"]
    #[serde(rename = "application/vnd.filmit.zfc")]
    #[serde(alias = "zfc")]
    VndFilmitZfc,
    #[doc = "application/vnd.fints"]
    #[serde(rename = "application/vnd.fints")]
    VndFints,
    #[doc = "application/vnd.firemonkeys.cloudcell"]
    #[serde(rename = "application/vnd.firemonkeys.cloudcell")]
    VndFiremonkeysCloudcell,
    #[doc = "application/vnd.FloGraphIt"]
    #[serde(rename = "application/vnd.FloGraphIt")]
    #[serde(alias = "gph")]
    VndFloGraphIt,
    #[doc = "application/vnd.fluxtime.clip"]
    #[serde(rename = "application/vnd.fluxtime.clip")]
    #[serde(alias = "ftc")]
    VndFluxtimeClip,
    #[doc = "application/vnd.font-fontforge-sfd"]
    #[serde(rename = "application/vnd.font-fontforge-sfd")]
    #[serde(alias = "sfd")]
    VndFontFontforgeSfd,
    #[doc = "application/vnd.framemaker"]
    #[serde(rename = "application/vnd.framemaker")]
    #[serde(alias = "fm")]
    VndFramemaker,
    #[doc = "application/vnd.fsc.weblaunch"]
    #[serde(rename = "application/vnd.fsc.weblaunch")]
    #[serde(alias = "fsc")]
    VndFscWeblaunch,
    #[doc = "application/vnd.fujifilm.fb.docuworks"]
    #[serde(rename = "application/vnd.fujifilm.fb.docuworks")]
    VndFujifilmFbDocuworks,
    #[doc = "application/vnd.fujifilm.fb.docuworks.binder"]
    #[serde(rename = "application/vnd.fujifilm.fb.docuworks.binder")]
    VndFujifilmFbDocuworksBinder,
    #[doc = "application/vnd.fujifilm.fb.docuworks.container"]
    #[serde(rename = "application/vnd.fujifilm.fb.docuworks.container")]
    VndFujifilmFbDocuworksContainer,
    #[doc = "application/vnd.fujifilm.fb.jfi+xml"]
    #[serde(rename = "application/vnd.fujifilm.fb.jfi+xml")]
    VndFujifilmFbJfiXml,
    #[doc = "application/vnd.fujitsu.oasys"]
    #[serde(rename = "application/vnd.fujitsu.oasys")]
    #[serde(alias = "oas")]
    VndFujitsuOasys,
    #[doc = "application/vnd.fujitsu.oasys2"]
    #[serde(rename = "application/vnd.fujitsu.oasys2")]
    #[serde(alias = "oa2")]
    VndFujitsuOasys2,
    #[doc = "application/vnd.fujitsu.oasys3"]
    #[serde(rename = "application/vnd.fujitsu.oasys3")]
    #[serde(alias = "oa3")]
    VndFujitsuOasys3,
    #[doc = "application/vnd.fujitsu.oasysgp"]
    #[serde(rename = "application/vnd.fujitsu.oasysgp")]
    #[serde(alias = "fg5")]
    VndFujitsuOasysgp,
    #[doc = "application/vnd.fujitsu.oasysprs"]
    #[serde(rename = "application/vnd.fujitsu.oasysprs")]
    #[serde(alias = "bh2")]
    VndFujitsuOasysprs,
    #[doc = "application/vnd.fujixerox.ART4"]
    #[serde(rename = "application/vnd.fujixerox.ART4")]
    VndFujixeroxART4,
    #[doc = "application/vnd.fujixerox.ART-EX"]
    #[serde(rename = "application/vnd.fujixerox.ART-EX")]
    VndFujixeroxARTEX,
    #[doc = "application/vnd.fujixerox.ddd"]
    #[serde(rename = "application/vnd.fujixerox.ddd")]
    #[serde(alias = "ddd")]
    VndFujixeroxDdd,
    #[doc = "application/vnd.fujixerox.docuworks"]
    #[serde(rename = "application/vnd.fujixerox.docuworks")]
    #[serde(alias = "xdw")]
    VndFujixeroxDocuworks,
    #[doc = "application/vnd.fujixerox.docuworks.binder"]
    #[serde(rename = "application/vnd.fujixerox.docuworks.binder")]
    #[serde(alias = "xbd")]
    VndFujixeroxDocuworksBinder,
    #[doc = "application/vnd.fujixerox.docuworks.container"]
    #[serde(rename = "application/vnd.fujixerox.docuworks.container")]
    #[serde(alias = "xct")]
    VndFujixeroxDocuworksContainer,
    #[doc = "application/vnd.fujixerox.HBPL"]
    #[serde(rename = "application/vnd.fujixerox.HBPL")]
    VndFujixeroxHBPL,
    #[doc = "application/vnd.fut-misnet"]
    #[serde(rename = "application/vnd.fut-misnet")]
    VndFutMisnet,
    #[doc = "application/vnd.futoin+cbor"]
    #[serde(rename = "application/vnd.futoin+cbor")]
    VndFutoinCbor,
    #[doc = "application/vnd.futoin+json"]
    #[serde(rename = "application/vnd.futoin+json")]
    VndFutoinJson,
    #[doc = "application/vnd.fuzzysheet"]
    #[serde(rename = "application/vnd.fuzzysheet")]
    #[serde(alias = "fzs")]
    VndFuzzysheet,
    #[doc = "application/vnd.genomatix.tuxedo"]
    #[serde(rename = "application/vnd.genomatix.tuxedo")]
    #[serde(alias = "txd")]
    VndGenomatixTuxedo,
    #[doc = "application/vnd.genozip"]
    #[serde(rename = "application/vnd.genozip")]
    #[serde(alias = "genozip")]
    VndGenozip,
    #[doc = "application/vnd.gentics.grd+json"]
    #[serde(rename = "application/vnd.gentics.grd+json")]
    VndGenticsGrdJson,
    #[doc = "application/vnd.gentoo.catmetadata+xml"]
    #[serde(rename = "application/vnd.gentoo.catmetadata+xml")]
    VndGentooCatmetadataXml,
    #[doc = "application/vnd.gentoo.ebuild"]
    #[serde(rename = "application/vnd.gentoo.ebuild")]
    #[serde(alias = "ebuild")]
    VndGentooEbuild,
    #[doc = "application/vnd.gentoo.eclass"]
    #[serde(rename = "application/vnd.gentoo.eclass")]
    #[serde(alias = "eclass")]
    VndGentooEclass,
    #[doc = "application/vnd.gentoo.gpkg"]
    #[serde(rename = "application/vnd.gentoo.gpkg")]
    #[serde(alias = "gpkg.tar")]
    VndGentooGpkg,
    #[doc = "application/vnd.gentoo.manifest"]
    #[serde(rename = "application/vnd.gentoo.manifest")]
    VndGentooManifest,
    #[doc = "application/vnd.gentoo.xpak"]
    #[serde(rename = "application/vnd.gentoo.xpak")]
    #[serde(alias = "tbz2")]
    #[serde(alias = "xpak")]
    VndGentooXpak,
    #[doc = "application/vnd.gentoo.pkgmetadata+xml"]
    #[serde(rename = "application/vnd.gentoo.pkgmetadata+xml")]
    VndGentooPkgmetadataXml,
    #[doc = "application/vnd.geogebra.file"]
    #[serde(rename = "application/vnd.geogebra.file")]
    #[serde(alias = "ggb")]
    VndGeogebraFile,
    #[doc = "application/vnd.geogebra.slides"]
    #[serde(rename = "application/vnd.geogebra.slides")]
    #[serde(alias = "ggs")]
    VndGeogebraSlides,
    #[doc = "application/vnd.geogebra.tool"]
    #[serde(rename = "application/vnd.geogebra.tool")]
    #[serde(alias = "ggt")]
    VndGeogebraTool,
    #[doc = "application/vnd.geometry-explorer"]
    #[serde(rename = "application/vnd.geometry-explorer")]
    #[serde(alias = "gex")]
    #[serde(alias = "gre")]
    VndGeometryExplorer,
    #[doc = "application/vnd.geonext"]
    #[serde(rename = "application/vnd.geonext")]
    #[serde(alias = "gxt")]
    VndGeonext,
    #[doc = "application/vnd.geoplan"]
    #[serde(rename = "application/vnd.geoplan")]
    #[serde(alias = "g2w")]
    VndGeoplan,
    #[doc = "application/vnd.geospace"]
    #[serde(rename = "application/vnd.geospace")]
    #[serde(alias = "g3w")]
    VndGeospace,
    #[doc = "application/vnd.gerber"]
    #[serde(rename = "application/vnd.gerber")]
    VndGerber,
    #[doc = "application/vnd.globalplatform.card-content-mgt"]
    #[serde(rename = "application/vnd.globalplatform.card-content-mgt")]
    VndGlobalplatformCardContentMgt,
    #[doc = "application/vnd.globalplatform.card-content-mgt-response"]
    #[serde(rename = "application/vnd.globalplatform.card-content-mgt-response")]
    VndGlobalplatformCardContentMgtResponse,
    #[doc = "application/vnd.gnu.taler.exchange+json"]
    #[serde(rename = "application/vnd.gnu.taler.exchange+json")]
    VndGnuTalerExchangeJson,
    #[doc = "application/vnd.gnu.taler.merchant+json"]
    #[serde(rename = "application/vnd.gnu.taler.merchant+json")]
    VndGnuTalerMerchantJson,
    #[doc = "application/vnd.google-earth.kml+xml"]
    #[serde(rename = "application/vnd.google-earth.kml+xml")]
    #[serde(alias = "kml")]
    VndGoogleEarthKmlXml,
    #[doc = "application/vnd.google-earth.kmz"]
    #[serde(rename = "application/vnd.google-earth.kmz")]
    #[serde(alias = "kmz")]
    VndGoogleEarthKmz,
    #[doc = "application/vnd.gov.sk.e-form+xml"]
    #[serde(rename = "application/vnd.gov.sk.e-form+xml")]
    VndGovSkEFormXml,
    #[doc = "application/vnd.gov.sk.e-form+zip"]
    #[serde(rename = "application/vnd.gov.sk.e-form+zip")]
    VndGovSkEFormZip,
    #[doc = "application/vnd.gov.sk.xmldatacontainer+xml"]
    #[serde(rename = "application/vnd.gov.sk.xmldatacontainer+xml")]
    VndGovSkXmldatacontainerXml,
    #[doc = "application/vnd.gpxsee.map+xml"]
    #[serde(rename = "application/vnd.gpxsee.map+xml")]
    VndGpxseeMapXml,
    #[doc = "application/vnd.grafeq"]
    #[serde(rename = "application/vnd.grafeq")]
    #[serde(alias = "gqf")]
    #[serde(alias = "gqs")]
    VndGrafeq,
    #[doc = "application/vnd.gridmp"]
    #[serde(rename = "application/vnd.gridmp")]
    VndGridmp,
    #[doc = "application/vnd.groove-account"]
    #[serde(rename = "application/vnd.groove-account")]
    #[serde(alias = "gac")]
    VndGrooveAccount,
    #[doc = "application/vnd.groove-help"]
    #[serde(rename = "application/vnd.groove-help")]
    #[serde(alias = "ghf")]
    VndGrooveHelp,
    #[doc = "application/vnd.groove-identity-message"]
    #[serde(rename = "application/vnd.groove-identity-message")]
    #[serde(alias = "gim")]
    VndGrooveIdentityMessage,
    #[doc = "application/vnd.groove-injector"]
    #[serde(rename = "application/vnd.groove-injector")]
    #[serde(alias = "grv")]
    VndGrooveInjector,
    #[doc = "application/vnd.groove-tool-message"]
    #[serde(rename = "application/vnd.groove-tool-message")]
    #[serde(alias = "gtm")]
    VndGrooveToolMessage,
    #[doc = "application/vnd.groove-tool-template"]
    #[serde(rename = "application/vnd.groove-tool-template")]
    #[serde(alias = "tpl")]
    VndGrooveToolTemplate,
    #[doc = "application/vnd.groove-vcard"]
    #[serde(rename = "application/vnd.groove-vcard")]
    #[serde(alias = "vcg")]
    VndGrooveVcard,
    #[doc = "application/vnd.hal+json"]
    #[serde(rename = "application/vnd.hal+json")]
    VndHalJson,
    #[doc = "application/vnd.hal+xml"]
    #[serde(rename = "application/vnd.hal+xml")]
    #[serde(alias = "hal")]
    VndHalXml,
    #[doc = "application/vnd.HandHeld-Entertainment+xml"]
    #[serde(rename = "application/vnd.HandHeld-Entertainment+xml")]
    #[serde(alias = "zmm")]
    VndHandHeldEntertainmentXml,
    #[doc = "application/vnd.hbci"]
    #[serde(rename = "application/vnd.hbci")]
    #[serde(alias = "hbci")]
    #[serde(alias = "hbc")]
    #[serde(alias = "kom")]
    #[serde(alias = "upa")]
    #[serde(alias = "pkd")]
    #[serde(alias = "bpd")]
    VndHbci,
    #[doc = "application/vnd.hc+json"]
    #[serde(rename = "application/vnd.hc+json")]
    VndHcJson,
    #[doc = "application/vnd.hcl-bireports"]
    #[serde(rename = "application/vnd.hcl-bireports")]
    VndHclBireports,
    #[doc = "application/vnd.hdt"]
    #[serde(rename = "application/vnd.hdt")]
    #[serde(alias = "hdt")]
    VndHdt,
    #[doc = "application/vnd.heroku+json"]
    #[serde(rename = "application/vnd.heroku+json")]
    VndHerokuJson,
    #[doc = "application/vnd.hhe.lesson-player"]
    #[serde(rename = "application/vnd.hhe.lesson-player")]
    #[serde(alias = "les")]
    VndHheLessonPlayer,
    #[doc = "application/vnd.hp-HPGL"]
    #[serde(rename = "application/vnd.hp-HPGL")]
    #[serde(alias = "hpgl")]
    VndHpHPGL,
    #[doc = "application/vnd.hp-hpid"]
    #[serde(rename = "application/vnd.hp-hpid")]
    #[serde(alias = "hpi")]
    #[serde(alias = "hpid")]
    VndHpHpid,
    #[doc = "application/vnd.hp-hps"]
    #[serde(rename = "application/vnd.hp-hps")]
    #[serde(alias = "hps")]
    VndHpHps,
    #[doc = "application/vnd.hp-jlyt"]
    #[serde(rename = "application/vnd.hp-jlyt")]
    #[serde(alias = "jlt")]
    VndHpJlyt,
    #[doc = "application/vnd.hp-PCL"]
    #[serde(rename = "application/vnd.hp-PCL")]
    #[serde(alias = "pcl")]
    VndHpPCL,
    #[doc = "application/vnd.hp-PCLXL"]
    #[serde(rename = "application/vnd.hp-PCLXL")]
    VndHpPCLXL,
    #[doc = "application/vnd.hsl"]
    #[serde(rename = "application/vnd.hsl")]
    #[serde(alias = "hsl")]
    VndHsl,
    #[doc = "application/vnd.httphone"]
    #[serde(rename = "application/vnd.httphone")]
    VndHttphone,
    #[doc = "application/vnd.hydrostatix.sof-data"]
    #[serde(rename = "application/vnd.hydrostatix.sof-data")]
    #[serde(alias = "sfd-hdstx")]
    VndHydrostatixSofData,
    #[doc = "application/vnd.hyper-item+json"]
    #[serde(rename = "application/vnd.hyper-item+json")]
    VndHyperItemJson,
    #[doc = "application/vnd.hyper+json"]
    #[serde(rename = "application/vnd.hyper+json")]
    VndHyperJson,
    #[doc = "application/vnd.hyperdrive+json"]
    #[serde(rename = "application/vnd.hyperdrive+json")]
    VndHyperdriveJson,
    #[doc = "application/vnd.hzn-3d-crossword"]
    #[serde(rename = "application/vnd.hzn-3d-crossword")]
    #[serde(alias = "x3d")]
    VndHzn3DCrossword,
    #[doc = "application/vnd.ibm.electronic-media"]
    #[serde(rename = "application/vnd.ibm.electronic-media")]
    #[serde(alias = "emm")]
    VndIbmElectronicMedia,
    #[doc = "application/vnd.ibm.MiniPay"]
    #[serde(rename = "application/vnd.ibm.MiniPay")]
    #[serde(alias = "mpy")]
    VndIbmMiniPay,
    #[doc = "application/vnd.ibm.rights-management"]
    #[serde(rename = "application/vnd.ibm.rights-management")]
    #[serde(alias = "irm")]
    VndIbmRightsManagement,
    #[doc = "application/vnd.ibm.secure-container"]
    #[serde(rename = "application/vnd.ibm.secure-container")]
    #[serde(alias = "sc")]
    VndIbmSecureContainer,
    #[doc = "application/vnd.iccprofile"]
    #[serde(rename = "application/vnd.iccprofile")]
    #[serde(alias = "icc")]
    #[serde(alias = "icm")]
    VndIccprofile,
    #[doc = "application/vnd.ieee.1905"]
    #[serde(rename = "application/vnd.ieee.1905")]
    #[serde(alias = "1905.1")]
    VndIeee1905,
    #[doc = "application/vnd.igloader"]
    #[serde(rename = "application/vnd.igloader")]
    #[serde(alias = "igl")]
    VndIgloader,
    #[doc = "application/vnd.imagemeter.folder+zip"]
    #[serde(rename = "application/vnd.imagemeter.folder+zip")]
    #[serde(alias = "imf")]
    VndImagemeterFolderZip,
    #[doc = "application/vnd.imagemeter.image+zip"]
    #[serde(rename = "application/vnd.imagemeter.image+zip")]
    #[serde(alias = "imi")]
    VndImagemeterImageZip,
    #[doc = "application/vnd.immervision-ivp"]
    #[serde(rename = "application/vnd.immervision-ivp")]
    #[serde(alias = "ivp")]
    VndImmervisionIvp,
    #[doc = "application/vnd.immervision-ivu"]
    #[serde(rename = "application/vnd.immervision-ivu")]
    #[serde(alias = "ivu")]
    VndImmervisionIvu,
    #[doc = "application/vnd.ims.imsccv1p1"]
    #[serde(rename = "application/vnd.ims.imsccv1p1")]
    #[serde(alias = "imscc")]
    VndImsImsccv1P1,
    #[doc = "application/vnd.ims.imsccv1p2"]
    #[serde(rename = "application/vnd.ims.imsccv1p2")]
    VndImsImsccv1P2,
    #[doc = "application/vnd.ims.imsccv1p3"]
    #[serde(rename = "application/vnd.ims.imsccv1p3")]
    VndImsImsccv1P3,
    #[doc = "application/vnd.ims.lis.v2.result+json"]
    #[serde(rename = "application/vnd.ims.lis.v2.result+json")]
    VndImsLisV2ResultJson,
    #[doc = "application/vnd.ims.lti.v2.toolconsumerprofile+json"]
    #[serde(rename = "application/vnd.ims.lti.v2.toolconsumerprofile+json")]
    VndImsLtiV2ToolconsumerprofileJson,
    #[doc = "application/vnd.ims.lti.v2.toolproxy.id+json"]
    #[serde(rename = "application/vnd.ims.lti.v2.toolproxy.id+json")]
    VndImsLtiV2ToolproxyIdJson,
    #[doc = "application/vnd.ims.lti.v2.toolproxy+json"]
    #[serde(rename = "application/vnd.ims.lti.v2.toolproxy+json")]
    VndImsLtiV2ToolproxyJson,
    #[doc = "application/vnd.ims.lti.v2.toolsettings+json"]
    #[serde(rename = "application/vnd.ims.lti.v2.toolsettings+json")]
    VndImsLtiV2ToolsettingsJson,
    #[doc = "application/vnd.ims.lti.v2.toolsettings.simple+json"]
    #[serde(rename = "application/vnd.ims.lti.v2.toolsettings.simple+json")]
    VndImsLtiV2ToolsettingsSimpleJson,
    #[doc = "application/vnd.informedcontrol.rms+xml"]
    #[serde(rename = "application/vnd.informedcontrol.rms+xml")]
    VndInformedcontrolRmsXml,
    #[doc = "application/vnd.infotech.project"]
    #[serde(rename = "application/vnd.infotech.project")]
    VndInfotechProject,
    #[doc = "application/vnd.infotech.project+xml"]
    #[serde(rename = "application/vnd.infotech.project+xml")]
    VndInfotechProjectXml,
    #[doc = "application/vnd.innopath.wamp.notification"]
    #[serde(rename = "application/vnd.innopath.wamp.notification")]
    VndInnopathWampNotification,
    #[doc = "application/vnd.insors.igm"]
    #[serde(rename = "application/vnd.insors.igm")]
    #[serde(alias = "igm")]
    VndInsorsIgm,
    #[doc = "application/vnd.intercon.formnet"]
    #[serde(rename = "application/vnd.intercon.formnet")]
    #[serde(alias = "xpw")]
    #[serde(alias = "xpx")]
    VndInterconFormnet,
    #[doc = "application/vnd.intergeo"]
    #[serde(rename = "application/vnd.intergeo")]
    #[serde(alias = "i2g")]
    VndIntergeo,
    #[doc = "application/vnd.intertrust.digibox"]
    #[serde(rename = "application/vnd.intertrust.digibox")]
    VndIntertrustDigibox,
    #[doc = "application/vnd.intertrust.nncp"]
    #[serde(rename = "application/vnd.intertrust.nncp")]
    VndIntertrustNncp,
    #[doc = "application/vnd.intu.qbo"]
    #[serde(rename = "application/vnd.intu.qbo")]
    #[serde(alias = "qbo")]
    VndIntuQbo,
    #[doc = "application/vnd.intu.qfx"]
    #[serde(rename = "application/vnd.intu.qfx")]
    #[serde(alias = "qfx")]
    VndIntuQfx,
    #[doc = "application/vnd.ipfs.ipns-record"]
    #[serde(rename = "application/vnd.ipfs.ipns-record")]
    VndIpfsIpnsRecord,
    #[doc = "application/vnd.ipld.car"]
    #[serde(rename = "application/vnd.ipld.car")]
    #[serde(alias = "car")]
    VndIpldCar,
    #[doc = "application/vnd.ipld.dag-cbor"]
    #[serde(rename = "application/vnd.ipld.dag-cbor")]
    VndIpldDagCbor,
    #[doc = "application/vnd.ipld.dag-json"]
    #[serde(rename = "application/vnd.ipld.dag-json")]
    VndIpldDagJson,
    #[doc = "application/vnd.ipld.raw"]
    #[serde(rename = "application/vnd.ipld.raw")]
    VndIpldRaw,
    #[doc = "application/vnd.iptc.g2.catalogitem+xml"]
    #[serde(rename = "application/vnd.iptc.g2.catalogitem+xml")]
    VndIptcG2CatalogitemXml,
    #[doc = "application/vnd.iptc.g2.conceptitem+xml"]
    #[serde(rename = "application/vnd.iptc.g2.conceptitem+xml")]
    VndIptcG2ConceptitemXml,
    #[doc = "application/vnd.iptc.g2.knowledgeitem+xml"]
    #[serde(rename = "application/vnd.iptc.g2.knowledgeitem+xml")]
    VndIptcG2KnowledgeitemXml,
    #[doc = "application/vnd.iptc.g2.newsitem+xml"]
    #[serde(rename = "application/vnd.iptc.g2.newsitem+xml")]
    VndIptcG2NewsitemXml,
    #[doc = "application/vnd.iptc.g2.newsmessage+xml"]
    #[serde(rename = "application/vnd.iptc.g2.newsmessage+xml")]
    VndIptcG2NewsmessageXml,
    #[doc = "application/vnd.iptc.g2.packageitem+xml"]
    #[serde(rename = "application/vnd.iptc.g2.packageitem+xml")]
    VndIptcG2PackageitemXml,
    #[doc = "application/vnd.iptc.g2.planningitem+xml"]
    #[serde(rename = "application/vnd.iptc.g2.planningitem+xml")]
    VndIptcG2PlanningitemXml,
    #[doc = "application/vnd.ipunplugged.rcprofile"]
    #[serde(rename = "application/vnd.ipunplugged.rcprofile")]
    #[serde(alias = "rcprofile")]
    VndIpunpluggedRcprofile,
    #[doc = "application/vnd.irepository.package+xml"]
    #[serde(rename = "application/vnd.irepository.package+xml")]
    #[serde(alias = "irp")]
    VndIrepositoryPackageXml,
    #[doc = "application/vnd.is-xpr"]
    #[serde(rename = "application/vnd.is-xpr")]
    #[serde(alias = "xpr")]
    VndIsXpr,
    #[doc = "application/vnd.isac.fcs"]
    #[serde(rename = "application/vnd.isac.fcs")]
    #[serde(alias = "fcs")]
    VndIsacFcs,
    #[doc = "application/vnd.jam"]
    #[serde(rename = "application/vnd.jam")]
    #[serde(alias = "jam")]
    VndJam,
    #[doc = "application/vnd.iso11783-10+zip"]
    #[serde(rename = "application/vnd.iso11783-10+zip")]
    VndIso1178310Zip,
    #[doc = "application/vnd.japannet-directory-service"]
    #[serde(rename = "application/vnd.japannet-directory-service")]
    VndJapannetDirectoryService,
    #[doc = "application/vnd.japannet-jpnstore-wakeup"]
    #[serde(rename = "application/vnd.japannet-jpnstore-wakeup")]
    VndJapannetJpnstoreWakeup,
    #[doc = "application/vnd.japannet-payment-wakeup"]
    #[serde(rename = "application/vnd.japannet-payment-wakeup")]
    VndJapannetPaymentWakeup,
    #[doc = "application/vnd.japannet-registration"]
    #[serde(rename = "application/vnd.japannet-registration")]
    VndJapannetRegistration,
    #[doc = "application/vnd.japannet-registration-wakeup"]
    #[serde(rename = "application/vnd.japannet-registration-wakeup")]
    VndJapannetRegistrationWakeup,
    #[doc = "application/vnd.japannet-setstore-wakeup"]
    #[serde(rename = "application/vnd.japannet-setstore-wakeup")]
    VndJapannetSetstoreWakeup,
    #[doc = "application/vnd.japannet-verification"]
    #[serde(rename = "application/vnd.japannet-verification")]
    VndJapannetVerification,
    #[doc = "application/vnd.japannet-verification-wakeup"]
    #[serde(rename = "application/vnd.japannet-verification-wakeup")]
    VndJapannetVerificationWakeup,
    #[doc = "application/vnd.jcp.javame.midlet-rms"]
    #[serde(rename = "application/vnd.jcp.javame.midlet-rms")]
    #[serde(alias = "rms")]
    VndJcpJavameMidletRms,
    #[doc = "application/vnd.jisp"]
    #[serde(rename = "application/vnd.jisp")]
    #[serde(alias = "jisp")]
    VndJisp,
    #[doc = "application/vnd.joost.joda-archive"]
    #[serde(rename = "application/vnd.joost.joda-archive")]
    #[serde(alias = "joda")]
    VndJoostJodaArchive,
    #[doc = "application/vnd.jsk.isdn-ngn"]
    #[serde(rename = "application/vnd.jsk.isdn-ngn")]
    VndJskIsdnNgn,
    #[doc = "application/vnd.kahootz"]
    #[serde(rename = "application/vnd.kahootz")]
    #[serde(alias = "ktz")]
    #[serde(alias = "ktr")]
    VndKahootz,
    #[doc = "application/vnd.kde.karbon"]
    #[serde(rename = "application/vnd.kde.karbon")]
    #[serde(alias = "karbon")]
    VndKdeKarbon,
    #[doc = "application/vnd.kde.kchart"]
    #[serde(rename = "application/vnd.kde.kchart")]
    #[serde(alias = "chrt")]
    VndKdeKchart,
    #[doc = "application/vnd.kde.kformula"]
    #[serde(rename = "application/vnd.kde.kformula")]
    #[serde(alias = "kfo")]
    VndKdeKformula,
    #[doc = "application/vnd.kde.kivio"]
    #[serde(rename = "application/vnd.kde.kivio")]
    #[serde(alias = "flw")]
    VndKdeKivio,
    #[doc = "application/vnd.kde.kontour"]
    #[serde(rename = "application/vnd.kde.kontour")]
    #[serde(alias = "kon")]
    VndKdeKontour,
    #[doc = "application/vnd.kde.kpresenter"]
    #[serde(rename = "application/vnd.kde.kpresenter")]
    #[serde(alias = "kpr")]
    #[serde(alias = "kpt")]
    VndKdeKpresenter,
    #[doc = "application/vnd.kde.kspread"]
    #[serde(rename = "application/vnd.kde.kspread")]
    #[serde(alias = "ksp")]
    VndKdeKspread,
    #[doc = "application/vnd.kde.kword"]
    #[serde(rename = "application/vnd.kde.kword")]
    #[serde(alias = "kwd")]
    #[serde(alias = "kwt")]
    VndKdeKword,
    #[doc = "application/vnd.kenameaapp"]
    #[serde(rename = "application/vnd.kenameaapp")]
    #[serde(alias = "htke")]
    VndKenameaapp,
    #[doc = "application/vnd.kidspiration"]
    #[serde(rename = "application/vnd.kidspiration")]
    #[serde(alias = "kia")]
    VndKidspiration,
    #[doc = "application/vnd.Kinar"]
    #[serde(rename = "application/vnd.Kinar")]
    #[serde(alias = "kne")]
    #[serde(alias = "knp")]
    #[serde(alias = "sdf")]
    VndKinar,
    #[doc = "application/vnd.koan"]
    #[serde(rename = "application/vnd.koan")]
    #[serde(alias = "skp")]
    #[serde(alias = "skd")]
    #[serde(alias = "skm")]
    #[serde(alias = "skt")]
    VndKoan,
    #[doc = "application/vnd.kodak-descriptor"]
    #[serde(rename = "application/vnd.kodak-descriptor")]
    #[serde(alias = "sse")]
    VndKodakDescriptor,
    #[doc = "application/vnd.las"]
    #[serde(rename = "application/vnd.las")]
    #[serde(alias = "las")]
    VndLas,
    #[doc = "application/vnd.las.las+json"]
    #[serde(rename = "application/vnd.las.las+json")]
    #[serde(alias = "lasjson")]
    VndLasLasJson,
    #[doc = "application/vnd.las.las+xml"]
    #[serde(rename = "application/vnd.las.las+xml")]
    #[serde(alias = "lasxml")]
    VndLasLasXml,
    #[doc = "application/vnd.laszip"]
    #[serde(rename = "application/vnd.laszip")]
    VndLaszip,
    #[doc = "application/vnd.leap+json"]
    #[serde(rename = "application/vnd.leap+json")]
    VndLeapJson,
    #[doc = "application/vnd.liberty-request+xml"]
    #[serde(rename = "application/vnd.liberty-request+xml")]
    VndLibertyRequestXml,
    #[doc = "application/vnd.llamagraphics.life-balance.desktop"]
    #[serde(rename = "application/vnd.llamagraphics.life-balance.desktop")]
    #[serde(alias = "lbd")]
    VndLlamagraphicsLifeBalanceDesktop,
    #[doc = "application/vnd.llamagraphics.life-balance.exchange+xml"]
    #[serde(rename = "application/vnd.llamagraphics.life-balance.exchange+xml")]
    #[serde(alias = "lbe")]
    VndLlamagraphicsLifeBalanceExchangeXml,
    #[doc = "application/vnd.logipipe.circuit+zip"]
    #[serde(rename = "application/vnd.logipipe.circuit+zip")]
    #[serde(alias = "lcs")]
    #[serde(alias = "lca")]
    VndLogipipeCircuitZip,
    #[doc = "application/vnd.loom"]
    #[serde(rename = "application/vnd.loom")]
    #[serde(alias = "loom")]
    VndLoom,
    #[doc = "application/vnd.lotus-1-2-3"]
    #[serde(rename = "application/vnd.lotus-1-2-3")]
    #[serde(alias = "123")]
    #[serde(alias = "wk4")]
    #[serde(alias = "wk3")]
    #[serde(alias = "wk1")]
    VndLotus123,
    #[doc = "application/vnd.lotus-approach"]
    #[serde(rename = "application/vnd.lotus-approach")]
    #[serde(alias = "apr")]
    #[serde(alias = "vew")]
    VndLotusApproach,
    #[doc = "application/vnd.lotus-freelance"]
    #[serde(rename = "application/vnd.lotus-freelance")]
    #[serde(alias = "prz")]
    #[serde(alias = "pre")]
    VndLotusFreelance,
    #[doc = "application/vnd.lotus-notes"]
    #[serde(rename = "application/vnd.lotus-notes")]
    #[serde(alias = "nsf")]
    #[serde(alias = "ntf")]
    #[serde(alias = "ndl")]
    #[serde(alias = "ns4")]
    #[serde(alias = "ns3")]
    #[serde(alias = "ns2")]
    #[serde(alias = "nsh")]
    #[serde(alias = "nsg")]
    VndLotusNotes,
    #[doc = "application/vnd.lotus-organizer"]
    #[serde(rename = "application/vnd.lotus-organizer")]
    #[serde(alias = "or3")]
    #[serde(alias = "or2")]
    #[serde(alias = "org")]
    VndLotusOrganizer,
    #[doc = "application/vnd.lotus-screencam"]
    #[serde(rename = "application/vnd.lotus-screencam")]
    #[serde(alias = "scm")]
    VndLotusScreencam,
    #[doc = "application/vnd.lotus-wordpro"]
    #[serde(rename = "application/vnd.lotus-wordpro")]
    #[serde(alias = "lwp")]
    #[serde(alias = "sam")]
    VndLotusWordpro,
    #[doc = "application/vnd.macports.portpkg"]
    #[serde(rename = "application/vnd.macports.portpkg")]
    #[serde(alias = "portpkg")]
    VndMacportsPortpkg,
    #[doc = "application/vnd.mapbox-vector-tile"]
    #[serde(rename = "application/vnd.mapbox-vector-tile")]
    #[serde(alias = "mvt")]
    VndMapboxVectorTile,
    #[doc = "application/vnd.marlin.drm.actiontoken+xml"]
    #[serde(rename = "application/vnd.marlin.drm.actiontoken+xml")]
    VndMarlinDrmActiontokenXml,
    #[doc = "application/vnd.marlin.drm.conftoken+xml"]
    #[serde(rename = "application/vnd.marlin.drm.conftoken+xml")]
    VndMarlinDrmConftokenXml,
    #[doc = "application/vnd.marlin.drm.license+xml"]
    #[serde(rename = "application/vnd.marlin.drm.license+xml")]
    VndMarlinDrmLicenseXml,
    #[doc = "application/vnd.marlin.drm.mdcf"]
    #[serde(rename = "application/vnd.marlin.drm.mdcf")]
    #[serde(alias = "mdc")]
    VndMarlinDrmMdcf,
    #[doc = "application/vnd.mason+json"]
    #[serde(rename = "application/vnd.mason+json")]
    VndMasonJson,
    #[doc = "application/vnd.maxar.archive.3tz+zip"]
    #[serde(rename = "application/vnd.maxar.archive.3tz+zip")]
    #[serde(alias = "3tz")]
    VndMaxarArchive3TzZip,
    #[doc = "application/vnd.maxmind.maxmind-db"]
    #[serde(rename = "application/vnd.maxmind.maxmind-db")]
    #[serde(alias = "mmdb")]
    VndMaxmindMaxmindDb,
    #[doc = "application/vnd.mcd"]
    #[serde(rename = "application/vnd.mcd")]
    #[serde(alias = "mcd")]
    VndMcd,
    #[doc = "application/vnd.mdl"]
    #[serde(rename = "application/vnd.mdl")]
    #[serde(alias = "mdl")]
    VndMdl,
    #[doc = "application/vnd.mdl-mbsdf"]
    #[serde(rename = "application/vnd.mdl-mbsdf")]
    #[serde(alias = "mbsdf")]
    VndMdlMbsdf,
    #[doc = "application/vnd.medcalcdata"]
    #[serde(rename = "application/vnd.medcalcdata")]
    #[serde(alias = "mc1")]
    VndMedcalcdata,
    #[doc = "application/vnd.mediastation.cdkey"]
    #[serde(rename = "application/vnd.mediastation.cdkey")]
    #[serde(alias = "cdkey")]
    VndMediastationCdkey,
    #[doc = "application/vnd.medicalholodeck.recordxr"]
    #[serde(rename = "application/vnd.medicalholodeck.recordxr")]
    #[serde(alias = "rxr")]
    VndMedicalholodeckRecordxr,
    #[doc = "application/vnd.meridian-slingshot"]
    #[serde(rename = "application/vnd.meridian-slingshot")]
    VndMeridianSlingshot,
    #[doc = "application/vnd.MFER"]
    #[serde(rename = "application/vnd.MFER")]
    #[serde(alias = "mwf")]
    VndMFER,
    #[doc = "application/vnd.mfmp"]
    #[serde(rename = "application/vnd.mfmp")]
    #[serde(alias = "mfm")]
    VndMfmp,
    #[doc = "application/vnd.micro+json"]
    #[serde(rename = "application/vnd.micro+json")]
    VndMicroJson,
    #[doc = "application/vnd.micrografx.flo"]
    #[serde(rename = "application/vnd.micrografx.flo")]
    #[serde(alias = "flo")]
    VndMicrografxFlo,
    #[doc = "application/vnd.micrografx.igx"]
    #[serde(rename = "application/vnd.micrografx.igx")]
    #[serde(alias = "igx")]
    VndMicrografxIgx,
    #[doc = "application/vnd.microsoft.portable-executable"]
    #[serde(rename = "application/vnd.microsoft.portable-executable")]
    VndMicrosoftPortableExecutable,
    #[doc = "application/vnd.microsoft.windows.thumbnail-cache"]
    #[serde(rename = "application/vnd.microsoft.windows.thumbnail-cache")]
    VndMicrosoftWindowsThumbnailCache,
    #[doc = "application/vnd.miele+json"]
    #[serde(rename = "application/vnd.miele+json")]
    VndMieleJson,
    #[doc = "application/vnd.mif"]
    #[serde(rename = "application/vnd.mif")]
    #[serde(alias = "mif")]
    VndMif,
    #[doc = "application/vnd.minisoft-hp3000-save"]
    #[serde(rename = "application/vnd.minisoft-hp3000-save")]
    VndMinisoftHp3000Save,
    #[doc = "application/vnd.mitsubishi.misty-guard.trustweb"]
    #[serde(rename = "application/vnd.mitsubishi.misty-guard.trustweb")]
    VndMitsubishiMistyGuardTrustweb,
    #[doc = "application/vnd.Mobius.DAF"]
    #[serde(rename = "application/vnd.Mobius.DAF")]
    #[serde(alias = "daf")]
    VndMobiusDAF,
    #[doc = "application/vnd.Mobius.DIS"]
    #[serde(rename = "application/vnd.Mobius.DIS")]
    #[serde(alias = "dis")]
    VndMobiusDIS,
    #[doc = "application/vnd.Mobius.MBK"]
    #[serde(rename = "application/vnd.Mobius.MBK")]
    #[serde(alias = "mbk")]
    VndMobiusMBK,
    #[doc = "application/vnd.Mobius.MQY"]
    #[serde(rename = "application/vnd.Mobius.MQY")]
    #[serde(alias = "mqy")]
    VndMobiusMQY,
    #[doc = "application/vnd.Mobius.MSL"]
    #[serde(rename = "application/vnd.Mobius.MSL")]
    #[serde(alias = "msl")]
    VndMobiusMSL,
    #[doc = "application/vnd.Mobius.PLC"]
    #[serde(rename = "application/vnd.Mobius.PLC")]
    #[serde(alias = "plc")]
    VndMobiusPLC,
    #[doc = "application/vnd.Mobius.TXF"]
    #[serde(rename = "application/vnd.Mobius.TXF")]
    #[serde(alias = "txf")]
    VndMobiusTXF,
    #[doc = "application/vnd.modl"]
    #[serde(rename = "application/vnd.modl")]
    #[serde(alias = "modl")]
    VndModl,
    #[doc = "application/vnd.mophun.application"]
    #[serde(rename = "application/vnd.mophun.application")]
    #[serde(alias = "mpn")]
    VndMophunApplication,
    #[doc = "application/vnd.mophun.certificate"]
    #[serde(rename = "application/vnd.mophun.certificate")]
    #[serde(alias = "mpc")]
    VndMophunCertificate,
    #[doc = "application/vnd.motorola.flexsuite"]
    #[serde(rename = "application/vnd.motorola.flexsuite")]
    VndMotorolaFlexsuite,
    #[doc = "application/vnd.motorola.flexsuite.adsi"]
    #[serde(rename = "application/vnd.motorola.flexsuite.adsi")]
    VndMotorolaFlexsuiteAdsi,
    #[doc = "application/vnd.motorola.flexsuite.fis"]
    #[serde(rename = "application/vnd.motorola.flexsuite.fis")]
    VndMotorolaFlexsuiteFis,
    #[doc = "application/vnd.motorola.flexsuite.gotap"]
    #[serde(rename = "application/vnd.motorola.flexsuite.gotap")]
    VndMotorolaFlexsuiteGotap,
    #[doc = "application/vnd.motorola.flexsuite.kmr"]
    #[serde(rename = "application/vnd.motorola.flexsuite.kmr")]
    VndMotorolaFlexsuiteKmr,
    #[doc = "application/vnd.motorola.flexsuite.ttc"]
    #[serde(rename = "application/vnd.motorola.flexsuite.ttc")]
    VndMotorolaFlexsuiteTtc,
    #[doc = "application/vnd.motorola.flexsuite.wem"]
    #[serde(rename = "application/vnd.motorola.flexsuite.wem")]
    VndMotorolaFlexsuiteWem,
    #[doc = "application/vnd.motorola.iprm"]
    #[serde(rename = "application/vnd.motorola.iprm")]
    VndMotorolaIprm,
    #[doc = "application/vnd.mozilla.xul+xml"]
    #[serde(rename = "application/vnd.mozilla.xul+xml")]
    #[serde(alias = "xul")]
    VndMozillaXulXml,
    #[doc = "application/vnd.ms-artgalry"]
    #[serde(rename = "application/vnd.ms-artgalry")]
    #[serde(alias = "cil")]
    VndMsArtgalry,
    #[doc = "application/vnd.ms-asf"]
    #[serde(rename = "application/vnd.ms-asf")]
    #[serde(alias = "asf")]
    VndMsAsf,
    #[doc = "application/vnd.ms-cab-compressed"]
    #[serde(rename = "application/vnd.ms-cab-compressed")]
    #[serde(alias = "cab")]
    VndMsCabCompressed,
    #[doc = "application/vnd.ms-3mfdocument"]
    #[serde(rename = "application/vnd.ms-3mfdocument")]
    #[serde(alias = "3mf")]
    VndMs3Mfdocument,
    #[doc = "application/vnd.ms-excel"]
    #[serde(rename = "application/vnd.ms-excel")]
    #[serde(alias = "xls")]
    #[serde(alias = "xlm")]
    #[serde(alias = "xla")]
    #[serde(alias = "xlc")]
    #[serde(alias = "xlt")]
    #[serde(alias = "xlw")]
    VndMsExcel,
    #[doc = "application/vnd.ms-excel.addin.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-excel.addin.macroEnabled.12")]
    #[serde(alias = "xlam")]
    VndMsExcelAddinMacroEnabled12,
    #[doc = "application/vnd.ms-excel.sheet.binary.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-excel.sheet.binary.macroEnabled.12")]
    #[serde(alias = "xlsb")]
    VndMsExcelSheetBinaryMacroEnabled12,
    #[doc = "application/vnd.ms-excel.sheet.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-excel.sheet.macroEnabled.12")]
    #[serde(alias = "xlsm")]
    VndMsExcelSheetMacroEnabled12,
    #[doc = "application/vnd.ms-excel.template.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-excel.template.macroEnabled.12")]
    #[serde(alias = "xltm")]
    VndMsExcelTemplateMacroEnabled12,
    #[doc = "application/vnd.ms-fontobject"]
    #[serde(rename = "application/vnd.ms-fontobject")]
    #[serde(alias = "eot")]
    VndMsFontobject,
    #[doc = "application/vnd.ms-htmlhelp"]
    #[serde(rename = "application/vnd.ms-htmlhelp")]
    #[serde(alias = "chm")]
    VndMsHtmlhelp,
    #[doc = "application/vnd.ms-ims"]
    #[serde(rename = "application/vnd.ms-ims")]
    #[serde(alias = "ims")]
    VndMsIms,
    #[doc = "application/vnd.ms-lrm"]
    #[serde(rename = "application/vnd.ms-lrm")]
    #[serde(alias = "lrm")]
    VndMsLrm,
    #[doc = "application/vnd.ms-office.activeX+xml"]
    #[serde(rename = "application/vnd.ms-office.activeX+xml")]
    VndMsOfficeActiveXXml,
    #[doc = "application/vnd.ms-officetheme"]
    #[serde(rename = "application/vnd.ms-officetheme")]
    #[serde(alias = "thmx")]
    VndMsOfficetheme,
    #[doc = "application/vnd.ms-playready.initiator+xml"]
    #[serde(rename = "application/vnd.ms-playready.initiator+xml")]
    VndMsPlayreadyInitiatorXml,
    #[doc = "application/vnd.ms-powerpoint"]
    #[serde(rename = "application/vnd.ms-powerpoint")]
    #[serde(alias = "ppt")]
    #[serde(alias = "pps")]
    #[serde(alias = "pot")]
    VndMsPowerpoint,
    #[doc = "application/vnd.ms-powerpoint.addin.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-powerpoint.addin.macroEnabled.12")]
    #[serde(alias = "ppam")]
    VndMsPowerpointAddinMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.presentation.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-powerpoint.presentation.macroEnabled.12")]
    #[serde(alias = "pptm")]
    VndMsPowerpointPresentationMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.slide.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-powerpoint.slide.macroEnabled.12")]
    #[serde(alias = "sldm")]
    VndMsPowerpointSlideMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.slideshow.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-powerpoint.slideshow.macroEnabled.12")]
    #[serde(alias = "ppsm")]
    VndMsPowerpointSlideshowMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.template.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-powerpoint.template.macroEnabled.12")]
    #[serde(alias = "potm")]
    VndMsPowerpointTemplateMacroEnabled12,
    #[doc = "application/vnd.ms-PrintDeviceCapabilities+xml"]
    #[serde(rename = "application/vnd.ms-PrintDeviceCapabilities+xml")]
    VndMsPrintDeviceCapabilitiesXml,
    #[doc = "application/vnd.ms-PrintSchemaTicket+xml"]
    #[serde(rename = "application/vnd.ms-PrintSchemaTicket+xml")]
    VndMsPrintSchemaTicketXml,
    #[doc = "application/vnd.ms-project"]
    #[serde(rename = "application/vnd.ms-project")]
    #[serde(alias = "mpp")]
    #[serde(alias = "mpt")]
    VndMsProject,
    #[doc = "application/vnd.ms-tnef"]
    #[serde(rename = "application/vnd.ms-tnef")]
    #[serde(alias = "tnef")]
    #[serde(alias = "tnf")]
    VndMsTnef,
    #[doc = "application/vnd.ms-windows.devicepairing"]
    #[serde(rename = "application/vnd.ms-windows.devicepairing")]
    VndMsWindowsDevicepairing,
    #[doc = "application/vnd.ms-windows.nwprinting.oob"]
    #[serde(rename = "application/vnd.ms-windows.nwprinting.oob")]
    VndMsWindowsNwprintingOob,
    #[doc = "application/vnd.ms-windows.printerpairing"]
    #[serde(rename = "application/vnd.ms-windows.printerpairing")]
    VndMsWindowsPrinterpairing,
    #[doc = "application/vnd.ms-windows.wsd.oob"]
    #[serde(rename = "application/vnd.ms-windows.wsd.oob")]
    VndMsWindowsWsdOob,
    #[doc = "application/vnd.ms-wmdrm.lic-chlg-req"]
    #[serde(rename = "application/vnd.ms-wmdrm.lic-chlg-req")]
    VndMsWmdrmLicChlgReq,
    #[doc = "application/vnd.ms-wmdrm.lic-resp"]
    #[serde(rename = "application/vnd.ms-wmdrm.lic-resp")]
    VndMsWmdrmLicResp,
    #[doc = "application/vnd.ms-wmdrm.meter-chlg-req"]
    #[serde(rename = "application/vnd.ms-wmdrm.meter-chlg-req")]
    VndMsWmdrmMeterChlgReq,
    #[doc = "application/vnd.ms-wmdrm.meter-resp"]
    #[serde(rename = "application/vnd.ms-wmdrm.meter-resp")]
    VndMsWmdrmMeterResp,
    #[doc = "application/vnd.ms-word.document.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-word.document.macroEnabled.12")]
    #[serde(alias = "docm")]
    VndMsWordDocumentMacroEnabled12,
    #[doc = "application/vnd.ms-word.template.macroEnabled.12"]
    #[serde(rename = "application/vnd.ms-word.template.macroEnabled.12")]
    #[serde(alias = "dotm")]
    VndMsWordTemplateMacroEnabled12,
    #[doc = "application/vnd.ms-works"]
    #[serde(rename = "application/vnd.ms-works")]
    #[serde(alias = "wcm")]
    #[serde(alias = "wdb")]
    #[serde(alias = "wks")]
    #[serde(alias = "wps")]
    VndMsWorks,
    #[doc = "application/vnd.ms-wpl"]
    #[serde(rename = "application/vnd.ms-wpl")]
    #[serde(alias = "wpl")]
    VndMsWpl,
    #[doc = "application/vnd.ms-xpsdocument"]
    #[serde(rename = "application/vnd.ms-xpsdocument")]
    #[serde(alias = "xps")]
    VndMsXpsdocument,
    #[doc = "application/vnd.msa-disk-image"]
    #[serde(rename = "application/vnd.msa-disk-image")]
    #[serde(alias = "msa")]
    VndMsaDiskImage,
    #[doc = "application/vnd.mseq"]
    #[serde(rename = "application/vnd.mseq")]
    #[serde(alias = "mseq")]
    VndMseq,
    #[doc = "application/vnd.msign"]
    #[serde(rename = "application/vnd.msign")]
    VndMsign,
    #[doc = "application/vnd.multiad.creator"]
    #[serde(rename = "application/vnd.multiad.creator")]
    #[serde(alias = "crtr")]
    VndMultiadCreator,
    #[doc = "application/vnd.multiad.creator.cif"]
    #[serde(rename = "application/vnd.multiad.creator.cif")]
    #[serde(alias = "cif")]
    VndMultiadCreatorCif,
    #[doc = "application/vnd.musician"]
    #[serde(rename = "application/vnd.musician")]
    #[serde(alias = "mus")]
    VndMusician,
    #[doc = "application/vnd.music-niff"]
    #[serde(rename = "application/vnd.music-niff")]
    VndMusicNiff,
    #[doc = "application/vnd.muvee.style"]
    #[serde(rename = "application/vnd.muvee.style")]
    #[serde(alias = "msty")]
    VndMuveeStyle,
    #[doc = "application/vnd.mynfc"]
    #[serde(rename = "application/vnd.mynfc")]
    #[serde(alias = "taglet")]
    VndMynfc,
    #[doc = "application/vnd.nacamar.ybrid+json"]
    #[serde(rename = "application/vnd.nacamar.ybrid+json")]
    VndNacamarYbridJson,
    #[doc = "application/vnd.ncd.control"]
    #[serde(rename = "application/vnd.ncd.control")]
    VndNcdControl,
    #[doc = "application/vnd.ncd.reference"]
    #[serde(rename = "application/vnd.ncd.reference")]
    VndNcdReference,
    #[doc = "application/vnd.nearst.inv+json"]
    #[serde(rename = "application/vnd.nearst.inv+json")]
    VndNearstInvJson,
    #[doc = "application/vnd.nebumind.line"]
    #[serde(rename = "application/vnd.nebumind.line")]
    #[serde(alias = "nebul")]
    #[serde(alias = "line")]
    VndNebumindLine,
    #[doc = "application/vnd.nervana"]
    #[serde(rename = "application/vnd.nervana")]
    #[serde(alias = "entity")]
    #[serde(alias = "request")]
    #[serde(alias = "bkm")]
    #[serde(alias = "kcm")]
    VndNervana,
    #[doc = "application/vnd.netfpx"]
    #[serde(rename = "application/vnd.netfpx")]
    VndNetfpx,
    #[doc = "application/vnd.neurolanguage.nlu"]
    #[serde(rename = "application/vnd.neurolanguage.nlu")]
    #[serde(alias = "nlu")]
    VndNeurolanguageNlu,
    #[doc = "application/vnd.nimn"]
    #[serde(rename = "application/vnd.nimn")]
    #[serde(alias = "nimn")]
    VndNimn,
    #[doc = "application/vnd.nintendo.snes.rom"]
    #[serde(rename = "application/vnd.nintendo.snes.rom")]
    #[serde(alias = "sfc")]
    #[serde(alias = "smc")]
    VndNintendoSnesRom,
    #[doc = "application/vnd.nintendo.nitro.rom"]
    #[serde(rename = "application/vnd.nintendo.nitro.rom")]
    #[serde(alias = "nds")]
    VndNintendoNitroRom,
    #[doc = "application/vnd.nitf"]
    #[serde(rename = "application/vnd.nitf")]
    #[serde(alias = "nitf")]
    VndNitf,
    #[doc = "application/vnd.noblenet-directory"]
    #[serde(rename = "application/vnd.noblenet-directory")]
    #[serde(alias = "nnd")]
    VndNoblenetDirectory,
    #[doc = "application/vnd.noblenet-sealer"]
    #[serde(rename = "application/vnd.noblenet-sealer")]
    #[serde(alias = "nns")]
    VndNoblenetSealer,
    #[doc = "application/vnd.noblenet-web"]
    #[serde(rename = "application/vnd.noblenet-web")]
    #[serde(alias = "nnw")]
    VndNoblenetWeb,
    #[doc = "application/vnd.nokia.catalogs"]
    #[serde(rename = "application/vnd.nokia.catalogs")]
    VndNokiaCatalogs,
    #[doc = "application/vnd.nokia.conml+wbxml"]
    #[serde(rename = "application/vnd.nokia.conml+wbxml")]
    VndNokiaConmlWbxml,
    #[doc = "application/vnd.nokia.conml+xml"]
    #[serde(rename = "application/vnd.nokia.conml+xml")]
    VndNokiaConmlXml,
    #[doc = "application/vnd.nokia.iptv.config+xml"]
    #[serde(rename = "application/vnd.nokia.iptv.config+xml")]
    VndNokiaIptvConfigXml,
    #[doc = "application/vnd.nokia.iSDS-radio-presets"]
    #[serde(rename = "application/vnd.nokia.iSDS-radio-presets")]
    VndNokiaISDSRadioPresets,
    #[doc = "application/vnd.nokia.landmark+wbxml"]
    #[serde(rename = "application/vnd.nokia.landmark+wbxml")]
    VndNokiaLandmarkWbxml,
    #[doc = "application/vnd.nokia.landmark+xml"]
    #[serde(rename = "application/vnd.nokia.landmark+xml")]
    VndNokiaLandmarkXml,
    #[doc = "application/vnd.nokia.landmarkcollection+xml"]
    #[serde(rename = "application/vnd.nokia.landmarkcollection+xml")]
    VndNokiaLandmarkcollectionXml,
    #[doc = "application/vnd.nokia.ncd"]
    #[serde(rename = "application/vnd.nokia.ncd")]
    VndNokiaNcd,
    #[doc = "application/vnd.nokia.n-gage.ac+xml"]
    #[serde(rename = "application/vnd.nokia.n-gage.ac+xml")]
    #[serde(alias = "ac")]
    VndNokiaNGageAcXml,
    #[doc = "application/vnd.nokia.n-gage.data"]
    #[serde(rename = "application/vnd.nokia.n-gage.data")]
    #[serde(alias = "ngdat")]
    VndNokiaNGageData,
    #[doc = "application/vnd.nokia.pcd+wbxml"]
    #[serde(rename = "application/vnd.nokia.pcd+wbxml")]
    VndNokiaPcdWbxml,
    #[doc = "application/vnd.nokia.pcd+xml"]
    #[serde(rename = "application/vnd.nokia.pcd+xml")]
    VndNokiaPcdXml,
    #[doc = "application/vnd.nokia.radio-preset"]
    #[serde(rename = "application/vnd.nokia.radio-preset")]
    #[serde(alias = "rpst")]
    VndNokiaRadioPreset,
    #[doc = "application/vnd.nokia.radio-presets"]
    #[serde(rename = "application/vnd.nokia.radio-presets")]
    #[serde(alias = "rpss")]
    VndNokiaRadioPresets,
    #[doc = "application/vnd.novadigm.EDM"]
    #[serde(rename = "application/vnd.novadigm.EDM")]
    #[serde(alias = "edm")]
    VndNovadigmEDM,
    #[doc = "application/vnd.novadigm.EDX"]
    #[serde(rename = "application/vnd.novadigm.EDX")]
    #[serde(alias = "edx")]
    VndNovadigmEDX,
    #[doc = "application/vnd.novadigm.EXT"]
    #[serde(rename = "application/vnd.novadigm.EXT")]
    #[serde(alias = "ext")]
    VndNovadigmEXT,
    #[doc = "application/vnd.ntt-local.content-share"]
    #[serde(rename = "application/vnd.ntt-local.content-share")]
    VndNttLocalContentShare,
    #[doc = "application/vnd.ntt-local.file-transfer"]
    #[serde(rename = "application/vnd.ntt-local.file-transfer")]
    VndNttLocalFileTransfer,
    #[doc = "application/vnd.ntt-local.ogw_remote-access"]
    #[serde(rename = "application/vnd.ntt-local.ogw_remote-access")]
    VndNttLocalOgwRemoteAccess,
    #[doc = "application/vnd.ntt-local.sip-ta_remote"]
    #[serde(rename = "application/vnd.ntt-local.sip-ta_remote")]
    VndNttLocalSipTaRemote,
    #[doc = "application/vnd.ntt-local.sip-ta_tcp_stream"]
    #[serde(rename = "application/vnd.ntt-local.sip-ta_tcp_stream")]
    VndNttLocalSipTaTcpStream,
    #[doc = "application/vnd.oasis.opendocument.base"]
    #[serde(rename = "application/vnd.oasis.opendocument.base")]
    #[serde(alias = "odb")]
    VndOasisOpendocumentBase,
    #[doc = "application/vnd.oasis.opendocument.chart"]
    #[serde(rename = "application/vnd.oasis.opendocument.chart")]
    #[serde(alias = "odc")]
    VndOasisOpendocumentChart,
    #[doc = "application/vnd.oasis.opendocument.chart-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.chart-template")]
    #[serde(alias = "otc")]
    VndOasisOpendocumentChartTemplate,
    #[doc = "application/vnd.oasis.opendocument.formula"]
    #[serde(rename = "application/vnd.oasis.opendocument.formula")]
    #[serde(alias = "odf")]
    VndOasisOpendocumentFormula,
    #[doc = "application/vnd.oasis.opendocument.formula-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.formula-template")]
    VndOasisOpendocumentFormulaTemplate,
    #[doc = "application/vnd.oasis.opendocument.graphics"]
    #[serde(rename = "application/vnd.oasis.opendocument.graphics")]
    #[serde(alias = "odg")]
    VndOasisOpendocumentGraphics,
    #[doc = "application/vnd.oasis.opendocument.graphics-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.graphics-template")]
    #[serde(alias = "otg")]
    VndOasisOpendocumentGraphicsTemplate,
    #[doc = "application/vnd.oasis.opendocument.image"]
    #[serde(rename = "application/vnd.oasis.opendocument.image")]
    #[serde(alias = "odi")]
    VndOasisOpendocumentImage,
    #[doc = "application/vnd.oasis.opendocument.image-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.image-template")]
    #[serde(alias = "oti")]
    VndOasisOpendocumentImageTemplate,
    #[doc = "application/vnd.oasis.opendocument.presentation"]
    #[serde(rename = "application/vnd.oasis.opendocument.presentation")]
    #[serde(alias = "odp")]
    VndOasisOpendocumentPresentation,
    #[doc = "application/vnd.oasis.opendocument.presentation-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.presentation-template")]
    #[serde(alias = "otp")]
    VndOasisOpendocumentPresentationTemplate,
    #[doc = "application/vnd.oasis.opendocument.spreadsheet"]
    #[serde(rename = "application/vnd.oasis.opendocument.spreadsheet")]
    #[serde(alias = "ods")]
    VndOasisOpendocumentSpreadsheet,
    #[doc = "application/vnd.oasis.opendocument.spreadsheet-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.spreadsheet-template")]
    #[serde(alias = "ots")]
    VndOasisOpendocumentSpreadsheetTemplate,
    #[doc = "application/vnd.oasis.opendocument.text"]
    #[serde(rename = "application/vnd.oasis.opendocument.text")]
    #[serde(alias = "odt")]
    VndOasisOpendocumentText,
    #[doc = "application/vnd.oasis.opendocument.text-master"]
    #[serde(rename = "application/vnd.oasis.opendocument.text-master")]
    #[serde(alias = "odm")]
    VndOasisOpendocumentTextMaster,
    #[doc = "application/vnd.oasis.opendocument.text-master-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.text-master-template")]
    #[serde(alias = "otm")]
    VndOasisOpendocumentTextMasterTemplate,
    #[doc = "application/vnd.oasis.opendocument.text-template"]
    #[serde(rename = "application/vnd.oasis.opendocument.text-template")]
    #[serde(alias = "ott")]
    VndOasisOpendocumentTextTemplate,
    #[doc = "application/vnd.oasis.opendocument.text-web"]
    #[serde(rename = "application/vnd.oasis.opendocument.text-web")]
    #[serde(alias = "oth")]
    VndOasisOpendocumentTextWeb,
    #[doc = "application/vnd.obn"]
    #[serde(rename = "application/vnd.obn")]
    VndObn,
    #[doc = "application/vnd.ocf+cbor"]
    #[serde(rename = "application/vnd.ocf+cbor")]
    VndOcfCbor,
    #[doc = "application/vnd.oci.image.manifest.v1+json"]
    #[serde(rename = "application/vnd.oci.image.manifest.v1+json")]
    VndOciImageManifestV1Json,
    #[doc = "application/vnd.oftn.l10n+json"]
    #[serde(rename = "application/vnd.oftn.l10n+json")]
    VndOftnL10NJson,
    #[doc = "application/vnd.oipf.contentaccessdownload+xml"]
    #[serde(rename = "application/vnd.oipf.contentaccessdownload+xml")]
    VndOipfContentaccessdownloadXml,
    #[doc = "application/vnd.oipf.contentaccessstreaming+xml"]
    #[serde(rename = "application/vnd.oipf.contentaccessstreaming+xml")]
    VndOipfContentaccessstreamingXml,
    #[doc = "application/vnd.oipf.cspg-hexbinary"]
    #[serde(rename = "application/vnd.oipf.cspg-hexbinary")]
    VndOipfCspgHexbinary,
    #[doc = "application/vnd.oipf.dae.svg+xml"]
    #[serde(rename = "application/vnd.oipf.dae.svg+xml")]
    VndOipfDaeSvgXml,
    #[doc = "application/vnd.oipf.dae.xhtml+xml"]
    #[serde(rename = "application/vnd.oipf.dae.xhtml+xml")]
    VndOipfDaeXhtmlXml,
    #[doc = "application/vnd.oipf.mippvcontrolmessage+xml"]
    #[serde(rename = "application/vnd.oipf.mippvcontrolmessage+xml")]
    VndOipfMippvcontrolmessageXml,
    #[doc = "application/vnd.oipf.pae.gem"]
    #[serde(rename = "application/vnd.oipf.pae.gem")]
    VndOipfPaeGem,
    #[doc = "application/vnd.oipf.spdiscovery+xml"]
    #[serde(rename = "application/vnd.oipf.spdiscovery+xml")]
    VndOipfSpdiscoveryXml,
    #[doc = "application/vnd.oipf.spdlist+xml"]
    #[serde(rename = "application/vnd.oipf.spdlist+xml")]
    VndOipfSpdlistXml,
    #[doc = "application/vnd.oipf.ueprofile+xml"]
    #[serde(rename = "application/vnd.oipf.ueprofile+xml")]
    VndOipfUeprofileXml,
    #[doc = "application/vnd.oipf.userprofile+xml"]
    #[serde(rename = "application/vnd.oipf.userprofile+xml")]
    VndOipfUserprofileXml,
    #[doc = "application/vnd.olpc-sugar"]
    #[serde(rename = "application/vnd.olpc-sugar")]
    #[serde(alias = "xo")]
    VndOlpcSugar,
    #[doc = "application/vnd.oma.bcast.associated-procedure-parameter+xml"]
    #[serde(rename = "application/vnd.oma.bcast.associated-procedure-parameter+xml")]
    VndOmaBcastAssociatedProcedureParameterXml,
    #[doc = "application/vnd.oma.bcast.drm-trigger+xml"]
    #[serde(rename = "application/vnd.oma.bcast.drm-trigger+xml")]
    VndOmaBcastDrmTriggerXml,
    #[doc = "application/vnd.oma.bcast.imd+xml"]
    #[serde(rename = "application/vnd.oma.bcast.imd+xml")]
    VndOmaBcastImdXml,
    #[doc = "application/vnd.oma.bcast.ltkm"]
    #[serde(rename = "application/vnd.oma.bcast.ltkm")]
    VndOmaBcastLtkm,
    #[doc = "application/vnd.oma.bcast.notification+xml"]
    #[serde(rename = "application/vnd.oma.bcast.notification+xml")]
    VndOmaBcastNotificationXml,
    #[doc = "application/vnd.oma.bcast.provisioningtrigger"]
    #[serde(rename = "application/vnd.oma.bcast.provisioningtrigger")]
    VndOmaBcastProvisioningtrigger,
    #[doc = "application/vnd.oma.bcast.sgboot"]
    #[serde(rename = "application/vnd.oma.bcast.sgboot")]
    VndOmaBcastSgboot,
    #[doc = "application/vnd.oma.bcast.sgdd+xml"]
    #[serde(rename = "application/vnd.oma.bcast.sgdd+xml")]
    VndOmaBcastSgddXml,
    #[doc = "application/vnd.oma.bcast.sgdu"]
    #[serde(rename = "application/vnd.oma.bcast.sgdu")]
    VndOmaBcastSgdu,
    #[doc = "application/vnd.oma.bcast.simple-symbol-container"]
    #[serde(rename = "application/vnd.oma.bcast.simple-symbol-container")]
    VndOmaBcastSimpleSymbolContainer,
    #[doc = "application/vnd.oma.bcast.smartcard-trigger+xml"]
    #[serde(rename = "application/vnd.oma.bcast.smartcard-trigger+xml")]
    VndOmaBcastSmartcardTriggerXml,
    #[doc = "application/vnd.oma.bcast.sprov+xml"]
    #[serde(rename = "application/vnd.oma.bcast.sprov+xml")]
    VndOmaBcastSprovXml,
    #[doc = "application/vnd.oma.bcast.stkm"]
    #[serde(rename = "application/vnd.oma.bcast.stkm")]
    VndOmaBcastStkm,
    #[doc = "application/vnd.oma.cab-address-book+xml"]
    #[serde(rename = "application/vnd.oma.cab-address-book+xml")]
    VndOmaCabAddressBookXml,
    #[doc = "application/vnd.oma.cab-feature-handler+xml"]
    #[serde(rename = "application/vnd.oma.cab-feature-handler+xml")]
    VndOmaCabFeatureHandlerXml,
    #[doc = "application/vnd.oma.cab-pcc+xml"]
    #[serde(rename = "application/vnd.oma.cab-pcc+xml")]
    VndOmaCabPccXml,
    #[doc = "application/vnd.oma.cab-subs-invite+xml"]
    #[serde(rename = "application/vnd.oma.cab-subs-invite+xml")]
    VndOmaCabSubsInviteXml,
    #[doc = "application/vnd.oma.cab-user-prefs+xml"]
    #[serde(rename = "application/vnd.oma.cab-user-prefs+xml")]
    VndOmaCabUserPrefsXml,
    #[doc = "application/vnd.oma.dcd"]
    #[serde(rename = "application/vnd.oma.dcd")]
    VndOmaDcd,
    #[doc = "application/vnd.oma.dcdc"]
    #[serde(rename = "application/vnd.oma.dcdc")]
    VndOmaDcdc,
    #[doc = "application/vnd.oma.dd2+xml"]
    #[serde(rename = "application/vnd.oma.dd2+xml")]
    #[serde(alias = "dd2")]
    VndOmaDd2Xml,
    #[doc = "application/vnd.oma.drm.risd+xml"]
    #[serde(rename = "application/vnd.oma.drm.risd+xml")]
    VndOmaDrmRisdXml,
    #[doc = "application/vnd.oma.group-usage-list+xml"]
    #[serde(rename = "application/vnd.oma.group-usage-list+xml")]
    VndOmaGroupUsageListXml,
    #[doc = "application/vnd.oma.lwm2m+cbor"]
    #[serde(rename = "application/vnd.oma.lwm2m+cbor")]
    VndOmaLwm2MCbor,
    #[doc = "application/vnd.oma.lwm2m+json"]
    #[serde(rename = "application/vnd.oma.lwm2m+json")]
    VndOmaLwm2MJson,
    #[doc = "application/vnd.oma.lwm2m+tlv"]
    #[serde(rename = "application/vnd.oma.lwm2m+tlv")]
    VndOmaLwm2MTlv,
    #[doc = "application/vnd.oma.pal+xml"]
    #[serde(rename = "application/vnd.oma.pal+xml")]
    VndOmaPalXml,
    #[doc = "application/vnd.oma.poc.detailed-progress-report+xml"]
    #[serde(rename = "application/vnd.oma.poc.detailed-progress-report+xml")]
    VndOmaPocDetailedProgressReportXml,
    #[doc = "application/vnd.oma.poc.final-report+xml"]
    #[serde(rename = "application/vnd.oma.poc.final-report+xml")]
    VndOmaPocFinalReportXml,
    #[doc = "application/vnd.oma.poc.groups+xml"]
    #[serde(rename = "application/vnd.oma.poc.groups+xml")]
    VndOmaPocGroupsXml,
    #[doc = "application/vnd.oma.poc.invocation-descriptor+xml"]
    #[serde(rename = "application/vnd.oma.poc.invocation-descriptor+xml")]
    VndOmaPocInvocationDescriptorXml,
    #[doc = "application/vnd.oma.poc.optimized-progress-report+xml"]
    #[serde(rename = "application/vnd.oma.poc.optimized-progress-report+xml")]
    VndOmaPocOptimizedProgressReportXml,
    #[doc = "application/vnd.oma.push"]
    #[serde(rename = "application/vnd.oma.push")]
    VndOmaPush,
    #[doc = "application/vnd.oma.scidm.messages+xml"]
    #[serde(rename = "application/vnd.oma.scidm.messages+xml")]
    VndOmaScidmMessagesXml,
    #[doc = "application/vnd.oma.xcap-directory+xml"]
    #[serde(rename = "application/vnd.oma.xcap-directory+xml")]
    VndOmaXcapDirectoryXml,
    #[doc = "application/vnd.omads-email+xml"]
    #[serde(rename = "application/vnd.omads-email+xml")]
    VndOmadsEmailXml,
    #[doc = "application/vnd.omads-file+xml"]
    #[serde(rename = "application/vnd.omads-file+xml")]
    VndOmadsFileXml,
    #[doc = "application/vnd.omads-folder+xml"]
    #[serde(rename = "application/vnd.omads-folder+xml")]
    VndOmadsFolderXml,
    #[doc = "application/vnd.omaloc-supl-init"]
    #[serde(rename = "application/vnd.omaloc-supl-init")]
    VndOmalocSuplInit,
    #[doc = "application/vnd.oma-scws-config"]
    #[serde(rename = "application/vnd.oma-scws-config")]
    VndOmaScwsConfig,
    #[doc = "application/vnd.oma-scws-http-request"]
    #[serde(rename = "application/vnd.oma-scws-http-request")]
    VndOmaScwsHttpRequest,
    #[doc = "application/vnd.oma-scws-http-response"]
    #[serde(rename = "application/vnd.oma-scws-http-response")]
    VndOmaScwsHttpResponse,
    #[doc = "application/vnd.onepager"]
    #[serde(rename = "application/vnd.onepager")]
    #[serde(alias = "tam")]
    VndOnepager,
    #[doc = "application/vnd.onepagertamp"]
    #[serde(rename = "application/vnd.onepagertamp")]
    #[serde(alias = "tamp")]
    VndOnepagertamp,
    #[doc = "application/vnd.onepagertamx"]
    #[serde(rename = "application/vnd.onepagertamx")]
    #[serde(alias = "tamx")]
    VndOnepagertamx,
    #[doc = "application/vnd.onepagertat"]
    #[serde(rename = "application/vnd.onepagertat")]
    #[serde(alias = "tat")]
    VndOnepagertat,
    #[doc = "application/vnd.onepagertatp"]
    #[serde(rename = "application/vnd.onepagertatp")]
    #[serde(alias = "tatp")]
    VndOnepagertatp,
    #[doc = "application/vnd.onepagertatx"]
    #[serde(rename = "application/vnd.onepagertatx")]
    #[serde(alias = "tatx")]
    VndOnepagertatx,
    #[doc = "application/vnd.onvif.metadata"]
    #[serde(rename = "application/vnd.onvif.metadata")]
    VndOnvifMetadata,
    #[doc = "application/vnd.openblox.game-binary"]
    #[serde(rename = "application/vnd.openblox.game-binary")]
    #[serde(alias = "obg")]
    VndOpenbloxGameBinary,
    #[doc = "application/vnd.openblox.game+xml"]
    #[serde(rename = "application/vnd.openblox.game+xml")]
    #[serde(alias = "obgx")]
    VndOpenbloxGameXml,
    #[doc = "application/vnd.openeye.oeb"]
    #[serde(rename = "application/vnd.openeye.oeb")]
    #[serde(alias = "oeb")]
    VndOpeneyeOeb,
    #[doc = "application/vnd.openstreetmap.data+xml"]
    #[serde(rename = "application/vnd.openstreetmap.data+xml")]
    #[serde(alias = "osm")]
    VndOpenstreetmapDataXml,
    #[doc = "application/vnd.opentimestamps.ots"]
    #[serde(rename = "application/vnd.opentimestamps.ots")]
    VndOpentimestampsOts,
    #[doc = "application/vnd.openxmlformats-officedocument.custom-properties+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.custom-properties+xml")]
    VndOpenxmlformatsOfficedocumentCustomPropertiesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml")]
    VndOpenxmlformatsOfficedocumentCustomXmlPropertiesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawing+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawing+xml")]
    VndOpenxmlformatsOfficedocumentDrawingXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml")]
    VndOpenxmlformatsOfficedocumentDrawingmlChartXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml")]
    VndOpenxmlformatsOfficedocumentDrawingmlChartshapesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml")]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramColorsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml")]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramDataXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml")]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramLayoutXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml")]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramStyleXml,
    #[doc = "application/vnd.openxmlformats-officedocument.extended-properties+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.extended-properties+xml")]
    VndOpenxmlformatsOfficedocumentExtendedPropertiesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlCommentAuthorsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.comments+xml")]
    VndOpenxmlformatsOfficedocumentPresentationmlCommentsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlHandoutMasterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlNotesMasterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlNotesSlideXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.presentation"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.presentation")]
    #[serde(alias = "pptx")]
    VndOpenxmlformatsOfficedocumentPresentationmlPresentation,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlPresentationMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml")]
    VndOpenxmlformatsOfficedocumentPresentationmlPresPropsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slide"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.slide")]
    #[serde(alias = "sldx")]
    VndOpenxmlformatsOfficedocumentPresentationmlSlide,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml")]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideLayoutXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideMasterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideshow"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.slideshow")]
    #[serde(alias = "ppsx")]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideshow,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideshowMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideUpdateInfoXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlTableStylesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml")]
    VndOpenxmlformatsOfficedocumentPresentationmlTagsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.template"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.template")]
    #[serde(alias = "potx")]
    VndOpenxmlformatsOfficedocumentPresentationmlTemplate,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"
    )]
    VndOpenxmlformatsOfficedocumentPresentationmlTemplateMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml")]
    VndOpenxmlformatsOfficedocumentPresentationmlViewPropsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlCalcChainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlChartsheetXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlCommentsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlConnectionsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlDialogsheetXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlExternalLinkXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheDefinitionXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheRecordsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlQueryTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionHeadersXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionLogXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSharedStringsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    #[serde(alias = "xlsx")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSheet,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMetadataXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlStylesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTableSingleCellsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.template"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.template")]
    #[serde(alias = "xltx")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplate,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplateMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlUserNamesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
    )]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlVolatileDependenciesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml")]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlWorksheetXml,
    #[doc = "application/vnd.openxmlformats-officedocument.theme+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.theme+xml")]
    VndOpenxmlformatsOfficedocumentThemeXml,
    #[doc = "application/vnd.openxmlformats-officedocument.themeOverride+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.themeOverride+xml")]
    VndOpenxmlformatsOfficedocumentThemeOverrideXml,
    #[doc = "application/vnd.openxmlformats-officedocument.vmlDrawing"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.vmlDrawing")]
    VndOpenxmlformatsOfficedocumentVmlDrawing,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlCommentsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.document"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    #[serde(alias = "docx")]
    VndOpenxmlformatsOfficedocumentWordprocessingmlDocument,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentGlossaryXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlEndnotesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlFontTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml")]
    VndOpenxmlformatsOfficedocumentWordprocessingmlFooterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlFootnotesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlNumberingXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlSettingsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml")]
    VndOpenxmlformatsOfficedocumentWordprocessingmlStylesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.template"]
    #[serde(rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.template")]
    #[serde(alias = "dotx")]
    VndOpenxmlformatsOfficedocumentWordprocessingmlTemplate,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlTemplateMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"]
    #[serde(
        rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
    )]
    VndOpenxmlformatsOfficedocumentWordprocessingmlWebSettingsXml,
    #[doc = "application/vnd.openxmlformats-package.core-properties+xml"]
    #[serde(rename = "application/vnd.openxmlformats-package.core-properties+xml")]
    VndOpenxmlformatsPackageCorePropertiesXml,
    #[doc = "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"]
    #[serde(rename = "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml")]
    VndOpenxmlformatsPackageDigitalSignatureXmlsignatureXml,
    #[doc = "application/vnd.openxmlformats-package.relationships+xml"]
    #[serde(rename = "application/vnd.openxmlformats-package.relationships+xml")]
    VndOpenxmlformatsPackageRelationshipsXml,
    #[doc = "application/vnd.oracle.resource+json"]
    #[serde(rename = "application/vnd.oracle.resource+json")]
    VndOracleResourceJson,
    #[doc = "application/vnd.orange.indata"]
    #[serde(rename = "application/vnd.orange.indata")]
    VndOrangeIndata,
    #[doc = "application/vnd.osa.netdeploy"]
    #[serde(rename = "application/vnd.osa.netdeploy")]
    #[serde(alias = "ndc")]
    VndOsaNetdeploy,
    #[doc = "application/vnd.osgeo.mapguide.package"]
    #[serde(rename = "application/vnd.osgeo.mapguide.package")]
    #[serde(alias = "mgp")]
    VndOsgeoMapguidePackage,
    #[doc = "application/vnd.osgi.bundle"]
    #[serde(rename = "application/vnd.osgi.bundle")]
    VndOsgiBundle,
    #[doc = "application/vnd.osgi.dp"]
    #[serde(rename = "application/vnd.osgi.dp")]
    #[serde(alias = "dp")]
    VndOsgiDp,
    #[doc = "application/vnd.osgi.subsystem"]
    #[serde(rename = "application/vnd.osgi.subsystem")]
    #[serde(alias = "esa")]
    VndOsgiSubsystem,
    #[doc = "application/vnd.otps.ct-kip+xml"]
    #[serde(rename = "application/vnd.otps.ct-kip+xml")]
    VndOtpsCtKipXml,
    #[doc = "application/vnd.oxli.countgraph"]
    #[serde(rename = "application/vnd.oxli.countgraph")]
    #[serde(alias = "oxlicg")]
    VndOxliCountgraph,
    #[doc = "application/vnd.pagerduty+json"]
    #[serde(rename = "application/vnd.pagerduty+json")]
    VndPagerdutyJson,
    #[doc = "application/vnd.palm"]
    #[serde(rename = "application/vnd.palm")]
    #[serde(alias = "prc")]
    #[serde(alias = "pdb")]
    #[serde(alias = "pqa")]
    #[serde(alias = "oprc")]
    VndPalm,
    #[doc = "application/vnd.panoply"]
    #[serde(rename = "application/vnd.panoply")]
    #[serde(alias = "plp")]
    VndPanoply,
    #[doc = "application/vnd.paos.xml"]
    #[serde(rename = "application/vnd.paos.xml")]
    VndPaosXml,
    #[doc = "application/vnd.patentdive"]
    #[serde(rename = "application/vnd.patentdive")]
    #[serde(alias = "dive")]
    VndPatentdive,
    #[doc = "application/vnd.patientecommsdoc"]
    #[serde(rename = "application/vnd.patientecommsdoc")]
    VndPatientecommsdoc,
    #[doc = "application/vnd.pawaafile"]
    #[serde(rename = "application/vnd.pawaafile")]
    #[serde(alias = "paw")]
    VndPawaafile,
    #[doc = "application/vnd.pcos"]
    #[serde(rename = "application/vnd.pcos")]
    VndPcos,
    #[doc = "application/vnd.pg.format"]
    #[serde(rename = "application/vnd.pg.format")]
    #[serde(alias = "str")]
    VndPgFormat,
    #[doc = "application/vnd.pg.osasli"]
    #[serde(rename = "application/vnd.pg.osasli")]
    #[serde(alias = "ei6")]
    VndPgOsasli,
    #[doc = "application/vnd.piaccess.application-licence"]
    #[serde(rename = "application/vnd.piaccess.application-licence")]
    #[serde(alias = "pil")]
    VndPiaccessApplicationLicence,
    #[doc = "application/vnd.picsel"]
    #[serde(rename = "application/vnd.picsel")]
    #[serde(alias = "efif")]
    VndPicsel,
    #[doc = "application/vnd.pmi.widget"]
    #[serde(rename = "application/vnd.pmi.widget")]
    #[serde(alias = "wg")]
    VndPmiWidget,
    #[doc = "application/vnd.poc.group-advertisement+xml"]
    #[serde(rename = "application/vnd.poc.group-advertisement+xml")]
    VndPocGroupAdvertisementXml,
    #[doc = "application/vnd.pocketlearn"]
    #[serde(rename = "application/vnd.pocketlearn")]
    #[serde(alias = "plf")]
    VndPocketlearn,
    #[doc = "application/vnd.powerbuilder6"]
    #[serde(rename = "application/vnd.powerbuilder6")]
    #[serde(alias = "pbd")]
    VndPowerbuilder6,
    #[doc = "application/vnd.powerbuilder6-s"]
    #[serde(rename = "application/vnd.powerbuilder6-s")]
    VndPowerbuilder6S,
    #[doc = "application/vnd.powerbuilder7"]
    #[serde(rename = "application/vnd.powerbuilder7")]
    VndPowerbuilder7,
    #[doc = "application/vnd.powerbuilder75"]
    #[serde(rename = "application/vnd.powerbuilder75")]
    VndPowerbuilder75,
    #[doc = "application/vnd.powerbuilder75-s"]
    #[serde(rename = "application/vnd.powerbuilder75-s")]
    VndPowerbuilder75S,
    #[doc = "application/vnd.powerbuilder7-s"]
    #[serde(rename = "application/vnd.powerbuilder7-s")]
    VndPowerbuilder7S,
    #[doc = "application/vnd.preminet"]
    #[serde(rename = "application/vnd.preminet")]
    #[serde(alias = "preminet")]
    VndPreminet,
    #[doc = "application/vnd.previewsystems.box"]
    #[serde(rename = "application/vnd.previewsystems.box")]
    #[serde(alias = "box")]
    #[serde(alias = "vbox")]
    VndPreviewsystemsBox,
    #[doc = "application/vnd.proteus.magazine"]
    #[serde(rename = "application/vnd.proteus.magazine")]
    #[serde(alias = "mgz")]
    VndProteusMagazine,
    #[doc = "application/vnd.psfs"]
    #[serde(rename = "application/vnd.psfs")]
    #[serde(alias = "psfs")]
    VndPsfs,
    #[doc = "application/vnd.pt.mundusmundi"]
    #[serde(rename = "application/vnd.pt.mundusmundi")]
    VndPtMundusmundi,
    #[doc = "application/vnd.publishare-delta-tree"]
    #[serde(rename = "application/vnd.publishare-delta-tree")]
    #[serde(alias = "qps")]
    VndPublishareDeltaTree,
    #[doc = "application/vnd.pvi.ptid1"]
    #[serde(rename = "application/vnd.pvi.ptid1")]
    #[serde(alias = "ptid")]
    VndPviPtid1,
    #[doc = "application/vnd.pwg-multiplexed"]
    #[serde(rename = "application/vnd.pwg-multiplexed")]
    VndPwgMultiplexed,
    #[doc = "application/vnd.pwg-xhtml-print+xml"]
    #[serde(rename = "application/vnd.pwg-xhtml-print+xml")]
    VndPwgXhtmlPrintXml,
    #[doc = "application/vnd.qualcomm.brew-app-res"]
    #[serde(rename = "application/vnd.qualcomm.brew-app-res")]
    #[serde(alias = "bar")]
    VndQualcommBrewAppRes,
    #[doc = "application/vnd.quarantainenet"]
    #[serde(rename = "application/vnd.quarantainenet")]
    VndQuarantainenet,
    #[doc = "application/vnd.Quark.QuarkXPress"]
    #[serde(rename = "application/vnd.Quark.QuarkXPress")]
    #[serde(alias = "qxd")]
    #[serde(alias = "qxt")]
    #[serde(alias = "qwd")]
    #[serde(alias = "qwt")]
    #[serde(alias = "qxl")]
    #[serde(alias = "qxb")]
    VndQuarkQuarkXPress,
    #[doc = "application/vnd.quobject-quoxdocument"]
    #[serde(rename = "application/vnd.quobject-quoxdocument")]
    #[serde(alias = "quox")]
    #[serde(alias = "quiz")]
    VndQuobjectQuoxdocument,
    #[doc = "application/vnd.radisys.moml+xml"]
    #[serde(rename = "application/vnd.radisys.moml+xml")]
    VndRadisysMomlXml,
    #[doc = "application/vnd.radisys.msml-audit-conf+xml"]
    #[serde(rename = "application/vnd.radisys.msml-audit-conf+xml")]
    VndRadisysMsmlAuditConfXml,
    #[doc = "application/vnd.radisys.msml-audit-conn+xml"]
    #[serde(rename = "application/vnd.radisys.msml-audit-conn+xml")]
    VndRadisysMsmlAuditConnXml,
    #[doc = "application/vnd.radisys.msml-audit-dialog+xml"]
    #[serde(rename = "application/vnd.radisys.msml-audit-dialog+xml")]
    VndRadisysMsmlAuditDialogXml,
    #[doc = "application/vnd.radisys.msml-audit-stream+xml"]
    #[serde(rename = "application/vnd.radisys.msml-audit-stream+xml")]
    VndRadisysMsmlAuditStreamXml,
    #[doc = "application/vnd.radisys.msml-audit+xml"]
    #[serde(rename = "application/vnd.radisys.msml-audit+xml")]
    VndRadisysMsmlAuditXml,
    #[doc = "application/vnd.radisys.msml-conf+xml"]
    #[serde(rename = "application/vnd.radisys.msml-conf+xml")]
    VndRadisysMsmlConfXml,
    #[doc = "application/vnd.radisys.msml-dialog-base+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog-base+xml")]
    VndRadisysMsmlDialogBaseXml,
    #[doc = "application/vnd.radisys.msml-dialog-fax-detect+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog-fax-detect+xml")]
    VndRadisysMsmlDialogFaxDetectXml,
    #[doc = "application/vnd.radisys.msml-dialog-fax-sendrecv+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog-fax-sendrecv+xml")]
    VndRadisysMsmlDialogFaxSendrecvXml,
    #[doc = "application/vnd.radisys.msml-dialog-group+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog-group+xml")]
    VndRadisysMsmlDialogGroupXml,
    #[doc = "application/vnd.radisys.msml-dialog-speech+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog-speech+xml")]
    VndRadisysMsmlDialogSpeechXml,
    #[doc = "application/vnd.radisys.msml-dialog-transform+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog-transform+xml")]
    VndRadisysMsmlDialogTransformXml,
    #[doc = "application/vnd.radisys.msml-dialog+xml"]
    #[serde(rename = "application/vnd.radisys.msml-dialog+xml")]
    VndRadisysMsmlDialogXml,
    #[doc = "application/vnd.radisys.msml+xml"]
    #[serde(rename = "application/vnd.radisys.msml+xml")]
    VndRadisysMsmlXml,
    #[doc = "application/vnd.rainstor.data"]
    #[serde(rename = "application/vnd.rainstor.data")]
    #[serde(alias = "tree")]
    VndRainstorData,
    #[doc = "application/vnd.rapid"]
    #[serde(rename = "application/vnd.rapid")]
    VndRapid,
    #[doc = "application/vnd.rar"]
    #[serde(rename = "application/vnd.rar")]
    #[serde(alias = "rar")]
    VndRar,
    #[doc = "application/vnd.realvnc.bed"]
    #[serde(rename = "application/vnd.realvnc.bed")]
    #[serde(alias = "bed")]
    VndRealvncBed,
    #[doc = "application/vnd.recordare.musicxml"]
    #[serde(rename = "application/vnd.recordare.musicxml")]
    #[serde(alias = "mxl")]
    VndRecordareMusicxml,
    #[doc = "application/vnd.recordare.musicxml+xml"]
    #[serde(rename = "application/vnd.recordare.musicxml+xml")]
    VndRecordareMusicxmlXml,
    #[doc = "application/vnd.RenLearn.rlprint"]
    #[serde(rename = "application/vnd.RenLearn.rlprint")]
    VndRenLearnRlprint,
    #[doc = "application/vnd.resilient.logic"]
    #[serde(rename = "application/vnd.resilient.logic")]
    #[serde(alias = "rlm")]
    #[serde(alias = "reload")]
    VndResilientLogic,
    #[doc = "application/vnd.restful+json"]
    #[serde(rename = "application/vnd.restful+json")]
    VndRestfulJson,
    #[doc = "application/vnd.rig.cryptonote"]
    #[serde(rename = "application/vnd.rig.cryptonote")]
    #[serde(alias = "cryptonote")]
    VndRigCryptonote,
    #[doc = "application/vnd.route66.link66+xml"]
    #[serde(rename = "application/vnd.route66.link66+xml")]
    #[serde(alias = "link66")]
    VndRoute66Link66Xml,
    #[doc = "application/vnd.rs-274x"]
    #[serde(rename = "application/vnd.rs-274x")]
    VndRs274X,
    #[doc = "application/vnd.ruckus.download"]
    #[serde(rename = "application/vnd.ruckus.download")]
    VndRuckusDownload,
    #[doc = "application/vnd.s3sms"]
    #[serde(rename = "application/vnd.s3sms")]
    VndS3Sms,
    #[doc = "application/vnd.sailingtracker.track"]
    #[serde(rename = "application/vnd.sailingtracker.track")]
    #[serde(alias = "st")]
    VndSailingtrackerTrack,
    #[doc = "application/vnd.sar"]
    #[serde(rename = "application/vnd.sar")]
    #[serde(alias = "SAR")]
    VndSar,
    #[doc = "application/vnd.sbm.cid"]
    #[serde(rename = "application/vnd.sbm.cid")]
    VndSbmCid,
    #[doc = "application/vnd.sbm.mid2"]
    #[serde(rename = "application/vnd.sbm.mid2")]
    VndSbmMid2,
    #[doc = "application/vnd.scribus"]
    #[serde(rename = "application/vnd.scribus")]
    #[serde(alias = "scd")]
    #[serde(alias = "sla")]
    #[serde(alias = "slaz")]
    VndScribus,
    #[doc = "application/vnd.sealed.3df"]
    #[serde(rename = "application/vnd.sealed.3df")]
    #[serde(alias = "s3df")]
    VndSealed3Df,
    #[doc = "application/vnd.sealed.csf"]
    #[serde(rename = "application/vnd.sealed.csf")]
    #[serde(alias = "scsf")]
    VndSealedCsf,
    #[doc = "application/vnd.sealed.doc"]
    #[serde(rename = "application/vnd.sealed.doc")]
    #[serde(alias = "sdoc")]
    #[serde(alias = "sdo")]
    #[serde(alias = "s1w")]
    VndSealedDoc,
    #[doc = "application/vnd.sealed.eml"]
    #[serde(rename = "application/vnd.sealed.eml")]
    #[serde(alias = "seml")]
    #[serde(alias = "sem")]
    VndSealedEml,
    #[doc = "application/vnd.sealed.mht"]
    #[serde(rename = "application/vnd.sealed.mht")]
    #[serde(alias = "smht")]
    #[serde(alias = "smh")]
    VndSealedMht,
    #[doc = "application/vnd.sealed.net"]
    #[serde(rename = "application/vnd.sealed.net")]
    VndSealedNet,
    #[doc = "application/vnd.sealed.ppt"]
    #[serde(rename = "application/vnd.sealed.ppt")]
    #[serde(alias = "sppt")]
    #[serde(alias = "s1p")]
    VndSealedPpt,
    #[doc = "application/vnd.sealed.tiff"]
    #[serde(rename = "application/vnd.sealed.tiff")]
    #[serde(alias = "stif")]
    VndSealedTiff,
    #[doc = "application/vnd.sealed.xls"]
    #[serde(rename = "application/vnd.sealed.xls")]
    #[serde(alias = "sxls")]
    #[serde(alias = "sxl")]
    #[serde(alias = "s1e")]
    VndSealedXls,
    #[doc = "application/vnd.sealedmedia.softseal.html"]
    #[serde(rename = "application/vnd.sealedmedia.softseal.html")]
    #[serde(alias = "stml")]
    #[serde(alias = "s1h")]
    VndSealedmediaSoftsealHtml,
    #[doc = "application/vnd.sealedmedia.softseal.pdf"]
    #[serde(rename = "application/vnd.sealedmedia.softseal.pdf")]
    #[serde(alias = "spdf")]
    #[serde(alias = "spd")]
    #[serde(alias = "s1a")]
    VndSealedmediaSoftsealPdf,
    #[doc = "application/vnd.seemail"]
    #[serde(rename = "application/vnd.seemail")]
    #[serde(alias = "see")]
    VndSeemail,
    #[doc = "application/vnd.seis+json"]
    #[serde(rename = "application/vnd.seis+json")]
    VndSeisJson,
    #[doc = "application/vnd.sema"]
    #[serde(rename = "application/vnd.sema")]
    #[serde(alias = "sema")]
    VndSema,
    #[doc = "application/vnd.semd"]
    #[serde(rename = "application/vnd.semd")]
    #[serde(alias = "semd")]
    VndSemd,
    #[doc = "application/vnd.semf"]
    #[serde(rename = "application/vnd.semf")]
    #[serde(alias = "semf")]
    VndSemf,
    #[doc = "application/vnd.shade-save-file"]
    #[serde(rename = "application/vnd.shade-save-file")]
    #[serde(alias = "ssv")]
    VndShadeSaveFile,
    #[doc = "application/vnd.shana.informed.formdata"]
    #[serde(rename = "application/vnd.shana.informed.formdata")]
    #[serde(alias = "ifm")]
    VndShanaInformedFormdata,
    #[doc = "application/vnd.shana.informed.formtemplate"]
    #[serde(rename = "application/vnd.shana.informed.formtemplate")]
    #[serde(alias = "itp")]
    VndShanaInformedFormtemplate,
    #[doc = "application/vnd.shana.informed.interchange"]
    #[serde(rename = "application/vnd.shana.informed.interchange")]
    #[serde(alias = "iif")]
    VndShanaInformedInterchange,
    #[doc = "application/vnd.shana.informed.package"]
    #[serde(rename = "application/vnd.shana.informed.package")]
    #[serde(alias = "ipk")]
    VndShanaInformedPackage,
    #[doc = "application/vnd.shootproof+json"]
    #[serde(rename = "application/vnd.shootproof+json")]
    VndShootproofJson,
    #[doc = "application/vnd.shopkick+json"]
    #[serde(rename = "application/vnd.shopkick+json")]
    VndShopkickJson,
    #[doc = "application/vnd.shp"]
    #[serde(rename = "application/vnd.shp")]
    #[serde(alias = "shp")]
    VndShp,
    #[doc = "application/vnd.shx"]
    #[serde(rename = "application/vnd.shx")]
    #[serde(alias = "shx")]
    VndShx,
    #[doc = "application/vnd.sigrok.session"]
    #[serde(rename = "application/vnd.sigrok.session")]
    #[serde(alias = "sr")]
    VndSigrokSession,
    #[doc = "application/vnd.SimTech-MindMapper"]
    #[serde(rename = "application/vnd.SimTech-MindMapper")]
    #[serde(alias = "twd")]
    #[serde(alias = "twds")]
    VndSimTechMindMapper,
    #[doc = "application/vnd.siren+json"]
    #[serde(rename = "application/vnd.siren+json")]
    VndSirenJson,
    #[doc = "application/vnd.smaf"]
    #[serde(rename = "application/vnd.smaf")]
    #[serde(alias = "mmf")]
    VndSmaf,
    #[doc = "application/vnd.smart.notebook"]
    #[serde(rename = "application/vnd.smart.notebook")]
    #[serde(alias = "notebook")]
    VndSmartNotebook,
    #[doc = "application/vnd.smart.teacher"]
    #[serde(rename = "application/vnd.smart.teacher")]
    #[serde(alias = "teacher")]
    VndSmartTeacher,
    #[doc = "application/vnd.smintio.portals.archive"]
    #[serde(rename = "application/vnd.smintio.portals.archive")]
    #[serde(alias = "sipa")]
    VndSmintioPortalsArchive,
    #[doc = "application/vnd.snesdev-page-table"]
    #[serde(rename = "application/vnd.snesdev-page-table")]
    #[serde(alias = "ptrom")]
    #[serde(alias = "pt")]
    VndSnesdevPageTable,
    #[doc = "application/vnd.software602.filler.form+xml"]
    #[serde(rename = "application/vnd.software602.filler.form+xml")]
    #[serde(alias = "fo")]
    VndSoftware602FillerFormXml,
    #[doc = "application/vnd.software602.filler.form-xml-zip"]
    #[serde(rename = "application/vnd.software602.filler.form-xml-zip")]
    #[serde(alias = "zfo")]
    VndSoftware602FillerFormXmlZip,
    #[doc = "application/vnd.solent.sdkm+xml"]
    #[serde(rename = "application/vnd.solent.sdkm+xml")]
    #[serde(alias = "sdkm")]
    #[serde(alias = "sdkd")]
    VndSolentSdkmXml,
    #[doc = "application/vnd.spotfire.dxp"]
    #[serde(rename = "application/vnd.spotfire.dxp")]
    #[serde(alias = "dxp")]
    VndSpotfireDxp,
    #[doc = "application/vnd.spotfire.sfs"]
    #[serde(rename = "application/vnd.spotfire.sfs")]
    #[serde(alias = "sfs")]
    VndSpotfireSfs,
    #[doc = "application/vnd.sqlite3"]
    #[serde(rename = "application/vnd.sqlite3")]
    #[serde(alias = "sqlite")]
    #[serde(alias = "sqlite3")]
    VndSqlite3,
    #[doc = "application/vnd.sss-cod"]
    #[serde(rename = "application/vnd.sss-cod")]
    VndSssCod,
    #[doc = "application/vnd.sss-dtf"]
    #[serde(rename = "application/vnd.sss-dtf")]
    VndSssDtf,
    #[doc = "application/vnd.sss-ntf"]
    #[serde(rename = "application/vnd.sss-ntf")]
    VndSssNtf,
    #[doc = "application/vnd.stepmania.package"]
    #[serde(rename = "application/vnd.stepmania.package")]
    #[serde(alias = "smzip")]
    VndStepmaniaPackage,
    #[doc = "application/vnd.stepmania.stepchart"]
    #[serde(rename = "application/vnd.stepmania.stepchart")]
    #[serde(alias = "sm")]
    VndStepmaniaStepchart,
    #[doc = "application/vnd.street-stream"]
    #[serde(rename = "application/vnd.street-stream")]
    VndStreetStream,
    #[doc = "application/vnd.sun.wadl+xml"]
    #[serde(rename = "application/vnd.sun.wadl+xml")]
    #[serde(alias = "wadl")]
    VndSunWadlXml,
    #[doc = "application/vnd.sus-calendar"]
    #[serde(rename = "application/vnd.sus-calendar")]
    #[serde(alias = "sus")]
    #[serde(alias = "susp")]
    VndSusCalendar,
    #[doc = "application/vnd.svd"]
    #[serde(rename = "application/vnd.svd")]
    VndSvd,
    #[doc = "application/vnd.swiftview-ics"]
    #[serde(rename = "application/vnd.swiftview-ics")]
    VndSwiftviewIcs,
    #[doc = "application/vnd.sybyl.mol2"]
    #[serde(rename = "application/vnd.sybyl.mol2")]
    #[serde(alias = "ml2")]
    #[serde(alias = "mol2")]
    #[serde(alias = "sy2")]
    VndSybylMol2,
    #[doc = "application/vnd.sycle+xml"]
    #[serde(rename = "application/vnd.sycle+xml")]
    #[serde(alias = "scl")]
    VndSycleXml,
    #[doc = "application/vnd.syft+json"]
    #[serde(rename = "application/vnd.syft+json")]
    #[serde(alias = "syft.json")]
    VndSyftJson,
    #[doc = "application/vnd.syncml.dm.notification"]
    #[serde(rename = "application/vnd.syncml.dm.notification")]
    VndSyncmlDmNotification,
    #[doc = "application/vnd.syncml.dmddf+xml"]
    #[serde(rename = "application/vnd.syncml.dmddf+xml")]
    #[serde(alias = "ddf")]
    VndSyncmlDmddfXml,
    #[doc = "application/vnd.syncml.dmtnds+wbxml"]
    #[serde(rename = "application/vnd.syncml.dmtnds+wbxml")]
    VndSyncmlDmtndsWbxml,
    #[doc = "application/vnd.syncml.dmtnds+xml"]
    #[serde(rename = "application/vnd.syncml.dmtnds+xml")]
    VndSyncmlDmtndsXml,
    #[doc = "application/vnd.syncml.dmddf+wbxml"]
    #[serde(rename = "application/vnd.syncml.dmddf+wbxml")]
    VndSyncmlDmddfWbxml,
    #[doc = "application/vnd.syncml.dm+wbxml"]
    #[serde(rename = "application/vnd.syncml.dm+wbxml")]
    #[serde(alias = "bdm")]
    VndSyncmlDmWbxml,
    #[doc = "application/vnd.syncml.dm+xml"]
    #[serde(rename = "application/vnd.syncml.dm+xml")]
    #[serde(alias = "xdm")]
    VndSyncmlDmXml,
    #[doc = "application/vnd.syncml.ds.notification"]
    #[serde(rename = "application/vnd.syncml.ds.notification")]
    VndSyncmlDsNotification,
    #[doc = "application/vnd.syncml+xml"]
    #[serde(rename = "application/vnd.syncml+xml")]
    #[serde(alias = "xsm")]
    VndSyncmlXml,
    #[doc = "application/vnd.tableschema+json"]
    #[serde(rename = "application/vnd.tableschema+json")]
    VndTableschemaJson,
    #[doc = "application/vnd.tao.intent-module-archive"]
    #[serde(rename = "application/vnd.tao.intent-module-archive")]
    #[serde(alias = "tao")]
    VndTaoIntentModuleArchive,
    #[doc = "application/vnd.tcpdump.pcap"]
    #[serde(rename = "application/vnd.tcpdump.pcap")]
    #[serde(alias = "pcap")]
    #[serde(alias = "cap")]
    #[serde(alias = "dmp")]
    VndTcpdumpPcap,
    #[doc = "application/vnd.think-cell.ppttc+json"]
    #[serde(rename = "application/vnd.think-cell.ppttc+json")]
    #[serde(alias = "ppttc")]
    VndThinkCellPpttcJson,
    #[doc = "application/vnd.tml"]
    #[serde(rename = "application/vnd.tml")]
    #[serde(alias = "vfr")]
    #[serde(alias = "viaframe")]
    VndTml,
    #[doc = "application/vnd.tmd.mediaflex.api+xml"]
    #[serde(rename = "application/vnd.tmd.mediaflex.api+xml")]
    VndTmdMediaflexApiXml,
    #[doc = "application/vnd.tmobile-livetv"]
    #[serde(rename = "application/vnd.tmobile-livetv")]
    #[serde(alias = "tmo")]
    VndTmobileLivetv,
    #[doc = "application/vnd.tri.onesource"]
    #[serde(rename = "application/vnd.tri.onesource")]
    VndTriOnesource,
    #[doc = "application/vnd.trid.tpt"]
    #[serde(rename = "application/vnd.trid.tpt")]
    #[serde(alias = "tpt")]
    VndTridTpt,
    #[doc = "application/vnd.triscape.mxs"]
    #[serde(rename = "application/vnd.triscape.mxs")]
    #[serde(alias = "mxs")]
    VndTriscapeMxs,
    #[doc = "application/vnd.trueapp"]
    #[serde(rename = "application/vnd.trueapp")]
    #[serde(alias = "tra")]
    VndTrueapp,
    #[doc = "application/vnd.truedoc"]
    #[serde(rename = "application/vnd.truedoc")]
    VndTruedoc,
    #[doc = "application/vnd.ubisoft.webplayer"]
    #[serde(rename = "application/vnd.ubisoft.webplayer")]
    VndUbisoftWebplayer,
    #[doc = "application/vnd.ufdl"]
    #[serde(rename = "application/vnd.ufdl")]
    #[serde(alias = "ufdl")]
    #[serde(alias = "ufd")]
    #[serde(alias = "frm")]
    VndUfdl,
    #[doc = "application/vnd.uiq.theme"]
    #[serde(rename = "application/vnd.uiq.theme")]
    #[serde(alias = "utz")]
    VndUiqTheme,
    #[doc = "application/vnd.umajin"]
    #[serde(rename = "application/vnd.umajin")]
    #[serde(alias = "umj")]
    VndUmajin,
    #[doc = "application/vnd.unity"]
    #[serde(rename = "application/vnd.unity")]
    #[serde(alias = "unityweb")]
    VndUnity,
    #[doc = "application/vnd.uoml+xml"]
    #[serde(rename = "application/vnd.uoml+xml")]
    #[serde(alias = "uoml")]
    #[serde(alias = "uo")]
    VndUomlXml,
    #[doc = "application/vnd.uplanet.alert"]
    #[serde(rename = "application/vnd.uplanet.alert")]
    VndUplanetAlert,
    #[doc = "application/vnd.uplanet.alert-wbxml"]
    #[serde(rename = "application/vnd.uplanet.alert-wbxml")]
    VndUplanetAlertWbxml,
    #[doc = "application/vnd.uplanet.bearer-choice"]
    #[serde(rename = "application/vnd.uplanet.bearer-choice")]
    VndUplanetBearerChoice,
    #[doc = "application/vnd.uplanet.bearer-choice-wbxml"]
    #[serde(rename = "application/vnd.uplanet.bearer-choice-wbxml")]
    VndUplanetBearerChoiceWbxml,
    #[doc = "application/vnd.uplanet.cacheop"]
    #[serde(rename = "application/vnd.uplanet.cacheop")]
    VndUplanetCacheop,
    #[doc = "application/vnd.uplanet.cacheop-wbxml"]
    #[serde(rename = "application/vnd.uplanet.cacheop-wbxml")]
    VndUplanetCacheopWbxml,
    #[doc = "application/vnd.uplanet.channel"]
    #[serde(rename = "application/vnd.uplanet.channel")]
    VndUplanetChannel,
    #[doc = "application/vnd.uplanet.channel-wbxml"]
    #[serde(rename = "application/vnd.uplanet.channel-wbxml")]
    VndUplanetChannelWbxml,
    #[doc = "application/vnd.uplanet.list"]
    #[serde(rename = "application/vnd.uplanet.list")]
    VndUplanetList,
    #[doc = "application/vnd.uplanet.listcmd"]
    #[serde(rename = "application/vnd.uplanet.listcmd")]
    VndUplanetListcmd,
    #[doc = "application/vnd.uplanet.listcmd-wbxml"]
    #[serde(rename = "application/vnd.uplanet.listcmd-wbxml")]
    VndUplanetListcmdWbxml,
    #[doc = "application/vnd.uplanet.list-wbxml"]
    #[serde(rename = "application/vnd.uplanet.list-wbxml")]
    VndUplanetListWbxml,
    #[doc = "application/vnd.uri-map"]
    #[serde(rename = "application/vnd.uri-map")]
    #[serde(alias = "urim")]
    #[serde(alias = "urimap")]
    VndUriMap,
    #[doc = "application/vnd.uplanet.signal"]
    #[serde(rename = "application/vnd.uplanet.signal")]
    VndUplanetSignal,
    #[doc = "application/vnd.valve.source.material"]
    #[serde(rename = "application/vnd.valve.source.material")]
    #[serde(alias = "vmt")]
    VndValveSourceMaterial,
    #[doc = "application/vnd.vcx"]
    #[serde(rename = "application/vnd.vcx")]
    #[serde(alias = "vcx")]
    VndVcx,
    #[doc = "application/vnd.vd-study"]
    #[serde(rename = "application/vnd.vd-study")]
    #[serde(alias = "mxi")]
    #[serde(alias = "study-inter")]
    #[serde(alias = "model-inter")]
    VndVdStudy,
    #[doc = "application/vnd.vectorworks"]
    #[serde(rename = "application/vnd.vectorworks")]
    #[serde(alias = "vwx")]
    VndVectorworks,
    #[doc = "application/vnd.vel+json"]
    #[serde(rename = "application/vnd.vel+json")]
    VndVelJson,
    #[doc = "application/vnd.verimatrix.vcas"]
    #[serde(rename = "application/vnd.verimatrix.vcas")]
    VndVerimatrixVcas,
    #[doc = "application/vnd.veritone.aion+json"]
    #[serde(rename = "application/vnd.veritone.aion+json")]
    #[serde(alias = "aion")]
    #[serde(alias = "vtnstd")]
    VndVeritoneAionJson,
    #[doc = "application/vnd.veryant.thin"]
    #[serde(rename = "application/vnd.veryant.thin")]
    #[serde(alias = "istc")]
    #[serde(alias = "isws")]
    VndVeryantThin,
    #[doc = "application/vnd.ves.encrypted"]
    #[serde(rename = "application/vnd.ves.encrypted")]
    #[serde(alias = "VES")]
    VndVesEncrypted,
    #[doc = "application/vnd.vidsoft.vidconference"]
    #[serde(rename = "application/vnd.vidsoft.vidconference")]
    #[serde(alias = "vsc")]
    VndVidsoftVidconference,
    #[doc = "application/vnd.visio"]
    #[serde(rename = "application/vnd.visio")]
    #[serde(alias = "vsd")]
    #[serde(alias = "vst")]
    #[serde(alias = "vsw")]
    #[serde(alias = "vss")]
    VndVisio,
    #[doc = "application/vnd.visionary"]
    #[serde(rename = "application/vnd.visionary")]
    #[serde(alias = "vis")]
    VndVisionary,
    #[doc = "application/vnd.vividence.scriptfile"]
    #[serde(rename = "application/vnd.vividence.scriptfile")]
    VndVividenceScriptfile,
    #[doc = "application/vnd.vsf"]
    #[serde(rename = "application/vnd.vsf")]
    #[serde(alias = "vsf")]
    VndVsf,
    #[doc = "application/vnd.wap.sic"]
    #[serde(rename = "application/vnd.wap.sic")]
    #[serde(alias = "sic")]
    VndWapSic,
    #[doc = "application/vnd.wap.slc"]
    #[serde(rename = "application/vnd.wap.slc")]
    #[serde(alias = "slc")]
    VndWapSlc,
    #[doc = "application/vnd.wap.wbxml"]
    #[serde(rename = "application/vnd.wap.wbxml")]
    #[serde(alias = "wbxml")]
    VndWapWbxml,
    #[doc = "application/vnd.wap.wmlc"]
    #[serde(rename = "application/vnd.wap.wmlc")]
    #[serde(alias = "wmlc")]
    VndWapWmlc,
    #[doc = "application/vnd.wap.wmlscriptc"]
    #[serde(rename = "application/vnd.wap.wmlscriptc")]
    #[serde(alias = "wmlsc")]
    VndWapWmlscriptc,
    #[doc = "application/vnd.wasmflow.wafl"]
    #[serde(rename = "application/vnd.wasmflow.wafl")]
    #[serde(alias = "wafl")]
    VndWasmflowWafl,
    #[doc = "application/vnd.webturbo"]
    #[serde(rename = "application/vnd.webturbo")]
    #[serde(alias = "wtb")]
    VndWebturbo,
    #[doc = "application/vnd.wfa.dpp"]
    #[serde(rename = "application/vnd.wfa.dpp")]
    VndWfaDpp,
    #[doc = "application/vnd.wfa.p2p"]
    #[serde(rename = "application/vnd.wfa.p2p")]
    #[serde(alias = "p2p")]
    VndWfaP2P,
    #[doc = "application/vnd.wfa.wsc"]
    #[serde(rename = "application/vnd.wfa.wsc")]
    #[serde(alias = "wsc")]
    VndWfaWsc,
    #[doc = "application/vnd.windows.devicepairing"]
    #[serde(rename = "application/vnd.windows.devicepairing")]
    VndWindowsDevicepairing,
    #[doc = "application/vnd.wmc"]
    #[serde(rename = "application/vnd.wmc")]
    #[serde(alias = "wmc")]
    VndWmc,
    #[doc = "application/vnd.wmf.bootstrap"]
    #[serde(rename = "application/vnd.wmf.bootstrap")]
    VndWmfBootstrap,
    #[doc = "application/vnd.wolfram.mathematica"]
    #[serde(rename = "application/vnd.wolfram.mathematica")]
    VndWolframMathematica,
    #[doc = "application/vnd.wolfram.mathematica.package"]
    #[serde(rename = "application/vnd.wolfram.mathematica.package")]
    #[serde(alias = "m")]
    VndWolframMathematicaPackage,
    #[doc = "application/vnd.wolfram.player"]
    #[serde(rename = "application/vnd.wolfram.player")]
    #[serde(alias = "nbp")]
    VndWolframPlayer,
    #[doc = "application/vnd.wordlift"]
    #[serde(rename = "application/vnd.wordlift")]
    VndWordlift,
    #[doc = "application/vnd.wordperfect"]
    #[serde(rename = "application/vnd.wordperfect")]
    #[serde(alias = "wpd")]
    VndWordperfect,
    #[doc = "application/vnd.wqd"]
    #[serde(rename = "application/vnd.wqd")]
    #[serde(alias = "wqd")]
    VndWqd,
    #[doc = "application/vnd.wrq-hp3000-labelled"]
    #[serde(rename = "application/vnd.wrq-hp3000-labelled")]
    VndWrqHp3000Labelled,
    #[doc = "application/vnd.wt.stf"]
    #[serde(rename = "application/vnd.wt.stf")]
    #[serde(alias = "stf")]
    VndWtStf,
    #[doc = "application/vnd.wv.csp+xml"]
    #[serde(rename = "application/vnd.wv.csp+xml")]
    VndWvCspXml,
    #[doc = "application/vnd.wv.csp+wbxml"]
    #[serde(rename = "application/vnd.wv.csp+wbxml")]
    #[serde(alias = "wv")]
    VndWvCspWbxml,
    #[doc = "application/vnd.wv.ssp+xml"]
    #[serde(rename = "application/vnd.wv.ssp+xml")]
    VndWvSspXml,
    #[doc = "application/vnd.xacml+json"]
    #[serde(rename = "application/vnd.xacml+json")]
    VndXacmlJson,
    #[doc = "application/vnd.xara"]
    #[serde(rename = "application/vnd.xara")]
    #[serde(alias = "xar")]
    VndXara,
    #[doc = "application/vnd.xfdl"]
    #[serde(rename = "application/vnd.xfdl")]
    #[serde(alias = "xfdl")]
    #[serde(alias = "xfd")]
    VndXfdl,
    #[doc = "application/vnd.xfdl.webform"]
    #[serde(rename = "application/vnd.xfdl.webform")]
    VndXfdlWebform,
    #[doc = "application/vnd.xmi+xml"]
    #[serde(rename = "application/vnd.xmi+xml")]
    VndXmiXml,
    #[doc = "application/vnd.xmpie.cpkg"]
    #[serde(rename = "application/vnd.xmpie.cpkg")]
    #[serde(alias = "cpkg")]
    VndXmpieCpkg,
    #[doc = "application/vnd.xmpie.dpkg"]
    #[serde(rename = "application/vnd.xmpie.dpkg")]
    #[serde(alias = "dpkg")]
    VndXmpieDpkg,
    #[doc = "application/vnd.xmpie.plan"]
    #[serde(rename = "application/vnd.xmpie.plan")]
    VndXmpiePlan,
    #[doc = "application/vnd.xmpie.ppkg"]
    #[serde(rename = "application/vnd.xmpie.ppkg")]
    #[serde(alias = "ppkg")]
    VndXmpiePpkg,
    #[doc = "application/vnd.xmpie.xlim"]
    #[serde(rename = "application/vnd.xmpie.xlim")]
    #[serde(alias = "xlim")]
    VndXmpieXlim,
    #[doc = "application/vnd.yamaha.hv-dic"]
    #[serde(rename = "application/vnd.yamaha.hv-dic")]
    #[serde(alias = "hvd")]
    VndYamahaHvDic,
    #[doc = "application/vnd.yamaha.hv-script"]
    #[serde(rename = "application/vnd.yamaha.hv-script")]
    #[serde(alias = "hvs")]
    VndYamahaHvScript,
    #[doc = "application/vnd.yamaha.hv-voice"]
    #[serde(rename = "application/vnd.yamaha.hv-voice")]
    #[serde(alias = "hvp")]
    VndYamahaHvVoice,
    #[doc = "application/vnd.yamaha.openscoreformat.osfpvg+xml"]
    #[serde(rename = "application/vnd.yamaha.openscoreformat.osfpvg+xml")]
    VndYamahaOpenscoreformatOsfpvgXml,
    #[doc = "application/vnd.yamaha.openscoreformat"]
    #[serde(rename = "application/vnd.yamaha.openscoreformat")]
    #[serde(alias = "osf")]
    VndYamahaOpenscoreformat,
    #[doc = "application/vnd.yamaha.remote-setup"]
    #[serde(rename = "application/vnd.yamaha.remote-setup")]
    VndYamahaRemoteSetup,
    #[doc = "application/vnd.yamaha.smaf-audio"]
    #[serde(rename = "application/vnd.yamaha.smaf-audio")]
    #[serde(alias = "saf")]
    VndYamahaSmafAudio,
    #[doc = "application/vnd.yamaha.smaf-phrase"]
    #[serde(rename = "application/vnd.yamaha.smaf-phrase")]
    #[serde(alias = "spf")]
    VndYamahaSmafPhrase,
    #[doc = "application/vnd.yamaha.through-ngn"]
    #[serde(rename = "application/vnd.yamaha.through-ngn")]
    VndYamahaThroughNgn,
    #[doc = "application/vnd.yamaha.tunnel-udpencap"]
    #[serde(rename = "application/vnd.yamaha.tunnel-udpencap")]
    VndYamahaTunnelUdpencap,
    #[doc = "application/vnd.yaoweme"]
    #[serde(rename = "application/vnd.yaoweme")]
    #[serde(alias = "yme")]
    VndYaoweme,
    #[doc = "application/vnd.yellowriver-custom-menu"]
    #[serde(rename = "application/vnd.yellowriver-custom-menu")]
    #[serde(alias = "cmp")]
    VndYellowriverCustomMenu,
    #[doc = "application/vnd.zul"]
    #[serde(rename = "application/vnd.zul")]
    #[serde(alias = "zir")]
    #[serde(alias = "zirz")]
    VndZul,
    #[doc = "application/vnd.zzazz.deck+xml"]
    #[serde(rename = "application/vnd.zzazz.deck+xml")]
    #[serde(alias = "zaz")]
    VndZzazzDeckXml,
    #[doc = "application/voicexml+xml"]
    #[serde(rename = "application/voicexml+xml")]
    #[serde(alias = "vxml")]
    VoicexmlXml,
    #[doc = "application/voucher-cms+json"]
    #[serde(rename = "application/voucher-cms+json")]
    #[serde(alias = "vcj")]
    VoucherCmsJson,
    #[doc = "application/vq-rtcpxr"]
    #[serde(rename = "application/vq-rtcpxr")]
    VqRtcpxr,
    #[doc = "application/wasm"]
    #[serde(rename = "application/wasm")]
    #[serde(alias = "wasm")]
    Wasm,
    #[doc = "application/watcherinfo+xml"]
    #[serde(rename = "application/watcherinfo+xml")]
    #[serde(alias = "wif")]
    WatcherinfoXml,
    #[doc = "application/webpush-options+json"]
    #[serde(rename = "application/webpush-options+json")]
    WebpushOptionsJson,
    #[doc = "application/whoispp-query"]
    #[serde(rename = "application/whoispp-query")]
    WhoisppQuery,
    #[doc = "application/whoispp-response"]
    #[serde(rename = "application/whoispp-response")]
    WhoisppResponse,
    #[doc = "application/widget"]
    #[serde(rename = "application/widget")]
    #[serde(alias = "wgt")]
    Widget,
    #[doc = "application/wita"]
    #[serde(rename = "application/wita")]
    Wita,
    #[doc = "application/wordperfect5.1"]
    #[serde(rename = "application/wordperfect5.1")]
    Wordperfect51,
    #[doc = "application/wsdl+xml"]
    #[serde(rename = "application/wsdl+xml")]
    #[serde(alias = "wsdl")]
    WsdlXml,
    #[doc = "application/wspolicy+xml"]
    #[serde(rename = "application/wspolicy+xml")]
    #[serde(alias = "wspolicy")]
    WspolicyXml,
    #[doc = "application/x-pki-message"]
    #[serde(rename = "application/x-pki-message")]
    XPkiMessage,
    #[doc = "application/x-www-form-urlencoded"]
    #[serde(rename = "application/x-www-form-urlencoded")]
    XWwwFormUrlencoded,
    #[doc = "application/x-x509-ca-cert"]
    #[serde(rename = "application/x-x509-ca-cert")]
    XX509CaCert,
    #[doc = "application/x-x509-ca-ra-cert"]
    #[serde(rename = "application/x-x509-ca-ra-cert")]
    XX509CaRaCert,
    #[doc = "application/x-x509-next-ca-cert"]
    #[serde(rename = "application/x-x509-next-ca-cert")]
    XX509NextCaCert,
    #[doc = "application/x400-bp"]
    #[serde(rename = "application/x400-bp")]
    X400Bp,
    #[doc = "application/xacml+xml"]
    #[serde(rename = "application/xacml+xml")]
    XacmlXml,
    #[doc = "application/xcap-att+xml"]
    #[serde(rename = "application/xcap-att+xml")]
    #[serde(alias = "xav")]
    XcapAttXml,
    #[doc = "application/xcap-caps+xml"]
    #[serde(rename = "application/xcap-caps+xml")]
    #[serde(alias = "xca")]
    XcapCapsXml,
    #[doc = "application/xcap-diff+xml"]
    #[serde(rename = "application/xcap-diff+xml")]
    #[serde(alias = "xdf")]
    XcapDiffXml,
    #[doc = "application/xcap-el+xml"]
    #[serde(rename = "application/xcap-el+xml")]
    #[serde(alias = "xel")]
    XcapElXml,
    #[doc = "application/xcap-error+xml"]
    #[serde(rename = "application/xcap-error+xml")]
    #[serde(alias = "xer")]
    XcapErrorXml,
    #[doc = "application/xcap-ns+xml"]
    #[serde(rename = "application/xcap-ns+xml")]
    #[serde(alias = "xns")]
    XcapNsXml,
    #[doc = "application/xcon-conference-info-diff+xml"]
    #[serde(rename = "application/xcon-conference-info-diff+xml")]
    XconConferenceInfoDiffXml,
    #[doc = "application/xcon-conference-info+xml"]
    #[serde(rename = "application/xcon-conference-info+xml")]
    XconConferenceInfoXml,
    #[doc = "application/xenc+xml"]
    #[serde(rename = "application/xenc+xml")]
    XencXml,
    #[doc = "application/xfdf"]
    #[serde(rename = "application/xfdf")]
    #[serde(alias = "xfdf")]
    Xfdf,
    #[doc = "application/xhtml+xml"]
    #[serde(rename = "application/xhtml+xml")]
    #[serde(alias = "xhtml")]
    #[serde(alias = "xhtm")]
    #[serde(alias = "xht")]
    XhtmlXml,
    #[doc = "application/xliff+xml"]
    #[serde(rename = "application/xliff+xml")]
    #[serde(alias = "xlf")]
    XliffXml,
    #[doc = "application/xml"]
    #[serde(rename = "application/xml")]
    Xml,
    #[doc = "application/xml-dtd"]
    #[serde(rename = "application/xml-dtd")]
    #[serde(alias = "dtd")]
    XmlDtd,
    #[doc = "application/xml-external-parsed-entity"]
    #[serde(rename = "application/xml-external-parsed-entity")]
    XmlExternalParsedEntity,
    #[doc = "application/xml-patch+xml"]
    #[serde(rename = "application/xml-patch+xml")]
    XmlPatchXml,
    #[doc = "application/xmpp+xml"]
    #[serde(rename = "application/xmpp+xml")]
    XmppXml,
    #[doc = "application/xop+xml"]
    #[serde(rename = "application/xop+xml")]
    #[serde(alias = "xop")]
    XopXml,
    #[doc = "application/xslt+xml"]
    #[serde(rename = "application/xslt+xml")]
    #[serde(alias = "xsl")]
    #[serde(alias = "xslt")]
    XsltXml,
    #[doc = "application/xv+xml"]
    #[serde(rename = "application/xv+xml")]
    #[serde(alias = "mxml")]
    #[serde(alias = "xhvml")]
    #[serde(alias = "xvml")]
    #[serde(alias = "xvm")]
    XvXml,
    #[doc = "application/yaml"]
    #[serde(rename = "application/yaml")]
    Yaml,
    #[doc = "application/yang"]
    #[serde(rename = "application/yang")]
    #[serde(alias = "yang")]
    Yang,
    #[doc = "application/yang-data+cbor"]
    #[serde(rename = "application/yang-data+cbor")]
    YangDataCbor,
    #[doc = "application/yang-data+json"]
    #[serde(rename = "application/yang-data+json")]
    YangDataJson,
    #[doc = "application/yang-data+xml"]
    #[serde(rename = "application/yang-data+xml")]
    YangDataXml,
    #[doc = "application/yang-patch+json"]
    #[serde(rename = "application/yang-patch+json")]
    YangPatchJson,
    #[doc = "application/yang-patch+xml"]
    #[serde(rename = "application/yang-patch+xml")]
    YangPatchXml,
    #[doc = "application/yin+xml"]
    #[serde(rename = "application/yin+xml")]
    #[serde(alias = "yin")]
    YinXml,
    #[doc = "application/zip"]
    #[serde(rename = "application/zip")]
    #[serde(alias = "zip")]
    Zip,
    #[doc = "application/zlib"]
    #[serde(rename = "application/zlib")]
    Zlib,
    #[doc = "application/zstd"]
    #[serde(rename = "application/zstd")]
    #[serde(alias = "zst")]
    Zstd,
}
impl ::std::fmt::Display for Application {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self { Application :: _1DInterleavedParityfec => write ! (f , "application/1d-interleaved-parityfec") ? , Application :: _3GpdashQoeReportXml => write ! (f , "application/3gpdash-qoe-report+xml") ? , Application :: _3GppHalJson => write ! (f , "application/3gppHal+json") ? , Application :: _3GppHalFormsJson => write ! (f , "application/3gppHalForms+json") ? , Application :: _3GppImsXml => write ! (f , "application/3gpp-ims+xml") ? , Application :: A2L => write ! (f , "application/A2L") ? , Application :: AceCbor => write ! (f , "application/ace+cbor") ? , Application :: AceJson => write ! (f , "application/ace+json") ? , Application :: Activemessage => write ! (f , "application/activemessage") ? , Application :: ActivityJson => write ! (f , "application/activity+json") ? , Application :: AifCbor => write ! (f , "application/aif+cbor") ? , Application :: AifJson => write ! (f , "application/aif+json") ? , Application :: AltoCdniJson => write ! (f , "application/alto-cdni+json") ? , Application :: AltoCdnifilterJson => write ! (f , "application/alto-cdnifilter+json") ? , Application :: AltoCostmapJson => write ! (f , "application/alto-costmap+json") ? , Application :: AltoCostmapfilterJson => write ! (f , "application/alto-costmapfilter+json") ? , Application :: AltoDirectoryJson => write ! (f , "application/alto-directory+json") ? , Application :: AltoEndpointpropJson => write ! (f , "application/alto-endpointprop+json") ? , Application :: AltoEndpointpropparamsJson => write ! (f , "application/alto-endpointpropparams+json") ? , Application :: AltoEndpointcostJson => write ! (f , "application/alto-endpointcost+json") ? , Application :: AltoEndpointcostparamsJson => write ! (f , "application/alto-endpointcostparams+json") ? , Application :: AltoErrorJson => write ! (f , "application/alto-error+json") ? , Application :: AltoNetworkmapfilterJson => write ! (f , "application/alto-networkmapfilter+json") ? , Application :: AltoNetworkmapJson => write ! (f , "application/alto-networkmap+json") ? , Application :: AltoPropmapJson => write ! (f , "application/alto-propmap+json") ? , Application :: AltoPropmapparamsJson => write ! (f , "application/alto-propmapparams+json") ? , Application :: AltoUpdatestreamcontrolJson => write ! (f , "application/alto-updatestreamcontrol+json") ? , Application :: AltoUpdatestreamparamsJson => write ! (f , "application/alto-updatestreamparams+json") ? , Application :: Aml => write ! (f , "application/AML") ? , Application :: AndrewInset => write ! (f , "application/andrew-inset") ? , Application :: Applefile => write ! (f , "application/applefile") ? , Application :: AtJwt => write ! (f , "application/at+jwt") ? , Application :: Atf => write ! (f , "application/ATF") ? , Application :: Atfx => write ! (f , "application/ATFX") ? , Application :: AtomXml => write ! (f , "application/atom+xml") ? , Application :: AtomcatXml => write ! (f , "application/atomcat+xml") ? , Application :: AtomdeletedXml => write ! (f , "application/atomdeleted+xml") ? , Application :: Atomicmail => write ! (f , "application/atomicmail") ? , Application :: AtomsvcXml => write ! (f , "application/atomsvc+xml") ? , Application :: AtscDwdXml => write ! (f , "application/atsc-dwd+xml") ? , Application :: AtscDynamicEventMessage => write ! (f , "application/atsc-dynamic-event-message") ? , Application :: AtscHeldXml => write ! (f , "application/atsc-held+xml") ? , Application :: AtscRdtJson => write ! (f , "application/atsc-rdt+json") ? , Application :: AtscRsatXml => write ! (f , "application/atsc-rsat+xml") ? , Application :: Atxml => write ! (f , "application/ATXML") ? , Application :: AuthPolicyXml => write ! (f , "application/auth-policy+xml") ? , Application :: AutomationmlAmlXml => write ! (f , "application/automationml-aml+xml") ? , Application :: AutomationmlAmlxZip => write ! (f , "application/automationml-amlx+zip") ? , Application :: BacnetXddZip => write ! (f , "application/bacnet-xdd+zip") ? , Application :: BatchSMTP => write ! (f , "application/batch-SMTP") ? , Application :: BeepXml => write ! (f , "application/beep+xml") ? , Application :: CalendarJson => write ! (f , "application/calendar+json") ? , Application :: CalendarXml => write ! (f , "application/calendar+xml") ? , Application :: CallCompletion => write ! (f , "application/call-completion") ? , Application :: Cals1840 => write ! (f , "application/CALS-1840") ? , Application :: CaptiveJson => write ! (f , "application/captive+json") ? , Application :: Cbor => write ! (f , "application/cbor") ? , Application :: CborSeq => write ! (f , "application/cbor-seq") ? , Application :: Cccex => write ! (f , "application/cccex") ? , Application :: CcmpXml => write ! (f , "application/ccmp+xml") ? , Application :: CcxmlXml => write ! (f , "application/ccxml+xml") ? , Application :: CdaXml => write ! (f , "application/cda+xml") ? , Application :: CdfxXml => write ! (f , "application/CDFX+XML") ? , Application :: CdmiCapability => write ! (f , "application/cdmi-capability") ? , Application :: CdmiContainer => write ! (f , "application/cdmi-container") ? , Application :: CdmiDomain => write ! (f , "application/cdmi-domain") ? , Application :: CdmiObject => write ! (f , "application/cdmi-object") ? , Application :: CdmiQueue => write ! (f , "application/cdmi-queue") ? , Application :: Cdni => write ! (f , "application/cdni") ? , Application :: Cea => write ! (f , "application/CEA") ? , Application :: Cea2018Xml => write ! (f , "application/cea-2018+xml") ? , Application :: CellmlXml => write ! (f , "application/cellml+xml") ? , Application :: Cfw => write ! (f , "application/cfw") ? , Application :: CityJson => write ! (f , "application/city+json") ? , Application :: Clr => write ! (f , "application/clr") ? , Application :: ClueInfoXml => write ! (f , "application/clue_info+xml") ? , Application :: ClueXml => write ! (f , "application/clue+xml") ? , Application :: Cms => write ! (f , "application/cms") ? , Application :: CnrpXml => write ! (f , "application/cnrp+xml") ? , Application :: CoapGroupJson => write ! (f , "application/coap-group+json") ? , Application :: CoapPayload => write ! (f , "application/coap-payload") ? , Application :: Commonground => write ! (f , "application/commonground") ? , Application :: ConciseProblemDetailsCbor => write ! (f , "application/concise-problem-details+cbor") ? , Application :: ConferenceInfoXml => write ! (f , "application/conference-info+xml") ? , Application :: CplXml => write ! (f , "application/cpl+xml") ? , Application :: Cose => write ! (f , "application/cose") ? , Application :: CoseKey => write ! (f , "application/cose-key") ? , Application :: CoseKeySet => write ! (f , "application/cose-key-set") ? , Application :: CoseX509 => write ! (f , "application/cose-x509") ? , Application :: Csrattrs => write ! (f , "application/csrattrs") ? , Application :: CstaXml => write ! (f , "application/csta+xml") ? , Application :: CstadataXml => write ! (f , "application/CSTAdata+xml") ? , Application :: CsvmJson => write ! (f , "application/csvm+json") ? , Application :: Cwl => write ! (f , "application/cwl") ? , Application :: CwlJson => write ! (f , "application/cwl+json") ? , Application :: Cwt => write ! (f , "application/cwt") ? , Application :: Cybercash => write ! (f , "application/cybercash") ? , Application :: DashXml => write ! (f , "application/dash+xml") ? , Application :: DashPatchXml => write ! (f , "application/dash-patch+xml") ? , Application :: Dashdelta => write ! (f , "application/dashdelta") ? , Application :: DavmountXml => write ! (f , "application/davmount+xml") ? , Application :: DcaRft => write ! (f , "application/dca-rft") ? , Application :: Dcd => write ! (f , "application/DCD") ? , Application :: DecDx => write ! (f , "application/dec-dx") ? , Application :: DialogInfoXml => write ! (f , "application/dialog-info+xml") ? , Application :: Dicom => write ! (f , "application/dicom") ? , Application :: DicomJson => write ! (f , "application/dicom+json") ? , Application :: DicomXml => write ! (f , "application/dicom+xml") ? , Application :: Dii => write ! (f , "application/DII") ? , Application :: Dit => write ! (f , "application/DIT") ? , Application :: Dns => write ! (f , "application/dns") ? , Application :: DnsJson => write ! (f , "application/dns+json") ? , Application :: DnsMessage => write ! (f , "application/dns-message") ? , Application :: DotsCbor => write ! (f , "application/dots+cbor") ? , Application :: DpopJwt => write ! (f , "application/dpop+jwt") ? , Application :: DskppXml => write ! (f , "application/dskpp+xml") ? , Application :: DsscDer => write ! (f , "application/dssc+der") ? , Application :: DsscXml => write ! (f , "application/dssc+xml") ? , Application :: Dvcs => write ! (f , "application/dvcs") ? , Application :: EdiConsent => write ! (f , "application/EDI-consent") ? , Application :: Edifact => write ! (f , "application/EDIFACT") ? , Application :: EdiX12 => write ! (f , "application/EDI-X12") ? , Application :: Efi => write ! (f , "application/efi") ? , Application :: ElmJson => write ! (f , "application/elm+json") ? , Application :: ElmXml => write ! (f , "application/elm+xml") ? , Application :: EmergencyCallDataCapXml => write ! (f , "application/EmergencyCallData.cap+xml") ? , Application :: EmergencyCallDataCommentXml => write ! (f , "application/EmergencyCallData.Comment+xml") ? , Application :: EmergencyCallDataControlXml => write ! (f , "application/EmergencyCallData.Control+xml") ? , Application :: EmergencyCallDataDeviceInfoXml => write ! (f , "application/EmergencyCallData.DeviceInfo+xml") ? , Application :: EmergencyCallDataECallMSD => write ! (f , "application/EmergencyCallData.eCall.MSD") ? , Application :: EmergencyCallDataLegacyESNJson => write ! (f , "application/EmergencyCallData.LegacyESN+json") ? , Application :: EmergencyCallDataProviderInfoXml => write ! (f , "application/EmergencyCallData.ProviderInfo+xml") ? , Application :: EmergencyCallDataServiceInfoXml => write ! (f , "application/EmergencyCallData.ServiceInfo+xml") ? , Application :: EmergencyCallDataSubscriberInfoXml => write ! (f , "application/EmergencyCallData.SubscriberInfo+xml") ? , Application :: EmergencyCallDataVEDSXml => write ! (f , "application/EmergencyCallData.VEDS+xml") ? , Application :: EmmaXml => write ! (f , "application/emma+xml") ? , Application :: EmotionmlXml => write ! (f , "application/emotionml+xml") ? , Application :: Encaprtp => write ! (f , "application/encaprtp") ? , Application :: EppXml => write ! (f , "application/epp+xml") ? , Application :: EpubZip => write ! (f , "application/epub+zip") ? , Application :: Eshop => write ! (f , "application/eshop") ? , Application :: Example => write ! (f , "application/example") ? , Application :: Exi => write ! (f , "application/exi") ? , Application :: ExpectCtReportJson => write ! (f , "application/expect-ct-report+json") ? , Application :: Express => write ! (f , "application/express") ? , Application :: Fastinfoset => write ! (f , "application/fastinfoset") ? , Application :: Fastsoap => write ! (f , "application/fastsoap") ? , Application :: Fdf => write ! (f , "application/fdf") ? , Application :: FdtXml => write ! (f , "application/fdt+xml") ? , Application :: FhirJson => write ! (f , "application/fhir+json") ? , Application :: FhirXml => write ! (f , "application/fhir+xml") ? , Application :: Fits => write ! (f , "application/fits") ? , Application :: Flexfec => write ! (f , "application/flexfec") ? , Application :: FontTdpfr => write ! (f , "application/font-tdpfr") ? , Application :: FrameworkAttributesXml => write ! (f , "application/framework-attributes+xml") ? , Application :: GeoJson => write ! (f , "application/geo+json") ? , Application :: GeoJsonSeq => write ! (f , "application/geo+json-seq") ? , Application :: GeopackageSqlite3 => write ! (f , "application/geopackage+sqlite3") ? , Application :: GeoxacmlXml => write ! (f , "application/geoxacml+xml") ? , Application :: GltfBuffer => write ! (f , "application/gltf-buffer") ? , Application :: GmlXml => write ! (f , "application/gml+xml") ? , Application :: Gzip => write ! (f , "application/gzip") ? , Application :: H224 => write ! (f , "application/H224") ? , Application :: HeldXml => write ! (f , "application/held+xml") ? , Application :: Hl7V2Xml => write ! (f , "application/hl7v2+xml") ? , Application :: Http => write ! (f , "application/http") ? , Application :: Hyperstudio => write ! (f , "application/hyperstudio") ? , Application :: IbeKeyRequestXml => write ! (f , "application/ibe-key-request+xml") ? , Application :: IbePkgReplyXml => write ! (f , "application/ibe-pkg-reply+xml") ? , Application :: IbePpData => write ! (f , "application/ibe-pp-data") ? , Application :: Iges => write ! (f , "application/iges") ? , Application :: ImIscomposingXml => write ! (f , "application/im-iscomposing+xml") ? , Application :: Index => write ! (f , "application/index") ? , Application :: IndexCmd => write ! (f , "application/index.cmd") ? , Application :: IndexObj => write ! (f , "application/index.obj") ? , Application :: IndexResponse => write ! (f , "application/index.response") ? , Application :: IndexVnd => write ! (f , "application/index.vnd") ? , Application :: InkmlXml => write ! (f , "application/inkml+xml") ? , Application :: Iotp => write ! (f , "application/IOTP") ? , Application :: Ipfix => write ! (f , "application/ipfix") ? , Application :: Ipp => write ! (f , "application/ipp") ? , Application :: Isup => write ! (f , "application/ISUP") ? , Application :: ItsXml => write ! (f , "application/its+xml") ? , Application :: JavaArchive => write ! (f , "application/java-archive") ? , Application :: Jf2FeedJson => write ! (f , "application/jf2feed+json") ? , Application :: Jose => write ! (f , "application/jose") ? , Application :: JoseJson => write ! (f , "application/jose+json") ? , Application :: JrdJson => write ! (f , "application/jrd+json") ? , Application :: JscalendarJson => write ! (f , "application/jscalendar+json") ? , Application :: Json => write ! (f , "application/json") ? , Application :: JsonPatchJson => write ! (f , "application/json-patch+json") ? , Application :: JsonSeq => write ! (f , "application/json-seq") ? , Application :: JwkJson => write ! (f , "application/jwk+json") ? , Application :: JwkSetJson => write ! (f , "application/jwk-set+json") ? , Application :: Jwt => write ! (f , "application/jwt") ? , Application :: KpmlRequestXml => write ! (f , "application/kpml-request+xml") ? , Application :: KpmlResponseXml => write ! (f , "application/kpml-response+xml") ? , Application :: LdJson => write ! (f , "application/ld+json") ? , Application :: LgrXml => write ! (f , "application/lgr+xml") ? , Application :: LinkFormat => write ! (f , "application/link-format") ? , Application :: Linkset => write ! (f , "application/linkset") ? , Application :: LinksetJson => write ! (f , "application/linkset+json") ? , Application :: LoadControlXml => write ! (f , "application/load-control+xml") ? , Application :: LogoutJwt => write ! (f , "application/logout+jwt") ? , Application :: LostXml => write ! (f , "application/lost+xml") ? , Application :: LostsyncXml => write ! (f , "application/lostsync+xml") ? , Application :: LpfZip => write ! (f , "application/lpf+zip") ? , Application :: Lxf => write ! (f , "application/LXF") ? , Application :: MacBinhex40 => write ! (f , "application/mac-binhex40") ? , Application :: Macwriteii => write ! (f , "application/macwriteii") ? , Application :: MadsXml => write ! (f , "application/mads+xml") ? , Application :: ManifestJson => write ! (f , "application/manifest+json") ? , Application :: Marc => write ! (f , "application/marc") ? , Application :: MarcxmlXml => write ! (f , "application/marcxml+xml") ? , Application :: Mathematica => write ! (f , "application/mathematica") ? , Application :: MathmlXml => write ! (f , "application/mathml+xml") ? , Application :: MathmlContentXml => write ! (f , "application/mathml-content+xml") ? , Application :: MathmlPresentationXml => write ! (f , "application/mathml-presentation+xml") ? , Application :: MbmsAssociatedProcedureDescriptionXml => write ! (f , "application/mbms-associated-procedure-description+xml") ? , Application :: MbmsDeregisterXml => write ! (f , "application/mbms-deregister+xml") ? , Application :: MbmsEnvelopeXml => write ! (f , "application/mbms-envelope+xml") ? , Application :: MbmsMskResponseXml => write ! (f , "application/mbms-msk-response+xml") ? , Application :: MbmsMskXml => write ! (f , "application/mbms-msk+xml") ? , Application :: MbmsProtectionDescriptionXml => write ! (f , "application/mbms-protection-description+xml") ? , Application :: MbmsReceptionReportXml => write ! (f , "application/mbms-reception-report+xml") ? , Application :: MbmsRegisterResponseXml => write ! (f , "application/mbms-register-response+xml") ? , Application :: MbmsRegisterXml => write ! (f , "application/mbms-register+xml") ? , Application :: MbmsScheduleXml => write ! (f , "application/mbms-schedule+xml") ? , Application :: MbmsUserServiceDescriptionXml => write ! (f , "application/mbms-user-service-description+xml") ? , Application :: Mbox => write ! (f , "application/mbox") ? , Application :: MediaControlXml => write ! (f , "application/media_control+xml") ? , Application :: MediaPolicyDatasetXml => write ! (f , "application/media-policy-dataset+xml") ? , Application :: MediaservercontrolXml => write ! (f , "application/mediaservercontrol+xml") ? , Application :: MergePatchJson => write ! (f , "application/merge-patch+json") ? , Application :: Metalink4Xml => write ! (f , "application/metalink4+xml") ? , Application :: MetsXml => write ! (f , "application/mets+xml") ? , Application :: Mf4 => write ! (f , "application/MF4") ? , Application :: Mikey => write ! (f , "application/mikey") ? , Application :: Mipc => write ! (f , "application/mipc") ? , Application :: MissingBlocksCborSeq => write ! (f , "application/missing-blocks+cbor-seq") ? , Application :: MmtAeiXml => write ! (f , "application/mmt-aei+xml") ? , Application :: MmtUsdXml => write ! (f , "application/mmt-usd+xml") ? , Application :: ModsXml => write ! (f , "application/mods+xml") ? , Application :: MossKeys => write ! (f , "application/moss-keys") ? , Application :: MossSignature => write ! (f , "application/moss-signature") ? , Application :: MosskeyData => write ! (f , "application/mosskey-data") ? , Application :: MosskeyRequest => write ! (f , "application/mosskey-request") ? , Application :: Mp21 => write ! (f , "application/mp21") ? , Application :: Mp4 => write ! (f , "application/mp4") ? , Application :: Mpeg4Generic => write ! (f , "application/mpeg4-generic") ? , Application :: Mpeg4Iod => write ! (f , "application/mpeg4-iod") ? , Application :: Mpeg4IodXmt => write ! (f , "application/mpeg4-iod-xmt") ? , Application :: MrbConsumerXml => write ! (f , "application/mrb-consumer+xml") ? , Application :: MrbPublishXml => write ! (f , "application/mrb-publish+xml") ? , Application :: MscIvrXml => write ! (f , "application/msc-ivr+xml") ? , Application :: MscMixerXml => write ! (f , "application/msc-mixer+xml") ? , Application :: Msword => write ! (f , "application/msword") ? , Application :: MudJson => write ! (f , "application/mud+json") ? , Application :: MultipartCore => write ! (f , "application/multipart-core") ? , Application :: Mxf => write ! (f , "application/mxf") ? , Application :: NQuads => write ! (f , "application/n-quads") ? , Application :: NTriples => write ! (f , "application/n-triples") ? , Application :: Nasdata => write ! (f , "application/nasdata") ? , Application :: NewsCheckgroups => write ! (f , "application/news-checkgroups") ? , Application :: NewsGroupinfo => write ! (f , "application/news-groupinfo") ? , Application :: NewsTransmission => write ! (f , "application/news-transmission") ? , Application :: NlsmlXml => write ! (f , "application/nlsml+xml") ? , Application :: Node => write ! (f , "application/node") ? , Application :: Nss => write ! (f , "application/nss") ? , Application :: OauthAuthzReqJwt => write ! (f , "application/oauth-authz-req+jwt") ? , Application :: ObliviousDnsMessage => write ! (f , "application/oblivious-dns-message") ? , Application :: OcspRequest => write ! (f , "application/ocsp-request") ? , Application :: OcspResponse => write ! (f , "application/ocsp-response") ? , Application :: OctetStream => write ! (f , "application/octet-stream") ? , Application :: Oda => write ! (f , "application/ODA") ? , Application :: OdmXml => write ! (f , "application/odm+xml") ? , Application :: Odx => write ! (f , "application/ODX") ? , Application :: OebpsPackageXml => write ! (f , "application/oebps-package+xml") ? , Application :: Ogg => write ! (f , "application/ogg") ? , Application :: OhttpKeys => write ! (f , "application/ohttp-keys") ? , Application :: OpcNodesetXml => write ! (f , "application/opc-nodeset+xml") ? , Application :: Oscore => write ! (f , "application/oscore") ? , Application :: Oxps => write ! (f , "application/oxps") ? , Application :: P21 => write ! (f , "application/p21") ? , Application :: P21Zip => write ! (f , "application/p21+zip") ? , Application :: P2POverlayXml => write ! (f , "application/p2p-overlay+xml") ? , Application :: Parityfec => write ! (f , "application/parityfec") ? , Application :: Passport => write ! (f , "application/passport") ? , Application :: PatchOpsErrorXml => write ! (f , "application/patch-ops-error+xml") ? , Application :: Pdf => write ! (f , "application/pdf") ? , Application :: Pdx => write ! (f , "application/PDX") ? , Application :: PemCertificateChain => write ! (f , "application/pem-certificate-chain") ? , Application :: PgpEncrypted => write ! (f , "application/pgp-encrypted") ? , Application :: PgpKeys => write ! (f , "application/pgp-keys") ? , Application :: PgpSignature => write ! (f , "application/pgp-signature") ? , Application :: PidfDiffXml => write ! (f , "application/pidf-diff+xml") ? , Application :: PidfXml => write ! (f , "application/pidf+xml") ? , Application :: Pkcs10 => write ! (f , "application/pkcs10") ? , Application :: Pkcs7Mime => write ! (f , "application/pkcs7-mime") ? , Application :: Pkcs7Signature => write ! (f , "application/pkcs7-signature") ? , Application :: Pkcs8 => write ! (f , "application/pkcs8") ? , Application :: Pkcs8Encrypted => write ! (f , "application/pkcs8-encrypted") ? , Application :: Pkcs12 => write ! (f , "application/pkcs12") ? , Application :: PkixAttrCert => write ! (f , "application/pkix-attr-cert") ? , Application :: PkixCert => write ! (f , "application/pkix-cert") ? , Application :: PkixCrl => write ! (f , "application/pkix-crl") ? , Application :: PkixPkipath => write ! (f , "application/pkix-pkipath") ? , Application :: Pkixcmp => write ! (f , "application/pkixcmp") ? , Application :: PlsXml => write ! (f , "application/pls+xml") ? , Application :: PocSettingsXml => write ! (f , "application/poc-settings+xml") ? , Application :: Postscript => write ! (f , "application/postscript") ? , Application :: PpspTrackerJson => write ! (f , "application/ppsp-tracker+json") ? , Application :: ProblemJson => write ! (f , "application/problem+json") ? , Application :: ProblemXml => write ! (f , "application/problem+xml") ? , Application :: ProvenanceXml => write ! (f , "application/provenance+xml") ? , Application :: PrsAlvestrandTitraxSheet => write ! (f , "application/prs.alvestrand.titrax-sheet") ? , Application :: PrsCww => write ! (f , "application/prs.cww") ? , Application :: PrsCyn => write ! (f , "application/prs.cyn") ? , Application :: PrsHpubZip => write ! (f , "application/prs.hpub+zip") ? , Application :: PrsImpliedDocumentXml => write ! (f , "application/prs.implied-document+xml") ? , Application :: PrsImpliedExecutable => write ! (f , "application/prs.implied-executable") ? , Application :: PrsImpliedStructure => write ! (f , "application/prs.implied-structure") ? , Application :: PrsNprend => write ! (f , "application/prs.nprend") ? , Application :: PrsPlucker => write ! (f , "application/prs.plucker") ? , Application :: PrsRdfXmlCrypt => write ! (f , "application/prs.rdf-xml-crypt") ? , Application :: PrsXsfXml => write ! (f , "application/prs.xsf+xml") ? , Application :: PskcXml => write ! (f , "application/pskc+xml") ? , Application :: PvdJson => write ! (f , "application/pvd+json") ? , Application :: RdfXml => write ! (f , "application/rdf+xml") ? , Application :: RouteApdXml => write ! (f , "application/route-apd+xml") ? , Application :: RouteSTsidXml => write ! (f , "application/route-s-tsid+xml") ? , Application :: RouteUsdXml => write ! (f , "application/route-usd+xml") ? , Application :: Qsig => write ! (f , "application/QSIG") ? , Application :: Raptorfec => write ! (f , "application/raptorfec") ? , Application :: RdapJson => write ! (f , "application/rdap+json") ? , Application :: ReginfoXml => write ! (f , "application/reginfo+xml") ? , Application :: RelaxNgCompactSyntax => write ! (f , "application/relax-ng-compact-syntax") ? , Application :: ReputonJson => write ! (f , "application/reputon+json") ? , Application :: ResourceListsDiffXml => write ! (f , "application/resource-lists-diff+xml") ? , Application :: ResourceListsXml => write ! (f , "application/resource-lists+xml") ? , Application :: RfcXml => write ! (f , "application/rfc+xml") ? , Application :: Riscos => write ! (f , "application/riscos") ? , Application :: RlmiXml => write ! (f , "application/rlmi+xml") ? , Application :: RlsServicesXml => write ! (f , "application/rls-services+xml") ? , Application :: RpkiChecklist => write ! (f , "application/rpki-checklist") ? , Application :: RpkiGhostbusters => write ! (f , "application/rpki-ghostbusters") ? , Application :: RpkiManifest => write ! (f , "application/rpki-manifest") ? , Application :: RpkiPublication => write ! (f , "application/rpki-publication") ? , Application :: RpkiRoa => write ! (f , "application/rpki-roa") ? , Application :: RpkiUpdown => write ! (f , "application/rpki-updown") ? , Application :: Rtf => write ! (f , "application/rtf") ? , Application :: Rtploopback => write ! (f , "application/rtploopback") ? , Application :: Rtx => write ! (f , "application/rtx") ? , Application :: SamlassertionXml => write ! (f , "application/samlassertion+xml") ? , Application :: SamlmetadataXml => write ! (f , "application/samlmetadata+xml") ? , Application :: SarifExternalPropertiesJson => write ! (f , "application/sarif-external-properties+json") ? , Application :: SarifJson => write ! (f , "application/sarif+json") ? , Application :: Sbe => write ! (f , "application/sbe") ? , Application :: SbmlXml => write ! (f , "application/sbml+xml") ? , Application :: ScaipXml => write ! (f , "application/scaip+xml") ? , Application :: ScimJson => write ! (f , "application/scim+json") ? , Application :: ScvpCvRequest => write ! (f , "application/scvp-cv-request") ? , Application :: ScvpCvResponse => write ! (f , "application/scvp-cv-response") ? , Application :: ScvpVpRequest => write ! (f , "application/scvp-vp-request") ? , Application :: ScvpVpResponse => write ! (f , "application/scvp-vp-response") ? , Application :: Sdp => write ! (f , "application/sdp") ? , Application :: SeceventJwt => write ! (f , "application/secevent+jwt") ? , Application :: SenmlEtchCbor => write ! (f , "application/senml-etch+cbor") ? , Application :: SenmlEtchJson => write ! (f , "application/senml-etch+json") ? , Application :: SenmlExi => write ! (f , "application/senml-exi") ? , Application :: SenmlCbor => write ! (f , "application/senml+cbor") ? , Application :: SenmlJson => write ! (f , "application/senml+json") ? , Application :: SenmlXml => write ! (f , "application/senml+xml") ? , Application :: SensmlExi => write ! (f , "application/sensml-exi") ? , Application :: SensmlCbor => write ! (f , "application/sensml+cbor") ? , Application :: SensmlJson => write ! (f , "application/sensml+json") ? , Application :: SensmlXml => write ! (f , "application/sensml+xml") ? , Application :: SepExi => write ! (f , "application/sep-exi") ? , Application :: SepXml => write ! (f , "application/sep+xml") ? , Application :: SessionInfo => write ! (f , "application/session-info") ? , Application :: SetPayment => write ! (f , "application/set-payment") ? , Application :: SetPaymentInitiation => write ! (f , "application/set-payment-initiation") ? , Application :: SetRegistration => write ! (f , "application/set-registration") ? , Application :: SetRegistrationInitiation => write ! (f , "application/set-registration-initiation") ? , Application :: Sgml => write ! (f , "application/SGML") ? , Application :: SgmlOpenCatalog => write ! (f , "application/sgml-open-catalog") ? , Application :: ShfXml => write ! (f , "application/shf+xml") ? , Application :: Sieve => write ! (f , "application/sieve") ? , Application :: SimpleFilterXml => write ! (f , "application/simple-filter+xml") ? , Application :: SimpleMessageSummary => write ! (f , "application/simple-message-summary") ? , Application :: SimpleSymbolContainer => write ! (f , "application/simpleSymbolContainer") ? , Application :: Sipc => write ! (f , "application/sipc") ? , Application :: Slate => write ! (f , "application/slate") ? , Application :: SmilXml => write ! (f , "application/smil+xml") ? , Application :: Smpte336M => write ! (f , "application/smpte336m") ? , Application :: SoapFastinfoset => write ! (f , "application/soap+fastinfoset") ? , Application :: SoapXml => write ! (f , "application/soap+xml") ? , Application :: SparqlQuery => write ! (f , "application/sparql-query") ? , Application :: SpdxJson => write ! (f , "application/spdx+json") ? , Application :: SparqlResultsXml => write ! (f , "application/sparql-results+xml") ? , Application :: SpiritsEventXml => write ! (f , "application/spirits-event+xml") ? , Application :: Sql => write ! (f , "application/sql") ? , Application :: Srgs => write ! (f , "application/srgs") ? , Application :: SrgsXml => write ! (f , "application/srgs+xml") ? , Application :: SruXml => write ! (f , "application/sru+xml") ? , Application :: SsmlXml => write ! (f , "application/ssml+xml") ? , Application :: StixJson => write ! (f , "application/stix+json") ? , Application :: SwidCbor => write ! (f , "application/swid+cbor") ? , Application :: SwidXml => write ! (f , "application/swid+xml") ? , Application :: TampApexUpdate => write ! (f , "application/tamp-apex-update") ? , Application :: TampApexUpdateConfirm => write ! (f , "application/tamp-apex-update-confirm") ? , Application :: TampCommunityUpdate => write ! (f , "application/tamp-community-update") ? , Application :: TampCommunityUpdateConfirm => write ! (f , "application/tamp-community-update-confirm") ? , Application :: TampError => write ! (f , "application/tamp-error") ? , Application :: TampSequenceAdjust => write ! (f , "application/tamp-sequence-adjust") ? , Application :: TampSequenceAdjustConfirm => write ! (f , "application/tamp-sequence-adjust-confirm") ? , Application :: TampStatusQuery => write ! (f , "application/tamp-status-query") ? , Application :: TampStatusResponse => write ! (f , "application/tamp-status-response") ? , Application :: TampUpdate => write ! (f , "application/tamp-update") ? , Application :: TampUpdateConfirm => write ! (f , "application/tamp-update-confirm") ? , Application :: TaxiiJson => write ! (f , "application/taxii+json") ? , Application :: TdJson => write ! (f , "application/td+json") ? , Application :: TeiXml => write ! (f , "application/tei+xml") ? , Application :: TetraIsi => write ! (f , "application/TETRA_ISI") ? , Application :: ThraudXml => write ! (f , "application/thraud+xml") ? , Application :: TimestampQuery => write ! (f , "application/timestamp-query") ? , Application :: TimestampReply => write ! (f , "application/timestamp-reply") ? , Application :: TimestampedData => write ! (f , "application/timestamped-data") ? , Application :: TlsrptGzip => write ! (f , "application/tlsrpt+gzip") ? , Application :: TlsrptJson => write ! (f , "application/tlsrpt+json") ? , Application :: TmJson => write ! (f , "application/tm+json") ? , Application :: Tnauthlist => write ! (f , "application/tnauthlist") ? , Application :: TokenIntrospectionJwt => write ! (f , "application/token-introspection+jwt") ? , Application :: TrickleIceSdpfrag => write ! (f , "application/trickle-ice-sdpfrag") ? , Application :: Trig => write ! (f , "application/trig") ? , Application :: TtmlXml => write ! (f , "application/ttml+xml") ? , Application :: TveTrigger => write ! (f , "application/tve-trigger") ? , Application :: Tzif => write ! (f , "application/tzif") ? , Application :: TzifLeap => write ! (f , "application/tzif-leap") ? , Application :: Ulpfec => write ! (f , "application/ulpfec") ? , Application :: UrcGrpsheetXml => write ! (f , "application/urc-grpsheet+xml") ? , Application :: UrcRessheetXml => write ! (f , "application/urc-ressheet+xml") ? , Application :: UrcTargetdescXml => write ! (f , "application/urc-targetdesc+xml") ? , Application :: UrcUisocketdescXml => write ! (f , "application/urc-uisocketdesc+xml") ? , Application :: VcardJson => write ! (f , "application/vcard+json") ? , Application :: VcardXml => write ! (f , "application/vcard+xml") ? , Application :: Vemmi => write ! (f , "application/vemmi") ? , Application :: Vnd1000MindsDecisionModelXml => write ! (f , "application/vnd.1000minds.decision-model+xml") ? , Application :: Vnd1Ob => write ! (f , "application/vnd.1ob") ? , Application :: Vnd3Gpp5Gnas => write ! (f , "application/vnd.3gpp.5gnas") ? , Application :: Vnd3GppAccessTransferEventsXml => write ! (f , "application/vnd.3gpp.access-transfer-events+xml") ? , Application :: Vnd3GppBsfXml => write ! (f , "application/vnd.3gpp.bsf+xml") ? , Application :: Vnd3GppCrsXml => write ! (f , "application/vnd.3gpp.crs+xml") ? , Application :: Vnd3GppCurrentLocationDiscoveryXml => write ! (f , "application/vnd.3gpp.current-location-discovery+xml") ? , Application :: Vnd3GppGMOPXml => write ! (f , "application/vnd.3gpp.GMOP+xml") ? , Application :: Vnd3GppGtpc => write ! (f , "application/vnd.3gpp.gtpc") ? , Application :: Vnd3GppInterworkingData => write ! (f , "application/vnd.3gpp.interworking-data") ? , Application :: Vnd3GppLpp => write ! (f , "application/vnd.3gpp.lpp") ? , Application :: Vnd3GppMcSignallingEar => write ! (f , "application/vnd.3gpp.mc-signalling-ear") ? , Application :: Vnd3GppMcdataAffiliationCommandXml => write ! (f , "application/vnd.3gpp.mcdata-affiliation-command+xml") ? , Application :: Vnd3GppMcdataInfoXml => write ! (f , "application/vnd.3gpp.mcdata-info+xml") ? , Application :: Vnd3GppMcdataMsgstoreCtrlRequestXml => write ! (f , "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml") ? , Application :: Vnd3GppMcdataPayload => write ! (f , "application/vnd.3gpp.mcdata-payload") ? , Application :: Vnd3GppMcdataRegroupXml => write ! (f , "application/vnd.3gpp.mcdata-regroup+xml") ? , Application :: Vnd3GppMcdataServiceConfigXml => write ! (f , "application/vnd.3gpp.mcdata-service-config+xml") ? , Application :: Vnd3GppMcdataSignalling => write ! (f , "application/vnd.3gpp.mcdata-signalling") ? , Application :: Vnd3GppMcdataUeConfigXml => write ! (f , "application/vnd.3gpp.mcdata-ue-config+xml") ? , Application :: Vnd3GppMcdataUserProfileXml => write ! (f , "application/vnd.3gpp.mcdata-user-profile+xml") ? , Application :: Vnd3GppMcpttAffiliationCommandXml => write ! (f , "application/vnd.3gpp.mcptt-affiliation-command+xml") ? , Application :: Vnd3GppMcpttFloorRequestXml => write ! (f , "application/vnd.3gpp.mcptt-floor-request+xml") ? , Application :: Vnd3GppMcpttInfoXml => write ! (f , "application/vnd.3gpp.mcptt-info+xml") ? , Application :: Vnd3GppMcpttLocationInfoXml => write ! (f , "application/vnd.3gpp.mcptt-location-info+xml") ? , Application :: Vnd3GppMcpttMbmsUsageInfoXml => write ! (f , "application/vnd.3gpp.mcptt-mbms-usage-info+xml") ? , Application :: Vnd3GppMcpttRegroupXml => write ! (f , "application/vnd.3gpp.mcptt-regroup+xml") ? , Application :: Vnd3GppMcpttServiceConfigXml => write ! (f , "application/vnd.3gpp.mcptt-service-config+xml") ? , Application :: Vnd3GppMcpttSignedXml => write ! (f , "application/vnd.3gpp.mcptt-signed+xml") ? , Application :: Vnd3GppMcpttUeConfigXml => write ! (f , "application/vnd.3gpp.mcptt-ue-config+xml") ? , Application :: Vnd3GppMcpttUeInitConfigXml => write ! (f , "application/vnd.3gpp.mcptt-ue-init-config+xml") ? , Application :: Vnd3GppMcpttUserProfileXml => write ! (f , "application/vnd.3gpp.mcptt-user-profile+xml") ? , Application :: Vnd3GppMcvideoAffiliationCommandXml => write ! (f , "application/vnd.3gpp.mcvideo-affiliation-command+xml") ? , Application :: Vnd3GppMcvideoInfoXml => write ! (f , "application/vnd.3gpp.mcvideo-info+xml") ? , Application :: Vnd3GppMcvideoLocationInfoXml => write ! (f , "application/vnd.3gpp.mcvideo-location-info+xml") ? , Application :: Vnd3GppMcvideoMbmsUsageInfoXml => write ! (f , "application/vnd.3gpp.mcvideo-mbms-usage-info+xml") ? , Application :: Vnd3GppMcvideoRegroupXml => write ! (f , "application/vnd.3gpp.mcvideo-regroup+xml") ? , Application :: Vnd3GppMcvideoServiceConfigXml => write ! (f , "application/vnd.3gpp.mcvideo-service-config+xml") ? , Application :: Vnd3GppMcvideoTransmissionRequestXml => write ! (f , "application/vnd.3gpp.mcvideo-transmission-request+xml") ? , Application :: Vnd3GppMcvideoUeConfigXml => write ! (f , "application/vnd.3gpp.mcvideo-ue-config+xml") ? , Application :: Vnd3GppMcvideoUserProfileXml => write ! (f , "application/vnd.3gpp.mcvideo-user-profile+xml") ? , Application :: Vnd3GppMidCallXml => write ! (f , "application/vnd.3gpp.mid-call+xml") ? , Application :: Vnd3GppNgap => write ! (f , "application/vnd.3gpp.ngap") ? , Application :: Vnd3GppPfcp => write ! (f , "application/vnd.3gpp.pfcp") ? , Application :: Vnd3GppPicBwLarge => write ! (f , "application/vnd.3gpp.pic-bw-large") ? , Application :: Vnd3GppPicBwSmall => write ! (f , "application/vnd.3gpp.pic-bw-small") ? , Application :: Vnd3GppPicBwVar => write ! (f , "application/vnd.3gpp.pic-bw-var") ? , Application :: Vnd3GppProsePc3AXml => write ! (f , "application/vnd.3gpp-prose-pc3a+xml") ? , Application :: Vnd3GppProsePc3AchXml => write ! (f , "application/vnd.3gpp-prose-pc3ach+xml") ? , Application :: Vnd3GppProsePc3ChXml => write ! (f , "application/vnd.3gpp-prose-pc3ch+xml") ? , Application :: Vnd3GppProsePc8Xml => write ! (f , "application/vnd.3gpp-prose-pc8+xml") ? , Application :: Vnd3GppProseXml => write ! (f , "application/vnd.3gpp-prose+xml") ? , Application :: Vnd3GppS1Ap => write ! (f , "application/vnd.3gpp.s1ap") ? , Application :: Vnd3GppSealGroupDocXml => write ! (f , "application/vnd.3gpp.seal-group-doc+xml") ? , Application :: Vnd3GppSealInfoXml => write ! (f , "application/vnd.3gpp.seal-info+xml") ? , Application :: Vnd3GppSealLocationInfoXml => write ! (f , "application/vnd.3gpp.seal-location-info+xml") ? , Application :: Vnd3GppSealMbmsUsageInfoXml => write ! (f , "application/vnd.3gpp.seal-mbms-usage-info+xml") ? , Application :: Vnd3GppSealNetworkQoSManagementInfoXml => write ! (f , "application/vnd.3gpp.seal-network-QoS-management-info+xml") ? , Application :: Vnd3GppSealUeConfigInfoXml => write ! (f , "application/vnd.3gpp.seal-ue-config-info+xml") ? , Application :: Vnd3GppSealUnicastInfoXml => write ! (f , "application/vnd.3gpp.seal-unicast-info+xml") ? , Application :: Vnd3GppSealUserProfileInfoXml => write ! (f , "application/vnd.3gpp.seal-user-profile-info+xml") ? , Application :: Vnd3GppSms => write ! (f , "application/vnd.3gpp.sms") ? , Application :: Vnd3GppSmsXml => write ! (f , "application/vnd.3gpp.sms+xml") ? , Application :: Vnd3GppSrvccExtXml => write ! (f , "application/vnd.3gpp.srvcc-ext+xml") ? , Application :: Vnd3GppSRVCCInfoXml => write ! (f , "application/vnd.3gpp.SRVCC-info+xml") ? , Application :: Vnd3GppStateAndEventInfoXml => write ! (f , "application/vnd.3gpp.state-and-event-info+xml") ? , Application :: Vnd3GppUssdXml => write ! (f , "application/vnd.3gpp.ussd+xml") ? , Application :: Vnd3GppVaeInfoXml => write ! (f , "application/vnd.3gpp.vae-info+xml") ? , Application :: Vnd3GppV2XLocalServiceInformation => write ! (f , "application/vnd.3gpp-v2x-local-service-information") ? , Application :: Vnd3Gpp2BcmcsinfoXml => write ! (f , "application/vnd.3gpp2.bcmcsinfo+xml") ? , Application :: Vnd3Gpp2Sms => write ! (f , "application/vnd.3gpp2.sms") ? , Application :: Vnd3Gpp2Tcap => write ! (f , "application/vnd.3gpp2.tcap") ? , Application :: Vnd3GppV2X => write ! (f , "application/vnd.3gpp.v2x") ? , Application :: Vnd3LightssoftwareImagescal => write ! (f , "application/vnd.3lightssoftware.imagescal") ? , Application :: Vnd3MPostItNotes => write ! (f , "application/vnd.3M.Post-it-Notes") ? , Application :: VndAccpacSimplyAso => write ! (f , "application/vnd.accpac.simply.aso") ? , Application :: VndAccpacSimplyImp => write ! (f , "application/vnd.accpac.simply.imp") ? , Application :: VndAcmAddressxferJson => write ! (f , "application/vnd.acm.addressxfer+json") ? , Application :: VndAcucobol => write ! (f , "application/vnd.acucobol") ? , Application :: VndAcucorp => write ! (f , "application/vnd.acucorp") ? , Application :: VndAdobeFlashMovie => write ! (f , "application/vnd.adobe.flash.movie") ? , Application :: VndAdobeFormscentralFcdt => write ! (f , "application/vnd.adobe.formscentral.fcdt") ? , Application :: VndAdobeFxp => write ! (f , "application/vnd.adobe.fxp") ? , Application :: VndAdobePartialUpload => write ! (f , "application/vnd.adobe.partial-upload") ? , Application :: VndAdobeXdpXml => write ! (f , "application/vnd.adobe.xdp+xml") ? , Application :: VndAetherImp => write ! (f , "application/vnd.aether.imp") ? , Application :: VndAfpcAfplinedata => write ! (f , "application/vnd.afpc.afplinedata") ? , Application :: VndAfpcAfplinedataPagedef => write ! (f , "application/vnd.afpc.afplinedata-pagedef") ? , Application :: VndAfpcCmocaCmresource => write ! (f , "application/vnd.afpc.cmoca-cmresource") ? , Application :: VndAfpcFocaCharset => write ! (f , "application/vnd.afpc.foca-charset") ? , Application :: VndAfpcFocaCodedfont => write ! (f , "application/vnd.afpc.foca-codedfont") ? , Application :: VndAfpcFocaCodepage => write ! (f , "application/vnd.afpc.foca-codepage") ? , Application :: VndAfpcModca => write ! (f , "application/vnd.afpc.modca") ? , Application :: VndAfpcModcaCmtable => write ! (f , "application/vnd.afpc.modca-cmtable") ? , Application :: VndAfpcModcaFormdef => write ! (f , "application/vnd.afpc.modca-formdef") ? , Application :: VndAfpcModcaMediummap => write ! (f , "application/vnd.afpc.modca-mediummap") ? , Application :: VndAfpcModcaObjectcontainer => write ! (f , "application/vnd.afpc.modca-objectcontainer") ? , Application :: VndAfpcModcaOverlay => write ! (f , "application/vnd.afpc.modca-overlay") ? , Application :: VndAfpcModcaPagesegment => write ! (f , "application/vnd.afpc.modca-pagesegment") ? , Application :: VndAge => write ! (f , "application/vnd.age") ? , Application :: VndAhBarcode => write ! (f , "application/vnd.ah-barcode") ? , Application :: VndAheadSpace => write ! (f , "application/vnd.ahead.space") ? , Application :: VndAirzipFilesecureAzf => write ! (f , "application/vnd.airzip.filesecure.azf") ? , Application :: VndAirzipFilesecureAzs => write ! (f , "application/vnd.airzip.filesecure.azs") ? , Application :: VndAmadeusJson => write ! (f , "application/vnd.amadeus+json") ? , Application :: VndAmazonMobi8Ebook => write ! (f , "application/vnd.amazon.mobi8-ebook") ? , Application :: VndAmericandynamicsAcc => write ! (f , "application/vnd.americandynamics.acc") ? , Application :: VndAmigaAmi => write ! (f , "application/vnd.amiga.ami") ? , Application :: VndAmundsenMazeXml => write ! (f , "application/vnd.amundsen.maze+xml") ? , Application :: VndAndroidOta => write ! (f , "application/vnd.android.ota") ? , Application :: VndAnki => write ! (f , "application/vnd.anki") ? , Application :: VndAnserWebCertificateIssueInitiation => write ! (f , "application/vnd.anser-web-certificate-issue-initiation") ? , Application :: VndAntixGameComponent => write ! (f , "application/vnd.antix.game-component") ? , Application :: VndApacheArrowFile => write ! (f , "application/vnd.apache.arrow.file") ? , Application :: VndApacheArrowStream => write ! (f , "application/vnd.apache.arrow.stream") ? , Application :: VndApacheThriftBinary => write ! (f , "application/vnd.apache.thrift.binary") ? , Application :: VndApacheThriftCompact => write ! (f , "application/vnd.apache.thrift.compact") ? , Application :: VndApacheThriftJson => write ! (f , "application/vnd.apache.thrift.json") ? , Application :: VndApexlang => write ! (f , "application/vnd.apexlang") ? , Application :: VndApiJson => write ! (f , "application/vnd.api+json") ? , Application :: VndAplextorWarrpJson => write ! (f , "application/vnd.aplextor.warrp+json") ? , Application :: VndApothekendeReservationJson => write ! (f , "application/vnd.apothekende.reservation+json") ? , Application :: VndAppleInstallerXml => write ! (f , "application/vnd.apple.installer+xml") ? , Application :: VndAppleKeynote => write ! (f , "application/vnd.apple.keynote") ? , Application :: VndAppleMpegurl => write ! (f , "application/vnd.apple.mpegurl") ? , Application :: VndAppleNumbers => write ! (f , "application/vnd.apple.numbers") ? , Application :: VndApplePages => write ! (f , "application/vnd.apple.pages") ? , Application :: VndAristanetworksSwi => write ! (f , "application/vnd.aristanetworks.swi") ? , Application :: VndArtisanJson => write ! (f , "application/vnd.artisan+json") ? , Application :: VndArtsquare => write ! (f , "application/vnd.artsquare") ? , Application :: VndAstraeaSoftwareIota => write ! (f , "application/vnd.astraea-software.iota") ? , Application :: VndAudiograph => write ! (f , "application/vnd.audiograph") ? , Application :: VndAutopackage => write ! (f , "application/vnd.autopackage") ? , Application :: VndAvalonJson => write ! (f , "application/vnd.avalon+json") ? , Application :: VndAvistarXml => write ! (f , "application/vnd.avistar+xml") ? , Application :: VndBalsamiqBmmlXml => write ! (f , "application/vnd.balsamiq.bmml+xml") ? , Application :: VndBananaAccounting => write ! (f , "application/vnd.banana-accounting") ? , Application :: VndBbfUspError => write ! (f , "application/vnd.bbf.usp.error") ? , Application :: VndBbfUspMsg => write ! (f , "application/vnd.bbf.usp.msg") ? , Application :: VndBbfUspMsgJson => write ! (f , "application/vnd.bbf.usp.msg+json") ? , Application :: VndBalsamiqBmpr => write ! (f , "application/vnd.balsamiq.bmpr") ? , Application :: VndBekitzurStechJson => write ! (f , "application/vnd.bekitzur-stech+json") ? , Application :: VndBelightsoftLhzdZip => write ! (f , "application/vnd.belightsoft.lhzd+zip") ? , Application :: VndBelightsoftLhzlZip => write ! (f , "application/vnd.belightsoft.lhzl+zip") ? , Application :: VndBintMedContent => write ! (f , "application/vnd.bint.med-content") ? , Application :: VndBiopaxRdfXml => write ! (f , "application/vnd.biopax.rdf+xml") ? , Application :: VndBlinkIdbValueWrapper => write ! (f , "application/vnd.blink-idb-value-wrapper") ? , Application :: VndBlueiceMultipass => write ! (f , "application/vnd.blueice.multipass") ? , Application :: VndBluetoothEpOob => write ! (f , "application/vnd.bluetooth.ep.oob") ? , Application :: VndBluetoothLeOob => write ! (f , "application/vnd.bluetooth.le.oob") ? , Application :: VndBmi => write ! (f , "application/vnd.bmi") ? , Application :: VndBpf => write ! (f , "application/vnd.bpf") ? , Application :: VndBpf3 => write ! (f , "application/vnd.bpf3") ? , Application :: VndBusinessobjects => write ! (f , "application/vnd.businessobjects") ? , Application :: VndByuUapiJson => write ! (f , "application/vnd.byu.uapi+json") ? , Application :: VndCabJscript => write ! (f , "application/vnd.cab-jscript") ? , Application :: VndCanonCpdl => write ! (f , "application/vnd.canon-cpdl") ? , Application :: VndCanonLips => write ! (f , "application/vnd.canon-lips") ? , Application :: VndCapasystemsPgJson => write ! (f , "application/vnd.capasystems-pg+json") ? , Application :: VndCendioThinlincClientconf => write ! (f , "application/vnd.cendio.thinlinc.clientconf") ? , Application :: VndCenturySystemsTcpStream => write ! (f , "application/vnd.century-systems.tcp_stream") ? , Application :: VndChemdrawXml => write ! (f , "application/vnd.chemdraw+xml") ? , Application :: VndChessPgn => write ! (f , "application/vnd.chess-pgn") ? , Application :: VndChipnutsKaraokeMmd => write ! (f , "application/vnd.chipnuts.karaoke-mmd") ? , Application :: VndCiedi => write ! (f , "application/vnd.ciedi") ? , Application :: VndCinderella => write ! (f , "application/vnd.cinderella") ? , Application :: VndCirpackIsdnExt => write ! (f , "application/vnd.cirpack.isdn-ext") ? , Application :: VndCitationstylesStyleXml => write ! (f , "application/vnd.citationstyles.style+xml") ? , Application :: VndClaymore => write ! (f , "application/vnd.claymore") ? , Application :: VndCloantoRp9 => write ! (f , "application/vnd.cloanto.rp9") ? , Application :: VndClonkC4Group => write ! (f , "application/vnd.clonk.c4group") ? , Application :: VndCluetrustCartomobileConfig => write ! (f , "application/vnd.cluetrust.cartomobile-config") ? , Application :: VndCluetrustCartomobileConfigPkg => write ! (f , "application/vnd.cluetrust.cartomobile-config-pkg") ? , Application :: VndCncfHelmChartContentV1TarGzip => write ! (f , "application/vnd.cncf.helm.chart.content.v1.tar+gzip") ? , Application :: VndCncfHelmChartProvenanceV1Prov => write ! (f , "application/vnd.cncf.helm.chart.provenance.v1.prov") ? , Application :: VndCncfHelmConfigV1Json => write ! (f , "application/vnd.cncf.helm.config.v1+json") ? , Application :: VndCoffeescript => write ! (f , "application/vnd.coffeescript") ? , Application :: VndCollabioXodocumentsDocument => write ! (f , "application/vnd.collabio.xodocuments.document") ? , Application :: VndCollabioXodocumentsDocumentTemplate => write ! (f , "application/vnd.collabio.xodocuments.document-template") ? , Application :: VndCollabioXodocumentsPresentation => write ! (f , "application/vnd.collabio.xodocuments.presentation") ? , Application :: VndCollabioXodocumentsPresentationTemplate => write ! (f , "application/vnd.collabio.xodocuments.presentation-template") ? , Application :: VndCollabioXodocumentsSpreadsheet => write ! (f , "application/vnd.collabio.xodocuments.spreadsheet") ? , Application :: VndCollabioXodocumentsSpreadsheetTemplate => write ! (f , "application/vnd.collabio.xodocuments.spreadsheet-template") ? , Application :: VndCollectionDocJson => write ! (f , "application/vnd.collection.doc+json") ? , Application :: VndCollectionJson => write ! (f , "application/vnd.collection+json") ? , Application :: VndCollectionNextJson => write ! (f , "application/vnd.collection.next+json") ? , Application :: VndComicbookRar => write ! (f , "application/vnd.comicbook-rar") ? , Application :: VndComicbookZip => write ! (f , "application/vnd.comicbook+zip") ? , Application :: VndCommerceBattelle => write ! (f , "application/vnd.commerce-battelle") ? , Application :: VndCommonspace => write ! (f , "application/vnd.commonspace") ? , Application :: VndCoreosIgnitionJson => write ! (f , "application/vnd.coreos.ignition+json") ? , Application :: VndCosmocaller => write ! (f , "application/vnd.cosmocaller") ? , Application :: VndContactCmsg => write ! (f , "application/vnd.contact.cmsg") ? , Application :: VndCrickClicker => write ! (f , "application/vnd.crick.clicker") ? , Application :: VndCrickClickerKeyboard => write ! (f , "application/vnd.crick.clicker.keyboard") ? , Application :: VndCrickClickerPalette => write ! (f , "application/vnd.crick.clicker.palette") ? , Application :: VndCrickClickerTemplate => write ! (f , "application/vnd.crick.clicker.template") ? , Application :: VndCrickClickerWordbank => write ! (f , "application/vnd.crick.clicker.wordbank") ? , Application :: VndCriticaltoolsWbsXml => write ! (f , "application/vnd.criticaltools.wbs+xml") ? , Application :: VndCryptiiPipeJson => write ! (f , "application/vnd.cryptii.pipe+json") ? , Application :: VndCryptoShadeFile => write ! (f , "application/vnd.crypto-shade-file") ? , Application :: VndCryptomatorEncrypted => write ! (f , "application/vnd.cryptomator.encrypted") ? , Application :: VndCryptomatorVault => write ! (f , "application/vnd.cryptomator.vault") ? , Application :: VndCtcPosml => write ! (f , "application/vnd.ctc-posml") ? , Application :: VndCtctWsXml => write ! (f , "application/vnd.ctct.ws+xml") ? , Application :: VndCupsPdf => write ! (f , "application/vnd.cups-pdf") ? , Application :: VndCupsPostscript => write ! (f , "application/vnd.cups-postscript") ? , Application :: VndCupsPpd => write ! (f , "application/vnd.cups-ppd") ? , Application :: VndCupsRaster => write ! (f , "application/vnd.cups-raster") ? , Application :: VndCupsRaw => write ! (f , "application/vnd.cups-raw") ? , Application :: VndCurl => write ! (f , "application/vnd.curl") ? , Application :: VndCyanDeanRootXml => write ! (f , "application/vnd.cyan.dean.root+xml") ? , Application :: VndCybank => write ! (f , "application/vnd.cybank") ? , Application :: VndCyclonedxJson => write ! (f , "application/vnd.cyclonedx+json") ? , Application :: VndCyclonedxXml => write ! (f , "application/vnd.cyclonedx+xml") ? , Application :: VndD2LCoursepackage1P0Zip => write ! (f , "application/vnd.d2l.coursepackage1p0+zip") ? , Application :: VndD3MDataset => write ! (f , "application/vnd.d3m-dataset") ? , Application :: VndD3MProblem => write ! (f , "application/vnd.d3m-problem") ? , Application :: VndDart => write ! (f , "application/vnd.dart") ? , Application :: VndDataVisionRdz => write ! (f , "application/vnd.data-vision.rdz") ? , Application :: VndDatalog => write ! (f , "application/vnd.datalog") ? , Application :: VndDatapackageJson => write ! (f , "application/vnd.datapackage+json") ? , Application :: VndDataresourceJson => write ! (f , "application/vnd.dataresource+json") ? , Application :: VndDbf => write ! (f , "application/vnd.dbf") ? , Application :: VndDebianBinaryPackage => write ! (f , "application/vnd.debian.binary-package") ? , Application :: VndDeceData => write ! (f , "application/vnd.dece.data") ? , Application :: VndDeceTtmlXml => write ! (f , "application/vnd.dece.ttml+xml") ? , Application :: VndDeceUnspecified => write ! (f , "application/vnd.dece.unspecified") ? , Application :: VndDeceZip => write ! (f , "application/vnd.dece.zip") ? , Application :: VndDenovoFcselayoutLink => write ! (f , "application/vnd.denovo.fcselayout-link") ? , Application :: VndDesmumeMovie => write ! (f , "application/vnd.desmume.movie") ? , Application :: VndDirBiPlateDlNosuffix => write ! (f , "application/vnd.dir-bi.plate-dl-nosuffix") ? , Application :: VndDmDelegationXml => write ! (f , "application/vnd.dm.delegation+xml") ? , Application :: VndDna => write ! (f , "application/vnd.dna") ? , Application :: VndDocumentJson => write ! (f , "application/vnd.document+json") ? , Application :: VndDolbyMobile1 => write ! (f , "application/vnd.dolby.mobile.1") ? , Application :: VndDolbyMobile2 => write ! (f , "application/vnd.dolby.mobile.2") ? , Application :: VndDoremirScorecloudBinaryDocument => write ! (f , "application/vnd.doremir.scorecloud-binary-document") ? , Application :: VndDpgraph => write ! (f , "application/vnd.dpgraph") ? , Application :: VndDreamfactory => write ! (f , "application/vnd.dreamfactory") ? , Application :: VndDriveJson => write ! (f , "application/vnd.drive+json") ? , Application :: VndDtgLocal => write ! (f , "application/vnd.dtg.local") ? , Application :: VndDtgLocalFlash => write ! (f , "application/vnd.dtg.local.flash") ? , Application :: VndDtgLocalHtml => write ! (f , "application/vnd.dtg.local.html") ? , Application :: VndDvbAit => write ! (f , "application/vnd.dvb.ait") ? , Application :: VndDvbDvbislXml => write ! (f , "application/vnd.dvb.dvbisl+xml") ? , Application :: VndDvbDvbj => write ! (f , "application/vnd.dvb.dvbj") ? , Application :: VndDvbEsgcontainer => write ! (f , "application/vnd.dvb.esgcontainer") ? , Application :: VndDvbIpdcdftnotifaccess => write ! (f , "application/vnd.dvb.ipdcdftnotifaccess") ? , Application :: VndDvbIpdcesgaccess => write ! (f , "application/vnd.dvb.ipdcesgaccess") ? , Application :: VndDvbIpdcesgaccess2 => write ! (f , "application/vnd.dvb.ipdcesgaccess2") ? , Application :: VndDvbIpdcesgpdd => write ! (f , "application/vnd.dvb.ipdcesgpdd") ? , Application :: VndDvbIpdcroaming => write ! (f , "application/vnd.dvb.ipdcroaming") ? , Application :: VndDvbIptvAlfecBase => write ! (f , "application/vnd.dvb.iptv.alfec-base") ? , Application :: VndDvbIptvAlfecEnhancement => write ! (f , "application/vnd.dvb.iptv.alfec-enhancement") ? , Application :: VndDvbNotifAggregateRootXml => write ! (f , "application/vnd.dvb.notif-aggregate-root+xml") ? , Application :: VndDvbNotifContainerXml => write ! (f , "application/vnd.dvb.notif-container+xml") ? , Application :: VndDvbNotifGenericXml => write ! (f , "application/vnd.dvb.notif-generic+xml") ? , Application :: VndDvbNotifIaMsglistXml => write ! (f , "application/vnd.dvb.notif-ia-msglist+xml") ? , Application :: VndDvbNotifIaRegistrationRequestXml => write ! (f , "application/vnd.dvb.notif-ia-registration-request+xml") ? , Application :: VndDvbNotifIaRegistrationResponseXml => write ! (f , "application/vnd.dvb.notif-ia-registration-response+xml") ? , Application :: VndDvbNotifInitXml => write ! (f , "application/vnd.dvb.notif-init+xml") ? , Application :: VndDvbPfr => write ! (f , "application/vnd.dvb.pfr") ? , Application :: VndDvbService => write ! (f , "application/vnd.dvb.service") ? , Application :: VndDxr => write ! (f , "application/vnd.dxr") ? , Application :: VndDynageo => write ! (f , "application/vnd.dynageo") ? , Application :: VndDzr => write ! (f , "application/vnd.dzr") ? , Application :: VndEasykaraokeCdgdownload => write ! (f , "application/vnd.easykaraoke.cdgdownload") ? , Application :: VndEcipRlp => write ! (f , "application/vnd.ecip.rlp") ? , Application :: VndEcdisUpdate => write ! (f , "application/vnd.ecdis-update") ? , Application :: VndEclipseDittoJson => write ! (f , "application/vnd.eclipse.ditto+json") ? , Application :: VndEcowinChart => write ! (f , "application/vnd.ecowin.chart") ? , Application :: VndEcowinFilerequest => write ! (f , "application/vnd.ecowin.filerequest") ? , Application :: VndEcowinFileupdate => write ! (f , "application/vnd.ecowin.fileupdate") ? , Application :: VndEcowinSeries => write ! (f , "application/vnd.ecowin.series") ? , Application :: VndEcowinSeriesrequest => write ! (f , "application/vnd.ecowin.seriesrequest") ? , Application :: VndEcowinSeriesupdate => write ! (f , "application/vnd.ecowin.seriesupdate") ? , Application :: VndEfiImg => write ! (f , "application/vnd.efi.img") ? , Application :: VndEfiIso => write ! (f , "application/vnd.efi.iso") ? , Application :: VndElnZip => write ! (f , "application/vnd.eln+zip") ? , Application :: VndEmclientAccessrequestXml => write ! (f , "application/vnd.emclient.accessrequest+xml") ? , Application :: VndEnliven => write ! (f , "application/vnd.enliven") ? , Application :: VndEnphaseEnvoy => write ! (f , "application/vnd.enphase.envoy") ? , Application :: VndEprintsDataXml => write ! (f , "application/vnd.eprints.data+xml") ? , Application :: VndEpsonEsf => write ! (f , "application/vnd.epson.esf") ? , Application :: VndEpsonMsf => write ! (f , "application/vnd.epson.msf") ? , Application :: VndEpsonQuickanime => write ! (f , "application/vnd.epson.quickanime") ? , Application :: VndEpsonSalt => write ! (f , "application/vnd.epson.salt") ? , Application :: VndEpsonSsf => write ! (f , "application/vnd.epson.ssf") ? , Application :: VndEricssonQuickcall => write ! (f , "application/vnd.ericsson.quickcall") ? , Application :: VndEspassEspassZip => write ! (f , "application/vnd.espass-espass+zip") ? , Application :: VndEszigno3Xml => write ! (f , "application/vnd.eszigno3+xml") ? , Application :: VndEtsiAocXml => write ! (f , "application/vnd.etsi.aoc+xml") ? , Application :: VndEtsiAsicSZip => write ! (f , "application/vnd.etsi.asic-s+zip") ? , Application :: VndEtsiAsicEZip => write ! (f , "application/vnd.etsi.asic-e+zip") ? , Application :: VndEtsiCugXml => write ! (f , "application/vnd.etsi.cug+xml") ? , Application :: VndEtsiIptvcommandXml => write ! (f , "application/vnd.etsi.iptvcommand+xml") ? , Application :: VndEtsiIptvdiscoveryXml => write ! (f , "application/vnd.etsi.iptvdiscovery+xml") ? , Application :: VndEtsiIptvprofileXml => write ! (f , "application/vnd.etsi.iptvprofile+xml") ? , Application :: VndEtsiIptvsadBcXml => write ! (f , "application/vnd.etsi.iptvsad-bc+xml") ? , Application :: VndEtsiIptvsadCodXml => write ! (f , "application/vnd.etsi.iptvsad-cod+xml") ? , Application :: VndEtsiIptvsadNpvrXml => write ! (f , "application/vnd.etsi.iptvsad-npvr+xml") ? , Application :: VndEtsiIptvserviceXml => write ! (f , "application/vnd.etsi.iptvservice+xml") ? , Application :: VndEtsiIptvsyncXml => write ! (f , "application/vnd.etsi.iptvsync+xml") ? , Application :: VndEtsiIptvueprofileXml => write ! (f , "application/vnd.etsi.iptvueprofile+xml") ? , Application :: VndEtsiMcidXml => write ! (f , "application/vnd.etsi.mcid+xml") ? , Application :: VndEtsiMheg5 => write ! (f , "application/vnd.etsi.mheg5") ? , Application :: VndEtsiOverloadControlPolicyDatasetXml => write ! (f , "application/vnd.etsi.overload-control-policy-dataset+xml") ? , Application :: VndEtsiPstnXml => write ! (f , "application/vnd.etsi.pstn+xml") ? , Application :: VndEtsiSciXml => write ! (f , "application/vnd.etsi.sci+xml") ? , Application :: VndEtsiSimservsXml => write ! (f , "application/vnd.etsi.simservs+xml") ? , Application :: VndEtsiTimestampToken => write ! (f , "application/vnd.etsi.timestamp-token") ? , Application :: VndEtsiTslXml => write ! (f , "application/vnd.etsi.tsl+xml") ? , Application :: VndEtsiTslDer => write ! (f , "application/vnd.etsi.tsl.der") ? , Application :: VndEuKasparianCarJson => write ! (f , "application/vnd.eu.kasparian.car+json") ? , Application :: VndEudoraData => write ! (f , "application/vnd.eudora.data") ? , Application :: VndEvolvEcigProfile => write ! (f , "application/vnd.evolv.ecig.profile") ? , Application :: VndEvolvEcigSettings => write ! (f , "application/vnd.evolv.ecig.settings") ? , Application :: VndEvolvEcigTheme => write ! (f , "application/vnd.evolv.ecig.theme") ? , Application :: VndExstreamEmpowerZip => write ! (f , "application/vnd.exstream-empower+zip") ? , Application :: VndExstreamPackage => write ! (f , "application/vnd.exstream-package") ? , Application :: VndEzpixAlbum => write ! (f , "application/vnd.ezpix-album") ? , Application :: VndEzpixPackage => write ! (f , "application/vnd.ezpix-package") ? , Application :: VndFSecureMobile => write ! (f , "application/vnd.f-secure.mobile") ? , Application :: VndFastcopyDiskImage => write ! (f , "application/vnd.fastcopy-disk-image") ? , Application :: VndFamilysearchGedcomZip => write ! (f , "application/vnd.familysearch.gedcom+zip") ? , Application :: VndFdsnMseed => write ! (f , "application/vnd.fdsn.mseed") ? , Application :: VndFdsnSeed => write ! (f , "application/vnd.fdsn.seed") ? , Application :: VndFfsns => write ! (f , "application/vnd.ffsns") ? , Application :: VndFiclabFlbZip => write ! (f , "application/vnd.ficlab.flb+zip") ? , Application :: VndFilmitZfc => write ! (f , "application/vnd.filmit.zfc") ? , Application :: VndFints => write ! (f , "application/vnd.fints") ? , Application :: VndFiremonkeysCloudcell => write ! (f , "application/vnd.firemonkeys.cloudcell") ? , Application :: VndFloGraphIt => write ! (f , "application/vnd.FloGraphIt") ? , Application :: VndFluxtimeClip => write ! (f , "application/vnd.fluxtime.clip") ? , Application :: VndFontFontforgeSfd => write ! (f , "application/vnd.font-fontforge-sfd") ? , Application :: VndFramemaker => write ! (f , "application/vnd.framemaker") ? , Application :: VndFscWeblaunch => write ! (f , "application/vnd.fsc.weblaunch") ? , Application :: VndFujifilmFbDocuworks => write ! (f , "application/vnd.fujifilm.fb.docuworks") ? , Application :: VndFujifilmFbDocuworksBinder => write ! (f , "application/vnd.fujifilm.fb.docuworks.binder") ? , Application :: VndFujifilmFbDocuworksContainer => write ! (f , "application/vnd.fujifilm.fb.docuworks.container") ? , Application :: VndFujifilmFbJfiXml => write ! (f , "application/vnd.fujifilm.fb.jfi+xml") ? , Application :: VndFujitsuOasys => write ! (f , "application/vnd.fujitsu.oasys") ? , Application :: VndFujitsuOasys2 => write ! (f , "application/vnd.fujitsu.oasys2") ? , Application :: VndFujitsuOasys3 => write ! (f , "application/vnd.fujitsu.oasys3") ? , Application :: VndFujitsuOasysgp => write ! (f , "application/vnd.fujitsu.oasysgp") ? , Application :: VndFujitsuOasysprs => write ! (f , "application/vnd.fujitsu.oasysprs") ? , Application :: VndFujixeroxART4 => write ! (f , "application/vnd.fujixerox.ART4") ? , Application :: VndFujixeroxARTEX => write ! (f , "application/vnd.fujixerox.ART-EX") ? , Application :: VndFujixeroxDdd => write ! (f , "application/vnd.fujixerox.ddd") ? , Application :: VndFujixeroxDocuworks => write ! (f , "application/vnd.fujixerox.docuworks") ? , Application :: VndFujixeroxDocuworksBinder => write ! (f , "application/vnd.fujixerox.docuworks.binder") ? , Application :: VndFujixeroxDocuworksContainer => write ! (f , "application/vnd.fujixerox.docuworks.container") ? , Application :: VndFujixeroxHBPL => write ! (f , "application/vnd.fujixerox.HBPL") ? , Application :: VndFutMisnet => write ! (f , "application/vnd.fut-misnet") ? , Application :: VndFutoinCbor => write ! (f , "application/vnd.futoin+cbor") ? , Application :: VndFutoinJson => write ! (f , "application/vnd.futoin+json") ? , Application :: VndFuzzysheet => write ! (f , "application/vnd.fuzzysheet") ? , Application :: VndGenomatixTuxedo => write ! (f , "application/vnd.genomatix.tuxedo") ? , Application :: VndGenozip => write ! (f , "application/vnd.genozip") ? , Application :: VndGenticsGrdJson => write ! (f , "application/vnd.gentics.grd+json") ? , Application :: VndGentooCatmetadataXml => write ! (f , "application/vnd.gentoo.catmetadata+xml") ? , Application :: VndGentooEbuild => write ! (f , "application/vnd.gentoo.ebuild") ? , Application :: VndGentooEclass => write ! (f , "application/vnd.gentoo.eclass") ? , Application :: VndGentooGpkg => write ! (f , "application/vnd.gentoo.gpkg") ? , Application :: VndGentooManifest => write ! (f , "application/vnd.gentoo.manifest") ? , Application :: VndGentooXpak => write ! (f , "application/vnd.gentoo.xpak") ? , Application :: VndGentooPkgmetadataXml => write ! (f , "application/vnd.gentoo.pkgmetadata+xml") ? , Application :: VndGeogebraFile => write ! (f , "application/vnd.geogebra.file") ? , Application :: VndGeogebraSlides => write ! (f , "application/vnd.geogebra.slides") ? , Application :: VndGeogebraTool => write ! (f , "application/vnd.geogebra.tool") ? , Application :: VndGeometryExplorer => write ! (f , "application/vnd.geometry-explorer") ? , Application :: VndGeonext => write ! (f , "application/vnd.geonext") ? , Application :: VndGeoplan => write ! (f , "application/vnd.geoplan") ? , Application :: VndGeospace => write ! (f , "application/vnd.geospace") ? , Application :: VndGerber => write ! (f , "application/vnd.gerber") ? , Application :: VndGlobalplatformCardContentMgt => write ! (f , "application/vnd.globalplatform.card-content-mgt") ? , Application :: VndGlobalplatformCardContentMgtResponse => write ! (f , "application/vnd.globalplatform.card-content-mgt-response") ? , Application :: VndGnuTalerExchangeJson => write ! (f , "application/vnd.gnu.taler.exchange+json") ? , Application :: VndGnuTalerMerchantJson => write ! (f , "application/vnd.gnu.taler.merchant+json") ? , Application :: VndGoogleEarthKmlXml => write ! (f , "application/vnd.google-earth.kml+xml") ? , Application :: VndGoogleEarthKmz => write ! (f , "application/vnd.google-earth.kmz") ? , Application :: VndGovSkEFormXml => write ! (f , "application/vnd.gov.sk.e-form+xml") ? , Application :: VndGovSkEFormZip => write ! (f , "application/vnd.gov.sk.e-form+zip") ? , Application :: VndGovSkXmldatacontainerXml => write ! (f , "application/vnd.gov.sk.xmldatacontainer+xml") ? , Application :: VndGpxseeMapXml => write ! (f , "application/vnd.gpxsee.map+xml") ? , Application :: VndGrafeq => write ! (f , "application/vnd.grafeq") ? , Application :: VndGridmp => write ! (f , "application/vnd.gridmp") ? , Application :: VndGrooveAccount => write ! (f , "application/vnd.groove-account") ? , Application :: VndGrooveHelp => write ! (f , "application/vnd.groove-help") ? , Application :: VndGrooveIdentityMessage => write ! (f , "application/vnd.groove-identity-message") ? , Application :: VndGrooveInjector => write ! (f , "application/vnd.groove-injector") ? , Application :: VndGrooveToolMessage => write ! (f , "application/vnd.groove-tool-message") ? , Application :: VndGrooveToolTemplate => write ! (f , "application/vnd.groove-tool-template") ? , Application :: VndGrooveVcard => write ! (f , "application/vnd.groove-vcard") ? , Application :: VndHalJson => write ! (f , "application/vnd.hal+json") ? , Application :: VndHalXml => write ! (f , "application/vnd.hal+xml") ? , Application :: VndHandHeldEntertainmentXml => write ! (f , "application/vnd.HandHeld-Entertainment+xml") ? , Application :: VndHbci => write ! (f , "application/vnd.hbci") ? , Application :: VndHcJson => write ! (f , "application/vnd.hc+json") ? , Application :: VndHclBireports => write ! (f , "application/vnd.hcl-bireports") ? , Application :: VndHdt => write ! (f , "application/vnd.hdt") ? , Application :: VndHerokuJson => write ! (f , "application/vnd.heroku+json") ? , Application :: VndHheLessonPlayer => write ! (f , "application/vnd.hhe.lesson-player") ? , Application :: VndHpHPGL => write ! (f , "application/vnd.hp-HPGL") ? , Application :: VndHpHpid => write ! (f , "application/vnd.hp-hpid") ? , Application :: VndHpHps => write ! (f , "application/vnd.hp-hps") ? , Application :: VndHpJlyt => write ! (f , "application/vnd.hp-jlyt") ? , Application :: VndHpPCL => write ! (f , "application/vnd.hp-PCL") ? , Application :: VndHpPCLXL => write ! (f , "application/vnd.hp-PCLXL") ? , Application :: VndHsl => write ! (f , "application/vnd.hsl") ? , Application :: VndHttphone => write ! (f , "application/vnd.httphone") ? , Application :: VndHydrostatixSofData => write ! (f , "application/vnd.hydrostatix.sof-data") ? , Application :: VndHyperItemJson => write ! (f , "application/vnd.hyper-item+json") ? , Application :: VndHyperJson => write ! (f , "application/vnd.hyper+json") ? , Application :: VndHyperdriveJson => write ! (f , "application/vnd.hyperdrive+json") ? , Application :: VndHzn3DCrossword => write ! (f , "application/vnd.hzn-3d-crossword") ? , Application :: VndIbmElectronicMedia => write ! (f , "application/vnd.ibm.electronic-media") ? , Application :: VndIbmMiniPay => write ! (f , "application/vnd.ibm.MiniPay") ? , Application :: VndIbmRightsManagement => write ! (f , "application/vnd.ibm.rights-management") ? , Application :: VndIbmSecureContainer => write ! (f , "application/vnd.ibm.secure-container") ? , Application :: VndIccprofile => write ! (f , "application/vnd.iccprofile") ? , Application :: VndIeee1905 => write ! (f , "application/vnd.ieee.1905") ? , Application :: VndIgloader => write ! (f , "application/vnd.igloader") ? , Application :: VndImagemeterFolderZip => write ! (f , "application/vnd.imagemeter.folder+zip") ? , Application :: VndImagemeterImageZip => write ! (f , "application/vnd.imagemeter.image+zip") ? , Application :: VndImmervisionIvp => write ! (f , "application/vnd.immervision-ivp") ? , Application :: VndImmervisionIvu => write ! (f , "application/vnd.immervision-ivu") ? , Application :: VndImsImsccv1P1 => write ! (f , "application/vnd.ims.imsccv1p1") ? , Application :: VndImsImsccv1P2 => write ! (f , "application/vnd.ims.imsccv1p2") ? , Application :: VndImsImsccv1P3 => write ! (f , "application/vnd.ims.imsccv1p3") ? , Application :: VndImsLisV2ResultJson => write ! (f , "application/vnd.ims.lis.v2.result+json") ? , Application :: VndImsLtiV2ToolconsumerprofileJson => write ! (f , "application/vnd.ims.lti.v2.toolconsumerprofile+json") ? , Application :: VndImsLtiV2ToolproxyIdJson => write ! (f , "application/vnd.ims.lti.v2.toolproxy.id+json") ? , Application :: VndImsLtiV2ToolproxyJson => write ! (f , "application/vnd.ims.lti.v2.toolproxy+json") ? , Application :: VndImsLtiV2ToolsettingsJson => write ! (f , "application/vnd.ims.lti.v2.toolsettings+json") ? , Application :: VndImsLtiV2ToolsettingsSimpleJson => write ! (f , "application/vnd.ims.lti.v2.toolsettings.simple+json") ? , Application :: VndInformedcontrolRmsXml => write ! (f , "application/vnd.informedcontrol.rms+xml") ? , Application :: VndInfotechProject => write ! (f , "application/vnd.infotech.project") ? , Application :: VndInfotechProjectXml => write ! (f , "application/vnd.infotech.project+xml") ? , Application :: VndInnopathWampNotification => write ! (f , "application/vnd.innopath.wamp.notification") ? , Application :: VndInsorsIgm => write ! (f , "application/vnd.insors.igm") ? , Application :: VndInterconFormnet => write ! (f , "application/vnd.intercon.formnet") ? , Application :: VndIntergeo => write ! (f , "application/vnd.intergeo") ? , Application :: VndIntertrustDigibox => write ! (f , "application/vnd.intertrust.digibox") ? , Application :: VndIntertrustNncp => write ! (f , "application/vnd.intertrust.nncp") ? , Application :: VndIntuQbo => write ! (f , "application/vnd.intu.qbo") ? , Application :: VndIntuQfx => write ! (f , "application/vnd.intu.qfx") ? , Application :: VndIpfsIpnsRecord => write ! (f , "application/vnd.ipfs.ipns-record") ? , Application :: VndIpldCar => write ! (f , "application/vnd.ipld.car") ? , Application :: VndIpldDagCbor => write ! (f , "application/vnd.ipld.dag-cbor") ? , Application :: VndIpldDagJson => write ! (f , "application/vnd.ipld.dag-json") ? , Application :: VndIpldRaw => write ! (f , "application/vnd.ipld.raw") ? , Application :: VndIptcG2CatalogitemXml => write ! (f , "application/vnd.iptc.g2.catalogitem+xml") ? , Application :: VndIptcG2ConceptitemXml => write ! (f , "application/vnd.iptc.g2.conceptitem+xml") ? , Application :: VndIptcG2KnowledgeitemXml => write ! (f , "application/vnd.iptc.g2.knowledgeitem+xml") ? , Application :: VndIptcG2NewsitemXml => write ! (f , "application/vnd.iptc.g2.newsitem+xml") ? , Application :: VndIptcG2NewsmessageXml => write ! (f , "application/vnd.iptc.g2.newsmessage+xml") ? , Application :: VndIptcG2PackageitemXml => write ! (f , "application/vnd.iptc.g2.packageitem+xml") ? , Application :: VndIptcG2PlanningitemXml => write ! (f , "application/vnd.iptc.g2.planningitem+xml") ? , Application :: VndIpunpluggedRcprofile => write ! (f , "application/vnd.ipunplugged.rcprofile") ? , Application :: VndIrepositoryPackageXml => write ! (f , "application/vnd.irepository.package+xml") ? , Application :: VndIsXpr => write ! (f , "application/vnd.is-xpr") ? , Application :: VndIsacFcs => write ! (f , "application/vnd.isac.fcs") ? , Application :: VndJam => write ! (f , "application/vnd.jam") ? , Application :: VndIso1178310Zip => write ! (f , "application/vnd.iso11783-10+zip") ? , Application :: VndJapannetDirectoryService => write ! (f , "application/vnd.japannet-directory-service") ? , Application :: VndJapannetJpnstoreWakeup => write ! (f , "application/vnd.japannet-jpnstore-wakeup") ? , Application :: VndJapannetPaymentWakeup => write ! (f , "application/vnd.japannet-payment-wakeup") ? , Application :: VndJapannetRegistration => write ! (f , "application/vnd.japannet-registration") ? , Application :: VndJapannetRegistrationWakeup => write ! (f , "application/vnd.japannet-registration-wakeup") ? , Application :: VndJapannetSetstoreWakeup => write ! (f , "application/vnd.japannet-setstore-wakeup") ? , Application :: VndJapannetVerification => write ! (f , "application/vnd.japannet-verification") ? , Application :: VndJapannetVerificationWakeup => write ! (f , "application/vnd.japannet-verification-wakeup") ? , Application :: VndJcpJavameMidletRms => write ! (f , "application/vnd.jcp.javame.midlet-rms") ? , Application :: VndJisp => write ! (f , "application/vnd.jisp") ? , Application :: VndJoostJodaArchive => write ! (f , "application/vnd.joost.joda-archive") ? , Application :: VndJskIsdnNgn => write ! (f , "application/vnd.jsk.isdn-ngn") ? , Application :: VndKahootz => write ! (f , "application/vnd.kahootz") ? , Application :: VndKdeKarbon => write ! (f , "application/vnd.kde.karbon") ? , Application :: VndKdeKchart => write ! (f , "application/vnd.kde.kchart") ? , Application :: VndKdeKformula => write ! (f , "application/vnd.kde.kformula") ? , Application :: VndKdeKivio => write ! (f , "application/vnd.kde.kivio") ? , Application :: VndKdeKontour => write ! (f , "application/vnd.kde.kontour") ? , Application :: VndKdeKpresenter => write ! (f , "application/vnd.kde.kpresenter") ? , Application :: VndKdeKspread => write ! (f , "application/vnd.kde.kspread") ? , Application :: VndKdeKword => write ! (f , "application/vnd.kde.kword") ? , Application :: VndKenameaapp => write ! (f , "application/vnd.kenameaapp") ? , Application :: VndKidspiration => write ! (f , "application/vnd.kidspiration") ? , Application :: VndKinar => write ! (f , "application/vnd.Kinar") ? , Application :: VndKoan => write ! (f , "application/vnd.koan") ? , Application :: VndKodakDescriptor => write ! (f , "application/vnd.kodak-descriptor") ? , Application :: VndLas => write ! (f , "application/vnd.las") ? , Application :: VndLasLasJson => write ! (f , "application/vnd.las.las+json") ? , Application :: VndLasLasXml => write ! (f , "application/vnd.las.las+xml") ? , Application :: VndLaszip => write ! (f , "application/vnd.laszip") ? , Application :: VndLeapJson => write ! (f , "application/vnd.leap+json") ? , Application :: VndLibertyRequestXml => write ! (f , "application/vnd.liberty-request+xml") ? , Application :: VndLlamagraphicsLifeBalanceDesktop => write ! (f , "application/vnd.llamagraphics.life-balance.desktop") ? , Application :: VndLlamagraphicsLifeBalanceExchangeXml => write ! (f , "application/vnd.llamagraphics.life-balance.exchange+xml") ? , Application :: VndLogipipeCircuitZip => write ! (f , "application/vnd.logipipe.circuit+zip") ? , Application :: VndLoom => write ! (f , "application/vnd.loom") ? , Application :: VndLotus123 => write ! (f , "application/vnd.lotus-1-2-3") ? , Application :: VndLotusApproach => write ! (f , "application/vnd.lotus-approach") ? , Application :: VndLotusFreelance => write ! (f , "application/vnd.lotus-freelance") ? , Application :: VndLotusNotes => write ! (f , "application/vnd.lotus-notes") ? , Application :: VndLotusOrganizer => write ! (f , "application/vnd.lotus-organizer") ? , Application :: VndLotusScreencam => write ! (f , "application/vnd.lotus-screencam") ? , Application :: VndLotusWordpro => write ! (f , "application/vnd.lotus-wordpro") ? , Application :: VndMacportsPortpkg => write ! (f , "application/vnd.macports.portpkg") ? , Application :: VndMapboxVectorTile => write ! (f , "application/vnd.mapbox-vector-tile") ? , Application :: VndMarlinDrmActiontokenXml => write ! (f , "application/vnd.marlin.drm.actiontoken+xml") ? , Application :: VndMarlinDrmConftokenXml => write ! (f , "application/vnd.marlin.drm.conftoken+xml") ? , Application :: VndMarlinDrmLicenseXml => write ! (f , "application/vnd.marlin.drm.license+xml") ? , Application :: VndMarlinDrmMdcf => write ! (f , "application/vnd.marlin.drm.mdcf") ? , Application :: VndMasonJson => write ! (f , "application/vnd.mason+json") ? , Application :: VndMaxarArchive3TzZip => write ! (f , "application/vnd.maxar.archive.3tz+zip") ? , Application :: VndMaxmindMaxmindDb => write ! (f , "application/vnd.maxmind.maxmind-db") ? , Application :: VndMcd => write ! (f , "application/vnd.mcd") ? , Application :: VndMdl => write ! (f , "application/vnd.mdl") ? , Application :: VndMdlMbsdf => write ! (f , "application/vnd.mdl-mbsdf") ? , Application :: VndMedcalcdata => write ! (f , "application/vnd.medcalcdata") ? , Application :: VndMediastationCdkey => write ! (f , "application/vnd.mediastation.cdkey") ? , Application :: VndMedicalholodeckRecordxr => write ! (f , "application/vnd.medicalholodeck.recordxr") ? , Application :: VndMeridianSlingshot => write ! (f , "application/vnd.meridian-slingshot") ? , Application :: VndMFER => write ! (f , "application/vnd.MFER") ? , Application :: VndMfmp => write ! (f , "application/vnd.mfmp") ? , Application :: VndMicroJson => write ! (f , "application/vnd.micro+json") ? , Application :: VndMicrografxFlo => write ! (f , "application/vnd.micrografx.flo") ? , Application :: VndMicrografxIgx => write ! (f , "application/vnd.micrografx.igx") ? , Application :: VndMicrosoftPortableExecutable => write ! (f , "application/vnd.microsoft.portable-executable") ? , Application :: VndMicrosoftWindowsThumbnailCache => write ! (f , "application/vnd.microsoft.windows.thumbnail-cache") ? , Application :: VndMieleJson => write ! (f , "application/vnd.miele+json") ? , Application :: VndMif => write ! (f , "application/vnd.mif") ? , Application :: VndMinisoftHp3000Save => write ! (f , "application/vnd.minisoft-hp3000-save") ? , Application :: VndMitsubishiMistyGuardTrustweb => write ! (f , "application/vnd.mitsubishi.misty-guard.trustweb") ? , Application :: VndMobiusDAF => write ! (f , "application/vnd.Mobius.DAF") ? , Application :: VndMobiusDIS => write ! (f , "application/vnd.Mobius.DIS") ? , Application :: VndMobiusMBK => write ! (f , "application/vnd.Mobius.MBK") ? , Application :: VndMobiusMQY => write ! (f , "application/vnd.Mobius.MQY") ? , Application :: VndMobiusMSL => write ! (f , "application/vnd.Mobius.MSL") ? , Application :: VndMobiusPLC => write ! (f , "application/vnd.Mobius.PLC") ? , Application :: VndMobiusTXF => write ! (f , "application/vnd.Mobius.TXF") ? , Application :: VndModl => write ! (f , "application/vnd.modl") ? , Application :: VndMophunApplication => write ! (f , "application/vnd.mophun.application") ? , Application :: VndMophunCertificate => write ! (f , "application/vnd.mophun.certificate") ? , Application :: VndMotorolaFlexsuite => write ! (f , "application/vnd.motorola.flexsuite") ? , Application :: VndMotorolaFlexsuiteAdsi => write ! (f , "application/vnd.motorola.flexsuite.adsi") ? , Application :: VndMotorolaFlexsuiteFis => write ! (f , "application/vnd.motorola.flexsuite.fis") ? , Application :: VndMotorolaFlexsuiteGotap => write ! (f , "application/vnd.motorola.flexsuite.gotap") ? , Application :: VndMotorolaFlexsuiteKmr => write ! (f , "application/vnd.motorola.flexsuite.kmr") ? , Application :: VndMotorolaFlexsuiteTtc => write ! (f , "application/vnd.motorola.flexsuite.ttc") ? , Application :: VndMotorolaFlexsuiteWem => write ! (f , "application/vnd.motorola.flexsuite.wem") ? , Application :: VndMotorolaIprm => write ! (f , "application/vnd.motorola.iprm") ? , Application :: VndMozillaXulXml => write ! (f , "application/vnd.mozilla.xul+xml") ? , Application :: VndMsArtgalry => write ! (f , "application/vnd.ms-artgalry") ? , Application :: VndMsAsf => write ! (f , "application/vnd.ms-asf") ? , Application :: VndMsCabCompressed => write ! (f , "application/vnd.ms-cab-compressed") ? , Application :: VndMs3Mfdocument => write ! (f , "application/vnd.ms-3mfdocument") ? , Application :: VndMsExcel => write ! (f , "application/vnd.ms-excel") ? , Application :: VndMsExcelAddinMacroEnabled12 => write ! (f , "application/vnd.ms-excel.addin.macroEnabled.12") ? , Application :: VndMsExcelSheetBinaryMacroEnabled12 => write ! (f , "application/vnd.ms-excel.sheet.binary.macroEnabled.12") ? , Application :: VndMsExcelSheetMacroEnabled12 => write ! (f , "application/vnd.ms-excel.sheet.macroEnabled.12") ? , Application :: VndMsExcelTemplateMacroEnabled12 => write ! (f , "application/vnd.ms-excel.template.macroEnabled.12") ? , Application :: VndMsFontobject => write ! (f , "application/vnd.ms-fontobject") ? , Application :: VndMsHtmlhelp => write ! (f , "application/vnd.ms-htmlhelp") ? , Application :: VndMsIms => write ! (f , "application/vnd.ms-ims") ? , Application :: VndMsLrm => write ! (f , "application/vnd.ms-lrm") ? , Application :: VndMsOfficeActiveXXml => write ! (f , "application/vnd.ms-office.activeX+xml") ? , Application :: VndMsOfficetheme => write ! (f , "application/vnd.ms-officetheme") ? , Application :: VndMsPlayreadyInitiatorXml => write ! (f , "application/vnd.ms-playready.initiator+xml") ? , Application :: VndMsPowerpoint => write ! (f , "application/vnd.ms-powerpoint") ? , Application :: VndMsPowerpointAddinMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.addin.macroEnabled.12") ? , Application :: VndMsPowerpointPresentationMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.presentation.macroEnabled.12") ? , Application :: VndMsPowerpointSlideMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.slide.macroEnabled.12") ? , Application :: VndMsPowerpointSlideshowMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.slideshow.macroEnabled.12") ? , Application :: VndMsPowerpointTemplateMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.template.macroEnabled.12") ? , Application :: VndMsPrintDeviceCapabilitiesXml => write ! (f , "application/vnd.ms-PrintDeviceCapabilities+xml") ? , Application :: VndMsPrintSchemaTicketXml => write ! (f , "application/vnd.ms-PrintSchemaTicket+xml") ? , Application :: VndMsProject => write ! (f , "application/vnd.ms-project") ? , Application :: VndMsTnef => write ! (f , "application/vnd.ms-tnef") ? , Application :: VndMsWindowsDevicepairing => write ! (f , "application/vnd.ms-windows.devicepairing") ? , Application :: VndMsWindowsNwprintingOob => write ! (f , "application/vnd.ms-windows.nwprinting.oob") ? , Application :: VndMsWindowsPrinterpairing => write ! (f , "application/vnd.ms-windows.printerpairing") ? , Application :: VndMsWindowsWsdOob => write ! (f , "application/vnd.ms-windows.wsd.oob") ? , Application :: VndMsWmdrmLicChlgReq => write ! (f , "application/vnd.ms-wmdrm.lic-chlg-req") ? , Application :: VndMsWmdrmLicResp => write ! (f , "application/vnd.ms-wmdrm.lic-resp") ? , Application :: VndMsWmdrmMeterChlgReq => write ! (f , "application/vnd.ms-wmdrm.meter-chlg-req") ? , Application :: VndMsWmdrmMeterResp => write ! (f , "application/vnd.ms-wmdrm.meter-resp") ? , Application :: VndMsWordDocumentMacroEnabled12 => write ! (f , "application/vnd.ms-word.document.macroEnabled.12") ? , Application :: VndMsWordTemplateMacroEnabled12 => write ! (f , "application/vnd.ms-word.template.macroEnabled.12") ? , Application :: VndMsWorks => write ! (f , "application/vnd.ms-works") ? , Application :: VndMsWpl => write ! (f , "application/vnd.ms-wpl") ? , Application :: VndMsXpsdocument => write ! (f , "application/vnd.ms-xpsdocument") ? , Application :: VndMsaDiskImage => write ! (f , "application/vnd.msa-disk-image") ? , Application :: VndMseq => write ! (f , "application/vnd.mseq") ? , Application :: VndMsign => write ! (f , "application/vnd.msign") ? , Application :: VndMultiadCreator => write ! (f , "application/vnd.multiad.creator") ? , Application :: VndMultiadCreatorCif => write ! (f , "application/vnd.multiad.creator.cif") ? , Application :: VndMusician => write ! (f , "application/vnd.musician") ? , Application :: VndMusicNiff => write ! (f , "application/vnd.music-niff") ? , Application :: VndMuveeStyle => write ! (f , "application/vnd.muvee.style") ? , Application :: VndMynfc => write ! (f , "application/vnd.mynfc") ? , Application :: VndNacamarYbridJson => write ! (f , "application/vnd.nacamar.ybrid+json") ? , Application :: VndNcdControl => write ! (f , "application/vnd.ncd.control") ? , Application :: VndNcdReference => write ! (f , "application/vnd.ncd.reference") ? , Application :: VndNearstInvJson => write ! (f , "application/vnd.nearst.inv+json") ? , Application :: VndNebumindLine => write ! (f , "application/vnd.nebumind.line") ? , Application :: VndNervana => write ! (f , "application/vnd.nervana") ? , Application :: VndNetfpx => write ! (f , "application/vnd.netfpx") ? , Application :: VndNeurolanguageNlu => write ! (f , "application/vnd.neurolanguage.nlu") ? , Application :: VndNimn => write ! (f , "application/vnd.nimn") ? , Application :: VndNintendoSnesRom => write ! (f , "application/vnd.nintendo.snes.rom") ? , Application :: VndNintendoNitroRom => write ! (f , "application/vnd.nintendo.nitro.rom") ? , Application :: VndNitf => write ! (f , "application/vnd.nitf") ? , Application :: VndNoblenetDirectory => write ! (f , "application/vnd.noblenet-directory") ? , Application :: VndNoblenetSealer => write ! (f , "application/vnd.noblenet-sealer") ? , Application :: VndNoblenetWeb => write ! (f , "application/vnd.noblenet-web") ? , Application :: VndNokiaCatalogs => write ! (f , "application/vnd.nokia.catalogs") ? , Application :: VndNokiaConmlWbxml => write ! (f , "application/vnd.nokia.conml+wbxml") ? , Application :: VndNokiaConmlXml => write ! (f , "application/vnd.nokia.conml+xml") ? , Application :: VndNokiaIptvConfigXml => write ! (f , "application/vnd.nokia.iptv.config+xml") ? , Application :: VndNokiaISDSRadioPresets => write ! (f , "application/vnd.nokia.iSDS-radio-presets") ? , Application :: VndNokiaLandmarkWbxml => write ! (f , "application/vnd.nokia.landmark+wbxml") ? , Application :: VndNokiaLandmarkXml => write ! (f , "application/vnd.nokia.landmark+xml") ? , Application :: VndNokiaLandmarkcollectionXml => write ! (f , "application/vnd.nokia.landmarkcollection+xml") ? , Application :: VndNokiaNcd => write ! (f , "application/vnd.nokia.ncd") ? , Application :: VndNokiaNGageAcXml => write ! (f , "application/vnd.nokia.n-gage.ac+xml") ? , Application :: VndNokiaNGageData => write ! (f , "application/vnd.nokia.n-gage.data") ? , Application :: VndNokiaPcdWbxml => write ! (f , "application/vnd.nokia.pcd+wbxml") ? , Application :: VndNokiaPcdXml => write ! (f , "application/vnd.nokia.pcd+xml") ? , Application :: VndNokiaRadioPreset => write ! (f , "application/vnd.nokia.radio-preset") ? , Application :: VndNokiaRadioPresets => write ! (f , "application/vnd.nokia.radio-presets") ? , Application :: VndNovadigmEDM => write ! (f , "application/vnd.novadigm.EDM") ? , Application :: VndNovadigmEDX => write ! (f , "application/vnd.novadigm.EDX") ? , Application :: VndNovadigmEXT => write ! (f , "application/vnd.novadigm.EXT") ? , Application :: VndNttLocalContentShare => write ! (f , "application/vnd.ntt-local.content-share") ? , Application :: VndNttLocalFileTransfer => write ! (f , "application/vnd.ntt-local.file-transfer") ? , Application :: VndNttLocalOgwRemoteAccess => write ! (f , "application/vnd.ntt-local.ogw_remote-access") ? , Application :: VndNttLocalSipTaRemote => write ! (f , "application/vnd.ntt-local.sip-ta_remote") ? , Application :: VndNttLocalSipTaTcpStream => write ! (f , "application/vnd.ntt-local.sip-ta_tcp_stream") ? , Application :: VndOasisOpendocumentBase => write ! (f , "application/vnd.oasis.opendocument.base") ? , Application :: VndOasisOpendocumentChart => write ! (f , "application/vnd.oasis.opendocument.chart") ? , Application :: VndOasisOpendocumentChartTemplate => write ! (f , "application/vnd.oasis.opendocument.chart-template") ? , Application :: VndOasisOpendocumentFormula => write ! (f , "application/vnd.oasis.opendocument.formula") ? , Application :: VndOasisOpendocumentFormulaTemplate => write ! (f , "application/vnd.oasis.opendocument.formula-template") ? , Application :: VndOasisOpendocumentGraphics => write ! (f , "application/vnd.oasis.opendocument.graphics") ? , Application :: VndOasisOpendocumentGraphicsTemplate => write ! (f , "application/vnd.oasis.opendocument.graphics-template") ? , Application :: VndOasisOpendocumentImage => write ! (f , "application/vnd.oasis.opendocument.image") ? , Application :: VndOasisOpendocumentImageTemplate => write ! (f , "application/vnd.oasis.opendocument.image-template") ? , Application :: VndOasisOpendocumentPresentation => write ! (f , "application/vnd.oasis.opendocument.presentation") ? , Application :: VndOasisOpendocumentPresentationTemplate => write ! (f , "application/vnd.oasis.opendocument.presentation-template") ? , Application :: VndOasisOpendocumentSpreadsheet => write ! (f , "application/vnd.oasis.opendocument.spreadsheet") ? , Application :: VndOasisOpendocumentSpreadsheetTemplate => write ! (f , "application/vnd.oasis.opendocument.spreadsheet-template") ? , Application :: VndOasisOpendocumentText => write ! (f , "application/vnd.oasis.opendocument.text") ? , Application :: VndOasisOpendocumentTextMaster => write ! (f , "application/vnd.oasis.opendocument.text-master") ? , Application :: VndOasisOpendocumentTextMasterTemplate => write ! (f , "application/vnd.oasis.opendocument.text-master-template") ? , Application :: VndOasisOpendocumentTextTemplate => write ! (f , "application/vnd.oasis.opendocument.text-template") ? , Application :: VndOasisOpendocumentTextWeb => write ! (f , "application/vnd.oasis.opendocument.text-web") ? , Application :: VndObn => write ! (f , "application/vnd.obn") ? , Application :: VndOcfCbor => write ! (f , "application/vnd.ocf+cbor") ? , Application :: VndOciImageManifestV1Json => write ! (f , "application/vnd.oci.image.manifest.v1+json") ? , Application :: VndOftnL10NJson => write ! (f , "application/vnd.oftn.l10n+json") ? , Application :: VndOipfContentaccessdownloadXml => write ! (f , "application/vnd.oipf.contentaccessdownload+xml") ? , Application :: VndOipfContentaccessstreamingXml => write ! (f , "application/vnd.oipf.contentaccessstreaming+xml") ? , Application :: VndOipfCspgHexbinary => write ! (f , "application/vnd.oipf.cspg-hexbinary") ? , Application :: VndOipfDaeSvgXml => write ! (f , "application/vnd.oipf.dae.svg+xml") ? , Application :: VndOipfDaeXhtmlXml => write ! (f , "application/vnd.oipf.dae.xhtml+xml") ? , Application :: VndOipfMippvcontrolmessageXml => write ! (f , "application/vnd.oipf.mippvcontrolmessage+xml") ? , Application :: VndOipfPaeGem => write ! (f , "application/vnd.oipf.pae.gem") ? , Application :: VndOipfSpdiscoveryXml => write ! (f , "application/vnd.oipf.spdiscovery+xml") ? , Application :: VndOipfSpdlistXml => write ! (f , "application/vnd.oipf.spdlist+xml") ? , Application :: VndOipfUeprofileXml => write ! (f , "application/vnd.oipf.ueprofile+xml") ? , Application :: VndOipfUserprofileXml => write ! (f , "application/vnd.oipf.userprofile+xml") ? , Application :: VndOlpcSugar => write ! (f , "application/vnd.olpc-sugar") ? , Application :: VndOmaBcastAssociatedProcedureParameterXml => write ! (f , "application/vnd.oma.bcast.associated-procedure-parameter+xml") ? , Application :: VndOmaBcastDrmTriggerXml => write ! (f , "application/vnd.oma.bcast.drm-trigger+xml") ? , Application :: VndOmaBcastImdXml => write ! (f , "application/vnd.oma.bcast.imd+xml") ? , Application :: VndOmaBcastLtkm => write ! (f , "application/vnd.oma.bcast.ltkm") ? , Application :: VndOmaBcastNotificationXml => write ! (f , "application/vnd.oma.bcast.notification+xml") ? , Application :: VndOmaBcastProvisioningtrigger => write ! (f , "application/vnd.oma.bcast.provisioningtrigger") ? , Application :: VndOmaBcastSgboot => write ! (f , "application/vnd.oma.bcast.sgboot") ? , Application :: VndOmaBcastSgddXml => write ! (f , "application/vnd.oma.bcast.sgdd+xml") ? , Application :: VndOmaBcastSgdu => write ! (f , "application/vnd.oma.bcast.sgdu") ? , Application :: VndOmaBcastSimpleSymbolContainer => write ! (f , "application/vnd.oma.bcast.simple-symbol-container") ? , Application :: VndOmaBcastSmartcardTriggerXml => write ! (f , "application/vnd.oma.bcast.smartcard-trigger+xml") ? , Application :: VndOmaBcastSprovXml => write ! (f , "application/vnd.oma.bcast.sprov+xml") ? , Application :: VndOmaBcastStkm => write ! (f , "application/vnd.oma.bcast.stkm") ? , Application :: VndOmaCabAddressBookXml => write ! (f , "application/vnd.oma.cab-address-book+xml") ? , Application :: VndOmaCabFeatureHandlerXml => write ! (f , "application/vnd.oma.cab-feature-handler+xml") ? , Application :: VndOmaCabPccXml => write ! (f , "application/vnd.oma.cab-pcc+xml") ? , Application :: VndOmaCabSubsInviteXml => write ! (f , "application/vnd.oma.cab-subs-invite+xml") ? , Application :: VndOmaCabUserPrefsXml => write ! (f , "application/vnd.oma.cab-user-prefs+xml") ? , Application :: VndOmaDcd => write ! (f , "application/vnd.oma.dcd") ? , Application :: VndOmaDcdc => write ! (f , "application/vnd.oma.dcdc") ? , Application :: VndOmaDd2Xml => write ! (f , "application/vnd.oma.dd2+xml") ? , Application :: VndOmaDrmRisdXml => write ! (f , "application/vnd.oma.drm.risd+xml") ? , Application :: VndOmaGroupUsageListXml => write ! (f , "application/vnd.oma.group-usage-list+xml") ? , Application :: VndOmaLwm2MCbor => write ! (f , "application/vnd.oma.lwm2m+cbor") ? , Application :: VndOmaLwm2MJson => write ! (f , "application/vnd.oma.lwm2m+json") ? , Application :: VndOmaLwm2MTlv => write ! (f , "application/vnd.oma.lwm2m+tlv") ? , Application :: VndOmaPalXml => write ! (f , "application/vnd.oma.pal+xml") ? , Application :: VndOmaPocDetailedProgressReportXml => write ! (f , "application/vnd.oma.poc.detailed-progress-report+xml") ? , Application :: VndOmaPocFinalReportXml => write ! (f , "application/vnd.oma.poc.final-report+xml") ? , Application :: VndOmaPocGroupsXml => write ! (f , "application/vnd.oma.poc.groups+xml") ? , Application :: VndOmaPocInvocationDescriptorXml => write ! (f , "application/vnd.oma.poc.invocation-descriptor+xml") ? , Application :: VndOmaPocOptimizedProgressReportXml => write ! (f , "application/vnd.oma.poc.optimized-progress-report+xml") ? , Application :: VndOmaPush => write ! (f , "application/vnd.oma.push") ? , Application :: VndOmaScidmMessagesXml => write ! (f , "application/vnd.oma.scidm.messages+xml") ? , Application :: VndOmaXcapDirectoryXml => write ! (f , "application/vnd.oma.xcap-directory+xml") ? , Application :: VndOmadsEmailXml => write ! (f , "application/vnd.omads-email+xml") ? , Application :: VndOmadsFileXml => write ! (f , "application/vnd.omads-file+xml") ? , Application :: VndOmadsFolderXml => write ! (f , "application/vnd.omads-folder+xml") ? , Application :: VndOmalocSuplInit => write ! (f , "application/vnd.omaloc-supl-init") ? , Application :: VndOmaScwsConfig => write ! (f , "application/vnd.oma-scws-config") ? , Application :: VndOmaScwsHttpRequest => write ! (f , "application/vnd.oma-scws-http-request") ? , Application :: VndOmaScwsHttpResponse => write ! (f , "application/vnd.oma-scws-http-response") ? , Application :: VndOnepager => write ! (f , "application/vnd.onepager") ? , Application :: VndOnepagertamp => write ! (f , "application/vnd.onepagertamp") ? , Application :: VndOnepagertamx => write ! (f , "application/vnd.onepagertamx") ? , Application :: VndOnepagertat => write ! (f , "application/vnd.onepagertat") ? , Application :: VndOnepagertatp => write ! (f , "application/vnd.onepagertatp") ? , Application :: VndOnepagertatx => write ! (f , "application/vnd.onepagertatx") ? , Application :: VndOnvifMetadata => write ! (f , "application/vnd.onvif.metadata") ? , Application :: VndOpenbloxGameBinary => write ! (f , "application/vnd.openblox.game-binary") ? , Application :: VndOpenbloxGameXml => write ! (f , "application/vnd.openblox.game+xml") ? , Application :: VndOpeneyeOeb => write ! (f , "application/vnd.openeye.oeb") ? , Application :: VndOpenstreetmapDataXml => write ! (f , "application/vnd.openstreetmap.data+xml") ? , Application :: VndOpentimestampsOts => write ! (f , "application/vnd.opentimestamps.ots") ? , Application :: VndOpenxmlformatsOfficedocumentCustomPropertiesXml => write ! (f , "application/vnd.openxmlformats-officedocument.custom-properties+xml") ? , Application :: VndOpenxmlformatsOfficedocumentCustomXmlPropertiesXml => write ! (f , "application/vnd.openxmlformats-officedocument.customXmlProperties+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawing+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.chart+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartshapesXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramColorsXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramDataXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramLayoutXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramStyleXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml") ? , Application :: VndOpenxmlformatsOfficedocumentExtendedPropertiesXml => write ! (f , "application/vnd.openxmlformats-officedocument.extended-properties+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentAuthorsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.comments+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlHandoutMasterXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesMasterXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesSlideXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentation => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.presentation") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentationMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresPropsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlide => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slide") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slide+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideLayoutXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideMasterXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshow => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideshow") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshowMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideUpdateInfoXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTableStylesXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTagsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.tags+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplate => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.template") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplateMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlViewPropsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCalcChainXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlChartsheetXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCommentsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlConnectionsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlDialogsheetXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlExternalLinkXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheDefinitionXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheRecordsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlQueryTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionHeadersXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionLogXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSharedStringsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheet => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMetadataXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlStylesXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableSingleCellsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplate => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.template") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplateMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlUserNamesXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlVolatileDependenciesXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlWorksheetXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml") ? , Application :: VndOpenxmlformatsOfficedocumentThemeXml => write ! (f , "application/vnd.openxmlformats-officedocument.theme+xml") ? , Application :: VndOpenxmlformatsOfficedocumentThemeOverrideXml => write ! (f , "application/vnd.openxmlformats-officedocument.themeOverride+xml") ? , Application :: VndOpenxmlformatsOfficedocumentVmlDrawing => write ! (f , "application/vnd.openxmlformats-officedocument.vmlDrawing") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlCommentsXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocument => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.document") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentGlossaryXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlEndnotesXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFontTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFooterXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFootnotesXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlNumberingXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlSettingsXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlStylesXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplate => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.template") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplateMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlWebSettingsXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml") ? , Application :: VndOpenxmlformatsPackageCorePropertiesXml => write ! (f , "application/vnd.openxmlformats-package.core-properties+xml") ? , Application :: VndOpenxmlformatsPackageDigitalSignatureXmlsignatureXml => write ! (f , "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml") ? , Application :: VndOpenxmlformatsPackageRelationshipsXml => write ! (f , "application/vnd.openxmlformats-package.relationships+xml") ? , Application :: VndOracleResourceJson => write ! (f , "application/vnd.oracle.resource+json") ? , Application :: VndOrangeIndata => write ! (f , "application/vnd.orange.indata") ? , Application :: VndOsaNetdeploy => write ! (f , "application/vnd.osa.netdeploy") ? , Application :: VndOsgeoMapguidePackage => write ! (f , "application/vnd.osgeo.mapguide.package") ? , Application :: VndOsgiBundle => write ! (f , "application/vnd.osgi.bundle") ? , Application :: VndOsgiDp => write ! (f , "application/vnd.osgi.dp") ? , Application :: VndOsgiSubsystem => write ! (f , "application/vnd.osgi.subsystem") ? , Application :: VndOtpsCtKipXml => write ! (f , "application/vnd.otps.ct-kip+xml") ? , Application :: VndOxliCountgraph => write ! (f , "application/vnd.oxli.countgraph") ? , Application :: VndPagerdutyJson => write ! (f , "application/vnd.pagerduty+json") ? , Application :: VndPalm => write ! (f , "application/vnd.palm") ? , Application :: VndPanoply => write ! (f , "application/vnd.panoply") ? , Application :: VndPaosXml => write ! (f , "application/vnd.paos.xml") ? , Application :: VndPatentdive => write ! (f , "application/vnd.patentdive") ? , Application :: VndPatientecommsdoc => write ! (f , "application/vnd.patientecommsdoc") ? , Application :: VndPawaafile => write ! (f , "application/vnd.pawaafile") ? , Application :: VndPcos => write ! (f , "application/vnd.pcos") ? , Application :: VndPgFormat => write ! (f , "application/vnd.pg.format") ? , Application :: VndPgOsasli => write ! (f , "application/vnd.pg.osasli") ? , Application :: VndPiaccessApplicationLicence => write ! (f , "application/vnd.piaccess.application-licence") ? , Application :: VndPicsel => write ! (f , "application/vnd.picsel") ? , Application :: VndPmiWidget => write ! (f , "application/vnd.pmi.widget") ? , Application :: VndPocGroupAdvertisementXml => write ! (f , "application/vnd.poc.group-advertisement+xml") ? , Application :: VndPocketlearn => write ! (f , "application/vnd.pocketlearn") ? , Application :: VndPowerbuilder6 => write ! (f , "application/vnd.powerbuilder6") ? , Application :: VndPowerbuilder6S => write ! (f , "application/vnd.powerbuilder6-s") ? , Application :: VndPowerbuilder7 => write ! (f , "application/vnd.powerbuilder7") ? , Application :: VndPowerbuilder75 => write ! (f , "application/vnd.powerbuilder75") ? , Application :: VndPowerbuilder75S => write ! (f , "application/vnd.powerbuilder75-s") ? , Application :: VndPowerbuilder7S => write ! (f , "application/vnd.powerbuilder7-s") ? , Application :: VndPreminet => write ! (f , "application/vnd.preminet") ? , Application :: VndPreviewsystemsBox => write ! (f , "application/vnd.previewsystems.box") ? , Application :: VndProteusMagazine => write ! (f , "application/vnd.proteus.magazine") ? , Application :: VndPsfs => write ! (f , "application/vnd.psfs") ? , Application :: VndPtMundusmundi => write ! (f , "application/vnd.pt.mundusmundi") ? , Application :: VndPublishareDeltaTree => write ! (f , "application/vnd.publishare-delta-tree") ? , Application :: VndPviPtid1 => write ! (f , "application/vnd.pvi.ptid1") ? , Application :: VndPwgMultiplexed => write ! (f , "application/vnd.pwg-multiplexed") ? , Application :: VndPwgXhtmlPrintXml => write ! (f , "application/vnd.pwg-xhtml-print+xml") ? , Application :: VndQualcommBrewAppRes => write ! (f , "application/vnd.qualcomm.brew-app-res") ? , Application :: VndQuarantainenet => write ! (f , "application/vnd.quarantainenet") ? , Application :: VndQuarkQuarkXPress => write ! (f , "application/vnd.Quark.QuarkXPress") ? , Application :: VndQuobjectQuoxdocument => write ! (f , "application/vnd.quobject-quoxdocument") ? , Application :: VndRadisysMomlXml => write ! (f , "application/vnd.radisys.moml+xml") ? , Application :: VndRadisysMsmlAuditConfXml => write ! (f , "application/vnd.radisys.msml-audit-conf+xml") ? , Application :: VndRadisysMsmlAuditConnXml => write ! (f , "application/vnd.radisys.msml-audit-conn+xml") ? , Application :: VndRadisysMsmlAuditDialogXml => write ! (f , "application/vnd.radisys.msml-audit-dialog+xml") ? , Application :: VndRadisysMsmlAuditStreamXml => write ! (f , "application/vnd.radisys.msml-audit-stream+xml") ? , Application :: VndRadisysMsmlAuditXml => write ! (f , "application/vnd.radisys.msml-audit+xml") ? , Application :: VndRadisysMsmlConfXml => write ! (f , "application/vnd.radisys.msml-conf+xml") ? , Application :: VndRadisysMsmlDialogBaseXml => write ! (f , "application/vnd.radisys.msml-dialog-base+xml") ? , Application :: VndRadisysMsmlDialogFaxDetectXml => write ! (f , "application/vnd.radisys.msml-dialog-fax-detect+xml") ? , Application :: VndRadisysMsmlDialogFaxSendrecvXml => write ! (f , "application/vnd.radisys.msml-dialog-fax-sendrecv+xml") ? , Application :: VndRadisysMsmlDialogGroupXml => write ! (f , "application/vnd.radisys.msml-dialog-group+xml") ? , Application :: VndRadisysMsmlDialogSpeechXml => write ! (f , "application/vnd.radisys.msml-dialog-speech+xml") ? , Application :: VndRadisysMsmlDialogTransformXml => write ! (f , "application/vnd.radisys.msml-dialog-transform+xml") ? , Application :: VndRadisysMsmlDialogXml => write ! (f , "application/vnd.radisys.msml-dialog+xml") ? , Application :: VndRadisysMsmlXml => write ! (f , "application/vnd.radisys.msml+xml") ? , Application :: VndRainstorData => write ! (f , "application/vnd.rainstor.data") ? , Application :: VndRapid => write ! (f , "application/vnd.rapid") ? , Application :: VndRar => write ! (f , "application/vnd.rar") ? , Application :: VndRealvncBed => write ! (f , "application/vnd.realvnc.bed") ? , Application :: VndRecordareMusicxml => write ! (f , "application/vnd.recordare.musicxml") ? , Application :: VndRecordareMusicxmlXml => write ! (f , "application/vnd.recordare.musicxml+xml") ? , Application :: VndRenLearnRlprint => write ! (f , "application/vnd.RenLearn.rlprint") ? , Application :: VndResilientLogic => write ! (f , "application/vnd.resilient.logic") ? , Application :: VndRestfulJson => write ! (f , "application/vnd.restful+json") ? , Application :: VndRigCryptonote => write ! (f , "application/vnd.rig.cryptonote") ? , Application :: VndRoute66Link66Xml => write ! (f , "application/vnd.route66.link66+xml") ? , Application :: VndRs274X => write ! (f , "application/vnd.rs-274x") ? , Application :: VndRuckusDownload => write ! (f , "application/vnd.ruckus.download") ? , Application :: VndS3Sms => write ! (f , "application/vnd.s3sms") ? , Application :: VndSailingtrackerTrack => write ! (f , "application/vnd.sailingtracker.track") ? , Application :: VndSar => write ! (f , "application/vnd.sar") ? , Application :: VndSbmCid => write ! (f , "application/vnd.sbm.cid") ? , Application :: VndSbmMid2 => write ! (f , "application/vnd.sbm.mid2") ? , Application :: VndScribus => write ! (f , "application/vnd.scribus") ? , Application :: VndSealed3Df => write ! (f , "application/vnd.sealed.3df") ? , Application :: VndSealedCsf => write ! (f , "application/vnd.sealed.csf") ? , Application :: VndSealedDoc => write ! (f , "application/vnd.sealed.doc") ? , Application :: VndSealedEml => write ! (f , "application/vnd.sealed.eml") ? , Application :: VndSealedMht => write ! (f , "application/vnd.sealed.mht") ? , Application :: VndSealedNet => write ! (f , "application/vnd.sealed.net") ? , Application :: VndSealedPpt => write ! (f , "application/vnd.sealed.ppt") ? , Application :: VndSealedTiff => write ! (f , "application/vnd.sealed.tiff") ? , Application :: VndSealedXls => write ! (f , "application/vnd.sealed.xls") ? , Application :: VndSealedmediaSoftsealHtml => write ! (f , "application/vnd.sealedmedia.softseal.html") ? , Application :: VndSealedmediaSoftsealPdf => write ! (f , "application/vnd.sealedmedia.softseal.pdf") ? , Application :: VndSeemail => write ! (f , "application/vnd.seemail") ? , Application :: VndSeisJson => write ! (f , "application/vnd.seis+json") ? , Application :: VndSema => write ! (f , "application/vnd.sema") ? , Application :: VndSemd => write ! (f , "application/vnd.semd") ? , Application :: VndSemf => write ! (f , "application/vnd.semf") ? , Application :: VndShadeSaveFile => write ! (f , "application/vnd.shade-save-file") ? , Application :: VndShanaInformedFormdata => write ! (f , "application/vnd.shana.informed.formdata") ? , Application :: VndShanaInformedFormtemplate => write ! (f , "application/vnd.shana.informed.formtemplate") ? , Application :: VndShanaInformedInterchange => write ! (f , "application/vnd.shana.informed.interchange") ? , Application :: VndShanaInformedPackage => write ! (f , "application/vnd.shana.informed.package") ? , Application :: VndShootproofJson => write ! (f , "application/vnd.shootproof+json") ? , Application :: VndShopkickJson => write ! (f , "application/vnd.shopkick+json") ? , Application :: VndShp => write ! (f , "application/vnd.shp") ? , Application :: VndShx => write ! (f , "application/vnd.shx") ? , Application :: VndSigrokSession => write ! (f , "application/vnd.sigrok.session") ? , Application :: VndSimTechMindMapper => write ! (f , "application/vnd.SimTech-MindMapper") ? , Application :: VndSirenJson => write ! (f , "application/vnd.siren+json") ? , Application :: VndSmaf => write ! (f , "application/vnd.smaf") ? , Application :: VndSmartNotebook => write ! (f , "application/vnd.smart.notebook") ? , Application :: VndSmartTeacher => write ! (f , "application/vnd.smart.teacher") ? , Application :: VndSmintioPortalsArchive => write ! (f , "application/vnd.smintio.portals.archive") ? , Application :: VndSnesdevPageTable => write ! (f , "application/vnd.snesdev-page-table") ? , Application :: VndSoftware602FillerFormXml => write ! (f , "application/vnd.software602.filler.form+xml") ? , Application :: VndSoftware602FillerFormXmlZip => write ! (f , "application/vnd.software602.filler.form-xml-zip") ? , Application :: VndSolentSdkmXml => write ! (f , "application/vnd.solent.sdkm+xml") ? , Application :: VndSpotfireDxp => write ! (f , "application/vnd.spotfire.dxp") ? , Application :: VndSpotfireSfs => write ! (f , "application/vnd.spotfire.sfs") ? , Application :: VndSqlite3 => write ! (f , "application/vnd.sqlite3") ? , Application :: VndSssCod => write ! (f , "application/vnd.sss-cod") ? , Application :: VndSssDtf => write ! (f , "application/vnd.sss-dtf") ? , Application :: VndSssNtf => write ! (f , "application/vnd.sss-ntf") ? , Application :: VndStepmaniaPackage => write ! (f , "application/vnd.stepmania.package") ? , Application :: VndStepmaniaStepchart => write ! (f , "application/vnd.stepmania.stepchart") ? , Application :: VndStreetStream => write ! (f , "application/vnd.street-stream") ? , Application :: VndSunWadlXml => write ! (f , "application/vnd.sun.wadl+xml") ? , Application :: VndSusCalendar => write ! (f , "application/vnd.sus-calendar") ? , Application :: VndSvd => write ! (f , "application/vnd.svd") ? , Application :: VndSwiftviewIcs => write ! (f , "application/vnd.swiftview-ics") ? , Application :: VndSybylMol2 => write ! (f , "application/vnd.sybyl.mol2") ? , Application :: VndSycleXml => write ! (f , "application/vnd.sycle+xml") ? , Application :: VndSyftJson => write ! (f , "application/vnd.syft+json") ? , Application :: VndSyncmlDmNotification => write ! (f , "application/vnd.syncml.dm.notification") ? , Application :: VndSyncmlDmddfXml => write ! (f , "application/vnd.syncml.dmddf+xml") ? , Application :: VndSyncmlDmtndsWbxml => write ! (f , "application/vnd.syncml.dmtnds+wbxml") ? , Application :: VndSyncmlDmtndsXml => write ! (f , "application/vnd.syncml.dmtnds+xml") ? , Application :: VndSyncmlDmddfWbxml => write ! (f , "application/vnd.syncml.dmddf+wbxml") ? , Application :: VndSyncmlDmWbxml => write ! (f , "application/vnd.syncml.dm+wbxml") ? , Application :: VndSyncmlDmXml => write ! (f , "application/vnd.syncml.dm+xml") ? , Application :: VndSyncmlDsNotification => write ! (f , "application/vnd.syncml.ds.notification") ? , Application :: VndSyncmlXml => write ! (f , "application/vnd.syncml+xml") ? , Application :: VndTableschemaJson => write ! (f , "application/vnd.tableschema+json") ? , Application :: VndTaoIntentModuleArchive => write ! (f , "application/vnd.tao.intent-module-archive") ? , Application :: VndTcpdumpPcap => write ! (f , "application/vnd.tcpdump.pcap") ? , Application :: VndThinkCellPpttcJson => write ! (f , "application/vnd.think-cell.ppttc+json") ? , Application :: VndTml => write ! (f , "application/vnd.tml") ? , Application :: VndTmdMediaflexApiXml => write ! (f , "application/vnd.tmd.mediaflex.api+xml") ? , Application :: VndTmobileLivetv => write ! (f , "application/vnd.tmobile-livetv") ? , Application :: VndTriOnesource => write ! (f , "application/vnd.tri.onesource") ? , Application :: VndTridTpt => write ! (f , "application/vnd.trid.tpt") ? , Application :: VndTriscapeMxs => write ! (f , "application/vnd.triscape.mxs") ? , Application :: VndTrueapp => write ! (f , "application/vnd.trueapp") ? , Application :: VndTruedoc => write ! (f , "application/vnd.truedoc") ? , Application :: VndUbisoftWebplayer => write ! (f , "application/vnd.ubisoft.webplayer") ? , Application :: VndUfdl => write ! (f , "application/vnd.ufdl") ? , Application :: VndUiqTheme => write ! (f , "application/vnd.uiq.theme") ? , Application :: VndUmajin => write ! (f , "application/vnd.umajin") ? , Application :: VndUnity => write ! (f , "application/vnd.unity") ? , Application :: VndUomlXml => write ! (f , "application/vnd.uoml+xml") ? , Application :: VndUplanetAlert => write ! (f , "application/vnd.uplanet.alert") ? , Application :: VndUplanetAlertWbxml => write ! (f , "application/vnd.uplanet.alert-wbxml") ? , Application :: VndUplanetBearerChoice => write ! (f , "application/vnd.uplanet.bearer-choice") ? , Application :: VndUplanetBearerChoiceWbxml => write ! (f , "application/vnd.uplanet.bearer-choice-wbxml") ? , Application :: VndUplanetCacheop => write ! (f , "application/vnd.uplanet.cacheop") ? , Application :: VndUplanetCacheopWbxml => write ! (f , "application/vnd.uplanet.cacheop-wbxml") ? , Application :: VndUplanetChannel => write ! (f , "application/vnd.uplanet.channel") ? , Application :: VndUplanetChannelWbxml => write ! (f , "application/vnd.uplanet.channel-wbxml") ? , Application :: VndUplanetList => write ! (f , "application/vnd.uplanet.list") ? , Application :: VndUplanetListcmd => write ! (f , "application/vnd.uplanet.listcmd") ? , Application :: VndUplanetListcmdWbxml => write ! (f , "application/vnd.uplanet.listcmd-wbxml") ? , Application :: VndUplanetListWbxml => write ! (f , "application/vnd.uplanet.list-wbxml") ? , Application :: VndUriMap => write ! (f , "application/vnd.uri-map") ? , Application :: VndUplanetSignal => write ! (f , "application/vnd.uplanet.signal") ? , Application :: VndValveSourceMaterial => write ! (f , "application/vnd.valve.source.material") ? , Application :: VndVcx => write ! (f , "application/vnd.vcx") ? , Application :: VndVdStudy => write ! (f , "application/vnd.vd-study") ? , Application :: VndVectorworks => write ! (f , "application/vnd.vectorworks") ? , Application :: VndVelJson => write ! (f , "application/vnd.vel+json") ? , Application :: VndVerimatrixVcas => write ! (f , "application/vnd.verimatrix.vcas") ? , Application :: VndVeritoneAionJson => write ! (f , "application/vnd.veritone.aion+json") ? , Application :: VndVeryantThin => write ! (f , "application/vnd.veryant.thin") ? , Application :: VndVesEncrypted => write ! (f , "application/vnd.ves.encrypted") ? , Application :: VndVidsoftVidconference => write ! (f , "application/vnd.vidsoft.vidconference") ? , Application :: VndVisio => write ! (f , "application/vnd.visio") ? , Application :: VndVisionary => write ! (f , "application/vnd.visionary") ? , Application :: VndVividenceScriptfile => write ! (f , "application/vnd.vividence.scriptfile") ? , Application :: VndVsf => write ! (f , "application/vnd.vsf") ? , Application :: VndWapSic => write ! (f , "application/vnd.wap.sic") ? , Application :: VndWapSlc => write ! (f , "application/vnd.wap.slc") ? , Application :: VndWapWbxml => write ! (f , "application/vnd.wap.wbxml") ? , Application :: VndWapWmlc => write ! (f , "application/vnd.wap.wmlc") ? , Application :: VndWapWmlscriptc => write ! (f , "application/vnd.wap.wmlscriptc") ? , Application :: VndWasmflowWafl => write ! (f , "application/vnd.wasmflow.wafl") ? , Application :: VndWebturbo => write ! (f , "application/vnd.webturbo") ? , Application :: VndWfaDpp => write ! (f , "application/vnd.wfa.dpp") ? , Application :: VndWfaP2P => write ! (f , "application/vnd.wfa.p2p") ? , Application :: VndWfaWsc => write ! (f , "application/vnd.wfa.wsc") ? , Application :: VndWindowsDevicepairing => write ! (f , "application/vnd.windows.devicepairing") ? , Application :: VndWmc => write ! (f , "application/vnd.wmc") ? , Application :: VndWmfBootstrap => write ! (f , "application/vnd.wmf.bootstrap") ? , Application :: VndWolframMathematica => write ! (f , "application/vnd.wolfram.mathematica") ? , Application :: VndWolframMathematicaPackage => write ! (f , "application/vnd.wolfram.mathematica.package") ? , Application :: VndWolframPlayer => write ! (f , "application/vnd.wolfram.player") ? , Application :: VndWordlift => write ! (f , "application/vnd.wordlift") ? , Application :: VndWordperfect => write ! (f , "application/vnd.wordperfect") ? , Application :: VndWqd => write ! (f , "application/vnd.wqd") ? , Application :: VndWrqHp3000Labelled => write ! (f , "application/vnd.wrq-hp3000-labelled") ? , Application :: VndWtStf => write ! (f , "application/vnd.wt.stf") ? , Application :: VndWvCspXml => write ! (f , "application/vnd.wv.csp+xml") ? , Application :: VndWvCspWbxml => write ! (f , "application/vnd.wv.csp+wbxml") ? , Application :: VndWvSspXml => write ! (f , "application/vnd.wv.ssp+xml") ? , Application :: VndXacmlJson => write ! (f , "application/vnd.xacml+json") ? , Application :: VndXara => write ! (f , "application/vnd.xara") ? , Application :: VndXfdl => write ! (f , "application/vnd.xfdl") ? , Application :: VndXfdlWebform => write ! (f , "application/vnd.xfdl.webform") ? , Application :: VndXmiXml => write ! (f , "application/vnd.xmi+xml") ? , Application :: VndXmpieCpkg => write ! (f , "application/vnd.xmpie.cpkg") ? , Application :: VndXmpieDpkg => write ! (f , "application/vnd.xmpie.dpkg") ? , Application :: VndXmpiePlan => write ! (f , "application/vnd.xmpie.plan") ? , Application :: VndXmpiePpkg => write ! (f , "application/vnd.xmpie.ppkg") ? , Application :: VndXmpieXlim => write ! (f , "application/vnd.xmpie.xlim") ? , Application :: VndYamahaHvDic => write ! (f , "application/vnd.yamaha.hv-dic") ? , Application :: VndYamahaHvScript => write ! (f , "application/vnd.yamaha.hv-script") ? , Application :: VndYamahaHvVoice => write ! (f , "application/vnd.yamaha.hv-voice") ? , Application :: VndYamahaOpenscoreformatOsfpvgXml => write ! (f , "application/vnd.yamaha.openscoreformat.osfpvg+xml") ? , Application :: VndYamahaOpenscoreformat => write ! (f , "application/vnd.yamaha.openscoreformat") ? , Application :: VndYamahaRemoteSetup => write ! (f , "application/vnd.yamaha.remote-setup") ? , Application :: VndYamahaSmafAudio => write ! (f , "application/vnd.yamaha.smaf-audio") ? , Application :: VndYamahaSmafPhrase => write ! (f , "application/vnd.yamaha.smaf-phrase") ? , Application :: VndYamahaThroughNgn => write ! (f , "application/vnd.yamaha.through-ngn") ? , Application :: VndYamahaTunnelUdpencap => write ! (f , "application/vnd.yamaha.tunnel-udpencap") ? , Application :: VndYaoweme => write ! (f , "application/vnd.yaoweme") ? , Application :: VndYellowriverCustomMenu => write ! (f , "application/vnd.yellowriver-custom-menu") ? , Application :: VndZul => write ! (f , "application/vnd.zul") ? , Application :: VndZzazzDeckXml => write ! (f , "application/vnd.zzazz.deck+xml") ? , Application :: VoicexmlXml => write ! (f , "application/voicexml+xml") ? , Application :: VoucherCmsJson => write ! (f , "application/voucher-cms+json") ? , Application :: VqRtcpxr => write ! (f , "application/vq-rtcpxr") ? , Application :: Wasm => write ! (f , "application/wasm") ? , Application :: WatcherinfoXml => write ! (f , "application/watcherinfo+xml") ? , Application :: WebpushOptionsJson => write ! (f , "application/webpush-options+json") ? , Application :: WhoisppQuery => write ! (f , "application/whoispp-query") ? , Application :: WhoisppResponse => write ! (f , "application/whoispp-response") ? , Application :: Widget => write ! (f , "application/widget") ? , Application :: Wita => write ! (f , "application/wita") ? , Application :: Wordperfect51 => write ! (f , "application/wordperfect5.1") ? , Application :: WsdlXml => write ! (f , "application/wsdl+xml") ? , Application :: WspolicyXml => write ! (f , "application/wspolicy+xml") ? , Application :: XPkiMessage => write ! (f , "application/x-pki-message") ? , Application :: XWwwFormUrlencoded => write ! (f , "application/x-www-form-urlencoded") ? , Application :: XX509CaCert => write ! (f , "application/x-x509-ca-cert") ? , Application :: XX509CaRaCert => write ! (f , "application/x-x509-ca-ra-cert") ? , Application :: XX509NextCaCert => write ! (f , "application/x-x509-next-ca-cert") ? , Application :: X400Bp => write ! (f , "application/x400-bp") ? , Application :: XacmlXml => write ! (f , "application/xacml+xml") ? , Application :: XcapAttXml => write ! (f , "application/xcap-att+xml") ? , Application :: XcapCapsXml => write ! (f , "application/xcap-caps+xml") ? , Application :: XcapDiffXml => write ! (f , "application/xcap-diff+xml") ? , Application :: XcapElXml => write ! (f , "application/xcap-el+xml") ? , Application :: XcapErrorXml => write ! (f , "application/xcap-error+xml") ? , Application :: XcapNsXml => write ! (f , "application/xcap-ns+xml") ? , Application :: XconConferenceInfoDiffXml => write ! (f , "application/xcon-conference-info-diff+xml") ? , Application :: XconConferenceInfoXml => write ! (f , "application/xcon-conference-info+xml") ? , Application :: XencXml => write ! (f , "application/xenc+xml") ? , Application :: Xfdf => write ! (f , "application/xfdf") ? , Application :: XhtmlXml => write ! (f , "application/xhtml+xml") ? , Application :: XliffXml => write ! (f , "application/xliff+xml") ? , Application :: Xml => write ! (f , "application/xml") ? , Application :: XmlDtd => write ! (f , "application/xml-dtd") ? , Application :: XmlExternalParsedEntity => write ! (f , "application/xml-external-parsed-entity") ? , Application :: XmlPatchXml => write ! (f , "application/xml-patch+xml") ? , Application :: XmppXml => write ! (f , "application/xmpp+xml") ? , Application :: XopXml => write ! (f , "application/xop+xml") ? , Application :: XsltXml => write ! (f , "application/xslt+xml") ? , Application :: XvXml => write ! (f , "application/xv+xml") ? , Application :: Yaml => write ! (f , "application/yaml") ? , Application :: Yang => write ! (f , "application/yang") ? , Application :: YangDataCbor => write ! (f , "application/yang-data+cbor") ? , Application :: YangDataJson => write ! (f , "application/yang-data+json") ? , Application :: YangDataXml => write ! (f , "application/yang-data+xml") ? , Application :: YangPatchJson => write ! (f , "application/yang-patch+json") ? , Application :: YangPatchXml => write ! (f , "application/yang-patch+xml") ? , Application :: YinXml => write ! (f , "application/yin+xml") ? , Application :: Zip => write ! (f , "application/zip") ? , Application :: Zlib => write ! (f , "application/zlib") ? , Application :: Zstd => write ! (f , "application/zstd") ? , }
        Ok(())
    }
}
impl ::std::str::FromStr for Application {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input { "application/1d-interleaved-parityfec" => Ok (Application :: _1DInterleavedParityfec) , "application/3gpdash-qoe-report+xml" => Ok (Application :: _3GpdashQoeReportXml) , "application/3gppHal+json" => Ok (Application :: _3GppHalJson) , "application/3gppHalForms+json" => Ok (Application :: _3GppHalFormsJson) , "application/3gpp-ims+xml" => Ok (Application :: _3GppImsXml) , "application/A2L" | "a2l" => Ok (Application :: A2L) , "application/ace+cbor" => Ok (Application :: AceCbor) , "application/ace+json" => Ok (Application :: AceJson) , "application/activemessage" => Ok (Application :: Activemessage) , "application/activity+json" => Ok (Application :: ActivityJson) , "application/aif+cbor" => Ok (Application :: AifCbor) , "application/aif+json" => Ok (Application :: AifJson) , "application/alto-cdni+json" => Ok (Application :: AltoCdniJson) , "application/alto-cdnifilter+json" => Ok (Application :: AltoCdnifilterJson) , "application/alto-costmap+json" => Ok (Application :: AltoCostmapJson) , "application/alto-costmapfilter+json" => Ok (Application :: AltoCostmapfilterJson) , "application/alto-directory+json" => Ok (Application :: AltoDirectoryJson) , "application/alto-endpointprop+json" => Ok (Application :: AltoEndpointpropJson) , "application/alto-endpointpropparams+json" => Ok (Application :: AltoEndpointpropparamsJson) , "application/alto-endpointcost+json" => Ok (Application :: AltoEndpointcostJson) , "application/alto-endpointcostparams+json" => Ok (Application :: AltoEndpointcostparamsJson) , "application/alto-error+json" => Ok (Application :: AltoErrorJson) , "application/alto-networkmapfilter+json" => Ok (Application :: AltoNetworkmapfilterJson) , "application/alto-networkmap+json" => Ok (Application :: AltoNetworkmapJson) , "application/alto-propmap+json" => Ok (Application :: AltoPropmapJson) , "application/alto-propmapparams+json" => Ok (Application :: AltoPropmapparamsJson) , "application/alto-updatestreamcontrol+json" => Ok (Application :: AltoUpdatestreamcontrolJson) , "application/alto-updatestreamparams+json" => Ok (Application :: AltoUpdatestreamparamsJson) , "application/AML" | "aml" => Ok (Application :: Aml) , "application/andrew-inset" | "ez" => Ok (Application :: AndrewInset) , "application/applefile" => Ok (Application :: Applefile) , "application/at+jwt" => Ok (Application :: AtJwt) , "application/ATF" | "atf" => Ok (Application :: Atf) , "application/ATFX" | "atfx" => Ok (Application :: Atfx) , "application/atom+xml" | "atom" => Ok (Application :: AtomXml) , "application/atomcat+xml" | "atomcat" => Ok (Application :: AtomcatXml) , "application/atomdeleted+xml" | "atomdeleted" => Ok (Application :: AtomdeletedXml) , "application/atomicmail" => Ok (Application :: Atomicmail) , "application/atomsvc+xml" | "atomsvc" => Ok (Application :: AtomsvcXml) , "application/atsc-dwd+xml" | "dwd" => Ok (Application :: AtscDwdXml) , "application/atsc-dynamic-event-message" => Ok (Application :: AtscDynamicEventMessage) , "application/atsc-held+xml" | "held" => Ok (Application :: AtscHeldXml) , "application/atsc-rdt+json" => Ok (Application :: AtscRdtJson) , "application/atsc-rsat+xml" | "rsat" => Ok (Application :: AtscRsatXml) , "application/ATXML" | "atxml" => Ok (Application :: Atxml) , "application/auth-policy+xml" | "apxml" => Ok (Application :: AuthPolicyXml) , "application/automationml-aml+xml" => Ok (Application :: AutomationmlAmlXml) , "application/automationml-amlx+zip" | "amlx" => Ok (Application :: AutomationmlAmlxZip) , "application/bacnet-xdd+zip" | "xdd" => Ok (Application :: BacnetXddZip) , "application/batch-SMTP" => Ok (Application :: BatchSMTP) , "application/beep+xml" => Ok (Application :: BeepXml) , "application/calendar+json" => Ok (Application :: CalendarJson) , "application/calendar+xml" | "xcs" => Ok (Application :: CalendarXml) , "application/call-completion" => Ok (Application :: CallCompletion) , "application/CALS-1840" => Ok (Application :: Cals1840) , "application/captive+json" => Ok (Application :: CaptiveJson) , "application/cbor" | "cbor" => Ok (Application :: Cbor) , "application/cbor-seq" => Ok (Application :: CborSeq) , "application/cccex" | "c3ex" => Ok (Application :: Cccex) , "application/ccmp+xml" | "ccmp" => Ok (Application :: CcmpXml) , "application/ccxml+xml" | "ccxml" => Ok (Application :: CcxmlXml) , "application/cda+xml" => Ok (Application :: CdaXml) , "application/CDFX+XML" | "cdfx" => Ok (Application :: CdfxXml) , "application/cdmi-capability" | "cdmia" => Ok (Application :: CdmiCapability) , "application/cdmi-container" | "cdmic" => Ok (Application :: CdmiContainer) , "application/cdmi-domain" | "cdmid" => Ok (Application :: CdmiDomain) , "application/cdmi-object" | "cdmio" => Ok (Application :: CdmiObject) , "application/cdmi-queue" | "cdmiq" => Ok (Application :: CdmiQueue) , "application/cdni" => Ok (Application :: Cdni) , "application/CEA" | "cea" => Ok (Application :: Cea) , "application/cea-2018+xml" => Ok (Application :: Cea2018Xml) , "application/cellml+xml" | "cellml" | "cml" => Ok (Application :: CellmlXml) , "application/cfw" => Ok (Application :: Cfw) , "application/city+json" => Ok (Application :: CityJson) , "application/clr" | "1clr" => Ok (Application :: Clr) , "application/clue_info+xml" | "clue" => Ok (Application :: ClueInfoXml) , "application/clue+xml" => Ok (Application :: ClueXml) , "application/cms" | "cmsc" => Ok (Application :: Cms) , "application/cnrp+xml" => Ok (Application :: CnrpXml) , "application/coap-group+json" => Ok (Application :: CoapGroupJson) , "application/coap-payload" => Ok (Application :: CoapPayload) , "application/commonground" => Ok (Application :: Commonground) , "application/concise-problem-details+cbor" => Ok (Application :: ConciseProblemDetailsCbor) , "application/conference-info+xml" => Ok (Application :: ConferenceInfoXml) , "application/cpl+xml" | "cpl" => Ok (Application :: CplXml) , "application/cose" => Ok (Application :: Cose) , "application/cose-key" => Ok (Application :: CoseKey) , "application/cose-key-set" => Ok (Application :: CoseKeySet) , "application/cose-x509" => Ok (Application :: CoseX509) , "application/csrattrs" | "csrattrs" => Ok (Application :: Csrattrs) , "application/csta+xml" => Ok (Application :: CstaXml) , "application/CSTAdata+xml" => Ok (Application :: CstadataXml) , "application/csvm+json" => Ok (Application :: CsvmJson) , "application/cwl" | "cwl" => Ok (Application :: Cwl) , "application/cwl+json" | "cwl.json" => Ok (Application :: CwlJson) , "application/cwt" => Ok (Application :: Cwt) , "application/cybercash" => Ok (Application :: Cybercash) , "application/dash+xml" | "mpd" => Ok (Application :: DashXml) , "application/dash-patch+xml" => Ok (Application :: DashPatchXml) , "application/dashdelta" | "mpdd" => Ok (Application :: Dashdelta) , "application/davmount+xml" | "davmount" => Ok (Application :: DavmountXml) , "application/dca-rft" => Ok (Application :: DcaRft) , "application/DCD" | "dcd" => Ok (Application :: Dcd) , "application/dec-dx" => Ok (Application :: DecDx) , "application/dialog-info+xml" => Ok (Application :: DialogInfoXml) , "application/dicom" | "dcm" => Ok (Application :: Dicom) , "application/dicom+json" => Ok (Application :: DicomJson) , "application/dicom+xml" => Ok (Application :: DicomXml) , "application/DII" | "dii" => Ok (Application :: Dii) , "application/DIT" | "dit" => Ok (Application :: Dit) , "application/dns" => Ok (Application :: Dns) , "application/dns+json" => Ok (Application :: DnsJson) , "application/dns-message" => Ok (Application :: DnsMessage) , "application/dots+cbor" => Ok (Application :: DotsCbor) , "application/dpop+jwt" => Ok (Application :: DpopJwt) , "application/dskpp+xml" | "xmls" => Ok (Application :: DskppXml) , "application/dssc+der" | "dssc" => Ok (Application :: DsscDer) , "application/dssc+xml" | "xdssc" => Ok (Application :: DsscXml) , "application/dvcs" | "dvc" => Ok (Application :: Dvcs) , "application/EDI-consent" => Ok (Application :: EdiConsent) , "application/EDIFACT" => Ok (Application :: Edifact) , "application/EDI-X12" => Ok (Application :: EdiX12) , "application/efi" | "efi" => Ok (Application :: Efi) , "application/elm+json" => Ok (Application :: ElmJson) , "application/elm+xml" => Ok (Application :: ElmXml) , "application/EmergencyCallData.cap+xml" => Ok (Application :: EmergencyCallDataCapXml) , "application/EmergencyCallData.Comment+xml" => Ok (Application :: EmergencyCallDataCommentXml) , "application/EmergencyCallData.Control+xml" => Ok (Application :: EmergencyCallDataControlXml) , "application/EmergencyCallData.DeviceInfo+xml" => Ok (Application :: EmergencyCallDataDeviceInfoXml) , "application/EmergencyCallData.eCall.MSD" => Ok (Application :: EmergencyCallDataECallMSD) , "application/EmergencyCallData.LegacyESN+json" => Ok (Application :: EmergencyCallDataLegacyESNJson) , "application/EmergencyCallData.ProviderInfo+xml" => Ok (Application :: EmergencyCallDataProviderInfoXml) , "application/EmergencyCallData.ServiceInfo+xml" => Ok (Application :: EmergencyCallDataServiceInfoXml) , "application/EmergencyCallData.SubscriberInfo+xml" => Ok (Application :: EmergencyCallDataSubscriberInfoXml) , "application/EmergencyCallData.VEDS+xml" => Ok (Application :: EmergencyCallDataVEDSXml) , "application/emma+xml" | "emma" => Ok (Application :: EmmaXml) , "application/emotionml+xml" | "emotionml" => Ok (Application :: EmotionmlXml) , "application/encaprtp" => Ok (Application :: Encaprtp) , "application/epp+xml" => Ok (Application :: EppXml) , "application/epub+zip" | "epub" => Ok (Application :: EpubZip) , "application/eshop" => Ok (Application :: Eshop) , "application/example" => Ok (Application :: Example) , "application/exi" | "exi" => Ok (Application :: Exi) , "application/expect-ct-report+json" => Ok (Application :: ExpectCtReportJson) , "application/express" | "exp" => Ok (Application :: Express) , "application/fastinfoset" | "finf" => Ok (Application :: Fastinfoset) , "application/fastsoap" => Ok (Application :: Fastsoap) , "application/fdf" => Ok (Application :: Fdf) , "application/fdt+xml" | "fdt" => Ok (Application :: FdtXml) , "application/fhir+json" => Ok (Application :: FhirJson) , "application/fhir+xml" => Ok (Application :: FhirXml) , "application/fits" => Ok (Application :: Fits) , "application/flexfec" => Ok (Application :: Flexfec) , "application/font-tdpfr" | "pfr" => Ok (Application :: FontTdpfr) , "application/framework-attributes+xml" => Ok (Application :: FrameworkAttributesXml) , "application/geo+json" | "geojson" => Ok (Application :: GeoJson) , "application/geo+json-seq" => Ok (Application :: GeoJsonSeq) , "application/geopackage+sqlite3" | "gpkg" => Ok (Application :: GeopackageSqlite3) , "application/geoxacml+xml" => Ok (Application :: GeoxacmlXml) , "application/gltf-buffer" | "glbin" | "glbuf" => Ok (Application :: GltfBuffer) , "application/gml+xml" | "gml" => Ok (Application :: GmlXml) , "application/gzip" | "gz" | "tgz" => Ok (Application :: Gzip) , "application/H224" => Ok (Application :: H224) , "application/held+xml" => Ok (Application :: HeldXml) , "application/hl7v2+xml" => Ok (Application :: Hl7V2Xml) , "application/http" => Ok (Application :: Http) , "application/hyperstudio" | "stk" => Ok (Application :: Hyperstudio) , "application/ibe-key-request+xml" => Ok (Application :: IbeKeyRequestXml) , "application/ibe-pkg-reply+xml" => Ok (Application :: IbePkgReplyXml) , "application/ibe-pp-data" => Ok (Application :: IbePpData) , "application/iges" => Ok (Application :: Iges) , "application/im-iscomposing+xml" => Ok (Application :: ImIscomposingXml) , "application/index" => Ok (Application :: Index) , "application/index.cmd" => Ok (Application :: IndexCmd) , "application/index.obj" => Ok (Application :: IndexObj) , "application/index.response" => Ok (Application :: IndexResponse) , "application/index.vnd" => Ok (Application :: IndexVnd) , "application/inkml+xml" | "ink" | "inkml" => Ok (Application :: InkmlXml) , "application/IOTP" => Ok (Application :: Iotp) , "application/ipfix" | "ipfix" => Ok (Application :: Ipfix) , "application/ipp" => Ok (Application :: Ipp) , "application/ISUP" => Ok (Application :: Isup) , "application/its+xml" | "its" => Ok (Application :: ItsXml) , "application/java-archive" | "jar" => Ok (Application :: JavaArchive) , "application/jf2feed+json" => Ok (Application :: Jf2FeedJson) , "application/jose" => Ok (Application :: Jose) , "application/jose+json" => Ok (Application :: JoseJson) , "application/jrd+json" | "jrd" => Ok (Application :: JrdJson) , "application/jscalendar+json" => Ok (Application :: JscalendarJson) , "application/json" | "json" => Ok (Application :: Json) , "application/json-patch+json" | "json-patch" => Ok (Application :: JsonPatchJson) , "application/json-seq" => Ok (Application :: JsonSeq) , "application/jwk+json" => Ok (Application :: JwkJson) , "application/jwk-set+json" => Ok (Application :: JwkSetJson) , "application/jwt" => Ok (Application :: Jwt) , "application/kpml-request+xml" => Ok (Application :: KpmlRequestXml) , "application/kpml-response+xml" => Ok (Application :: KpmlResponseXml) , "application/ld+json" | "jsonld" => Ok (Application :: LdJson) , "application/lgr+xml" | "lgr" => Ok (Application :: LgrXml) , "application/link-format" | "wlnk" => Ok (Application :: LinkFormat) , "application/linkset" => Ok (Application :: Linkset) , "application/linkset+json" => Ok (Application :: LinksetJson) , "application/load-control+xml" => Ok (Application :: LoadControlXml) , "application/logout+jwt" => Ok (Application :: LogoutJwt) , "application/lost+xml" | "lostxml" => Ok (Application :: LostXml) , "application/lostsync+xml" | "lostsyncxml" => Ok (Application :: LostsyncXml) , "application/lpf+zip" | "lpf" => Ok (Application :: LpfZip) , "application/LXF" | "lxf" => Ok (Application :: Lxf) , "application/mac-binhex40" | "hqx" => Ok (Application :: MacBinhex40) , "application/macwriteii" => Ok (Application :: Macwriteii) , "application/mads+xml" | "mads" => Ok (Application :: MadsXml) , "application/manifest+json" | "webmanifest" => Ok (Application :: ManifestJson) , "application/marc" | "mrc" => Ok (Application :: Marc) , "application/marcxml+xml" | "mrcx" => Ok (Application :: MarcxmlXml) , "application/mathematica" | "nb" | "ma" | "mb" => Ok (Application :: Mathematica) , "application/mathml+xml" | "mml" => Ok (Application :: MathmlXml) , "application/mathml-content+xml" => Ok (Application :: MathmlContentXml) , "application/mathml-presentation+xml" => Ok (Application :: MathmlPresentationXml) , "application/mbms-associated-procedure-description+xml" => Ok (Application :: MbmsAssociatedProcedureDescriptionXml) , "application/mbms-deregister+xml" => Ok (Application :: MbmsDeregisterXml) , "application/mbms-envelope+xml" => Ok (Application :: MbmsEnvelopeXml) , "application/mbms-msk-response+xml" => Ok (Application :: MbmsMskResponseXml) , "application/mbms-msk+xml" => Ok (Application :: MbmsMskXml) , "application/mbms-protection-description+xml" => Ok (Application :: MbmsProtectionDescriptionXml) , "application/mbms-reception-report+xml" => Ok (Application :: MbmsReceptionReportXml) , "application/mbms-register-response+xml" => Ok (Application :: MbmsRegisterResponseXml) , "application/mbms-register+xml" => Ok (Application :: MbmsRegisterXml) , "application/mbms-schedule+xml" => Ok (Application :: MbmsScheduleXml) , "application/mbms-user-service-description+xml" => Ok (Application :: MbmsUserServiceDescriptionXml) , "application/mbox" | "mbox" => Ok (Application :: Mbox) , "application/media_control+xml" => Ok (Application :: MediaControlXml) , "application/media-policy-dataset+xml" => Ok (Application :: MediaPolicyDatasetXml) , "application/mediaservercontrol+xml" => Ok (Application :: MediaservercontrolXml) , "application/merge-patch+json" => Ok (Application :: MergePatchJson) , "application/metalink4+xml" | "meta4" => Ok (Application :: Metalink4Xml) , "application/mets+xml" | "mets" => Ok (Application :: MetsXml) , "application/MF4" | "mf4" => Ok (Application :: Mf4) , "application/mikey" => Ok (Application :: Mikey) , "application/mipc" | "h5" => Ok (Application :: Mipc) , "application/missing-blocks+cbor-seq" => Ok (Application :: MissingBlocksCborSeq) , "application/mmt-aei+xml" | "maei" => Ok (Application :: MmtAeiXml) , "application/mmt-usd+xml" | "musd" => Ok (Application :: MmtUsdXml) , "application/mods+xml" | "mods" => Ok (Application :: ModsXml) , "application/moss-keys" => Ok (Application :: MossKeys) , "application/moss-signature" => Ok (Application :: MossSignature) , "application/mosskey-data" => Ok (Application :: MosskeyData) , "application/mosskey-request" => Ok (Application :: MosskeyRequest) , "application/mp21" | "m21" | "mp21" => Ok (Application :: Mp21) , "application/mp4" => Ok (Application :: Mp4) , "application/mpeg4-generic" => Ok (Application :: Mpeg4Generic) , "application/mpeg4-iod" => Ok (Application :: Mpeg4Iod) , "application/mpeg4-iod-xmt" => Ok (Application :: Mpeg4IodXmt) , "application/mrb-consumer+xml" => Ok (Application :: MrbConsumerXml) , "application/mrb-publish+xml" => Ok (Application :: MrbPublishXml) , "application/msc-ivr+xml" => Ok (Application :: MscIvrXml) , "application/msc-mixer+xml" => Ok (Application :: MscMixerXml) , "application/msword" | "doc" => Ok (Application :: Msword) , "application/mud+json" => Ok (Application :: MudJson) , "application/multipart-core" => Ok (Application :: MultipartCore) , "application/mxf" | "mxf" => Ok (Application :: Mxf) , "application/n-quads" | "nq" => Ok (Application :: NQuads) , "application/n-triples" | "nt" => Ok (Application :: NTriples) , "application/nasdata" => Ok (Application :: Nasdata) , "application/news-checkgroups" => Ok (Application :: NewsCheckgroups) , "application/news-groupinfo" => Ok (Application :: NewsGroupinfo) , "application/news-transmission" => Ok (Application :: NewsTransmission) , "application/nlsml+xml" => Ok (Application :: NlsmlXml) , "application/node" => Ok (Application :: Node) , "application/nss" => Ok (Application :: Nss) , "application/oauth-authz-req+jwt" => Ok (Application :: OauthAuthzReqJwt) , "application/oblivious-dns-message" => Ok (Application :: ObliviousDnsMessage) , "application/ocsp-request" | "orq" => Ok (Application :: OcspRequest) , "application/ocsp-response" | "ors" => Ok (Application :: OcspResponse) , "application/octet-stream" | "bin" | "lha" | "lzh" | "exe" | "class" | "so" | "dll" | "img" | "iso" => Ok (Application :: OctetStream) , "application/ODA" | "oda" => Ok (Application :: Oda) , "application/odm+xml" => Ok (Application :: OdmXml) , "application/ODX" | "odx" => Ok (Application :: Odx) , "application/oebps-package+xml" | "opf" => Ok (Application :: OebpsPackageXml) , "application/ogg" | "ogx" => Ok (Application :: Ogg) , "application/ohttp-keys" => Ok (Application :: OhttpKeys) , "application/opc-nodeset+xml" => Ok (Application :: OpcNodesetXml) , "application/oscore" => Ok (Application :: Oscore) , "application/oxps" | "oxps" => Ok (Application :: Oxps) , "application/p21" | "p21" | "stp" | "step" | "stpnc" | "210" | "ifc" => Ok (Application :: P21) , "application/p21+zip" | "stpz" => Ok (Application :: P21Zip) , "application/p2p-overlay+xml" | "relo" => Ok (Application :: P2POverlayXml) , "application/parityfec" => Ok (Application :: Parityfec) , "application/passport" => Ok (Application :: Passport) , "application/patch-ops-error+xml" => Ok (Application :: PatchOpsErrorXml) , "application/pdf" | "pdf" => Ok (Application :: Pdf) , "application/PDX" | "pdx" => Ok (Application :: Pdx) , "application/pem-certificate-chain" | "pem" => Ok (Application :: PemCertificateChain) , "application/pgp-encrypted" | "pgp" => Ok (Application :: PgpEncrypted) , "application/pgp-keys" => Ok (Application :: PgpKeys) , "application/pgp-signature" | "sig" => Ok (Application :: PgpSignature) , "application/pidf-diff+xml" => Ok (Application :: PidfDiffXml) , "application/pidf+xml" => Ok (Application :: PidfXml) , "application/pkcs10" | "p10" => Ok (Application :: Pkcs10) , "application/pkcs7-mime" | "p7m" | "p7c" => Ok (Application :: Pkcs7Mime) , "application/pkcs7-signature" | "p7s" => Ok (Application :: Pkcs7Signature) , "application/pkcs8" | "p8" => Ok (Application :: Pkcs8) , "application/pkcs8-encrypted" | "p8e" => Ok (Application :: Pkcs8Encrypted) , "application/pkcs12" | "p12" | "pfx" => Ok (Application :: Pkcs12) , "application/pkix-attr-cert" => Ok (Application :: PkixAttrCert) , "application/pkix-cert" | "cer" => Ok (Application :: PkixCert) , "application/pkix-crl" | "crl" => Ok (Application :: PkixCrl) , "application/pkix-pkipath" | "pkipath" => Ok (Application :: PkixPkipath) , "application/pkixcmp" | "pki" => Ok (Application :: Pkixcmp) , "application/pls+xml" | "pls" => Ok (Application :: PlsXml) , "application/poc-settings+xml" => Ok (Application :: PocSettingsXml) , "application/postscript" | "ps" | "eps" | "ai" => Ok (Application :: Postscript) , "application/ppsp-tracker+json" => Ok (Application :: PpspTrackerJson) , "application/problem+json" => Ok (Application :: ProblemJson) , "application/problem+xml" => Ok (Application :: ProblemXml) , "application/provenance+xml" | "provx" => Ok (Application :: ProvenanceXml) , "application/prs.alvestrand.titrax-sheet" => Ok (Application :: PrsAlvestrandTitraxSheet) , "application/prs.cww" | "cw" | "cww" => Ok (Application :: PrsCww) , "application/prs.cyn" => Ok (Application :: PrsCyn) , "application/prs.hpub+zip" | "hpub" => Ok (Application :: PrsHpubZip) , "application/prs.implied-document+xml" => Ok (Application :: PrsImpliedDocumentXml) , "application/prs.implied-executable" => Ok (Application :: PrsImpliedExecutable) , "application/prs.implied-structure" => Ok (Application :: PrsImpliedStructure) , "application/prs.nprend" | "rnd" | "rct" => Ok (Application :: PrsNprend) , "application/prs.plucker" => Ok (Application :: PrsPlucker) , "application/prs.rdf-xml-crypt" | "rdf-crypt" => Ok (Application :: PrsRdfXmlCrypt) , "application/prs.xsf+xml" | "xsf" => Ok (Application :: PrsXsfXml) , "application/pskc+xml" | "pskcxml" => Ok (Application :: PskcXml) , "application/pvd+json" => Ok (Application :: PvdJson) , "application/rdf+xml" | "rdf" => Ok (Application :: RdfXml) , "application/route-apd+xml" | "rapd" => Ok (Application :: RouteApdXml) , "application/route-s-tsid+xml" | "sls" => Ok (Application :: RouteSTsidXml) , "application/route-usd+xml" | "rusd" => Ok (Application :: RouteUsdXml) , "application/QSIG" => Ok (Application :: Qsig) , "application/raptorfec" => Ok (Application :: Raptorfec) , "application/rdap+json" => Ok (Application :: RdapJson) , "application/reginfo+xml" | "rif" => Ok (Application :: ReginfoXml) , "application/relax-ng-compact-syntax" | "rnc" => Ok (Application :: RelaxNgCompactSyntax) , "application/reputon+json" => Ok (Application :: ReputonJson) , "application/resource-lists-diff+xml" | "rld" => Ok (Application :: ResourceListsDiffXml) , "application/resource-lists+xml" | "rl" => Ok (Application :: ResourceListsXml) , "application/rfc+xml" | "rfcxml" => Ok (Application :: RfcXml) , "application/riscos" => Ok (Application :: Riscos) , "application/rlmi+xml" => Ok (Application :: RlmiXml) , "application/rls-services+xml" | "rs" => Ok (Application :: RlsServicesXml) , "application/rpki-checklist" => Ok (Application :: RpkiChecklist) , "application/rpki-ghostbusters" | "gbr" => Ok (Application :: RpkiGhostbusters) , "application/rpki-manifest" | "mft" => Ok (Application :: RpkiManifest) , "application/rpki-publication" => Ok (Application :: RpkiPublication) , "application/rpki-roa" | "roa" => Ok (Application :: RpkiRoa) , "application/rpki-updown" => Ok (Application :: RpkiUpdown) , "application/rtf" | "rtf" => Ok (Application :: Rtf) , "application/rtploopback" => Ok (Application :: Rtploopback) , "application/rtx" => Ok (Application :: Rtx) , "application/samlassertion+xml" => Ok (Application :: SamlassertionXml) , "application/samlmetadata+xml" => Ok (Application :: SamlmetadataXml) , "application/sarif-external-properties+json" | "sarif-external-properties" | "sarif-external-properties.json" => Ok (Application :: SarifExternalPropertiesJson) , "application/sarif+json" | "sarif" | "sarif.json" => Ok (Application :: SarifJson) , "application/sbe" => Ok (Application :: Sbe) , "application/sbml+xml" => Ok (Application :: SbmlXml) , "application/scaip+xml" => Ok (Application :: ScaipXml) , "application/scim+json" | "scim" => Ok (Application :: ScimJson) , "application/scvp-cv-request" | "scq" => Ok (Application :: ScvpCvRequest) , "application/scvp-cv-response" | "scs" => Ok (Application :: ScvpCvResponse) , "application/scvp-vp-request" | "spq" => Ok (Application :: ScvpVpRequest) , "application/scvp-vp-response" | "spp" => Ok (Application :: ScvpVpResponse) , "application/sdp" | "sdp" => Ok (Application :: Sdp) , "application/secevent+jwt" => Ok (Application :: SeceventJwt) , "application/senml-etch+cbor" | "senml-etchc" => Ok (Application :: SenmlEtchCbor) , "application/senml-etch+json" | "senml-etchj" => Ok (Application :: SenmlEtchJson) , "application/senml-exi" | "senmle" => Ok (Application :: SenmlExi) , "application/senml+cbor" | "senmlc" => Ok (Application :: SenmlCbor) , "application/senml+json" | "senml" => Ok (Application :: SenmlJson) , "application/senml+xml" | "senmlx" => Ok (Application :: SenmlXml) , "application/sensml-exi" | "sensmle" => Ok (Application :: SensmlExi) , "application/sensml+cbor" | "sensmlc" => Ok (Application :: SensmlCbor) , "application/sensml+json" | "sensml" => Ok (Application :: SensmlJson) , "application/sensml+xml" | "sensmlx" => Ok (Application :: SensmlXml) , "application/sep-exi" => Ok (Application :: SepExi) , "application/sep+xml" => Ok (Application :: SepXml) , "application/session-info" => Ok (Application :: SessionInfo) , "application/set-payment" => Ok (Application :: SetPayment) , "application/set-payment-initiation" => Ok (Application :: SetPaymentInitiation) , "application/set-registration" => Ok (Application :: SetRegistration) , "application/set-registration-initiation" => Ok (Application :: SetRegistrationInitiation) , "application/SGML" => Ok (Application :: Sgml) , "application/sgml-open-catalog" | "soc" => Ok (Application :: SgmlOpenCatalog) , "application/shf+xml" | "shf" => Ok (Application :: ShfXml) , "application/sieve" | "siv" | "sieve" => Ok (Application :: Sieve) , "application/simple-filter+xml" | "cl" => Ok (Application :: SimpleFilterXml) , "application/simple-message-summary" => Ok (Application :: SimpleMessageSummary) , "application/simpleSymbolContainer" => Ok (Application :: SimpleSymbolContainer) , "application/sipc" => Ok (Application :: Sipc) , "application/slate" => Ok (Application :: Slate) , "application/smil+xml" | "smil" | "smi" | "sml" => Ok (Application :: SmilXml) , "application/smpte336m" => Ok (Application :: Smpte336M) , "application/soap+fastinfoset" => Ok (Application :: SoapFastinfoset) , "application/soap+xml" => Ok (Application :: SoapXml) , "application/sparql-query" | "rq" => Ok (Application :: SparqlQuery) , "application/spdx+json" | "spdx.json" => Ok (Application :: SpdxJson) , "application/sparql-results+xml" | "srx" => Ok (Application :: SparqlResultsXml) , "application/spirits-event+xml" => Ok (Application :: SpiritsEventXml) , "application/sql" | "sql" => Ok (Application :: Sql) , "application/srgs" | "gram" => Ok (Application :: Srgs) , "application/srgs+xml" | "grxml" => Ok (Application :: SrgsXml) , "application/sru+xml" | "sru" => Ok (Application :: SruXml) , "application/ssml+xml" | "ssml" => Ok (Application :: SsmlXml) , "application/stix+json" | "stix" => Ok (Application :: StixJson) , "application/swid+cbor" | "coswid" => Ok (Application :: SwidCbor) , "application/swid+xml" | "swidtag" => Ok (Application :: SwidXml) , "application/tamp-apex-update" | "tau" => Ok (Application :: TampApexUpdate) , "application/tamp-apex-update-confirm" | "auc" => Ok (Application :: TampApexUpdateConfirm) , "application/tamp-community-update" | "tcu" => Ok (Application :: TampCommunityUpdate) , "application/tamp-community-update-confirm" | "cuc" => Ok (Application :: TampCommunityUpdateConfirm) , "application/tamp-error" | "ter" => Ok (Application :: TampError) , "application/tamp-sequence-adjust" | "tsa" => Ok (Application :: TampSequenceAdjust) , "application/tamp-sequence-adjust-confirm" | "sac" => Ok (Application :: TampSequenceAdjustConfirm) , "application/tamp-status-query" => Ok (Application :: TampStatusQuery) , "application/tamp-status-response" => Ok (Application :: TampStatusResponse) , "application/tamp-update" | "tur" => Ok (Application :: TampUpdate) , "application/tamp-update-confirm" | "tuc" => Ok (Application :: TampUpdateConfirm) , "application/taxii+json" => Ok (Application :: TaxiiJson) , "application/td+json" | "jsontd" => Ok (Application :: TdJson) , "application/tei+xml" | "tei" | "teiCorpus" | "odd" => Ok (Application :: TeiXml) , "application/TETRA_ISI" => Ok (Application :: TetraIsi) , "application/thraud+xml" | "tfi" => Ok (Application :: ThraudXml) , "application/timestamp-query" | "tsq" => Ok (Application :: TimestampQuery) , "application/timestamp-reply" | "tsr" => Ok (Application :: TimestampReply) , "application/timestamped-data" | "tsd" => Ok (Application :: TimestampedData) , "application/tlsrpt+gzip" => Ok (Application :: TlsrptGzip) , "application/tlsrpt+json" => Ok (Application :: TlsrptJson) , "application/tm+json" | "jsontm" | "tm.json" | "tm.jsonld" => Ok (Application :: TmJson) , "application/tnauthlist" => Ok (Application :: Tnauthlist) , "application/token-introspection+jwt" => Ok (Application :: TokenIntrospectionJwt) , "application/trickle-ice-sdpfrag" => Ok (Application :: TrickleIceSdpfrag) , "application/trig" | "trig" => Ok (Application :: Trig) , "application/ttml+xml" | "ttml" => Ok (Application :: TtmlXml) , "application/tve-trigger" => Ok (Application :: TveTrigger) , "application/tzif" => Ok (Application :: Tzif) , "application/tzif-leap" => Ok (Application :: TzifLeap) , "application/ulpfec" => Ok (Application :: Ulpfec) , "application/urc-grpsheet+xml" | "gsheet" => Ok (Application :: UrcGrpsheetXml) , "application/urc-ressheet+xml" | "rsheet" => Ok (Application :: UrcRessheetXml) , "application/urc-targetdesc+xml" | "td" => Ok (Application :: UrcTargetdescXml) , "application/urc-uisocketdesc+xml" | "uis" => Ok (Application :: UrcUisocketdescXml) , "application/vcard+json" => Ok (Application :: VcardJson) , "application/vcard+xml" => Ok (Application :: VcardXml) , "application/vemmi" => Ok (Application :: Vemmi) , "application/vnd.1000minds.decision-model+xml" | "1km" => Ok (Application :: Vnd1000MindsDecisionModelXml) , "application/vnd.1ob" | "ob" => Ok (Application :: Vnd1Ob) , "application/vnd.3gpp.5gnas" => Ok (Application :: Vnd3Gpp5Gnas) , "application/vnd.3gpp.access-transfer-events+xml" => Ok (Application :: Vnd3GppAccessTransferEventsXml) , "application/vnd.3gpp.bsf+xml" => Ok (Application :: Vnd3GppBsfXml) , "application/vnd.3gpp.crs+xml" => Ok (Application :: Vnd3GppCrsXml) , "application/vnd.3gpp.current-location-discovery+xml" => Ok (Application :: Vnd3GppCurrentLocationDiscoveryXml) , "application/vnd.3gpp.GMOP+xml" => Ok (Application :: Vnd3GppGMOPXml) , "application/vnd.3gpp.gtpc" => Ok (Application :: Vnd3GppGtpc) , "application/vnd.3gpp.interworking-data" => Ok (Application :: Vnd3GppInterworkingData) , "application/vnd.3gpp.lpp" => Ok (Application :: Vnd3GppLpp) , "application/vnd.3gpp.mc-signalling-ear" => Ok (Application :: Vnd3GppMcSignallingEar) , "application/vnd.3gpp.mcdata-affiliation-command+xml" => Ok (Application :: Vnd3GppMcdataAffiliationCommandXml) , "application/vnd.3gpp.mcdata-info+xml" => Ok (Application :: Vnd3GppMcdataInfoXml) , "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml" => Ok (Application :: Vnd3GppMcdataMsgstoreCtrlRequestXml) , "application/vnd.3gpp.mcdata-payload" => Ok (Application :: Vnd3GppMcdataPayload) , "application/vnd.3gpp.mcdata-regroup+xml" => Ok (Application :: Vnd3GppMcdataRegroupXml) , "application/vnd.3gpp.mcdata-service-config+xml" => Ok (Application :: Vnd3GppMcdataServiceConfigXml) , "application/vnd.3gpp.mcdata-signalling" => Ok (Application :: Vnd3GppMcdataSignalling) , "application/vnd.3gpp.mcdata-ue-config+xml" => Ok (Application :: Vnd3GppMcdataUeConfigXml) , "application/vnd.3gpp.mcdata-user-profile+xml" => Ok (Application :: Vnd3GppMcdataUserProfileXml) , "application/vnd.3gpp.mcptt-affiliation-command+xml" => Ok (Application :: Vnd3GppMcpttAffiliationCommandXml) , "application/vnd.3gpp.mcptt-floor-request+xml" => Ok (Application :: Vnd3GppMcpttFloorRequestXml) , "application/vnd.3gpp.mcptt-info+xml" => Ok (Application :: Vnd3GppMcpttInfoXml) , "application/vnd.3gpp.mcptt-location-info+xml" => Ok (Application :: Vnd3GppMcpttLocationInfoXml) , "application/vnd.3gpp.mcptt-mbms-usage-info+xml" => Ok (Application :: Vnd3GppMcpttMbmsUsageInfoXml) , "application/vnd.3gpp.mcptt-regroup+xml" => Ok (Application :: Vnd3GppMcpttRegroupXml) , "application/vnd.3gpp.mcptt-service-config+xml" => Ok (Application :: Vnd3GppMcpttServiceConfigXml) , "application/vnd.3gpp.mcptt-signed+xml" => Ok (Application :: Vnd3GppMcpttSignedXml) , "application/vnd.3gpp.mcptt-ue-config+xml" => Ok (Application :: Vnd3GppMcpttUeConfigXml) , "application/vnd.3gpp.mcptt-ue-init-config+xml" => Ok (Application :: Vnd3GppMcpttUeInitConfigXml) , "application/vnd.3gpp.mcptt-user-profile+xml" => Ok (Application :: Vnd3GppMcpttUserProfileXml) , "application/vnd.3gpp.mcvideo-affiliation-command+xml" => Ok (Application :: Vnd3GppMcvideoAffiliationCommandXml) , "application/vnd.3gpp.mcvideo-info+xml" => Ok (Application :: Vnd3GppMcvideoInfoXml) , "application/vnd.3gpp.mcvideo-location-info+xml" => Ok (Application :: Vnd3GppMcvideoLocationInfoXml) , "application/vnd.3gpp.mcvideo-mbms-usage-info+xml" => Ok (Application :: Vnd3GppMcvideoMbmsUsageInfoXml) , "application/vnd.3gpp.mcvideo-regroup+xml" => Ok (Application :: Vnd3GppMcvideoRegroupXml) , "application/vnd.3gpp.mcvideo-service-config+xml" => Ok (Application :: Vnd3GppMcvideoServiceConfigXml) , "application/vnd.3gpp.mcvideo-transmission-request+xml" => Ok (Application :: Vnd3GppMcvideoTransmissionRequestXml) , "application/vnd.3gpp.mcvideo-ue-config+xml" => Ok (Application :: Vnd3GppMcvideoUeConfigXml) , "application/vnd.3gpp.mcvideo-user-profile+xml" => Ok (Application :: Vnd3GppMcvideoUserProfileXml) , "application/vnd.3gpp.mid-call+xml" => Ok (Application :: Vnd3GppMidCallXml) , "application/vnd.3gpp.ngap" => Ok (Application :: Vnd3GppNgap) , "application/vnd.3gpp.pfcp" => Ok (Application :: Vnd3GppPfcp) , "application/vnd.3gpp.pic-bw-large" | "plb" => Ok (Application :: Vnd3GppPicBwLarge) , "application/vnd.3gpp.pic-bw-small" | "psb" => Ok (Application :: Vnd3GppPicBwSmall) , "application/vnd.3gpp.pic-bw-var" | "pvb" => Ok (Application :: Vnd3GppPicBwVar) , "application/vnd.3gpp-prose-pc3a+xml" => Ok (Application :: Vnd3GppProsePc3AXml) , "application/vnd.3gpp-prose-pc3ach+xml" => Ok (Application :: Vnd3GppProsePc3AchXml) , "application/vnd.3gpp-prose-pc3ch+xml" => Ok (Application :: Vnd3GppProsePc3ChXml) , "application/vnd.3gpp-prose-pc8+xml" => Ok (Application :: Vnd3GppProsePc8Xml) , "application/vnd.3gpp-prose+xml" => Ok (Application :: Vnd3GppProseXml) , "application/vnd.3gpp.s1ap" => Ok (Application :: Vnd3GppS1Ap) , "application/vnd.3gpp.seal-group-doc+xml" => Ok (Application :: Vnd3GppSealGroupDocXml) , "application/vnd.3gpp.seal-info+xml" => Ok (Application :: Vnd3GppSealInfoXml) , "application/vnd.3gpp.seal-location-info+xml" => Ok (Application :: Vnd3GppSealLocationInfoXml) , "application/vnd.3gpp.seal-mbms-usage-info+xml" => Ok (Application :: Vnd3GppSealMbmsUsageInfoXml) , "application/vnd.3gpp.seal-network-QoS-management-info+xml" => Ok (Application :: Vnd3GppSealNetworkQoSManagementInfoXml) , "application/vnd.3gpp.seal-ue-config-info+xml" => Ok (Application :: Vnd3GppSealUeConfigInfoXml) , "application/vnd.3gpp.seal-unicast-info+xml" => Ok (Application :: Vnd3GppSealUnicastInfoXml) , "application/vnd.3gpp.seal-user-profile-info+xml" => Ok (Application :: Vnd3GppSealUserProfileInfoXml) , "application/vnd.3gpp.sms" => Ok (Application :: Vnd3GppSms) , "application/vnd.3gpp.sms+xml" => Ok (Application :: Vnd3GppSmsXml) , "application/vnd.3gpp.srvcc-ext+xml" => Ok (Application :: Vnd3GppSrvccExtXml) , "application/vnd.3gpp.SRVCC-info+xml" => Ok (Application :: Vnd3GppSRVCCInfoXml) , "application/vnd.3gpp.state-and-event-info+xml" => Ok (Application :: Vnd3GppStateAndEventInfoXml) , "application/vnd.3gpp.ussd+xml" => Ok (Application :: Vnd3GppUssdXml) , "application/vnd.3gpp.vae-info+xml" => Ok (Application :: Vnd3GppVaeInfoXml) , "application/vnd.3gpp-v2x-local-service-information" => Ok (Application :: Vnd3GppV2XLocalServiceInformation) , "application/vnd.3gpp2.bcmcsinfo+xml" => Ok (Application :: Vnd3Gpp2BcmcsinfoXml) , "application/vnd.3gpp2.sms" | "sms" => Ok (Application :: Vnd3Gpp2Sms) , "application/vnd.3gpp2.tcap" | "tcap" => Ok (Application :: Vnd3Gpp2Tcap) , "application/vnd.3gpp.v2x" => Ok (Application :: Vnd3GppV2X) , "application/vnd.3lightssoftware.imagescal" | "imgcal" => Ok (Application :: Vnd3LightssoftwareImagescal) , "application/vnd.3M.Post-it-Notes" | "pwn" => Ok (Application :: Vnd3MPostItNotes) , "application/vnd.accpac.simply.aso" | "aso" => Ok (Application :: VndAccpacSimplyAso) , "application/vnd.accpac.simply.imp" | "imp" => Ok (Application :: VndAccpacSimplyImp) , "application/vnd.acm.addressxfer+json" => Ok (Application :: VndAcmAddressxferJson) , "application/vnd.acucobol" | "acu" => Ok (Application :: VndAcucobol) , "application/vnd.acucorp" | "atc" | "acutc" => Ok (Application :: VndAcucorp) , "application/vnd.adobe.flash.movie" | "swf" => Ok (Application :: VndAdobeFlashMovie) , "application/vnd.adobe.formscentral.fcdt" | "fcdt" => Ok (Application :: VndAdobeFormscentralFcdt) , "application/vnd.adobe.fxp" | "fxp" | "fxpl" => Ok (Application :: VndAdobeFxp) , "application/vnd.adobe.partial-upload" => Ok (Application :: VndAdobePartialUpload) , "application/vnd.adobe.xdp+xml" | "xdp" => Ok (Application :: VndAdobeXdpXml) , "application/vnd.aether.imp" => Ok (Application :: VndAetherImp) , "application/vnd.afpc.afplinedata" => Ok (Application :: VndAfpcAfplinedata) , "application/vnd.afpc.afplinedata-pagedef" => Ok (Application :: VndAfpcAfplinedataPagedef) , "application/vnd.afpc.cmoca-cmresource" => Ok (Application :: VndAfpcCmocaCmresource) , "application/vnd.afpc.foca-charset" => Ok (Application :: VndAfpcFocaCharset) , "application/vnd.afpc.foca-codedfont" => Ok (Application :: VndAfpcFocaCodedfont) , "application/vnd.afpc.foca-codepage" => Ok (Application :: VndAfpcFocaCodepage) , "application/vnd.afpc.modca" | "list3820" | "listafp" | "afp" | "pseg3820" => Ok (Application :: VndAfpcModca) , "application/vnd.afpc.modca-cmtable" => Ok (Application :: VndAfpcModcaCmtable) , "application/vnd.afpc.modca-formdef" => Ok (Application :: VndAfpcModcaFormdef) , "application/vnd.afpc.modca-mediummap" => Ok (Application :: VndAfpcModcaMediummap) , "application/vnd.afpc.modca-objectcontainer" => Ok (Application :: VndAfpcModcaObjectcontainer) , "application/vnd.afpc.modca-overlay" | "ovl" => Ok (Application :: VndAfpcModcaOverlay) , "application/vnd.afpc.modca-pagesegment" | "psg" => Ok (Application :: VndAfpcModcaPagesegment) , "application/vnd.age" | "age" => Ok (Application :: VndAge) , "application/vnd.ah-barcode" => Ok (Application :: VndAhBarcode) , "application/vnd.ahead.space" | "ahead" => Ok (Application :: VndAheadSpace) , "application/vnd.airzip.filesecure.azf" | "azf" => Ok (Application :: VndAirzipFilesecureAzf) , "application/vnd.airzip.filesecure.azs" | "azs" => Ok (Application :: VndAirzipFilesecureAzs) , "application/vnd.amadeus+json" => Ok (Application :: VndAmadeusJson) , "application/vnd.amazon.mobi8-ebook" | "azw3" => Ok (Application :: VndAmazonMobi8Ebook) , "application/vnd.americandynamics.acc" | "acc" => Ok (Application :: VndAmericandynamicsAcc) , "application/vnd.amiga.ami" | "ami" => Ok (Application :: VndAmigaAmi) , "application/vnd.amundsen.maze+xml" => Ok (Application :: VndAmundsenMazeXml) , "application/vnd.android.ota" | "ota" => Ok (Application :: VndAndroidOta) , "application/vnd.anki" | "apkg" => Ok (Application :: VndAnki) , "application/vnd.anser-web-certificate-issue-initiation" | "cii" => Ok (Application :: VndAnserWebCertificateIssueInitiation) , "application/vnd.antix.game-component" => Ok (Application :: VndAntixGameComponent) , "application/vnd.apache.arrow.file" | "arrow" => Ok (Application :: VndApacheArrowFile) , "application/vnd.apache.arrow.stream" | "arrows" => Ok (Application :: VndApacheArrowStream) , "application/vnd.apache.thrift.binary" => Ok (Application :: VndApacheThriftBinary) , "application/vnd.apache.thrift.compact" => Ok (Application :: VndApacheThriftCompact) , "application/vnd.apache.thrift.json" => Ok (Application :: VndApacheThriftJson) , "application/vnd.apexlang" | "apexland" | "apex" | "axdl" => Ok (Application :: VndApexlang) , "application/vnd.api+json" => Ok (Application :: VndApiJson) , "application/vnd.aplextor.warrp+json" => Ok (Application :: VndAplextorWarrpJson) , "application/vnd.apothekende.reservation+json" => Ok (Application :: VndApothekendeReservationJson) , "application/vnd.apple.installer+xml" | "dist" | "distz" | "pkg" | "mpkg" => Ok (Application :: VndAppleInstallerXml) , "application/vnd.apple.keynote" | "keynote" => Ok (Application :: VndAppleKeynote) , "application/vnd.apple.mpegurl" | "m3u8" => Ok (Application :: VndAppleMpegurl) , "application/vnd.apple.numbers" | "numbers" => Ok (Application :: VndAppleNumbers) , "application/vnd.apple.pages" | "pages" => Ok (Application :: VndApplePages) , "application/vnd.aristanetworks.swi" | "swi" => Ok (Application :: VndAristanetworksSwi) , "application/vnd.artisan+json" | "artisan" => Ok (Application :: VndArtisanJson) , "application/vnd.artsquare" => Ok (Application :: VndArtsquare) , "application/vnd.astraea-software.iota" | "iota" => Ok (Application :: VndAstraeaSoftwareIota) , "application/vnd.audiograph" | "aep" => Ok (Application :: VndAudiograph) , "application/vnd.autopackage" | "package" => Ok (Application :: VndAutopackage) , "application/vnd.avalon+json" => Ok (Application :: VndAvalonJson) , "application/vnd.avistar+xml" => Ok (Application :: VndAvistarXml) , "application/vnd.balsamiq.bmml+xml" | "bmml" => Ok (Application :: VndBalsamiqBmmlXml) , "application/vnd.banana-accounting" | "ac2" => Ok (Application :: VndBananaAccounting) , "application/vnd.bbf.usp.error" => Ok (Application :: VndBbfUspError) , "application/vnd.bbf.usp.msg" => Ok (Application :: VndBbfUspMsg) , "application/vnd.bbf.usp.msg+json" => Ok (Application :: VndBbfUspMsgJson) , "application/vnd.balsamiq.bmpr" | "bmpr" => Ok (Application :: VndBalsamiqBmpr) , "application/vnd.bekitzur-stech+json" => Ok (Application :: VndBekitzurStechJson) , "application/vnd.belightsoft.lhzd+zip" | "lhzd" => Ok (Application :: VndBelightsoftLhzdZip) , "application/vnd.belightsoft.lhzl+zip" | "lhzl" => Ok (Application :: VndBelightsoftLhzlZip) , "application/vnd.bint.med-content" => Ok (Application :: VndBintMedContent) , "application/vnd.biopax.rdf+xml" => Ok (Application :: VndBiopaxRdfXml) , "application/vnd.blink-idb-value-wrapper" => Ok (Application :: VndBlinkIdbValueWrapper) , "application/vnd.blueice.multipass" | "mpm" => Ok (Application :: VndBlueiceMultipass) , "application/vnd.bluetooth.ep.oob" | "ep" => Ok (Application :: VndBluetoothEpOob) , "application/vnd.bluetooth.le.oob" | "le" => Ok (Application :: VndBluetoothLeOob) , "application/vnd.bmi" | "bmi" => Ok (Application :: VndBmi) , "application/vnd.bpf" => Ok (Application :: VndBpf) , "application/vnd.bpf3" => Ok (Application :: VndBpf3) , "application/vnd.businessobjects" | "rep" => Ok (Application :: VndBusinessobjects) , "application/vnd.byu.uapi+json" => Ok (Application :: VndByuUapiJson) , "application/vnd.cab-jscript" => Ok (Application :: VndCabJscript) , "application/vnd.canon-cpdl" => Ok (Application :: VndCanonCpdl) , "application/vnd.canon-lips" => Ok (Application :: VndCanonLips) , "application/vnd.capasystems-pg+json" => Ok (Application :: VndCapasystemsPgJson) , "application/vnd.cendio.thinlinc.clientconf" | "tlclient" => Ok (Application :: VndCendioThinlincClientconf) , "application/vnd.century-systems.tcp_stream" => Ok (Application :: VndCenturySystemsTcpStream) , "application/vnd.chemdraw+xml" | "cdxml" => Ok (Application :: VndChemdrawXml) , "application/vnd.chess-pgn" | "pgn" => Ok (Application :: VndChessPgn) , "application/vnd.chipnuts.karaoke-mmd" | "mmd" => Ok (Application :: VndChipnutsKaraokeMmd) , "application/vnd.ciedi" => Ok (Application :: VndCiedi) , "application/vnd.cinderella" | "cdy" => Ok (Application :: VndCinderella) , "application/vnd.cirpack.isdn-ext" => Ok (Application :: VndCirpackIsdnExt) , "application/vnd.citationstyles.style+xml" | "csl" => Ok (Application :: VndCitationstylesStyleXml) , "application/vnd.claymore" | "cla" => Ok (Application :: VndClaymore) , "application/vnd.cloanto.rp9" | "rp9" => Ok (Application :: VndCloantoRp9) , "application/vnd.clonk.c4group" | "c4g" | "c4d" | "c4f" | "c4p" | "c4u" => Ok (Application :: VndClonkC4Group) , "application/vnd.cluetrust.cartomobile-config" | "c11amc" => Ok (Application :: VndCluetrustCartomobileConfig) , "application/vnd.cluetrust.cartomobile-config-pkg" | "c11amz" => Ok (Application :: VndCluetrustCartomobileConfigPkg) , "application/vnd.cncf.helm.chart.content.v1.tar+gzip" => Ok (Application :: VndCncfHelmChartContentV1TarGzip) , "application/vnd.cncf.helm.chart.provenance.v1.prov" => Ok (Application :: VndCncfHelmChartProvenanceV1Prov) , "application/vnd.cncf.helm.config.v1+json" => Ok (Application :: VndCncfHelmConfigV1Json) , "application/vnd.coffeescript" | "coffee" => Ok (Application :: VndCoffeescript) , "application/vnd.collabio.xodocuments.document" | "xodt" => Ok (Application :: VndCollabioXodocumentsDocument) , "application/vnd.collabio.xodocuments.document-template" | "xott" => Ok (Application :: VndCollabioXodocumentsDocumentTemplate) , "application/vnd.collabio.xodocuments.presentation" | "xodp" => Ok (Application :: VndCollabioXodocumentsPresentation) , "application/vnd.collabio.xodocuments.presentation-template" | "xotp" => Ok (Application :: VndCollabioXodocumentsPresentationTemplate) , "application/vnd.collabio.xodocuments.spreadsheet" | "xods" => Ok (Application :: VndCollabioXodocumentsSpreadsheet) , "application/vnd.collabio.xodocuments.spreadsheet-template" | "xots" => Ok (Application :: VndCollabioXodocumentsSpreadsheetTemplate) , "application/vnd.collection.doc+json" => Ok (Application :: VndCollectionDocJson) , "application/vnd.collection+json" => Ok (Application :: VndCollectionJson) , "application/vnd.collection.next+json" => Ok (Application :: VndCollectionNextJson) , "application/vnd.comicbook-rar" | "cbr" => Ok (Application :: VndComicbookRar) , "application/vnd.comicbook+zip" | "cbz" => Ok (Application :: VndComicbookZip) , "application/vnd.commerce-battelle" | "ica" | "icf" | "icd" | "ic0" | "ic1" | "ic2" | "ic3" | "ic4" | "ic5" | "ic6" | "ic7" | "ic8" => Ok (Application :: VndCommerceBattelle) , "application/vnd.commonspace" | "csp" | "cst" => Ok (Application :: VndCommonspace) , "application/vnd.coreos.ignition+json" | "ign" | "ignition" => Ok (Application :: VndCoreosIgnitionJson) , "application/vnd.cosmocaller" | "cmc" => Ok (Application :: VndCosmocaller) , "application/vnd.contact.cmsg" | "cdbcmsg" => Ok (Application :: VndContactCmsg) , "application/vnd.crick.clicker" | "clkx" => Ok (Application :: VndCrickClicker) , "application/vnd.crick.clicker.keyboard" | "clkk" => Ok (Application :: VndCrickClickerKeyboard) , "application/vnd.crick.clicker.palette" | "clkp" => Ok (Application :: VndCrickClickerPalette) , "application/vnd.crick.clicker.template" | "clkt" => Ok (Application :: VndCrickClickerTemplate) , "application/vnd.crick.clicker.wordbank" | "clkw" => Ok (Application :: VndCrickClickerWordbank) , "application/vnd.criticaltools.wbs+xml" | "wbs" => Ok (Application :: VndCriticaltoolsWbsXml) , "application/vnd.cryptii.pipe+json" => Ok (Application :: VndCryptiiPipeJson) , "application/vnd.crypto-shade-file" | "ssvc" => Ok (Application :: VndCryptoShadeFile) , "application/vnd.cryptomator.encrypted" | "c9r" | "c9s" => Ok (Application :: VndCryptomatorEncrypted) , "application/vnd.cryptomator.vault" | "cryptomator" => Ok (Application :: VndCryptomatorVault) , "application/vnd.ctc-posml" | "pml" => Ok (Application :: VndCtcPosml) , "application/vnd.ctct.ws+xml" => Ok (Application :: VndCtctWsXml) , "application/vnd.cups-pdf" => Ok (Application :: VndCupsPdf) , "application/vnd.cups-postscript" => Ok (Application :: VndCupsPostscript) , "application/vnd.cups-ppd" | "ppd" => Ok (Application :: VndCupsPpd) , "application/vnd.cups-raster" => Ok (Application :: VndCupsRaster) , "application/vnd.cups-raw" => Ok (Application :: VndCupsRaw) , "application/vnd.curl" | "curl" => Ok (Application :: VndCurl) , "application/vnd.cyan.dean.root+xml" => Ok (Application :: VndCyanDeanRootXml) , "application/vnd.cybank" => Ok (Application :: VndCybank) , "application/vnd.cyclonedx+json" => Ok (Application :: VndCyclonedxJson) , "application/vnd.cyclonedx+xml" => Ok (Application :: VndCyclonedxXml) , "application/vnd.d2l.coursepackage1p0+zip" => Ok (Application :: VndD2LCoursepackage1P0Zip) , "application/vnd.d3m-dataset" => Ok (Application :: VndD3MDataset) , "application/vnd.d3m-problem" => Ok (Application :: VndD3MProblem) , "application/vnd.dart" | "dart" => Ok (Application :: VndDart) , "application/vnd.data-vision.rdz" | "rdz" => Ok (Application :: VndDataVisionRdz) , "application/vnd.datalog" | "dl" => Ok (Application :: VndDatalog) , "application/vnd.datapackage+json" => Ok (Application :: VndDatapackageJson) , "application/vnd.dataresource+json" => Ok (Application :: VndDataresourceJson) , "application/vnd.dbf" | "dbf" => Ok (Application :: VndDbf) , "application/vnd.debian.binary-package" | "deb" | "udeb" => Ok (Application :: VndDebianBinaryPackage) , "application/vnd.dece.data" | "uvf" | "uvvf" | "uvd" | "uvvd" => Ok (Application :: VndDeceData) , "application/vnd.dece.ttml+xml" | "uvt" | "uvvt" => Ok (Application :: VndDeceTtmlXml) , "application/vnd.dece.unspecified" | "uvx" | "uvvx" => Ok (Application :: VndDeceUnspecified) , "application/vnd.dece.zip" | "uvz" | "uvvz" => Ok (Application :: VndDeceZip) , "application/vnd.denovo.fcselayout-link" | "fe_launch" => Ok (Application :: VndDenovoFcselayoutLink) , "application/vnd.desmume.movie" | "dsm" => Ok (Application :: VndDesmumeMovie) , "application/vnd.dir-bi.plate-dl-nosuffix" => Ok (Application :: VndDirBiPlateDlNosuffix) , "application/vnd.dm.delegation+xml" => Ok (Application :: VndDmDelegationXml) , "application/vnd.dna" | "dna" => Ok (Application :: VndDna) , "application/vnd.document+json" | "docjson" => Ok (Application :: VndDocumentJson) , "application/vnd.dolby.mobile.1" => Ok (Application :: VndDolbyMobile1) , "application/vnd.dolby.mobile.2" => Ok (Application :: VndDolbyMobile2) , "application/vnd.doremir.scorecloud-binary-document" | "scld" => Ok (Application :: VndDoremirScorecloudBinaryDocument) , "application/vnd.dpgraph" | "dpg" | "mwc" | "dpgraph" => Ok (Application :: VndDpgraph) , "application/vnd.dreamfactory" | "dfac" => Ok (Application :: VndDreamfactory) , "application/vnd.drive+json" => Ok (Application :: VndDriveJson) , "application/vnd.dtg.local" => Ok (Application :: VndDtgLocal) , "application/vnd.dtg.local.flash" | "fla" => Ok (Application :: VndDtgLocalFlash) , "application/vnd.dtg.local.html" => Ok (Application :: VndDtgLocalHtml) , "application/vnd.dvb.ait" | "ait" => Ok (Application :: VndDvbAit) , "application/vnd.dvb.dvbisl+xml" => Ok (Application :: VndDvbDvbislXml) , "application/vnd.dvb.dvbj" => Ok (Application :: VndDvbDvbj) , "application/vnd.dvb.esgcontainer" => Ok (Application :: VndDvbEsgcontainer) , "application/vnd.dvb.ipdcdftnotifaccess" => Ok (Application :: VndDvbIpdcdftnotifaccess) , "application/vnd.dvb.ipdcesgaccess" => Ok (Application :: VndDvbIpdcesgaccess) , "application/vnd.dvb.ipdcesgaccess2" => Ok (Application :: VndDvbIpdcesgaccess2) , "application/vnd.dvb.ipdcesgpdd" => Ok (Application :: VndDvbIpdcesgpdd) , "application/vnd.dvb.ipdcroaming" => Ok (Application :: VndDvbIpdcroaming) , "application/vnd.dvb.iptv.alfec-base" => Ok (Application :: VndDvbIptvAlfecBase) , "application/vnd.dvb.iptv.alfec-enhancement" => Ok (Application :: VndDvbIptvAlfecEnhancement) , "application/vnd.dvb.notif-aggregate-root+xml" => Ok (Application :: VndDvbNotifAggregateRootXml) , "application/vnd.dvb.notif-container+xml" => Ok (Application :: VndDvbNotifContainerXml) , "application/vnd.dvb.notif-generic+xml" => Ok (Application :: VndDvbNotifGenericXml) , "application/vnd.dvb.notif-ia-msglist+xml" => Ok (Application :: VndDvbNotifIaMsglistXml) , "application/vnd.dvb.notif-ia-registration-request+xml" => Ok (Application :: VndDvbNotifIaRegistrationRequestXml) , "application/vnd.dvb.notif-ia-registration-response+xml" => Ok (Application :: VndDvbNotifIaRegistrationResponseXml) , "application/vnd.dvb.notif-init+xml" => Ok (Application :: VndDvbNotifInitXml) , "application/vnd.dvb.pfr" => Ok (Application :: VndDvbPfr) , "application/vnd.dvb.service" | "svc" => Ok (Application :: VndDvbService) , "application/vnd.dxr" => Ok (Application :: VndDxr) , "application/vnd.dynageo" | "geo" => Ok (Application :: VndDynageo) , "application/vnd.dzr" | "dzr" => Ok (Application :: VndDzr) , "application/vnd.easykaraoke.cdgdownload" => Ok (Application :: VndEasykaraokeCdgdownload) , "application/vnd.ecip.rlp" => Ok (Application :: VndEcipRlp) , "application/vnd.ecdis-update" => Ok (Application :: VndEcdisUpdate) , "application/vnd.eclipse.ditto+json" => Ok (Application :: VndEclipseDittoJson) , "application/vnd.ecowin.chart" | "mag" => Ok (Application :: VndEcowinChart) , "application/vnd.ecowin.filerequest" => Ok (Application :: VndEcowinFilerequest) , "application/vnd.ecowin.fileupdate" => Ok (Application :: VndEcowinFileupdate) , "application/vnd.ecowin.series" => Ok (Application :: VndEcowinSeries) , "application/vnd.ecowin.seriesrequest" => Ok (Application :: VndEcowinSeriesrequest) , "application/vnd.ecowin.seriesupdate" => Ok (Application :: VndEcowinSeriesupdate) , "application/vnd.efi.img" => Ok (Application :: VndEfiImg) , "application/vnd.efi.iso" => Ok (Application :: VndEfiIso) , "application/vnd.eln+zip" | "eln" => Ok (Application :: VndElnZip) , "application/vnd.emclient.accessrequest+xml" => Ok (Application :: VndEmclientAccessrequestXml) , "application/vnd.enliven" | "nml" => Ok (Application :: VndEnliven) , "application/vnd.enphase.envoy" => Ok (Application :: VndEnphaseEnvoy) , "application/vnd.eprints.data+xml" => Ok (Application :: VndEprintsDataXml) , "application/vnd.epson.esf" | "esf" => Ok (Application :: VndEpsonEsf) , "application/vnd.epson.msf" | "msf" => Ok (Application :: VndEpsonMsf) , "application/vnd.epson.quickanime" | "qam" => Ok (Application :: VndEpsonQuickanime) , "application/vnd.epson.salt" | "slt" => Ok (Application :: VndEpsonSalt) , "application/vnd.epson.ssf" | "ssf" => Ok (Application :: VndEpsonSsf) , "application/vnd.ericsson.quickcall" | "qcall" | "qca" => Ok (Application :: VndEricssonQuickcall) , "application/vnd.espass-espass+zip" | "espass" => Ok (Application :: VndEspassEspassZip) , "application/vnd.eszigno3+xml" | "es3" | "et3" => Ok (Application :: VndEszigno3Xml) , "application/vnd.etsi.aoc+xml" => Ok (Application :: VndEtsiAocXml) , "application/vnd.etsi.asic-s+zip" | "asics" => Ok (Application :: VndEtsiAsicSZip) , "application/vnd.etsi.asic-e+zip" | "asice" | "sce" => Ok (Application :: VndEtsiAsicEZip) , "application/vnd.etsi.cug+xml" => Ok (Application :: VndEtsiCugXml) , "application/vnd.etsi.iptvcommand+xml" => Ok (Application :: VndEtsiIptvcommandXml) , "application/vnd.etsi.iptvdiscovery+xml" => Ok (Application :: VndEtsiIptvdiscoveryXml) , "application/vnd.etsi.iptvprofile+xml" => Ok (Application :: VndEtsiIptvprofileXml) , "application/vnd.etsi.iptvsad-bc+xml" => Ok (Application :: VndEtsiIptvsadBcXml) , "application/vnd.etsi.iptvsad-cod+xml" => Ok (Application :: VndEtsiIptvsadCodXml) , "application/vnd.etsi.iptvsad-npvr+xml" => Ok (Application :: VndEtsiIptvsadNpvrXml) , "application/vnd.etsi.iptvservice+xml" => Ok (Application :: VndEtsiIptvserviceXml) , "application/vnd.etsi.iptvsync+xml" => Ok (Application :: VndEtsiIptvsyncXml) , "application/vnd.etsi.iptvueprofile+xml" => Ok (Application :: VndEtsiIptvueprofileXml) , "application/vnd.etsi.mcid+xml" => Ok (Application :: VndEtsiMcidXml) , "application/vnd.etsi.mheg5" => Ok (Application :: VndEtsiMheg5) , "application/vnd.etsi.overload-control-policy-dataset+xml" => Ok (Application :: VndEtsiOverloadControlPolicyDatasetXml) , "application/vnd.etsi.pstn+xml" => Ok (Application :: VndEtsiPstnXml) , "application/vnd.etsi.sci+xml" => Ok (Application :: VndEtsiSciXml) , "application/vnd.etsi.simservs+xml" => Ok (Application :: VndEtsiSimservsXml) , "application/vnd.etsi.timestamp-token" | "tst" => Ok (Application :: VndEtsiTimestampToken) , "application/vnd.etsi.tsl+xml" => Ok (Application :: VndEtsiTslXml) , "application/vnd.etsi.tsl.der" => Ok (Application :: VndEtsiTslDer) , "application/vnd.eu.kasparian.car+json" | "carjson" => Ok (Application :: VndEuKasparianCarJson) , "application/vnd.eudora.data" => Ok (Application :: VndEudoraData) , "application/vnd.evolv.ecig.profile" | "ecigprofile" => Ok (Application :: VndEvolvEcigProfile) , "application/vnd.evolv.ecig.settings" | "ecig" => Ok (Application :: VndEvolvEcigSettings) , "application/vnd.evolv.ecig.theme" | "ecigtheme" => Ok (Application :: VndEvolvEcigTheme) , "application/vnd.exstream-empower+zip" | "mpw" => Ok (Application :: VndExstreamEmpowerZip) , "application/vnd.exstream-package" | "pub" => Ok (Application :: VndExstreamPackage) , "application/vnd.ezpix-album" | "ez2" => Ok (Application :: VndEzpixAlbum) , "application/vnd.ezpix-package" | "ez3" => Ok (Application :: VndEzpixPackage) , "application/vnd.f-secure.mobile" => Ok (Application :: VndFSecureMobile) , "application/vnd.fastcopy-disk-image" | "dim" => Ok (Application :: VndFastcopyDiskImage) , "application/vnd.familysearch.gedcom+zip" | "gdz" => Ok (Application :: VndFamilysearchGedcomZip) , "application/vnd.fdsn.mseed" | "msd" | "mseed" => Ok (Application :: VndFdsnMseed) , "application/vnd.fdsn.seed" | "seed" | "dataless" => Ok (Application :: VndFdsnSeed) , "application/vnd.ffsns" => Ok (Application :: VndFfsns) , "application/vnd.ficlab.flb+zip" | "flb" => Ok (Application :: VndFiclabFlbZip) , "application/vnd.filmit.zfc" | "zfc" => Ok (Application :: VndFilmitZfc) , "application/vnd.fints" => Ok (Application :: VndFints) , "application/vnd.firemonkeys.cloudcell" => Ok (Application :: VndFiremonkeysCloudcell) , "application/vnd.FloGraphIt" | "gph" => Ok (Application :: VndFloGraphIt) , "application/vnd.fluxtime.clip" | "ftc" => Ok (Application :: VndFluxtimeClip) , "application/vnd.font-fontforge-sfd" | "sfd" => Ok (Application :: VndFontFontforgeSfd) , "application/vnd.framemaker" | "fm" => Ok (Application :: VndFramemaker) , "application/vnd.fsc.weblaunch" | "fsc" => Ok (Application :: VndFscWeblaunch) , "application/vnd.fujifilm.fb.docuworks" => Ok (Application :: VndFujifilmFbDocuworks) , "application/vnd.fujifilm.fb.docuworks.binder" => Ok (Application :: VndFujifilmFbDocuworksBinder) , "application/vnd.fujifilm.fb.docuworks.container" => Ok (Application :: VndFujifilmFbDocuworksContainer) , "application/vnd.fujifilm.fb.jfi+xml" => Ok (Application :: VndFujifilmFbJfiXml) , "application/vnd.fujitsu.oasys" | "oas" => Ok (Application :: VndFujitsuOasys) , "application/vnd.fujitsu.oasys2" | "oa2" => Ok (Application :: VndFujitsuOasys2) , "application/vnd.fujitsu.oasys3" | "oa3" => Ok (Application :: VndFujitsuOasys3) , "application/vnd.fujitsu.oasysgp" | "fg5" => Ok (Application :: VndFujitsuOasysgp) , "application/vnd.fujitsu.oasysprs" | "bh2" => Ok (Application :: VndFujitsuOasysprs) , "application/vnd.fujixerox.ART4" => Ok (Application :: VndFujixeroxART4) , "application/vnd.fujixerox.ART-EX" => Ok (Application :: VndFujixeroxARTEX) , "application/vnd.fujixerox.ddd" | "ddd" => Ok (Application :: VndFujixeroxDdd) , "application/vnd.fujixerox.docuworks" | "xdw" => Ok (Application :: VndFujixeroxDocuworks) , "application/vnd.fujixerox.docuworks.binder" | "xbd" => Ok (Application :: VndFujixeroxDocuworksBinder) , "application/vnd.fujixerox.docuworks.container" | "xct" => Ok (Application :: VndFujixeroxDocuworksContainer) , "application/vnd.fujixerox.HBPL" => Ok (Application :: VndFujixeroxHBPL) , "application/vnd.fut-misnet" => Ok (Application :: VndFutMisnet) , "application/vnd.futoin+cbor" => Ok (Application :: VndFutoinCbor) , "application/vnd.futoin+json" => Ok (Application :: VndFutoinJson) , "application/vnd.fuzzysheet" | "fzs" => Ok (Application :: VndFuzzysheet) , "application/vnd.genomatix.tuxedo" | "txd" => Ok (Application :: VndGenomatixTuxedo) , "application/vnd.genozip" | "genozip" => Ok (Application :: VndGenozip) , "application/vnd.gentics.grd+json" => Ok (Application :: VndGenticsGrdJson) , "application/vnd.gentoo.catmetadata+xml" => Ok (Application :: VndGentooCatmetadataXml) , "application/vnd.gentoo.ebuild" | "ebuild" => Ok (Application :: VndGentooEbuild) , "application/vnd.gentoo.eclass" | "eclass" => Ok (Application :: VndGentooEclass) , "application/vnd.gentoo.gpkg" | "gpkg.tar" => Ok (Application :: VndGentooGpkg) , "application/vnd.gentoo.manifest" => Ok (Application :: VndGentooManifest) , "application/vnd.gentoo.xpak" | "tbz2" | "xpak" => Ok (Application :: VndGentooXpak) , "application/vnd.gentoo.pkgmetadata+xml" => Ok (Application :: VndGentooPkgmetadataXml) , "application/vnd.geogebra.file" | "ggb" => Ok (Application :: VndGeogebraFile) , "application/vnd.geogebra.slides" | "ggs" => Ok (Application :: VndGeogebraSlides) , "application/vnd.geogebra.tool" | "ggt" => Ok (Application :: VndGeogebraTool) , "application/vnd.geometry-explorer" | "gex" | "gre" => Ok (Application :: VndGeometryExplorer) , "application/vnd.geonext" | "gxt" => Ok (Application :: VndGeonext) , "application/vnd.geoplan" | "g2w" => Ok (Application :: VndGeoplan) , "application/vnd.geospace" | "g3w" => Ok (Application :: VndGeospace) , "application/vnd.gerber" => Ok (Application :: VndGerber) , "application/vnd.globalplatform.card-content-mgt" => Ok (Application :: VndGlobalplatformCardContentMgt) , "application/vnd.globalplatform.card-content-mgt-response" => Ok (Application :: VndGlobalplatformCardContentMgtResponse) , "application/vnd.gnu.taler.exchange+json" => Ok (Application :: VndGnuTalerExchangeJson) , "application/vnd.gnu.taler.merchant+json" => Ok (Application :: VndGnuTalerMerchantJson) , "application/vnd.google-earth.kml+xml" | "kml" => Ok (Application :: VndGoogleEarthKmlXml) , "application/vnd.google-earth.kmz" | "kmz" => Ok (Application :: VndGoogleEarthKmz) , "application/vnd.gov.sk.e-form+xml" => Ok (Application :: VndGovSkEFormXml) , "application/vnd.gov.sk.e-form+zip" => Ok (Application :: VndGovSkEFormZip) , "application/vnd.gov.sk.xmldatacontainer+xml" => Ok (Application :: VndGovSkXmldatacontainerXml) , "application/vnd.gpxsee.map+xml" => Ok (Application :: VndGpxseeMapXml) , "application/vnd.grafeq" | "gqf" | "gqs" => Ok (Application :: VndGrafeq) , "application/vnd.gridmp" => Ok (Application :: VndGridmp) , "application/vnd.groove-account" | "gac" => Ok (Application :: VndGrooveAccount) , "application/vnd.groove-help" | "ghf" => Ok (Application :: VndGrooveHelp) , "application/vnd.groove-identity-message" | "gim" => Ok (Application :: VndGrooveIdentityMessage) , "application/vnd.groove-injector" | "grv" => Ok (Application :: VndGrooveInjector) , "application/vnd.groove-tool-message" | "gtm" => Ok (Application :: VndGrooveToolMessage) , "application/vnd.groove-tool-template" | "tpl" => Ok (Application :: VndGrooveToolTemplate) , "application/vnd.groove-vcard" | "vcg" => Ok (Application :: VndGrooveVcard) , "application/vnd.hal+json" => Ok (Application :: VndHalJson) , "application/vnd.hal+xml" | "hal" => Ok (Application :: VndHalXml) , "application/vnd.HandHeld-Entertainment+xml" | "zmm" => Ok (Application :: VndHandHeldEntertainmentXml) , "application/vnd.hbci" | "hbci" | "hbc" | "kom" | "upa" | "pkd" | "bpd" => Ok (Application :: VndHbci) , "application/vnd.hc+json" => Ok (Application :: VndHcJson) , "application/vnd.hcl-bireports" => Ok (Application :: VndHclBireports) , "application/vnd.hdt" | "hdt" => Ok (Application :: VndHdt) , "application/vnd.heroku+json" => Ok (Application :: VndHerokuJson) , "application/vnd.hhe.lesson-player" | "les" => Ok (Application :: VndHheLessonPlayer) , "application/vnd.hp-HPGL" | "hpgl" => Ok (Application :: VndHpHPGL) , "application/vnd.hp-hpid" | "hpi" | "hpid" => Ok (Application :: VndHpHpid) , "application/vnd.hp-hps" | "hps" => Ok (Application :: VndHpHps) , "application/vnd.hp-jlyt" | "jlt" => Ok (Application :: VndHpJlyt) , "application/vnd.hp-PCL" | "pcl" => Ok (Application :: VndHpPCL) , "application/vnd.hp-PCLXL" => Ok (Application :: VndHpPCLXL) , "application/vnd.hsl" | "hsl" => Ok (Application :: VndHsl) , "application/vnd.httphone" => Ok (Application :: VndHttphone) , "application/vnd.hydrostatix.sof-data" | "sfd-hdstx" => Ok (Application :: VndHydrostatixSofData) , "application/vnd.hyper-item+json" => Ok (Application :: VndHyperItemJson) , "application/vnd.hyper+json" => Ok (Application :: VndHyperJson) , "application/vnd.hyperdrive+json" => Ok (Application :: VndHyperdriveJson) , "application/vnd.hzn-3d-crossword" | "x3d" => Ok (Application :: VndHzn3DCrossword) , "application/vnd.ibm.electronic-media" | "emm" => Ok (Application :: VndIbmElectronicMedia) , "application/vnd.ibm.MiniPay" | "mpy" => Ok (Application :: VndIbmMiniPay) , "application/vnd.ibm.rights-management" | "irm" => Ok (Application :: VndIbmRightsManagement) , "application/vnd.ibm.secure-container" | "sc" => Ok (Application :: VndIbmSecureContainer) , "application/vnd.iccprofile" | "icc" | "icm" => Ok (Application :: VndIccprofile) , "application/vnd.ieee.1905" | "1905.1" => Ok (Application :: VndIeee1905) , "application/vnd.igloader" | "igl" => Ok (Application :: VndIgloader) , "application/vnd.imagemeter.folder+zip" | "imf" => Ok (Application :: VndImagemeterFolderZip) , "application/vnd.imagemeter.image+zip" | "imi" => Ok (Application :: VndImagemeterImageZip) , "application/vnd.immervision-ivp" | "ivp" => Ok (Application :: VndImmervisionIvp) , "application/vnd.immervision-ivu" | "ivu" => Ok (Application :: VndImmervisionIvu) , "application/vnd.ims.imsccv1p1" | "imscc" => Ok (Application :: VndImsImsccv1P1) , "application/vnd.ims.imsccv1p2" => Ok (Application :: VndImsImsccv1P2) , "application/vnd.ims.imsccv1p3" => Ok (Application :: VndImsImsccv1P3) , "application/vnd.ims.lis.v2.result+json" => Ok (Application :: VndImsLisV2ResultJson) , "application/vnd.ims.lti.v2.toolconsumerprofile+json" => Ok (Application :: VndImsLtiV2ToolconsumerprofileJson) , "application/vnd.ims.lti.v2.toolproxy.id+json" => Ok (Application :: VndImsLtiV2ToolproxyIdJson) , "application/vnd.ims.lti.v2.toolproxy+json" => Ok (Application :: VndImsLtiV2ToolproxyJson) , "application/vnd.ims.lti.v2.toolsettings+json" => Ok (Application :: VndImsLtiV2ToolsettingsJson) , "application/vnd.ims.lti.v2.toolsettings.simple+json" => Ok (Application :: VndImsLtiV2ToolsettingsSimpleJson) , "application/vnd.informedcontrol.rms+xml" => Ok (Application :: VndInformedcontrolRmsXml) , "application/vnd.infotech.project" => Ok (Application :: VndInfotechProject) , "application/vnd.infotech.project+xml" => Ok (Application :: VndInfotechProjectXml) , "application/vnd.innopath.wamp.notification" => Ok (Application :: VndInnopathWampNotification) , "application/vnd.insors.igm" | "igm" => Ok (Application :: VndInsorsIgm) , "application/vnd.intercon.formnet" | "xpw" | "xpx" => Ok (Application :: VndInterconFormnet) , "application/vnd.intergeo" | "i2g" => Ok (Application :: VndIntergeo) , "application/vnd.intertrust.digibox" => Ok (Application :: VndIntertrustDigibox) , "application/vnd.intertrust.nncp" => Ok (Application :: VndIntertrustNncp) , "application/vnd.intu.qbo" | "qbo" => Ok (Application :: VndIntuQbo) , "application/vnd.intu.qfx" | "qfx" => Ok (Application :: VndIntuQfx) , "application/vnd.ipfs.ipns-record" => Ok (Application :: VndIpfsIpnsRecord) , "application/vnd.ipld.car" | "car" => Ok (Application :: VndIpldCar) , "application/vnd.ipld.dag-cbor" => Ok (Application :: VndIpldDagCbor) , "application/vnd.ipld.dag-json" => Ok (Application :: VndIpldDagJson) , "application/vnd.ipld.raw" => Ok (Application :: VndIpldRaw) , "application/vnd.iptc.g2.catalogitem+xml" => Ok (Application :: VndIptcG2CatalogitemXml) , "application/vnd.iptc.g2.conceptitem+xml" => Ok (Application :: VndIptcG2ConceptitemXml) , "application/vnd.iptc.g2.knowledgeitem+xml" => Ok (Application :: VndIptcG2KnowledgeitemXml) , "application/vnd.iptc.g2.newsitem+xml" => Ok (Application :: VndIptcG2NewsitemXml) , "application/vnd.iptc.g2.newsmessage+xml" => Ok (Application :: VndIptcG2NewsmessageXml) , "application/vnd.iptc.g2.packageitem+xml" => Ok (Application :: VndIptcG2PackageitemXml) , "application/vnd.iptc.g2.planningitem+xml" => Ok (Application :: VndIptcG2PlanningitemXml) , "application/vnd.ipunplugged.rcprofile" | "rcprofile" => Ok (Application :: VndIpunpluggedRcprofile) , "application/vnd.irepository.package+xml" | "irp" => Ok (Application :: VndIrepositoryPackageXml) , "application/vnd.is-xpr" | "xpr" => Ok (Application :: VndIsXpr) , "application/vnd.isac.fcs" | "fcs" => Ok (Application :: VndIsacFcs) , "application/vnd.jam" | "jam" => Ok (Application :: VndJam) , "application/vnd.iso11783-10+zip" => Ok (Application :: VndIso1178310Zip) , "application/vnd.japannet-directory-service" => Ok (Application :: VndJapannetDirectoryService) , "application/vnd.japannet-jpnstore-wakeup" => Ok (Application :: VndJapannetJpnstoreWakeup) , "application/vnd.japannet-payment-wakeup" => Ok (Application :: VndJapannetPaymentWakeup) , "application/vnd.japannet-registration" => Ok (Application :: VndJapannetRegistration) , "application/vnd.japannet-registration-wakeup" => Ok (Application :: VndJapannetRegistrationWakeup) , "application/vnd.japannet-setstore-wakeup" => Ok (Application :: VndJapannetSetstoreWakeup) , "application/vnd.japannet-verification" => Ok (Application :: VndJapannetVerification) , "application/vnd.japannet-verification-wakeup" => Ok (Application :: VndJapannetVerificationWakeup) , "application/vnd.jcp.javame.midlet-rms" | "rms" => Ok (Application :: VndJcpJavameMidletRms) , "application/vnd.jisp" | "jisp" => Ok (Application :: VndJisp) , "application/vnd.joost.joda-archive" | "joda" => Ok (Application :: VndJoostJodaArchive) , "application/vnd.jsk.isdn-ngn" => Ok (Application :: VndJskIsdnNgn) , "application/vnd.kahootz" | "ktz" | "ktr" => Ok (Application :: VndKahootz) , "application/vnd.kde.karbon" | "karbon" => Ok (Application :: VndKdeKarbon) , "application/vnd.kde.kchart" | "chrt" => Ok (Application :: VndKdeKchart) , "application/vnd.kde.kformula" | "kfo" => Ok (Application :: VndKdeKformula) , "application/vnd.kde.kivio" | "flw" => Ok (Application :: VndKdeKivio) , "application/vnd.kde.kontour" | "kon" => Ok (Application :: VndKdeKontour) , "application/vnd.kde.kpresenter" | "kpr" | "kpt" => Ok (Application :: VndKdeKpresenter) , "application/vnd.kde.kspread" | "ksp" => Ok (Application :: VndKdeKspread) , "application/vnd.kde.kword" | "kwd" | "kwt" => Ok (Application :: VndKdeKword) , "application/vnd.kenameaapp" | "htke" => Ok (Application :: VndKenameaapp) , "application/vnd.kidspiration" | "kia" => Ok (Application :: VndKidspiration) , "application/vnd.Kinar" | "kne" | "knp" | "sdf" => Ok (Application :: VndKinar) , "application/vnd.koan" | "skp" | "skd" | "skm" | "skt" => Ok (Application :: VndKoan) , "application/vnd.kodak-descriptor" | "sse" => Ok (Application :: VndKodakDescriptor) , "application/vnd.las" | "las" => Ok (Application :: VndLas) , "application/vnd.las.las+json" | "lasjson" => Ok (Application :: VndLasLasJson) , "application/vnd.las.las+xml" | "lasxml" => Ok (Application :: VndLasLasXml) , "application/vnd.laszip" => Ok (Application :: VndLaszip) , "application/vnd.leap+json" => Ok (Application :: VndLeapJson) , "application/vnd.liberty-request+xml" => Ok (Application :: VndLibertyRequestXml) , "application/vnd.llamagraphics.life-balance.desktop" | "lbd" => Ok (Application :: VndLlamagraphicsLifeBalanceDesktop) , "application/vnd.llamagraphics.life-balance.exchange+xml" | "lbe" => Ok (Application :: VndLlamagraphicsLifeBalanceExchangeXml) , "application/vnd.logipipe.circuit+zip" | "lcs" | "lca" => Ok (Application :: VndLogipipeCircuitZip) , "application/vnd.loom" | "loom" => Ok (Application :: VndLoom) , "application/vnd.lotus-1-2-3" | "123" | "wk4" | "wk3" | "wk1" => Ok (Application :: VndLotus123) , "application/vnd.lotus-approach" | "apr" | "vew" => Ok (Application :: VndLotusApproach) , "application/vnd.lotus-freelance" | "prz" | "pre" => Ok (Application :: VndLotusFreelance) , "application/vnd.lotus-notes" | "nsf" | "ntf" | "ndl" | "ns4" | "ns3" | "ns2" | "nsh" | "nsg" => Ok (Application :: VndLotusNotes) , "application/vnd.lotus-organizer" | "or3" | "or2" | "org" => Ok (Application :: VndLotusOrganizer) , "application/vnd.lotus-screencam" | "scm" => Ok (Application :: VndLotusScreencam) , "application/vnd.lotus-wordpro" | "lwp" | "sam" => Ok (Application :: VndLotusWordpro) , "application/vnd.macports.portpkg" | "portpkg" => Ok (Application :: VndMacportsPortpkg) , "application/vnd.mapbox-vector-tile" | "mvt" => Ok (Application :: VndMapboxVectorTile) , "application/vnd.marlin.drm.actiontoken+xml" => Ok (Application :: VndMarlinDrmActiontokenXml) , "application/vnd.marlin.drm.conftoken+xml" => Ok (Application :: VndMarlinDrmConftokenXml) , "application/vnd.marlin.drm.license+xml" => Ok (Application :: VndMarlinDrmLicenseXml) , "application/vnd.marlin.drm.mdcf" | "mdc" => Ok (Application :: VndMarlinDrmMdcf) , "application/vnd.mason+json" => Ok (Application :: VndMasonJson) , "application/vnd.maxar.archive.3tz+zip" | "3tz" => Ok (Application :: VndMaxarArchive3TzZip) , "application/vnd.maxmind.maxmind-db" | "mmdb" => Ok (Application :: VndMaxmindMaxmindDb) , "application/vnd.mcd" | "mcd" => Ok (Application :: VndMcd) , "application/vnd.mdl" | "mdl" => Ok (Application :: VndMdl) , "application/vnd.mdl-mbsdf" | "mbsdf" => Ok (Application :: VndMdlMbsdf) , "application/vnd.medcalcdata" | "mc1" => Ok (Application :: VndMedcalcdata) , "application/vnd.mediastation.cdkey" | "cdkey" => Ok (Application :: VndMediastationCdkey) , "application/vnd.medicalholodeck.recordxr" | "rxr" => Ok (Application :: VndMedicalholodeckRecordxr) , "application/vnd.meridian-slingshot" => Ok (Application :: VndMeridianSlingshot) , "application/vnd.MFER" | "mwf" => Ok (Application :: VndMFER) , "application/vnd.mfmp" | "mfm" => Ok (Application :: VndMfmp) , "application/vnd.micro+json" => Ok (Application :: VndMicroJson) , "application/vnd.micrografx.flo" | "flo" => Ok (Application :: VndMicrografxFlo) , "application/vnd.micrografx.igx" | "igx" => Ok (Application :: VndMicrografxIgx) , "application/vnd.microsoft.portable-executable" => Ok (Application :: VndMicrosoftPortableExecutable) , "application/vnd.microsoft.windows.thumbnail-cache" => Ok (Application :: VndMicrosoftWindowsThumbnailCache) , "application/vnd.miele+json" => Ok (Application :: VndMieleJson) , "application/vnd.mif" | "mif" => Ok (Application :: VndMif) , "application/vnd.minisoft-hp3000-save" => Ok (Application :: VndMinisoftHp3000Save) , "application/vnd.mitsubishi.misty-guard.trustweb" => Ok (Application :: VndMitsubishiMistyGuardTrustweb) , "application/vnd.Mobius.DAF" | "daf" => Ok (Application :: VndMobiusDAF) , "application/vnd.Mobius.DIS" | "dis" => Ok (Application :: VndMobiusDIS) , "application/vnd.Mobius.MBK" | "mbk" => Ok (Application :: VndMobiusMBK) , "application/vnd.Mobius.MQY" | "mqy" => Ok (Application :: VndMobiusMQY) , "application/vnd.Mobius.MSL" | "msl" => Ok (Application :: VndMobiusMSL) , "application/vnd.Mobius.PLC" | "plc" => Ok (Application :: VndMobiusPLC) , "application/vnd.Mobius.TXF" | "txf" => Ok (Application :: VndMobiusTXF) , "application/vnd.modl" | "modl" => Ok (Application :: VndModl) , "application/vnd.mophun.application" | "mpn" => Ok (Application :: VndMophunApplication) , "application/vnd.mophun.certificate" | "mpc" => Ok (Application :: VndMophunCertificate) , "application/vnd.motorola.flexsuite" => Ok (Application :: VndMotorolaFlexsuite) , "application/vnd.motorola.flexsuite.adsi" => Ok (Application :: VndMotorolaFlexsuiteAdsi) , "application/vnd.motorola.flexsuite.fis" => Ok (Application :: VndMotorolaFlexsuiteFis) , "application/vnd.motorola.flexsuite.gotap" => Ok (Application :: VndMotorolaFlexsuiteGotap) , "application/vnd.motorola.flexsuite.kmr" => Ok (Application :: VndMotorolaFlexsuiteKmr) , "application/vnd.motorola.flexsuite.ttc" => Ok (Application :: VndMotorolaFlexsuiteTtc) , "application/vnd.motorola.flexsuite.wem" => Ok (Application :: VndMotorolaFlexsuiteWem) , "application/vnd.motorola.iprm" => Ok (Application :: VndMotorolaIprm) , "application/vnd.mozilla.xul+xml" | "xul" => Ok (Application :: VndMozillaXulXml) , "application/vnd.ms-artgalry" | "cil" => Ok (Application :: VndMsArtgalry) , "application/vnd.ms-asf" | "asf" => Ok (Application :: VndMsAsf) , "application/vnd.ms-cab-compressed" | "cab" => Ok (Application :: VndMsCabCompressed) , "application/vnd.ms-3mfdocument" | "3mf" => Ok (Application :: VndMs3Mfdocument) , "application/vnd.ms-excel" | "xls" | "xlm" | "xla" | "xlc" | "xlt" | "xlw" => Ok (Application :: VndMsExcel) , "application/vnd.ms-excel.addin.macroEnabled.12" | "xlam" => Ok (Application :: VndMsExcelAddinMacroEnabled12) , "application/vnd.ms-excel.sheet.binary.macroEnabled.12" | "xlsb" => Ok (Application :: VndMsExcelSheetBinaryMacroEnabled12) , "application/vnd.ms-excel.sheet.macroEnabled.12" | "xlsm" => Ok (Application :: VndMsExcelSheetMacroEnabled12) , "application/vnd.ms-excel.template.macroEnabled.12" | "xltm" => Ok (Application :: VndMsExcelTemplateMacroEnabled12) , "application/vnd.ms-fontobject" | "eot" => Ok (Application :: VndMsFontobject) , "application/vnd.ms-htmlhelp" | "chm" => Ok (Application :: VndMsHtmlhelp) , "application/vnd.ms-ims" | "ims" => Ok (Application :: VndMsIms) , "application/vnd.ms-lrm" | "lrm" => Ok (Application :: VndMsLrm) , "application/vnd.ms-office.activeX+xml" => Ok (Application :: VndMsOfficeActiveXXml) , "application/vnd.ms-officetheme" | "thmx" => Ok (Application :: VndMsOfficetheme) , "application/vnd.ms-playready.initiator+xml" => Ok (Application :: VndMsPlayreadyInitiatorXml) , "application/vnd.ms-powerpoint" | "ppt" | "pps" | "pot" => Ok (Application :: VndMsPowerpoint) , "application/vnd.ms-powerpoint.addin.macroEnabled.12" | "ppam" => Ok (Application :: VndMsPowerpointAddinMacroEnabled12) , "application/vnd.ms-powerpoint.presentation.macroEnabled.12" | "pptm" => Ok (Application :: VndMsPowerpointPresentationMacroEnabled12) , "application/vnd.ms-powerpoint.slide.macroEnabled.12" | "sldm" => Ok (Application :: VndMsPowerpointSlideMacroEnabled12) , "application/vnd.ms-powerpoint.slideshow.macroEnabled.12" | "ppsm" => Ok (Application :: VndMsPowerpointSlideshowMacroEnabled12) , "application/vnd.ms-powerpoint.template.macroEnabled.12" | "potm" => Ok (Application :: VndMsPowerpointTemplateMacroEnabled12) , "application/vnd.ms-PrintDeviceCapabilities+xml" => Ok (Application :: VndMsPrintDeviceCapabilitiesXml) , "application/vnd.ms-PrintSchemaTicket+xml" => Ok (Application :: VndMsPrintSchemaTicketXml) , "application/vnd.ms-project" | "mpp" | "mpt" => Ok (Application :: VndMsProject) , "application/vnd.ms-tnef" | "tnef" | "tnf" => Ok (Application :: VndMsTnef) , "application/vnd.ms-windows.devicepairing" => Ok (Application :: VndMsWindowsDevicepairing) , "application/vnd.ms-windows.nwprinting.oob" => Ok (Application :: VndMsWindowsNwprintingOob) , "application/vnd.ms-windows.printerpairing" => Ok (Application :: VndMsWindowsPrinterpairing) , "application/vnd.ms-windows.wsd.oob" => Ok (Application :: VndMsWindowsWsdOob) , "application/vnd.ms-wmdrm.lic-chlg-req" => Ok (Application :: VndMsWmdrmLicChlgReq) , "application/vnd.ms-wmdrm.lic-resp" => Ok (Application :: VndMsWmdrmLicResp) , "application/vnd.ms-wmdrm.meter-chlg-req" => Ok (Application :: VndMsWmdrmMeterChlgReq) , "application/vnd.ms-wmdrm.meter-resp" => Ok (Application :: VndMsWmdrmMeterResp) , "application/vnd.ms-word.document.macroEnabled.12" | "docm" => Ok (Application :: VndMsWordDocumentMacroEnabled12) , "application/vnd.ms-word.template.macroEnabled.12" | "dotm" => Ok (Application :: VndMsWordTemplateMacroEnabled12) , "application/vnd.ms-works" | "wcm" | "wdb" | "wks" | "wps" => Ok (Application :: VndMsWorks) , "application/vnd.ms-wpl" | "wpl" => Ok (Application :: VndMsWpl) , "application/vnd.ms-xpsdocument" | "xps" => Ok (Application :: VndMsXpsdocument) , "application/vnd.msa-disk-image" | "msa" => Ok (Application :: VndMsaDiskImage) , "application/vnd.mseq" | "mseq" => Ok (Application :: VndMseq) , "application/vnd.msign" => Ok (Application :: VndMsign) , "application/vnd.multiad.creator" | "crtr" => Ok (Application :: VndMultiadCreator) , "application/vnd.multiad.creator.cif" | "cif" => Ok (Application :: VndMultiadCreatorCif) , "application/vnd.musician" | "mus" => Ok (Application :: VndMusician) , "application/vnd.music-niff" => Ok (Application :: VndMusicNiff) , "application/vnd.muvee.style" | "msty" => Ok (Application :: VndMuveeStyle) , "application/vnd.mynfc" | "taglet" => Ok (Application :: VndMynfc) , "application/vnd.nacamar.ybrid+json" => Ok (Application :: VndNacamarYbridJson) , "application/vnd.ncd.control" => Ok (Application :: VndNcdControl) , "application/vnd.ncd.reference" => Ok (Application :: VndNcdReference) , "application/vnd.nearst.inv+json" => Ok (Application :: VndNearstInvJson) , "application/vnd.nebumind.line" | "nebul" | "line" => Ok (Application :: VndNebumindLine) , "application/vnd.nervana" | "entity" | "request" | "bkm" | "kcm" => Ok (Application :: VndNervana) , "application/vnd.netfpx" => Ok (Application :: VndNetfpx) , "application/vnd.neurolanguage.nlu" | "nlu" => Ok (Application :: VndNeurolanguageNlu) , "application/vnd.nimn" | "nimn" => Ok (Application :: VndNimn) , "application/vnd.nintendo.snes.rom" | "sfc" | "smc" => Ok (Application :: VndNintendoSnesRom) , "application/vnd.nintendo.nitro.rom" | "nds" => Ok (Application :: VndNintendoNitroRom) , "application/vnd.nitf" | "nitf" => Ok (Application :: VndNitf) , "application/vnd.noblenet-directory" | "nnd" => Ok (Application :: VndNoblenetDirectory) , "application/vnd.noblenet-sealer" | "nns" => Ok (Application :: VndNoblenetSealer) , "application/vnd.noblenet-web" | "nnw" => Ok (Application :: VndNoblenetWeb) , "application/vnd.nokia.catalogs" => Ok (Application :: VndNokiaCatalogs) , "application/vnd.nokia.conml+wbxml" => Ok (Application :: VndNokiaConmlWbxml) , "application/vnd.nokia.conml+xml" => Ok (Application :: VndNokiaConmlXml) , "application/vnd.nokia.iptv.config+xml" => Ok (Application :: VndNokiaIptvConfigXml) , "application/vnd.nokia.iSDS-radio-presets" => Ok (Application :: VndNokiaISDSRadioPresets) , "application/vnd.nokia.landmark+wbxml" => Ok (Application :: VndNokiaLandmarkWbxml) , "application/vnd.nokia.landmark+xml" => Ok (Application :: VndNokiaLandmarkXml) , "application/vnd.nokia.landmarkcollection+xml" => Ok (Application :: VndNokiaLandmarkcollectionXml) , "application/vnd.nokia.ncd" => Ok (Application :: VndNokiaNcd) , "application/vnd.nokia.n-gage.ac+xml" | "ac" => Ok (Application :: VndNokiaNGageAcXml) , "application/vnd.nokia.n-gage.data" | "ngdat" => Ok (Application :: VndNokiaNGageData) , "application/vnd.nokia.pcd+wbxml" => Ok (Application :: VndNokiaPcdWbxml) , "application/vnd.nokia.pcd+xml" => Ok (Application :: VndNokiaPcdXml) , "application/vnd.nokia.radio-preset" | "rpst" => Ok (Application :: VndNokiaRadioPreset) , "application/vnd.nokia.radio-presets" | "rpss" => Ok (Application :: VndNokiaRadioPresets) , "application/vnd.novadigm.EDM" | "edm" => Ok (Application :: VndNovadigmEDM) , "application/vnd.novadigm.EDX" | "edx" => Ok (Application :: VndNovadigmEDX) , "application/vnd.novadigm.EXT" | "ext" => Ok (Application :: VndNovadigmEXT) , "application/vnd.ntt-local.content-share" => Ok (Application :: VndNttLocalContentShare) , "application/vnd.ntt-local.file-transfer" => Ok (Application :: VndNttLocalFileTransfer) , "application/vnd.ntt-local.ogw_remote-access" => Ok (Application :: VndNttLocalOgwRemoteAccess) , "application/vnd.ntt-local.sip-ta_remote" => Ok (Application :: VndNttLocalSipTaRemote) , "application/vnd.ntt-local.sip-ta_tcp_stream" => Ok (Application :: VndNttLocalSipTaTcpStream) , "application/vnd.oasis.opendocument.base" | "odb" => Ok (Application :: VndOasisOpendocumentBase) , "application/vnd.oasis.opendocument.chart" | "odc" => Ok (Application :: VndOasisOpendocumentChart) , "application/vnd.oasis.opendocument.chart-template" | "otc" => Ok (Application :: VndOasisOpendocumentChartTemplate) , "application/vnd.oasis.opendocument.formula" | "odf" => Ok (Application :: VndOasisOpendocumentFormula) , "application/vnd.oasis.opendocument.formula-template" => Ok (Application :: VndOasisOpendocumentFormulaTemplate) , "application/vnd.oasis.opendocument.graphics" | "odg" => Ok (Application :: VndOasisOpendocumentGraphics) , "application/vnd.oasis.opendocument.graphics-template" | "otg" => Ok (Application :: VndOasisOpendocumentGraphicsTemplate) , "application/vnd.oasis.opendocument.image" | "odi" => Ok (Application :: VndOasisOpendocumentImage) , "application/vnd.oasis.opendocument.image-template" | "oti" => Ok (Application :: VndOasisOpendocumentImageTemplate) , "application/vnd.oasis.opendocument.presentation" | "odp" => Ok (Application :: VndOasisOpendocumentPresentation) , "application/vnd.oasis.opendocument.presentation-template" | "otp" => Ok (Application :: VndOasisOpendocumentPresentationTemplate) , "application/vnd.oasis.opendocument.spreadsheet" | "ods" => Ok (Application :: VndOasisOpendocumentSpreadsheet) , "application/vnd.oasis.opendocument.spreadsheet-template" | "ots" => Ok (Application :: VndOasisOpendocumentSpreadsheetTemplate) , "application/vnd.oasis.opendocument.text" | "odt" => Ok (Application :: VndOasisOpendocumentText) , "application/vnd.oasis.opendocument.text-master" | "odm" => Ok (Application :: VndOasisOpendocumentTextMaster) , "application/vnd.oasis.opendocument.text-master-template" | "otm" => Ok (Application :: VndOasisOpendocumentTextMasterTemplate) , "application/vnd.oasis.opendocument.text-template" | "ott" => Ok (Application :: VndOasisOpendocumentTextTemplate) , "application/vnd.oasis.opendocument.text-web" | "oth" => Ok (Application :: VndOasisOpendocumentTextWeb) , "application/vnd.obn" => Ok (Application :: VndObn) , "application/vnd.ocf+cbor" => Ok (Application :: VndOcfCbor) , "application/vnd.oci.image.manifest.v1+json" => Ok (Application :: VndOciImageManifestV1Json) , "application/vnd.oftn.l10n+json" => Ok (Application :: VndOftnL10NJson) , "application/vnd.oipf.contentaccessdownload+xml" => Ok (Application :: VndOipfContentaccessdownloadXml) , "application/vnd.oipf.contentaccessstreaming+xml" => Ok (Application :: VndOipfContentaccessstreamingXml) , "application/vnd.oipf.cspg-hexbinary" => Ok (Application :: VndOipfCspgHexbinary) , "application/vnd.oipf.dae.svg+xml" => Ok (Application :: VndOipfDaeSvgXml) , "application/vnd.oipf.dae.xhtml+xml" => Ok (Application :: VndOipfDaeXhtmlXml) , "application/vnd.oipf.mippvcontrolmessage+xml" => Ok (Application :: VndOipfMippvcontrolmessageXml) , "application/vnd.oipf.pae.gem" => Ok (Application :: VndOipfPaeGem) , "application/vnd.oipf.spdiscovery+xml" => Ok (Application :: VndOipfSpdiscoveryXml) , "application/vnd.oipf.spdlist+xml" => Ok (Application :: VndOipfSpdlistXml) , "application/vnd.oipf.ueprofile+xml" => Ok (Application :: VndOipfUeprofileXml) , "application/vnd.oipf.userprofile+xml" => Ok (Application :: VndOipfUserprofileXml) , "application/vnd.olpc-sugar" | "xo" => Ok (Application :: VndOlpcSugar) , "application/vnd.oma.bcast.associated-procedure-parameter+xml" => Ok (Application :: VndOmaBcastAssociatedProcedureParameterXml) , "application/vnd.oma.bcast.drm-trigger+xml" => Ok (Application :: VndOmaBcastDrmTriggerXml) , "application/vnd.oma.bcast.imd+xml" => Ok (Application :: VndOmaBcastImdXml) , "application/vnd.oma.bcast.ltkm" => Ok (Application :: VndOmaBcastLtkm) , "application/vnd.oma.bcast.notification+xml" => Ok (Application :: VndOmaBcastNotificationXml) , "application/vnd.oma.bcast.provisioningtrigger" => Ok (Application :: VndOmaBcastProvisioningtrigger) , "application/vnd.oma.bcast.sgboot" => Ok (Application :: VndOmaBcastSgboot) , "application/vnd.oma.bcast.sgdd+xml" => Ok (Application :: VndOmaBcastSgddXml) , "application/vnd.oma.bcast.sgdu" => Ok (Application :: VndOmaBcastSgdu) , "application/vnd.oma.bcast.simple-symbol-container" => Ok (Application :: VndOmaBcastSimpleSymbolContainer) , "application/vnd.oma.bcast.smartcard-trigger+xml" => Ok (Application :: VndOmaBcastSmartcardTriggerXml) , "application/vnd.oma.bcast.sprov+xml" => Ok (Application :: VndOmaBcastSprovXml) , "application/vnd.oma.bcast.stkm" => Ok (Application :: VndOmaBcastStkm) , "application/vnd.oma.cab-address-book+xml" => Ok (Application :: VndOmaCabAddressBookXml) , "application/vnd.oma.cab-feature-handler+xml" => Ok (Application :: VndOmaCabFeatureHandlerXml) , "application/vnd.oma.cab-pcc+xml" => Ok (Application :: VndOmaCabPccXml) , "application/vnd.oma.cab-subs-invite+xml" => Ok (Application :: VndOmaCabSubsInviteXml) , "application/vnd.oma.cab-user-prefs+xml" => Ok (Application :: VndOmaCabUserPrefsXml) , "application/vnd.oma.dcd" => Ok (Application :: VndOmaDcd) , "application/vnd.oma.dcdc" => Ok (Application :: VndOmaDcdc) , "application/vnd.oma.dd2+xml" | "dd2" => Ok (Application :: VndOmaDd2Xml) , "application/vnd.oma.drm.risd+xml" => Ok (Application :: VndOmaDrmRisdXml) , "application/vnd.oma.group-usage-list+xml" => Ok (Application :: VndOmaGroupUsageListXml) , "application/vnd.oma.lwm2m+cbor" => Ok (Application :: VndOmaLwm2MCbor) , "application/vnd.oma.lwm2m+json" => Ok (Application :: VndOmaLwm2MJson) , "application/vnd.oma.lwm2m+tlv" => Ok (Application :: VndOmaLwm2MTlv) , "application/vnd.oma.pal+xml" => Ok (Application :: VndOmaPalXml) , "application/vnd.oma.poc.detailed-progress-report+xml" => Ok (Application :: VndOmaPocDetailedProgressReportXml) , "application/vnd.oma.poc.final-report+xml" => Ok (Application :: VndOmaPocFinalReportXml) , "application/vnd.oma.poc.groups+xml" => Ok (Application :: VndOmaPocGroupsXml) , "application/vnd.oma.poc.invocation-descriptor+xml" => Ok (Application :: VndOmaPocInvocationDescriptorXml) , "application/vnd.oma.poc.optimized-progress-report+xml" => Ok (Application :: VndOmaPocOptimizedProgressReportXml) , "application/vnd.oma.push" => Ok (Application :: VndOmaPush) , "application/vnd.oma.scidm.messages+xml" => Ok (Application :: VndOmaScidmMessagesXml) , "application/vnd.oma.xcap-directory+xml" => Ok (Application :: VndOmaXcapDirectoryXml) , "application/vnd.omads-email+xml" => Ok (Application :: VndOmadsEmailXml) , "application/vnd.omads-file+xml" => Ok (Application :: VndOmadsFileXml) , "application/vnd.omads-folder+xml" => Ok (Application :: VndOmadsFolderXml) , "application/vnd.omaloc-supl-init" => Ok (Application :: VndOmalocSuplInit) , "application/vnd.oma-scws-config" => Ok (Application :: VndOmaScwsConfig) , "application/vnd.oma-scws-http-request" => Ok (Application :: VndOmaScwsHttpRequest) , "application/vnd.oma-scws-http-response" => Ok (Application :: VndOmaScwsHttpResponse) , "application/vnd.onepager" | "tam" => Ok (Application :: VndOnepager) , "application/vnd.onepagertamp" | "tamp" => Ok (Application :: VndOnepagertamp) , "application/vnd.onepagertamx" | "tamx" => Ok (Application :: VndOnepagertamx) , "application/vnd.onepagertat" | "tat" => Ok (Application :: VndOnepagertat) , "application/vnd.onepagertatp" | "tatp" => Ok (Application :: VndOnepagertatp) , "application/vnd.onepagertatx" | "tatx" => Ok (Application :: VndOnepagertatx) , "application/vnd.onvif.metadata" => Ok (Application :: VndOnvifMetadata) , "application/vnd.openblox.game-binary" | "obg" => Ok (Application :: VndOpenbloxGameBinary) , "application/vnd.openblox.game+xml" | "obgx" => Ok (Application :: VndOpenbloxGameXml) , "application/vnd.openeye.oeb" | "oeb" => Ok (Application :: VndOpeneyeOeb) , "application/vnd.openstreetmap.data+xml" | "osm" => Ok (Application :: VndOpenstreetmapDataXml) , "application/vnd.opentimestamps.ots" => Ok (Application :: VndOpentimestampsOts) , "application/vnd.openxmlformats-officedocument.custom-properties+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentCustomPropertiesXml) , "application/vnd.openxmlformats-officedocument.customXmlProperties+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentCustomXmlPropertiesXml) , "application/vnd.openxmlformats-officedocument.drawing+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingXml) , "application/vnd.openxmlformats-officedocument.drawingml.chart+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartXml) , "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartshapesXml) , "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramColorsXml) , "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramDataXml) , "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramLayoutXml) , "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramStyleXml) , "application/vnd.openxmlformats-officedocument.extended-properties+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentExtendedPropertiesXml) , "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentAuthorsXml) , "application/vnd.openxmlformats-officedocument.presentationml.comments+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentsXml) , "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlHandoutMasterXml) , "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesMasterXml) , "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesSlideXml) , "application/vnd.openxmlformats-officedocument.presentationml.presentation" | "pptx" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentation) , "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentationMainXml) , "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresPropsXml) , "application/vnd.openxmlformats-officedocument.presentationml.slide" | "sldx" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlide) , "application/vnd.openxmlformats-officedocument.presentationml.slide+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideXml) , "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideLayoutXml) , "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideMasterXml) , "application/vnd.openxmlformats-officedocument.presentationml.slideshow" | "ppsx" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshow) , "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshowMainXml) , "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideUpdateInfoXml) , "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlTableStylesXml) , "application/vnd.openxmlformats-officedocument.presentationml.tags+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlTagsXml) , "application/vnd.openxmlformats-officedocument.presentationml.template" | "potx" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplate) , "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplateMainXml) , "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentPresentationmlViewPropsXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCalcChainXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlChartsheetXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCommentsXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlConnectionsXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlDialogsheetXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlExternalLinkXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheDefinitionXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheRecordsXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotTableXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlQueryTableXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionHeadersXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionLogXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSharedStringsXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" | "xlsx" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheet) , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMainXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMetadataXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlStylesXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableSingleCellsXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.template" | "xltx" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplate) , "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplateMainXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlUserNamesXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlVolatileDependenciesXml) , "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlWorksheetXml) , "application/vnd.openxmlformats-officedocument.theme+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentThemeXml) , "application/vnd.openxmlformats-officedocument.themeOverride+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentThemeOverrideXml) , "application/vnd.openxmlformats-officedocument.vmlDrawing" => Ok (Application :: VndOpenxmlformatsOfficedocumentVmlDrawing) , "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlCommentsXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.document" | "docx" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocument) , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentGlossaryXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentMainXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlEndnotesXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFontTableXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFooterXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFootnotesXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlNumberingXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlSettingsXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlStylesXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.template" | "dotx" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplate) , "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplateMainXml) , "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml" => Ok (Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlWebSettingsXml) , "application/vnd.openxmlformats-package.core-properties+xml" => Ok (Application :: VndOpenxmlformatsPackageCorePropertiesXml) , "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml" => Ok (Application :: VndOpenxmlformatsPackageDigitalSignatureXmlsignatureXml) , "application/vnd.openxmlformats-package.relationships+xml" => Ok (Application :: VndOpenxmlformatsPackageRelationshipsXml) , "application/vnd.oracle.resource+json" => Ok (Application :: VndOracleResourceJson) , "application/vnd.orange.indata" => Ok (Application :: VndOrangeIndata) , "application/vnd.osa.netdeploy" | "ndc" => Ok (Application :: VndOsaNetdeploy) , "application/vnd.osgeo.mapguide.package" | "mgp" => Ok (Application :: VndOsgeoMapguidePackage) , "application/vnd.osgi.bundle" => Ok (Application :: VndOsgiBundle) , "application/vnd.osgi.dp" | "dp" => Ok (Application :: VndOsgiDp) , "application/vnd.osgi.subsystem" | "esa" => Ok (Application :: VndOsgiSubsystem) , "application/vnd.otps.ct-kip+xml" => Ok (Application :: VndOtpsCtKipXml) , "application/vnd.oxli.countgraph" | "oxlicg" => Ok (Application :: VndOxliCountgraph) , "application/vnd.pagerduty+json" => Ok (Application :: VndPagerdutyJson) , "application/vnd.palm" | "prc" | "pdb" | "pqa" | "oprc" => Ok (Application :: VndPalm) , "application/vnd.panoply" | "plp" => Ok (Application :: VndPanoply) , "application/vnd.paos.xml" => Ok (Application :: VndPaosXml) , "application/vnd.patentdive" | "dive" => Ok (Application :: VndPatentdive) , "application/vnd.patientecommsdoc" => Ok (Application :: VndPatientecommsdoc) , "application/vnd.pawaafile" | "paw" => Ok (Application :: VndPawaafile) , "application/vnd.pcos" => Ok (Application :: VndPcos) , "application/vnd.pg.format" | "str" => Ok (Application :: VndPgFormat) , "application/vnd.pg.osasli" | "ei6" => Ok (Application :: VndPgOsasli) , "application/vnd.piaccess.application-licence" | "pil" => Ok (Application :: VndPiaccessApplicationLicence) , "application/vnd.picsel" | "efif" => Ok (Application :: VndPicsel) , "application/vnd.pmi.widget" | "wg" => Ok (Application :: VndPmiWidget) , "application/vnd.poc.group-advertisement+xml" => Ok (Application :: VndPocGroupAdvertisementXml) , "application/vnd.pocketlearn" | "plf" => Ok (Application :: VndPocketlearn) , "application/vnd.powerbuilder6" | "pbd" => Ok (Application :: VndPowerbuilder6) , "application/vnd.powerbuilder6-s" => Ok (Application :: VndPowerbuilder6S) , "application/vnd.powerbuilder7" => Ok (Application :: VndPowerbuilder7) , "application/vnd.powerbuilder75" => Ok (Application :: VndPowerbuilder75) , "application/vnd.powerbuilder75-s" => Ok (Application :: VndPowerbuilder75S) , "application/vnd.powerbuilder7-s" => Ok (Application :: VndPowerbuilder7S) , "application/vnd.preminet" | "preminet" => Ok (Application :: VndPreminet) , "application/vnd.previewsystems.box" | "box" | "vbox" => Ok (Application :: VndPreviewsystemsBox) , "application/vnd.proteus.magazine" | "mgz" => Ok (Application :: VndProteusMagazine) , "application/vnd.psfs" | "psfs" => Ok (Application :: VndPsfs) , "application/vnd.pt.mundusmundi" => Ok (Application :: VndPtMundusmundi) , "application/vnd.publishare-delta-tree" | "qps" => Ok (Application :: VndPublishareDeltaTree) , "application/vnd.pvi.ptid1" | "ptid" => Ok (Application :: VndPviPtid1) , "application/vnd.pwg-multiplexed" => Ok (Application :: VndPwgMultiplexed) , "application/vnd.pwg-xhtml-print+xml" => Ok (Application :: VndPwgXhtmlPrintXml) , "application/vnd.qualcomm.brew-app-res" | "bar" => Ok (Application :: VndQualcommBrewAppRes) , "application/vnd.quarantainenet" => Ok (Application :: VndQuarantainenet) , "application/vnd.Quark.QuarkXPress" | "qxd" | "qxt" | "qwd" | "qwt" | "qxl" | "qxb" => Ok (Application :: VndQuarkQuarkXPress) , "application/vnd.quobject-quoxdocument" | "quox" | "quiz" => Ok (Application :: VndQuobjectQuoxdocument) , "application/vnd.radisys.moml+xml" => Ok (Application :: VndRadisysMomlXml) , "application/vnd.radisys.msml-audit-conf+xml" => Ok (Application :: VndRadisysMsmlAuditConfXml) , "application/vnd.radisys.msml-audit-conn+xml" => Ok (Application :: VndRadisysMsmlAuditConnXml) , "application/vnd.radisys.msml-audit-dialog+xml" => Ok (Application :: VndRadisysMsmlAuditDialogXml) , "application/vnd.radisys.msml-audit-stream+xml" => Ok (Application :: VndRadisysMsmlAuditStreamXml) , "application/vnd.radisys.msml-audit+xml" => Ok (Application :: VndRadisysMsmlAuditXml) , "application/vnd.radisys.msml-conf+xml" => Ok (Application :: VndRadisysMsmlConfXml) , "application/vnd.radisys.msml-dialog-base+xml" => Ok (Application :: VndRadisysMsmlDialogBaseXml) , "application/vnd.radisys.msml-dialog-fax-detect+xml" => Ok (Application :: VndRadisysMsmlDialogFaxDetectXml) , "application/vnd.radisys.msml-dialog-fax-sendrecv+xml" => Ok (Application :: VndRadisysMsmlDialogFaxSendrecvXml) , "application/vnd.radisys.msml-dialog-group+xml" => Ok (Application :: VndRadisysMsmlDialogGroupXml) , "application/vnd.radisys.msml-dialog-speech+xml" => Ok (Application :: VndRadisysMsmlDialogSpeechXml) , "application/vnd.radisys.msml-dialog-transform+xml" => Ok (Application :: VndRadisysMsmlDialogTransformXml) , "application/vnd.radisys.msml-dialog+xml" => Ok (Application :: VndRadisysMsmlDialogXml) , "application/vnd.radisys.msml+xml" => Ok (Application :: VndRadisysMsmlXml) , "application/vnd.rainstor.data" | "tree" => Ok (Application :: VndRainstorData) , "application/vnd.rapid" => Ok (Application :: VndRapid) , "application/vnd.rar" | "rar" => Ok (Application :: VndRar) , "application/vnd.realvnc.bed" | "bed" => Ok (Application :: VndRealvncBed) , "application/vnd.recordare.musicxml" | "mxl" => Ok (Application :: VndRecordareMusicxml) , "application/vnd.recordare.musicxml+xml" => Ok (Application :: VndRecordareMusicxmlXml) , "application/vnd.RenLearn.rlprint" => Ok (Application :: VndRenLearnRlprint) , "application/vnd.resilient.logic" | "rlm" | "reload" => Ok (Application :: VndResilientLogic) , "application/vnd.restful+json" => Ok (Application :: VndRestfulJson) , "application/vnd.rig.cryptonote" | "cryptonote" => Ok (Application :: VndRigCryptonote) , "application/vnd.route66.link66+xml" | "link66" => Ok (Application :: VndRoute66Link66Xml) , "application/vnd.rs-274x" => Ok (Application :: VndRs274X) , "application/vnd.ruckus.download" => Ok (Application :: VndRuckusDownload) , "application/vnd.s3sms" => Ok (Application :: VndS3Sms) , "application/vnd.sailingtracker.track" | "st" => Ok (Application :: VndSailingtrackerTrack) , "application/vnd.sar" | "SAR" => Ok (Application :: VndSar) , "application/vnd.sbm.cid" => Ok (Application :: VndSbmCid) , "application/vnd.sbm.mid2" => Ok (Application :: VndSbmMid2) , "application/vnd.scribus" | "scd" | "sla" | "slaz" => Ok (Application :: VndScribus) , "application/vnd.sealed.3df" | "s3df" => Ok (Application :: VndSealed3Df) , "application/vnd.sealed.csf" | "scsf" => Ok (Application :: VndSealedCsf) , "application/vnd.sealed.doc" | "sdoc" | "sdo" | "s1w" => Ok (Application :: VndSealedDoc) , "application/vnd.sealed.eml" | "seml" | "sem" => Ok (Application :: VndSealedEml) , "application/vnd.sealed.mht" | "smht" | "smh" => Ok (Application :: VndSealedMht) , "application/vnd.sealed.net" => Ok (Application :: VndSealedNet) , "application/vnd.sealed.ppt" | "sppt" | "s1p" => Ok (Application :: VndSealedPpt) , "application/vnd.sealed.tiff" | "stif" => Ok (Application :: VndSealedTiff) , "application/vnd.sealed.xls" | "sxls" | "sxl" | "s1e" => Ok (Application :: VndSealedXls) , "application/vnd.sealedmedia.softseal.html" | "stml" | "s1h" => Ok (Application :: VndSealedmediaSoftsealHtml) , "application/vnd.sealedmedia.softseal.pdf" | "spdf" | "spd" | "s1a" => Ok (Application :: VndSealedmediaSoftsealPdf) , "application/vnd.seemail" | "see" => Ok (Application :: VndSeemail) , "application/vnd.seis+json" => Ok (Application :: VndSeisJson) , "application/vnd.sema" | "sema" => Ok (Application :: VndSema) , "application/vnd.semd" | "semd" => Ok (Application :: VndSemd) , "application/vnd.semf" | "semf" => Ok (Application :: VndSemf) , "application/vnd.shade-save-file" | "ssv" => Ok (Application :: VndShadeSaveFile) , "application/vnd.shana.informed.formdata" | "ifm" => Ok (Application :: VndShanaInformedFormdata) , "application/vnd.shana.informed.formtemplate" | "itp" => Ok (Application :: VndShanaInformedFormtemplate) , "application/vnd.shana.informed.interchange" | "iif" => Ok (Application :: VndShanaInformedInterchange) , "application/vnd.shana.informed.package" | "ipk" => Ok (Application :: VndShanaInformedPackage) , "application/vnd.shootproof+json" => Ok (Application :: VndShootproofJson) , "application/vnd.shopkick+json" => Ok (Application :: VndShopkickJson) , "application/vnd.shp" | "shp" => Ok (Application :: VndShp) , "application/vnd.shx" | "shx" => Ok (Application :: VndShx) , "application/vnd.sigrok.session" | "sr" => Ok (Application :: VndSigrokSession) , "application/vnd.SimTech-MindMapper" | "twd" | "twds" => Ok (Application :: VndSimTechMindMapper) , "application/vnd.siren+json" => Ok (Application :: VndSirenJson) , "application/vnd.smaf" | "mmf" => Ok (Application :: VndSmaf) , "application/vnd.smart.notebook" | "notebook" => Ok (Application :: VndSmartNotebook) , "application/vnd.smart.teacher" | "teacher" => Ok (Application :: VndSmartTeacher) , "application/vnd.smintio.portals.archive" | "sipa" => Ok (Application :: VndSmintioPortalsArchive) , "application/vnd.snesdev-page-table" | "ptrom" | "pt" => Ok (Application :: VndSnesdevPageTable) , "application/vnd.software602.filler.form+xml" | "fo" => Ok (Application :: VndSoftware602FillerFormXml) , "application/vnd.software602.filler.form-xml-zip" | "zfo" => Ok (Application :: VndSoftware602FillerFormXmlZip) , "application/vnd.solent.sdkm+xml" | "sdkm" | "sdkd" => Ok (Application :: VndSolentSdkmXml) , "application/vnd.spotfire.dxp" | "dxp" => Ok (Application :: VndSpotfireDxp) , "application/vnd.spotfire.sfs" | "sfs" => Ok (Application :: VndSpotfireSfs) , "application/vnd.sqlite3" | "sqlite" | "sqlite3" => Ok (Application :: VndSqlite3) , "application/vnd.sss-cod" => Ok (Application :: VndSssCod) , "application/vnd.sss-dtf" => Ok (Application :: VndSssDtf) , "application/vnd.sss-ntf" => Ok (Application :: VndSssNtf) , "application/vnd.stepmania.package" | "smzip" => Ok (Application :: VndStepmaniaPackage) , "application/vnd.stepmania.stepchart" | "sm" => Ok (Application :: VndStepmaniaStepchart) , "application/vnd.street-stream" => Ok (Application :: VndStreetStream) , "application/vnd.sun.wadl+xml" | "wadl" => Ok (Application :: VndSunWadlXml) , "application/vnd.sus-calendar" | "sus" | "susp" => Ok (Application :: VndSusCalendar) , "application/vnd.svd" => Ok (Application :: VndSvd) , "application/vnd.swiftview-ics" => Ok (Application :: VndSwiftviewIcs) , "application/vnd.sybyl.mol2" | "ml2" | "mol2" | "sy2" => Ok (Application :: VndSybylMol2) , "application/vnd.sycle+xml" | "scl" => Ok (Application :: VndSycleXml) , "application/vnd.syft+json" | "syft.json" => Ok (Application :: VndSyftJson) , "application/vnd.syncml.dm.notification" => Ok (Application :: VndSyncmlDmNotification) , "application/vnd.syncml.dmddf+xml" | "ddf" => Ok (Application :: VndSyncmlDmddfXml) , "application/vnd.syncml.dmtnds+wbxml" => Ok (Application :: VndSyncmlDmtndsWbxml) , "application/vnd.syncml.dmtnds+xml" => Ok (Application :: VndSyncmlDmtndsXml) , "application/vnd.syncml.dmddf+wbxml" => Ok (Application :: VndSyncmlDmddfWbxml) , "application/vnd.syncml.dm+wbxml" | "bdm" => Ok (Application :: VndSyncmlDmWbxml) , "application/vnd.syncml.dm+xml" | "xdm" => Ok (Application :: VndSyncmlDmXml) , "application/vnd.syncml.ds.notification" => Ok (Application :: VndSyncmlDsNotification) , "application/vnd.syncml+xml" | "xsm" => Ok (Application :: VndSyncmlXml) , "application/vnd.tableschema+json" => Ok (Application :: VndTableschemaJson) , "application/vnd.tao.intent-module-archive" | "tao" => Ok (Application :: VndTaoIntentModuleArchive) , "application/vnd.tcpdump.pcap" | "pcap" | "cap" | "dmp" => Ok (Application :: VndTcpdumpPcap) , "application/vnd.think-cell.ppttc+json" | "ppttc" => Ok (Application :: VndThinkCellPpttcJson) , "application/vnd.tml" | "vfr" | "viaframe" => Ok (Application :: VndTml) , "application/vnd.tmd.mediaflex.api+xml" => Ok (Application :: VndTmdMediaflexApiXml) , "application/vnd.tmobile-livetv" | "tmo" => Ok (Application :: VndTmobileLivetv) , "application/vnd.tri.onesource" => Ok (Application :: VndTriOnesource) , "application/vnd.trid.tpt" | "tpt" => Ok (Application :: VndTridTpt) , "application/vnd.triscape.mxs" | "mxs" => Ok (Application :: VndTriscapeMxs) , "application/vnd.trueapp" | "tra" => Ok (Application :: VndTrueapp) , "application/vnd.truedoc" => Ok (Application :: VndTruedoc) , "application/vnd.ubisoft.webplayer" => Ok (Application :: VndUbisoftWebplayer) , "application/vnd.ufdl" | "ufdl" | "ufd" | "frm" => Ok (Application :: VndUfdl) , "application/vnd.uiq.theme" | "utz" => Ok (Application :: VndUiqTheme) , "application/vnd.umajin" | "umj" => Ok (Application :: VndUmajin) , "application/vnd.unity" | "unityweb" => Ok (Application :: VndUnity) , "application/vnd.uoml+xml" | "uoml" | "uo" => Ok (Application :: VndUomlXml) , "application/vnd.uplanet.alert" => Ok (Application :: VndUplanetAlert) , "application/vnd.uplanet.alert-wbxml" => Ok (Application :: VndUplanetAlertWbxml) , "application/vnd.uplanet.bearer-choice" => Ok (Application :: VndUplanetBearerChoice) , "application/vnd.uplanet.bearer-choice-wbxml" => Ok (Application :: VndUplanetBearerChoiceWbxml) , "application/vnd.uplanet.cacheop" => Ok (Application :: VndUplanetCacheop) , "application/vnd.uplanet.cacheop-wbxml" => Ok (Application :: VndUplanetCacheopWbxml) , "application/vnd.uplanet.channel" => Ok (Application :: VndUplanetChannel) , "application/vnd.uplanet.channel-wbxml" => Ok (Application :: VndUplanetChannelWbxml) , "application/vnd.uplanet.list" => Ok (Application :: VndUplanetList) , "application/vnd.uplanet.listcmd" => Ok (Application :: VndUplanetListcmd) , "application/vnd.uplanet.listcmd-wbxml" => Ok (Application :: VndUplanetListcmdWbxml) , "application/vnd.uplanet.list-wbxml" => Ok (Application :: VndUplanetListWbxml) , "application/vnd.uri-map" | "urim" | "urimap" => Ok (Application :: VndUriMap) , "application/vnd.uplanet.signal" => Ok (Application :: VndUplanetSignal) , "application/vnd.valve.source.material" | "vmt" => Ok (Application :: VndValveSourceMaterial) , "application/vnd.vcx" | "vcx" => Ok (Application :: VndVcx) , "application/vnd.vd-study" | "mxi" | "study-inter" | "model-inter" => Ok (Application :: VndVdStudy) , "application/vnd.vectorworks" | "vwx" => Ok (Application :: VndVectorworks) , "application/vnd.vel+json" => Ok (Application :: VndVelJson) , "application/vnd.verimatrix.vcas" => Ok (Application :: VndVerimatrixVcas) , "application/vnd.veritone.aion+json" | "aion" | "vtnstd" => Ok (Application :: VndVeritoneAionJson) , "application/vnd.veryant.thin" | "istc" | "isws" => Ok (Application :: VndVeryantThin) , "application/vnd.ves.encrypted" | "VES" => Ok (Application :: VndVesEncrypted) , "application/vnd.vidsoft.vidconference" | "vsc" => Ok (Application :: VndVidsoftVidconference) , "application/vnd.visio" | "vsd" | "vst" | "vsw" | "vss" => Ok (Application :: VndVisio) , "application/vnd.visionary" | "vis" => Ok (Application :: VndVisionary) , "application/vnd.vividence.scriptfile" => Ok (Application :: VndVividenceScriptfile) , "application/vnd.vsf" | "vsf" => Ok (Application :: VndVsf) , "application/vnd.wap.sic" | "sic" => Ok (Application :: VndWapSic) , "application/vnd.wap.slc" | "slc" => Ok (Application :: VndWapSlc) , "application/vnd.wap.wbxml" | "wbxml" => Ok (Application :: VndWapWbxml) , "application/vnd.wap.wmlc" | "wmlc" => Ok (Application :: VndWapWmlc) , "application/vnd.wap.wmlscriptc" | "wmlsc" => Ok (Application :: VndWapWmlscriptc) , "application/vnd.wasmflow.wafl" | "wafl" => Ok (Application :: VndWasmflowWafl) , "application/vnd.webturbo" | "wtb" => Ok (Application :: VndWebturbo) , "application/vnd.wfa.dpp" => Ok (Application :: VndWfaDpp) , "application/vnd.wfa.p2p" | "p2p" => Ok (Application :: VndWfaP2P) , "application/vnd.wfa.wsc" | "wsc" => Ok (Application :: VndWfaWsc) , "application/vnd.windows.devicepairing" => Ok (Application :: VndWindowsDevicepairing) , "application/vnd.wmc" | "wmc" => Ok (Application :: VndWmc) , "application/vnd.wmf.bootstrap" => Ok (Application :: VndWmfBootstrap) , "application/vnd.wolfram.mathematica" => Ok (Application :: VndWolframMathematica) , "application/vnd.wolfram.mathematica.package" | "m" => Ok (Application :: VndWolframMathematicaPackage) , "application/vnd.wolfram.player" | "nbp" => Ok (Application :: VndWolframPlayer) , "application/vnd.wordlift" => Ok (Application :: VndWordlift) , "application/vnd.wordperfect" | "wpd" => Ok (Application :: VndWordperfect) , "application/vnd.wqd" | "wqd" => Ok (Application :: VndWqd) , "application/vnd.wrq-hp3000-labelled" => Ok (Application :: VndWrqHp3000Labelled) , "application/vnd.wt.stf" | "stf" => Ok (Application :: VndWtStf) , "application/vnd.wv.csp+xml" => Ok (Application :: VndWvCspXml) , "application/vnd.wv.csp+wbxml" | "wv" => Ok (Application :: VndWvCspWbxml) , "application/vnd.wv.ssp+xml" => Ok (Application :: VndWvSspXml) , "application/vnd.xacml+json" => Ok (Application :: VndXacmlJson) , "application/vnd.xara" | "xar" => Ok (Application :: VndXara) , "application/vnd.xfdl" | "xfdl" | "xfd" => Ok (Application :: VndXfdl) , "application/vnd.xfdl.webform" => Ok (Application :: VndXfdlWebform) , "application/vnd.xmi+xml" => Ok (Application :: VndXmiXml) , "application/vnd.xmpie.cpkg" | "cpkg" => Ok (Application :: VndXmpieCpkg) , "application/vnd.xmpie.dpkg" | "dpkg" => Ok (Application :: VndXmpieDpkg) , "application/vnd.xmpie.plan" => Ok (Application :: VndXmpiePlan) , "application/vnd.xmpie.ppkg" | "ppkg" => Ok (Application :: VndXmpiePpkg) , "application/vnd.xmpie.xlim" | "xlim" => Ok (Application :: VndXmpieXlim) , "application/vnd.yamaha.hv-dic" | "hvd" => Ok (Application :: VndYamahaHvDic) , "application/vnd.yamaha.hv-script" | "hvs" => Ok (Application :: VndYamahaHvScript) , "application/vnd.yamaha.hv-voice" | "hvp" => Ok (Application :: VndYamahaHvVoice) , "application/vnd.yamaha.openscoreformat.osfpvg+xml" => Ok (Application :: VndYamahaOpenscoreformatOsfpvgXml) , "application/vnd.yamaha.openscoreformat" | "osf" => Ok (Application :: VndYamahaOpenscoreformat) , "application/vnd.yamaha.remote-setup" => Ok (Application :: VndYamahaRemoteSetup) , "application/vnd.yamaha.smaf-audio" | "saf" => Ok (Application :: VndYamahaSmafAudio) , "application/vnd.yamaha.smaf-phrase" | "spf" => Ok (Application :: VndYamahaSmafPhrase) , "application/vnd.yamaha.through-ngn" => Ok (Application :: VndYamahaThroughNgn) , "application/vnd.yamaha.tunnel-udpencap" => Ok (Application :: VndYamahaTunnelUdpencap) , "application/vnd.yaoweme" | "yme" => Ok (Application :: VndYaoweme) , "application/vnd.yellowriver-custom-menu" | "cmp" => Ok (Application :: VndYellowriverCustomMenu) , "application/vnd.zul" | "zir" | "zirz" => Ok (Application :: VndZul) , "application/vnd.zzazz.deck+xml" | "zaz" => Ok (Application :: VndZzazzDeckXml) , "application/voicexml+xml" | "vxml" => Ok (Application :: VoicexmlXml) , "application/voucher-cms+json" | "vcj" => Ok (Application :: VoucherCmsJson) , "application/vq-rtcpxr" => Ok (Application :: VqRtcpxr) , "application/wasm" | "wasm" => Ok (Application :: Wasm) , "application/watcherinfo+xml" | "wif" => Ok (Application :: WatcherinfoXml) , "application/webpush-options+json" => Ok (Application :: WebpushOptionsJson) , "application/whoispp-query" => Ok (Application :: WhoisppQuery) , "application/whoispp-response" => Ok (Application :: WhoisppResponse) , "application/widget" | "wgt" => Ok (Application :: Widget) , "application/wita" => Ok (Application :: Wita) , "application/wordperfect5.1" => Ok (Application :: Wordperfect51) , "application/wsdl+xml" | "wsdl" => Ok (Application :: WsdlXml) , "application/wspolicy+xml" | "wspolicy" => Ok (Application :: WspolicyXml) , "application/x-pki-message" => Ok (Application :: XPkiMessage) , "application/x-www-form-urlencoded" => Ok (Application :: XWwwFormUrlencoded) , "application/x-x509-ca-cert" => Ok (Application :: XX509CaCert) , "application/x-x509-ca-ra-cert" => Ok (Application :: XX509CaRaCert) , "application/x-x509-next-ca-cert" => Ok (Application :: XX509NextCaCert) , "application/x400-bp" => Ok (Application :: X400Bp) , "application/xacml+xml" => Ok (Application :: XacmlXml) , "application/xcap-att+xml" | "xav" => Ok (Application :: XcapAttXml) , "application/xcap-caps+xml" | "xca" => Ok (Application :: XcapCapsXml) , "application/xcap-diff+xml" | "xdf" => Ok (Application :: XcapDiffXml) , "application/xcap-el+xml" | "xel" => Ok (Application :: XcapElXml) , "application/xcap-error+xml" | "xer" => Ok (Application :: XcapErrorXml) , "application/xcap-ns+xml" | "xns" => Ok (Application :: XcapNsXml) , "application/xcon-conference-info-diff+xml" => Ok (Application :: XconConferenceInfoDiffXml) , "application/xcon-conference-info+xml" => Ok (Application :: XconConferenceInfoXml) , "application/xenc+xml" => Ok (Application :: XencXml) , "application/xfdf" | "xfdf" => Ok (Application :: Xfdf) , "application/xhtml+xml" | "xhtml" | "xhtm" | "xht" => Ok (Application :: XhtmlXml) , "application/xliff+xml" | "xlf" => Ok (Application :: XliffXml) , "application/xml" => Ok (Application :: Xml) , "application/xml-dtd" | "dtd" => Ok (Application :: XmlDtd) , "application/xml-external-parsed-entity" => Ok (Application :: XmlExternalParsedEntity) , "application/xml-patch+xml" => Ok (Application :: XmlPatchXml) , "application/xmpp+xml" => Ok (Application :: XmppXml) , "application/xop+xml" | "xop" => Ok (Application :: XopXml) , "application/xslt+xml" | "xsl" | "xslt" => Ok (Application :: XsltXml) , "application/xv+xml" | "mxml" | "xhvml" | "xvml" | "xvm" => Ok (Application :: XvXml) , "application/yaml" => Ok (Application :: Yaml) , "application/yang" | "yang" => Ok (Application :: Yang) , "application/yang-data+cbor" => Ok (Application :: YangDataCbor) , "application/yang-data+json" => Ok (Application :: YangDataJson) , "application/yang-data+xml" => Ok (Application :: YangDataXml) , "application/yang-patch+json" => Ok (Application :: YangPatchJson) , "application/yang-patch+xml" => Ok (Application :: YangPatchXml) , "application/yin+xml" | "yin" => Ok (Application :: YinXml) , "application/zip" | "zip" => Ok (Application :: Zip) , "application/zlib" => Ok (Application :: Zlib) , "application/zstd" | "zst" => Ok (Application :: Zstd) , _ => Err (()) , }
    }
}
