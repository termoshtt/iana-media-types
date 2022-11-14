#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Application {
    #[doc = "application/1d-interleaved-parityfec"]
    _1DInterleavedParityfec,
    #[doc = "application/3gpdash-qoe-report+xml"]
    _3GpdashQoeReportXml,
    #[doc = "application/3gppHal+json"]
    _3GppHalJson,
    #[doc = "application/3gppHalForms+json"]
    _3GppHalFormsJson,
    #[doc = "application/3gpp-ims+xml"]
    _3GppImsXml,
    #[doc = "application/A2L"]
    A2L,
    #[doc = "application/ace+cbor"]
    AceCbor,
    #[doc = "application/ace+json"]
    AceJson,
    #[doc = "application/activemessage"]
    Activemessage,
    #[doc = "application/activity+json"]
    ActivityJson,
    #[doc = "application/aif+cbor"]
    AifCbor,
    #[doc = "application/aif+json"]
    AifJson,
    #[doc = "application/alto-cdni+json"]
    AltoCdniJson,
    #[doc = "application/alto-cdnifilter+json"]
    AltoCdnifilterJson,
    #[doc = "application/alto-costmap+json"]
    AltoCostmapJson,
    #[doc = "application/alto-costmapfilter+json"]
    AltoCostmapfilterJson,
    #[doc = "application/alto-directory+json"]
    AltoDirectoryJson,
    #[doc = "application/alto-endpointprop+json"]
    AltoEndpointpropJson,
    #[doc = "application/alto-endpointpropparams+json"]
    AltoEndpointpropparamsJson,
    #[doc = "application/alto-endpointcost+json"]
    AltoEndpointcostJson,
    #[doc = "application/alto-endpointcostparams+json"]
    AltoEndpointcostparamsJson,
    #[doc = "application/alto-error+json"]
    AltoErrorJson,
    #[doc = "application/alto-networkmapfilter+json"]
    AltoNetworkmapfilterJson,
    #[doc = "application/alto-networkmap+json"]
    AltoNetworkmapJson,
    #[doc = "application/alto-propmap+json"]
    AltoPropmapJson,
    #[doc = "application/alto-propmapparams+json"]
    AltoPropmapparamsJson,
    #[doc = "application/alto-updatestreamcontrol+json"]
    AltoUpdatestreamcontrolJson,
    #[doc = "application/alto-updatestreamparams+json"]
    AltoUpdatestreamparamsJson,
    #[doc = "application/AML"]
    Aml,
    #[doc = "application/andrew-inset"]
    AndrewInset,
    #[doc = "application/applefile"]
    Applefile,
    #[doc = "application/at+jwt"]
    AtJwt,
    #[doc = "application/ATF"]
    Atf,
    #[doc = "application/ATFX"]
    Atfx,
    #[doc = "application/atom+xml"]
    AtomXml,
    #[doc = "application/atomcat+xml"]
    AtomcatXml,
    #[doc = "application/atomdeleted+xml"]
    AtomdeletedXml,
    #[doc = "application/atomicmail"]
    Atomicmail,
    #[doc = "application/atomsvc+xml"]
    AtomsvcXml,
    #[doc = "application/atsc-dwd+xml"]
    AtscDwdXml,
    #[doc = "application/atsc-dynamic-event-message"]
    AtscDynamicEventMessage,
    #[doc = "application/atsc-held+xml"]
    AtscHeldXml,
    #[doc = "application/atsc-rdt+json"]
    AtscRdtJson,
    #[doc = "application/atsc-rsat+xml"]
    AtscRsatXml,
    #[doc = "application/ATXML"]
    Atxml,
    #[doc = "application/auth-policy+xml"]
    AuthPolicyXml,
    #[doc = "application/automationml-aml+xml"]
    AutomationmlAmlXml,
    #[doc = "application/automationml-amlx+zip"]
    AutomationmlAmlxZip,
    #[doc = "application/bacnet-xdd+zip"]
    BacnetXddZip,
    #[doc = "application/batch-SMTP"]
    BatchSMTP,
    #[doc = "application/beep+xml"]
    BeepXml,
    #[doc = "application/calendar+json"]
    CalendarJson,
    #[doc = "application/calendar+xml"]
    CalendarXml,
    #[doc = "application/call-completion"]
    CallCompletion,
    #[doc = "application/CALS-1840"]
    Cals1840,
    #[doc = "application/captive+json"]
    CaptiveJson,
    #[doc = "application/cbor"]
    Cbor,
    #[doc = "application/cbor-seq"]
    CborSeq,
    #[doc = "application/cccex"]
    Cccex,
    #[doc = "application/ccmp+xml"]
    CcmpXml,
    #[doc = "application/ccxml+xml"]
    CcxmlXml,
    #[doc = "application/cda+xml"]
    CdaXml,
    #[doc = "application/CDFX+XML"]
    CdfxXml,
    #[doc = "application/cdmi-capability"]
    CdmiCapability,
    #[doc = "application/cdmi-container"]
    CdmiContainer,
    #[doc = "application/cdmi-domain"]
    CdmiDomain,
    #[doc = "application/cdmi-object"]
    CdmiObject,
    #[doc = "application/cdmi-queue"]
    CdmiQueue,
    #[doc = "application/cdni"]
    Cdni,
    #[doc = "application/CEA"]
    Cea,
    #[doc = "application/cea-2018+xml"]
    Cea2018Xml,
    #[doc = "application/cellml+xml"]
    CellmlXml,
    #[doc = "application/cfw"]
    Cfw,
    #[doc = "application/city+json"]
    CityJson,
    #[doc = "application/clr"]
    Clr,
    #[doc = "application/clue_info+xml"]
    ClueInfoXml,
    #[doc = "application/clue+xml"]
    ClueXml,
    #[doc = "application/cms"]
    Cms,
    #[doc = "application/cnrp+xml"]
    CnrpXml,
    #[doc = "application/coap-group+json"]
    CoapGroupJson,
    #[doc = "application/coap-payload"]
    CoapPayload,
    #[doc = "application/commonground"]
    Commonground,
    #[doc = "application/concise-problem-details+cbor"]
    ConciseProblemDetailsCbor,
    #[doc = "application/conference-info+xml"]
    ConferenceInfoXml,
    #[doc = "application/cpl+xml"]
    CplXml,
    #[doc = "application/cose"]
    Cose,
    #[doc = "application/cose-key"]
    CoseKey,
    #[doc = "application/cose-key-set"]
    CoseKeySet,
    #[doc = "application/cose-x509"]
    CoseX509,
    #[doc = "application/csrattrs"]
    Csrattrs,
    #[doc = "application/csta+xml"]
    CstaXml,
    #[doc = "application/CSTAdata+xml"]
    CstadataXml,
    #[doc = "application/csvm+json"]
    CsvmJson,
    #[doc = "application/cwl"]
    Cwl,
    #[doc = "application/cwl+json"]
    CwlJson,
    #[doc = "application/cwt"]
    Cwt,
    #[doc = "application/cybercash"]
    Cybercash,
    #[doc = "application/dash+xml"]
    DashXml,
    #[doc = "application/dash-patch+xml"]
    DashPatchXml,
    #[doc = "application/dashdelta"]
    Dashdelta,
    #[doc = "application/davmount+xml"]
    DavmountXml,
    #[doc = "application/dca-rft"]
    DcaRft,
    #[doc = "application/DCD"]
    Dcd,
    #[doc = "application/dec-dx"]
    DecDx,
    #[doc = "application/dialog-info+xml"]
    DialogInfoXml,
    #[doc = "application/dicom"]
    Dicom,
    #[doc = "application/dicom+json"]
    DicomJson,
    #[doc = "application/dicom+xml"]
    DicomXml,
    #[doc = "application/DII"]
    Dii,
    #[doc = "application/DIT"]
    Dit,
    #[doc = "application/dns"]
    Dns,
    #[doc = "application/dns+json"]
    DnsJson,
    #[doc = "application/dns-message"]
    DnsMessage,
    #[doc = "application/dots+cbor"]
    DotsCbor,
    #[doc = "application/dskpp+xml"]
    DskppXml,
    #[doc = "application/dssc+der"]
    DsscDer,
    #[doc = "application/dssc+xml"]
    DsscXml,
    #[doc = "application/dvcs"]
    Dvcs,
    #[doc = "application/EDI-consent"]
    EdiConsent,
    #[doc = "application/EDIFACT"]
    Edifact,
    #[doc = "application/EDI-X12"]
    EdiX12,
    #[doc = "application/efi"]
    Efi,
    #[doc = "application/elm+json"]
    ElmJson,
    #[doc = "application/elm+xml"]
    ElmXml,
    #[doc = "application/EmergencyCallData.cap+xml"]
    EmergencyCallDataCapXml,
    #[doc = "application/EmergencyCallData.Comment+xml"]
    EmergencyCallDataCommentXml,
    #[doc = "application/EmergencyCallData.Control+xml"]
    EmergencyCallDataControlXml,
    #[doc = "application/EmergencyCallData.DeviceInfo+xml"]
    EmergencyCallDataDeviceInfoXml,
    #[doc = "application/EmergencyCallData.eCall.MSD"]
    EmergencyCallDataECallMSD,
    #[doc = "application/EmergencyCallData.LegacyESN+json"]
    EmergencyCallDataLegacyESNJson,
    #[doc = "application/EmergencyCallData.ProviderInfo+xml"]
    EmergencyCallDataProviderInfoXml,
    #[doc = "application/EmergencyCallData.ServiceInfo+xml"]
    EmergencyCallDataServiceInfoXml,
    #[doc = "application/EmergencyCallData.SubscriberInfo+xml"]
    EmergencyCallDataSubscriberInfoXml,
    #[doc = "application/EmergencyCallData.VEDS+xml"]
    EmergencyCallDataVEDSXml,
    #[doc = "application/emma+xml"]
    EmmaXml,
    #[doc = "application/emotionml+xml"]
    EmotionmlXml,
    #[doc = "application/encaprtp"]
    Encaprtp,
    #[doc = "application/epp+xml"]
    EppXml,
    #[doc = "application/epub+zip"]
    EpubZip,
    #[doc = "application/eshop"]
    Eshop,
    #[doc = "application/example"]
    Example,
    #[doc = "application/exi"]
    Exi,
    #[doc = "application/expect-ct-report+json"]
    ExpectCtReportJson,
    #[doc = "application/express"]
    Express,
    #[doc = "application/fastinfoset"]
    Fastinfoset,
    #[doc = "application/fastsoap"]
    Fastsoap,
    #[doc = "application/fdf"]
    Fdf,
    #[doc = "application/fdt+xml"]
    FdtXml,
    #[doc = "application/fhir+json"]
    FhirJson,
    #[doc = "application/fhir+xml"]
    FhirXml,
    #[doc = "application/fits"]
    Fits,
    #[doc = "application/flexfec"]
    Flexfec,
    #[doc = "application/font-tdpfr"]
    FontTdpfr,
    #[doc = "application/framework-attributes+xml"]
    FrameworkAttributesXml,
    #[doc = "application/geo+json"]
    GeoJson,
    #[doc = "application/geo+json-seq"]
    GeoJsonSeq,
    #[doc = "application/geopackage+sqlite3"]
    GeopackageSqlite3,
    #[doc = "application/geoxacml+xml"]
    GeoxacmlXml,
    #[doc = "application/gltf-buffer"]
    GltfBuffer,
    #[doc = "application/gml+xml"]
    GmlXml,
    #[doc = "application/gzip"]
    Gzip,
    #[doc = "application/H224"]
    H224,
    #[doc = "application/held+xml"]
    HeldXml,
    #[doc = "application/hl7v2+xml"]
    Hl7V2Xml,
    #[doc = "application/http"]
    Http,
    #[doc = "application/hyperstudio"]
    Hyperstudio,
    #[doc = "application/ibe-key-request+xml"]
    IbeKeyRequestXml,
    #[doc = "application/ibe-pkg-reply+xml"]
    IbePkgReplyXml,
    #[doc = "application/ibe-pp-data"]
    IbePpData,
    #[doc = "application/iges"]
    Iges,
    #[doc = "application/im-iscomposing+xml"]
    ImIscomposingXml,
    #[doc = "application/index"]
    Index,
    #[doc = "application/index.cmd"]
    IndexCmd,
    #[doc = "application/index.obj"]
    IndexObj,
    #[doc = "application/index.response"]
    IndexResponse,
    #[doc = "application/index.vnd"]
    IndexVnd,
    #[doc = "application/inkml+xml"]
    InkmlXml,
    #[doc = "application/IOTP"]
    Iotp,
    #[doc = "application/ipfix"]
    Ipfix,
    #[doc = "application/ipp"]
    Ipp,
    #[doc = "application/ISUP"]
    Isup,
    #[doc = "application/its+xml"]
    ItsXml,
    #[doc = "application/jf2feed+json"]
    Jf2FeedJson,
    #[doc = "application/jose"]
    Jose,
    #[doc = "application/jose+json"]
    JoseJson,
    #[doc = "application/jrd+json"]
    JrdJson,
    #[doc = "application/jscalendar+json"]
    JscalendarJson,
    #[doc = "application/json"]
    Json,
    #[doc = "application/json-patch+json"]
    JsonPatchJson,
    #[doc = "application/json-seq"]
    JsonSeq,
    #[doc = "application/jwk+json"]
    JwkJson,
    #[doc = "application/jwk-set+json"]
    JwkSetJson,
    #[doc = "application/jwt"]
    Jwt,
    #[doc = "application/kpml-request+xml"]
    KpmlRequestXml,
    #[doc = "application/kpml-response+xml"]
    KpmlResponseXml,
    #[doc = "application/ld+json"]
    LdJson,
    #[doc = "application/lgr+xml"]
    LgrXml,
    #[doc = "application/link-format"]
    LinkFormat,
    #[doc = "application/linkset"]
    Linkset,
    #[doc = "application/linkset+json"]
    LinksetJson,
    #[doc = "application/load-control+xml"]
    LoadControlXml,
    #[doc = "application/logout+jwt"]
    LogoutJwt,
    #[doc = "application/lost+xml"]
    LostXml,
    #[doc = "application/lostsync+xml"]
    LostsyncXml,
    #[doc = "application/lpf+zip"]
    LpfZip,
    #[doc = "application/LXF"]
    Lxf,
    #[doc = "application/mac-binhex40"]
    MacBinhex40,
    #[doc = "application/macwriteii"]
    Macwriteii,
    #[doc = "application/mads+xml"]
    MadsXml,
    #[doc = "application/manifest+json"]
    ManifestJson,
    #[doc = "application/marc"]
    Marc,
    #[doc = "application/marcxml+xml"]
    MarcxmlXml,
    #[doc = "application/mathematica"]
    Mathematica,
    #[doc = "application/mathml+xml"]
    MathmlXml,
    #[doc = "application/mathml-content+xml"]
    MathmlContentXml,
    #[doc = "application/mathml-presentation+xml"]
    MathmlPresentationXml,
    #[doc = "application/mbms-associated-procedure-description+xml"]
    MbmsAssociatedProcedureDescriptionXml,
    #[doc = "application/mbms-deregister+xml"]
    MbmsDeregisterXml,
    #[doc = "application/mbms-envelope+xml"]
    MbmsEnvelopeXml,
    #[doc = "application/mbms-msk-response+xml"]
    MbmsMskResponseXml,
    #[doc = "application/mbms-msk+xml"]
    MbmsMskXml,
    #[doc = "application/mbms-protection-description+xml"]
    MbmsProtectionDescriptionXml,
    #[doc = "application/mbms-reception-report+xml"]
    MbmsReceptionReportXml,
    #[doc = "application/mbms-register-response+xml"]
    MbmsRegisterResponseXml,
    #[doc = "application/mbms-register+xml"]
    MbmsRegisterXml,
    #[doc = "application/mbms-schedule+xml"]
    MbmsScheduleXml,
    #[doc = "application/mbms-user-service-description+xml"]
    MbmsUserServiceDescriptionXml,
    #[doc = "application/mbox"]
    Mbox,
    #[doc = "application/media_control+xml"]
    MediaControlXml,
    #[doc = "application/media-policy-dataset+xml"]
    MediaPolicyDatasetXml,
    #[doc = "application/mediaservercontrol+xml"]
    MediaservercontrolXml,
    #[doc = "application/merge-patch+json"]
    MergePatchJson,
    #[doc = "application/metalink4+xml"]
    Metalink4Xml,
    #[doc = "application/mets+xml"]
    MetsXml,
    #[doc = "application/MF4"]
    Mf4,
    #[doc = "application/mikey"]
    Mikey,
    #[doc = "application/mipc"]
    Mipc,
    #[doc = "application/missing-blocks+cbor-seq"]
    MissingBlocksCborSeq,
    #[doc = "application/mmt-aei+xml"]
    MmtAeiXml,
    #[doc = "application/mmt-usd+xml"]
    MmtUsdXml,
    #[doc = "application/mods+xml"]
    ModsXml,
    #[doc = "application/moss-keys"]
    MossKeys,
    #[doc = "application/moss-signature"]
    MossSignature,
    #[doc = "application/mosskey-data"]
    MosskeyData,
    #[doc = "application/mosskey-request"]
    MosskeyRequest,
    #[doc = "application/mp21"]
    Mp21,
    #[doc = "application/mp4"]
    Mp4,
    #[doc = "application/mpeg4-generic"]
    Mpeg4Generic,
    #[doc = "application/mpeg4-iod"]
    Mpeg4Iod,
    #[doc = "application/mpeg4-iod-xmt"]
    Mpeg4IodXmt,
    #[doc = "application/mrb-consumer+xml"]
    MrbConsumerXml,
    #[doc = "application/mrb-publish+xml"]
    MrbPublishXml,
    #[doc = "application/msc-ivr+xml"]
    MscIvrXml,
    #[doc = "application/msc-mixer+xml"]
    MscMixerXml,
    #[doc = "application/msword"]
    Msword,
    #[doc = "application/mud+json"]
    MudJson,
    #[doc = "application/multipart-core"]
    MultipartCore,
    #[doc = "application/mxf"]
    Mxf,
    #[doc = "application/n-quads"]
    NQuads,
    #[doc = "application/n-triples"]
    NTriples,
    #[doc = "application/nasdata"]
    Nasdata,
    #[doc = "application/news-checkgroups"]
    NewsCheckgroups,
    #[doc = "application/news-groupinfo"]
    NewsGroupinfo,
    #[doc = "application/news-transmission"]
    NewsTransmission,
    #[doc = "application/nlsml+xml"]
    NlsmlXml,
    #[doc = "application/node"]
    Node,
    #[doc = "application/nss"]
    Nss,
    #[doc = "application/oauth-authz-req+jwt"]
    OauthAuthzReqJwt,
    #[doc = "application/oblivious-dns-message"]
    ObliviousDnsMessage,
    #[doc = "application/ocsp-request"]
    OcspRequest,
    #[doc = "application/ocsp-response"]
    OcspResponse,
    #[doc = "application/octet-stream"]
    OctetStream,
    #[doc = "application/ODA"]
    Oda,
    #[doc = "application/odm+xml"]
    OdmXml,
    #[doc = "application/ODX"]
    Odx,
    #[doc = "application/oebps-package+xml"]
    OebpsPackageXml,
    #[doc = "application/ogg"]
    Ogg,
    #[doc = "application/opc-nodeset+xml"]
    OpcNodesetXml,
    #[doc = "application/oscore"]
    Oscore,
    #[doc = "application/oxps"]
    Oxps,
    #[doc = "application/p21"]
    P21,
    #[doc = "application/p21+zip"]
    P21Zip,
    #[doc = "application/p2p-overlay+xml"]
    P2POverlayXml,
    #[doc = "application/parityfec"]
    Parityfec,
    #[doc = "application/passport"]
    Passport,
    #[doc = "application/patch-ops-error+xml"]
    PatchOpsErrorXml,
    #[doc = "application/pdf"]
    Pdf,
    #[doc = "application/PDX"]
    Pdx,
    #[doc = "application/pem-certificate-chain"]
    PemCertificateChain,
    #[doc = "application/pgp-encrypted"]
    PgpEncrypted,
    #[doc = "application/pgp-keys"]
    PgpKeys,
    #[doc = "application/pgp-signature"]
    PgpSignature,
    #[doc = "application/pidf-diff+xml"]
    PidfDiffXml,
    #[doc = "application/pidf+xml"]
    PidfXml,
    #[doc = "application/pkcs10"]
    Pkcs10,
    #[doc = "application/pkcs7-mime"]
    Pkcs7Mime,
    #[doc = "application/pkcs7-signature"]
    Pkcs7Signature,
    #[doc = "application/pkcs8"]
    Pkcs8,
    #[doc = "application/pkcs8-encrypted"]
    Pkcs8Encrypted,
    #[doc = "application/pkcs12"]
    Pkcs12,
    #[doc = "application/pkix-attr-cert"]
    PkixAttrCert,
    #[doc = "application/pkix-cert"]
    PkixCert,
    #[doc = "application/pkix-crl"]
    PkixCrl,
    #[doc = "application/pkix-pkipath"]
    PkixPkipath,
    #[doc = "application/pkixcmp"]
    Pkixcmp,
    #[doc = "application/pls+xml"]
    PlsXml,
    #[doc = "application/poc-settings+xml"]
    PocSettingsXml,
    #[doc = "application/postscript"]
    Postscript,
    #[doc = "application/ppsp-tracker+json"]
    PpspTrackerJson,
    #[doc = "application/problem+json"]
    ProblemJson,
    #[doc = "application/problem+xml"]
    ProblemXml,
    #[doc = "application/provenance+xml"]
    ProvenanceXml,
    #[doc = "application/prs.alvestrand.titrax-sheet"]
    PrsAlvestrandTitraxSheet,
    #[doc = "application/prs.cww"]
    PrsCww,
    #[doc = "application/prs.cyn"]
    PrsCyn,
    #[doc = "application/prs.hpub+zip"]
    PrsHpubZip,
    #[doc = "application/prs.nprend"]
    PrsNprend,
    #[doc = "application/prs.plucker"]
    PrsPlucker,
    #[doc = "application/prs.rdf-xml-crypt"]
    PrsRdfXmlCrypt,
    #[doc = "application/prs.xsf+xml"]
    PrsXsfXml,
    #[doc = "application/pskc+xml"]
    PskcXml,
    #[doc = "application/pvd+json"]
    PvdJson,
    #[doc = "application/rdf+xml"]
    RdfXml,
    #[doc = "application/route-apd+xml"]
    RouteApdXml,
    #[doc = "application/route-s-tsid+xml"]
    RouteSTsidXml,
    #[doc = "application/route-usd+xml"]
    RouteUsdXml,
    #[doc = "application/QSIG"]
    Qsig,
    #[doc = "application/raptorfec"]
    Raptorfec,
    #[doc = "application/rdap+json"]
    RdapJson,
    #[doc = "application/reginfo+xml"]
    ReginfoXml,
    #[doc = "application/relax-ng-compact-syntax"]
    RelaxNgCompactSyntax,
    #[doc = "application/remote-printing"]
    RemotePrinting,
    #[doc = "application/reputon+json"]
    ReputonJson,
    #[doc = "application/resource-lists-diff+xml"]
    ResourceListsDiffXml,
    #[doc = "application/resource-lists+xml"]
    ResourceListsXml,
    #[doc = "application/rfc+xml"]
    RfcXml,
    #[doc = "application/riscos"]
    Riscos,
    #[doc = "application/rlmi+xml"]
    RlmiXml,
    #[doc = "application/rls-services+xml"]
    RlsServicesXml,
    #[doc = "application/rpki-checklist"]
    RpkiChecklist,
    #[doc = "application/rpki-ghostbusters"]
    RpkiGhostbusters,
    #[doc = "application/rpki-manifest"]
    RpkiManifest,
    #[doc = "application/rpki-publication"]
    RpkiPublication,
    #[doc = "application/rpki-roa"]
    RpkiRoa,
    #[doc = "application/rpki-updown"]
    RpkiUpdown,
    #[doc = "application/rtf"]
    Rtf,
    #[doc = "application/rtploopback"]
    Rtploopback,
    #[doc = "application/rtx"]
    Rtx,
    #[doc = "application/samlassertion+xml"]
    SamlassertionXml,
    #[doc = "application/samlmetadata+xml"]
    SamlmetadataXml,
    #[doc = "application/sarif-external-properties+json"]
    SarifExternalPropertiesJson,
    #[doc = "application/sarif+json"]
    SarifJson,
    #[doc = "application/sbe"]
    Sbe,
    #[doc = "application/sbml+xml"]
    SbmlXml,
    #[doc = "application/scaip+xml"]
    ScaipXml,
    #[doc = "application/scim+json"]
    ScimJson,
    #[doc = "application/scvp-cv-request"]
    ScvpCvRequest,
    #[doc = "application/scvp-cv-response"]
    ScvpCvResponse,
    #[doc = "application/scvp-vp-request"]
    ScvpVpRequest,
    #[doc = "application/scvp-vp-response"]
    ScvpVpResponse,
    #[doc = "application/sdp"]
    Sdp,
    #[doc = "application/secevent+jwt"]
    SeceventJwt,
    #[doc = "application/senml-etch+cbor"]
    SenmlEtchCbor,
    #[doc = "application/senml-etch+json"]
    SenmlEtchJson,
    #[doc = "application/senml-exi"]
    SenmlExi,
    #[doc = "application/senml+cbor"]
    SenmlCbor,
    #[doc = "application/senml+json"]
    SenmlJson,
    #[doc = "application/senml+xml"]
    SenmlXml,
    #[doc = "application/sensml-exi"]
    SensmlExi,
    #[doc = "application/sensml+cbor"]
    SensmlCbor,
    #[doc = "application/sensml+json"]
    SensmlJson,
    #[doc = "application/sensml+xml"]
    SensmlXml,
    #[doc = "application/sep-exi"]
    SepExi,
    #[doc = "application/sep+xml"]
    SepXml,
    #[doc = "application/session-info"]
    SessionInfo,
    #[doc = "application/set-payment"]
    SetPayment,
    #[doc = "application/set-payment-initiation"]
    SetPaymentInitiation,
    #[doc = "application/set-registration"]
    SetRegistration,
    #[doc = "application/set-registration-initiation"]
    SetRegistrationInitiation,
    #[doc = "application/SGML"]
    Sgml,
    #[doc = "application/sgml-open-catalog"]
    SgmlOpenCatalog,
    #[doc = "application/shf+xml"]
    ShfXml,
    #[doc = "application/sieve"]
    Sieve,
    #[doc = "application/simple-filter+xml"]
    SimpleFilterXml,
    #[doc = "application/simple-message-summary"]
    SimpleMessageSummary,
    #[doc = "application/simpleSymbolContainer"]
    SimpleSymbolContainer,
    #[doc = "application/sipc"]
    Sipc,
    #[doc = "application/slate"]
    Slate,
    #[doc = "application/smil+xml"]
    SmilXml,
    #[doc = "application/smpte336m"]
    Smpte336M,
    #[doc = "application/soap+fastinfoset"]
    SoapFastinfoset,
    #[doc = "application/soap+xml"]
    SoapXml,
    #[doc = "application/sparql-query"]
    SparqlQuery,
    #[doc = "application/spdx+json"]
    SpdxJson,
    #[doc = "application/sparql-results+xml"]
    SparqlResultsXml,
    #[doc = "application/spirits-event+xml"]
    SpiritsEventXml,
    #[doc = "application/sql"]
    Sql,
    #[doc = "application/srgs"]
    Srgs,
    #[doc = "application/srgs+xml"]
    SrgsXml,
    #[doc = "application/sru+xml"]
    SruXml,
    #[doc = "application/ssml+xml"]
    SsmlXml,
    #[doc = "application/stix+json"]
    StixJson,
    #[doc = "application/swid+cbor"]
    SwidCbor,
    #[doc = "application/swid+xml"]
    SwidXml,
    #[doc = "application/tamp-apex-update"]
    TampApexUpdate,
    #[doc = "application/tamp-apex-update-confirm"]
    TampApexUpdateConfirm,
    #[doc = "application/tamp-community-update"]
    TampCommunityUpdate,
    #[doc = "application/tamp-community-update-confirm"]
    TampCommunityUpdateConfirm,
    #[doc = "application/tamp-error"]
    TampError,
    #[doc = "application/tamp-sequence-adjust"]
    TampSequenceAdjust,
    #[doc = "application/tamp-sequence-adjust-confirm"]
    TampSequenceAdjustConfirm,
    #[doc = "application/tamp-status-query"]
    TampStatusQuery,
    #[doc = "application/tamp-status-response"]
    TampStatusResponse,
    #[doc = "application/tamp-update"]
    TampUpdate,
    #[doc = "application/tamp-update-confirm"]
    TampUpdateConfirm,
    #[doc = "application/taxii+json"]
    TaxiiJson,
    #[doc = "application/td+json"]
    TdJson,
    #[doc = "application/tei+xml"]
    TeiXml,
    #[doc = "application/TETRA_ISI"]
    TetraIsi,
    #[doc = "application/thraud+xml"]
    ThraudXml,
    #[doc = "application/timestamp-query"]
    TimestampQuery,
    #[doc = "application/timestamp-reply"]
    TimestampReply,
    #[doc = "application/timestamped-data"]
    TimestampedData,
    #[doc = "application/tlsrpt+gzip"]
    TlsrptGzip,
    #[doc = "application/tlsrpt+json"]
    TlsrptJson,
    #[doc = "application/tm+json"]
    TmJson,
    #[doc = "application/tnauthlist"]
    Tnauthlist,
    #[doc = "application/token-introspection+jwt"]
    TokenIntrospectionJwt,
    #[doc = "application/trickle-ice-sdpfrag"]
    TrickleIceSdpfrag,
    #[doc = "application/trig"]
    Trig,
    #[doc = "application/ttml+xml"]
    TtmlXml,
    #[doc = "application/tve-trigger"]
    TveTrigger,
    #[doc = "application/tzif"]
    Tzif,
    #[doc = "application/tzif-leap"]
    TzifLeap,
    #[doc = "application/ulpfec"]
    Ulpfec,
    #[doc = "application/urc-grpsheet+xml"]
    UrcGrpsheetXml,
    #[doc = "application/urc-ressheet+xml"]
    UrcRessheetXml,
    #[doc = "application/urc-targetdesc+xml"]
    UrcTargetdescXml,
    #[doc = "application/urc-uisocketdesc+xml"]
    UrcUisocketdescXml,
    #[doc = "application/vcard+json"]
    VcardJson,
    #[doc = "application/vcard+xml"]
    VcardXml,
    #[doc = "application/vemmi"]
    Vemmi,
    #[doc = "application/vnd.1000minds.decision-model+xml"]
    Vnd1000MindsDecisionModelXml,
    #[doc = "application/vnd.3gpp.5gnas"]
    Vnd3Gpp5Gnas,
    #[doc = "application/vnd.3gpp.access-transfer-events+xml"]
    Vnd3GppAccessTransferEventsXml,
    #[doc = "application/vnd.3gpp.bsf+xml"]
    Vnd3GppBsfXml,
    #[doc = "application/vnd.3gpp.GMOP+xml"]
    Vnd3GppGMOPXml,
    #[doc = "application/vnd.3gpp.gtpc"]
    Vnd3GppGtpc,
    #[doc = "application/vnd.3gpp.interworking-data"]
    Vnd3GppInterworkingData,
    #[doc = "application/vnd.3gpp.lpp"]
    Vnd3GppLpp,
    #[doc = "application/vnd.3gpp.mc-signalling-ear"]
    Vnd3GppMcSignallingEar,
    #[doc = "application/vnd.3gpp.mcdata-affiliation-command+xml"]
    Vnd3GppMcdataAffiliationCommandXml,
    #[doc = "application/vnd.3gpp.mcdata-info+xml"]
    Vnd3GppMcdataInfoXml,
    #[doc = "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml"]
    Vnd3GppMcdataMsgstoreCtrlRequestXml,
    #[doc = "application/vnd.3gpp.mcdata-payload"]
    Vnd3GppMcdataPayload,
    #[doc = "application/vnd.3gpp.mcdata-regroup+xml"]
    Vnd3GppMcdataRegroupXml,
    #[doc = "application/vnd.3gpp.mcdata-service-config+xml"]
    Vnd3GppMcdataServiceConfigXml,
    #[doc = "application/vnd.3gpp.mcdata-signalling"]
    Vnd3GppMcdataSignalling,
    #[doc = "application/vnd.3gpp.mcdata-ue-config+xml"]
    Vnd3GppMcdataUeConfigXml,
    #[doc = "application/vnd.3gpp.mcdata-user-profile+xml"]
    Vnd3GppMcdataUserProfileXml,
    #[doc = "application/vnd.3gpp.mcptt-affiliation-command+xml"]
    Vnd3GppMcpttAffiliationCommandXml,
    #[doc = "application/vnd.3gpp.mcptt-floor-request+xml"]
    Vnd3GppMcpttFloorRequestXml,
    #[doc = "application/vnd.3gpp.mcptt-info+xml"]
    Vnd3GppMcpttInfoXml,
    #[doc = "application/vnd.3gpp.mcptt-location-info+xml"]
    Vnd3GppMcpttLocationInfoXml,
    #[doc = "application/vnd.3gpp.mcptt-mbms-usage-info+xml"]
    Vnd3GppMcpttMbmsUsageInfoXml,
    #[doc = "application/vnd.3gpp.mcptt-service-config+xml"]
    Vnd3GppMcpttServiceConfigXml,
    #[doc = "application/vnd.3gpp.mcptt-signed+xml"]
    Vnd3GppMcpttSignedXml,
    #[doc = "application/vnd.3gpp.mcptt-ue-config+xml"]
    Vnd3GppMcpttUeConfigXml,
    #[doc = "application/vnd.3gpp.mcptt-ue-init-config+xml"]
    Vnd3GppMcpttUeInitConfigXml,
    #[doc = "application/vnd.3gpp.mcptt-user-profile+xml"]
    Vnd3GppMcpttUserProfileXml,
    #[doc = "application/vnd.3gpp.mcvideo-affiliation-command+xml"]
    Vnd3GppMcvideoAffiliationCommandXml,
    #[doc = "application/vnd.3gpp.mcvideo-info+xml"]
    Vnd3GppMcvideoInfoXml,
    #[doc = "application/vnd.3gpp.mcvideo-location-info+xml"]
    Vnd3GppMcvideoLocationInfoXml,
    #[doc = "application/vnd.3gpp.mcvideo-mbms-usage-info+xml"]
    Vnd3GppMcvideoMbmsUsageInfoXml,
    #[doc = "application/vnd.3gpp.mcvideo-service-config+xml"]
    Vnd3GppMcvideoServiceConfigXml,
    #[doc = "application/vnd.3gpp.mcvideo-transmission-request+xml"]
    Vnd3GppMcvideoTransmissionRequestXml,
    #[doc = "application/vnd.3gpp.mcvideo-ue-config+xml"]
    Vnd3GppMcvideoUeConfigXml,
    #[doc = "application/vnd.3gpp.mcvideo-user-profile+xml"]
    Vnd3GppMcvideoUserProfileXml,
    #[doc = "application/vnd.3gpp.mid-call+xml"]
    Vnd3GppMidCallXml,
    #[doc = "application/vnd.3gpp.ngap"]
    Vnd3GppNgap,
    #[doc = "application/vnd.3gpp.pfcp"]
    Vnd3GppPfcp,
    #[doc = "application/vnd.3gpp.pic-bw-large"]
    Vnd3GppPicBwLarge,
    #[doc = "application/vnd.3gpp.pic-bw-small"]
    Vnd3GppPicBwSmall,
    #[doc = "application/vnd.3gpp.pic-bw-var"]
    Vnd3GppPicBwVar,
    #[doc = "application/vnd.3gpp-prose-pc3a+xml"]
    Vnd3GppProsePc3AXml,
    #[doc = "application/vnd.3gpp-prose-pc3ach+xml"]
    Vnd3GppProsePc3AchXml,
    #[doc = "application/vnd.3gpp-prose-pc3ch+xml"]
    Vnd3GppProsePc3ChXml,
    #[doc = "application/vnd.3gpp-prose-pc8+xml"]
    Vnd3GppProsePc8Xml,
    #[doc = "application/vnd.3gpp-prose+xml"]
    Vnd3GppProseXml,
    #[doc = "application/vnd.3gpp.s1ap"]
    Vnd3GppS1Ap,
    #[doc = "application/vnd.3gpp.sms"]
    Vnd3GppSms,
    #[doc = "application/vnd.3gpp.sms+xml"]
    Vnd3GppSmsXml,
    #[doc = "application/vnd.3gpp.srvcc-ext+xml"]
    Vnd3GppSrvccExtXml,
    #[doc = "application/vnd.3gpp.SRVCC-info+xml"]
    Vnd3GppSRVCCInfoXml,
    #[doc = "application/vnd.3gpp.state-and-event-info+xml"]
    Vnd3GppStateAndEventInfoXml,
    #[doc = "application/vnd.3gpp.ussd+xml"]
    Vnd3GppUssdXml,
    #[doc = "application/vnd.3gpp-v2x-local-service-information"]
    Vnd3GppV2XLocalServiceInformation,
    #[doc = "application/vnd.3gpp2.bcmcsinfo+xml"]
    Vnd3Gpp2BcmcsinfoXml,
    #[doc = "application/vnd.3gpp2.sms"]
    Vnd3Gpp2Sms,
    #[doc = "application/vnd.3gpp2.tcap"]
    Vnd3Gpp2Tcap,
    #[doc = "application/vnd.3lightssoftware.imagescal"]
    Vnd3LightssoftwareImagescal,
    #[doc = "application/vnd.3M.Post-it-Notes"]
    Vnd3MPostItNotes,
    #[doc = "application/vnd.accpac.simply.aso"]
    VndAccpacSimplyAso,
    #[doc = "application/vnd.accpac.simply.imp"]
    VndAccpacSimplyImp,
    #[doc = "application/vnd.acucobol"]
    VndAcucobol,
    #[doc = "application/vnd.acucorp"]
    VndAcucorp,
    #[doc = "application/vnd.adobe.flash.movie"]
    VndAdobeFlashMovie,
    #[doc = "application/vnd.adobe.formscentral.fcdt"]
    VndAdobeFormscentralFcdt,
    #[doc = "application/vnd.adobe.fxp"]
    VndAdobeFxp,
    #[doc = "application/vnd.adobe.partial-upload"]
    VndAdobePartialUpload,
    #[doc = "application/vnd.adobe.xdp+xml"]
    VndAdobeXdpXml,
    #[doc = "application/vnd.aether.imp"]
    VndAetherImp,
    #[doc = "application/vnd.afpc.afplinedata"]
    VndAfpcAfplinedata,
    #[doc = "application/vnd.afpc.afplinedata-pagedef"]
    VndAfpcAfplinedataPagedef,
    #[doc = "application/vnd.afpc.cmoca-cmresource"]
    VndAfpcCmocaCmresource,
    #[doc = "application/vnd.afpc.foca-charset"]
    VndAfpcFocaCharset,
    #[doc = "application/vnd.afpc.foca-codedfont"]
    VndAfpcFocaCodedfont,
    #[doc = "application/vnd.afpc.foca-codepage"]
    VndAfpcFocaCodepage,
    #[doc = "application/vnd.afpc.modca"]
    VndAfpcModca,
    #[doc = "application/vnd.afpc.modca-cmtable"]
    VndAfpcModcaCmtable,
    #[doc = "application/vnd.afpc.modca-formdef"]
    VndAfpcModcaFormdef,
    #[doc = "application/vnd.afpc.modca-mediummap"]
    VndAfpcModcaMediummap,
    #[doc = "application/vnd.afpc.modca-objectcontainer"]
    VndAfpcModcaObjectcontainer,
    #[doc = "application/vnd.afpc.modca-overlay"]
    VndAfpcModcaOverlay,
    #[doc = "application/vnd.afpc.modca-pagesegment"]
    VndAfpcModcaPagesegment,
    #[doc = "application/vnd.age"]
    VndAge,
    #[doc = "application/vnd.ah-barcode"]
    VndAhBarcode,
    #[doc = "application/vnd.ahead.space"]
    VndAheadSpace,
    #[doc = "application/vnd.airzip.filesecure.azf"]
    VndAirzipFilesecureAzf,
    #[doc = "application/vnd.airzip.filesecure.azs"]
    VndAirzipFilesecureAzs,
    #[doc = "application/vnd.amadeus+json"]
    VndAmadeusJson,
    #[doc = "application/vnd.amazon.mobi8-ebook"]
    VndAmazonMobi8Ebook,
    #[doc = "application/vnd.americandynamics.acc"]
    VndAmericandynamicsAcc,
    #[doc = "application/vnd.amiga.ami"]
    VndAmigaAmi,
    #[doc = "application/vnd.amundsen.maze+xml"]
    VndAmundsenMazeXml,
    #[doc = "application/vnd.android.ota"]
    VndAndroidOta,
    #[doc = "application/vnd.anki"]
    VndAnki,
    #[doc = "application/vnd.anser-web-certificate-issue-initiation"]
    VndAnserWebCertificateIssueInitiation,
    #[doc = "application/vnd.antix.game-component"]
    VndAntixGameComponent,
    #[doc = "application/vnd.apache.arrow.file"]
    VndApacheArrowFile,
    #[doc = "application/vnd.apache.arrow.stream"]
    VndApacheArrowStream,
    #[doc = "application/vnd.apache.thrift.binary"]
    VndApacheThriftBinary,
    #[doc = "application/vnd.apache.thrift.compact"]
    VndApacheThriftCompact,
    #[doc = "application/vnd.apache.thrift.json"]
    VndApacheThriftJson,
    #[doc = "application/vnd.apexlang"]
    VndApexlang,
    #[doc = "application/vnd.api+json"]
    VndApiJson,
    #[doc = "application/vnd.aplextor.warrp+json"]
    VndAplextorWarrpJson,
    #[doc = "application/vnd.apothekende.reservation+json"]
    VndApothekendeReservationJson,
    #[doc = "application/vnd.apple.installer+xml"]
    VndAppleInstallerXml,
    #[doc = "application/vnd.apple.keynote"]
    VndAppleKeynote,
    #[doc = "application/vnd.apple.mpegurl"]
    VndAppleMpegurl,
    #[doc = "application/vnd.apple.numbers"]
    VndAppleNumbers,
    #[doc = "application/vnd.apple.pages"]
    VndApplePages,
    #[doc = "application/vnd.aristanetworks.swi"]
    VndAristanetworksSwi,
    #[doc = "application/vnd.artisan+json"]
    VndArtisanJson,
    #[doc = "application/vnd.artsquare"]
    VndArtsquare,
    #[doc = "application/vnd.astraea-software.iota"]
    VndAstraeaSoftwareIota,
    #[doc = "application/vnd.audiograph"]
    VndAudiograph,
    #[doc = "application/vnd.autopackage"]
    VndAutopackage,
    #[doc = "application/vnd.avalon+json"]
    VndAvalonJson,
    #[doc = "application/vnd.avistar+xml"]
    VndAvistarXml,
    #[doc = "application/vnd.balsamiq.bmml+xml"]
    VndBalsamiqBmmlXml,
    #[doc = "application/vnd.banana-accounting"]
    VndBananaAccounting,
    #[doc = "application/vnd.bbf.usp.error"]
    VndBbfUspError,
    #[doc = "application/vnd.bbf.usp.msg"]
    VndBbfUspMsg,
    #[doc = "application/vnd.bbf.usp.msg+json"]
    VndBbfUspMsgJson,
    #[doc = "application/vnd.balsamiq.bmpr"]
    VndBalsamiqBmpr,
    #[doc = "application/vnd.bekitzur-stech+json"]
    VndBekitzurStechJson,
    #[doc = "application/vnd.belightsoft.lhzd+zip"]
    VndBelightsoftLhzdZip,
    #[doc = "application/vnd.belightsoft.lhzl+zip"]
    VndBelightsoftLhzlZip,
    #[doc = "application/vnd.bint.med-content"]
    VndBintMedContent,
    #[doc = "application/vnd.biopax.rdf+xml"]
    VndBiopaxRdfXml,
    #[doc = "application/vnd.blink-idb-value-wrapper"]
    VndBlinkIdbValueWrapper,
    #[doc = "application/vnd.blueice.multipass"]
    VndBlueiceMultipass,
    #[doc = "application/vnd.bluetooth.ep.oob"]
    VndBluetoothEpOob,
    #[doc = "application/vnd.bluetooth.le.oob"]
    VndBluetoothLeOob,
    #[doc = "application/vnd.bmi"]
    VndBmi,
    #[doc = "application/vnd.bpf"]
    VndBpf,
    #[doc = "application/vnd.bpf3"]
    VndBpf3,
    #[doc = "application/vnd.businessobjects"]
    VndBusinessobjects,
    #[doc = "application/vnd.byu.uapi+json"]
    VndByuUapiJson,
    #[doc = "application/vnd.cab-jscript"]
    VndCabJscript,
    #[doc = "application/vnd.canon-cpdl"]
    VndCanonCpdl,
    #[doc = "application/vnd.canon-lips"]
    VndCanonLips,
    #[doc = "application/vnd.capasystems-pg+json"]
    VndCapasystemsPgJson,
    #[doc = "application/vnd.cendio.thinlinc.clientconf"]
    VndCendioThinlincClientconf,
    #[doc = "application/vnd.century-systems.tcp_stream"]
    VndCenturySystemsTcpStream,
    #[doc = "application/vnd.chemdraw+xml"]
    VndChemdrawXml,
    #[doc = "application/vnd.chess-pgn"]
    VndChessPgn,
    #[doc = "application/vnd.chipnuts.karaoke-mmd"]
    VndChipnutsKaraokeMmd,
    #[doc = "application/vnd.ciedi"]
    VndCiedi,
    #[doc = "application/vnd.cinderella"]
    VndCinderella,
    #[doc = "application/vnd.cirpack.isdn-ext"]
    VndCirpackIsdnExt,
    #[doc = "application/vnd.citationstyles.style+xml"]
    VndCitationstylesStyleXml,
    #[doc = "application/vnd.claymore"]
    VndClaymore,
    #[doc = "application/vnd.cloanto.rp9"]
    VndCloantoRp9,
    #[doc = "application/vnd.clonk.c4group"]
    VndClonkC4Group,
    #[doc = "application/vnd.cluetrust.cartomobile-config"]
    VndCluetrustCartomobileConfig,
    #[doc = "application/vnd.cluetrust.cartomobile-config-pkg"]
    VndCluetrustCartomobileConfigPkg,
    #[doc = "application/vnd.cncf.helm.chart.content.v1.tar+gzip"]
    VndCncfHelmChartContentV1TarGzip,
    #[doc = "application/vnd.cncf.helm.chart.provenance.v1.prov"]
    VndCncfHelmChartProvenanceV1Prov,
    #[doc = "application/vnd.coffeescript"]
    VndCoffeescript,
    #[doc = "application/vnd.collabio.xodocuments.document"]
    VndCollabioXodocumentsDocument,
    #[doc = "application/vnd.collabio.xodocuments.document-template"]
    VndCollabioXodocumentsDocumentTemplate,
    #[doc = "application/vnd.collabio.xodocuments.presentation"]
    VndCollabioXodocumentsPresentation,
    #[doc = "application/vnd.collabio.xodocuments.presentation-template"]
    VndCollabioXodocumentsPresentationTemplate,
    #[doc = "application/vnd.collabio.xodocuments.spreadsheet"]
    VndCollabioXodocumentsSpreadsheet,
    #[doc = "application/vnd.collabio.xodocuments.spreadsheet-template"]
    VndCollabioXodocumentsSpreadsheetTemplate,
    #[doc = "application/vnd.collection.doc+json"]
    VndCollectionDocJson,
    #[doc = "application/vnd.collection+json"]
    VndCollectionJson,
    #[doc = "application/vnd.collection.next+json"]
    VndCollectionNextJson,
    #[doc = "application/vnd.comicbook-rar"]
    VndComicbookRar,
    #[doc = "application/vnd.comicbook+zip"]
    VndComicbookZip,
    #[doc = "application/vnd.commerce-battelle"]
    VndCommerceBattelle,
    #[doc = "application/vnd.commonspace"]
    VndCommonspace,
    #[doc = "application/vnd.coreos.ignition+json"]
    VndCoreosIgnitionJson,
    #[doc = "application/vnd.cosmocaller"]
    VndCosmocaller,
    #[doc = "application/vnd.contact.cmsg"]
    VndContactCmsg,
    #[doc = "application/vnd.crick.clicker"]
    VndCrickClicker,
    #[doc = "application/vnd.crick.clicker.keyboard"]
    VndCrickClickerKeyboard,
    #[doc = "application/vnd.crick.clicker.palette"]
    VndCrickClickerPalette,
    #[doc = "application/vnd.crick.clicker.template"]
    VndCrickClickerTemplate,
    #[doc = "application/vnd.crick.clicker.wordbank"]
    VndCrickClickerWordbank,
    #[doc = "application/vnd.criticaltools.wbs+xml"]
    VndCriticaltoolsWbsXml,
    #[doc = "application/vnd.cryptii.pipe+json"]
    VndCryptiiPipeJson,
    #[doc = "application/vnd.crypto-shade-file"]
    VndCryptoShadeFile,
    #[doc = "application/vnd.cryptomator.encrypted"]
    VndCryptomatorEncrypted,
    #[doc = "application/vnd.cryptomator.vault"]
    VndCryptomatorVault,
    #[doc = "application/vnd.ctc-posml"]
    VndCtcPosml,
    #[doc = "application/vnd.ctct.ws+xml"]
    VndCtctWsXml,
    #[doc = "application/vnd.cups-pdf"]
    VndCupsPdf,
    #[doc = "application/vnd.cups-postscript"]
    VndCupsPostscript,
    #[doc = "application/vnd.cups-ppd"]
    VndCupsPpd,
    #[doc = "application/vnd.cups-raster"]
    VndCupsRaster,
    #[doc = "application/vnd.cups-raw"]
    VndCupsRaw,
    #[doc = "application/vnd.curl"]
    VndCurl,
    #[doc = "application/vnd.cyan.dean.root+xml"]
    VndCyanDeanRootXml,
    #[doc = "application/vnd.cybank"]
    VndCybank,
    #[doc = "application/vnd.cyclonedx+json"]
    VndCyclonedxJson,
    #[doc = "application/vnd.cyclonedx+xml"]
    VndCyclonedxXml,
    #[doc = "application/vnd.d2l.coursepackage1p0+zip"]
    VndD2LCoursepackage1P0Zip,
    #[doc = "application/vnd.d3m-dataset"]
    VndD3MDataset,
    #[doc = "application/vnd.d3m-problem"]
    VndD3MProblem,
    #[doc = "application/vnd.dart"]
    VndDart,
    #[doc = "application/vnd.data-vision.rdz"]
    VndDataVisionRdz,
    #[doc = "application/vnd.datalog"]
    VndDatalog,
    #[doc = "application/vnd.datapackage+json"]
    VndDatapackageJson,
    #[doc = "application/vnd.dataresource+json"]
    VndDataresourceJson,
    #[doc = "application/vnd.dbf"]
    VndDbf,
    #[doc = "application/vnd.debian.binary-package"]
    VndDebianBinaryPackage,
    #[doc = "application/vnd.dece.data"]
    VndDeceData,
    #[doc = "application/vnd.dece.ttml+xml"]
    VndDeceTtmlXml,
    #[doc = "application/vnd.dece.unspecified"]
    VndDeceUnspecified,
    #[doc = "application/vnd.dece.zip"]
    VndDeceZip,
    #[doc = "application/vnd.denovo.fcselayout-link"]
    VndDenovoFcselayoutLink,
    #[doc = "application/vnd.desmume.movie"]
    VndDesmumeMovie,
    #[doc = "application/vnd.dir-bi.plate-dl-nosuffix"]
    VndDirBiPlateDlNosuffix,
    #[doc = "application/vnd.dm.delegation+xml"]
    VndDmDelegationXml,
    #[doc = "application/vnd.dna"]
    VndDna,
    #[doc = "application/vnd.document+json"]
    VndDocumentJson,
    #[doc = "application/vnd.dolby.mobile.1"]
    VndDolbyMobile1,
    #[doc = "application/vnd.dolby.mobile.2"]
    VndDolbyMobile2,
    #[doc = "application/vnd.doremir.scorecloud-binary-document"]
    VndDoremirScorecloudBinaryDocument,
    #[doc = "application/vnd.dpgraph"]
    VndDpgraph,
    #[doc = "application/vnd.dreamfactory"]
    VndDreamfactory,
    #[doc = "application/vnd.drive+json"]
    VndDriveJson,
    #[doc = "application/vnd.dtg.local"]
    VndDtgLocal,
    #[doc = "application/vnd.dtg.local.flash"]
    VndDtgLocalFlash,
    #[doc = "application/vnd.dtg.local.html"]
    VndDtgLocalHtml,
    #[doc = "application/vnd.dvb.ait"]
    VndDvbAit,
    #[doc = "application/vnd.dvb.dvbisl+xml"]
    VndDvbDvbislXml,
    #[doc = "application/vnd.dvb.dvbj"]
    VndDvbDvbj,
    #[doc = "application/vnd.dvb.esgcontainer"]
    VndDvbEsgcontainer,
    #[doc = "application/vnd.dvb.ipdcdftnotifaccess"]
    VndDvbIpdcdftnotifaccess,
    #[doc = "application/vnd.dvb.ipdcesgaccess"]
    VndDvbIpdcesgaccess,
    #[doc = "application/vnd.dvb.ipdcesgaccess2"]
    VndDvbIpdcesgaccess2,
    #[doc = "application/vnd.dvb.ipdcesgpdd"]
    VndDvbIpdcesgpdd,
    #[doc = "application/vnd.dvb.ipdcroaming"]
    VndDvbIpdcroaming,
    #[doc = "application/vnd.dvb.iptv.alfec-base"]
    VndDvbIptvAlfecBase,
    #[doc = "application/vnd.dvb.iptv.alfec-enhancement"]
    VndDvbIptvAlfecEnhancement,
    #[doc = "application/vnd.dvb.notif-aggregate-root+xml"]
    VndDvbNotifAggregateRootXml,
    #[doc = "application/vnd.dvb.notif-container+xml"]
    VndDvbNotifContainerXml,
    #[doc = "application/vnd.dvb.notif-generic+xml"]
    VndDvbNotifGenericXml,
    #[doc = "application/vnd.dvb.notif-ia-msglist+xml"]
    VndDvbNotifIaMsglistXml,
    #[doc = "application/vnd.dvb.notif-ia-registration-request+xml"]
    VndDvbNotifIaRegistrationRequestXml,
    #[doc = "application/vnd.dvb.notif-ia-registration-response+xml"]
    VndDvbNotifIaRegistrationResponseXml,
    #[doc = "application/vnd.dvb.notif-init+xml"]
    VndDvbNotifInitXml,
    #[doc = "application/vnd.dvb.pfr"]
    VndDvbPfr,
    #[doc = "application/vnd.dvb.service"]
    VndDvbService,
    #[doc = "application/vnd.dxr"]
    VndDxr,
    #[doc = "application/vnd.dynageo"]
    VndDynageo,
    #[doc = "application/vnd.dzr"]
    VndDzr,
    #[doc = "application/vnd.easykaraoke.cdgdownload"]
    VndEasykaraokeCdgdownload,
    #[doc = "application/vnd.ecip.rlp"]
    VndEcipRlp,
    #[doc = "application/vnd.ecdis-update"]
    VndEcdisUpdate,
    #[doc = "application/vnd.eclipse.ditto+json"]
    VndEclipseDittoJson,
    #[doc = "application/vnd.ecowin.chart"]
    VndEcowinChart,
    #[doc = "application/vnd.ecowin.filerequest"]
    VndEcowinFilerequest,
    #[doc = "application/vnd.ecowin.fileupdate"]
    VndEcowinFileupdate,
    #[doc = "application/vnd.ecowin.series"]
    VndEcowinSeries,
    #[doc = "application/vnd.ecowin.seriesrequest"]
    VndEcowinSeriesrequest,
    #[doc = "application/vnd.ecowin.seriesupdate"]
    VndEcowinSeriesupdate,
    #[doc = "application/vnd.efi.img"]
    VndEfiImg,
    #[doc = "application/vnd.efi.iso"]
    VndEfiIso,
    #[doc = "application/vnd.emclient.accessrequest+xml"]
    VndEmclientAccessrequestXml,
    #[doc = "application/vnd.enliven"]
    VndEnliven,
    #[doc = "application/vnd.enphase.envoy"]
    VndEnphaseEnvoy,
    #[doc = "application/vnd.eprints.data+xml"]
    VndEprintsDataXml,
    #[doc = "application/vnd.epson.esf"]
    VndEpsonEsf,
    #[doc = "application/vnd.epson.msf"]
    VndEpsonMsf,
    #[doc = "application/vnd.epson.quickanime"]
    VndEpsonQuickanime,
    #[doc = "application/vnd.epson.salt"]
    VndEpsonSalt,
    #[doc = "application/vnd.epson.ssf"]
    VndEpsonSsf,
    #[doc = "application/vnd.ericsson.quickcall"]
    VndEricssonQuickcall,
    #[doc = "application/vnd.espass-espass+zip"]
    VndEspassEspassZip,
    #[doc = "application/vnd.eszigno3+xml"]
    VndEszigno3Xml,
    #[doc = "application/vnd.etsi.aoc+xml"]
    VndEtsiAocXml,
    #[doc = "application/vnd.etsi.asic-s+zip"]
    VndEtsiAsicSZip,
    #[doc = "application/vnd.etsi.asic-e+zip"]
    VndEtsiAsicEZip,
    #[doc = "application/vnd.etsi.cug+xml"]
    VndEtsiCugXml,
    #[doc = "application/vnd.etsi.iptvcommand+xml"]
    VndEtsiIptvcommandXml,
    #[doc = "application/vnd.etsi.iptvdiscovery+xml"]
    VndEtsiIptvdiscoveryXml,
    #[doc = "application/vnd.etsi.iptvprofile+xml"]
    VndEtsiIptvprofileXml,
    #[doc = "application/vnd.etsi.iptvsad-bc+xml"]
    VndEtsiIptvsadBcXml,
    #[doc = "application/vnd.etsi.iptvsad-cod+xml"]
    VndEtsiIptvsadCodXml,
    #[doc = "application/vnd.etsi.iptvsad-npvr+xml"]
    VndEtsiIptvsadNpvrXml,
    #[doc = "application/vnd.etsi.iptvservice+xml"]
    VndEtsiIptvserviceXml,
    #[doc = "application/vnd.etsi.iptvsync+xml"]
    VndEtsiIptvsyncXml,
    #[doc = "application/vnd.etsi.iptvueprofile+xml"]
    VndEtsiIptvueprofileXml,
    #[doc = "application/vnd.etsi.mcid+xml"]
    VndEtsiMcidXml,
    #[doc = "application/vnd.etsi.mheg5"]
    VndEtsiMheg5,
    #[doc = "application/vnd.etsi.overload-control-policy-dataset+xml"]
    VndEtsiOverloadControlPolicyDatasetXml,
    #[doc = "application/vnd.etsi.pstn+xml"]
    VndEtsiPstnXml,
    #[doc = "application/vnd.etsi.sci+xml"]
    VndEtsiSciXml,
    #[doc = "application/vnd.etsi.simservs+xml"]
    VndEtsiSimservsXml,
    #[doc = "application/vnd.etsi.timestamp-token"]
    VndEtsiTimestampToken,
    #[doc = "application/vnd.etsi.tsl+xml"]
    VndEtsiTslXml,
    #[doc = "application/vnd.etsi.tsl.der"]
    VndEtsiTslDer,
    #[doc = "application/vnd.eu.kasparian.car+json"]
    VndEuKasparianCarJson,
    #[doc = "application/vnd.eudora.data"]
    VndEudoraData,
    #[doc = "application/vnd.evolv.ecig.profile"]
    VndEvolvEcigProfile,
    #[doc = "application/vnd.evolv.ecig.settings"]
    VndEvolvEcigSettings,
    #[doc = "application/vnd.evolv.ecig.theme"]
    VndEvolvEcigTheme,
    #[doc = "application/vnd.exstream-empower+zip"]
    VndExstreamEmpowerZip,
    #[doc = "application/vnd.exstream-package"]
    VndExstreamPackage,
    #[doc = "application/vnd.ezpix-album"]
    VndEzpixAlbum,
    #[doc = "application/vnd.ezpix-package"]
    VndEzpixPackage,
    #[doc = "application/vnd.f-secure.mobile"]
    VndFSecureMobile,
    #[doc = "application/vnd.fastcopy-disk-image"]
    VndFastcopyDiskImage,
    #[doc = "application/vnd.familysearch.gedcom+zip"]
    VndFamilysearchGedcomZip,
    #[doc = "application/vnd.fdsn.mseed"]
    VndFdsnMseed,
    #[doc = "application/vnd.fdsn.seed"]
    VndFdsnSeed,
    #[doc = "application/vnd.ffsns"]
    VndFfsns,
    #[doc = "application/vnd.ficlab.flb+zip"]
    VndFiclabFlbZip,
    #[doc = "application/vnd.filmit.zfc"]
    VndFilmitZfc,
    #[doc = "application/vnd.fints"]
    VndFints,
    #[doc = "application/vnd.firemonkeys.cloudcell"]
    VndFiremonkeysCloudcell,
    #[doc = "application/vnd.FloGraphIt"]
    VndFloGraphIt,
    #[doc = "application/vnd.fluxtime.clip"]
    VndFluxtimeClip,
    #[doc = "application/vnd.font-fontforge-sfd"]
    VndFontFontforgeSfd,
    #[doc = "application/vnd.framemaker"]
    VndFramemaker,
    #[doc = "application/vnd.fsc.weblaunch"]
    VndFscWeblaunch,
    #[doc = "application/vnd.fujifilm.fb.docuworks"]
    VndFujifilmFbDocuworks,
    #[doc = "application/vnd.fujifilm.fb.docuworks.binder"]
    VndFujifilmFbDocuworksBinder,
    #[doc = "application/vnd.fujifilm.fb.docuworks.container"]
    VndFujifilmFbDocuworksContainer,
    #[doc = "application/vnd.fujifilm.fb.jfi+xml"]
    VndFujifilmFbJfiXml,
    #[doc = "application/vnd.fujitsu.oasys"]
    VndFujitsuOasys,
    #[doc = "application/vnd.fujitsu.oasys2"]
    VndFujitsuOasys2,
    #[doc = "application/vnd.fujitsu.oasys3"]
    VndFujitsuOasys3,
    #[doc = "application/vnd.fujitsu.oasysgp"]
    VndFujitsuOasysgp,
    #[doc = "application/vnd.fujitsu.oasysprs"]
    VndFujitsuOasysprs,
    #[doc = "application/vnd.fujixerox.ART4"]
    VndFujixeroxART4,
    #[doc = "application/vnd.fujixerox.ART-EX"]
    VndFujixeroxARTEX,
    #[doc = "application/vnd.fujixerox.ddd"]
    VndFujixeroxDdd,
    #[doc = "application/vnd.fujixerox.docuworks"]
    VndFujixeroxDocuworks,
    #[doc = "application/vnd.fujixerox.docuworks.binder"]
    VndFujixeroxDocuworksBinder,
    #[doc = "application/vnd.fujixerox.docuworks.container"]
    VndFujixeroxDocuworksContainer,
    #[doc = "application/vnd.fujixerox.HBPL"]
    VndFujixeroxHBPL,
    #[doc = "application/vnd.fut-misnet"]
    VndFutMisnet,
    #[doc = "application/vnd.futoin+cbor"]
    VndFutoinCbor,
    #[doc = "application/vnd.futoin+json"]
    VndFutoinJson,
    #[doc = "application/vnd.fuzzysheet"]
    VndFuzzysheet,
    #[doc = "application/vnd.genomatix.tuxedo"]
    VndGenomatixTuxedo,
    #[doc = "application/vnd.genozip"]
    VndGenozip,
    #[doc = "application/vnd.gentics.grd+json"]
    VndGenticsGrdJson,
    #[doc = "application/vnd.gentoo.catmetadata+xml"]
    VndGentooCatmetadataXml,
    #[doc = "application/vnd.gentoo.ebuild"]
    VndGentooEbuild,
    #[doc = "application/vnd.gentoo.eclass"]
    VndGentooEclass,
    #[doc = "application/vnd.gentoo.manifest"]
    VndGentooManifest,
    #[doc = "application/vnd.gentoo.pkgmetadata+xml"]
    VndGentooPkgmetadataXml,
    #[doc = "application/vnd.geogebra.file"]
    VndGeogebraFile,
    #[doc = "application/vnd.geogebra.slides"]
    VndGeogebraSlides,
    #[doc = "application/vnd.geogebra.tool"]
    VndGeogebraTool,
    #[doc = "application/vnd.geometry-explorer"]
    VndGeometryExplorer,
    #[doc = "application/vnd.geonext"]
    VndGeonext,
    #[doc = "application/vnd.geoplan"]
    VndGeoplan,
    #[doc = "application/vnd.geospace"]
    VndGeospace,
    #[doc = "application/vnd.gerber"]
    VndGerber,
    #[doc = "application/vnd.globalplatform.card-content-mgt"]
    VndGlobalplatformCardContentMgt,
    #[doc = "application/vnd.globalplatform.card-content-mgt-response"]
    VndGlobalplatformCardContentMgtResponse,
    #[doc = "application/vnd.gnu.taler.exchange+json"]
    VndGnuTalerExchangeJson,
    #[doc = "application/vnd.gnu.taler.merchant+json"]
    VndGnuTalerMerchantJson,
    #[doc = "application/vnd.google-earth.kml+xml"]
    VndGoogleEarthKmlXml,
    #[doc = "application/vnd.google-earth.kmz"]
    VndGoogleEarthKmz,
    #[doc = "application/vnd.gov.sk.e-form+xml"]
    VndGovSkEFormXml,
    #[doc = "application/vnd.gov.sk.e-form+zip"]
    VndGovSkEFormZip,
    #[doc = "application/vnd.gov.sk.xmldatacontainer+xml"]
    VndGovSkXmldatacontainerXml,
    #[doc = "application/vnd.grafeq"]
    VndGrafeq,
    #[doc = "application/vnd.gridmp"]
    VndGridmp,
    #[doc = "application/vnd.groove-account"]
    VndGrooveAccount,
    #[doc = "application/vnd.groove-help"]
    VndGrooveHelp,
    #[doc = "application/vnd.groove-identity-message"]
    VndGrooveIdentityMessage,
    #[doc = "application/vnd.groove-injector"]
    VndGrooveInjector,
    #[doc = "application/vnd.groove-tool-message"]
    VndGrooveToolMessage,
    #[doc = "application/vnd.groove-tool-template"]
    VndGrooveToolTemplate,
    #[doc = "application/vnd.groove-vcard"]
    VndGrooveVcard,
    #[doc = "application/vnd.hal+json"]
    VndHalJson,
    #[doc = "application/vnd.hal+xml"]
    VndHalXml,
    #[doc = "application/vnd.HandHeld-Entertainment+xml"]
    VndHandHeldEntertainmentXml,
    #[doc = "application/vnd.hbci"]
    VndHbci,
    #[doc = "application/vnd.hc+json"]
    VndHcJson,
    #[doc = "application/vnd.hcl-bireports"]
    VndHclBireports,
    #[doc = "application/vnd.hdt"]
    VndHdt,
    #[doc = "application/vnd.heroku+json"]
    VndHerokuJson,
    #[doc = "application/vnd.hhe.lesson-player"]
    VndHheLessonPlayer,
    #[doc = "application/vnd.hp-HPGL"]
    VndHpHPGL,
    #[doc = "application/vnd.hp-hpid"]
    VndHpHpid,
    #[doc = "application/vnd.hp-hps"]
    VndHpHps,
    #[doc = "application/vnd.hp-jlyt"]
    VndHpJlyt,
    #[doc = "application/vnd.hp-PCL"]
    VndHpPCL,
    #[doc = "application/vnd.hp-PCLXL"]
    VndHpPCLXL,
    #[doc = "application/vnd.httphone"]
    VndHttphone,
    #[doc = "application/vnd.hydrostatix.sof-data"]
    VndHydrostatixSofData,
    #[doc = "application/vnd.hyper-item+json"]
    VndHyperItemJson,
    #[doc = "application/vnd.hyper+json"]
    VndHyperJson,
    #[doc = "application/vnd.hyperdrive+json"]
    VndHyperdriveJson,
    #[doc = "application/vnd.hzn-3d-crossword"]
    VndHzn3DCrossword,
    #[doc = "application/vnd.ibm.electronic-media"]
    VndIbmElectronicMedia,
    #[doc = "application/vnd.ibm.MiniPay"]
    VndIbmMiniPay,
    #[doc = "application/vnd.ibm.rights-management"]
    VndIbmRightsManagement,
    #[doc = "application/vnd.ibm.secure-container"]
    VndIbmSecureContainer,
    #[doc = "application/vnd.iccprofile"]
    VndIccprofile,
    #[doc = "application/vnd.ieee.1905"]
    VndIeee1905,
    #[doc = "application/vnd.igloader"]
    VndIgloader,
    #[doc = "application/vnd.imagemeter.folder+zip"]
    VndImagemeterFolderZip,
    #[doc = "application/vnd.imagemeter.image+zip"]
    VndImagemeterImageZip,
    #[doc = "application/vnd.immervision-ivp"]
    VndImmervisionIvp,
    #[doc = "application/vnd.immervision-ivu"]
    VndImmervisionIvu,
    #[doc = "application/vnd.ims.imsccv1p1"]
    VndImsImsccv1P1,
    #[doc = "application/vnd.ims.imsccv1p2"]
    VndImsImsccv1P2,
    #[doc = "application/vnd.ims.imsccv1p3"]
    VndImsImsccv1P3,
    #[doc = "application/vnd.ims.lis.v2.result+json"]
    VndImsLisV2ResultJson,
    #[doc = "application/vnd.ims.lti.v2.toolconsumerprofile+json"]
    VndImsLtiV2ToolconsumerprofileJson,
    #[doc = "application/vnd.ims.lti.v2.toolproxy.id+json"]
    VndImsLtiV2ToolproxyIdJson,
    #[doc = "application/vnd.ims.lti.v2.toolproxy+json"]
    VndImsLtiV2ToolproxyJson,
    #[doc = "application/vnd.ims.lti.v2.toolsettings+json"]
    VndImsLtiV2ToolsettingsJson,
    #[doc = "application/vnd.ims.lti.v2.toolsettings.simple+json"]
    VndImsLtiV2ToolsettingsSimpleJson,
    #[doc = "application/vnd.informedcontrol.rms+xml"]
    VndInformedcontrolRmsXml,
    #[doc = "application/vnd.infotech.project"]
    VndInfotechProject,
    #[doc = "application/vnd.infotech.project+xml"]
    VndInfotechProjectXml,
    #[doc = "application/vnd.innopath.wamp.notification"]
    VndInnopathWampNotification,
    #[doc = "application/vnd.insors.igm"]
    VndInsorsIgm,
    #[doc = "application/vnd.intercon.formnet"]
    VndInterconFormnet,
    #[doc = "application/vnd.intergeo"]
    VndIntergeo,
    #[doc = "application/vnd.intertrust.digibox"]
    VndIntertrustDigibox,
    #[doc = "application/vnd.intertrust.nncp"]
    VndIntertrustNncp,
    #[doc = "application/vnd.intu.qbo"]
    VndIntuQbo,
    #[doc = "application/vnd.intu.qfx"]
    VndIntuQfx,
    #[doc = "application/vnd.ipld.car"]
    VndIpldCar,
    #[doc = "application/vnd.ipld.dag-cbor"]
    VndIpldDagCbor,
    #[doc = "application/vnd.ipld.dag-json"]
    VndIpldDagJson,
    #[doc = "application/vnd.ipld.raw"]
    VndIpldRaw,
    #[doc = "application/vnd.iptc.g2.catalogitem+xml"]
    VndIptcG2CatalogitemXml,
    #[doc = "application/vnd.iptc.g2.conceptitem+xml"]
    VndIptcG2ConceptitemXml,
    #[doc = "application/vnd.iptc.g2.knowledgeitem+xml"]
    VndIptcG2KnowledgeitemXml,
    #[doc = "application/vnd.iptc.g2.newsitem+xml"]
    VndIptcG2NewsitemXml,
    #[doc = "application/vnd.iptc.g2.newsmessage+xml"]
    VndIptcG2NewsmessageXml,
    #[doc = "application/vnd.iptc.g2.packageitem+xml"]
    VndIptcG2PackageitemXml,
    #[doc = "application/vnd.iptc.g2.planningitem+xml"]
    VndIptcG2PlanningitemXml,
    #[doc = "application/vnd.ipunplugged.rcprofile"]
    VndIpunpluggedRcprofile,
    #[doc = "application/vnd.irepository.package+xml"]
    VndIrepositoryPackageXml,
    #[doc = "application/vnd.is-xpr"]
    VndIsXpr,
    #[doc = "application/vnd.isac.fcs"]
    VndIsacFcs,
    #[doc = "application/vnd.jam"]
    VndJam,
    #[doc = "application/vnd.iso11783-10+zip"]
    VndIso1178310Zip,
    #[doc = "application/vnd.japannet-directory-service"]
    VndJapannetDirectoryService,
    #[doc = "application/vnd.japannet-jpnstore-wakeup"]
    VndJapannetJpnstoreWakeup,
    #[doc = "application/vnd.japannet-payment-wakeup"]
    VndJapannetPaymentWakeup,
    #[doc = "application/vnd.japannet-registration"]
    VndJapannetRegistration,
    #[doc = "application/vnd.japannet-registration-wakeup"]
    VndJapannetRegistrationWakeup,
    #[doc = "application/vnd.japannet-setstore-wakeup"]
    VndJapannetSetstoreWakeup,
    #[doc = "application/vnd.japannet-verification"]
    VndJapannetVerification,
    #[doc = "application/vnd.japannet-verification-wakeup"]
    VndJapannetVerificationWakeup,
    #[doc = "application/vnd.jcp.javame.midlet-rms"]
    VndJcpJavameMidletRms,
    #[doc = "application/vnd.jisp"]
    VndJisp,
    #[doc = "application/vnd.joost.joda-archive"]
    VndJoostJodaArchive,
    #[doc = "application/vnd.jsk.isdn-ngn"]
    VndJskIsdnNgn,
    #[doc = "application/vnd.kahootz"]
    VndKahootz,
    #[doc = "application/vnd.kde.karbon"]
    VndKdeKarbon,
    #[doc = "application/vnd.kde.kchart"]
    VndKdeKchart,
    #[doc = "application/vnd.kde.kformula"]
    VndKdeKformula,
    #[doc = "application/vnd.kde.kivio"]
    VndKdeKivio,
    #[doc = "application/vnd.kde.kontour"]
    VndKdeKontour,
    #[doc = "application/vnd.kde.kpresenter"]
    VndKdeKpresenter,
    #[doc = "application/vnd.kde.kspread"]
    VndKdeKspread,
    #[doc = "application/vnd.kde.kword"]
    VndKdeKword,
    #[doc = "application/vnd.kenameaapp"]
    VndKenameaapp,
    #[doc = "application/vnd.kidspiration"]
    VndKidspiration,
    #[doc = "application/vnd.Kinar"]
    VndKinar,
    #[doc = "application/vnd.koan"]
    VndKoan,
    #[doc = "application/vnd.kodak-descriptor"]
    VndKodakDescriptor,
    #[doc = "application/vnd.las"]
    VndLas,
    #[doc = "application/vnd.las.las+json"]
    VndLasLasJson,
    #[doc = "application/vnd.las.las+xml"]
    VndLasLasXml,
    #[doc = "application/vnd.laszip"]
    VndLaszip,
    #[doc = "application/vnd.leap+json"]
    VndLeapJson,
    #[doc = "application/vnd.liberty-request+xml"]
    VndLibertyRequestXml,
    #[doc = "application/vnd.llamagraphics.life-balance.desktop"]
    VndLlamagraphicsLifeBalanceDesktop,
    #[doc = "application/vnd.llamagraphics.life-balance.exchange+xml"]
    VndLlamagraphicsLifeBalanceExchangeXml,
    #[doc = "application/vnd.logipipe.circuit+zip"]
    VndLogipipeCircuitZip,
    #[doc = "application/vnd.loom"]
    VndLoom,
    #[doc = "application/vnd.lotus-1-2-3"]
    VndLotus123,
    #[doc = "application/vnd.lotus-approach"]
    VndLotusApproach,
    #[doc = "application/vnd.lotus-freelance"]
    VndLotusFreelance,
    #[doc = "application/vnd.lotus-notes"]
    VndLotusNotes,
    #[doc = "application/vnd.lotus-organizer"]
    VndLotusOrganizer,
    #[doc = "application/vnd.lotus-screencam"]
    VndLotusScreencam,
    #[doc = "application/vnd.lotus-wordpro"]
    VndLotusWordpro,
    #[doc = "application/vnd.macports.portpkg"]
    VndMacportsPortpkg,
    #[doc = "application/vnd.mapbox-vector-tile"]
    VndMapboxVectorTile,
    #[doc = "application/vnd.marlin.drm.actiontoken+xml"]
    VndMarlinDrmActiontokenXml,
    #[doc = "application/vnd.marlin.drm.conftoken+xml"]
    VndMarlinDrmConftokenXml,
    #[doc = "application/vnd.marlin.drm.license+xml"]
    VndMarlinDrmLicenseXml,
    #[doc = "application/vnd.marlin.drm.mdcf"]
    VndMarlinDrmMdcf,
    #[doc = "application/vnd.mason+json"]
    VndMasonJson,
    #[doc = "application/vnd.maxar.archive.3tz+zip"]
    VndMaxarArchive3TzZip,
    #[doc = "application/vnd.maxmind.maxmind-db"]
    VndMaxmindMaxmindDb,
    #[doc = "application/vnd.mcd"]
    VndMcd,
    #[doc = "application/vnd.medcalcdata"]
    VndMedcalcdata,
    #[doc = "application/vnd.mediastation.cdkey"]
    VndMediastationCdkey,
    #[doc = "application/vnd.meridian-slingshot"]
    VndMeridianSlingshot,
    #[doc = "application/vnd.MFER"]
    VndMFER,
    #[doc = "application/vnd.mfmp"]
    VndMfmp,
    #[doc = "application/vnd.micro+json"]
    VndMicroJson,
    #[doc = "application/vnd.micrografx.flo"]
    VndMicrografxFlo,
    #[doc = "application/vnd.micrografx.igx"]
    VndMicrografxIgx,
    #[doc = "application/vnd.microsoft.portable-executable"]
    VndMicrosoftPortableExecutable,
    #[doc = "application/vnd.microsoft.windows.thumbnail-cache"]
    VndMicrosoftWindowsThumbnailCache,
    #[doc = "application/vnd.miele+json"]
    VndMieleJson,
    #[doc = "application/vnd.mif"]
    VndMif,
    #[doc = "application/vnd.minisoft-hp3000-save"]
    VndMinisoftHp3000Save,
    #[doc = "application/vnd.mitsubishi.misty-guard.trustweb"]
    VndMitsubishiMistyGuardTrustweb,
    #[doc = "application/vnd.Mobius.DAF"]
    VndMobiusDAF,
    #[doc = "application/vnd.Mobius.DIS"]
    VndMobiusDIS,
    #[doc = "application/vnd.Mobius.MBK"]
    VndMobiusMBK,
    #[doc = "application/vnd.Mobius.MQY"]
    VndMobiusMQY,
    #[doc = "application/vnd.Mobius.MSL"]
    VndMobiusMSL,
    #[doc = "application/vnd.Mobius.PLC"]
    VndMobiusPLC,
    #[doc = "application/vnd.Mobius.TXF"]
    VndMobiusTXF,
    #[doc = "application/vnd.mophun.application"]
    VndMophunApplication,
    #[doc = "application/vnd.mophun.certificate"]
    VndMophunCertificate,
    #[doc = "application/vnd.motorola.flexsuite"]
    VndMotorolaFlexsuite,
    #[doc = "application/vnd.motorola.flexsuite.adsi"]
    VndMotorolaFlexsuiteAdsi,
    #[doc = "application/vnd.motorola.flexsuite.fis"]
    VndMotorolaFlexsuiteFis,
    #[doc = "application/vnd.motorola.flexsuite.gotap"]
    VndMotorolaFlexsuiteGotap,
    #[doc = "application/vnd.motorola.flexsuite.kmr"]
    VndMotorolaFlexsuiteKmr,
    #[doc = "application/vnd.motorola.flexsuite.ttc"]
    VndMotorolaFlexsuiteTtc,
    #[doc = "application/vnd.motorola.flexsuite.wem"]
    VndMotorolaFlexsuiteWem,
    #[doc = "application/vnd.motorola.iprm"]
    VndMotorolaIprm,
    #[doc = "application/vnd.mozilla.xul+xml"]
    VndMozillaXulXml,
    #[doc = "application/vnd.ms-artgalry"]
    VndMsArtgalry,
    #[doc = "application/vnd.ms-asf"]
    VndMsAsf,
    #[doc = "application/vnd.ms-cab-compressed"]
    VndMsCabCompressed,
    #[doc = "application/vnd.ms-3mfdocument"]
    VndMs3Mfdocument,
    #[doc = "application/vnd.ms-excel"]
    VndMsExcel,
    #[doc = "application/vnd.ms-excel.addin.macroEnabled.12"]
    VndMsExcelAddinMacroEnabled12,
    #[doc = "application/vnd.ms-excel.sheet.binary.macroEnabled.12"]
    VndMsExcelSheetBinaryMacroEnabled12,
    #[doc = "application/vnd.ms-excel.sheet.macroEnabled.12"]
    VndMsExcelSheetMacroEnabled12,
    #[doc = "application/vnd.ms-excel.template.macroEnabled.12"]
    VndMsExcelTemplateMacroEnabled12,
    #[doc = "application/vnd.ms-fontobject"]
    VndMsFontobject,
    #[doc = "application/vnd.ms-htmlhelp"]
    VndMsHtmlhelp,
    #[doc = "application/vnd.ms-ims"]
    VndMsIms,
    #[doc = "application/vnd.ms-lrm"]
    VndMsLrm,
    #[doc = "application/vnd.ms-office.activeX+xml"]
    VndMsOfficeActiveXXml,
    #[doc = "application/vnd.ms-officetheme"]
    VndMsOfficetheme,
    #[doc = "application/vnd.ms-playready.initiator+xml"]
    VndMsPlayreadyInitiatorXml,
    #[doc = "application/vnd.ms-powerpoint"]
    VndMsPowerpoint,
    #[doc = "application/vnd.ms-powerpoint.addin.macroEnabled.12"]
    VndMsPowerpointAddinMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.presentation.macroEnabled.12"]
    VndMsPowerpointPresentationMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.slide.macroEnabled.12"]
    VndMsPowerpointSlideMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.slideshow.macroEnabled.12"]
    VndMsPowerpointSlideshowMacroEnabled12,
    #[doc = "application/vnd.ms-powerpoint.template.macroEnabled.12"]
    VndMsPowerpointTemplateMacroEnabled12,
    #[doc = "application/vnd.ms-PrintDeviceCapabilities+xml"]
    VndMsPrintDeviceCapabilitiesXml,
    #[doc = "application/vnd.ms-PrintSchemaTicket+xml"]
    VndMsPrintSchemaTicketXml,
    #[doc = "application/vnd.ms-project"]
    VndMsProject,
    #[doc = "application/vnd.ms-tnef"]
    VndMsTnef,
    #[doc = "application/vnd.ms-windows.devicepairing"]
    VndMsWindowsDevicepairing,
    #[doc = "application/vnd.ms-windows.nwprinting.oob"]
    VndMsWindowsNwprintingOob,
    #[doc = "application/vnd.ms-windows.printerpairing"]
    VndMsWindowsPrinterpairing,
    #[doc = "application/vnd.ms-windows.wsd.oob"]
    VndMsWindowsWsdOob,
    #[doc = "application/vnd.ms-wmdrm.lic-chlg-req"]
    VndMsWmdrmLicChlgReq,
    #[doc = "application/vnd.ms-wmdrm.lic-resp"]
    VndMsWmdrmLicResp,
    #[doc = "application/vnd.ms-wmdrm.meter-chlg-req"]
    VndMsWmdrmMeterChlgReq,
    #[doc = "application/vnd.ms-wmdrm.meter-resp"]
    VndMsWmdrmMeterResp,
    #[doc = "application/vnd.ms-word.document.macroEnabled.12"]
    VndMsWordDocumentMacroEnabled12,
    #[doc = "application/vnd.ms-word.template.macroEnabled.12"]
    VndMsWordTemplateMacroEnabled12,
    #[doc = "application/vnd.ms-works"]
    VndMsWorks,
    #[doc = "application/vnd.ms-wpl"]
    VndMsWpl,
    #[doc = "application/vnd.ms-xpsdocument"]
    VndMsXpsdocument,
    #[doc = "application/vnd.msa-disk-image"]
    VndMsaDiskImage,
    #[doc = "application/vnd.mseq"]
    VndMseq,
    #[doc = "application/vnd.msign"]
    VndMsign,
    #[doc = "application/vnd.multiad.creator"]
    VndMultiadCreator,
    #[doc = "application/vnd.multiad.creator.cif"]
    VndMultiadCreatorCif,
    #[doc = "application/vnd.musician"]
    VndMusician,
    #[doc = "application/vnd.music-niff"]
    VndMusicNiff,
    #[doc = "application/vnd.muvee.style"]
    VndMuveeStyle,
    #[doc = "application/vnd.mynfc"]
    VndMynfc,
    #[doc = "application/vnd.nacamar.ybrid+json"]
    VndNacamarYbridJson,
    #[doc = "application/vnd.ncd.control"]
    VndNcdControl,
    #[doc = "application/vnd.ncd.reference"]
    VndNcdReference,
    #[doc = "application/vnd.nearst.inv+json"]
    VndNearstInvJson,
    #[doc = "application/vnd.nebumind.line"]
    VndNebumindLine,
    #[doc = "application/vnd.nervana"]
    VndNervana,
    #[doc = "application/vnd.netfpx"]
    VndNetfpx,
    #[doc = "application/vnd.neurolanguage.nlu"]
    VndNeurolanguageNlu,
    #[doc = "application/vnd.nimn"]
    VndNimn,
    #[doc = "application/vnd.nintendo.snes.rom"]
    VndNintendoSnesRom,
    #[doc = "application/vnd.nintendo.nitro.rom"]
    VndNintendoNitroRom,
    #[doc = "application/vnd.nitf"]
    VndNitf,
    #[doc = "application/vnd.noblenet-directory"]
    VndNoblenetDirectory,
    #[doc = "application/vnd.noblenet-sealer"]
    VndNoblenetSealer,
    #[doc = "application/vnd.noblenet-web"]
    VndNoblenetWeb,
    #[doc = "application/vnd.nokia.catalogs"]
    VndNokiaCatalogs,
    #[doc = "application/vnd.nokia.conml+wbxml"]
    VndNokiaConmlWbxml,
    #[doc = "application/vnd.nokia.conml+xml"]
    VndNokiaConmlXml,
    #[doc = "application/vnd.nokia.iptv.config+xml"]
    VndNokiaIptvConfigXml,
    #[doc = "application/vnd.nokia.iSDS-radio-presets"]
    VndNokiaISDSRadioPresets,
    #[doc = "application/vnd.nokia.landmark+wbxml"]
    VndNokiaLandmarkWbxml,
    #[doc = "application/vnd.nokia.landmark+xml"]
    VndNokiaLandmarkXml,
    #[doc = "application/vnd.nokia.landmarkcollection+xml"]
    VndNokiaLandmarkcollectionXml,
    #[doc = "application/vnd.nokia.ncd"]
    VndNokiaNcd,
    #[doc = "application/vnd.nokia.n-gage.ac+xml"]
    VndNokiaNGageAcXml,
    #[doc = "application/vnd.nokia.n-gage.data"]
    VndNokiaNGageData,
    #[doc = "application/vnd.nokia.pcd+wbxml"]
    VndNokiaPcdWbxml,
    #[doc = "application/vnd.nokia.pcd+xml"]
    VndNokiaPcdXml,
    #[doc = "application/vnd.nokia.radio-preset"]
    VndNokiaRadioPreset,
    #[doc = "application/vnd.nokia.radio-presets"]
    VndNokiaRadioPresets,
    #[doc = "application/vnd.novadigm.EDM"]
    VndNovadigmEDM,
    #[doc = "application/vnd.novadigm.EDX"]
    VndNovadigmEDX,
    #[doc = "application/vnd.novadigm.EXT"]
    VndNovadigmEXT,
    #[doc = "application/vnd.ntt-local.content-share"]
    VndNttLocalContentShare,
    #[doc = "application/vnd.ntt-local.file-transfer"]
    VndNttLocalFileTransfer,
    #[doc = "application/vnd.ntt-local.ogw_remote-access"]
    VndNttLocalOgwRemoteAccess,
    #[doc = "application/vnd.ntt-local.sip-ta_remote"]
    VndNttLocalSipTaRemote,
    #[doc = "application/vnd.ntt-local.sip-ta_tcp_stream"]
    VndNttLocalSipTaTcpStream,
    #[doc = "application/vnd.oasis.opendocument.base"]
    VndOasisOpendocumentBase,
    #[doc = "application/vnd.oasis.opendocument.chart"]
    VndOasisOpendocumentChart,
    #[doc = "application/vnd.oasis.opendocument.chart-template"]
    VndOasisOpendocumentChartTemplate,
    #[doc = "application/vnd.oasis.opendocument.formula"]
    VndOasisOpendocumentFormula,
    #[doc = "application/vnd.oasis.opendocument.formula-template"]
    VndOasisOpendocumentFormulaTemplate,
    #[doc = "application/vnd.oasis.opendocument.graphics"]
    VndOasisOpendocumentGraphics,
    #[doc = "application/vnd.oasis.opendocument.graphics-template"]
    VndOasisOpendocumentGraphicsTemplate,
    #[doc = "application/vnd.oasis.opendocument.image"]
    VndOasisOpendocumentImage,
    #[doc = "application/vnd.oasis.opendocument.image-template"]
    VndOasisOpendocumentImageTemplate,
    #[doc = "application/vnd.oasis.opendocument.presentation"]
    VndOasisOpendocumentPresentation,
    #[doc = "application/vnd.oasis.opendocument.presentation-template"]
    VndOasisOpendocumentPresentationTemplate,
    #[doc = "application/vnd.oasis.opendocument.spreadsheet"]
    VndOasisOpendocumentSpreadsheet,
    #[doc = "application/vnd.oasis.opendocument.spreadsheet-template"]
    VndOasisOpendocumentSpreadsheetTemplate,
    #[doc = "application/vnd.oasis.opendocument.text"]
    VndOasisOpendocumentText,
    #[doc = "application/vnd.oasis.opendocument.text-master"]
    VndOasisOpendocumentTextMaster,
    #[doc = "application/vnd.oasis.opendocument.text-template"]
    VndOasisOpendocumentTextTemplate,
    #[doc = "application/vnd.oasis.opendocument.text-web"]
    VndOasisOpendocumentTextWeb,
    #[doc = "application/vnd.obn"]
    VndObn,
    #[doc = "application/vnd.ocf+cbor"]
    VndOcfCbor,
    #[doc = "application/vnd.oci.image.manifest.v1+json"]
    VndOciImageManifestV1Json,
    #[doc = "application/vnd.oftn.l10n+json"]
    VndOftnL10NJson,
    #[doc = "application/vnd.oipf.contentaccessdownload+xml"]
    VndOipfContentaccessdownloadXml,
    #[doc = "application/vnd.oipf.contentaccessstreaming+xml"]
    VndOipfContentaccessstreamingXml,
    #[doc = "application/vnd.oipf.cspg-hexbinary"]
    VndOipfCspgHexbinary,
    #[doc = "application/vnd.oipf.dae.svg+xml"]
    VndOipfDaeSvgXml,
    #[doc = "application/vnd.oipf.dae.xhtml+xml"]
    VndOipfDaeXhtmlXml,
    #[doc = "application/vnd.oipf.mippvcontrolmessage+xml"]
    VndOipfMippvcontrolmessageXml,
    #[doc = "application/vnd.oipf.pae.gem"]
    VndOipfPaeGem,
    #[doc = "application/vnd.oipf.spdiscovery+xml"]
    VndOipfSpdiscoveryXml,
    #[doc = "application/vnd.oipf.spdlist+xml"]
    VndOipfSpdlistXml,
    #[doc = "application/vnd.oipf.ueprofile+xml"]
    VndOipfUeprofileXml,
    #[doc = "application/vnd.oipf.userprofile+xml"]
    VndOipfUserprofileXml,
    #[doc = "application/vnd.olpc-sugar"]
    VndOlpcSugar,
    #[doc = "application/vnd.oma.bcast.associated-procedure-parameter+xml"]
    VndOmaBcastAssociatedProcedureParameterXml,
    #[doc = "application/vnd.oma.bcast.drm-trigger+xml"]
    VndOmaBcastDrmTriggerXml,
    #[doc = "application/vnd.oma.bcast.imd+xml"]
    VndOmaBcastImdXml,
    #[doc = "application/vnd.oma.bcast.ltkm"]
    VndOmaBcastLtkm,
    #[doc = "application/vnd.oma.bcast.notification+xml"]
    VndOmaBcastNotificationXml,
    #[doc = "application/vnd.oma.bcast.provisioningtrigger"]
    VndOmaBcastProvisioningtrigger,
    #[doc = "application/vnd.oma.bcast.sgboot"]
    VndOmaBcastSgboot,
    #[doc = "application/vnd.oma.bcast.sgdd+xml"]
    VndOmaBcastSgddXml,
    #[doc = "application/vnd.oma.bcast.sgdu"]
    VndOmaBcastSgdu,
    #[doc = "application/vnd.oma.bcast.simple-symbol-container"]
    VndOmaBcastSimpleSymbolContainer,
    #[doc = "application/vnd.oma.bcast.smartcard-trigger+xml"]
    VndOmaBcastSmartcardTriggerXml,
    #[doc = "application/vnd.oma.bcast.sprov+xml"]
    VndOmaBcastSprovXml,
    #[doc = "application/vnd.oma.bcast.stkm"]
    VndOmaBcastStkm,
    #[doc = "application/vnd.oma.cab-address-book+xml"]
    VndOmaCabAddressBookXml,
    #[doc = "application/vnd.oma.cab-feature-handler+xml"]
    VndOmaCabFeatureHandlerXml,
    #[doc = "application/vnd.oma.cab-pcc+xml"]
    VndOmaCabPccXml,
    #[doc = "application/vnd.oma.cab-subs-invite+xml"]
    VndOmaCabSubsInviteXml,
    #[doc = "application/vnd.oma.cab-user-prefs+xml"]
    VndOmaCabUserPrefsXml,
    #[doc = "application/vnd.oma.dcd"]
    VndOmaDcd,
    #[doc = "application/vnd.oma.dcdc"]
    VndOmaDcdc,
    #[doc = "application/vnd.oma.dd2+xml"]
    VndOmaDd2Xml,
    #[doc = "application/vnd.oma.drm.risd+xml"]
    VndOmaDrmRisdXml,
    #[doc = "application/vnd.oma.group-usage-list+xml"]
    VndOmaGroupUsageListXml,
    #[doc = "application/vnd.oma.lwm2m+cbor"]
    VndOmaLwm2MCbor,
    #[doc = "application/vnd.oma.lwm2m+json"]
    VndOmaLwm2MJson,
    #[doc = "application/vnd.oma.lwm2m+tlv"]
    VndOmaLwm2MTlv,
    #[doc = "application/vnd.oma.pal+xml"]
    VndOmaPalXml,
    #[doc = "application/vnd.oma.poc.detailed-progress-report+xml"]
    VndOmaPocDetailedProgressReportXml,
    #[doc = "application/vnd.oma.poc.final-report+xml"]
    VndOmaPocFinalReportXml,
    #[doc = "application/vnd.oma.poc.groups+xml"]
    VndOmaPocGroupsXml,
    #[doc = "application/vnd.oma.poc.invocation-descriptor+xml"]
    VndOmaPocInvocationDescriptorXml,
    #[doc = "application/vnd.oma.poc.optimized-progress-report+xml"]
    VndOmaPocOptimizedProgressReportXml,
    #[doc = "application/vnd.oma.push"]
    VndOmaPush,
    #[doc = "application/vnd.oma.scidm.messages+xml"]
    VndOmaScidmMessagesXml,
    #[doc = "application/vnd.oma.xcap-directory+xml"]
    VndOmaXcapDirectoryXml,
    #[doc = "application/vnd.omads-email+xml"]
    VndOmadsEmailXml,
    #[doc = "application/vnd.omads-file+xml"]
    VndOmadsFileXml,
    #[doc = "application/vnd.omads-folder+xml"]
    VndOmadsFolderXml,
    #[doc = "application/vnd.omaloc-supl-init"]
    VndOmalocSuplInit,
    #[doc = "application/vnd.oma-scws-config"]
    VndOmaScwsConfig,
    #[doc = "application/vnd.oma-scws-http-request"]
    VndOmaScwsHttpRequest,
    #[doc = "application/vnd.oma-scws-http-response"]
    VndOmaScwsHttpResponse,
    #[doc = "application/vnd.onepager"]
    VndOnepager,
    #[doc = "application/vnd.onepagertamp"]
    VndOnepagertamp,
    #[doc = "application/vnd.onepagertamx"]
    VndOnepagertamx,
    #[doc = "application/vnd.onepagertat"]
    VndOnepagertat,
    #[doc = "application/vnd.onepagertatp"]
    VndOnepagertatp,
    #[doc = "application/vnd.onepagertatx"]
    VndOnepagertatx,
    #[doc = "application/vnd.onvif.metadata"]
    VndOnvifMetadata,
    #[doc = "application/vnd.openblox.game-binary"]
    VndOpenbloxGameBinary,
    #[doc = "application/vnd.openblox.game+xml"]
    VndOpenbloxGameXml,
    #[doc = "application/vnd.openeye.oeb"]
    VndOpeneyeOeb,
    #[doc = "application/vnd.openstreetmap.data+xml"]
    VndOpenstreetmapDataXml,
    #[doc = "application/vnd.opentimestamps.ots"]
    VndOpentimestampsOts,
    #[doc = "application/vnd.openxmlformats-officedocument.custom-properties+xml"]
    VndOpenxmlformatsOfficedocumentCustomPropertiesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"]
    VndOpenxmlformatsOfficedocumentCustomXmlPropertiesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawing+xml"]
    VndOpenxmlformatsOfficedocumentDrawingXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"]
    VndOpenxmlformatsOfficedocumentDrawingmlChartXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"]
    VndOpenxmlformatsOfficedocumentDrawingmlChartshapesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramColorsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramDataXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramLayoutXml,
    #[doc = "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"]
    VndOpenxmlformatsOfficedocumentDrawingmlDiagramStyleXml,
    #[doc = "application/vnd.openxmlformats-officedocument.extended-properties+xml"]
    VndOpenxmlformatsOfficedocumentExtendedPropertiesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlCommentAuthorsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlCommentsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlHandoutMasterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlNotesMasterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlNotesSlideXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.presentation"]
    VndOpenxmlformatsOfficedocumentPresentationmlPresentation,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlPresentationMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlPresPropsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slide"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlide,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideLayoutXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideMasterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideshow"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideshow,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideshowMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlSlideUpdateInfoXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlTableStylesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlTagsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.template"]
    VndOpenxmlformatsOfficedocumentPresentationmlTemplate,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlTemplateMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"]
    VndOpenxmlformatsOfficedocumentPresentationmlViewPropsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlCalcChainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlChartsheetXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlCommentsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlConnectionsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlDialogsheetXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlExternalLinkXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheDefinitionXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheRecordsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlQueryTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionHeadersXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionLogXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSharedStringsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSheet,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMetadataXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlStylesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTableSingleCellsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.template"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplate,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplateMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlUserNamesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlVolatileDependenciesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"]
    VndOpenxmlformatsOfficedocumentSpreadsheetmlWorksheetXml,
    #[doc = "application/vnd.openxmlformats-officedocument.theme+xml"]
    VndOpenxmlformatsOfficedocumentThemeXml,
    #[doc = "application/vnd.openxmlformats-officedocument.themeOverride+xml"]
    VndOpenxmlformatsOfficedocumentThemeOverrideXml,
    #[doc = "application/vnd.openxmlformats-officedocument.vmlDrawing"]
    VndOpenxmlformatsOfficedocumentVmlDrawing,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlCommentsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.document"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlDocument,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentGlossaryXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlEndnotesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlFontTableXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlFooterXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlFootnotesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlNumberingXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlSettingsXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlStylesXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.template"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlTemplate,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlTemplateMainXml,
    #[doc = "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"]
    VndOpenxmlformatsOfficedocumentWordprocessingmlWebSettingsXml,
    #[doc = "application/vnd.openxmlformats-package.core-properties+xml"]
    VndOpenxmlformatsPackageCorePropertiesXml,
    #[doc = "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"]
    VndOpenxmlformatsPackageDigitalSignatureXmlsignatureXml,
    #[doc = "application/vnd.openxmlformats-package.relationships+xml"]
    VndOpenxmlformatsPackageRelationshipsXml,
    #[doc = "application/vnd.oracle.resource+json"]
    VndOracleResourceJson,
    #[doc = "application/vnd.orange.indata"]
    VndOrangeIndata,
    #[doc = "application/vnd.osa.netdeploy"]
    VndOsaNetdeploy,
    #[doc = "application/vnd.osgeo.mapguide.package"]
    VndOsgeoMapguidePackage,
    #[doc = "application/vnd.osgi.bundle"]
    VndOsgiBundle,
    #[doc = "application/vnd.osgi.dp"]
    VndOsgiDp,
    #[doc = "application/vnd.osgi.subsystem"]
    VndOsgiSubsystem,
    #[doc = "application/vnd.otps.ct-kip+xml"]
    VndOtpsCtKipXml,
    #[doc = "application/vnd.oxli.countgraph"]
    VndOxliCountgraph,
    #[doc = "application/vnd.pagerduty+json"]
    VndPagerdutyJson,
    #[doc = "application/vnd.palm"]
    VndPalm,
    #[doc = "application/vnd.panoply"]
    VndPanoply,
    #[doc = "application/vnd.paos.xml"]
    VndPaosXml,
    #[doc = "application/vnd.patentdive"]
    VndPatentdive,
    #[doc = "application/vnd.patientecommsdoc"]
    VndPatientecommsdoc,
    #[doc = "application/vnd.pawaafile"]
    VndPawaafile,
    #[doc = "application/vnd.pcos"]
    VndPcos,
    #[doc = "application/vnd.pg.format"]
    VndPgFormat,
    #[doc = "application/vnd.pg.osasli"]
    VndPgOsasli,
    #[doc = "application/vnd.piaccess.application-licence"]
    VndPiaccessApplicationLicence,
    #[doc = "application/vnd.picsel"]
    VndPicsel,
    #[doc = "application/vnd.pmi.widget"]
    VndPmiWidget,
    #[doc = "application/vnd.poc.group-advertisement+xml"]
    VndPocGroupAdvertisementXml,
    #[doc = "application/vnd.pocketlearn"]
    VndPocketlearn,
    #[doc = "application/vnd.powerbuilder6"]
    VndPowerbuilder6,
    #[doc = "application/vnd.powerbuilder6-s"]
    VndPowerbuilder6S,
    #[doc = "application/vnd.powerbuilder7"]
    VndPowerbuilder7,
    #[doc = "application/vnd.powerbuilder75"]
    VndPowerbuilder75,
    #[doc = "application/vnd.powerbuilder75-s"]
    VndPowerbuilder75S,
    #[doc = "application/vnd.powerbuilder7-s"]
    VndPowerbuilder7S,
    #[doc = "application/vnd.preminet"]
    VndPreminet,
    #[doc = "application/vnd.previewsystems.box"]
    VndPreviewsystemsBox,
    #[doc = "application/vnd.proteus.magazine"]
    VndProteusMagazine,
    #[doc = "application/vnd.psfs"]
    VndPsfs,
    #[doc = "application/vnd.publishare-delta-tree"]
    VndPublishareDeltaTree,
    #[doc = "application/vnd.pvi.ptid1"]
    VndPviPtid1,
    #[doc = "application/vnd.pwg-multiplexed"]
    VndPwgMultiplexed,
    #[doc = "application/vnd.pwg-xhtml-print+xml"]
    VndPwgXhtmlPrintXml,
    #[doc = "application/vnd.qualcomm.brew-app-res"]
    VndQualcommBrewAppRes,
    #[doc = "application/vnd.quarantainenet"]
    VndQuarantainenet,
    #[doc = "application/vnd.Quark.QuarkXPress"]
    VndQuarkQuarkXPress,
    #[doc = "application/vnd.quobject-quoxdocument"]
    VndQuobjectQuoxdocument,
    #[doc = "application/vnd.radisys.moml+xml"]
    VndRadisysMomlXml,
    #[doc = "application/vnd.radisys.msml-audit-conf+xml"]
    VndRadisysMsmlAuditConfXml,
    #[doc = "application/vnd.radisys.msml-audit-conn+xml"]
    VndRadisysMsmlAuditConnXml,
    #[doc = "application/vnd.radisys.msml-audit-dialog+xml"]
    VndRadisysMsmlAuditDialogXml,
    #[doc = "application/vnd.radisys.msml-audit-stream+xml"]
    VndRadisysMsmlAuditStreamXml,
    #[doc = "application/vnd.radisys.msml-audit+xml"]
    VndRadisysMsmlAuditXml,
    #[doc = "application/vnd.radisys.msml-conf+xml"]
    VndRadisysMsmlConfXml,
    #[doc = "application/vnd.radisys.msml-dialog-base+xml"]
    VndRadisysMsmlDialogBaseXml,
    #[doc = "application/vnd.radisys.msml-dialog-fax-detect+xml"]
    VndRadisysMsmlDialogFaxDetectXml,
    #[doc = "application/vnd.radisys.msml-dialog-fax-sendrecv+xml"]
    VndRadisysMsmlDialogFaxSendrecvXml,
    #[doc = "application/vnd.radisys.msml-dialog-group+xml"]
    VndRadisysMsmlDialogGroupXml,
    #[doc = "application/vnd.radisys.msml-dialog-speech+xml"]
    VndRadisysMsmlDialogSpeechXml,
    #[doc = "application/vnd.radisys.msml-dialog-transform+xml"]
    VndRadisysMsmlDialogTransformXml,
    #[doc = "application/vnd.radisys.msml-dialog+xml"]
    VndRadisysMsmlDialogXml,
    #[doc = "application/vnd.radisys.msml+xml"]
    VndRadisysMsmlXml,
    #[doc = "application/vnd.rainstor.data"]
    VndRainstorData,
    #[doc = "application/vnd.rapid"]
    VndRapid,
    #[doc = "application/vnd.rar"]
    VndRar,
    #[doc = "application/vnd.realvnc.bed"]
    VndRealvncBed,
    #[doc = "application/vnd.recordare.musicxml"]
    VndRecordareMusicxml,
    #[doc = "application/vnd.recordare.musicxml+xml"]
    VndRecordareMusicxmlXml,
    #[doc = "application/vnd.RenLearn.rlprint"]
    VndRenLearnRlprint,
    #[doc = "application/vnd.resilient.logic"]
    VndResilientLogic,
    #[doc = "application/vnd.restful+json"]
    VndRestfulJson,
    #[doc = "application/vnd.rig.cryptonote"]
    VndRigCryptonote,
    #[doc = "application/vnd.route66.link66+xml"]
    VndRoute66Link66Xml,
    #[doc = "application/vnd.rs-274x"]
    VndRs274X,
    #[doc = "application/vnd.ruckus.download"]
    VndRuckusDownload,
    #[doc = "application/vnd.s3sms"]
    VndS3Sms,
    #[doc = "application/vnd.sailingtracker.track"]
    VndSailingtrackerTrack,
    #[doc = "application/vnd.sar"]
    VndSar,
    #[doc = "application/vnd.sbm.cid"]
    VndSbmCid,
    #[doc = "application/vnd.sbm.mid2"]
    VndSbmMid2,
    #[doc = "application/vnd.scribus"]
    VndScribus,
    #[doc = "application/vnd.sealed.3df"]
    VndSealed3Df,
    #[doc = "application/vnd.sealed.csf"]
    VndSealedCsf,
    #[doc = "application/vnd.sealed.doc"]
    VndSealedDoc,
    #[doc = "application/vnd.sealed.eml"]
    VndSealedEml,
    #[doc = "application/vnd.sealed.mht"]
    VndSealedMht,
    #[doc = "application/vnd.sealed.net"]
    VndSealedNet,
    #[doc = "application/vnd.sealed.ppt"]
    VndSealedPpt,
    #[doc = "application/vnd.sealed.tiff"]
    VndSealedTiff,
    #[doc = "application/vnd.sealed.xls"]
    VndSealedXls,
    #[doc = "application/vnd.sealedmedia.softseal.html"]
    VndSealedmediaSoftsealHtml,
    #[doc = "application/vnd.sealedmedia.softseal.pdf"]
    VndSealedmediaSoftsealPdf,
    #[doc = "application/vnd.seemail"]
    VndSeemail,
    #[doc = "application/vnd.seis+json"]
    VndSeisJson,
    #[doc = "application/vnd.sema"]
    VndSema,
    #[doc = "application/vnd.semd"]
    VndSemd,
    #[doc = "application/vnd.semf"]
    VndSemf,
    #[doc = "application/vnd.shade-save-file"]
    VndShadeSaveFile,
    #[doc = "application/vnd.shana.informed.formdata"]
    VndShanaInformedFormdata,
    #[doc = "application/vnd.shana.informed.formtemplate"]
    VndShanaInformedFormtemplate,
    #[doc = "application/vnd.shana.informed.interchange"]
    VndShanaInformedInterchange,
    #[doc = "application/vnd.shana.informed.package"]
    VndShanaInformedPackage,
    #[doc = "application/vnd.shootproof+json"]
    VndShootproofJson,
    #[doc = "application/vnd.shopkick+json"]
    VndShopkickJson,
    #[doc = "application/vnd.shp"]
    VndShp,
    #[doc = "application/vnd.shx"]
    VndShx,
    #[doc = "application/vnd.sigrok.session"]
    VndSigrokSession,
    #[doc = "application/vnd.SimTech-MindMapper"]
    VndSimTechMindMapper,
    #[doc = "application/vnd.siren+json"]
    VndSirenJson,
    #[doc = "application/vnd.smaf"]
    VndSmaf,
    #[doc = "application/vnd.smart.notebook"]
    VndSmartNotebook,
    #[doc = "application/vnd.smart.teacher"]
    VndSmartTeacher,
    #[doc = "application/vnd.snesdev-page-table"]
    VndSnesdevPageTable,
    #[doc = "application/vnd.software602.filler.form+xml"]
    VndSoftware602FillerFormXml,
    #[doc = "application/vnd.software602.filler.form-xml-zip"]
    VndSoftware602FillerFormXmlZip,
    #[doc = "application/vnd.solent.sdkm+xml"]
    VndSolentSdkmXml,
    #[doc = "application/vnd.spotfire.dxp"]
    VndSpotfireDxp,
    #[doc = "application/vnd.spotfire.sfs"]
    VndSpotfireSfs,
    #[doc = "application/vnd.sqlite3"]
    VndSqlite3,
    #[doc = "application/vnd.sss-cod"]
    VndSssCod,
    #[doc = "application/vnd.sss-dtf"]
    VndSssDtf,
    #[doc = "application/vnd.sss-ntf"]
    VndSssNtf,
    #[doc = "application/vnd.stepmania.package"]
    VndStepmaniaPackage,
    #[doc = "application/vnd.stepmania.stepchart"]
    VndStepmaniaStepchart,
    #[doc = "application/vnd.street-stream"]
    VndStreetStream,
    #[doc = "application/vnd.sun.wadl+xml"]
    VndSunWadlXml,
    #[doc = "application/vnd.sus-calendar"]
    VndSusCalendar,
    #[doc = "application/vnd.svd"]
    VndSvd,
    #[doc = "application/vnd.swiftview-ics"]
    VndSwiftviewIcs,
    #[doc = "application/vnd.sybyl.mol2"]
    VndSybylMol2,
    #[doc = "application/vnd.sycle+xml"]
    VndSycleXml,
    #[doc = "application/vnd.syft+json"]
    VndSyftJson,
    #[doc = "application/vnd.syncml.dm.notification"]
    VndSyncmlDmNotification,
    #[doc = "application/vnd.syncml.dmddf+xml"]
    VndSyncmlDmddfXml,
    #[doc = "application/vnd.syncml.dmtnds+wbxml"]
    VndSyncmlDmtndsWbxml,
    #[doc = "application/vnd.syncml.dmtnds+xml"]
    VndSyncmlDmtndsXml,
    #[doc = "application/vnd.syncml.dmddf+wbxml"]
    VndSyncmlDmddfWbxml,
    #[doc = "application/vnd.syncml.dm+wbxml"]
    VndSyncmlDmWbxml,
    #[doc = "application/vnd.syncml.dm+xml"]
    VndSyncmlDmXml,
    #[doc = "application/vnd.syncml.ds.notification"]
    VndSyncmlDsNotification,
    #[doc = "application/vnd.syncml+xml"]
    VndSyncmlXml,
    #[doc = "application/vnd.tableschema+json"]
    VndTableschemaJson,
    #[doc = "application/vnd.tao.intent-module-archive"]
    VndTaoIntentModuleArchive,
    #[doc = "application/vnd.tcpdump.pcap"]
    VndTcpdumpPcap,
    #[doc = "application/vnd.think-cell.ppttc+json"]
    VndThinkCellPpttcJson,
    #[doc = "application/vnd.tml"]
    VndTml,
    #[doc = "application/vnd.tmd.mediaflex.api+xml"]
    VndTmdMediaflexApiXml,
    #[doc = "application/vnd.tmobile-livetv"]
    VndTmobileLivetv,
    #[doc = "application/vnd.tri.onesource"]
    VndTriOnesource,
    #[doc = "application/vnd.trid.tpt"]
    VndTridTpt,
    #[doc = "application/vnd.triscape.mxs"]
    VndTriscapeMxs,
    #[doc = "application/vnd.trueapp"]
    VndTrueapp,
    #[doc = "application/vnd.truedoc"]
    VndTruedoc,
    #[doc = "application/vnd.ubisoft.webplayer"]
    VndUbisoftWebplayer,
    #[doc = "application/vnd.ufdl"]
    VndUfdl,
    #[doc = "application/vnd.uiq.theme"]
    VndUiqTheme,
    #[doc = "application/vnd.umajin"]
    VndUmajin,
    #[doc = "application/vnd.unity"]
    VndUnity,
    #[doc = "application/vnd.uoml+xml"]
    VndUomlXml,
    #[doc = "application/vnd.uplanet.alert"]
    VndUplanetAlert,
    #[doc = "application/vnd.uplanet.alert-wbxml"]
    VndUplanetAlertWbxml,
    #[doc = "application/vnd.uplanet.bearer-choice"]
    VndUplanetBearerChoice,
    #[doc = "application/vnd.uplanet.bearer-choice-wbxml"]
    VndUplanetBearerChoiceWbxml,
    #[doc = "application/vnd.uplanet.cacheop"]
    VndUplanetCacheop,
    #[doc = "application/vnd.uplanet.cacheop-wbxml"]
    VndUplanetCacheopWbxml,
    #[doc = "application/vnd.uplanet.channel"]
    VndUplanetChannel,
    #[doc = "application/vnd.uplanet.channel-wbxml"]
    VndUplanetChannelWbxml,
    #[doc = "application/vnd.uplanet.list"]
    VndUplanetList,
    #[doc = "application/vnd.uplanet.listcmd"]
    VndUplanetListcmd,
    #[doc = "application/vnd.uplanet.listcmd-wbxml"]
    VndUplanetListcmdWbxml,
    #[doc = "application/vnd.uplanet.list-wbxml"]
    VndUplanetListWbxml,
    #[doc = "application/vnd.uri-map"]
    VndUriMap,
    #[doc = "application/vnd.uplanet.signal"]
    VndUplanetSignal,
    #[doc = "application/vnd.valve.source.material"]
    VndValveSourceMaterial,
    #[doc = "application/vnd.vcx"]
    VndVcx,
    #[doc = "application/vnd.vd-study"]
    VndVdStudy,
    #[doc = "application/vnd.vectorworks"]
    VndVectorworks,
    #[doc = "application/vnd.vel+json"]
    VndVelJson,
    #[doc = "application/vnd.verimatrix.vcas"]
    VndVerimatrixVcas,
    #[doc = "application/vnd.veritone.aion+json"]
    VndVeritoneAionJson,
    #[doc = "application/vnd.veryant.thin"]
    VndVeryantThin,
    #[doc = "application/vnd.ves.encrypted"]
    VndVesEncrypted,
    #[doc = "application/vnd.vidsoft.vidconference"]
    VndVidsoftVidconference,
    #[doc = "application/vnd.visio"]
    VndVisio,
    #[doc = "application/vnd.visionary"]
    VndVisionary,
    #[doc = "application/vnd.vividence.scriptfile"]
    VndVividenceScriptfile,
    #[doc = "application/vnd.vsf"]
    VndVsf,
    #[doc = "application/vnd.wap.sic"]
    VndWapSic,
    #[doc = "application/vnd.wap.slc"]
    VndWapSlc,
    #[doc = "application/vnd.wap.wbxml"]
    VndWapWbxml,
    #[doc = "application/vnd.wap.wmlc"]
    VndWapWmlc,
    #[doc = "application/vnd.wap.wmlscriptc"]
    VndWapWmlscriptc,
    #[doc = "application/vnd.wasmflow.wafl"]
    VndWasmflowWafl,
    #[doc = "application/vnd.webturbo"]
    VndWebturbo,
    #[doc = "application/vnd.wfa.dpp"]
    VndWfaDpp,
    #[doc = "application/vnd.wfa.p2p"]
    VndWfaP2P,
    #[doc = "application/vnd.wfa.wsc"]
    VndWfaWsc,
    #[doc = "application/vnd.windows.devicepairing"]
    VndWindowsDevicepairing,
    #[doc = "application/vnd.wmc"]
    VndWmc,
    #[doc = "application/vnd.wmf.bootstrap"]
    VndWmfBootstrap,
    #[doc = "application/vnd.wolfram.mathematica"]
    VndWolframMathematica,
    #[doc = "application/vnd.wolfram.mathematica.package"]
    VndWolframMathematicaPackage,
    #[doc = "application/vnd.wolfram.player"]
    VndWolframPlayer,
    #[doc = "application/vnd.wordperfect"]
    VndWordperfect,
    #[doc = "application/vnd.wqd"]
    VndWqd,
    #[doc = "application/vnd.wrq-hp3000-labelled"]
    VndWrqHp3000Labelled,
    #[doc = "application/vnd.wt.stf"]
    VndWtStf,
    #[doc = "application/vnd.wv.csp+xml"]
    VndWvCspXml,
    #[doc = "application/vnd.wv.csp+wbxml"]
    VndWvCspWbxml,
    #[doc = "application/vnd.wv.ssp+xml"]
    VndWvSspXml,
    #[doc = "application/vnd.xacml+json"]
    VndXacmlJson,
    #[doc = "application/vnd.xara"]
    VndXara,
    #[doc = "application/vnd.xfdl"]
    VndXfdl,
    #[doc = "application/vnd.xfdl.webform"]
    VndXfdlWebform,
    #[doc = "application/vnd.xmi+xml"]
    VndXmiXml,
    #[doc = "application/vnd.xmpie.cpkg"]
    VndXmpieCpkg,
    #[doc = "application/vnd.xmpie.dpkg"]
    VndXmpieDpkg,
    #[doc = "application/vnd.xmpie.plan"]
    VndXmpiePlan,
    #[doc = "application/vnd.xmpie.ppkg"]
    VndXmpiePpkg,
    #[doc = "application/vnd.xmpie.xlim"]
    VndXmpieXlim,
    #[doc = "application/vnd.yamaha.hv-dic"]
    VndYamahaHvDic,
    #[doc = "application/vnd.yamaha.hv-script"]
    VndYamahaHvScript,
    #[doc = "application/vnd.yamaha.hv-voice"]
    VndYamahaHvVoice,
    #[doc = "application/vnd.yamaha.openscoreformat.osfpvg+xml"]
    VndYamahaOpenscoreformatOsfpvgXml,
    #[doc = "application/vnd.yamaha.openscoreformat"]
    VndYamahaOpenscoreformat,
    #[doc = "application/vnd.yamaha.remote-setup"]
    VndYamahaRemoteSetup,
    #[doc = "application/vnd.yamaha.smaf-audio"]
    VndYamahaSmafAudio,
    #[doc = "application/vnd.yamaha.smaf-phrase"]
    VndYamahaSmafPhrase,
    #[doc = "application/vnd.yamaha.through-ngn"]
    VndYamahaThroughNgn,
    #[doc = "application/vnd.yamaha.tunnel-udpencap"]
    VndYamahaTunnelUdpencap,
    #[doc = "application/vnd.yaoweme"]
    VndYaoweme,
    #[doc = "application/vnd.yellowriver-custom-menu"]
    VndYellowriverCustomMenu,
    #[doc = "application/vnd.zul"]
    VndZul,
    #[doc = "application/vnd.zzazz.deck+xml"]
    VndZzazzDeckXml,
    #[doc = "application/voicexml+xml"]
    VoicexmlXml,
    #[doc = "application/voucher-cms+json"]
    VoucherCmsJson,
    #[doc = "application/vq-rtcpxr"]
    VqRtcpxr,
    #[doc = "application/wasm"]
    Wasm,
    #[doc = "application/watcherinfo+xml"]
    WatcherinfoXml,
    #[doc = "application/webpush-options+json"]
    WebpushOptionsJson,
    #[doc = "application/whoispp-query"]
    WhoisppQuery,
    #[doc = "application/whoispp-response"]
    WhoisppResponse,
    #[doc = "application/widget"]
    Widget,
    #[doc = "application/wita"]
    Wita,
    #[doc = "application/wordperfect5.1"]
    Wordperfect51,
    #[doc = "application/wsdl+xml"]
    WsdlXml,
    #[doc = "application/wspolicy+xml"]
    WspolicyXml,
    #[doc = "application/x-pki-message"]
    XPkiMessage,
    #[doc = "application/x-www-form-urlencoded"]
    XWwwFormUrlencoded,
    #[doc = "application/x-x509-ca-cert"]
    XX509CaCert,
    #[doc = "application/x-x509-ca-ra-cert"]
    XX509CaRaCert,
    #[doc = "application/x-x509-next-ca-cert"]
    XX509NextCaCert,
    #[doc = "application/x400-bp"]
    X400Bp,
    #[doc = "application/xacml+xml"]
    XacmlXml,
    #[doc = "application/xcap-att+xml"]
    XcapAttXml,
    #[doc = "application/xcap-caps+xml"]
    XcapCapsXml,
    #[doc = "application/xcap-diff+xml"]
    XcapDiffXml,
    #[doc = "application/xcap-el+xml"]
    XcapElXml,
    #[doc = "application/xcap-error+xml"]
    XcapErrorXml,
    #[doc = "application/xcap-ns+xml"]
    XcapNsXml,
    #[doc = "application/xcon-conference-info-diff+xml"]
    XconConferenceInfoDiffXml,
    #[doc = "application/xcon-conference-info+xml"]
    XconConferenceInfoXml,
    #[doc = "application/xenc+xml"]
    XencXml,
    #[doc = "application/xfdf"]
    Xfdf,
    #[doc = "application/xhtml+xml"]
    XhtmlXml,
    #[doc = "application/xliff+xml"]
    XliffXml,
    #[doc = "application/xml"]
    Xml,
    #[doc = "application/xml-dtd"]
    XmlDtd,
    #[doc = "application/xml-external-parsed-entity"]
    XmlExternalParsedEntity,
    #[doc = "application/xml-patch+xml"]
    XmlPatchXml,
    #[doc = "application/xmpp+xml"]
    XmppXml,
    #[doc = "application/xop+xml"]
    XopXml,
    #[doc = "application/xslt+xml"]
    XsltXml,
    #[doc = "application/xv+xml"]
    XvXml,
    #[doc = "application/yang"]
    Yang,
    #[doc = "application/yang-data+cbor"]
    YangDataCbor,
    #[doc = "application/yang-data+json"]
    YangDataJson,
    #[doc = "application/yang-data+xml"]
    YangDataXml,
    #[doc = "application/yang-patch+json"]
    YangPatchJson,
    #[doc = "application/yang-patch+xml"]
    YangPatchXml,
    #[doc = "application/yin+xml"]
    YinXml,
    #[doc = "application/zip"]
    Zip,
    #[doc = "application/zlib"]
    Zlib,
    #[doc = "application/zstd"]
    Zstd,
    Other(String),
}
impl ::std::fmt::Display for Application {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self { Application :: _1DInterleavedParityfec => write ! (f , "application/1d-interleaved-parityfec") ? , Application :: _3GpdashQoeReportXml => write ! (f , "application/3gpdash-qoe-report+xml") ? , Application :: _3GppHalJson => write ! (f , "application/3gppHal+json") ? , Application :: _3GppHalFormsJson => write ! (f , "application/3gppHalForms+json") ? , Application :: _3GppImsXml => write ! (f , "application/3gpp-ims+xml") ? , Application :: A2L => write ! (f , "application/A2L") ? , Application :: AceCbor => write ! (f , "application/ace+cbor") ? , Application :: AceJson => write ! (f , "application/ace+json") ? , Application :: Activemessage => write ! (f , "application/activemessage") ? , Application :: ActivityJson => write ! (f , "application/activity+json") ? , Application :: AifCbor => write ! (f , "application/aif+cbor") ? , Application :: AifJson => write ! (f , "application/aif+json") ? , Application :: AltoCdniJson => write ! (f , "application/alto-cdni+json") ? , Application :: AltoCdnifilterJson => write ! (f , "application/alto-cdnifilter+json") ? , Application :: AltoCostmapJson => write ! (f , "application/alto-costmap+json") ? , Application :: AltoCostmapfilterJson => write ! (f , "application/alto-costmapfilter+json") ? , Application :: AltoDirectoryJson => write ! (f , "application/alto-directory+json") ? , Application :: AltoEndpointpropJson => write ! (f , "application/alto-endpointprop+json") ? , Application :: AltoEndpointpropparamsJson => write ! (f , "application/alto-endpointpropparams+json") ? , Application :: AltoEndpointcostJson => write ! (f , "application/alto-endpointcost+json") ? , Application :: AltoEndpointcostparamsJson => write ! (f , "application/alto-endpointcostparams+json") ? , Application :: AltoErrorJson => write ! (f , "application/alto-error+json") ? , Application :: AltoNetworkmapfilterJson => write ! (f , "application/alto-networkmapfilter+json") ? , Application :: AltoNetworkmapJson => write ! (f , "application/alto-networkmap+json") ? , Application :: AltoPropmapJson => write ! (f , "application/alto-propmap+json") ? , Application :: AltoPropmapparamsJson => write ! (f , "application/alto-propmapparams+json") ? , Application :: AltoUpdatestreamcontrolJson => write ! (f , "application/alto-updatestreamcontrol+json") ? , Application :: AltoUpdatestreamparamsJson => write ! (f , "application/alto-updatestreamparams+json") ? , Application :: Aml => write ! (f , "application/AML") ? , Application :: AndrewInset => write ! (f , "application/andrew-inset") ? , Application :: Applefile => write ! (f , "application/applefile") ? , Application :: AtJwt => write ! (f , "application/at+jwt") ? , Application :: Atf => write ! (f , "application/ATF") ? , Application :: Atfx => write ! (f , "application/ATFX") ? , Application :: AtomXml => write ! (f , "application/atom+xml") ? , Application :: AtomcatXml => write ! (f , "application/atomcat+xml") ? , Application :: AtomdeletedXml => write ! (f , "application/atomdeleted+xml") ? , Application :: Atomicmail => write ! (f , "application/atomicmail") ? , Application :: AtomsvcXml => write ! (f , "application/atomsvc+xml") ? , Application :: AtscDwdXml => write ! (f , "application/atsc-dwd+xml") ? , Application :: AtscDynamicEventMessage => write ! (f , "application/atsc-dynamic-event-message") ? , Application :: AtscHeldXml => write ! (f , "application/atsc-held+xml") ? , Application :: AtscRdtJson => write ! (f , "application/atsc-rdt+json") ? , Application :: AtscRsatXml => write ! (f , "application/atsc-rsat+xml") ? , Application :: Atxml => write ! (f , "application/ATXML") ? , Application :: AuthPolicyXml => write ! (f , "application/auth-policy+xml") ? , Application :: AutomationmlAmlXml => write ! (f , "application/automationml-aml+xml") ? , Application :: AutomationmlAmlxZip => write ! (f , "application/automationml-amlx+zip") ? , Application :: BacnetXddZip => write ! (f , "application/bacnet-xdd+zip") ? , Application :: BatchSMTP => write ! (f , "application/batch-SMTP") ? , Application :: BeepXml => write ! (f , "application/beep+xml") ? , Application :: CalendarJson => write ! (f , "application/calendar+json") ? , Application :: CalendarXml => write ! (f , "application/calendar+xml") ? , Application :: CallCompletion => write ! (f , "application/call-completion") ? , Application :: Cals1840 => write ! (f , "application/CALS-1840") ? , Application :: CaptiveJson => write ! (f , "application/captive+json") ? , Application :: Cbor => write ! (f , "application/cbor") ? , Application :: CborSeq => write ! (f , "application/cbor-seq") ? , Application :: Cccex => write ! (f , "application/cccex") ? , Application :: CcmpXml => write ! (f , "application/ccmp+xml") ? , Application :: CcxmlXml => write ! (f , "application/ccxml+xml") ? , Application :: CdaXml => write ! (f , "application/cda+xml") ? , Application :: CdfxXml => write ! (f , "application/CDFX+XML") ? , Application :: CdmiCapability => write ! (f , "application/cdmi-capability") ? , Application :: CdmiContainer => write ! (f , "application/cdmi-container") ? , Application :: CdmiDomain => write ! (f , "application/cdmi-domain") ? , Application :: CdmiObject => write ! (f , "application/cdmi-object") ? , Application :: CdmiQueue => write ! (f , "application/cdmi-queue") ? , Application :: Cdni => write ! (f , "application/cdni") ? , Application :: Cea => write ! (f , "application/CEA") ? , Application :: Cea2018Xml => write ! (f , "application/cea-2018+xml") ? , Application :: CellmlXml => write ! (f , "application/cellml+xml") ? , Application :: Cfw => write ! (f , "application/cfw") ? , Application :: CityJson => write ! (f , "application/city+json") ? , Application :: Clr => write ! (f , "application/clr") ? , Application :: ClueInfoXml => write ! (f , "application/clue_info+xml") ? , Application :: ClueXml => write ! (f , "application/clue+xml") ? , Application :: Cms => write ! (f , "application/cms") ? , Application :: CnrpXml => write ! (f , "application/cnrp+xml") ? , Application :: CoapGroupJson => write ! (f , "application/coap-group+json") ? , Application :: CoapPayload => write ! (f , "application/coap-payload") ? , Application :: Commonground => write ! (f , "application/commonground") ? , Application :: ConciseProblemDetailsCbor => write ! (f , "application/concise-problem-details+cbor") ? , Application :: ConferenceInfoXml => write ! (f , "application/conference-info+xml") ? , Application :: CplXml => write ! (f , "application/cpl+xml") ? , Application :: Cose => write ! (f , "application/cose") ? , Application :: CoseKey => write ! (f , "application/cose-key") ? , Application :: CoseKeySet => write ! (f , "application/cose-key-set") ? , Application :: CoseX509 => write ! (f , "application/cose-x509") ? , Application :: Csrattrs => write ! (f , "application/csrattrs") ? , Application :: CstaXml => write ! (f , "application/csta+xml") ? , Application :: CstadataXml => write ! (f , "application/CSTAdata+xml") ? , Application :: CsvmJson => write ! (f , "application/csvm+json") ? , Application :: Cwl => write ! (f , "application/cwl") ? , Application :: CwlJson => write ! (f , "application/cwl+json") ? , Application :: Cwt => write ! (f , "application/cwt") ? , Application :: Cybercash => write ! (f , "application/cybercash") ? , Application :: DashXml => write ! (f , "application/dash+xml") ? , Application :: DashPatchXml => write ! (f , "application/dash-patch+xml") ? , Application :: Dashdelta => write ! (f , "application/dashdelta") ? , Application :: DavmountXml => write ! (f , "application/davmount+xml") ? , Application :: DcaRft => write ! (f , "application/dca-rft") ? , Application :: Dcd => write ! (f , "application/DCD") ? , Application :: DecDx => write ! (f , "application/dec-dx") ? , Application :: DialogInfoXml => write ! (f , "application/dialog-info+xml") ? , Application :: Dicom => write ! (f , "application/dicom") ? , Application :: DicomJson => write ! (f , "application/dicom+json") ? , Application :: DicomXml => write ! (f , "application/dicom+xml") ? , Application :: Dii => write ! (f , "application/DII") ? , Application :: Dit => write ! (f , "application/DIT") ? , Application :: Dns => write ! (f , "application/dns") ? , Application :: DnsJson => write ! (f , "application/dns+json") ? , Application :: DnsMessage => write ! (f , "application/dns-message") ? , Application :: DotsCbor => write ! (f , "application/dots+cbor") ? , Application :: DskppXml => write ! (f , "application/dskpp+xml") ? , Application :: DsscDer => write ! (f , "application/dssc+der") ? , Application :: DsscXml => write ! (f , "application/dssc+xml") ? , Application :: Dvcs => write ! (f , "application/dvcs") ? , Application :: EdiConsent => write ! (f , "application/EDI-consent") ? , Application :: Edifact => write ! (f , "application/EDIFACT") ? , Application :: EdiX12 => write ! (f , "application/EDI-X12") ? , Application :: Efi => write ! (f , "application/efi") ? , Application :: ElmJson => write ! (f , "application/elm+json") ? , Application :: ElmXml => write ! (f , "application/elm+xml") ? , Application :: EmergencyCallDataCapXml => write ! (f , "application/EmergencyCallData.cap+xml") ? , Application :: EmergencyCallDataCommentXml => write ! (f , "application/EmergencyCallData.Comment+xml") ? , Application :: EmergencyCallDataControlXml => write ! (f , "application/EmergencyCallData.Control+xml") ? , Application :: EmergencyCallDataDeviceInfoXml => write ! (f , "application/EmergencyCallData.DeviceInfo+xml") ? , Application :: EmergencyCallDataECallMSD => write ! (f , "application/EmergencyCallData.eCall.MSD") ? , Application :: EmergencyCallDataLegacyESNJson => write ! (f , "application/EmergencyCallData.LegacyESN+json") ? , Application :: EmergencyCallDataProviderInfoXml => write ! (f , "application/EmergencyCallData.ProviderInfo+xml") ? , Application :: EmergencyCallDataServiceInfoXml => write ! (f , "application/EmergencyCallData.ServiceInfo+xml") ? , Application :: EmergencyCallDataSubscriberInfoXml => write ! (f , "application/EmergencyCallData.SubscriberInfo+xml") ? , Application :: EmergencyCallDataVEDSXml => write ! (f , "application/EmergencyCallData.VEDS+xml") ? , Application :: EmmaXml => write ! (f , "application/emma+xml") ? , Application :: EmotionmlXml => write ! (f , "application/emotionml+xml") ? , Application :: Encaprtp => write ! (f , "application/encaprtp") ? , Application :: EppXml => write ! (f , "application/epp+xml") ? , Application :: EpubZip => write ! (f , "application/epub+zip") ? , Application :: Eshop => write ! (f , "application/eshop") ? , Application :: Example => write ! (f , "application/example") ? , Application :: Exi => write ! (f , "application/exi") ? , Application :: ExpectCtReportJson => write ! (f , "application/expect-ct-report+json") ? , Application :: Express => write ! (f , "application/express") ? , Application :: Fastinfoset => write ! (f , "application/fastinfoset") ? , Application :: Fastsoap => write ! (f , "application/fastsoap") ? , Application :: Fdf => write ! (f , "application/fdf") ? , Application :: FdtXml => write ! (f , "application/fdt+xml") ? , Application :: FhirJson => write ! (f , "application/fhir+json") ? , Application :: FhirXml => write ! (f , "application/fhir+xml") ? , Application :: Fits => write ! (f , "application/fits") ? , Application :: Flexfec => write ! (f , "application/flexfec") ? , Application :: FontTdpfr => write ! (f , "application/font-tdpfr") ? , Application :: FrameworkAttributesXml => write ! (f , "application/framework-attributes+xml") ? , Application :: GeoJson => write ! (f , "application/geo+json") ? , Application :: GeoJsonSeq => write ! (f , "application/geo+json-seq") ? , Application :: GeopackageSqlite3 => write ! (f , "application/geopackage+sqlite3") ? , Application :: GeoxacmlXml => write ! (f , "application/geoxacml+xml") ? , Application :: GltfBuffer => write ! (f , "application/gltf-buffer") ? , Application :: GmlXml => write ! (f , "application/gml+xml") ? , Application :: Gzip => write ! (f , "application/gzip") ? , Application :: H224 => write ! (f , "application/H224") ? , Application :: HeldXml => write ! (f , "application/held+xml") ? , Application :: Hl7V2Xml => write ! (f , "application/hl7v2+xml") ? , Application :: Http => write ! (f , "application/http") ? , Application :: Hyperstudio => write ! (f , "application/hyperstudio") ? , Application :: IbeKeyRequestXml => write ! (f , "application/ibe-key-request+xml") ? , Application :: IbePkgReplyXml => write ! (f , "application/ibe-pkg-reply+xml") ? , Application :: IbePpData => write ! (f , "application/ibe-pp-data") ? , Application :: Iges => write ! (f , "application/iges") ? , Application :: ImIscomposingXml => write ! (f , "application/im-iscomposing+xml") ? , Application :: Index => write ! (f , "application/index") ? , Application :: IndexCmd => write ! (f , "application/index.cmd") ? , Application :: IndexObj => write ! (f , "application/index.obj") ? , Application :: IndexResponse => write ! (f , "application/index.response") ? , Application :: IndexVnd => write ! (f , "application/index.vnd") ? , Application :: InkmlXml => write ! (f , "application/inkml+xml") ? , Application :: Iotp => write ! (f , "application/IOTP") ? , Application :: Ipfix => write ! (f , "application/ipfix") ? , Application :: Ipp => write ! (f , "application/ipp") ? , Application :: Isup => write ! (f , "application/ISUP") ? , Application :: ItsXml => write ! (f , "application/its+xml") ? , Application :: Jf2FeedJson => write ! (f , "application/jf2feed+json") ? , Application :: Jose => write ! (f , "application/jose") ? , Application :: JoseJson => write ! (f , "application/jose+json") ? , Application :: JrdJson => write ! (f , "application/jrd+json") ? , Application :: JscalendarJson => write ! (f , "application/jscalendar+json") ? , Application :: Json => write ! (f , "application/json") ? , Application :: JsonPatchJson => write ! (f , "application/json-patch+json") ? , Application :: JsonSeq => write ! (f , "application/json-seq") ? , Application :: JwkJson => write ! (f , "application/jwk+json") ? , Application :: JwkSetJson => write ! (f , "application/jwk-set+json") ? , Application :: Jwt => write ! (f , "application/jwt") ? , Application :: KpmlRequestXml => write ! (f , "application/kpml-request+xml") ? , Application :: KpmlResponseXml => write ! (f , "application/kpml-response+xml") ? , Application :: LdJson => write ! (f , "application/ld+json") ? , Application :: LgrXml => write ! (f , "application/lgr+xml") ? , Application :: LinkFormat => write ! (f , "application/link-format") ? , Application :: Linkset => write ! (f , "application/linkset") ? , Application :: LinksetJson => write ! (f , "application/linkset+json") ? , Application :: LoadControlXml => write ! (f , "application/load-control+xml") ? , Application :: LogoutJwt => write ! (f , "application/logout+jwt") ? , Application :: LostXml => write ! (f , "application/lost+xml") ? , Application :: LostsyncXml => write ! (f , "application/lostsync+xml") ? , Application :: LpfZip => write ! (f , "application/lpf+zip") ? , Application :: Lxf => write ! (f , "application/LXF") ? , Application :: MacBinhex40 => write ! (f , "application/mac-binhex40") ? , Application :: Macwriteii => write ! (f , "application/macwriteii") ? , Application :: MadsXml => write ! (f , "application/mads+xml") ? , Application :: ManifestJson => write ! (f , "application/manifest+json") ? , Application :: Marc => write ! (f , "application/marc") ? , Application :: MarcxmlXml => write ! (f , "application/marcxml+xml") ? , Application :: Mathematica => write ! (f , "application/mathematica") ? , Application :: MathmlXml => write ! (f , "application/mathml+xml") ? , Application :: MathmlContentXml => write ! (f , "application/mathml-content+xml") ? , Application :: MathmlPresentationXml => write ! (f , "application/mathml-presentation+xml") ? , Application :: MbmsAssociatedProcedureDescriptionXml => write ! (f , "application/mbms-associated-procedure-description+xml") ? , Application :: MbmsDeregisterXml => write ! (f , "application/mbms-deregister+xml") ? , Application :: MbmsEnvelopeXml => write ! (f , "application/mbms-envelope+xml") ? , Application :: MbmsMskResponseXml => write ! (f , "application/mbms-msk-response+xml") ? , Application :: MbmsMskXml => write ! (f , "application/mbms-msk+xml") ? , Application :: MbmsProtectionDescriptionXml => write ! (f , "application/mbms-protection-description+xml") ? , Application :: MbmsReceptionReportXml => write ! (f , "application/mbms-reception-report+xml") ? , Application :: MbmsRegisterResponseXml => write ! (f , "application/mbms-register-response+xml") ? , Application :: MbmsRegisterXml => write ! (f , "application/mbms-register+xml") ? , Application :: MbmsScheduleXml => write ! (f , "application/mbms-schedule+xml") ? , Application :: MbmsUserServiceDescriptionXml => write ! (f , "application/mbms-user-service-description+xml") ? , Application :: Mbox => write ! (f , "application/mbox") ? , Application :: MediaControlXml => write ! (f , "application/media_control+xml") ? , Application :: MediaPolicyDatasetXml => write ! (f , "application/media-policy-dataset+xml") ? , Application :: MediaservercontrolXml => write ! (f , "application/mediaservercontrol+xml") ? , Application :: MergePatchJson => write ! (f , "application/merge-patch+json") ? , Application :: Metalink4Xml => write ! (f , "application/metalink4+xml") ? , Application :: MetsXml => write ! (f , "application/mets+xml") ? , Application :: Mf4 => write ! (f , "application/MF4") ? , Application :: Mikey => write ! (f , "application/mikey") ? , Application :: Mipc => write ! (f , "application/mipc") ? , Application :: MissingBlocksCborSeq => write ! (f , "application/missing-blocks+cbor-seq") ? , Application :: MmtAeiXml => write ! (f , "application/mmt-aei+xml") ? , Application :: MmtUsdXml => write ! (f , "application/mmt-usd+xml") ? , Application :: ModsXml => write ! (f , "application/mods+xml") ? , Application :: MossKeys => write ! (f , "application/moss-keys") ? , Application :: MossSignature => write ! (f , "application/moss-signature") ? , Application :: MosskeyData => write ! (f , "application/mosskey-data") ? , Application :: MosskeyRequest => write ! (f , "application/mosskey-request") ? , Application :: Mp21 => write ! (f , "application/mp21") ? , Application :: Mp4 => write ! (f , "application/mp4") ? , Application :: Mpeg4Generic => write ! (f , "application/mpeg4-generic") ? , Application :: Mpeg4Iod => write ! (f , "application/mpeg4-iod") ? , Application :: Mpeg4IodXmt => write ! (f , "application/mpeg4-iod-xmt") ? , Application :: MrbConsumerXml => write ! (f , "application/mrb-consumer+xml") ? , Application :: MrbPublishXml => write ! (f , "application/mrb-publish+xml") ? , Application :: MscIvrXml => write ! (f , "application/msc-ivr+xml") ? , Application :: MscMixerXml => write ! (f , "application/msc-mixer+xml") ? , Application :: Msword => write ! (f , "application/msword") ? , Application :: MudJson => write ! (f , "application/mud+json") ? , Application :: MultipartCore => write ! (f , "application/multipart-core") ? , Application :: Mxf => write ! (f , "application/mxf") ? , Application :: NQuads => write ! (f , "application/n-quads") ? , Application :: NTriples => write ! (f , "application/n-triples") ? , Application :: Nasdata => write ! (f , "application/nasdata") ? , Application :: NewsCheckgroups => write ! (f , "application/news-checkgroups") ? , Application :: NewsGroupinfo => write ! (f , "application/news-groupinfo") ? , Application :: NewsTransmission => write ! (f , "application/news-transmission") ? , Application :: NlsmlXml => write ! (f , "application/nlsml+xml") ? , Application :: Node => write ! (f , "application/node") ? , Application :: Nss => write ! (f , "application/nss") ? , Application :: OauthAuthzReqJwt => write ! (f , "application/oauth-authz-req+jwt") ? , Application :: ObliviousDnsMessage => write ! (f , "application/oblivious-dns-message") ? , Application :: OcspRequest => write ! (f , "application/ocsp-request") ? , Application :: OcspResponse => write ! (f , "application/ocsp-response") ? , Application :: OctetStream => write ! (f , "application/octet-stream") ? , Application :: Oda => write ! (f , "application/ODA") ? , Application :: OdmXml => write ! (f , "application/odm+xml") ? , Application :: Odx => write ! (f , "application/ODX") ? , Application :: OebpsPackageXml => write ! (f , "application/oebps-package+xml") ? , Application :: Ogg => write ! (f , "application/ogg") ? , Application :: OpcNodesetXml => write ! (f , "application/opc-nodeset+xml") ? , Application :: Oscore => write ! (f , "application/oscore") ? , Application :: Oxps => write ! (f , "application/oxps") ? , Application :: P21 => write ! (f , "application/p21") ? , Application :: P21Zip => write ! (f , "application/p21+zip") ? , Application :: P2POverlayXml => write ! (f , "application/p2p-overlay+xml") ? , Application :: Parityfec => write ! (f , "application/parityfec") ? , Application :: Passport => write ! (f , "application/passport") ? , Application :: PatchOpsErrorXml => write ! (f , "application/patch-ops-error+xml") ? , Application :: Pdf => write ! (f , "application/pdf") ? , Application :: Pdx => write ! (f , "application/PDX") ? , Application :: PemCertificateChain => write ! (f , "application/pem-certificate-chain") ? , Application :: PgpEncrypted => write ! (f , "application/pgp-encrypted") ? , Application :: PgpKeys => write ! (f , "application/pgp-keys") ? , Application :: PgpSignature => write ! (f , "application/pgp-signature") ? , Application :: PidfDiffXml => write ! (f , "application/pidf-diff+xml") ? , Application :: PidfXml => write ! (f , "application/pidf+xml") ? , Application :: Pkcs10 => write ! (f , "application/pkcs10") ? , Application :: Pkcs7Mime => write ! (f , "application/pkcs7-mime") ? , Application :: Pkcs7Signature => write ! (f , "application/pkcs7-signature") ? , Application :: Pkcs8 => write ! (f , "application/pkcs8") ? , Application :: Pkcs8Encrypted => write ! (f , "application/pkcs8-encrypted") ? , Application :: Pkcs12 => write ! (f , "application/pkcs12") ? , Application :: PkixAttrCert => write ! (f , "application/pkix-attr-cert") ? , Application :: PkixCert => write ! (f , "application/pkix-cert") ? , Application :: PkixCrl => write ! (f , "application/pkix-crl") ? , Application :: PkixPkipath => write ! (f , "application/pkix-pkipath") ? , Application :: Pkixcmp => write ! (f , "application/pkixcmp") ? , Application :: PlsXml => write ! (f , "application/pls+xml") ? , Application :: PocSettingsXml => write ! (f , "application/poc-settings+xml") ? , Application :: Postscript => write ! (f , "application/postscript") ? , Application :: PpspTrackerJson => write ! (f , "application/ppsp-tracker+json") ? , Application :: ProblemJson => write ! (f , "application/problem+json") ? , Application :: ProblemXml => write ! (f , "application/problem+xml") ? , Application :: ProvenanceXml => write ! (f , "application/provenance+xml") ? , Application :: PrsAlvestrandTitraxSheet => write ! (f , "application/prs.alvestrand.titrax-sheet") ? , Application :: PrsCww => write ! (f , "application/prs.cww") ? , Application :: PrsCyn => write ! (f , "application/prs.cyn") ? , Application :: PrsHpubZip => write ! (f , "application/prs.hpub+zip") ? , Application :: PrsNprend => write ! (f , "application/prs.nprend") ? , Application :: PrsPlucker => write ! (f , "application/prs.plucker") ? , Application :: PrsRdfXmlCrypt => write ! (f , "application/prs.rdf-xml-crypt") ? , Application :: PrsXsfXml => write ! (f , "application/prs.xsf+xml") ? , Application :: PskcXml => write ! (f , "application/pskc+xml") ? , Application :: PvdJson => write ! (f , "application/pvd+json") ? , Application :: RdfXml => write ! (f , "application/rdf+xml") ? , Application :: RouteApdXml => write ! (f , "application/route-apd+xml") ? , Application :: RouteSTsidXml => write ! (f , "application/route-s-tsid+xml") ? , Application :: RouteUsdXml => write ! (f , "application/route-usd+xml") ? , Application :: Qsig => write ! (f , "application/QSIG") ? , Application :: Raptorfec => write ! (f , "application/raptorfec") ? , Application :: RdapJson => write ! (f , "application/rdap+json") ? , Application :: ReginfoXml => write ! (f , "application/reginfo+xml") ? , Application :: RelaxNgCompactSyntax => write ! (f , "application/relax-ng-compact-syntax") ? , Application :: RemotePrinting => write ! (f , "application/remote-printing") ? , Application :: ReputonJson => write ! (f , "application/reputon+json") ? , Application :: ResourceListsDiffXml => write ! (f , "application/resource-lists-diff+xml") ? , Application :: ResourceListsXml => write ! (f , "application/resource-lists+xml") ? , Application :: RfcXml => write ! (f , "application/rfc+xml") ? , Application :: Riscos => write ! (f , "application/riscos") ? , Application :: RlmiXml => write ! (f , "application/rlmi+xml") ? , Application :: RlsServicesXml => write ! (f , "application/rls-services+xml") ? , Application :: RpkiChecklist => write ! (f , "application/rpki-checklist") ? , Application :: RpkiGhostbusters => write ! (f , "application/rpki-ghostbusters") ? , Application :: RpkiManifest => write ! (f , "application/rpki-manifest") ? , Application :: RpkiPublication => write ! (f , "application/rpki-publication") ? , Application :: RpkiRoa => write ! (f , "application/rpki-roa") ? , Application :: RpkiUpdown => write ! (f , "application/rpki-updown") ? , Application :: Rtf => write ! (f , "application/rtf") ? , Application :: Rtploopback => write ! (f , "application/rtploopback") ? , Application :: Rtx => write ! (f , "application/rtx") ? , Application :: SamlassertionXml => write ! (f , "application/samlassertion+xml") ? , Application :: SamlmetadataXml => write ! (f , "application/samlmetadata+xml") ? , Application :: SarifExternalPropertiesJson => write ! (f , "application/sarif-external-properties+json") ? , Application :: SarifJson => write ! (f , "application/sarif+json") ? , Application :: Sbe => write ! (f , "application/sbe") ? , Application :: SbmlXml => write ! (f , "application/sbml+xml") ? , Application :: ScaipXml => write ! (f , "application/scaip+xml") ? , Application :: ScimJson => write ! (f , "application/scim+json") ? , Application :: ScvpCvRequest => write ! (f , "application/scvp-cv-request") ? , Application :: ScvpCvResponse => write ! (f , "application/scvp-cv-response") ? , Application :: ScvpVpRequest => write ! (f , "application/scvp-vp-request") ? , Application :: ScvpVpResponse => write ! (f , "application/scvp-vp-response") ? , Application :: Sdp => write ! (f , "application/sdp") ? , Application :: SeceventJwt => write ! (f , "application/secevent+jwt") ? , Application :: SenmlEtchCbor => write ! (f , "application/senml-etch+cbor") ? , Application :: SenmlEtchJson => write ! (f , "application/senml-etch+json") ? , Application :: SenmlExi => write ! (f , "application/senml-exi") ? , Application :: SenmlCbor => write ! (f , "application/senml+cbor") ? , Application :: SenmlJson => write ! (f , "application/senml+json") ? , Application :: SenmlXml => write ! (f , "application/senml+xml") ? , Application :: SensmlExi => write ! (f , "application/sensml-exi") ? , Application :: SensmlCbor => write ! (f , "application/sensml+cbor") ? , Application :: SensmlJson => write ! (f , "application/sensml+json") ? , Application :: SensmlXml => write ! (f , "application/sensml+xml") ? , Application :: SepExi => write ! (f , "application/sep-exi") ? , Application :: SepXml => write ! (f , "application/sep+xml") ? , Application :: SessionInfo => write ! (f , "application/session-info") ? , Application :: SetPayment => write ! (f , "application/set-payment") ? , Application :: SetPaymentInitiation => write ! (f , "application/set-payment-initiation") ? , Application :: SetRegistration => write ! (f , "application/set-registration") ? , Application :: SetRegistrationInitiation => write ! (f , "application/set-registration-initiation") ? , Application :: Sgml => write ! (f , "application/SGML") ? , Application :: SgmlOpenCatalog => write ! (f , "application/sgml-open-catalog") ? , Application :: ShfXml => write ! (f , "application/shf+xml") ? , Application :: Sieve => write ! (f , "application/sieve") ? , Application :: SimpleFilterXml => write ! (f , "application/simple-filter+xml") ? , Application :: SimpleMessageSummary => write ! (f , "application/simple-message-summary") ? , Application :: SimpleSymbolContainer => write ! (f , "application/simpleSymbolContainer") ? , Application :: Sipc => write ! (f , "application/sipc") ? , Application :: Slate => write ! (f , "application/slate") ? , Application :: SmilXml => write ! (f , "application/smil+xml") ? , Application :: Smpte336M => write ! (f , "application/smpte336m") ? , Application :: SoapFastinfoset => write ! (f , "application/soap+fastinfoset") ? , Application :: SoapXml => write ! (f , "application/soap+xml") ? , Application :: SparqlQuery => write ! (f , "application/sparql-query") ? , Application :: SpdxJson => write ! (f , "application/spdx+json") ? , Application :: SparqlResultsXml => write ! (f , "application/sparql-results+xml") ? , Application :: SpiritsEventXml => write ! (f , "application/spirits-event+xml") ? , Application :: Sql => write ! (f , "application/sql") ? , Application :: Srgs => write ! (f , "application/srgs") ? , Application :: SrgsXml => write ! (f , "application/srgs+xml") ? , Application :: SruXml => write ! (f , "application/sru+xml") ? , Application :: SsmlXml => write ! (f , "application/ssml+xml") ? , Application :: StixJson => write ! (f , "application/stix+json") ? , Application :: SwidCbor => write ! (f , "application/swid+cbor") ? , Application :: SwidXml => write ! (f , "application/swid+xml") ? , Application :: TampApexUpdate => write ! (f , "application/tamp-apex-update") ? , Application :: TampApexUpdateConfirm => write ! (f , "application/tamp-apex-update-confirm") ? , Application :: TampCommunityUpdate => write ! (f , "application/tamp-community-update") ? , Application :: TampCommunityUpdateConfirm => write ! (f , "application/tamp-community-update-confirm") ? , Application :: TampError => write ! (f , "application/tamp-error") ? , Application :: TampSequenceAdjust => write ! (f , "application/tamp-sequence-adjust") ? , Application :: TampSequenceAdjustConfirm => write ! (f , "application/tamp-sequence-adjust-confirm") ? , Application :: TampStatusQuery => write ! (f , "application/tamp-status-query") ? , Application :: TampStatusResponse => write ! (f , "application/tamp-status-response") ? , Application :: TampUpdate => write ! (f , "application/tamp-update") ? , Application :: TampUpdateConfirm => write ! (f , "application/tamp-update-confirm") ? , Application :: TaxiiJson => write ! (f , "application/taxii+json") ? , Application :: TdJson => write ! (f , "application/td+json") ? , Application :: TeiXml => write ! (f , "application/tei+xml") ? , Application :: TetraIsi => write ! (f , "application/TETRA_ISI") ? , Application :: ThraudXml => write ! (f , "application/thraud+xml") ? , Application :: TimestampQuery => write ! (f , "application/timestamp-query") ? , Application :: TimestampReply => write ! (f , "application/timestamp-reply") ? , Application :: TimestampedData => write ! (f , "application/timestamped-data") ? , Application :: TlsrptGzip => write ! (f , "application/tlsrpt+gzip") ? , Application :: TlsrptJson => write ! (f , "application/tlsrpt+json") ? , Application :: TmJson => write ! (f , "application/tm+json") ? , Application :: Tnauthlist => write ! (f , "application/tnauthlist") ? , Application :: TokenIntrospectionJwt => write ! (f , "application/token-introspection+jwt") ? , Application :: TrickleIceSdpfrag => write ! (f , "application/trickle-ice-sdpfrag") ? , Application :: Trig => write ! (f , "application/trig") ? , Application :: TtmlXml => write ! (f , "application/ttml+xml") ? , Application :: TveTrigger => write ! (f , "application/tve-trigger") ? , Application :: Tzif => write ! (f , "application/tzif") ? , Application :: TzifLeap => write ! (f , "application/tzif-leap") ? , Application :: Ulpfec => write ! (f , "application/ulpfec") ? , Application :: UrcGrpsheetXml => write ! (f , "application/urc-grpsheet+xml") ? , Application :: UrcRessheetXml => write ! (f , "application/urc-ressheet+xml") ? , Application :: UrcTargetdescXml => write ! (f , "application/urc-targetdesc+xml") ? , Application :: UrcUisocketdescXml => write ! (f , "application/urc-uisocketdesc+xml") ? , Application :: VcardJson => write ! (f , "application/vcard+json") ? , Application :: VcardXml => write ! (f , "application/vcard+xml") ? , Application :: Vemmi => write ! (f , "application/vemmi") ? , Application :: Vnd1000MindsDecisionModelXml => write ! (f , "application/vnd.1000minds.decision-model+xml") ? , Application :: Vnd3Gpp5Gnas => write ! (f , "application/vnd.3gpp.5gnas") ? , Application :: Vnd3GppAccessTransferEventsXml => write ! (f , "application/vnd.3gpp.access-transfer-events+xml") ? , Application :: Vnd3GppBsfXml => write ! (f , "application/vnd.3gpp.bsf+xml") ? , Application :: Vnd3GppGMOPXml => write ! (f , "application/vnd.3gpp.GMOP+xml") ? , Application :: Vnd3GppGtpc => write ! (f , "application/vnd.3gpp.gtpc") ? , Application :: Vnd3GppInterworkingData => write ! (f , "application/vnd.3gpp.interworking-data") ? , Application :: Vnd3GppLpp => write ! (f , "application/vnd.3gpp.lpp") ? , Application :: Vnd3GppMcSignallingEar => write ! (f , "application/vnd.3gpp.mc-signalling-ear") ? , Application :: Vnd3GppMcdataAffiliationCommandXml => write ! (f , "application/vnd.3gpp.mcdata-affiliation-command+xml") ? , Application :: Vnd3GppMcdataInfoXml => write ! (f , "application/vnd.3gpp.mcdata-info+xml") ? , Application :: Vnd3GppMcdataMsgstoreCtrlRequestXml => write ! (f , "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml") ? , Application :: Vnd3GppMcdataPayload => write ! (f , "application/vnd.3gpp.mcdata-payload") ? , Application :: Vnd3GppMcdataRegroupXml => write ! (f , "application/vnd.3gpp.mcdata-regroup+xml") ? , Application :: Vnd3GppMcdataServiceConfigXml => write ! (f , "application/vnd.3gpp.mcdata-service-config+xml") ? , Application :: Vnd3GppMcdataSignalling => write ! (f , "application/vnd.3gpp.mcdata-signalling") ? , Application :: Vnd3GppMcdataUeConfigXml => write ! (f , "application/vnd.3gpp.mcdata-ue-config+xml") ? , Application :: Vnd3GppMcdataUserProfileXml => write ! (f , "application/vnd.3gpp.mcdata-user-profile+xml") ? , Application :: Vnd3GppMcpttAffiliationCommandXml => write ! (f , "application/vnd.3gpp.mcptt-affiliation-command+xml") ? , Application :: Vnd3GppMcpttFloorRequestXml => write ! (f , "application/vnd.3gpp.mcptt-floor-request+xml") ? , Application :: Vnd3GppMcpttInfoXml => write ! (f , "application/vnd.3gpp.mcptt-info+xml") ? , Application :: Vnd3GppMcpttLocationInfoXml => write ! (f , "application/vnd.3gpp.mcptt-location-info+xml") ? , Application :: Vnd3GppMcpttMbmsUsageInfoXml => write ! (f , "application/vnd.3gpp.mcptt-mbms-usage-info+xml") ? , Application :: Vnd3GppMcpttServiceConfigXml => write ! (f , "application/vnd.3gpp.mcptt-service-config+xml") ? , Application :: Vnd3GppMcpttSignedXml => write ! (f , "application/vnd.3gpp.mcptt-signed+xml") ? , Application :: Vnd3GppMcpttUeConfigXml => write ! (f , "application/vnd.3gpp.mcptt-ue-config+xml") ? , Application :: Vnd3GppMcpttUeInitConfigXml => write ! (f , "application/vnd.3gpp.mcptt-ue-init-config+xml") ? , Application :: Vnd3GppMcpttUserProfileXml => write ! (f , "application/vnd.3gpp.mcptt-user-profile+xml") ? , Application :: Vnd3GppMcvideoAffiliationCommandXml => write ! (f , "application/vnd.3gpp.mcvideo-affiliation-command+xml") ? , Application :: Vnd3GppMcvideoInfoXml => write ! (f , "application/vnd.3gpp.mcvideo-info+xml") ? , Application :: Vnd3GppMcvideoLocationInfoXml => write ! (f , "application/vnd.3gpp.mcvideo-location-info+xml") ? , Application :: Vnd3GppMcvideoMbmsUsageInfoXml => write ! (f , "application/vnd.3gpp.mcvideo-mbms-usage-info+xml") ? , Application :: Vnd3GppMcvideoServiceConfigXml => write ! (f , "application/vnd.3gpp.mcvideo-service-config+xml") ? , Application :: Vnd3GppMcvideoTransmissionRequestXml => write ! (f , "application/vnd.3gpp.mcvideo-transmission-request+xml") ? , Application :: Vnd3GppMcvideoUeConfigXml => write ! (f , "application/vnd.3gpp.mcvideo-ue-config+xml") ? , Application :: Vnd3GppMcvideoUserProfileXml => write ! (f , "application/vnd.3gpp.mcvideo-user-profile+xml") ? , Application :: Vnd3GppMidCallXml => write ! (f , "application/vnd.3gpp.mid-call+xml") ? , Application :: Vnd3GppNgap => write ! (f , "application/vnd.3gpp.ngap") ? , Application :: Vnd3GppPfcp => write ! (f , "application/vnd.3gpp.pfcp") ? , Application :: Vnd3GppPicBwLarge => write ! (f , "application/vnd.3gpp.pic-bw-large") ? , Application :: Vnd3GppPicBwSmall => write ! (f , "application/vnd.3gpp.pic-bw-small") ? , Application :: Vnd3GppPicBwVar => write ! (f , "application/vnd.3gpp.pic-bw-var") ? , Application :: Vnd3GppProsePc3AXml => write ! (f , "application/vnd.3gpp-prose-pc3a+xml") ? , Application :: Vnd3GppProsePc3AchXml => write ! (f , "application/vnd.3gpp-prose-pc3ach+xml") ? , Application :: Vnd3GppProsePc3ChXml => write ! (f , "application/vnd.3gpp-prose-pc3ch+xml") ? , Application :: Vnd3GppProsePc8Xml => write ! (f , "application/vnd.3gpp-prose-pc8+xml") ? , Application :: Vnd3GppProseXml => write ! (f , "application/vnd.3gpp-prose+xml") ? , Application :: Vnd3GppS1Ap => write ! (f , "application/vnd.3gpp.s1ap") ? , Application :: Vnd3GppSms => write ! (f , "application/vnd.3gpp.sms") ? , Application :: Vnd3GppSmsXml => write ! (f , "application/vnd.3gpp.sms+xml") ? , Application :: Vnd3GppSrvccExtXml => write ! (f , "application/vnd.3gpp.srvcc-ext+xml") ? , Application :: Vnd3GppSRVCCInfoXml => write ! (f , "application/vnd.3gpp.SRVCC-info+xml") ? , Application :: Vnd3GppStateAndEventInfoXml => write ! (f , "application/vnd.3gpp.state-and-event-info+xml") ? , Application :: Vnd3GppUssdXml => write ! (f , "application/vnd.3gpp.ussd+xml") ? , Application :: Vnd3GppV2XLocalServiceInformation => write ! (f , "application/vnd.3gpp-v2x-local-service-information") ? , Application :: Vnd3Gpp2BcmcsinfoXml => write ! (f , "application/vnd.3gpp2.bcmcsinfo+xml") ? , Application :: Vnd3Gpp2Sms => write ! (f , "application/vnd.3gpp2.sms") ? , Application :: Vnd3Gpp2Tcap => write ! (f , "application/vnd.3gpp2.tcap") ? , Application :: Vnd3LightssoftwareImagescal => write ! (f , "application/vnd.3lightssoftware.imagescal") ? , Application :: Vnd3MPostItNotes => write ! (f , "application/vnd.3M.Post-it-Notes") ? , Application :: VndAccpacSimplyAso => write ! (f , "application/vnd.accpac.simply.aso") ? , Application :: VndAccpacSimplyImp => write ! (f , "application/vnd.accpac.simply.imp") ? , Application :: VndAcucobol => write ! (f , "application/vnd.acucobol") ? , Application :: VndAcucorp => write ! (f , "application/vnd.acucorp") ? , Application :: VndAdobeFlashMovie => write ! (f , "application/vnd.adobe.flash.movie") ? , Application :: VndAdobeFormscentralFcdt => write ! (f , "application/vnd.adobe.formscentral.fcdt") ? , Application :: VndAdobeFxp => write ! (f , "application/vnd.adobe.fxp") ? , Application :: VndAdobePartialUpload => write ! (f , "application/vnd.adobe.partial-upload") ? , Application :: VndAdobeXdpXml => write ! (f , "application/vnd.adobe.xdp+xml") ? , Application :: VndAetherImp => write ! (f , "application/vnd.aether.imp") ? , Application :: VndAfpcAfplinedata => write ! (f , "application/vnd.afpc.afplinedata") ? , Application :: VndAfpcAfplinedataPagedef => write ! (f , "application/vnd.afpc.afplinedata-pagedef") ? , Application :: VndAfpcCmocaCmresource => write ! (f , "application/vnd.afpc.cmoca-cmresource") ? , Application :: VndAfpcFocaCharset => write ! (f , "application/vnd.afpc.foca-charset") ? , Application :: VndAfpcFocaCodedfont => write ! (f , "application/vnd.afpc.foca-codedfont") ? , Application :: VndAfpcFocaCodepage => write ! (f , "application/vnd.afpc.foca-codepage") ? , Application :: VndAfpcModca => write ! (f , "application/vnd.afpc.modca") ? , Application :: VndAfpcModcaCmtable => write ! (f , "application/vnd.afpc.modca-cmtable") ? , Application :: VndAfpcModcaFormdef => write ! (f , "application/vnd.afpc.modca-formdef") ? , Application :: VndAfpcModcaMediummap => write ! (f , "application/vnd.afpc.modca-mediummap") ? , Application :: VndAfpcModcaObjectcontainer => write ! (f , "application/vnd.afpc.modca-objectcontainer") ? , Application :: VndAfpcModcaOverlay => write ! (f , "application/vnd.afpc.modca-overlay") ? , Application :: VndAfpcModcaPagesegment => write ! (f , "application/vnd.afpc.modca-pagesegment") ? , Application :: VndAge => write ! (f , "application/vnd.age") ? , Application :: VndAhBarcode => write ! (f , "application/vnd.ah-barcode") ? , Application :: VndAheadSpace => write ! (f , "application/vnd.ahead.space") ? , Application :: VndAirzipFilesecureAzf => write ! (f , "application/vnd.airzip.filesecure.azf") ? , Application :: VndAirzipFilesecureAzs => write ! (f , "application/vnd.airzip.filesecure.azs") ? , Application :: VndAmadeusJson => write ! (f , "application/vnd.amadeus+json") ? , Application :: VndAmazonMobi8Ebook => write ! (f , "application/vnd.amazon.mobi8-ebook") ? , Application :: VndAmericandynamicsAcc => write ! (f , "application/vnd.americandynamics.acc") ? , Application :: VndAmigaAmi => write ! (f , "application/vnd.amiga.ami") ? , Application :: VndAmundsenMazeXml => write ! (f , "application/vnd.amundsen.maze+xml") ? , Application :: VndAndroidOta => write ! (f , "application/vnd.android.ota") ? , Application :: VndAnki => write ! (f , "application/vnd.anki") ? , Application :: VndAnserWebCertificateIssueInitiation => write ! (f , "application/vnd.anser-web-certificate-issue-initiation") ? , Application :: VndAntixGameComponent => write ! (f , "application/vnd.antix.game-component") ? , Application :: VndApacheArrowFile => write ! (f , "application/vnd.apache.arrow.file") ? , Application :: VndApacheArrowStream => write ! (f , "application/vnd.apache.arrow.stream") ? , Application :: VndApacheThriftBinary => write ! (f , "application/vnd.apache.thrift.binary") ? , Application :: VndApacheThriftCompact => write ! (f , "application/vnd.apache.thrift.compact") ? , Application :: VndApacheThriftJson => write ! (f , "application/vnd.apache.thrift.json") ? , Application :: VndApexlang => write ! (f , "application/vnd.apexlang") ? , Application :: VndApiJson => write ! (f , "application/vnd.api+json") ? , Application :: VndAplextorWarrpJson => write ! (f , "application/vnd.aplextor.warrp+json") ? , Application :: VndApothekendeReservationJson => write ! (f , "application/vnd.apothekende.reservation+json") ? , Application :: VndAppleInstallerXml => write ! (f , "application/vnd.apple.installer+xml") ? , Application :: VndAppleKeynote => write ! (f , "application/vnd.apple.keynote") ? , Application :: VndAppleMpegurl => write ! (f , "application/vnd.apple.mpegurl") ? , Application :: VndAppleNumbers => write ! (f , "application/vnd.apple.numbers") ? , Application :: VndApplePages => write ! (f , "application/vnd.apple.pages") ? , Application :: VndAristanetworksSwi => write ! (f , "application/vnd.aristanetworks.swi") ? , Application :: VndArtisanJson => write ! (f , "application/vnd.artisan+json") ? , Application :: VndArtsquare => write ! (f , "application/vnd.artsquare") ? , Application :: VndAstraeaSoftwareIota => write ! (f , "application/vnd.astraea-software.iota") ? , Application :: VndAudiograph => write ! (f , "application/vnd.audiograph") ? , Application :: VndAutopackage => write ! (f , "application/vnd.autopackage") ? , Application :: VndAvalonJson => write ! (f , "application/vnd.avalon+json") ? , Application :: VndAvistarXml => write ! (f , "application/vnd.avistar+xml") ? , Application :: VndBalsamiqBmmlXml => write ! (f , "application/vnd.balsamiq.bmml+xml") ? , Application :: VndBananaAccounting => write ! (f , "application/vnd.banana-accounting") ? , Application :: VndBbfUspError => write ! (f , "application/vnd.bbf.usp.error") ? , Application :: VndBbfUspMsg => write ! (f , "application/vnd.bbf.usp.msg") ? , Application :: VndBbfUspMsgJson => write ! (f , "application/vnd.bbf.usp.msg+json") ? , Application :: VndBalsamiqBmpr => write ! (f , "application/vnd.balsamiq.bmpr") ? , Application :: VndBekitzurStechJson => write ! (f , "application/vnd.bekitzur-stech+json") ? , Application :: VndBelightsoftLhzdZip => write ! (f , "application/vnd.belightsoft.lhzd+zip") ? , Application :: VndBelightsoftLhzlZip => write ! (f , "application/vnd.belightsoft.lhzl+zip") ? , Application :: VndBintMedContent => write ! (f , "application/vnd.bint.med-content") ? , Application :: VndBiopaxRdfXml => write ! (f , "application/vnd.biopax.rdf+xml") ? , Application :: VndBlinkIdbValueWrapper => write ! (f , "application/vnd.blink-idb-value-wrapper") ? , Application :: VndBlueiceMultipass => write ! (f , "application/vnd.blueice.multipass") ? , Application :: VndBluetoothEpOob => write ! (f , "application/vnd.bluetooth.ep.oob") ? , Application :: VndBluetoothLeOob => write ! (f , "application/vnd.bluetooth.le.oob") ? , Application :: VndBmi => write ! (f , "application/vnd.bmi") ? , Application :: VndBpf => write ! (f , "application/vnd.bpf") ? , Application :: VndBpf3 => write ! (f , "application/vnd.bpf3") ? , Application :: VndBusinessobjects => write ! (f , "application/vnd.businessobjects") ? , Application :: VndByuUapiJson => write ! (f , "application/vnd.byu.uapi+json") ? , Application :: VndCabJscript => write ! (f , "application/vnd.cab-jscript") ? , Application :: VndCanonCpdl => write ! (f , "application/vnd.canon-cpdl") ? , Application :: VndCanonLips => write ! (f , "application/vnd.canon-lips") ? , Application :: VndCapasystemsPgJson => write ! (f , "application/vnd.capasystems-pg+json") ? , Application :: VndCendioThinlincClientconf => write ! (f , "application/vnd.cendio.thinlinc.clientconf") ? , Application :: VndCenturySystemsTcpStream => write ! (f , "application/vnd.century-systems.tcp_stream") ? , Application :: VndChemdrawXml => write ! (f , "application/vnd.chemdraw+xml") ? , Application :: VndChessPgn => write ! (f , "application/vnd.chess-pgn") ? , Application :: VndChipnutsKaraokeMmd => write ! (f , "application/vnd.chipnuts.karaoke-mmd") ? , Application :: VndCiedi => write ! (f , "application/vnd.ciedi") ? , Application :: VndCinderella => write ! (f , "application/vnd.cinderella") ? , Application :: VndCirpackIsdnExt => write ! (f , "application/vnd.cirpack.isdn-ext") ? , Application :: VndCitationstylesStyleXml => write ! (f , "application/vnd.citationstyles.style+xml") ? , Application :: VndClaymore => write ! (f , "application/vnd.claymore") ? , Application :: VndCloantoRp9 => write ! (f , "application/vnd.cloanto.rp9") ? , Application :: VndClonkC4Group => write ! (f , "application/vnd.clonk.c4group") ? , Application :: VndCluetrustCartomobileConfig => write ! (f , "application/vnd.cluetrust.cartomobile-config") ? , Application :: VndCluetrustCartomobileConfigPkg => write ! (f , "application/vnd.cluetrust.cartomobile-config-pkg") ? , Application :: VndCncfHelmChartContentV1TarGzip => write ! (f , "application/vnd.cncf.helm.chart.content.v1.tar+gzip") ? , Application :: VndCncfHelmChartProvenanceV1Prov => write ! (f , "application/vnd.cncf.helm.chart.provenance.v1.prov") ? , Application :: VndCoffeescript => write ! (f , "application/vnd.coffeescript") ? , Application :: VndCollabioXodocumentsDocument => write ! (f , "application/vnd.collabio.xodocuments.document") ? , Application :: VndCollabioXodocumentsDocumentTemplate => write ! (f , "application/vnd.collabio.xodocuments.document-template") ? , Application :: VndCollabioXodocumentsPresentation => write ! (f , "application/vnd.collabio.xodocuments.presentation") ? , Application :: VndCollabioXodocumentsPresentationTemplate => write ! (f , "application/vnd.collabio.xodocuments.presentation-template") ? , Application :: VndCollabioXodocumentsSpreadsheet => write ! (f , "application/vnd.collabio.xodocuments.spreadsheet") ? , Application :: VndCollabioXodocumentsSpreadsheetTemplate => write ! (f , "application/vnd.collabio.xodocuments.spreadsheet-template") ? , Application :: VndCollectionDocJson => write ! (f , "application/vnd.collection.doc+json") ? , Application :: VndCollectionJson => write ! (f , "application/vnd.collection+json") ? , Application :: VndCollectionNextJson => write ! (f , "application/vnd.collection.next+json") ? , Application :: VndComicbookRar => write ! (f , "application/vnd.comicbook-rar") ? , Application :: VndComicbookZip => write ! (f , "application/vnd.comicbook+zip") ? , Application :: VndCommerceBattelle => write ! (f , "application/vnd.commerce-battelle") ? , Application :: VndCommonspace => write ! (f , "application/vnd.commonspace") ? , Application :: VndCoreosIgnitionJson => write ! (f , "application/vnd.coreos.ignition+json") ? , Application :: VndCosmocaller => write ! (f , "application/vnd.cosmocaller") ? , Application :: VndContactCmsg => write ! (f , "application/vnd.contact.cmsg") ? , Application :: VndCrickClicker => write ! (f , "application/vnd.crick.clicker") ? , Application :: VndCrickClickerKeyboard => write ! (f , "application/vnd.crick.clicker.keyboard") ? , Application :: VndCrickClickerPalette => write ! (f , "application/vnd.crick.clicker.palette") ? , Application :: VndCrickClickerTemplate => write ! (f , "application/vnd.crick.clicker.template") ? , Application :: VndCrickClickerWordbank => write ! (f , "application/vnd.crick.clicker.wordbank") ? , Application :: VndCriticaltoolsWbsXml => write ! (f , "application/vnd.criticaltools.wbs+xml") ? , Application :: VndCryptiiPipeJson => write ! (f , "application/vnd.cryptii.pipe+json") ? , Application :: VndCryptoShadeFile => write ! (f , "application/vnd.crypto-shade-file") ? , Application :: VndCryptomatorEncrypted => write ! (f , "application/vnd.cryptomator.encrypted") ? , Application :: VndCryptomatorVault => write ! (f , "application/vnd.cryptomator.vault") ? , Application :: VndCtcPosml => write ! (f , "application/vnd.ctc-posml") ? , Application :: VndCtctWsXml => write ! (f , "application/vnd.ctct.ws+xml") ? , Application :: VndCupsPdf => write ! (f , "application/vnd.cups-pdf") ? , Application :: VndCupsPostscript => write ! (f , "application/vnd.cups-postscript") ? , Application :: VndCupsPpd => write ! (f , "application/vnd.cups-ppd") ? , Application :: VndCupsRaster => write ! (f , "application/vnd.cups-raster") ? , Application :: VndCupsRaw => write ! (f , "application/vnd.cups-raw") ? , Application :: VndCurl => write ! (f , "application/vnd.curl") ? , Application :: VndCyanDeanRootXml => write ! (f , "application/vnd.cyan.dean.root+xml") ? , Application :: VndCybank => write ! (f , "application/vnd.cybank") ? , Application :: VndCyclonedxJson => write ! (f , "application/vnd.cyclonedx+json") ? , Application :: VndCyclonedxXml => write ! (f , "application/vnd.cyclonedx+xml") ? , Application :: VndD2LCoursepackage1P0Zip => write ! (f , "application/vnd.d2l.coursepackage1p0+zip") ? , Application :: VndD3MDataset => write ! (f , "application/vnd.d3m-dataset") ? , Application :: VndD3MProblem => write ! (f , "application/vnd.d3m-problem") ? , Application :: VndDart => write ! (f , "application/vnd.dart") ? , Application :: VndDataVisionRdz => write ! (f , "application/vnd.data-vision.rdz") ? , Application :: VndDatalog => write ! (f , "application/vnd.datalog") ? , Application :: VndDatapackageJson => write ! (f , "application/vnd.datapackage+json") ? , Application :: VndDataresourceJson => write ! (f , "application/vnd.dataresource+json") ? , Application :: VndDbf => write ! (f , "application/vnd.dbf") ? , Application :: VndDebianBinaryPackage => write ! (f , "application/vnd.debian.binary-package") ? , Application :: VndDeceData => write ! (f , "application/vnd.dece.data") ? , Application :: VndDeceTtmlXml => write ! (f , "application/vnd.dece.ttml+xml") ? , Application :: VndDeceUnspecified => write ! (f , "application/vnd.dece.unspecified") ? , Application :: VndDeceZip => write ! (f , "application/vnd.dece.zip") ? , Application :: VndDenovoFcselayoutLink => write ! (f , "application/vnd.denovo.fcselayout-link") ? , Application :: VndDesmumeMovie => write ! (f , "application/vnd.desmume.movie") ? , Application :: VndDirBiPlateDlNosuffix => write ! (f , "application/vnd.dir-bi.plate-dl-nosuffix") ? , Application :: VndDmDelegationXml => write ! (f , "application/vnd.dm.delegation+xml") ? , Application :: VndDna => write ! (f , "application/vnd.dna") ? , Application :: VndDocumentJson => write ! (f , "application/vnd.document+json") ? , Application :: VndDolbyMobile1 => write ! (f , "application/vnd.dolby.mobile.1") ? , Application :: VndDolbyMobile2 => write ! (f , "application/vnd.dolby.mobile.2") ? , Application :: VndDoremirScorecloudBinaryDocument => write ! (f , "application/vnd.doremir.scorecloud-binary-document") ? , Application :: VndDpgraph => write ! (f , "application/vnd.dpgraph") ? , Application :: VndDreamfactory => write ! (f , "application/vnd.dreamfactory") ? , Application :: VndDriveJson => write ! (f , "application/vnd.drive+json") ? , Application :: VndDtgLocal => write ! (f , "application/vnd.dtg.local") ? , Application :: VndDtgLocalFlash => write ! (f , "application/vnd.dtg.local.flash") ? , Application :: VndDtgLocalHtml => write ! (f , "application/vnd.dtg.local.html") ? , Application :: VndDvbAit => write ! (f , "application/vnd.dvb.ait") ? , Application :: VndDvbDvbislXml => write ! (f , "application/vnd.dvb.dvbisl+xml") ? , Application :: VndDvbDvbj => write ! (f , "application/vnd.dvb.dvbj") ? , Application :: VndDvbEsgcontainer => write ! (f , "application/vnd.dvb.esgcontainer") ? , Application :: VndDvbIpdcdftnotifaccess => write ! (f , "application/vnd.dvb.ipdcdftnotifaccess") ? , Application :: VndDvbIpdcesgaccess => write ! (f , "application/vnd.dvb.ipdcesgaccess") ? , Application :: VndDvbIpdcesgaccess2 => write ! (f , "application/vnd.dvb.ipdcesgaccess2") ? , Application :: VndDvbIpdcesgpdd => write ! (f , "application/vnd.dvb.ipdcesgpdd") ? , Application :: VndDvbIpdcroaming => write ! (f , "application/vnd.dvb.ipdcroaming") ? , Application :: VndDvbIptvAlfecBase => write ! (f , "application/vnd.dvb.iptv.alfec-base") ? , Application :: VndDvbIptvAlfecEnhancement => write ! (f , "application/vnd.dvb.iptv.alfec-enhancement") ? , Application :: VndDvbNotifAggregateRootXml => write ! (f , "application/vnd.dvb.notif-aggregate-root+xml") ? , Application :: VndDvbNotifContainerXml => write ! (f , "application/vnd.dvb.notif-container+xml") ? , Application :: VndDvbNotifGenericXml => write ! (f , "application/vnd.dvb.notif-generic+xml") ? , Application :: VndDvbNotifIaMsglistXml => write ! (f , "application/vnd.dvb.notif-ia-msglist+xml") ? , Application :: VndDvbNotifIaRegistrationRequestXml => write ! (f , "application/vnd.dvb.notif-ia-registration-request+xml") ? , Application :: VndDvbNotifIaRegistrationResponseXml => write ! (f , "application/vnd.dvb.notif-ia-registration-response+xml") ? , Application :: VndDvbNotifInitXml => write ! (f , "application/vnd.dvb.notif-init+xml") ? , Application :: VndDvbPfr => write ! (f , "application/vnd.dvb.pfr") ? , Application :: VndDvbService => write ! (f , "application/vnd.dvb.service") ? , Application :: VndDxr => write ! (f , "application/vnd.dxr") ? , Application :: VndDynageo => write ! (f , "application/vnd.dynageo") ? , Application :: VndDzr => write ! (f , "application/vnd.dzr") ? , Application :: VndEasykaraokeCdgdownload => write ! (f , "application/vnd.easykaraoke.cdgdownload") ? , Application :: VndEcipRlp => write ! (f , "application/vnd.ecip.rlp") ? , Application :: VndEcdisUpdate => write ! (f , "application/vnd.ecdis-update") ? , Application :: VndEclipseDittoJson => write ! (f , "application/vnd.eclipse.ditto+json") ? , Application :: VndEcowinChart => write ! (f , "application/vnd.ecowin.chart") ? , Application :: VndEcowinFilerequest => write ! (f , "application/vnd.ecowin.filerequest") ? , Application :: VndEcowinFileupdate => write ! (f , "application/vnd.ecowin.fileupdate") ? , Application :: VndEcowinSeries => write ! (f , "application/vnd.ecowin.series") ? , Application :: VndEcowinSeriesrequest => write ! (f , "application/vnd.ecowin.seriesrequest") ? , Application :: VndEcowinSeriesupdate => write ! (f , "application/vnd.ecowin.seriesupdate") ? , Application :: VndEfiImg => write ! (f , "application/vnd.efi.img") ? , Application :: VndEfiIso => write ! (f , "application/vnd.efi.iso") ? , Application :: VndEmclientAccessrequestXml => write ! (f , "application/vnd.emclient.accessrequest+xml") ? , Application :: VndEnliven => write ! (f , "application/vnd.enliven") ? , Application :: VndEnphaseEnvoy => write ! (f , "application/vnd.enphase.envoy") ? , Application :: VndEprintsDataXml => write ! (f , "application/vnd.eprints.data+xml") ? , Application :: VndEpsonEsf => write ! (f , "application/vnd.epson.esf") ? , Application :: VndEpsonMsf => write ! (f , "application/vnd.epson.msf") ? , Application :: VndEpsonQuickanime => write ! (f , "application/vnd.epson.quickanime") ? , Application :: VndEpsonSalt => write ! (f , "application/vnd.epson.salt") ? , Application :: VndEpsonSsf => write ! (f , "application/vnd.epson.ssf") ? , Application :: VndEricssonQuickcall => write ! (f , "application/vnd.ericsson.quickcall") ? , Application :: VndEspassEspassZip => write ! (f , "application/vnd.espass-espass+zip") ? , Application :: VndEszigno3Xml => write ! (f , "application/vnd.eszigno3+xml") ? , Application :: VndEtsiAocXml => write ! (f , "application/vnd.etsi.aoc+xml") ? , Application :: VndEtsiAsicSZip => write ! (f , "application/vnd.etsi.asic-s+zip") ? , Application :: VndEtsiAsicEZip => write ! (f , "application/vnd.etsi.asic-e+zip") ? , Application :: VndEtsiCugXml => write ! (f , "application/vnd.etsi.cug+xml") ? , Application :: VndEtsiIptvcommandXml => write ! (f , "application/vnd.etsi.iptvcommand+xml") ? , Application :: VndEtsiIptvdiscoveryXml => write ! (f , "application/vnd.etsi.iptvdiscovery+xml") ? , Application :: VndEtsiIptvprofileXml => write ! (f , "application/vnd.etsi.iptvprofile+xml") ? , Application :: VndEtsiIptvsadBcXml => write ! (f , "application/vnd.etsi.iptvsad-bc+xml") ? , Application :: VndEtsiIptvsadCodXml => write ! (f , "application/vnd.etsi.iptvsad-cod+xml") ? , Application :: VndEtsiIptvsadNpvrXml => write ! (f , "application/vnd.etsi.iptvsad-npvr+xml") ? , Application :: VndEtsiIptvserviceXml => write ! (f , "application/vnd.etsi.iptvservice+xml") ? , Application :: VndEtsiIptvsyncXml => write ! (f , "application/vnd.etsi.iptvsync+xml") ? , Application :: VndEtsiIptvueprofileXml => write ! (f , "application/vnd.etsi.iptvueprofile+xml") ? , Application :: VndEtsiMcidXml => write ! (f , "application/vnd.etsi.mcid+xml") ? , Application :: VndEtsiMheg5 => write ! (f , "application/vnd.etsi.mheg5") ? , Application :: VndEtsiOverloadControlPolicyDatasetXml => write ! (f , "application/vnd.etsi.overload-control-policy-dataset+xml") ? , Application :: VndEtsiPstnXml => write ! (f , "application/vnd.etsi.pstn+xml") ? , Application :: VndEtsiSciXml => write ! (f , "application/vnd.etsi.sci+xml") ? , Application :: VndEtsiSimservsXml => write ! (f , "application/vnd.etsi.simservs+xml") ? , Application :: VndEtsiTimestampToken => write ! (f , "application/vnd.etsi.timestamp-token") ? , Application :: VndEtsiTslXml => write ! (f , "application/vnd.etsi.tsl+xml") ? , Application :: VndEtsiTslDer => write ! (f , "application/vnd.etsi.tsl.der") ? , Application :: VndEuKasparianCarJson => write ! (f , "application/vnd.eu.kasparian.car+json") ? , Application :: VndEudoraData => write ! (f , "application/vnd.eudora.data") ? , Application :: VndEvolvEcigProfile => write ! (f , "application/vnd.evolv.ecig.profile") ? , Application :: VndEvolvEcigSettings => write ! (f , "application/vnd.evolv.ecig.settings") ? , Application :: VndEvolvEcigTheme => write ! (f , "application/vnd.evolv.ecig.theme") ? , Application :: VndExstreamEmpowerZip => write ! (f , "application/vnd.exstream-empower+zip") ? , Application :: VndExstreamPackage => write ! (f , "application/vnd.exstream-package") ? , Application :: VndEzpixAlbum => write ! (f , "application/vnd.ezpix-album") ? , Application :: VndEzpixPackage => write ! (f , "application/vnd.ezpix-package") ? , Application :: VndFSecureMobile => write ! (f , "application/vnd.f-secure.mobile") ? , Application :: VndFastcopyDiskImage => write ! (f , "application/vnd.fastcopy-disk-image") ? , Application :: VndFamilysearchGedcomZip => write ! (f , "application/vnd.familysearch.gedcom+zip") ? , Application :: VndFdsnMseed => write ! (f , "application/vnd.fdsn.mseed") ? , Application :: VndFdsnSeed => write ! (f , "application/vnd.fdsn.seed") ? , Application :: VndFfsns => write ! (f , "application/vnd.ffsns") ? , Application :: VndFiclabFlbZip => write ! (f , "application/vnd.ficlab.flb+zip") ? , Application :: VndFilmitZfc => write ! (f , "application/vnd.filmit.zfc") ? , Application :: VndFints => write ! (f , "application/vnd.fints") ? , Application :: VndFiremonkeysCloudcell => write ! (f , "application/vnd.firemonkeys.cloudcell") ? , Application :: VndFloGraphIt => write ! (f , "application/vnd.FloGraphIt") ? , Application :: VndFluxtimeClip => write ! (f , "application/vnd.fluxtime.clip") ? , Application :: VndFontFontforgeSfd => write ! (f , "application/vnd.font-fontforge-sfd") ? , Application :: VndFramemaker => write ! (f , "application/vnd.framemaker") ? , Application :: VndFscWeblaunch => write ! (f , "application/vnd.fsc.weblaunch") ? , Application :: VndFujifilmFbDocuworks => write ! (f , "application/vnd.fujifilm.fb.docuworks") ? , Application :: VndFujifilmFbDocuworksBinder => write ! (f , "application/vnd.fujifilm.fb.docuworks.binder") ? , Application :: VndFujifilmFbDocuworksContainer => write ! (f , "application/vnd.fujifilm.fb.docuworks.container") ? , Application :: VndFujifilmFbJfiXml => write ! (f , "application/vnd.fujifilm.fb.jfi+xml") ? , Application :: VndFujitsuOasys => write ! (f , "application/vnd.fujitsu.oasys") ? , Application :: VndFujitsuOasys2 => write ! (f , "application/vnd.fujitsu.oasys2") ? , Application :: VndFujitsuOasys3 => write ! (f , "application/vnd.fujitsu.oasys3") ? , Application :: VndFujitsuOasysgp => write ! (f , "application/vnd.fujitsu.oasysgp") ? , Application :: VndFujitsuOasysprs => write ! (f , "application/vnd.fujitsu.oasysprs") ? , Application :: VndFujixeroxART4 => write ! (f , "application/vnd.fujixerox.ART4") ? , Application :: VndFujixeroxARTEX => write ! (f , "application/vnd.fujixerox.ART-EX") ? , Application :: VndFujixeroxDdd => write ! (f , "application/vnd.fujixerox.ddd") ? , Application :: VndFujixeroxDocuworks => write ! (f , "application/vnd.fujixerox.docuworks") ? , Application :: VndFujixeroxDocuworksBinder => write ! (f , "application/vnd.fujixerox.docuworks.binder") ? , Application :: VndFujixeroxDocuworksContainer => write ! (f , "application/vnd.fujixerox.docuworks.container") ? , Application :: VndFujixeroxHBPL => write ! (f , "application/vnd.fujixerox.HBPL") ? , Application :: VndFutMisnet => write ! (f , "application/vnd.fut-misnet") ? , Application :: VndFutoinCbor => write ! (f , "application/vnd.futoin+cbor") ? , Application :: VndFutoinJson => write ! (f , "application/vnd.futoin+json") ? , Application :: VndFuzzysheet => write ! (f , "application/vnd.fuzzysheet") ? , Application :: VndGenomatixTuxedo => write ! (f , "application/vnd.genomatix.tuxedo") ? , Application :: VndGenozip => write ! (f , "application/vnd.genozip") ? , Application :: VndGenticsGrdJson => write ! (f , "application/vnd.gentics.grd+json") ? , Application :: VndGentooCatmetadataXml => write ! (f , "application/vnd.gentoo.catmetadata+xml") ? , Application :: VndGentooEbuild => write ! (f , "application/vnd.gentoo.ebuild") ? , Application :: VndGentooEclass => write ! (f , "application/vnd.gentoo.eclass") ? , Application :: VndGentooManifest => write ! (f , "application/vnd.gentoo.manifest") ? , Application :: VndGentooPkgmetadataXml => write ! (f , "application/vnd.gentoo.pkgmetadata+xml") ? , Application :: VndGeogebraFile => write ! (f , "application/vnd.geogebra.file") ? , Application :: VndGeogebraSlides => write ! (f , "application/vnd.geogebra.slides") ? , Application :: VndGeogebraTool => write ! (f , "application/vnd.geogebra.tool") ? , Application :: VndGeometryExplorer => write ! (f , "application/vnd.geometry-explorer") ? , Application :: VndGeonext => write ! (f , "application/vnd.geonext") ? , Application :: VndGeoplan => write ! (f , "application/vnd.geoplan") ? , Application :: VndGeospace => write ! (f , "application/vnd.geospace") ? , Application :: VndGerber => write ! (f , "application/vnd.gerber") ? , Application :: VndGlobalplatformCardContentMgt => write ! (f , "application/vnd.globalplatform.card-content-mgt") ? , Application :: VndGlobalplatformCardContentMgtResponse => write ! (f , "application/vnd.globalplatform.card-content-mgt-response") ? , Application :: VndGnuTalerExchangeJson => write ! (f , "application/vnd.gnu.taler.exchange+json") ? , Application :: VndGnuTalerMerchantJson => write ! (f , "application/vnd.gnu.taler.merchant+json") ? , Application :: VndGoogleEarthKmlXml => write ! (f , "application/vnd.google-earth.kml+xml") ? , Application :: VndGoogleEarthKmz => write ! (f , "application/vnd.google-earth.kmz") ? , Application :: VndGovSkEFormXml => write ! (f , "application/vnd.gov.sk.e-form+xml") ? , Application :: VndGovSkEFormZip => write ! (f , "application/vnd.gov.sk.e-form+zip") ? , Application :: VndGovSkXmldatacontainerXml => write ! (f , "application/vnd.gov.sk.xmldatacontainer+xml") ? , Application :: VndGrafeq => write ! (f , "application/vnd.grafeq") ? , Application :: VndGridmp => write ! (f , "application/vnd.gridmp") ? , Application :: VndGrooveAccount => write ! (f , "application/vnd.groove-account") ? , Application :: VndGrooveHelp => write ! (f , "application/vnd.groove-help") ? , Application :: VndGrooveIdentityMessage => write ! (f , "application/vnd.groove-identity-message") ? , Application :: VndGrooveInjector => write ! (f , "application/vnd.groove-injector") ? , Application :: VndGrooveToolMessage => write ! (f , "application/vnd.groove-tool-message") ? , Application :: VndGrooveToolTemplate => write ! (f , "application/vnd.groove-tool-template") ? , Application :: VndGrooveVcard => write ! (f , "application/vnd.groove-vcard") ? , Application :: VndHalJson => write ! (f , "application/vnd.hal+json") ? , Application :: VndHalXml => write ! (f , "application/vnd.hal+xml") ? , Application :: VndHandHeldEntertainmentXml => write ! (f , "application/vnd.HandHeld-Entertainment+xml") ? , Application :: VndHbci => write ! (f , "application/vnd.hbci") ? , Application :: VndHcJson => write ! (f , "application/vnd.hc+json") ? , Application :: VndHclBireports => write ! (f , "application/vnd.hcl-bireports") ? , Application :: VndHdt => write ! (f , "application/vnd.hdt") ? , Application :: VndHerokuJson => write ! (f , "application/vnd.heroku+json") ? , Application :: VndHheLessonPlayer => write ! (f , "application/vnd.hhe.lesson-player") ? , Application :: VndHpHPGL => write ! (f , "application/vnd.hp-HPGL") ? , Application :: VndHpHpid => write ! (f , "application/vnd.hp-hpid") ? , Application :: VndHpHps => write ! (f , "application/vnd.hp-hps") ? , Application :: VndHpJlyt => write ! (f , "application/vnd.hp-jlyt") ? , Application :: VndHpPCL => write ! (f , "application/vnd.hp-PCL") ? , Application :: VndHpPCLXL => write ! (f , "application/vnd.hp-PCLXL") ? , Application :: VndHttphone => write ! (f , "application/vnd.httphone") ? , Application :: VndHydrostatixSofData => write ! (f , "application/vnd.hydrostatix.sof-data") ? , Application :: VndHyperItemJson => write ! (f , "application/vnd.hyper-item+json") ? , Application :: VndHyperJson => write ! (f , "application/vnd.hyper+json") ? , Application :: VndHyperdriveJson => write ! (f , "application/vnd.hyperdrive+json") ? , Application :: VndHzn3DCrossword => write ! (f , "application/vnd.hzn-3d-crossword") ? , Application :: VndIbmElectronicMedia => write ! (f , "application/vnd.ibm.electronic-media") ? , Application :: VndIbmMiniPay => write ! (f , "application/vnd.ibm.MiniPay") ? , Application :: VndIbmRightsManagement => write ! (f , "application/vnd.ibm.rights-management") ? , Application :: VndIbmSecureContainer => write ! (f , "application/vnd.ibm.secure-container") ? , Application :: VndIccprofile => write ! (f , "application/vnd.iccprofile") ? , Application :: VndIeee1905 => write ! (f , "application/vnd.ieee.1905") ? , Application :: VndIgloader => write ! (f , "application/vnd.igloader") ? , Application :: VndImagemeterFolderZip => write ! (f , "application/vnd.imagemeter.folder+zip") ? , Application :: VndImagemeterImageZip => write ! (f , "application/vnd.imagemeter.image+zip") ? , Application :: VndImmervisionIvp => write ! (f , "application/vnd.immervision-ivp") ? , Application :: VndImmervisionIvu => write ! (f , "application/vnd.immervision-ivu") ? , Application :: VndImsImsccv1P1 => write ! (f , "application/vnd.ims.imsccv1p1") ? , Application :: VndImsImsccv1P2 => write ! (f , "application/vnd.ims.imsccv1p2") ? , Application :: VndImsImsccv1P3 => write ! (f , "application/vnd.ims.imsccv1p3") ? , Application :: VndImsLisV2ResultJson => write ! (f , "application/vnd.ims.lis.v2.result+json") ? , Application :: VndImsLtiV2ToolconsumerprofileJson => write ! (f , "application/vnd.ims.lti.v2.toolconsumerprofile+json") ? , Application :: VndImsLtiV2ToolproxyIdJson => write ! (f , "application/vnd.ims.lti.v2.toolproxy.id+json") ? , Application :: VndImsLtiV2ToolproxyJson => write ! (f , "application/vnd.ims.lti.v2.toolproxy+json") ? , Application :: VndImsLtiV2ToolsettingsJson => write ! (f , "application/vnd.ims.lti.v2.toolsettings+json") ? , Application :: VndImsLtiV2ToolsettingsSimpleJson => write ! (f , "application/vnd.ims.lti.v2.toolsettings.simple+json") ? , Application :: VndInformedcontrolRmsXml => write ! (f , "application/vnd.informedcontrol.rms+xml") ? , Application :: VndInfotechProject => write ! (f , "application/vnd.infotech.project") ? , Application :: VndInfotechProjectXml => write ! (f , "application/vnd.infotech.project+xml") ? , Application :: VndInnopathWampNotification => write ! (f , "application/vnd.innopath.wamp.notification") ? , Application :: VndInsorsIgm => write ! (f , "application/vnd.insors.igm") ? , Application :: VndInterconFormnet => write ! (f , "application/vnd.intercon.formnet") ? , Application :: VndIntergeo => write ! (f , "application/vnd.intergeo") ? , Application :: VndIntertrustDigibox => write ! (f , "application/vnd.intertrust.digibox") ? , Application :: VndIntertrustNncp => write ! (f , "application/vnd.intertrust.nncp") ? , Application :: VndIntuQbo => write ! (f , "application/vnd.intu.qbo") ? , Application :: VndIntuQfx => write ! (f , "application/vnd.intu.qfx") ? , Application :: VndIpldCar => write ! (f , "application/vnd.ipld.car") ? , Application :: VndIpldDagCbor => write ! (f , "application/vnd.ipld.dag-cbor") ? , Application :: VndIpldDagJson => write ! (f , "application/vnd.ipld.dag-json") ? , Application :: VndIpldRaw => write ! (f , "application/vnd.ipld.raw") ? , Application :: VndIptcG2CatalogitemXml => write ! (f , "application/vnd.iptc.g2.catalogitem+xml") ? , Application :: VndIptcG2ConceptitemXml => write ! (f , "application/vnd.iptc.g2.conceptitem+xml") ? , Application :: VndIptcG2KnowledgeitemXml => write ! (f , "application/vnd.iptc.g2.knowledgeitem+xml") ? , Application :: VndIptcG2NewsitemXml => write ! (f , "application/vnd.iptc.g2.newsitem+xml") ? , Application :: VndIptcG2NewsmessageXml => write ! (f , "application/vnd.iptc.g2.newsmessage+xml") ? , Application :: VndIptcG2PackageitemXml => write ! (f , "application/vnd.iptc.g2.packageitem+xml") ? , Application :: VndIptcG2PlanningitemXml => write ! (f , "application/vnd.iptc.g2.planningitem+xml") ? , Application :: VndIpunpluggedRcprofile => write ! (f , "application/vnd.ipunplugged.rcprofile") ? , Application :: VndIrepositoryPackageXml => write ! (f , "application/vnd.irepository.package+xml") ? , Application :: VndIsXpr => write ! (f , "application/vnd.is-xpr") ? , Application :: VndIsacFcs => write ! (f , "application/vnd.isac.fcs") ? , Application :: VndJam => write ! (f , "application/vnd.jam") ? , Application :: VndIso1178310Zip => write ! (f , "application/vnd.iso11783-10+zip") ? , Application :: VndJapannetDirectoryService => write ! (f , "application/vnd.japannet-directory-service") ? , Application :: VndJapannetJpnstoreWakeup => write ! (f , "application/vnd.japannet-jpnstore-wakeup") ? , Application :: VndJapannetPaymentWakeup => write ! (f , "application/vnd.japannet-payment-wakeup") ? , Application :: VndJapannetRegistration => write ! (f , "application/vnd.japannet-registration") ? , Application :: VndJapannetRegistrationWakeup => write ! (f , "application/vnd.japannet-registration-wakeup") ? , Application :: VndJapannetSetstoreWakeup => write ! (f , "application/vnd.japannet-setstore-wakeup") ? , Application :: VndJapannetVerification => write ! (f , "application/vnd.japannet-verification") ? , Application :: VndJapannetVerificationWakeup => write ! (f , "application/vnd.japannet-verification-wakeup") ? , Application :: VndJcpJavameMidletRms => write ! (f , "application/vnd.jcp.javame.midlet-rms") ? , Application :: VndJisp => write ! (f , "application/vnd.jisp") ? , Application :: VndJoostJodaArchive => write ! (f , "application/vnd.joost.joda-archive") ? , Application :: VndJskIsdnNgn => write ! (f , "application/vnd.jsk.isdn-ngn") ? , Application :: VndKahootz => write ! (f , "application/vnd.kahootz") ? , Application :: VndKdeKarbon => write ! (f , "application/vnd.kde.karbon") ? , Application :: VndKdeKchart => write ! (f , "application/vnd.kde.kchart") ? , Application :: VndKdeKformula => write ! (f , "application/vnd.kde.kformula") ? , Application :: VndKdeKivio => write ! (f , "application/vnd.kde.kivio") ? , Application :: VndKdeKontour => write ! (f , "application/vnd.kde.kontour") ? , Application :: VndKdeKpresenter => write ! (f , "application/vnd.kde.kpresenter") ? , Application :: VndKdeKspread => write ! (f , "application/vnd.kde.kspread") ? , Application :: VndKdeKword => write ! (f , "application/vnd.kde.kword") ? , Application :: VndKenameaapp => write ! (f , "application/vnd.kenameaapp") ? , Application :: VndKidspiration => write ! (f , "application/vnd.kidspiration") ? , Application :: VndKinar => write ! (f , "application/vnd.Kinar") ? , Application :: VndKoan => write ! (f , "application/vnd.koan") ? , Application :: VndKodakDescriptor => write ! (f , "application/vnd.kodak-descriptor") ? , Application :: VndLas => write ! (f , "application/vnd.las") ? , Application :: VndLasLasJson => write ! (f , "application/vnd.las.las+json") ? , Application :: VndLasLasXml => write ! (f , "application/vnd.las.las+xml") ? , Application :: VndLaszip => write ! (f , "application/vnd.laszip") ? , Application :: VndLeapJson => write ! (f , "application/vnd.leap+json") ? , Application :: VndLibertyRequestXml => write ! (f , "application/vnd.liberty-request+xml") ? , Application :: VndLlamagraphicsLifeBalanceDesktop => write ! (f , "application/vnd.llamagraphics.life-balance.desktop") ? , Application :: VndLlamagraphicsLifeBalanceExchangeXml => write ! (f , "application/vnd.llamagraphics.life-balance.exchange+xml") ? , Application :: VndLogipipeCircuitZip => write ! (f , "application/vnd.logipipe.circuit+zip") ? , Application :: VndLoom => write ! (f , "application/vnd.loom") ? , Application :: VndLotus123 => write ! (f , "application/vnd.lotus-1-2-3") ? , Application :: VndLotusApproach => write ! (f , "application/vnd.lotus-approach") ? , Application :: VndLotusFreelance => write ! (f , "application/vnd.lotus-freelance") ? , Application :: VndLotusNotes => write ! (f , "application/vnd.lotus-notes") ? , Application :: VndLotusOrganizer => write ! (f , "application/vnd.lotus-organizer") ? , Application :: VndLotusScreencam => write ! (f , "application/vnd.lotus-screencam") ? , Application :: VndLotusWordpro => write ! (f , "application/vnd.lotus-wordpro") ? , Application :: VndMacportsPortpkg => write ! (f , "application/vnd.macports.portpkg") ? , Application :: VndMapboxVectorTile => write ! (f , "application/vnd.mapbox-vector-tile") ? , Application :: VndMarlinDrmActiontokenXml => write ! (f , "application/vnd.marlin.drm.actiontoken+xml") ? , Application :: VndMarlinDrmConftokenXml => write ! (f , "application/vnd.marlin.drm.conftoken+xml") ? , Application :: VndMarlinDrmLicenseXml => write ! (f , "application/vnd.marlin.drm.license+xml") ? , Application :: VndMarlinDrmMdcf => write ! (f , "application/vnd.marlin.drm.mdcf") ? , Application :: VndMasonJson => write ! (f , "application/vnd.mason+json") ? , Application :: VndMaxarArchive3TzZip => write ! (f , "application/vnd.maxar.archive.3tz+zip") ? , Application :: VndMaxmindMaxmindDb => write ! (f , "application/vnd.maxmind.maxmind-db") ? , Application :: VndMcd => write ! (f , "application/vnd.mcd") ? , Application :: VndMedcalcdata => write ! (f , "application/vnd.medcalcdata") ? , Application :: VndMediastationCdkey => write ! (f , "application/vnd.mediastation.cdkey") ? , Application :: VndMeridianSlingshot => write ! (f , "application/vnd.meridian-slingshot") ? , Application :: VndMFER => write ! (f , "application/vnd.MFER") ? , Application :: VndMfmp => write ! (f , "application/vnd.mfmp") ? , Application :: VndMicroJson => write ! (f , "application/vnd.micro+json") ? , Application :: VndMicrografxFlo => write ! (f , "application/vnd.micrografx.flo") ? , Application :: VndMicrografxIgx => write ! (f , "application/vnd.micrografx.igx") ? , Application :: VndMicrosoftPortableExecutable => write ! (f , "application/vnd.microsoft.portable-executable") ? , Application :: VndMicrosoftWindowsThumbnailCache => write ! (f , "application/vnd.microsoft.windows.thumbnail-cache") ? , Application :: VndMieleJson => write ! (f , "application/vnd.miele+json") ? , Application :: VndMif => write ! (f , "application/vnd.mif") ? , Application :: VndMinisoftHp3000Save => write ! (f , "application/vnd.minisoft-hp3000-save") ? , Application :: VndMitsubishiMistyGuardTrustweb => write ! (f , "application/vnd.mitsubishi.misty-guard.trustweb") ? , Application :: VndMobiusDAF => write ! (f , "application/vnd.Mobius.DAF") ? , Application :: VndMobiusDIS => write ! (f , "application/vnd.Mobius.DIS") ? , Application :: VndMobiusMBK => write ! (f , "application/vnd.Mobius.MBK") ? , Application :: VndMobiusMQY => write ! (f , "application/vnd.Mobius.MQY") ? , Application :: VndMobiusMSL => write ! (f , "application/vnd.Mobius.MSL") ? , Application :: VndMobiusPLC => write ! (f , "application/vnd.Mobius.PLC") ? , Application :: VndMobiusTXF => write ! (f , "application/vnd.Mobius.TXF") ? , Application :: VndMophunApplication => write ! (f , "application/vnd.mophun.application") ? , Application :: VndMophunCertificate => write ! (f , "application/vnd.mophun.certificate") ? , Application :: VndMotorolaFlexsuite => write ! (f , "application/vnd.motorola.flexsuite") ? , Application :: VndMotorolaFlexsuiteAdsi => write ! (f , "application/vnd.motorola.flexsuite.adsi") ? , Application :: VndMotorolaFlexsuiteFis => write ! (f , "application/vnd.motorola.flexsuite.fis") ? , Application :: VndMotorolaFlexsuiteGotap => write ! (f , "application/vnd.motorola.flexsuite.gotap") ? , Application :: VndMotorolaFlexsuiteKmr => write ! (f , "application/vnd.motorola.flexsuite.kmr") ? , Application :: VndMotorolaFlexsuiteTtc => write ! (f , "application/vnd.motorola.flexsuite.ttc") ? , Application :: VndMotorolaFlexsuiteWem => write ! (f , "application/vnd.motorola.flexsuite.wem") ? , Application :: VndMotorolaIprm => write ! (f , "application/vnd.motorola.iprm") ? , Application :: VndMozillaXulXml => write ! (f , "application/vnd.mozilla.xul+xml") ? , Application :: VndMsArtgalry => write ! (f , "application/vnd.ms-artgalry") ? , Application :: VndMsAsf => write ! (f , "application/vnd.ms-asf") ? , Application :: VndMsCabCompressed => write ! (f , "application/vnd.ms-cab-compressed") ? , Application :: VndMs3Mfdocument => write ! (f , "application/vnd.ms-3mfdocument") ? , Application :: VndMsExcel => write ! (f , "application/vnd.ms-excel") ? , Application :: VndMsExcelAddinMacroEnabled12 => write ! (f , "application/vnd.ms-excel.addin.macroEnabled.12") ? , Application :: VndMsExcelSheetBinaryMacroEnabled12 => write ! (f , "application/vnd.ms-excel.sheet.binary.macroEnabled.12") ? , Application :: VndMsExcelSheetMacroEnabled12 => write ! (f , "application/vnd.ms-excel.sheet.macroEnabled.12") ? , Application :: VndMsExcelTemplateMacroEnabled12 => write ! (f , "application/vnd.ms-excel.template.macroEnabled.12") ? , Application :: VndMsFontobject => write ! (f , "application/vnd.ms-fontobject") ? , Application :: VndMsHtmlhelp => write ! (f , "application/vnd.ms-htmlhelp") ? , Application :: VndMsIms => write ! (f , "application/vnd.ms-ims") ? , Application :: VndMsLrm => write ! (f , "application/vnd.ms-lrm") ? , Application :: VndMsOfficeActiveXXml => write ! (f , "application/vnd.ms-office.activeX+xml") ? , Application :: VndMsOfficetheme => write ! (f , "application/vnd.ms-officetheme") ? , Application :: VndMsPlayreadyInitiatorXml => write ! (f , "application/vnd.ms-playready.initiator+xml") ? , Application :: VndMsPowerpoint => write ! (f , "application/vnd.ms-powerpoint") ? , Application :: VndMsPowerpointAddinMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.addin.macroEnabled.12") ? , Application :: VndMsPowerpointPresentationMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.presentation.macroEnabled.12") ? , Application :: VndMsPowerpointSlideMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.slide.macroEnabled.12") ? , Application :: VndMsPowerpointSlideshowMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.slideshow.macroEnabled.12") ? , Application :: VndMsPowerpointTemplateMacroEnabled12 => write ! (f , "application/vnd.ms-powerpoint.template.macroEnabled.12") ? , Application :: VndMsPrintDeviceCapabilitiesXml => write ! (f , "application/vnd.ms-PrintDeviceCapabilities+xml") ? , Application :: VndMsPrintSchemaTicketXml => write ! (f , "application/vnd.ms-PrintSchemaTicket+xml") ? , Application :: VndMsProject => write ! (f , "application/vnd.ms-project") ? , Application :: VndMsTnef => write ! (f , "application/vnd.ms-tnef") ? , Application :: VndMsWindowsDevicepairing => write ! (f , "application/vnd.ms-windows.devicepairing") ? , Application :: VndMsWindowsNwprintingOob => write ! (f , "application/vnd.ms-windows.nwprinting.oob") ? , Application :: VndMsWindowsPrinterpairing => write ! (f , "application/vnd.ms-windows.printerpairing") ? , Application :: VndMsWindowsWsdOob => write ! (f , "application/vnd.ms-windows.wsd.oob") ? , Application :: VndMsWmdrmLicChlgReq => write ! (f , "application/vnd.ms-wmdrm.lic-chlg-req") ? , Application :: VndMsWmdrmLicResp => write ! (f , "application/vnd.ms-wmdrm.lic-resp") ? , Application :: VndMsWmdrmMeterChlgReq => write ! (f , "application/vnd.ms-wmdrm.meter-chlg-req") ? , Application :: VndMsWmdrmMeterResp => write ! (f , "application/vnd.ms-wmdrm.meter-resp") ? , Application :: VndMsWordDocumentMacroEnabled12 => write ! (f , "application/vnd.ms-word.document.macroEnabled.12") ? , Application :: VndMsWordTemplateMacroEnabled12 => write ! (f , "application/vnd.ms-word.template.macroEnabled.12") ? , Application :: VndMsWorks => write ! (f , "application/vnd.ms-works") ? , Application :: VndMsWpl => write ! (f , "application/vnd.ms-wpl") ? , Application :: VndMsXpsdocument => write ! (f , "application/vnd.ms-xpsdocument") ? , Application :: VndMsaDiskImage => write ! (f , "application/vnd.msa-disk-image") ? , Application :: VndMseq => write ! (f , "application/vnd.mseq") ? , Application :: VndMsign => write ! (f , "application/vnd.msign") ? , Application :: VndMultiadCreator => write ! (f , "application/vnd.multiad.creator") ? , Application :: VndMultiadCreatorCif => write ! (f , "application/vnd.multiad.creator.cif") ? , Application :: VndMusician => write ! (f , "application/vnd.musician") ? , Application :: VndMusicNiff => write ! (f , "application/vnd.music-niff") ? , Application :: VndMuveeStyle => write ! (f , "application/vnd.muvee.style") ? , Application :: VndMynfc => write ! (f , "application/vnd.mynfc") ? , Application :: VndNacamarYbridJson => write ! (f , "application/vnd.nacamar.ybrid+json") ? , Application :: VndNcdControl => write ! (f , "application/vnd.ncd.control") ? , Application :: VndNcdReference => write ! (f , "application/vnd.ncd.reference") ? , Application :: VndNearstInvJson => write ! (f , "application/vnd.nearst.inv+json") ? , Application :: VndNebumindLine => write ! (f , "application/vnd.nebumind.line") ? , Application :: VndNervana => write ! (f , "application/vnd.nervana") ? , Application :: VndNetfpx => write ! (f , "application/vnd.netfpx") ? , Application :: VndNeurolanguageNlu => write ! (f , "application/vnd.neurolanguage.nlu") ? , Application :: VndNimn => write ! (f , "application/vnd.nimn") ? , Application :: VndNintendoSnesRom => write ! (f , "application/vnd.nintendo.snes.rom") ? , Application :: VndNintendoNitroRom => write ! (f , "application/vnd.nintendo.nitro.rom") ? , Application :: VndNitf => write ! (f , "application/vnd.nitf") ? , Application :: VndNoblenetDirectory => write ! (f , "application/vnd.noblenet-directory") ? , Application :: VndNoblenetSealer => write ! (f , "application/vnd.noblenet-sealer") ? , Application :: VndNoblenetWeb => write ! (f , "application/vnd.noblenet-web") ? , Application :: VndNokiaCatalogs => write ! (f , "application/vnd.nokia.catalogs") ? , Application :: VndNokiaConmlWbxml => write ! (f , "application/vnd.nokia.conml+wbxml") ? , Application :: VndNokiaConmlXml => write ! (f , "application/vnd.nokia.conml+xml") ? , Application :: VndNokiaIptvConfigXml => write ! (f , "application/vnd.nokia.iptv.config+xml") ? , Application :: VndNokiaISDSRadioPresets => write ! (f , "application/vnd.nokia.iSDS-radio-presets") ? , Application :: VndNokiaLandmarkWbxml => write ! (f , "application/vnd.nokia.landmark+wbxml") ? , Application :: VndNokiaLandmarkXml => write ! (f , "application/vnd.nokia.landmark+xml") ? , Application :: VndNokiaLandmarkcollectionXml => write ! (f , "application/vnd.nokia.landmarkcollection+xml") ? , Application :: VndNokiaNcd => write ! (f , "application/vnd.nokia.ncd") ? , Application :: VndNokiaNGageAcXml => write ! (f , "application/vnd.nokia.n-gage.ac+xml") ? , Application :: VndNokiaNGageData => write ! (f , "application/vnd.nokia.n-gage.data") ? , Application :: VndNokiaPcdWbxml => write ! (f , "application/vnd.nokia.pcd+wbxml") ? , Application :: VndNokiaPcdXml => write ! (f , "application/vnd.nokia.pcd+xml") ? , Application :: VndNokiaRadioPreset => write ! (f , "application/vnd.nokia.radio-preset") ? , Application :: VndNokiaRadioPresets => write ! (f , "application/vnd.nokia.radio-presets") ? , Application :: VndNovadigmEDM => write ! (f , "application/vnd.novadigm.EDM") ? , Application :: VndNovadigmEDX => write ! (f , "application/vnd.novadigm.EDX") ? , Application :: VndNovadigmEXT => write ! (f , "application/vnd.novadigm.EXT") ? , Application :: VndNttLocalContentShare => write ! (f , "application/vnd.ntt-local.content-share") ? , Application :: VndNttLocalFileTransfer => write ! (f , "application/vnd.ntt-local.file-transfer") ? , Application :: VndNttLocalOgwRemoteAccess => write ! (f , "application/vnd.ntt-local.ogw_remote-access") ? , Application :: VndNttLocalSipTaRemote => write ! (f , "application/vnd.ntt-local.sip-ta_remote") ? , Application :: VndNttLocalSipTaTcpStream => write ! (f , "application/vnd.ntt-local.sip-ta_tcp_stream") ? , Application :: VndOasisOpendocumentBase => write ! (f , "application/vnd.oasis.opendocument.base") ? , Application :: VndOasisOpendocumentChart => write ! (f , "application/vnd.oasis.opendocument.chart") ? , Application :: VndOasisOpendocumentChartTemplate => write ! (f , "application/vnd.oasis.opendocument.chart-template") ? , Application :: VndOasisOpendocumentFormula => write ! (f , "application/vnd.oasis.opendocument.formula") ? , Application :: VndOasisOpendocumentFormulaTemplate => write ! (f , "application/vnd.oasis.opendocument.formula-template") ? , Application :: VndOasisOpendocumentGraphics => write ! (f , "application/vnd.oasis.opendocument.graphics") ? , Application :: VndOasisOpendocumentGraphicsTemplate => write ! (f , "application/vnd.oasis.opendocument.graphics-template") ? , Application :: VndOasisOpendocumentImage => write ! (f , "application/vnd.oasis.opendocument.image") ? , Application :: VndOasisOpendocumentImageTemplate => write ! (f , "application/vnd.oasis.opendocument.image-template") ? , Application :: VndOasisOpendocumentPresentation => write ! (f , "application/vnd.oasis.opendocument.presentation") ? , Application :: VndOasisOpendocumentPresentationTemplate => write ! (f , "application/vnd.oasis.opendocument.presentation-template") ? , Application :: VndOasisOpendocumentSpreadsheet => write ! (f , "application/vnd.oasis.opendocument.spreadsheet") ? , Application :: VndOasisOpendocumentSpreadsheetTemplate => write ! (f , "application/vnd.oasis.opendocument.spreadsheet-template") ? , Application :: VndOasisOpendocumentText => write ! (f , "application/vnd.oasis.opendocument.text") ? , Application :: VndOasisOpendocumentTextMaster => write ! (f , "application/vnd.oasis.opendocument.text-master") ? , Application :: VndOasisOpendocumentTextTemplate => write ! (f , "application/vnd.oasis.opendocument.text-template") ? , Application :: VndOasisOpendocumentTextWeb => write ! (f , "application/vnd.oasis.opendocument.text-web") ? , Application :: VndObn => write ! (f , "application/vnd.obn") ? , Application :: VndOcfCbor => write ! (f , "application/vnd.ocf+cbor") ? , Application :: VndOciImageManifestV1Json => write ! (f , "application/vnd.oci.image.manifest.v1+json") ? , Application :: VndOftnL10NJson => write ! (f , "application/vnd.oftn.l10n+json") ? , Application :: VndOipfContentaccessdownloadXml => write ! (f , "application/vnd.oipf.contentaccessdownload+xml") ? , Application :: VndOipfContentaccessstreamingXml => write ! (f , "application/vnd.oipf.contentaccessstreaming+xml") ? , Application :: VndOipfCspgHexbinary => write ! (f , "application/vnd.oipf.cspg-hexbinary") ? , Application :: VndOipfDaeSvgXml => write ! (f , "application/vnd.oipf.dae.svg+xml") ? , Application :: VndOipfDaeXhtmlXml => write ! (f , "application/vnd.oipf.dae.xhtml+xml") ? , Application :: VndOipfMippvcontrolmessageXml => write ! (f , "application/vnd.oipf.mippvcontrolmessage+xml") ? , Application :: VndOipfPaeGem => write ! (f , "application/vnd.oipf.pae.gem") ? , Application :: VndOipfSpdiscoveryXml => write ! (f , "application/vnd.oipf.spdiscovery+xml") ? , Application :: VndOipfSpdlistXml => write ! (f , "application/vnd.oipf.spdlist+xml") ? , Application :: VndOipfUeprofileXml => write ! (f , "application/vnd.oipf.ueprofile+xml") ? , Application :: VndOipfUserprofileXml => write ! (f , "application/vnd.oipf.userprofile+xml") ? , Application :: VndOlpcSugar => write ! (f , "application/vnd.olpc-sugar") ? , Application :: VndOmaBcastAssociatedProcedureParameterXml => write ! (f , "application/vnd.oma.bcast.associated-procedure-parameter+xml") ? , Application :: VndOmaBcastDrmTriggerXml => write ! (f , "application/vnd.oma.bcast.drm-trigger+xml") ? , Application :: VndOmaBcastImdXml => write ! (f , "application/vnd.oma.bcast.imd+xml") ? , Application :: VndOmaBcastLtkm => write ! (f , "application/vnd.oma.bcast.ltkm") ? , Application :: VndOmaBcastNotificationXml => write ! (f , "application/vnd.oma.bcast.notification+xml") ? , Application :: VndOmaBcastProvisioningtrigger => write ! (f , "application/vnd.oma.bcast.provisioningtrigger") ? , Application :: VndOmaBcastSgboot => write ! (f , "application/vnd.oma.bcast.sgboot") ? , Application :: VndOmaBcastSgddXml => write ! (f , "application/vnd.oma.bcast.sgdd+xml") ? , Application :: VndOmaBcastSgdu => write ! (f , "application/vnd.oma.bcast.sgdu") ? , Application :: VndOmaBcastSimpleSymbolContainer => write ! (f , "application/vnd.oma.bcast.simple-symbol-container") ? , Application :: VndOmaBcastSmartcardTriggerXml => write ! (f , "application/vnd.oma.bcast.smartcard-trigger+xml") ? , Application :: VndOmaBcastSprovXml => write ! (f , "application/vnd.oma.bcast.sprov+xml") ? , Application :: VndOmaBcastStkm => write ! (f , "application/vnd.oma.bcast.stkm") ? , Application :: VndOmaCabAddressBookXml => write ! (f , "application/vnd.oma.cab-address-book+xml") ? , Application :: VndOmaCabFeatureHandlerXml => write ! (f , "application/vnd.oma.cab-feature-handler+xml") ? , Application :: VndOmaCabPccXml => write ! (f , "application/vnd.oma.cab-pcc+xml") ? , Application :: VndOmaCabSubsInviteXml => write ! (f , "application/vnd.oma.cab-subs-invite+xml") ? , Application :: VndOmaCabUserPrefsXml => write ! (f , "application/vnd.oma.cab-user-prefs+xml") ? , Application :: VndOmaDcd => write ! (f , "application/vnd.oma.dcd") ? , Application :: VndOmaDcdc => write ! (f , "application/vnd.oma.dcdc") ? , Application :: VndOmaDd2Xml => write ! (f , "application/vnd.oma.dd2+xml") ? , Application :: VndOmaDrmRisdXml => write ! (f , "application/vnd.oma.drm.risd+xml") ? , Application :: VndOmaGroupUsageListXml => write ! (f , "application/vnd.oma.group-usage-list+xml") ? , Application :: VndOmaLwm2MCbor => write ! (f , "application/vnd.oma.lwm2m+cbor") ? , Application :: VndOmaLwm2MJson => write ! (f , "application/vnd.oma.lwm2m+json") ? , Application :: VndOmaLwm2MTlv => write ! (f , "application/vnd.oma.lwm2m+tlv") ? , Application :: VndOmaPalXml => write ! (f , "application/vnd.oma.pal+xml") ? , Application :: VndOmaPocDetailedProgressReportXml => write ! (f , "application/vnd.oma.poc.detailed-progress-report+xml") ? , Application :: VndOmaPocFinalReportXml => write ! (f , "application/vnd.oma.poc.final-report+xml") ? , Application :: VndOmaPocGroupsXml => write ! (f , "application/vnd.oma.poc.groups+xml") ? , Application :: VndOmaPocInvocationDescriptorXml => write ! (f , "application/vnd.oma.poc.invocation-descriptor+xml") ? , Application :: VndOmaPocOptimizedProgressReportXml => write ! (f , "application/vnd.oma.poc.optimized-progress-report+xml") ? , Application :: VndOmaPush => write ! (f , "application/vnd.oma.push") ? , Application :: VndOmaScidmMessagesXml => write ! (f , "application/vnd.oma.scidm.messages+xml") ? , Application :: VndOmaXcapDirectoryXml => write ! (f , "application/vnd.oma.xcap-directory+xml") ? , Application :: VndOmadsEmailXml => write ! (f , "application/vnd.omads-email+xml") ? , Application :: VndOmadsFileXml => write ! (f , "application/vnd.omads-file+xml") ? , Application :: VndOmadsFolderXml => write ! (f , "application/vnd.omads-folder+xml") ? , Application :: VndOmalocSuplInit => write ! (f , "application/vnd.omaloc-supl-init") ? , Application :: VndOmaScwsConfig => write ! (f , "application/vnd.oma-scws-config") ? , Application :: VndOmaScwsHttpRequest => write ! (f , "application/vnd.oma-scws-http-request") ? , Application :: VndOmaScwsHttpResponse => write ! (f , "application/vnd.oma-scws-http-response") ? , Application :: VndOnepager => write ! (f , "application/vnd.onepager") ? , Application :: VndOnepagertamp => write ! (f , "application/vnd.onepagertamp") ? , Application :: VndOnepagertamx => write ! (f , "application/vnd.onepagertamx") ? , Application :: VndOnepagertat => write ! (f , "application/vnd.onepagertat") ? , Application :: VndOnepagertatp => write ! (f , "application/vnd.onepagertatp") ? , Application :: VndOnepagertatx => write ! (f , "application/vnd.onepagertatx") ? , Application :: VndOnvifMetadata => write ! (f , "application/vnd.onvif.metadata") ? , Application :: VndOpenbloxGameBinary => write ! (f , "application/vnd.openblox.game-binary") ? , Application :: VndOpenbloxGameXml => write ! (f , "application/vnd.openblox.game+xml") ? , Application :: VndOpeneyeOeb => write ! (f , "application/vnd.openeye.oeb") ? , Application :: VndOpenstreetmapDataXml => write ! (f , "application/vnd.openstreetmap.data+xml") ? , Application :: VndOpentimestampsOts => write ! (f , "application/vnd.opentimestamps.ots") ? , Application :: VndOpenxmlformatsOfficedocumentCustomPropertiesXml => write ! (f , "application/vnd.openxmlformats-officedocument.custom-properties+xml") ? , Application :: VndOpenxmlformatsOfficedocumentCustomXmlPropertiesXml => write ! (f , "application/vnd.openxmlformats-officedocument.customXmlProperties+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawing+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.chart+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartshapesXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramColorsXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramDataXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramLayoutXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml") ? , Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramStyleXml => write ! (f , "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml") ? , Application :: VndOpenxmlformatsOfficedocumentExtendedPropertiesXml => write ! (f , "application/vnd.openxmlformats-officedocument.extended-properties+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentAuthorsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.comments+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlHandoutMasterXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesMasterXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesSlideXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentation => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.presentation") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentationMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresPropsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlide => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slide") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slide+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideLayoutXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideMasterXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshow => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideshow") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshowMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideUpdateInfoXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTableStylesXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTagsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.tags+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplate => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.template") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplateMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentPresentationmlViewPropsXml => write ! (f , "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCalcChainXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlChartsheetXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCommentsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlConnectionsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlDialogsheetXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlExternalLinkXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheDefinitionXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheRecordsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlQueryTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionHeadersXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionLogXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSharedStringsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheet => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMetadataXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlStylesXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableSingleCellsXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplate => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.template") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplateMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlUserNamesXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlVolatileDependenciesXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml") ? , Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlWorksheetXml => write ! (f , "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml") ? , Application :: VndOpenxmlformatsOfficedocumentThemeXml => write ! (f , "application/vnd.openxmlformats-officedocument.theme+xml") ? , Application :: VndOpenxmlformatsOfficedocumentThemeOverrideXml => write ! (f , "application/vnd.openxmlformats-officedocument.themeOverride+xml") ? , Application :: VndOpenxmlformatsOfficedocumentVmlDrawing => write ! (f , "application/vnd.openxmlformats-officedocument.vmlDrawing") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlCommentsXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocument => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.document") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentGlossaryXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlEndnotesXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFontTableXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFooterXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFootnotesXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlNumberingXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlSettingsXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlStylesXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplate => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.template") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplateMainXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml") ? , Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlWebSettingsXml => write ! (f , "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml") ? , Application :: VndOpenxmlformatsPackageCorePropertiesXml => write ! (f , "application/vnd.openxmlformats-package.core-properties+xml") ? , Application :: VndOpenxmlformatsPackageDigitalSignatureXmlsignatureXml => write ! (f , "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml") ? , Application :: VndOpenxmlformatsPackageRelationshipsXml => write ! (f , "application/vnd.openxmlformats-package.relationships+xml") ? , Application :: VndOracleResourceJson => write ! (f , "application/vnd.oracle.resource+json") ? , Application :: VndOrangeIndata => write ! (f , "application/vnd.orange.indata") ? , Application :: VndOsaNetdeploy => write ! (f , "application/vnd.osa.netdeploy") ? , Application :: VndOsgeoMapguidePackage => write ! (f , "application/vnd.osgeo.mapguide.package") ? , Application :: VndOsgiBundle => write ! (f , "application/vnd.osgi.bundle") ? , Application :: VndOsgiDp => write ! (f , "application/vnd.osgi.dp") ? , Application :: VndOsgiSubsystem => write ! (f , "application/vnd.osgi.subsystem") ? , Application :: VndOtpsCtKipXml => write ! (f , "application/vnd.otps.ct-kip+xml") ? , Application :: VndOxliCountgraph => write ! (f , "application/vnd.oxli.countgraph") ? , Application :: VndPagerdutyJson => write ! (f , "application/vnd.pagerduty+json") ? , Application :: VndPalm => write ! (f , "application/vnd.palm") ? , Application :: VndPanoply => write ! (f , "application/vnd.panoply") ? , Application :: VndPaosXml => write ! (f , "application/vnd.paos.xml") ? , Application :: VndPatentdive => write ! (f , "application/vnd.patentdive") ? , Application :: VndPatientecommsdoc => write ! (f , "application/vnd.patientecommsdoc") ? , Application :: VndPawaafile => write ! (f , "application/vnd.pawaafile") ? , Application :: VndPcos => write ! (f , "application/vnd.pcos") ? , Application :: VndPgFormat => write ! (f , "application/vnd.pg.format") ? , Application :: VndPgOsasli => write ! (f , "application/vnd.pg.osasli") ? , Application :: VndPiaccessApplicationLicence => write ! (f , "application/vnd.piaccess.application-licence") ? , Application :: VndPicsel => write ! (f , "application/vnd.picsel") ? , Application :: VndPmiWidget => write ! (f , "application/vnd.pmi.widget") ? , Application :: VndPocGroupAdvertisementXml => write ! (f , "application/vnd.poc.group-advertisement+xml") ? , Application :: VndPocketlearn => write ! (f , "application/vnd.pocketlearn") ? , Application :: VndPowerbuilder6 => write ! (f , "application/vnd.powerbuilder6") ? , Application :: VndPowerbuilder6S => write ! (f , "application/vnd.powerbuilder6-s") ? , Application :: VndPowerbuilder7 => write ! (f , "application/vnd.powerbuilder7") ? , Application :: VndPowerbuilder75 => write ! (f , "application/vnd.powerbuilder75") ? , Application :: VndPowerbuilder75S => write ! (f , "application/vnd.powerbuilder75-s") ? , Application :: VndPowerbuilder7S => write ! (f , "application/vnd.powerbuilder7-s") ? , Application :: VndPreminet => write ! (f , "application/vnd.preminet") ? , Application :: VndPreviewsystemsBox => write ! (f , "application/vnd.previewsystems.box") ? , Application :: VndProteusMagazine => write ! (f , "application/vnd.proteus.magazine") ? , Application :: VndPsfs => write ! (f , "application/vnd.psfs") ? , Application :: VndPublishareDeltaTree => write ! (f , "application/vnd.publishare-delta-tree") ? , Application :: VndPviPtid1 => write ! (f , "application/vnd.pvi.ptid1") ? , Application :: VndPwgMultiplexed => write ! (f , "application/vnd.pwg-multiplexed") ? , Application :: VndPwgXhtmlPrintXml => write ! (f , "application/vnd.pwg-xhtml-print+xml") ? , Application :: VndQualcommBrewAppRes => write ! (f , "application/vnd.qualcomm.brew-app-res") ? , Application :: VndQuarantainenet => write ! (f , "application/vnd.quarantainenet") ? , Application :: VndQuarkQuarkXPress => write ! (f , "application/vnd.Quark.QuarkXPress") ? , Application :: VndQuobjectQuoxdocument => write ! (f , "application/vnd.quobject-quoxdocument") ? , Application :: VndRadisysMomlXml => write ! (f , "application/vnd.radisys.moml+xml") ? , Application :: VndRadisysMsmlAuditConfXml => write ! (f , "application/vnd.radisys.msml-audit-conf+xml") ? , Application :: VndRadisysMsmlAuditConnXml => write ! (f , "application/vnd.radisys.msml-audit-conn+xml") ? , Application :: VndRadisysMsmlAuditDialogXml => write ! (f , "application/vnd.radisys.msml-audit-dialog+xml") ? , Application :: VndRadisysMsmlAuditStreamXml => write ! (f , "application/vnd.radisys.msml-audit-stream+xml") ? , Application :: VndRadisysMsmlAuditXml => write ! (f , "application/vnd.radisys.msml-audit+xml") ? , Application :: VndRadisysMsmlConfXml => write ! (f , "application/vnd.radisys.msml-conf+xml") ? , Application :: VndRadisysMsmlDialogBaseXml => write ! (f , "application/vnd.radisys.msml-dialog-base+xml") ? , Application :: VndRadisysMsmlDialogFaxDetectXml => write ! (f , "application/vnd.radisys.msml-dialog-fax-detect+xml") ? , Application :: VndRadisysMsmlDialogFaxSendrecvXml => write ! (f , "application/vnd.radisys.msml-dialog-fax-sendrecv+xml") ? , Application :: VndRadisysMsmlDialogGroupXml => write ! (f , "application/vnd.radisys.msml-dialog-group+xml") ? , Application :: VndRadisysMsmlDialogSpeechXml => write ! (f , "application/vnd.radisys.msml-dialog-speech+xml") ? , Application :: VndRadisysMsmlDialogTransformXml => write ! (f , "application/vnd.radisys.msml-dialog-transform+xml") ? , Application :: VndRadisysMsmlDialogXml => write ! (f , "application/vnd.radisys.msml-dialog+xml") ? , Application :: VndRadisysMsmlXml => write ! (f , "application/vnd.radisys.msml+xml") ? , Application :: VndRainstorData => write ! (f , "application/vnd.rainstor.data") ? , Application :: VndRapid => write ! (f , "application/vnd.rapid") ? , Application :: VndRar => write ! (f , "application/vnd.rar") ? , Application :: VndRealvncBed => write ! (f , "application/vnd.realvnc.bed") ? , Application :: VndRecordareMusicxml => write ! (f , "application/vnd.recordare.musicxml") ? , Application :: VndRecordareMusicxmlXml => write ! (f , "application/vnd.recordare.musicxml+xml") ? , Application :: VndRenLearnRlprint => write ! (f , "application/vnd.RenLearn.rlprint") ? , Application :: VndResilientLogic => write ! (f , "application/vnd.resilient.logic") ? , Application :: VndRestfulJson => write ! (f , "application/vnd.restful+json") ? , Application :: VndRigCryptonote => write ! (f , "application/vnd.rig.cryptonote") ? , Application :: VndRoute66Link66Xml => write ! (f , "application/vnd.route66.link66+xml") ? , Application :: VndRs274X => write ! (f , "application/vnd.rs-274x") ? , Application :: VndRuckusDownload => write ! (f , "application/vnd.ruckus.download") ? , Application :: VndS3Sms => write ! (f , "application/vnd.s3sms") ? , Application :: VndSailingtrackerTrack => write ! (f , "application/vnd.sailingtracker.track") ? , Application :: VndSar => write ! (f , "application/vnd.sar") ? , Application :: VndSbmCid => write ! (f , "application/vnd.sbm.cid") ? , Application :: VndSbmMid2 => write ! (f , "application/vnd.sbm.mid2") ? , Application :: VndScribus => write ! (f , "application/vnd.scribus") ? , Application :: VndSealed3Df => write ! (f , "application/vnd.sealed.3df") ? , Application :: VndSealedCsf => write ! (f , "application/vnd.sealed.csf") ? , Application :: VndSealedDoc => write ! (f , "application/vnd.sealed.doc") ? , Application :: VndSealedEml => write ! (f , "application/vnd.sealed.eml") ? , Application :: VndSealedMht => write ! (f , "application/vnd.sealed.mht") ? , Application :: VndSealedNet => write ! (f , "application/vnd.sealed.net") ? , Application :: VndSealedPpt => write ! (f , "application/vnd.sealed.ppt") ? , Application :: VndSealedTiff => write ! (f , "application/vnd.sealed.tiff") ? , Application :: VndSealedXls => write ! (f , "application/vnd.sealed.xls") ? , Application :: VndSealedmediaSoftsealHtml => write ! (f , "application/vnd.sealedmedia.softseal.html") ? , Application :: VndSealedmediaSoftsealPdf => write ! (f , "application/vnd.sealedmedia.softseal.pdf") ? , Application :: VndSeemail => write ! (f , "application/vnd.seemail") ? , Application :: VndSeisJson => write ! (f , "application/vnd.seis+json") ? , Application :: VndSema => write ! (f , "application/vnd.sema") ? , Application :: VndSemd => write ! (f , "application/vnd.semd") ? , Application :: VndSemf => write ! (f , "application/vnd.semf") ? , Application :: VndShadeSaveFile => write ! (f , "application/vnd.shade-save-file") ? , Application :: VndShanaInformedFormdata => write ! (f , "application/vnd.shana.informed.formdata") ? , Application :: VndShanaInformedFormtemplate => write ! (f , "application/vnd.shana.informed.formtemplate") ? , Application :: VndShanaInformedInterchange => write ! (f , "application/vnd.shana.informed.interchange") ? , Application :: VndShanaInformedPackage => write ! (f , "application/vnd.shana.informed.package") ? , Application :: VndShootproofJson => write ! (f , "application/vnd.shootproof+json") ? , Application :: VndShopkickJson => write ! (f , "application/vnd.shopkick+json") ? , Application :: VndShp => write ! (f , "application/vnd.shp") ? , Application :: VndShx => write ! (f , "application/vnd.shx") ? , Application :: VndSigrokSession => write ! (f , "application/vnd.sigrok.session") ? , Application :: VndSimTechMindMapper => write ! (f , "application/vnd.SimTech-MindMapper") ? , Application :: VndSirenJson => write ! (f , "application/vnd.siren+json") ? , Application :: VndSmaf => write ! (f , "application/vnd.smaf") ? , Application :: VndSmartNotebook => write ! (f , "application/vnd.smart.notebook") ? , Application :: VndSmartTeacher => write ! (f , "application/vnd.smart.teacher") ? , Application :: VndSnesdevPageTable => write ! (f , "application/vnd.snesdev-page-table") ? , Application :: VndSoftware602FillerFormXml => write ! (f , "application/vnd.software602.filler.form+xml") ? , Application :: VndSoftware602FillerFormXmlZip => write ! (f , "application/vnd.software602.filler.form-xml-zip") ? , Application :: VndSolentSdkmXml => write ! (f , "application/vnd.solent.sdkm+xml") ? , Application :: VndSpotfireDxp => write ! (f , "application/vnd.spotfire.dxp") ? , Application :: VndSpotfireSfs => write ! (f , "application/vnd.spotfire.sfs") ? , Application :: VndSqlite3 => write ! (f , "application/vnd.sqlite3") ? , Application :: VndSssCod => write ! (f , "application/vnd.sss-cod") ? , Application :: VndSssDtf => write ! (f , "application/vnd.sss-dtf") ? , Application :: VndSssNtf => write ! (f , "application/vnd.sss-ntf") ? , Application :: VndStepmaniaPackage => write ! (f , "application/vnd.stepmania.package") ? , Application :: VndStepmaniaStepchart => write ! (f , "application/vnd.stepmania.stepchart") ? , Application :: VndStreetStream => write ! (f , "application/vnd.street-stream") ? , Application :: VndSunWadlXml => write ! (f , "application/vnd.sun.wadl+xml") ? , Application :: VndSusCalendar => write ! (f , "application/vnd.sus-calendar") ? , Application :: VndSvd => write ! (f , "application/vnd.svd") ? , Application :: VndSwiftviewIcs => write ! (f , "application/vnd.swiftview-ics") ? , Application :: VndSybylMol2 => write ! (f , "application/vnd.sybyl.mol2") ? , Application :: VndSycleXml => write ! (f , "application/vnd.sycle+xml") ? , Application :: VndSyftJson => write ! (f , "application/vnd.syft+json") ? , Application :: VndSyncmlDmNotification => write ! (f , "application/vnd.syncml.dm.notification") ? , Application :: VndSyncmlDmddfXml => write ! (f , "application/vnd.syncml.dmddf+xml") ? , Application :: VndSyncmlDmtndsWbxml => write ! (f , "application/vnd.syncml.dmtnds+wbxml") ? , Application :: VndSyncmlDmtndsXml => write ! (f , "application/vnd.syncml.dmtnds+xml") ? , Application :: VndSyncmlDmddfWbxml => write ! (f , "application/vnd.syncml.dmddf+wbxml") ? , Application :: VndSyncmlDmWbxml => write ! (f , "application/vnd.syncml.dm+wbxml") ? , Application :: VndSyncmlDmXml => write ! (f , "application/vnd.syncml.dm+xml") ? , Application :: VndSyncmlDsNotification => write ! (f , "application/vnd.syncml.ds.notification") ? , Application :: VndSyncmlXml => write ! (f , "application/vnd.syncml+xml") ? , Application :: VndTableschemaJson => write ! (f , "application/vnd.tableschema+json") ? , Application :: VndTaoIntentModuleArchive => write ! (f , "application/vnd.tao.intent-module-archive") ? , Application :: VndTcpdumpPcap => write ! (f , "application/vnd.tcpdump.pcap") ? , Application :: VndThinkCellPpttcJson => write ! (f , "application/vnd.think-cell.ppttc+json") ? , Application :: VndTml => write ! (f , "application/vnd.tml") ? , Application :: VndTmdMediaflexApiXml => write ! (f , "application/vnd.tmd.mediaflex.api+xml") ? , Application :: VndTmobileLivetv => write ! (f , "application/vnd.tmobile-livetv") ? , Application :: VndTriOnesource => write ! (f , "application/vnd.tri.onesource") ? , Application :: VndTridTpt => write ! (f , "application/vnd.trid.tpt") ? , Application :: VndTriscapeMxs => write ! (f , "application/vnd.triscape.mxs") ? , Application :: VndTrueapp => write ! (f , "application/vnd.trueapp") ? , Application :: VndTruedoc => write ! (f , "application/vnd.truedoc") ? , Application :: VndUbisoftWebplayer => write ! (f , "application/vnd.ubisoft.webplayer") ? , Application :: VndUfdl => write ! (f , "application/vnd.ufdl") ? , Application :: VndUiqTheme => write ! (f , "application/vnd.uiq.theme") ? , Application :: VndUmajin => write ! (f , "application/vnd.umajin") ? , Application :: VndUnity => write ! (f , "application/vnd.unity") ? , Application :: VndUomlXml => write ! (f , "application/vnd.uoml+xml") ? , Application :: VndUplanetAlert => write ! (f , "application/vnd.uplanet.alert") ? , Application :: VndUplanetAlertWbxml => write ! (f , "application/vnd.uplanet.alert-wbxml") ? , Application :: VndUplanetBearerChoice => write ! (f , "application/vnd.uplanet.bearer-choice") ? , Application :: VndUplanetBearerChoiceWbxml => write ! (f , "application/vnd.uplanet.bearer-choice-wbxml") ? , Application :: VndUplanetCacheop => write ! (f , "application/vnd.uplanet.cacheop") ? , Application :: VndUplanetCacheopWbxml => write ! (f , "application/vnd.uplanet.cacheop-wbxml") ? , Application :: VndUplanetChannel => write ! (f , "application/vnd.uplanet.channel") ? , Application :: VndUplanetChannelWbxml => write ! (f , "application/vnd.uplanet.channel-wbxml") ? , Application :: VndUplanetList => write ! (f , "application/vnd.uplanet.list") ? , Application :: VndUplanetListcmd => write ! (f , "application/vnd.uplanet.listcmd") ? , Application :: VndUplanetListcmdWbxml => write ! (f , "application/vnd.uplanet.listcmd-wbxml") ? , Application :: VndUplanetListWbxml => write ! (f , "application/vnd.uplanet.list-wbxml") ? , Application :: VndUriMap => write ! (f , "application/vnd.uri-map") ? , Application :: VndUplanetSignal => write ! (f , "application/vnd.uplanet.signal") ? , Application :: VndValveSourceMaterial => write ! (f , "application/vnd.valve.source.material") ? , Application :: VndVcx => write ! (f , "application/vnd.vcx") ? , Application :: VndVdStudy => write ! (f , "application/vnd.vd-study") ? , Application :: VndVectorworks => write ! (f , "application/vnd.vectorworks") ? , Application :: VndVelJson => write ! (f , "application/vnd.vel+json") ? , Application :: VndVerimatrixVcas => write ! (f , "application/vnd.verimatrix.vcas") ? , Application :: VndVeritoneAionJson => write ! (f , "application/vnd.veritone.aion+json") ? , Application :: VndVeryantThin => write ! (f , "application/vnd.veryant.thin") ? , Application :: VndVesEncrypted => write ! (f , "application/vnd.ves.encrypted") ? , Application :: VndVidsoftVidconference => write ! (f , "application/vnd.vidsoft.vidconference") ? , Application :: VndVisio => write ! (f , "application/vnd.visio") ? , Application :: VndVisionary => write ! (f , "application/vnd.visionary") ? , Application :: VndVividenceScriptfile => write ! (f , "application/vnd.vividence.scriptfile") ? , Application :: VndVsf => write ! (f , "application/vnd.vsf") ? , Application :: VndWapSic => write ! (f , "application/vnd.wap.sic") ? , Application :: VndWapSlc => write ! (f , "application/vnd.wap.slc") ? , Application :: VndWapWbxml => write ! (f , "application/vnd.wap.wbxml") ? , Application :: VndWapWmlc => write ! (f , "application/vnd.wap.wmlc") ? , Application :: VndWapWmlscriptc => write ! (f , "application/vnd.wap.wmlscriptc") ? , Application :: VndWasmflowWafl => write ! (f , "application/vnd.wasmflow.wafl") ? , Application :: VndWebturbo => write ! (f , "application/vnd.webturbo") ? , Application :: VndWfaDpp => write ! (f , "application/vnd.wfa.dpp") ? , Application :: VndWfaP2P => write ! (f , "application/vnd.wfa.p2p") ? , Application :: VndWfaWsc => write ! (f , "application/vnd.wfa.wsc") ? , Application :: VndWindowsDevicepairing => write ! (f , "application/vnd.windows.devicepairing") ? , Application :: VndWmc => write ! (f , "application/vnd.wmc") ? , Application :: VndWmfBootstrap => write ! (f , "application/vnd.wmf.bootstrap") ? , Application :: VndWolframMathematica => write ! (f , "application/vnd.wolfram.mathematica") ? , Application :: VndWolframMathematicaPackage => write ! (f , "application/vnd.wolfram.mathematica.package") ? , Application :: VndWolframPlayer => write ! (f , "application/vnd.wolfram.player") ? , Application :: VndWordperfect => write ! (f , "application/vnd.wordperfect") ? , Application :: VndWqd => write ! (f , "application/vnd.wqd") ? , Application :: VndWrqHp3000Labelled => write ! (f , "application/vnd.wrq-hp3000-labelled") ? , Application :: VndWtStf => write ! (f , "application/vnd.wt.stf") ? , Application :: VndWvCspXml => write ! (f , "application/vnd.wv.csp+xml") ? , Application :: VndWvCspWbxml => write ! (f , "application/vnd.wv.csp+wbxml") ? , Application :: VndWvSspXml => write ! (f , "application/vnd.wv.ssp+xml") ? , Application :: VndXacmlJson => write ! (f , "application/vnd.xacml+json") ? , Application :: VndXara => write ! (f , "application/vnd.xara") ? , Application :: VndXfdl => write ! (f , "application/vnd.xfdl") ? , Application :: VndXfdlWebform => write ! (f , "application/vnd.xfdl.webform") ? , Application :: VndXmiXml => write ! (f , "application/vnd.xmi+xml") ? , Application :: VndXmpieCpkg => write ! (f , "application/vnd.xmpie.cpkg") ? , Application :: VndXmpieDpkg => write ! (f , "application/vnd.xmpie.dpkg") ? , Application :: VndXmpiePlan => write ! (f , "application/vnd.xmpie.plan") ? , Application :: VndXmpiePpkg => write ! (f , "application/vnd.xmpie.ppkg") ? , Application :: VndXmpieXlim => write ! (f , "application/vnd.xmpie.xlim") ? , Application :: VndYamahaHvDic => write ! (f , "application/vnd.yamaha.hv-dic") ? , Application :: VndYamahaHvScript => write ! (f , "application/vnd.yamaha.hv-script") ? , Application :: VndYamahaHvVoice => write ! (f , "application/vnd.yamaha.hv-voice") ? , Application :: VndYamahaOpenscoreformatOsfpvgXml => write ! (f , "application/vnd.yamaha.openscoreformat.osfpvg+xml") ? , Application :: VndYamahaOpenscoreformat => write ! (f , "application/vnd.yamaha.openscoreformat") ? , Application :: VndYamahaRemoteSetup => write ! (f , "application/vnd.yamaha.remote-setup") ? , Application :: VndYamahaSmafAudio => write ! (f , "application/vnd.yamaha.smaf-audio") ? , Application :: VndYamahaSmafPhrase => write ! (f , "application/vnd.yamaha.smaf-phrase") ? , Application :: VndYamahaThroughNgn => write ! (f , "application/vnd.yamaha.through-ngn") ? , Application :: VndYamahaTunnelUdpencap => write ! (f , "application/vnd.yamaha.tunnel-udpencap") ? , Application :: VndYaoweme => write ! (f , "application/vnd.yaoweme") ? , Application :: VndYellowriverCustomMenu => write ! (f , "application/vnd.yellowriver-custom-menu") ? , Application :: VndZul => write ! (f , "application/vnd.zul") ? , Application :: VndZzazzDeckXml => write ! (f , "application/vnd.zzazz.deck+xml") ? , Application :: VoicexmlXml => write ! (f , "application/voicexml+xml") ? , Application :: VoucherCmsJson => write ! (f , "application/voucher-cms+json") ? , Application :: VqRtcpxr => write ! (f , "application/vq-rtcpxr") ? , Application :: Wasm => write ! (f , "application/wasm") ? , Application :: WatcherinfoXml => write ! (f , "application/watcherinfo+xml") ? , Application :: WebpushOptionsJson => write ! (f , "application/webpush-options+json") ? , Application :: WhoisppQuery => write ! (f , "application/whoispp-query") ? , Application :: WhoisppResponse => write ! (f , "application/whoispp-response") ? , Application :: Widget => write ! (f , "application/widget") ? , Application :: Wita => write ! (f , "application/wita") ? , Application :: Wordperfect51 => write ! (f , "application/wordperfect5.1") ? , Application :: WsdlXml => write ! (f , "application/wsdl+xml") ? , Application :: WspolicyXml => write ! (f , "application/wspolicy+xml") ? , Application :: XPkiMessage => write ! (f , "application/x-pki-message") ? , Application :: XWwwFormUrlencoded => write ! (f , "application/x-www-form-urlencoded") ? , Application :: XX509CaCert => write ! (f , "application/x-x509-ca-cert") ? , Application :: XX509CaRaCert => write ! (f , "application/x-x509-ca-ra-cert") ? , Application :: XX509NextCaCert => write ! (f , "application/x-x509-next-ca-cert") ? , Application :: X400Bp => write ! (f , "application/x400-bp") ? , Application :: XacmlXml => write ! (f , "application/xacml+xml") ? , Application :: XcapAttXml => write ! (f , "application/xcap-att+xml") ? , Application :: XcapCapsXml => write ! (f , "application/xcap-caps+xml") ? , Application :: XcapDiffXml => write ! (f , "application/xcap-diff+xml") ? , Application :: XcapElXml => write ! (f , "application/xcap-el+xml") ? , Application :: XcapErrorXml => write ! (f , "application/xcap-error+xml") ? , Application :: XcapNsXml => write ! (f , "application/xcap-ns+xml") ? , Application :: XconConferenceInfoDiffXml => write ! (f , "application/xcon-conference-info-diff+xml") ? , Application :: XconConferenceInfoXml => write ! (f , "application/xcon-conference-info+xml") ? , Application :: XencXml => write ! (f , "application/xenc+xml") ? , Application :: Xfdf => write ! (f , "application/xfdf") ? , Application :: XhtmlXml => write ! (f , "application/xhtml+xml") ? , Application :: XliffXml => write ! (f , "application/xliff+xml") ? , Application :: Xml => write ! (f , "application/xml") ? , Application :: XmlDtd => write ! (f , "application/xml-dtd") ? , Application :: XmlExternalParsedEntity => write ! (f , "application/xml-external-parsed-entity") ? , Application :: XmlPatchXml => write ! (f , "application/xml-patch+xml") ? , Application :: XmppXml => write ! (f , "application/xmpp+xml") ? , Application :: XopXml => write ! (f , "application/xop+xml") ? , Application :: XsltXml => write ! (f , "application/xslt+xml") ? , Application :: XvXml => write ! (f , "application/xv+xml") ? , Application :: Yang => write ! (f , "application/yang") ? , Application :: YangDataCbor => write ! (f , "application/yang-data+cbor") ? , Application :: YangDataJson => write ! (f , "application/yang-data+json") ? , Application :: YangDataXml => write ! (f , "application/yang-data+xml") ? , Application :: YangPatchJson => write ! (f , "application/yang-patch+json") ? , Application :: YangPatchXml => write ! (f , "application/yang-patch+xml") ? , Application :: YinXml => write ! (f , "application/yin+xml") ? , Application :: Zip => write ! (f , "application/zip") ? , Application :: Zlib => write ! (f , "application/zlib") ? , Application :: Zstd => write ! (f , "application/zstd") ? , Application :: Other (template) => write ! (f , "{}" , template) ? , }
        Ok(())
    }
}
impl From<&str> for Application {
    fn from(input: &str) -> Self {
        match input { "application/1d-interleaved-parityfec" => Application :: _1DInterleavedParityfec , "application/3gpdash-qoe-report+xml" => Application :: _3GpdashQoeReportXml , "application/3gppHal+json" => Application :: _3GppHalJson , "application/3gppHalForms+json" => Application :: _3GppHalFormsJson , "application/3gpp-ims+xml" => Application :: _3GppImsXml , "application/A2L" => Application :: A2L , "application/ace+cbor" => Application :: AceCbor , "application/ace+json" => Application :: AceJson , "application/activemessage" => Application :: Activemessage , "application/activity+json" => Application :: ActivityJson , "application/aif+cbor" => Application :: AifCbor , "application/aif+json" => Application :: AifJson , "application/alto-cdni+json" => Application :: AltoCdniJson , "application/alto-cdnifilter+json" => Application :: AltoCdnifilterJson , "application/alto-costmap+json" => Application :: AltoCostmapJson , "application/alto-costmapfilter+json" => Application :: AltoCostmapfilterJson , "application/alto-directory+json" => Application :: AltoDirectoryJson , "application/alto-endpointprop+json" => Application :: AltoEndpointpropJson , "application/alto-endpointpropparams+json" => Application :: AltoEndpointpropparamsJson , "application/alto-endpointcost+json" => Application :: AltoEndpointcostJson , "application/alto-endpointcostparams+json" => Application :: AltoEndpointcostparamsJson , "application/alto-error+json" => Application :: AltoErrorJson , "application/alto-networkmapfilter+json" => Application :: AltoNetworkmapfilterJson , "application/alto-networkmap+json" => Application :: AltoNetworkmapJson , "application/alto-propmap+json" => Application :: AltoPropmapJson , "application/alto-propmapparams+json" => Application :: AltoPropmapparamsJson , "application/alto-updatestreamcontrol+json" => Application :: AltoUpdatestreamcontrolJson , "application/alto-updatestreamparams+json" => Application :: AltoUpdatestreamparamsJson , "application/AML" => Application :: Aml , "application/andrew-inset" => Application :: AndrewInset , "application/applefile" => Application :: Applefile , "application/at+jwt" => Application :: AtJwt , "application/ATF" => Application :: Atf , "application/ATFX" => Application :: Atfx , "application/atom+xml" => Application :: AtomXml , "application/atomcat+xml" => Application :: AtomcatXml , "application/atomdeleted+xml" => Application :: AtomdeletedXml , "application/atomicmail" => Application :: Atomicmail , "application/atomsvc+xml" => Application :: AtomsvcXml , "application/atsc-dwd+xml" => Application :: AtscDwdXml , "application/atsc-dynamic-event-message" => Application :: AtscDynamicEventMessage , "application/atsc-held+xml" => Application :: AtscHeldXml , "application/atsc-rdt+json" => Application :: AtscRdtJson , "application/atsc-rsat+xml" => Application :: AtscRsatXml , "application/ATXML" => Application :: Atxml , "application/auth-policy+xml" => Application :: AuthPolicyXml , "application/automationml-aml+xml" => Application :: AutomationmlAmlXml , "application/automationml-amlx+zip" => Application :: AutomationmlAmlxZip , "application/bacnet-xdd+zip" => Application :: BacnetXddZip , "application/batch-SMTP" => Application :: BatchSMTP , "application/beep+xml" => Application :: BeepXml , "application/calendar+json" => Application :: CalendarJson , "application/calendar+xml" => Application :: CalendarXml , "application/call-completion" => Application :: CallCompletion , "application/CALS-1840" => Application :: Cals1840 , "application/captive+json" => Application :: CaptiveJson , "application/cbor" => Application :: Cbor , "application/cbor-seq" => Application :: CborSeq , "application/cccex" => Application :: Cccex , "application/ccmp+xml" => Application :: CcmpXml , "application/ccxml+xml" => Application :: CcxmlXml , "application/cda+xml" => Application :: CdaXml , "application/CDFX+XML" => Application :: CdfxXml , "application/cdmi-capability" => Application :: CdmiCapability , "application/cdmi-container" => Application :: CdmiContainer , "application/cdmi-domain" => Application :: CdmiDomain , "application/cdmi-object" => Application :: CdmiObject , "application/cdmi-queue" => Application :: CdmiQueue , "application/cdni" => Application :: Cdni , "application/CEA" => Application :: Cea , "application/cea-2018+xml" => Application :: Cea2018Xml , "application/cellml+xml" => Application :: CellmlXml , "application/cfw" => Application :: Cfw , "application/city+json" => Application :: CityJson , "application/clr" => Application :: Clr , "application/clue_info+xml" => Application :: ClueInfoXml , "application/clue+xml" => Application :: ClueXml , "application/cms" => Application :: Cms , "application/cnrp+xml" => Application :: CnrpXml , "application/coap-group+json" => Application :: CoapGroupJson , "application/coap-payload" => Application :: CoapPayload , "application/commonground" => Application :: Commonground , "application/concise-problem-details+cbor" => Application :: ConciseProblemDetailsCbor , "application/conference-info+xml" => Application :: ConferenceInfoXml , "application/cpl+xml" => Application :: CplXml , "application/cose" => Application :: Cose , "application/cose-key" => Application :: CoseKey , "application/cose-key-set" => Application :: CoseKeySet , "application/cose-x509" => Application :: CoseX509 , "application/csrattrs" => Application :: Csrattrs , "application/csta+xml" => Application :: CstaXml , "application/CSTAdata+xml" => Application :: CstadataXml , "application/csvm+json" => Application :: CsvmJson , "application/cwl" => Application :: Cwl , "application/cwl+json" => Application :: CwlJson , "application/cwt" => Application :: Cwt , "application/cybercash" => Application :: Cybercash , "application/dash+xml" => Application :: DashXml , "application/dash-patch+xml" => Application :: DashPatchXml , "application/dashdelta" => Application :: Dashdelta , "application/davmount+xml" => Application :: DavmountXml , "application/dca-rft" => Application :: DcaRft , "application/DCD" => Application :: Dcd , "application/dec-dx" => Application :: DecDx , "application/dialog-info+xml" => Application :: DialogInfoXml , "application/dicom" => Application :: Dicom , "application/dicom+json" => Application :: DicomJson , "application/dicom+xml" => Application :: DicomXml , "application/DII" => Application :: Dii , "application/DIT" => Application :: Dit , "application/dns" => Application :: Dns , "application/dns+json" => Application :: DnsJson , "application/dns-message" => Application :: DnsMessage , "application/dots+cbor" => Application :: DotsCbor , "application/dskpp+xml" => Application :: DskppXml , "application/dssc+der" => Application :: DsscDer , "application/dssc+xml" => Application :: DsscXml , "application/dvcs" => Application :: Dvcs , "application/EDI-consent" => Application :: EdiConsent , "application/EDIFACT" => Application :: Edifact , "application/EDI-X12" => Application :: EdiX12 , "application/efi" => Application :: Efi , "application/elm+json" => Application :: ElmJson , "application/elm+xml" => Application :: ElmXml , "application/EmergencyCallData.cap+xml" => Application :: EmergencyCallDataCapXml , "application/EmergencyCallData.Comment+xml" => Application :: EmergencyCallDataCommentXml , "application/EmergencyCallData.Control+xml" => Application :: EmergencyCallDataControlXml , "application/EmergencyCallData.DeviceInfo+xml" => Application :: EmergencyCallDataDeviceInfoXml , "application/EmergencyCallData.eCall.MSD" => Application :: EmergencyCallDataECallMSD , "application/EmergencyCallData.LegacyESN+json" => Application :: EmergencyCallDataLegacyESNJson , "application/EmergencyCallData.ProviderInfo+xml" => Application :: EmergencyCallDataProviderInfoXml , "application/EmergencyCallData.ServiceInfo+xml" => Application :: EmergencyCallDataServiceInfoXml , "application/EmergencyCallData.SubscriberInfo+xml" => Application :: EmergencyCallDataSubscriberInfoXml , "application/EmergencyCallData.VEDS+xml" => Application :: EmergencyCallDataVEDSXml , "application/emma+xml" => Application :: EmmaXml , "application/emotionml+xml" => Application :: EmotionmlXml , "application/encaprtp" => Application :: Encaprtp , "application/epp+xml" => Application :: EppXml , "application/epub+zip" => Application :: EpubZip , "application/eshop" => Application :: Eshop , "application/example" => Application :: Example , "application/exi" => Application :: Exi , "application/expect-ct-report+json" => Application :: ExpectCtReportJson , "application/express" => Application :: Express , "application/fastinfoset" => Application :: Fastinfoset , "application/fastsoap" => Application :: Fastsoap , "application/fdf" => Application :: Fdf , "application/fdt+xml" => Application :: FdtXml , "application/fhir+json" => Application :: FhirJson , "application/fhir+xml" => Application :: FhirXml , "application/fits" => Application :: Fits , "application/flexfec" => Application :: Flexfec , "application/font-tdpfr" => Application :: FontTdpfr , "application/framework-attributes+xml" => Application :: FrameworkAttributesXml , "application/geo+json" => Application :: GeoJson , "application/geo+json-seq" => Application :: GeoJsonSeq , "application/geopackage+sqlite3" => Application :: GeopackageSqlite3 , "application/geoxacml+xml" => Application :: GeoxacmlXml , "application/gltf-buffer" => Application :: GltfBuffer , "application/gml+xml" => Application :: GmlXml , "application/gzip" => Application :: Gzip , "application/H224" => Application :: H224 , "application/held+xml" => Application :: HeldXml , "application/hl7v2+xml" => Application :: Hl7V2Xml , "application/http" => Application :: Http , "application/hyperstudio" => Application :: Hyperstudio , "application/ibe-key-request+xml" => Application :: IbeKeyRequestXml , "application/ibe-pkg-reply+xml" => Application :: IbePkgReplyXml , "application/ibe-pp-data" => Application :: IbePpData , "application/iges" => Application :: Iges , "application/im-iscomposing+xml" => Application :: ImIscomposingXml , "application/index" => Application :: Index , "application/index.cmd" => Application :: IndexCmd , "application/index.obj" => Application :: IndexObj , "application/index.response" => Application :: IndexResponse , "application/index.vnd" => Application :: IndexVnd , "application/inkml+xml" => Application :: InkmlXml , "application/IOTP" => Application :: Iotp , "application/ipfix" => Application :: Ipfix , "application/ipp" => Application :: Ipp , "application/ISUP" => Application :: Isup , "application/its+xml" => Application :: ItsXml , "application/jf2feed+json" => Application :: Jf2FeedJson , "application/jose" => Application :: Jose , "application/jose+json" => Application :: JoseJson , "application/jrd+json" => Application :: JrdJson , "application/jscalendar+json" => Application :: JscalendarJson , "application/json" => Application :: Json , "application/json-patch+json" => Application :: JsonPatchJson , "application/json-seq" => Application :: JsonSeq , "application/jwk+json" => Application :: JwkJson , "application/jwk-set+json" => Application :: JwkSetJson , "application/jwt" => Application :: Jwt , "application/kpml-request+xml" => Application :: KpmlRequestXml , "application/kpml-response+xml" => Application :: KpmlResponseXml , "application/ld+json" => Application :: LdJson , "application/lgr+xml" => Application :: LgrXml , "application/link-format" => Application :: LinkFormat , "application/linkset" => Application :: Linkset , "application/linkset+json" => Application :: LinksetJson , "application/load-control+xml" => Application :: LoadControlXml , "application/logout+jwt" => Application :: LogoutJwt , "application/lost+xml" => Application :: LostXml , "application/lostsync+xml" => Application :: LostsyncXml , "application/lpf+zip" => Application :: LpfZip , "application/LXF" => Application :: Lxf , "application/mac-binhex40" => Application :: MacBinhex40 , "application/macwriteii" => Application :: Macwriteii , "application/mads+xml" => Application :: MadsXml , "application/manifest+json" => Application :: ManifestJson , "application/marc" => Application :: Marc , "application/marcxml+xml" => Application :: MarcxmlXml , "application/mathematica" => Application :: Mathematica , "application/mathml+xml" => Application :: MathmlXml , "application/mathml-content+xml" => Application :: MathmlContentXml , "application/mathml-presentation+xml" => Application :: MathmlPresentationXml , "application/mbms-associated-procedure-description+xml" => Application :: MbmsAssociatedProcedureDescriptionXml , "application/mbms-deregister+xml" => Application :: MbmsDeregisterXml , "application/mbms-envelope+xml" => Application :: MbmsEnvelopeXml , "application/mbms-msk-response+xml" => Application :: MbmsMskResponseXml , "application/mbms-msk+xml" => Application :: MbmsMskXml , "application/mbms-protection-description+xml" => Application :: MbmsProtectionDescriptionXml , "application/mbms-reception-report+xml" => Application :: MbmsReceptionReportXml , "application/mbms-register-response+xml" => Application :: MbmsRegisterResponseXml , "application/mbms-register+xml" => Application :: MbmsRegisterXml , "application/mbms-schedule+xml" => Application :: MbmsScheduleXml , "application/mbms-user-service-description+xml" => Application :: MbmsUserServiceDescriptionXml , "application/mbox" => Application :: Mbox , "application/media_control+xml" => Application :: MediaControlXml , "application/media-policy-dataset+xml" => Application :: MediaPolicyDatasetXml , "application/mediaservercontrol+xml" => Application :: MediaservercontrolXml , "application/merge-patch+json" => Application :: MergePatchJson , "application/metalink4+xml" => Application :: Metalink4Xml , "application/mets+xml" => Application :: MetsXml , "application/MF4" => Application :: Mf4 , "application/mikey" => Application :: Mikey , "application/mipc" => Application :: Mipc , "application/missing-blocks+cbor-seq" => Application :: MissingBlocksCborSeq , "application/mmt-aei+xml" => Application :: MmtAeiXml , "application/mmt-usd+xml" => Application :: MmtUsdXml , "application/mods+xml" => Application :: ModsXml , "application/moss-keys" => Application :: MossKeys , "application/moss-signature" => Application :: MossSignature , "application/mosskey-data" => Application :: MosskeyData , "application/mosskey-request" => Application :: MosskeyRequest , "application/mp21" => Application :: Mp21 , "application/mp4" => Application :: Mp4 , "application/mpeg4-generic" => Application :: Mpeg4Generic , "application/mpeg4-iod" => Application :: Mpeg4Iod , "application/mpeg4-iod-xmt" => Application :: Mpeg4IodXmt , "application/mrb-consumer+xml" => Application :: MrbConsumerXml , "application/mrb-publish+xml" => Application :: MrbPublishXml , "application/msc-ivr+xml" => Application :: MscIvrXml , "application/msc-mixer+xml" => Application :: MscMixerXml , "application/msword" => Application :: Msword , "application/mud+json" => Application :: MudJson , "application/multipart-core" => Application :: MultipartCore , "application/mxf" => Application :: Mxf , "application/n-quads" => Application :: NQuads , "application/n-triples" => Application :: NTriples , "application/nasdata" => Application :: Nasdata , "application/news-checkgroups" => Application :: NewsCheckgroups , "application/news-groupinfo" => Application :: NewsGroupinfo , "application/news-transmission" => Application :: NewsTransmission , "application/nlsml+xml" => Application :: NlsmlXml , "application/node" => Application :: Node , "application/nss" => Application :: Nss , "application/oauth-authz-req+jwt" => Application :: OauthAuthzReqJwt , "application/oblivious-dns-message" => Application :: ObliviousDnsMessage , "application/ocsp-request" => Application :: OcspRequest , "application/ocsp-response" => Application :: OcspResponse , "application/octet-stream" => Application :: OctetStream , "application/ODA" => Application :: Oda , "application/odm+xml" => Application :: OdmXml , "application/ODX" => Application :: Odx , "application/oebps-package+xml" => Application :: OebpsPackageXml , "application/ogg" => Application :: Ogg , "application/opc-nodeset+xml" => Application :: OpcNodesetXml , "application/oscore" => Application :: Oscore , "application/oxps" => Application :: Oxps , "application/p21" => Application :: P21 , "application/p21+zip" => Application :: P21Zip , "application/p2p-overlay+xml" => Application :: P2POverlayXml , "application/parityfec" => Application :: Parityfec , "application/passport" => Application :: Passport , "application/patch-ops-error+xml" => Application :: PatchOpsErrorXml , "application/pdf" => Application :: Pdf , "application/PDX" => Application :: Pdx , "application/pem-certificate-chain" => Application :: PemCertificateChain , "application/pgp-encrypted" => Application :: PgpEncrypted , "application/pgp-keys" => Application :: PgpKeys , "application/pgp-signature" => Application :: PgpSignature , "application/pidf-diff+xml" => Application :: PidfDiffXml , "application/pidf+xml" => Application :: PidfXml , "application/pkcs10" => Application :: Pkcs10 , "application/pkcs7-mime" => Application :: Pkcs7Mime , "application/pkcs7-signature" => Application :: Pkcs7Signature , "application/pkcs8" => Application :: Pkcs8 , "application/pkcs8-encrypted" => Application :: Pkcs8Encrypted , "application/pkcs12" => Application :: Pkcs12 , "application/pkix-attr-cert" => Application :: PkixAttrCert , "application/pkix-cert" => Application :: PkixCert , "application/pkix-crl" => Application :: PkixCrl , "application/pkix-pkipath" => Application :: PkixPkipath , "application/pkixcmp" => Application :: Pkixcmp , "application/pls+xml" => Application :: PlsXml , "application/poc-settings+xml" => Application :: PocSettingsXml , "application/postscript" => Application :: Postscript , "application/ppsp-tracker+json" => Application :: PpspTrackerJson , "application/problem+json" => Application :: ProblemJson , "application/problem+xml" => Application :: ProblemXml , "application/provenance+xml" => Application :: ProvenanceXml , "application/prs.alvestrand.titrax-sheet" => Application :: PrsAlvestrandTitraxSheet , "application/prs.cww" => Application :: PrsCww , "application/prs.cyn" => Application :: PrsCyn , "application/prs.hpub+zip" => Application :: PrsHpubZip , "application/prs.nprend" => Application :: PrsNprend , "application/prs.plucker" => Application :: PrsPlucker , "application/prs.rdf-xml-crypt" => Application :: PrsRdfXmlCrypt , "application/prs.xsf+xml" => Application :: PrsXsfXml , "application/pskc+xml" => Application :: PskcXml , "application/pvd+json" => Application :: PvdJson , "application/rdf+xml" => Application :: RdfXml , "application/route-apd+xml" => Application :: RouteApdXml , "application/route-s-tsid+xml" => Application :: RouteSTsidXml , "application/route-usd+xml" => Application :: RouteUsdXml , "application/QSIG" => Application :: Qsig , "application/raptorfec" => Application :: Raptorfec , "application/rdap+json" => Application :: RdapJson , "application/reginfo+xml" => Application :: ReginfoXml , "application/relax-ng-compact-syntax" => Application :: RelaxNgCompactSyntax , "application/remote-printing" => Application :: RemotePrinting , "application/reputon+json" => Application :: ReputonJson , "application/resource-lists-diff+xml" => Application :: ResourceListsDiffXml , "application/resource-lists+xml" => Application :: ResourceListsXml , "application/rfc+xml" => Application :: RfcXml , "application/riscos" => Application :: Riscos , "application/rlmi+xml" => Application :: RlmiXml , "application/rls-services+xml" => Application :: RlsServicesXml , "application/rpki-checklist" => Application :: RpkiChecklist , "application/rpki-ghostbusters" => Application :: RpkiGhostbusters , "application/rpki-manifest" => Application :: RpkiManifest , "application/rpki-publication" => Application :: RpkiPublication , "application/rpki-roa" => Application :: RpkiRoa , "application/rpki-updown" => Application :: RpkiUpdown , "application/rtf" => Application :: Rtf , "application/rtploopback" => Application :: Rtploopback , "application/rtx" => Application :: Rtx , "application/samlassertion+xml" => Application :: SamlassertionXml , "application/samlmetadata+xml" => Application :: SamlmetadataXml , "application/sarif-external-properties+json" => Application :: SarifExternalPropertiesJson , "application/sarif+json" => Application :: SarifJson , "application/sbe" => Application :: Sbe , "application/sbml+xml" => Application :: SbmlXml , "application/scaip+xml" => Application :: ScaipXml , "application/scim+json" => Application :: ScimJson , "application/scvp-cv-request" => Application :: ScvpCvRequest , "application/scvp-cv-response" => Application :: ScvpCvResponse , "application/scvp-vp-request" => Application :: ScvpVpRequest , "application/scvp-vp-response" => Application :: ScvpVpResponse , "application/sdp" => Application :: Sdp , "application/secevent+jwt" => Application :: SeceventJwt , "application/senml-etch+cbor" => Application :: SenmlEtchCbor , "application/senml-etch+json" => Application :: SenmlEtchJson , "application/senml-exi" => Application :: SenmlExi , "application/senml+cbor" => Application :: SenmlCbor , "application/senml+json" => Application :: SenmlJson , "application/senml+xml" => Application :: SenmlXml , "application/sensml-exi" => Application :: SensmlExi , "application/sensml+cbor" => Application :: SensmlCbor , "application/sensml+json" => Application :: SensmlJson , "application/sensml+xml" => Application :: SensmlXml , "application/sep-exi" => Application :: SepExi , "application/sep+xml" => Application :: SepXml , "application/session-info" => Application :: SessionInfo , "application/set-payment" => Application :: SetPayment , "application/set-payment-initiation" => Application :: SetPaymentInitiation , "application/set-registration" => Application :: SetRegistration , "application/set-registration-initiation" => Application :: SetRegistrationInitiation , "application/SGML" => Application :: Sgml , "application/sgml-open-catalog" => Application :: SgmlOpenCatalog , "application/shf+xml" => Application :: ShfXml , "application/sieve" => Application :: Sieve , "application/simple-filter+xml" => Application :: SimpleFilterXml , "application/simple-message-summary" => Application :: SimpleMessageSummary , "application/simpleSymbolContainer" => Application :: SimpleSymbolContainer , "application/sipc" => Application :: Sipc , "application/slate" => Application :: Slate , "application/smil+xml" => Application :: SmilXml , "application/smpte336m" => Application :: Smpte336M , "application/soap+fastinfoset" => Application :: SoapFastinfoset , "application/soap+xml" => Application :: SoapXml , "application/sparql-query" => Application :: SparqlQuery , "application/spdx+json" => Application :: SpdxJson , "application/sparql-results+xml" => Application :: SparqlResultsXml , "application/spirits-event+xml" => Application :: SpiritsEventXml , "application/sql" => Application :: Sql , "application/srgs" => Application :: Srgs , "application/srgs+xml" => Application :: SrgsXml , "application/sru+xml" => Application :: SruXml , "application/ssml+xml" => Application :: SsmlXml , "application/stix+json" => Application :: StixJson , "application/swid+cbor" => Application :: SwidCbor , "application/swid+xml" => Application :: SwidXml , "application/tamp-apex-update" => Application :: TampApexUpdate , "application/tamp-apex-update-confirm" => Application :: TampApexUpdateConfirm , "application/tamp-community-update" => Application :: TampCommunityUpdate , "application/tamp-community-update-confirm" => Application :: TampCommunityUpdateConfirm , "application/tamp-error" => Application :: TampError , "application/tamp-sequence-adjust" => Application :: TampSequenceAdjust , "application/tamp-sequence-adjust-confirm" => Application :: TampSequenceAdjustConfirm , "application/tamp-status-query" => Application :: TampStatusQuery , "application/tamp-status-response" => Application :: TampStatusResponse , "application/tamp-update" => Application :: TampUpdate , "application/tamp-update-confirm" => Application :: TampUpdateConfirm , "application/taxii+json" => Application :: TaxiiJson , "application/td+json" => Application :: TdJson , "application/tei+xml" => Application :: TeiXml , "application/TETRA_ISI" => Application :: TetraIsi , "application/thraud+xml" => Application :: ThraudXml , "application/timestamp-query" => Application :: TimestampQuery , "application/timestamp-reply" => Application :: TimestampReply , "application/timestamped-data" => Application :: TimestampedData , "application/tlsrpt+gzip" => Application :: TlsrptGzip , "application/tlsrpt+json" => Application :: TlsrptJson , "application/tm+json" => Application :: TmJson , "application/tnauthlist" => Application :: Tnauthlist , "application/token-introspection+jwt" => Application :: TokenIntrospectionJwt , "application/trickle-ice-sdpfrag" => Application :: TrickleIceSdpfrag , "application/trig" => Application :: Trig , "application/ttml+xml" => Application :: TtmlXml , "application/tve-trigger" => Application :: TveTrigger , "application/tzif" => Application :: Tzif , "application/tzif-leap" => Application :: TzifLeap , "application/ulpfec" => Application :: Ulpfec , "application/urc-grpsheet+xml" => Application :: UrcGrpsheetXml , "application/urc-ressheet+xml" => Application :: UrcRessheetXml , "application/urc-targetdesc+xml" => Application :: UrcTargetdescXml , "application/urc-uisocketdesc+xml" => Application :: UrcUisocketdescXml , "application/vcard+json" => Application :: VcardJson , "application/vcard+xml" => Application :: VcardXml , "application/vemmi" => Application :: Vemmi , "application/vnd.1000minds.decision-model+xml" => Application :: Vnd1000MindsDecisionModelXml , "application/vnd.3gpp.5gnas" => Application :: Vnd3Gpp5Gnas , "application/vnd.3gpp.access-transfer-events+xml" => Application :: Vnd3GppAccessTransferEventsXml , "application/vnd.3gpp.bsf+xml" => Application :: Vnd3GppBsfXml , "application/vnd.3gpp.GMOP+xml" => Application :: Vnd3GppGMOPXml , "application/vnd.3gpp.gtpc" => Application :: Vnd3GppGtpc , "application/vnd.3gpp.interworking-data" => Application :: Vnd3GppInterworkingData , "application/vnd.3gpp.lpp" => Application :: Vnd3GppLpp , "application/vnd.3gpp.mc-signalling-ear" => Application :: Vnd3GppMcSignallingEar , "application/vnd.3gpp.mcdata-affiliation-command+xml" => Application :: Vnd3GppMcdataAffiliationCommandXml , "application/vnd.3gpp.mcdata-info+xml" => Application :: Vnd3GppMcdataInfoXml , "application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml" => Application :: Vnd3GppMcdataMsgstoreCtrlRequestXml , "application/vnd.3gpp.mcdata-payload" => Application :: Vnd3GppMcdataPayload , "application/vnd.3gpp.mcdata-regroup+xml" => Application :: Vnd3GppMcdataRegroupXml , "application/vnd.3gpp.mcdata-service-config+xml" => Application :: Vnd3GppMcdataServiceConfigXml , "application/vnd.3gpp.mcdata-signalling" => Application :: Vnd3GppMcdataSignalling , "application/vnd.3gpp.mcdata-ue-config+xml" => Application :: Vnd3GppMcdataUeConfigXml , "application/vnd.3gpp.mcdata-user-profile+xml" => Application :: Vnd3GppMcdataUserProfileXml , "application/vnd.3gpp.mcptt-affiliation-command+xml" => Application :: Vnd3GppMcpttAffiliationCommandXml , "application/vnd.3gpp.mcptt-floor-request+xml" => Application :: Vnd3GppMcpttFloorRequestXml , "application/vnd.3gpp.mcptt-info+xml" => Application :: Vnd3GppMcpttInfoXml , "application/vnd.3gpp.mcptt-location-info+xml" => Application :: Vnd3GppMcpttLocationInfoXml , "application/vnd.3gpp.mcptt-mbms-usage-info+xml" => Application :: Vnd3GppMcpttMbmsUsageInfoXml , "application/vnd.3gpp.mcptt-service-config+xml" => Application :: Vnd3GppMcpttServiceConfigXml , "application/vnd.3gpp.mcptt-signed+xml" => Application :: Vnd3GppMcpttSignedXml , "application/vnd.3gpp.mcptt-ue-config+xml" => Application :: Vnd3GppMcpttUeConfigXml , "application/vnd.3gpp.mcptt-ue-init-config+xml" => Application :: Vnd3GppMcpttUeInitConfigXml , "application/vnd.3gpp.mcptt-user-profile+xml" => Application :: Vnd3GppMcpttUserProfileXml , "application/vnd.3gpp.mcvideo-affiliation-command+xml" => Application :: Vnd3GppMcvideoAffiliationCommandXml , "application/vnd.3gpp.mcvideo-info+xml" => Application :: Vnd3GppMcvideoInfoXml , "application/vnd.3gpp.mcvideo-location-info+xml" => Application :: Vnd3GppMcvideoLocationInfoXml , "application/vnd.3gpp.mcvideo-mbms-usage-info+xml" => Application :: Vnd3GppMcvideoMbmsUsageInfoXml , "application/vnd.3gpp.mcvideo-service-config+xml" => Application :: Vnd3GppMcvideoServiceConfigXml , "application/vnd.3gpp.mcvideo-transmission-request+xml" => Application :: Vnd3GppMcvideoTransmissionRequestXml , "application/vnd.3gpp.mcvideo-ue-config+xml" => Application :: Vnd3GppMcvideoUeConfigXml , "application/vnd.3gpp.mcvideo-user-profile+xml" => Application :: Vnd3GppMcvideoUserProfileXml , "application/vnd.3gpp.mid-call+xml" => Application :: Vnd3GppMidCallXml , "application/vnd.3gpp.ngap" => Application :: Vnd3GppNgap , "application/vnd.3gpp.pfcp" => Application :: Vnd3GppPfcp , "application/vnd.3gpp.pic-bw-large" => Application :: Vnd3GppPicBwLarge , "application/vnd.3gpp.pic-bw-small" => Application :: Vnd3GppPicBwSmall , "application/vnd.3gpp.pic-bw-var" => Application :: Vnd3GppPicBwVar , "application/vnd.3gpp-prose-pc3a+xml" => Application :: Vnd3GppProsePc3AXml , "application/vnd.3gpp-prose-pc3ach+xml" => Application :: Vnd3GppProsePc3AchXml , "application/vnd.3gpp-prose-pc3ch+xml" => Application :: Vnd3GppProsePc3ChXml , "application/vnd.3gpp-prose-pc8+xml" => Application :: Vnd3GppProsePc8Xml , "application/vnd.3gpp-prose+xml" => Application :: Vnd3GppProseXml , "application/vnd.3gpp.s1ap" => Application :: Vnd3GppS1Ap , "application/vnd.3gpp.sms" => Application :: Vnd3GppSms , "application/vnd.3gpp.sms+xml" => Application :: Vnd3GppSmsXml , "application/vnd.3gpp.srvcc-ext+xml" => Application :: Vnd3GppSrvccExtXml , "application/vnd.3gpp.SRVCC-info+xml" => Application :: Vnd3GppSRVCCInfoXml , "application/vnd.3gpp.state-and-event-info+xml" => Application :: Vnd3GppStateAndEventInfoXml , "application/vnd.3gpp.ussd+xml" => Application :: Vnd3GppUssdXml , "application/vnd.3gpp-v2x-local-service-information" => Application :: Vnd3GppV2XLocalServiceInformation , "application/vnd.3gpp2.bcmcsinfo+xml" => Application :: Vnd3Gpp2BcmcsinfoXml , "application/vnd.3gpp2.sms" => Application :: Vnd3Gpp2Sms , "application/vnd.3gpp2.tcap" => Application :: Vnd3Gpp2Tcap , "application/vnd.3lightssoftware.imagescal" => Application :: Vnd3LightssoftwareImagescal , "application/vnd.3M.Post-it-Notes" => Application :: Vnd3MPostItNotes , "application/vnd.accpac.simply.aso" => Application :: VndAccpacSimplyAso , "application/vnd.accpac.simply.imp" => Application :: VndAccpacSimplyImp , "application/vnd.acucobol" => Application :: VndAcucobol , "application/vnd.acucorp" => Application :: VndAcucorp , "application/vnd.adobe.flash.movie" => Application :: VndAdobeFlashMovie , "application/vnd.adobe.formscentral.fcdt" => Application :: VndAdobeFormscentralFcdt , "application/vnd.adobe.fxp" => Application :: VndAdobeFxp , "application/vnd.adobe.partial-upload" => Application :: VndAdobePartialUpload , "application/vnd.adobe.xdp+xml" => Application :: VndAdobeXdpXml , "application/vnd.aether.imp" => Application :: VndAetherImp , "application/vnd.afpc.afplinedata" => Application :: VndAfpcAfplinedata , "application/vnd.afpc.afplinedata-pagedef" => Application :: VndAfpcAfplinedataPagedef , "application/vnd.afpc.cmoca-cmresource" => Application :: VndAfpcCmocaCmresource , "application/vnd.afpc.foca-charset" => Application :: VndAfpcFocaCharset , "application/vnd.afpc.foca-codedfont" => Application :: VndAfpcFocaCodedfont , "application/vnd.afpc.foca-codepage" => Application :: VndAfpcFocaCodepage , "application/vnd.afpc.modca" => Application :: VndAfpcModca , "application/vnd.afpc.modca-cmtable" => Application :: VndAfpcModcaCmtable , "application/vnd.afpc.modca-formdef" => Application :: VndAfpcModcaFormdef , "application/vnd.afpc.modca-mediummap" => Application :: VndAfpcModcaMediummap , "application/vnd.afpc.modca-objectcontainer" => Application :: VndAfpcModcaObjectcontainer , "application/vnd.afpc.modca-overlay" => Application :: VndAfpcModcaOverlay , "application/vnd.afpc.modca-pagesegment" => Application :: VndAfpcModcaPagesegment , "application/vnd.age" => Application :: VndAge , "application/vnd.ah-barcode" => Application :: VndAhBarcode , "application/vnd.ahead.space" => Application :: VndAheadSpace , "application/vnd.airzip.filesecure.azf" => Application :: VndAirzipFilesecureAzf , "application/vnd.airzip.filesecure.azs" => Application :: VndAirzipFilesecureAzs , "application/vnd.amadeus+json" => Application :: VndAmadeusJson , "application/vnd.amazon.mobi8-ebook" => Application :: VndAmazonMobi8Ebook , "application/vnd.americandynamics.acc" => Application :: VndAmericandynamicsAcc , "application/vnd.amiga.ami" => Application :: VndAmigaAmi , "application/vnd.amundsen.maze+xml" => Application :: VndAmundsenMazeXml , "application/vnd.android.ota" => Application :: VndAndroidOta , "application/vnd.anki" => Application :: VndAnki , "application/vnd.anser-web-certificate-issue-initiation" => Application :: VndAnserWebCertificateIssueInitiation , "application/vnd.antix.game-component" => Application :: VndAntixGameComponent , "application/vnd.apache.arrow.file" => Application :: VndApacheArrowFile , "application/vnd.apache.arrow.stream" => Application :: VndApacheArrowStream , "application/vnd.apache.thrift.binary" => Application :: VndApacheThriftBinary , "application/vnd.apache.thrift.compact" => Application :: VndApacheThriftCompact , "application/vnd.apache.thrift.json" => Application :: VndApacheThriftJson , "application/vnd.apexlang" => Application :: VndApexlang , "application/vnd.api+json" => Application :: VndApiJson , "application/vnd.aplextor.warrp+json" => Application :: VndAplextorWarrpJson , "application/vnd.apothekende.reservation+json" => Application :: VndApothekendeReservationJson , "application/vnd.apple.installer+xml" => Application :: VndAppleInstallerXml , "application/vnd.apple.keynote" => Application :: VndAppleKeynote , "application/vnd.apple.mpegurl" => Application :: VndAppleMpegurl , "application/vnd.apple.numbers" => Application :: VndAppleNumbers , "application/vnd.apple.pages" => Application :: VndApplePages , "application/vnd.aristanetworks.swi" => Application :: VndAristanetworksSwi , "application/vnd.artisan+json" => Application :: VndArtisanJson , "application/vnd.artsquare" => Application :: VndArtsquare , "application/vnd.astraea-software.iota" => Application :: VndAstraeaSoftwareIota , "application/vnd.audiograph" => Application :: VndAudiograph , "application/vnd.autopackage" => Application :: VndAutopackage , "application/vnd.avalon+json" => Application :: VndAvalonJson , "application/vnd.avistar+xml" => Application :: VndAvistarXml , "application/vnd.balsamiq.bmml+xml" => Application :: VndBalsamiqBmmlXml , "application/vnd.banana-accounting" => Application :: VndBananaAccounting , "application/vnd.bbf.usp.error" => Application :: VndBbfUspError , "application/vnd.bbf.usp.msg" => Application :: VndBbfUspMsg , "application/vnd.bbf.usp.msg+json" => Application :: VndBbfUspMsgJson , "application/vnd.balsamiq.bmpr" => Application :: VndBalsamiqBmpr , "application/vnd.bekitzur-stech+json" => Application :: VndBekitzurStechJson , "application/vnd.belightsoft.lhzd+zip" => Application :: VndBelightsoftLhzdZip , "application/vnd.belightsoft.lhzl+zip" => Application :: VndBelightsoftLhzlZip , "application/vnd.bint.med-content" => Application :: VndBintMedContent , "application/vnd.biopax.rdf+xml" => Application :: VndBiopaxRdfXml , "application/vnd.blink-idb-value-wrapper" => Application :: VndBlinkIdbValueWrapper , "application/vnd.blueice.multipass" => Application :: VndBlueiceMultipass , "application/vnd.bluetooth.ep.oob" => Application :: VndBluetoothEpOob , "application/vnd.bluetooth.le.oob" => Application :: VndBluetoothLeOob , "application/vnd.bmi" => Application :: VndBmi , "application/vnd.bpf" => Application :: VndBpf , "application/vnd.bpf3" => Application :: VndBpf3 , "application/vnd.businessobjects" => Application :: VndBusinessobjects , "application/vnd.byu.uapi+json" => Application :: VndByuUapiJson , "application/vnd.cab-jscript" => Application :: VndCabJscript , "application/vnd.canon-cpdl" => Application :: VndCanonCpdl , "application/vnd.canon-lips" => Application :: VndCanonLips , "application/vnd.capasystems-pg+json" => Application :: VndCapasystemsPgJson , "application/vnd.cendio.thinlinc.clientconf" => Application :: VndCendioThinlincClientconf , "application/vnd.century-systems.tcp_stream" => Application :: VndCenturySystemsTcpStream , "application/vnd.chemdraw+xml" => Application :: VndChemdrawXml , "application/vnd.chess-pgn" => Application :: VndChessPgn , "application/vnd.chipnuts.karaoke-mmd" => Application :: VndChipnutsKaraokeMmd , "application/vnd.ciedi" => Application :: VndCiedi , "application/vnd.cinderella" => Application :: VndCinderella , "application/vnd.cirpack.isdn-ext" => Application :: VndCirpackIsdnExt , "application/vnd.citationstyles.style+xml" => Application :: VndCitationstylesStyleXml , "application/vnd.claymore" => Application :: VndClaymore , "application/vnd.cloanto.rp9" => Application :: VndCloantoRp9 , "application/vnd.clonk.c4group" => Application :: VndClonkC4Group , "application/vnd.cluetrust.cartomobile-config" => Application :: VndCluetrustCartomobileConfig , "application/vnd.cluetrust.cartomobile-config-pkg" => Application :: VndCluetrustCartomobileConfigPkg , "application/vnd.cncf.helm.chart.content.v1.tar+gzip" => Application :: VndCncfHelmChartContentV1TarGzip , "application/vnd.cncf.helm.chart.provenance.v1.prov" => Application :: VndCncfHelmChartProvenanceV1Prov , "application/vnd.coffeescript" => Application :: VndCoffeescript , "application/vnd.collabio.xodocuments.document" => Application :: VndCollabioXodocumentsDocument , "application/vnd.collabio.xodocuments.document-template" => Application :: VndCollabioXodocumentsDocumentTemplate , "application/vnd.collabio.xodocuments.presentation" => Application :: VndCollabioXodocumentsPresentation , "application/vnd.collabio.xodocuments.presentation-template" => Application :: VndCollabioXodocumentsPresentationTemplate , "application/vnd.collabio.xodocuments.spreadsheet" => Application :: VndCollabioXodocumentsSpreadsheet , "application/vnd.collabio.xodocuments.spreadsheet-template" => Application :: VndCollabioXodocumentsSpreadsheetTemplate , "application/vnd.collection.doc+json" => Application :: VndCollectionDocJson , "application/vnd.collection+json" => Application :: VndCollectionJson , "application/vnd.collection.next+json" => Application :: VndCollectionNextJson , "application/vnd.comicbook-rar" => Application :: VndComicbookRar , "application/vnd.comicbook+zip" => Application :: VndComicbookZip , "application/vnd.commerce-battelle" => Application :: VndCommerceBattelle , "application/vnd.commonspace" => Application :: VndCommonspace , "application/vnd.coreos.ignition+json" => Application :: VndCoreosIgnitionJson , "application/vnd.cosmocaller" => Application :: VndCosmocaller , "application/vnd.contact.cmsg" => Application :: VndContactCmsg , "application/vnd.crick.clicker" => Application :: VndCrickClicker , "application/vnd.crick.clicker.keyboard" => Application :: VndCrickClickerKeyboard , "application/vnd.crick.clicker.palette" => Application :: VndCrickClickerPalette , "application/vnd.crick.clicker.template" => Application :: VndCrickClickerTemplate , "application/vnd.crick.clicker.wordbank" => Application :: VndCrickClickerWordbank , "application/vnd.criticaltools.wbs+xml" => Application :: VndCriticaltoolsWbsXml , "application/vnd.cryptii.pipe+json" => Application :: VndCryptiiPipeJson , "application/vnd.crypto-shade-file" => Application :: VndCryptoShadeFile , "application/vnd.cryptomator.encrypted" => Application :: VndCryptomatorEncrypted , "application/vnd.cryptomator.vault" => Application :: VndCryptomatorVault , "application/vnd.ctc-posml" => Application :: VndCtcPosml , "application/vnd.ctct.ws+xml" => Application :: VndCtctWsXml , "application/vnd.cups-pdf" => Application :: VndCupsPdf , "application/vnd.cups-postscript" => Application :: VndCupsPostscript , "application/vnd.cups-ppd" => Application :: VndCupsPpd , "application/vnd.cups-raster" => Application :: VndCupsRaster , "application/vnd.cups-raw" => Application :: VndCupsRaw , "application/vnd.curl" => Application :: VndCurl , "application/vnd.cyan.dean.root+xml" => Application :: VndCyanDeanRootXml , "application/vnd.cybank" => Application :: VndCybank , "application/vnd.cyclonedx+json" => Application :: VndCyclonedxJson , "application/vnd.cyclonedx+xml" => Application :: VndCyclonedxXml , "application/vnd.d2l.coursepackage1p0+zip" => Application :: VndD2LCoursepackage1P0Zip , "application/vnd.d3m-dataset" => Application :: VndD3MDataset , "application/vnd.d3m-problem" => Application :: VndD3MProblem , "application/vnd.dart" => Application :: VndDart , "application/vnd.data-vision.rdz" => Application :: VndDataVisionRdz , "application/vnd.datalog" => Application :: VndDatalog , "application/vnd.datapackage+json" => Application :: VndDatapackageJson , "application/vnd.dataresource+json" => Application :: VndDataresourceJson , "application/vnd.dbf" => Application :: VndDbf , "application/vnd.debian.binary-package" => Application :: VndDebianBinaryPackage , "application/vnd.dece.data" => Application :: VndDeceData , "application/vnd.dece.ttml+xml" => Application :: VndDeceTtmlXml , "application/vnd.dece.unspecified" => Application :: VndDeceUnspecified , "application/vnd.dece.zip" => Application :: VndDeceZip , "application/vnd.denovo.fcselayout-link" => Application :: VndDenovoFcselayoutLink , "application/vnd.desmume.movie" => Application :: VndDesmumeMovie , "application/vnd.dir-bi.plate-dl-nosuffix" => Application :: VndDirBiPlateDlNosuffix , "application/vnd.dm.delegation+xml" => Application :: VndDmDelegationXml , "application/vnd.dna" => Application :: VndDna , "application/vnd.document+json" => Application :: VndDocumentJson , "application/vnd.dolby.mobile.1" => Application :: VndDolbyMobile1 , "application/vnd.dolby.mobile.2" => Application :: VndDolbyMobile2 , "application/vnd.doremir.scorecloud-binary-document" => Application :: VndDoremirScorecloudBinaryDocument , "application/vnd.dpgraph" => Application :: VndDpgraph , "application/vnd.dreamfactory" => Application :: VndDreamfactory , "application/vnd.drive+json" => Application :: VndDriveJson , "application/vnd.dtg.local" => Application :: VndDtgLocal , "application/vnd.dtg.local.flash" => Application :: VndDtgLocalFlash , "application/vnd.dtg.local.html" => Application :: VndDtgLocalHtml , "application/vnd.dvb.ait" => Application :: VndDvbAit , "application/vnd.dvb.dvbisl+xml" => Application :: VndDvbDvbislXml , "application/vnd.dvb.dvbj" => Application :: VndDvbDvbj , "application/vnd.dvb.esgcontainer" => Application :: VndDvbEsgcontainer , "application/vnd.dvb.ipdcdftnotifaccess" => Application :: VndDvbIpdcdftnotifaccess , "application/vnd.dvb.ipdcesgaccess" => Application :: VndDvbIpdcesgaccess , "application/vnd.dvb.ipdcesgaccess2" => Application :: VndDvbIpdcesgaccess2 , "application/vnd.dvb.ipdcesgpdd" => Application :: VndDvbIpdcesgpdd , "application/vnd.dvb.ipdcroaming" => Application :: VndDvbIpdcroaming , "application/vnd.dvb.iptv.alfec-base" => Application :: VndDvbIptvAlfecBase , "application/vnd.dvb.iptv.alfec-enhancement" => Application :: VndDvbIptvAlfecEnhancement , "application/vnd.dvb.notif-aggregate-root+xml" => Application :: VndDvbNotifAggregateRootXml , "application/vnd.dvb.notif-container+xml" => Application :: VndDvbNotifContainerXml , "application/vnd.dvb.notif-generic+xml" => Application :: VndDvbNotifGenericXml , "application/vnd.dvb.notif-ia-msglist+xml" => Application :: VndDvbNotifIaMsglistXml , "application/vnd.dvb.notif-ia-registration-request+xml" => Application :: VndDvbNotifIaRegistrationRequestXml , "application/vnd.dvb.notif-ia-registration-response+xml" => Application :: VndDvbNotifIaRegistrationResponseXml , "application/vnd.dvb.notif-init+xml" => Application :: VndDvbNotifInitXml , "application/vnd.dvb.pfr" => Application :: VndDvbPfr , "application/vnd.dvb.service" => Application :: VndDvbService , "application/vnd.dxr" => Application :: VndDxr , "application/vnd.dynageo" => Application :: VndDynageo , "application/vnd.dzr" => Application :: VndDzr , "application/vnd.easykaraoke.cdgdownload" => Application :: VndEasykaraokeCdgdownload , "application/vnd.ecip.rlp" => Application :: VndEcipRlp , "application/vnd.ecdis-update" => Application :: VndEcdisUpdate , "application/vnd.eclipse.ditto+json" => Application :: VndEclipseDittoJson , "application/vnd.ecowin.chart" => Application :: VndEcowinChart , "application/vnd.ecowin.filerequest" => Application :: VndEcowinFilerequest , "application/vnd.ecowin.fileupdate" => Application :: VndEcowinFileupdate , "application/vnd.ecowin.series" => Application :: VndEcowinSeries , "application/vnd.ecowin.seriesrequest" => Application :: VndEcowinSeriesrequest , "application/vnd.ecowin.seriesupdate" => Application :: VndEcowinSeriesupdate , "application/vnd.efi.img" => Application :: VndEfiImg , "application/vnd.efi.iso" => Application :: VndEfiIso , "application/vnd.emclient.accessrequest+xml" => Application :: VndEmclientAccessrequestXml , "application/vnd.enliven" => Application :: VndEnliven , "application/vnd.enphase.envoy" => Application :: VndEnphaseEnvoy , "application/vnd.eprints.data+xml" => Application :: VndEprintsDataXml , "application/vnd.epson.esf" => Application :: VndEpsonEsf , "application/vnd.epson.msf" => Application :: VndEpsonMsf , "application/vnd.epson.quickanime" => Application :: VndEpsonQuickanime , "application/vnd.epson.salt" => Application :: VndEpsonSalt , "application/vnd.epson.ssf" => Application :: VndEpsonSsf , "application/vnd.ericsson.quickcall" => Application :: VndEricssonQuickcall , "application/vnd.espass-espass+zip" => Application :: VndEspassEspassZip , "application/vnd.eszigno3+xml" => Application :: VndEszigno3Xml , "application/vnd.etsi.aoc+xml" => Application :: VndEtsiAocXml , "application/vnd.etsi.asic-s+zip" => Application :: VndEtsiAsicSZip , "application/vnd.etsi.asic-e+zip" => Application :: VndEtsiAsicEZip , "application/vnd.etsi.cug+xml" => Application :: VndEtsiCugXml , "application/vnd.etsi.iptvcommand+xml" => Application :: VndEtsiIptvcommandXml , "application/vnd.etsi.iptvdiscovery+xml" => Application :: VndEtsiIptvdiscoveryXml , "application/vnd.etsi.iptvprofile+xml" => Application :: VndEtsiIptvprofileXml , "application/vnd.etsi.iptvsad-bc+xml" => Application :: VndEtsiIptvsadBcXml , "application/vnd.etsi.iptvsad-cod+xml" => Application :: VndEtsiIptvsadCodXml , "application/vnd.etsi.iptvsad-npvr+xml" => Application :: VndEtsiIptvsadNpvrXml , "application/vnd.etsi.iptvservice+xml" => Application :: VndEtsiIptvserviceXml , "application/vnd.etsi.iptvsync+xml" => Application :: VndEtsiIptvsyncXml , "application/vnd.etsi.iptvueprofile+xml" => Application :: VndEtsiIptvueprofileXml , "application/vnd.etsi.mcid+xml" => Application :: VndEtsiMcidXml , "application/vnd.etsi.mheg5" => Application :: VndEtsiMheg5 , "application/vnd.etsi.overload-control-policy-dataset+xml" => Application :: VndEtsiOverloadControlPolicyDatasetXml , "application/vnd.etsi.pstn+xml" => Application :: VndEtsiPstnXml , "application/vnd.etsi.sci+xml" => Application :: VndEtsiSciXml , "application/vnd.etsi.simservs+xml" => Application :: VndEtsiSimservsXml , "application/vnd.etsi.timestamp-token" => Application :: VndEtsiTimestampToken , "application/vnd.etsi.tsl+xml" => Application :: VndEtsiTslXml , "application/vnd.etsi.tsl.der" => Application :: VndEtsiTslDer , "application/vnd.eu.kasparian.car+json" => Application :: VndEuKasparianCarJson , "application/vnd.eudora.data" => Application :: VndEudoraData , "application/vnd.evolv.ecig.profile" => Application :: VndEvolvEcigProfile , "application/vnd.evolv.ecig.settings" => Application :: VndEvolvEcigSettings , "application/vnd.evolv.ecig.theme" => Application :: VndEvolvEcigTheme , "application/vnd.exstream-empower+zip" => Application :: VndExstreamEmpowerZip , "application/vnd.exstream-package" => Application :: VndExstreamPackage , "application/vnd.ezpix-album" => Application :: VndEzpixAlbum , "application/vnd.ezpix-package" => Application :: VndEzpixPackage , "application/vnd.f-secure.mobile" => Application :: VndFSecureMobile , "application/vnd.fastcopy-disk-image" => Application :: VndFastcopyDiskImage , "application/vnd.familysearch.gedcom+zip" => Application :: VndFamilysearchGedcomZip , "application/vnd.fdsn.mseed" => Application :: VndFdsnMseed , "application/vnd.fdsn.seed" => Application :: VndFdsnSeed , "application/vnd.ffsns" => Application :: VndFfsns , "application/vnd.ficlab.flb+zip" => Application :: VndFiclabFlbZip , "application/vnd.filmit.zfc" => Application :: VndFilmitZfc , "application/vnd.fints" => Application :: VndFints , "application/vnd.firemonkeys.cloudcell" => Application :: VndFiremonkeysCloudcell , "application/vnd.FloGraphIt" => Application :: VndFloGraphIt , "application/vnd.fluxtime.clip" => Application :: VndFluxtimeClip , "application/vnd.font-fontforge-sfd" => Application :: VndFontFontforgeSfd , "application/vnd.framemaker" => Application :: VndFramemaker , "application/vnd.fsc.weblaunch" => Application :: VndFscWeblaunch , "application/vnd.fujifilm.fb.docuworks" => Application :: VndFujifilmFbDocuworks , "application/vnd.fujifilm.fb.docuworks.binder" => Application :: VndFujifilmFbDocuworksBinder , "application/vnd.fujifilm.fb.docuworks.container" => Application :: VndFujifilmFbDocuworksContainer , "application/vnd.fujifilm.fb.jfi+xml" => Application :: VndFujifilmFbJfiXml , "application/vnd.fujitsu.oasys" => Application :: VndFujitsuOasys , "application/vnd.fujitsu.oasys2" => Application :: VndFujitsuOasys2 , "application/vnd.fujitsu.oasys3" => Application :: VndFujitsuOasys3 , "application/vnd.fujitsu.oasysgp" => Application :: VndFujitsuOasysgp , "application/vnd.fujitsu.oasysprs" => Application :: VndFujitsuOasysprs , "application/vnd.fujixerox.ART4" => Application :: VndFujixeroxART4 , "application/vnd.fujixerox.ART-EX" => Application :: VndFujixeroxARTEX , "application/vnd.fujixerox.ddd" => Application :: VndFujixeroxDdd , "application/vnd.fujixerox.docuworks" => Application :: VndFujixeroxDocuworks , "application/vnd.fujixerox.docuworks.binder" => Application :: VndFujixeroxDocuworksBinder , "application/vnd.fujixerox.docuworks.container" => Application :: VndFujixeroxDocuworksContainer , "application/vnd.fujixerox.HBPL" => Application :: VndFujixeroxHBPL , "application/vnd.fut-misnet" => Application :: VndFutMisnet , "application/vnd.futoin+cbor" => Application :: VndFutoinCbor , "application/vnd.futoin+json" => Application :: VndFutoinJson , "application/vnd.fuzzysheet" => Application :: VndFuzzysheet , "application/vnd.genomatix.tuxedo" => Application :: VndGenomatixTuxedo , "application/vnd.genozip" => Application :: VndGenozip , "application/vnd.gentics.grd+json" => Application :: VndGenticsGrdJson , "application/vnd.gentoo.catmetadata+xml" => Application :: VndGentooCatmetadataXml , "application/vnd.gentoo.ebuild" => Application :: VndGentooEbuild , "application/vnd.gentoo.eclass" => Application :: VndGentooEclass , "application/vnd.gentoo.manifest" => Application :: VndGentooManifest , "application/vnd.gentoo.pkgmetadata+xml" => Application :: VndGentooPkgmetadataXml , "application/vnd.geogebra.file" => Application :: VndGeogebraFile , "application/vnd.geogebra.slides" => Application :: VndGeogebraSlides , "application/vnd.geogebra.tool" => Application :: VndGeogebraTool , "application/vnd.geometry-explorer" => Application :: VndGeometryExplorer , "application/vnd.geonext" => Application :: VndGeonext , "application/vnd.geoplan" => Application :: VndGeoplan , "application/vnd.geospace" => Application :: VndGeospace , "application/vnd.gerber" => Application :: VndGerber , "application/vnd.globalplatform.card-content-mgt" => Application :: VndGlobalplatformCardContentMgt , "application/vnd.globalplatform.card-content-mgt-response" => Application :: VndGlobalplatformCardContentMgtResponse , "application/vnd.gnu.taler.exchange+json" => Application :: VndGnuTalerExchangeJson , "application/vnd.gnu.taler.merchant+json" => Application :: VndGnuTalerMerchantJson , "application/vnd.google-earth.kml+xml" => Application :: VndGoogleEarthKmlXml , "application/vnd.google-earth.kmz" => Application :: VndGoogleEarthKmz , "application/vnd.gov.sk.e-form+xml" => Application :: VndGovSkEFormXml , "application/vnd.gov.sk.e-form+zip" => Application :: VndGovSkEFormZip , "application/vnd.gov.sk.xmldatacontainer+xml" => Application :: VndGovSkXmldatacontainerXml , "application/vnd.grafeq" => Application :: VndGrafeq , "application/vnd.gridmp" => Application :: VndGridmp , "application/vnd.groove-account" => Application :: VndGrooveAccount , "application/vnd.groove-help" => Application :: VndGrooveHelp , "application/vnd.groove-identity-message" => Application :: VndGrooveIdentityMessage , "application/vnd.groove-injector" => Application :: VndGrooveInjector , "application/vnd.groove-tool-message" => Application :: VndGrooveToolMessage , "application/vnd.groove-tool-template" => Application :: VndGrooveToolTemplate , "application/vnd.groove-vcard" => Application :: VndGrooveVcard , "application/vnd.hal+json" => Application :: VndHalJson , "application/vnd.hal+xml" => Application :: VndHalXml , "application/vnd.HandHeld-Entertainment+xml" => Application :: VndHandHeldEntertainmentXml , "application/vnd.hbci" => Application :: VndHbci , "application/vnd.hc+json" => Application :: VndHcJson , "application/vnd.hcl-bireports" => Application :: VndHclBireports , "application/vnd.hdt" => Application :: VndHdt , "application/vnd.heroku+json" => Application :: VndHerokuJson , "application/vnd.hhe.lesson-player" => Application :: VndHheLessonPlayer , "application/vnd.hp-HPGL" => Application :: VndHpHPGL , "application/vnd.hp-hpid" => Application :: VndHpHpid , "application/vnd.hp-hps" => Application :: VndHpHps , "application/vnd.hp-jlyt" => Application :: VndHpJlyt , "application/vnd.hp-PCL" => Application :: VndHpPCL , "application/vnd.hp-PCLXL" => Application :: VndHpPCLXL , "application/vnd.httphone" => Application :: VndHttphone , "application/vnd.hydrostatix.sof-data" => Application :: VndHydrostatixSofData , "application/vnd.hyper-item+json" => Application :: VndHyperItemJson , "application/vnd.hyper+json" => Application :: VndHyperJson , "application/vnd.hyperdrive+json" => Application :: VndHyperdriveJson , "application/vnd.hzn-3d-crossword" => Application :: VndHzn3DCrossword , "application/vnd.ibm.electronic-media" => Application :: VndIbmElectronicMedia , "application/vnd.ibm.MiniPay" => Application :: VndIbmMiniPay , "application/vnd.ibm.rights-management" => Application :: VndIbmRightsManagement , "application/vnd.ibm.secure-container" => Application :: VndIbmSecureContainer , "application/vnd.iccprofile" => Application :: VndIccprofile , "application/vnd.ieee.1905" => Application :: VndIeee1905 , "application/vnd.igloader" => Application :: VndIgloader , "application/vnd.imagemeter.folder+zip" => Application :: VndImagemeterFolderZip , "application/vnd.imagemeter.image+zip" => Application :: VndImagemeterImageZip , "application/vnd.immervision-ivp" => Application :: VndImmervisionIvp , "application/vnd.immervision-ivu" => Application :: VndImmervisionIvu , "application/vnd.ims.imsccv1p1" => Application :: VndImsImsccv1P1 , "application/vnd.ims.imsccv1p2" => Application :: VndImsImsccv1P2 , "application/vnd.ims.imsccv1p3" => Application :: VndImsImsccv1P3 , "application/vnd.ims.lis.v2.result+json" => Application :: VndImsLisV2ResultJson , "application/vnd.ims.lti.v2.toolconsumerprofile+json" => Application :: VndImsLtiV2ToolconsumerprofileJson , "application/vnd.ims.lti.v2.toolproxy.id+json" => Application :: VndImsLtiV2ToolproxyIdJson , "application/vnd.ims.lti.v2.toolproxy+json" => Application :: VndImsLtiV2ToolproxyJson , "application/vnd.ims.lti.v2.toolsettings+json" => Application :: VndImsLtiV2ToolsettingsJson , "application/vnd.ims.lti.v2.toolsettings.simple+json" => Application :: VndImsLtiV2ToolsettingsSimpleJson , "application/vnd.informedcontrol.rms+xml" => Application :: VndInformedcontrolRmsXml , "application/vnd.infotech.project" => Application :: VndInfotechProject , "application/vnd.infotech.project+xml" => Application :: VndInfotechProjectXml , "application/vnd.innopath.wamp.notification" => Application :: VndInnopathWampNotification , "application/vnd.insors.igm" => Application :: VndInsorsIgm , "application/vnd.intercon.formnet" => Application :: VndInterconFormnet , "application/vnd.intergeo" => Application :: VndIntergeo , "application/vnd.intertrust.digibox" => Application :: VndIntertrustDigibox , "application/vnd.intertrust.nncp" => Application :: VndIntertrustNncp , "application/vnd.intu.qbo" => Application :: VndIntuQbo , "application/vnd.intu.qfx" => Application :: VndIntuQfx , "application/vnd.ipld.car" => Application :: VndIpldCar , "application/vnd.ipld.dag-cbor" => Application :: VndIpldDagCbor , "application/vnd.ipld.dag-json" => Application :: VndIpldDagJson , "application/vnd.ipld.raw" => Application :: VndIpldRaw , "application/vnd.iptc.g2.catalogitem+xml" => Application :: VndIptcG2CatalogitemXml , "application/vnd.iptc.g2.conceptitem+xml" => Application :: VndIptcG2ConceptitemXml , "application/vnd.iptc.g2.knowledgeitem+xml" => Application :: VndIptcG2KnowledgeitemXml , "application/vnd.iptc.g2.newsitem+xml" => Application :: VndIptcG2NewsitemXml , "application/vnd.iptc.g2.newsmessage+xml" => Application :: VndIptcG2NewsmessageXml , "application/vnd.iptc.g2.packageitem+xml" => Application :: VndIptcG2PackageitemXml , "application/vnd.iptc.g2.planningitem+xml" => Application :: VndIptcG2PlanningitemXml , "application/vnd.ipunplugged.rcprofile" => Application :: VndIpunpluggedRcprofile , "application/vnd.irepository.package+xml" => Application :: VndIrepositoryPackageXml , "application/vnd.is-xpr" => Application :: VndIsXpr , "application/vnd.isac.fcs" => Application :: VndIsacFcs , "application/vnd.jam" => Application :: VndJam , "application/vnd.iso11783-10+zip" => Application :: VndIso1178310Zip , "application/vnd.japannet-directory-service" => Application :: VndJapannetDirectoryService , "application/vnd.japannet-jpnstore-wakeup" => Application :: VndJapannetJpnstoreWakeup , "application/vnd.japannet-payment-wakeup" => Application :: VndJapannetPaymentWakeup , "application/vnd.japannet-registration" => Application :: VndJapannetRegistration , "application/vnd.japannet-registration-wakeup" => Application :: VndJapannetRegistrationWakeup , "application/vnd.japannet-setstore-wakeup" => Application :: VndJapannetSetstoreWakeup , "application/vnd.japannet-verification" => Application :: VndJapannetVerification , "application/vnd.japannet-verification-wakeup" => Application :: VndJapannetVerificationWakeup , "application/vnd.jcp.javame.midlet-rms" => Application :: VndJcpJavameMidletRms , "application/vnd.jisp" => Application :: VndJisp , "application/vnd.joost.joda-archive" => Application :: VndJoostJodaArchive , "application/vnd.jsk.isdn-ngn" => Application :: VndJskIsdnNgn , "application/vnd.kahootz" => Application :: VndKahootz , "application/vnd.kde.karbon" => Application :: VndKdeKarbon , "application/vnd.kde.kchart" => Application :: VndKdeKchart , "application/vnd.kde.kformula" => Application :: VndKdeKformula , "application/vnd.kde.kivio" => Application :: VndKdeKivio , "application/vnd.kde.kontour" => Application :: VndKdeKontour , "application/vnd.kde.kpresenter" => Application :: VndKdeKpresenter , "application/vnd.kde.kspread" => Application :: VndKdeKspread , "application/vnd.kde.kword" => Application :: VndKdeKword , "application/vnd.kenameaapp" => Application :: VndKenameaapp , "application/vnd.kidspiration" => Application :: VndKidspiration , "application/vnd.Kinar" => Application :: VndKinar , "application/vnd.koan" => Application :: VndKoan , "application/vnd.kodak-descriptor" => Application :: VndKodakDescriptor , "application/vnd.las" => Application :: VndLas , "application/vnd.las.las+json" => Application :: VndLasLasJson , "application/vnd.las.las+xml" => Application :: VndLasLasXml , "application/vnd.laszip" => Application :: VndLaszip , "application/vnd.leap+json" => Application :: VndLeapJson , "application/vnd.liberty-request+xml" => Application :: VndLibertyRequestXml , "application/vnd.llamagraphics.life-balance.desktop" => Application :: VndLlamagraphicsLifeBalanceDesktop , "application/vnd.llamagraphics.life-balance.exchange+xml" => Application :: VndLlamagraphicsLifeBalanceExchangeXml , "application/vnd.logipipe.circuit+zip" => Application :: VndLogipipeCircuitZip , "application/vnd.loom" => Application :: VndLoom , "application/vnd.lotus-1-2-3" => Application :: VndLotus123 , "application/vnd.lotus-approach" => Application :: VndLotusApproach , "application/vnd.lotus-freelance" => Application :: VndLotusFreelance , "application/vnd.lotus-notes" => Application :: VndLotusNotes , "application/vnd.lotus-organizer" => Application :: VndLotusOrganizer , "application/vnd.lotus-screencam" => Application :: VndLotusScreencam , "application/vnd.lotus-wordpro" => Application :: VndLotusWordpro , "application/vnd.macports.portpkg" => Application :: VndMacportsPortpkg , "application/vnd.mapbox-vector-tile" => Application :: VndMapboxVectorTile , "application/vnd.marlin.drm.actiontoken+xml" => Application :: VndMarlinDrmActiontokenXml , "application/vnd.marlin.drm.conftoken+xml" => Application :: VndMarlinDrmConftokenXml , "application/vnd.marlin.drm.license+xml" => Application :: VndMarlinDrmLicenseXml , "application/vnd.marlin.drm.mdcf" => Application :: VndMarlinDrmMdcf , "application/vnd.mason+json" => Application :: VndMasonJson , "application/vnd.maxar.archive.3tz+zip" => Application :: VndMaxarArchive3TzZip , "application/vnd.maxmind.maxmind-db" => Application :: VndMaxmindMaxmindDb , "application/vnd.mcd" => Application :: VndMcd , "application/vnd.medcalcdata" => Application :: VndMedcalcdata , "application/vnd.mediastation.cdkey" => Application :: VndMediastationCdkey , "application/vnd.meridian-slingshot" => Application :: VndMeridianSlingshot , "application/vnd.MFER" => Application :: VndMFER , "application/vnd.mfmp" => Application :: VndMfmp , "application/vnd.micro+json" => Application :: VndMicroJson , "application/vnd.micrografx.flo" => Application :: VndMicrografxFlo , "application/vnd.micrografx.igx" => Application :: VndMicrografxIgx , "application/vnd.microsoft.portable-executable" => Application :: VndMicrosoftPortableExecutable , "application/vnd.microsoft.windows.thumbnail-cache" => Application :: VndMicrosoftWindowsThumbnailCache , "application/vnd.miele+json" => Application :: VndMieleJson , "application/vnd.mif" => Application :: VndMif , "application/vnd.minisoft-hp3000-save" => Application :: VndMinisoftHp3000Save , "application/vnd.mitsubishi.misty-guard.trustweb" => Application :: VndMitsubishiMistyGuardTrustweb , "application/vnd.Mobius.DAF" => Application :: VndMobiusDAF , "application/vnd.Mobius.DIS" => Application :: VndMobiusDIS , "application/vnd.Mobius.MBK" => Application :: VndMobiusMBK , "application/vnd.Mobius.MQY" => Application :: VndMobiusMQY , "application/vnd.Mobius.MSL" => Application :: VndMobiusMSL , "application/vnd.Mobius.PLC" => Application :: VndMobiusPLC , "application/vnd.Mobius.TXF" => Application :: VndMobiusTXF , "application/vnd.mophun.application" => Application :: VndMophunApplication , "application/vnd.mophun.certificate" => Application :: VndMophunCertificate , "application/vnd.motorola.flexsuite" => Application :: VndMotorolaFlexsuite , "application/vnd.motorola.flexsuite.adsi" => Application :: VndMotorolaFlexsuiteAdsi , "application/vnd.motorola.flexsuite.fis" => Application :: VndMotorolaFlexsuiteFis , "application/vnd.motorola.flexsuite.gotap" => Application :: VndMotorolaFlexsuiteGotap , "application/vnd.motorola.flexsuite.kmr" => Application :: VndMotorolaFlexsuiteKmr , "application/vnd.motorola.flexsuite.ttc" => Application :: VndMotorolaFlexsuiteTtc , "application/vnd.motorola.flexsuite.wem" => Application :: VndMotorolaFlexsuiteWem , "application/vnd.motorola.iprm" => Application :: VndMotorolaIprm , "application/vnd.mozilla.xul+xml" => Application :: VndMozillaXulXml , "application/vnd.ms-artgalry" => Application :: VndMsArtgalry , "application/vnd.ms-asf" => Application :: VndMsAsf , "application/vnd.ms-cab-compressed" => Application :: VndMsCabCompressed , "application/vnd.ms-3mfdocument" => Application :: VndMs3Mfdocument , "application/vnd.ms-excel" => Application :: VndMsExcel , "application/vnd.ms-excel.addin.macroEnabled.12" => Application :: VndMsExcelAddinMacroEnabled12 , "application/vnd.ms-excel.sheet.binary.macroEnabled.12" => Application :: VndMsExcelSheetBinaryMacroEnabled12 , "application/vnd.ms-excel.sheet.macroEnabled.12" => Application :: VndMsExcelSheetMacroEnabled12 , "application/vnd.ms-excel.template.macroEnabled.12" => Application :: VndMsExcelTemplateMacroEnabled12 , "application/vnd.ms-fontobject" => Application :: VndMsFontobject , "application/vnd.ms-htmlhelp" => Application :: VndMsHtmlhelp , "application/vnd.ms-ims" => Application :: VndMsIms , "application/vnd.ms-lrm" => Application :: VndMsLrm , "application/vnd.ms-office.activeX+xml" => Application :: VndMsOfficeActiveXXml , "application/vnd.ms-officetheme" => Application :: VndMsOfficetheme , "application/vnd.ms-playready.initiator+xml" => Application :: VndMsPlayreadyInitiatorXml , "application/vnd.ms-powerpoint" => Application :: VndMsPowerpoint , "application/vnd.ms-powerpoint.addin.macroEnabled.12" => Application :: VndMsPowerpointAddinMacroEnabled12 , "application/vnd.ms-powerpoint.presentation.macroEnabled.12" => Application :: VndMsPowerpointPresentationMacroEnabled12 , "application/vnd.ms-powerpoint.slide.macroEnabled.12" => Application :: VndMsPowerpointSlideMacroEnabled12 , "application/vnd.ms-powerpoint.slideshow.macroEnabled.12" => Application :: VndMsPowerpointSlideshowMacroEnabled12 , "application/vnd.ms-powerpoint.template.macroEnabled.12" => Application :: VndMsPowerpointTemplateMacroEnabled12 , "application/vnd.ms-PrintDeviceCapabilities+xml" => Application :: VndMsPrintDeviceCapabilitiesXml , "application/vnd.ms-PrintSchemaTicket+xml" => Application :: VndMsPrintSchemaTicketXml , "application/vnd.ms-project" => Application :: VndMsProject , "application/vnd.ms-tnef" => Application :: VndMsTnef , "application/vnd.ms-windows.devicepairing" => Application :: VndMsWindowsDevicepairing , "application/vnd.ms-windows.nwprinting.oob" => Application :: VndMsWindowsNwprintingOob , "application/vnd.ms-windows.printerpairing" => Application :: VndMsWindowsPrinterpairing , "application/vnd.ms-windows.wsd.oob" => Application :: VndMsWindowsWsdOob , "application/vnd.ms-wmdrm.lic-chlg-req" => Application :: VndMsWmdrmLicChlgReq , "application/vnd.ms-wmdrm.lic-resp" => Application :: VndMsWmdrmLicResp , "application/vnd.ms-wmdrm.meter-chlg-req" => Application :: VndMsWmdrmMeterChlgReq , "application/vnd.ms-wmdrm.meter-resp" => Application :: VndMsWmdrmMeterResp , "application/vnd.ms-word.document.macroEnabled.12" => Application :: VndMsWordDocumentMacroEnabled12 , "application/vnd.ms-word.template.macroEnabled.12" => Application :: VndMsWordTemplateMacroEnabled12 , "application/vnd.ms-works" => Application :: VndMsWorks , "application/vnd.ms-wpl" => Application :: VndMsWpl , "application/vnd.ms-xpsdocument" => Application :: VndMsXpsdocument , "application/vnd.msa-disk-image" => Application :: VndMsaDiskImage , "application/vnd.mseq" => Application :: VndMseq , "application/vnd.msign" => Application :: VndMsign , "application/vnd.multiad.creator" => Application :: VndMultiadCreator , "application/vnd.multiad.creator.cif" => Application :: VndMultiadCreatorCif , "application/vnd.musician" => Application :: VndMusician , "application/vnd.music-niff" => Application :: VndMusicNiff , "application/vnd.muvee.style" => Application :: VndMuveeStyle , "application/vnd.mynfc" => Application :: VndMynfc , "application/vnd.nacamar.ybrid+json" => Application :: VndNacamarYbridJson , "application/vnd.ncd.control" => Application :: VndNcdControl , "application/vnd.ncd.reference" => Application :: VndNcdReference , "application/vnd.nearst.inv+json" => Application :: VndNearstInvJson , "application/vnd.nebumind.line" => Application :: VndNebumindLine , "application/vnd.nervana" => Application :: VndNervana , "application/vnd.netfpx" => Application :: VndNetfpx , "application/vnd.neurolanguage.nlu" => Application :: VndNeurolanguageNlu , "application/vnd.nimn" => Application :: VndNimn , "application/vnd.nintendo.snes.rom" => Application :: VndNintendoSnesRom , "application/vnd.nintendo.nitro.rom" => Application :: VndNintendoNitroRom , "application/vnd.nitf" => Application :: VndNitf , "application/vnd.noblenet-directory" => Application :: VndNoblenetDirectory , "application/vnd.noblenet-sealer" => Application :: VndNoblenetSealer , "application/vnd.noblenet-web" => Application :: VndNoblenetWeb , "application/vnd.nokia.catalogs" => Application :: VndNokiaCatalogs , "application/vnd.nokia.conml+wbxml" => Application :: VndNokiaConmlWbxml , "application/vnd.nokia.conml+xml" => Application :: VndNokiaConmlXml , "application/vnd.nokia.iptv.config+xml" => Application :: VndNokiaIptvConfigXml , "application/vnd.nokia.iSDS-radio-presets" => Application :: VndNokiaISDSRadioPresets , "application/vnd.nokia.landmark+wbxml" => Application :: VndNokiaLandmarkWbxml , "application/vnd.nokia.landmark+xml" => Application :: VndNokiaLandmarkXml , "application/vnd.nokia.landmarkcollection+xml" => Application :: VndNokiaLandmarkcollectionXml , "application/vnd.nokia.ncd" => Application :: VndNokiaNcd , "application/vnd.nokia.n-gage.ac+xml" => Application :: VndNokiaNGageAcXml , "application/vnd.nokia.n-gage.data" => Application :: VndNokiaNGageData , "application/vnd.nokia.pcd+wbxml" => Application :: VndNokiaPcdWbxml , "application/vnd.nokia.pcd+xml" => Application :: VndNokiaPcdXml , "application/vnd.nokia.radio-preset" => Application :: VndNokiaRadioPreset , "application/vnd.nokia.radio-presets" => Application :: VndNokiaRadioPresets , "application/vnd.novadigm.EDM" => Application :: VndNovadigmEDM , "application/vnd.novadigm.EDX" => Application :: VndNovadigmEDX , "application/vnd.novadigm.EXT" => Application :: VndNovadigmEXT , "application/vnd.ntt-local.content-share" => Application :: VndNttLocalContentShare , "application/vnd.ntt-local.file-transfer" => Application :: VndNttLocalFileTransfer , "application/vnd.ntt-local.ogw_remote-access" => Application :: VndNttLocalOgwRemoteAccess , "application/vnd.ntt-local.sip-ta_remote" => Application :: VndNttLocalSipTaRemote , "application/vnd.ntt-local.sip-ta_tcp_stream" => Application :: VndNttLocalSipTaTcpStream , "application/vnd.oasis.opendocument.base" => Application :: VndOasisOpendocumentBase , "application/vnd.oasis.opendocument.chart" => Application :: VndOasisOpendocumentChart , "application/vnd.oasis.opendocument.chart-template" => Application :: VndOasisOpendocumentChartTemplate , "application/vnd.oasis.opendocument.formula" => Application :: VndOasisOpendocumentFormula , "application/vnd.oasis.opendocument.formula-template" => Application :: VndOasisOpendocumentFormulaTemplate , "application/vnd.oasis.opendocument.graphics" => Application :: VndOasisOpendocumentGraphics , "application/vnd.oasis.opendocument.graphics-template" => Application :: VndOasisOpendocumentGraphicsTemplate , "application/vnd.oasis.opendocument.image" => Application :: VndOasisOpendocumentImage , "application/vnd.oasis.opendocument.image-template" => Application :: VndOasisOpendocumentImageTemplate , "application/vnd.oasis.opendocument.presentation" => Application :: VndOasisOpendocumentPresentation , "application/vnd.oasis.opendocument.presentation-template" => Application :: VndOasisOpendocumentPresentationTemplate , "application/vnd.oasis.opendocument.spreadsheet" => Application :: VndOasisOpendocumentSpreadsheet , "application/vnd.oasis.opendocument.spreadsheet-template" => Application :: VndOasisOpendocumentSpreadsheetTemplate , "application/vnd.oasis.opendocument.text" => Application :: VndOasisOpendocumentText , "application/vnd.oasis.opendocument.text-master" => Application :: VndOasisOpendocumentTextMaster , "application/vnd.oasis.opendocument.text-template" => Application :: VndOasisOpendocumentTextTemplate , "application/vnd.oasis.opendocument.text-web" => Application :: VndOasisOpendocumentTextWeb , "application/vnd.obn" => Application :: VndObn , "application/vnd.ocf+cbor" => Application :: VndOcfCbor , "application/vnd.oci.image.manifest.v1+json" => Application :: VndOciImageManifestV1Json , "application/vnd.oftn.l10n+json" => Application :: VndOftnL10NJson , "application/vnd.oipf.contentaccessdownload+xml" => Application :: VndOipfContentaccessdownloadXml , "application/vnd.oipf.contentaccessstreaming+xml" => Application :: VndOipfContentaccessstreamingXml , "application/vnd.oipf.cspg-hexbinary" => Application :: VndOipfCspgHexbinary , "application/vnd.oipf.dae.svg+xml" => Application :: VndOipfDaeSvgXml , "application/vnd.oipf.dae.xhtml+xml" => Application :: VndOipfDaeXhtmlXml , "application/vnd.oipf.mippvcontrolmessage+xml" => Application :: VndOipfMippvcontrolmessageXml , "application/vnd.oipf.pae.gem" => Application :: VndOipfPaeGem , "application/vnd.oipf.spdiscovery+xml" => Application :: VndOipfSpdiscoveryXml , "application/vnd.oipf.spdlist+xml" => Application :: VndOipfSpdlistXml , "application/vnd.oipf.ueprofile+xml" => Application :: VndOipfUeprofileXml , "application/vnd.oipf.userprofile+xml" => Application :: VndOipfUserprofileXml , "application/vnd.olpc-sugar" => Application :: VndOlpcSugar , "application/vnd.oma.bcast.associated-procedure-parameter+xml" => Application :: VndOmaBcastAssociatedProcedureParameterXml , "application/vnd.oma.bcast.drm-trigger+xml" => Application :: VndOmaBcastDrmTriggerXml , "application/vnd.oma.bcast.imd+xml" => Application :: VndOmaBcastImdXml , "application/vnd.oma.bcast.ltkm" => Application :: VndOmaBcastLtkm , "application/vnd.oma.bcast.notification+xml" => Application :: VndOmaBcastNotificationXml , "application/vnd.oma.bcast.provisioningtrigger" => Application :: VndOmaBcastProvisioningtrigger , "application/vnd.oma.bcast.sgboot" => Application :: VndOmaBcastSgboot , "application/vnd.oma.bcast.sgdd+xml" => Application :: VndOmaBcastSgddXml , "application/vnd.oma.bcast.sgdu" => Application :: VndOmaBcastSgdu , "application/vnd.oma.bcast.simple-symbol-container" => Application :: VndOmaBcastSimpleSymbolContainer , "application/vnd.oma.bcast.smartcard-trigger+xml" => Application :: VndOmaBcastSmartcardTriggerXml , "application/vnd.oma.bcast.sprov+xml" => Application :: VndOmaBcastSprovXml , "application/vnd.oma.bcast.stkm" => Application :: VndOmaBcastStkm , "application/vnd.oma.cab-address-book+xml" => Application :: VndOmaCabAddressBookXml , "application/vnd.oma.cab-feature-handler+xml" => Application :: VndOmaCabFeatureHandlerXml , "application/vnd.oma.cab-pcc+xml" => Application :: VndOmaCabPccXml , "application/vnd.oma.cab-subs-invite+xml" => Application :: VndOmaCabSubsInviteXml , "application/vnd.oma.cab-user-prefs+xml" => Application :: VndOmaCabUserPrefsXml , "application/vnd.oma.dcd" => Application :: VndOmaDcd , "application/vnd.oma.dcdc" => Application :: VndOmaDcdc , "application/vnd.oma.dd2+xml" => Application :: VndOmaDd2Xml , "application/vnd.oma.drm.risd+xml" => Application :: VndOmaDrmRisdXml , "application/vnd.oma.group-usage-list+xml" => Application :: VndOmaGroupUsageListXml , "application/vnd.oma.lwm2m+cbor" => Application :: VndOmaLwm2MCbor , "application/vnd.oma.lwm2m+json" => Application :: VndOmaLwm2MJson , "application/vnd.oma.lwm2m+tlv" => Application :: VndOmaLwm2MTlv , "application/vnd.oma.pal+xml" => Application :: VndOmaPalXml , "application/vnd.oma.poc.detailed-progress-report+xml" => Application :: VndOmaPocDetailedProgressReportXml , "application/vnd.oma.poc.final-report+xml" => Application :: VndOmaPocFinalReportXml , "application/vnd.oma.poc.groups+xml" => Application :: VndOmaPocGroupsXml , "application/vnd.oma.poc.invocation-descriptor+xml" => Application :: VndOmaPocInvocationDescriptorXml , "application/vnd.oma.poc.optimized-progress-report+xml" => Application :: VndOmaPocOptimizedProgressReportXml , "application/vnd.oma.push" => Application :: VndOmaPush , "application/vnd.oma.scidm.messages+xml" => Application :: VndOmaScidmMessagesXml , "application/vnd.oma.xcap-directory+xml" => Application :: VndOmaXcapDirectoryXml , "application/vnd.omads-email+xml" => Application :: VndOmadsEmailXml , "application/vnd.omads-file+xml" => Application :: VndOmadsFileXml , "application/vnd.omads-folder+xml" => Application :: VndOmadsFolderXml , "application/vnd.omaloc-supl-init" => Application :: VndOmalocSuplInit , "application/vnd.oma-scws-config" => Application :: VndOmaScwsConfig , "application/vnd.oma-scws-http-request" => Application :: VndOmaScwsHttpRequest , "application/vnd.oma-scws-http-response" => Application :: VndOmaScwsHttpResponse , "application/vnd.onepager" => Application :: VndOnepager , "application/vnd.onepagertamp" => Application :: VndOnepagertamp , "application/vnd.onepagertamx" => Application :: VndOnepagertamx , "application/vnd.onepagertat" => Application :: VndOnepagertat , "application/vnd.onepagertatp" => Application :: VndOnepagertatp , "application/vnd.onepagertatx" => Application :: VndOnepagertatx , "application/vnd.onvif.metadata" => Application :: VndOnvifMetadata , "application/vnd.openblox.game-binary" => Application :: VndOpenbloxGameBinary , "application/vnd.openblox.game+xml" => Application :: VndOpenbloxGameXml , "application/vnd.openeye.oeb" => Application :: VndOpeneyeOeb , "application/vnd.openstreetmap.data+xml" => Application :: VndOpenstreetmapDataXml , "application/vnd.opentimestamps.ots" => Application :: VndOpentimestampsOts , "application/vnd.openxmlformats-officedocument.custom-properties+xml" => Application :: VndOpenxmlformatsOfficedocumentCustomPropertiesXml , "application/vnd.openxmlformats-officedocument.customXmlProperties+xml" => Application :: VndOpenxmlformatsOfficedocumentCustomXmlPropertiesXml , "application/vnd.openxmlformats-officedocument.drawing+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingXml , "application/vnd.openxmlformats-officedocument.drawingml.chart+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartXml , "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingmlChartshapesXml , "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramColorsXml , "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramDataXml , "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramLayoutXml , "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml" => Application :: VndOpenxmlformatsOfficedocumentDrawingmlDiagramStyleXml , "application/vnd.openxmlformats-officedocument.extended-properties+xml" => Application :: VndOpenxmlformatsOfficedocumentExtendedPropertiesXml , "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentAuthorsXml , "application/vnd.openxmlformats-officedocument.presentationml.comments+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlCommentsXml , "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlHandoutMasterXml , "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesMasterXml , "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlNotesSlideXml , "application/vnd.openxmlformats-officedocument.presentationml.presentation" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentation , "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresentationMainXml , "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlPresPropsXml , "application/vnd.openxmlformats-officedocument.presentationml.slide" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlide , "application/vnd.openxmlformats-officedocument.presentationml.slide+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideXml , "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideLayoutXml , "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideMasterXml , "application/vnd.openxmlformats-officedocument.presentationml.slideshow" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshow , "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideshowMainXml , "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlSlideUpdateInfoXml , "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlTableStylesXml , "application/vnd.openxmlformats-officedocument.presentationml.tags+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlTagsXml , "application/vnd.openxmlformats-officedocument.presentationml.template" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplate , "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlTemplateMainXml , "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml" => Application :: VndOpenxmlformatsOfficedocumentPresentationmlViewPropsXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCalcChainXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlChartsheetXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlCommentsXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlConnectionsXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlDialogsheetXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlExternalLinkXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheDefinitionXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotCacheRecordsXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlPivotTableXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlQueryTableXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionHeadersXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlRevisionLogXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSharedStringsXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheet , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMainXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlSheetMetadataXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlStylesXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTableSingleCellsXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.template" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplate , "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlTemplateMainXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlUserNamesXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlVolatileDependenciesXml , "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml" => Application :: VndOpenxmlformatsOfficedocumentSpreadsheetmlWorksheetXml , "application/vnd.openxmlformats-officedocument.theme+xml" => Application :: VndOpenxmlformatsOfficedocumentThemeXml , "application/vnd.openxmlformats-officedocument.themeOverride+xml" => Application :: VndOpenxmlformatsOfficedocumentThemeOverrideXml , "application/vnd.openxmlformats-officedocument.vmlDrawing" => Application :: VndOpenxmlformatsOfficedocumentVmlDrawing , "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlCommentsXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocument , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentGlossaryXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlDocumentMainXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlEndnotesXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFontTableXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFooterXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlFootnotesXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlNumberingXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlSettingsXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlStylesXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.template" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplate , "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlTemplateMainXml , "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml" => Application :: VndOpenxmlformatsOfficedocumentWordprocessingmlWebSettingsXml , "application/vnd.openxmlformats-package.core-properties+xml" => Application :: VndOpenxmlformatsPackageCorePropertiesXml , "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml" => Application :: VndOpenxmlformatsPackageDigitalSignatureXmlsignatureXml , "application/vnd.openxmlformats-package.relationships+xml" => Application :: VndOpenxmlformatsPackageRelationshipsXml , "application/vnd.oracle.resource+json" => Application :: VndOracleResourceJson , "application/vnd.orange.indata" => Application :: VndOrangeIndata , "application/vnd.osa.netdeploy" => Application :: VndOsaNetdeploy , "application/vnd.osgeo.mapguide.package" => Application :: VndOsgeoMapguidePackage , "application/vnd.osgi.bundle" => Application :: VndOsgiBundle , "application/vnd.osgi.dp" => Application :: VndOsgiDp , "application/vnd.osgi.subsystem" => Application :: VndOsgiSubsystem , "application/vnd.otps.ct-kip+xml" => Application :: VndOtpsCtKipXml , "application/vnd.oxli.countgraph" => Application :: VndOxliCountgraph , "application/vnd.pagerduty+json" => Application :: VndPagerdutyJson , "application/vnd.palm" => Application :: VndPalm , "application/vnd.panoply" => Application :: VndPanoply , "application/vnd.paos.xml" => Application :: VndPaosXml , "application/vnd.patentdive" => Application :: VndPatentdive , "application/vnd.patientecommsdoc" => Application :: VndPatientecommsdoc , "application/vnd.pawaafile" => Application :: VndPawaafile , "application/vnd.pcos" => Application :: VndPcos , "application/vnd.pg.format" => Application :: VndPgFormat , "application/vnd.pg.osasli" => Application :: VndPgOsasli , "application/vnd.piaccess.application-licence" => Application :: VndPiaccessApplicationLicence , "application/vnd.picsel" => Application :: VndPicsel , "application/vnd.pmi.widget" => Application :: VndPmiWidget , "application/vnd.poc.group-advertisement+xml" => Application :: VndPocGroupAdvertisementXml , "application/vnd.pocketlearn" => Application :: VndPocketlearn , "application/vnd.powerbuilder6" => Application :: VndPowerbuilder6 , "application/vnd.powerbuilder6-s" => Application :: VndPowerbuilder6S , "application/vnd.powerbuilder7" => Application :: VndPowerbuilder7 , "application/vnd.powerbuilder75" => Application :: VndPowerbuilder75 , "application/vnd.powerbuilder75-s" => Application :: VndPowerbuilder75S , "application/vnd.powerbuilder7-s" => Application :: VndPowerbuilder7S , "application/vnd.preminet" => Application :: VndPreminet , "application/vnd.previewsystems.box" => Application :: VndPreviewsystemsBox , "application/vnd.proteus.magazine" => Application :: VndProteusMagazine , "application/vnd.psfs" => Application :: VndPsfs , "application/vnd.publishare-delta-tree" => Application :: VndPublishareDeltaTree , "application/vnd.pvi.ptid1" => Application :: VndPviPtid1 , "application/vnd.pwg-multiplexed" => Application :: VndPwgMultiplexed , "application/vnd.pwg-xhtml-print+xml" => Application :: VndPwgXhtmlPrintXml , "application/vnd.qualcomm.brew-app-res" => Application :: VndQualcommBrewAppRes , "application/vnd.quarantainenet" => Application :: VndQuarantainenet , "application/vnd.Quark.QuarkXPress" => Application :: VndQuarkQuarkXPress , "application/vnd.quobject-quoxdocument" => Application :: VndQuobjectQuoxdocument , "application/vnd.radisys.moml+xml" => Application :: VndRadisysMomlXml , "application/vnd.radisys.msml-audit-conf+xml" => Application :: VndRadisysMsmlAuditConfXml , "application/vnd.radisys.msml-audit-conn+xml" => Application :: VndRadisysMsmlAuditConnXml , "application/vnd.radisys.msml-audit-dialog+xml" => Application :: VndRadisysMsmlAuditDialogXml , "application/vnd.radisys.msml-audit-stream+xml" => Application :: VndRadisysMsmlAuditStreamXml , "application/vnd.radisys.msml-audit+xml" => Application :: VndRadisysMsmlAuditXml , "application/vnd.radisys.msml-conf+xml" => Application :: VndRadisysMsmlConfXml , "application/vnd.radisys.msml-dialog-base+xml" => Application :: VndRadisysMsmlDialogBaseXml , "application/vnd.radisys.msml-dialog-fax-detect+xml" => Application :: VndRadisysMsmlDialogFaxDetectXml , "application/vnd.radisys.msml-dialog-fax-sendrecv+xml" => Application :: VndRadisysMsmlDialogFaxSendrecvXml , "application/vnd.radisys.msml-dialog-group+xml" => Application :: VndRadisysMsmlDialogGroupXml , "application/vnd.radisys.msml-dialog-speech+xml" => Application :: VndRadisysMsmlDialogSpeechXml , "application/vnd.radisys.msml-dialog-transform+xml" => Application :: VndRadisysMsmlDialogTransformXml , "application/vnd.radisys.msml-dialog+xml" => Application :: VndRadisysMsmlDialogXml , "application/vnd.radisys.msml+xml" => Application :: VndRadisysMsmlXml , "application/vnd.rainstor.data" => Application :: VndRainstorData , "application/vnd.rapid" => Application :: VndRapid , "application/vnd.rar" => Application :: VndRar , "application/vnd.realvnc.bed" => Application :: VndRealvncBed , "application/vnd.recordare.musicxml" => Application :: VndRecordareMusicxml , "application/vnd.recordare.musicxml+xml" => Application :: VndRecordareMusicxmlXml , "application/vnd.RenLearn.rlprint" => Application :: VndRenLearnRlprint , "application/vnd.resilient.logic" => Application :: VndResilientLogic , "application/vnd.restful+json" => Application :: VndRestfulJson , "application/vnd.rig.cryptonote" => Application :: VndRigCryptonote , "application/vnd.route66.link66+xml" => Application :: VndRoute66Link66Xml , "application/vnd.rs-274x" => Application :: VndRs274X , "application/vnd.ruckus.download" => Application :: VndRuckusDownload , "application/vnd.s3sms" => Application :: VndS3Sms , "application/vnd.sailingtracker.track" => Application :: VndSailingtrackerTrack , "application/vnd.sar" => Application :: VndSar , "application/vnd.sbm.cid" => Application :: VndSbmCid , "application/vnd.sbm.mid2" => Application :: VndSbmMid2 , "application/vnd.scribus" => Application :: VndScribus , "application/vnd.sealed.3df" => Application :: VndSealed3Df , "application/vnd.sealed.csf" => Application :: VndSealedCsf , "application/vnd.sealed.doc" => Application :: VndSealedDoc , "application/vnd.sealed.eml" => Application :: VndSealedEml , "application/vnd.sealed.mht" => Application :: VndSealedMht , "application/vnd.sealed.net" => Application :: VndSealedNet , "application/vnd.sealed.ppt" => Application :: VndSealedPpt , "application/vnd.sealed.tiff" => Application :: VndSealedTiff , "application/vnd.sealed.xls" => Application :: VndSealedXls , "application/vnd.sealedmedia.softseal.html" => Application :: VndSealedmediaSoftsealHtml , "application/vnd.sealedmedia.softseal.pdf" => Application :: VndSealedmediaSoftsealPdf , "application/vnd.seemail" => Application :: VndSeemail , "application/vnd.seis+json" => Application :: VndSeisJson , "application/vnd.sema" => Application :: VndSema , "application/vnd.semd" => Application :: VndSemd , "application/vnd.semf" => Application :: VndSemf , "application/vnd.shade-save-file" => Application :: VndShadeSaveFile , "application/vnd.shana.informed.formdata" => Application :: VndShanaInformedFormdata , "application/vnd.shana.informed.formtemplate" => Application :: VndShanaInformedFormtemplate , "application/vnd.shana.informed.interchange" => Application :: VndShanaInformedInterchange , "application/vnd.shana.informed.package" => Application :: VndShanaInformedPackage , "application/vnd.shootproof+json" => Application :: VndShootproofJson , "application/vnd.shopkick+json" => Application :: VndShopkickJson , "application/vnd.shp" => Application :: VndShp , "application/vnd.shx" => Application :: VndShx , "application/vnd.sigrok.session" => Application :: VndSigrokSession , "application/vnd.SimTech-MindMapper" => Application :: VndSimTechMindMapper , "application/vnd.siren+json" => Application :: VndSirenJson , "application/vnd.smaf" => Application :: VndSmaf , "application/vnd.smart.notebook" => Application :: VndSmartNotebook , "application/vnd.smart.teacher" => Application :: VndSmartTeacher , "application/vnd.snesdev-page-table" => Application :: VndSnesdevPageTable , "application/vnd.software602.filler.form+xml" => Application :: VndSoftware602FillerFormXml , "application/vnd.software602.filler.form-xml-zip" => Application :: VndSoftware602FillerFormXmlZip , "application/vnd.solent.sdkm+xml" => Application :: VndSolentSdkmXml , "application/vnd.spotfire.dxp" => Application :: VndSpotfireDxp , "application/vnd.spotfire.sfs" => Application :: VndSpotfireSfs , "application/vnd.sqlite3" => Application :: VndSqlite3 , "application/vnd.sss-cod" => Application :: VndSssCod , "application/vnd.sss-dtf" => Application :: VndSssDtf , "application/vnd.sss-ntf" => Application :: VndSssNtf , "application/vnd.stepmania.package" => Application :: VndStepmaniaPackage , "application/vnd.stepmania.stepchart" => Application :: VndStepmaniaStepchart , "application/vnd.street-stream" => Application :: VndStreetStream , "application/vnd.sun.wadl+xml" => Application :: VndSunWadlXml , "application/vnd.sus-calendar" => Application :: VndSusCalendar , "application/vnd.svd" => Application :: VndSvd , "application/vnd.swiftview-ics" => Application :: VndSwiftviewIcs , "application/vnd.sybyl.mol2" => Application :: VndSybylMol2 , "application/vnd.sycle+xml" => Application :: VndSycleXml , "application/vnd.syft+json" => Application :: VndSyftJson , "application/vnd.syncml.dm.notification" => Application :: VndSyncmlDmNotification , "application/vnd.syncml.dmddf+xml" => Application :: VndSyncmlDmddfXml , "application/vnd.syncml.dmtnds+wbxml" => Application :: VndSyncmlDmtndsWbxml , "application/vnd.syncml.dmtnds+xml" => Application :: VndSyncmlDmtndsXml , "application/vnd.syncml.dmddf+wbxml" => Application :: VndSyncmlDmddfWbxml , "application/vnd.syncml.dm+wbxml" => Application :: VndSyncmlDmWbxml , "application/vnd.syncml.dm+xml" => Application :: VndSyncmlDmXml , "application/vnd.syncml.ds.notification" => Application :: VndSyncmlDsNotification , "application/vnd.syncml+xml" => Application :: VndSyncmlXml , "application/vnd.tableschema+json" => Application :: VndTableschemaJson , "application/vnd.tao.intent-module-archive" => Application :: VndTaoIntentModuleArchive , "application/vnd.tcpdump.pcap" => Application :: VndTcpdumpPcap , "application/vnd.think-cell.ppttc+json" => Application :: VndThinkCellPpttcJson , "application/vnd.tml" => Application :: VndTml , "application/vnd.tmd.mediaflex.api+xml" => Application :: VndTmdMediaflexApiXml , "application/vnd.tmobile-livetv" => Application :: VndTmobileLivetv , "application/vnd.tri.onesource" => Application :: VndTriOnesource , "application/vnd.trid.tpt" => Application :: VndTridTpt , "application/vnd.triscape.mxs" => Application :: VndTriscapeMxs , "application/vnd.trueapp" => Application :: VndTrueapp , "application/vnd.truedoc" => Application :: VndTruedoc , "application/vnd.ubisoft.webplayer" => Application :: VndUbisoftWebplayer , "application/vnd.ufdl" => Application :: VndUfdl , "application/vnd.uiq.theme" => Application :: VndUiqTheme , "application/vnd.umajin" => Application :: VndUmajin , "application/vnd.unity" => Application :: VndUnity , "application/vnd.uoml+xml" => Application :: VndUomlXml , "application/vnd.uplanet.alert" => Application :: VndUplanetAlert , "application/vnd.uplanet.alert-wbxml" => Application :: VndUplanetAlertWbxml , "application/vnd.uplanet.bearer-choice" => Application :: VndUplanetBearerChoice , "application/vnd.uplanet.bearer-choice-wbxml" => Application :: VndUplanetBearerChoiceWbxml , "application/vnd.uplanet.cacheop" => Application :: VndUplanetCacheop , "application/vnd.uplanet.cacheop-wbxml" => Application :: VndUplanetCacheopWbxml , "application/vnd.uplanet.channel" => Application :: VndUplanetChannel , "application/vnd.uplanet.channel-wbxml" => Application :: VndUplanetChannelWbxml , "application/vnd.uplanet.list" => Application :: VndUplanetList , "application/vnd.uplanet.listcmd" => Application :: VndUplanetListcmd , "application/vnd.uplanet.listcmd-wbxml" => Application :: VndUplanetListcmdWbxml , "application/vnd.uplanet.list-wbxml" => Application :: VndUplanetListWbxml , "application/vnd.uri-map" => Application :: VndUriMap , "application/vnd.uplanet.signal" => Application :: VndUplanetSignal , "application/vnd.valve.source.material" => Application :: VndValveSourceMaterial , "application/vnd.vcx" => Application :: VndVcx , "application/vnd.vd-study" => Application :: VndVdStudy , "application/vnd.vectorworks" => Application :: VndVectorworks , "application/vnd.vel+json" => Application :: VndVelJson , "application/vnd.verimatrix.vcas" => Application :: VndVerimatrixVcas , "application/vnd.veritone.aion+json" => Application :: VndVeritoneAionJson , "application/vnd.veryant.thin" => Application :: VndVeryantThin , "application/vnd.ves.encrypted" => Application :: VndVesEncrypted , "application/vnd.vidsoft.vidconference" => Application :: VndVidsoftVidconference , "application/vnd.visio" => Application :: VndVisio , "application/vnd.visionary" => Application :: VndVisionary , "application/vnd.vividence.scriptfile" => Application :: VndVividenceScriptfile , "application/vnd.vsf" => Application :: VndVsf , "application/vnd.wap.sic" => Application :: VndWapSic , "application/vnd.wap.slc" => Application :: VndWapSlc , "application/vnd.wap.wbxml" => Application :: VndWapWbxml , "application/vnd.wap.wmlc" => Application :: VndWapWmlc , "application/vnd.wap.wmlscriptc" => Application :: VndWapWmlscriptc , "application/vnd.wasmflow.wafl" => Application :: VndWasmflowWafl , "application/vnd.webturbo" => Application :: VndWebturbo , "application/vnd.wfa.dpp" => Application :: VndWfaDpp , "application/vnd.wfa.p2p" => Application :: VndWfaP2P , "application/vnd.wfa.wsc" => Application :: VndWfaWsc , "application/vnd.windows.devicepairing" => Application :: VndWindowsDevicepairing , "application/vnd.wmc" => Application :: VndWmc , "application/vnd.wmf.bootstrap" => Application :: VndWmfBootstrap , "application/vnd.wolfram.mathematica" => Application :: VndWolframMathematica , "application/vnd.wolfram.mathematica.package" => Application :: VndWolframMathematicaPackage , "application/vnd.wolfram.player" => Application :: VndWolframPlayer , "application/vnd.wordperfect" => Application :: VndWordperfect , "application/vnd.wqd" => Application :: VndWqd , "application/vnd.wrq-hp3000-labelled" => Application :: VndWrqHp3000Labelled , "application/vnd.wt.stf" => Application :: VndWtStf , "application/vnd.wv.csp+xml" => Application :: VndWvCspXml , "application/vnd.wv.csp+wbxml" => Application :: VndWvCspWbxml , "application/vnd.wv.ssp+xml" => Application :: VndWvSspXml , "application/vnd.xacml+json" => Application :: VndXacmlJson , "application/vnd.xara" => Application :: VndXara , "application/vnd.xfdl" => Application :: VndXfdl , "application/vnd.xfdl.webform" => Application :: VndXfdlWebform , "application/vnd.xmi+xml" => Application :: VndXmiXml , "application/vnd.xmpie.cpkg" => Application :: VndXmpieCpkg , "application/vnd.xmpie.dpkg" => Application :: VndXmpieDpkg , "application/vnd.xmpie.plan" => Application :: VndXmpiePlan , "application/vnd.xmpie.ppkg" => Application :: VndXmpiePpkg , "application/vnd.xmpie.xlim" => Application :: VndXmpieXlim , "application/vnd.yamaha.hv-dic" => Application :: VndYamahaHvDic , "application/vnd.yamaha.hv-script" => Application :: VndYamahaHvScript , "application/vnd.yamaha.hv-voice" => Application :: VndYamahaHvVoice , "application/vnd.yamaha.openscoreformat.osfpvg+xml" => Application :: VndYamahaOpenscoreformatOsfpvgXml , "application/vnd.yamaha.openscoreformat" => Application :: VndYamahaOpenscoreformat , "application/vnd.yamaha.remote-setup" => Application :: VndYamahaRemoteSetup , "application/vnd.yamaha.smaf-audio" => Application :: VndYamahaSmafAudio , "application/vnd.yamaha.smaf-phrase" => Application :: VndYamahaSmafPhrase , "application/vnd.yamaha.through-ngn" => Application :: VndYamahaThroughNgn , "application/vnd.yamaha.tunnel-udpencap" => Application :: VndYamahaTunnelUdpencap , "application/vnd.yaoweme" => Application :: VndYaoweme , "application/vnd.yellowriver-custom-menu" => Application :: VndYellowriverCustomMenu , "application/vnd.zul" => Application :: VndZul , "application/vnd.zzazz.deck+xml" => Application :: VndZzazzDeckXml , "application/voicexml+xml" => Application :: VoicexmlXml , "application/voucher-cms+json" => Application :: VoucherCmsJson , "application/vq-rtcpxr" => Application :: VqRtcpxr , "application/wasm" => Application :: Wasm , "application/watcherinfo+xml" => Application :: WatcherinfoXml , "application/webpush-options+json" => Application :: WebpushOptionsJson , "application/whoispp-query" => Application :: WhoisppQuery , "application/whoispp-response" => Application :: WhoisppResponse , "application/widget" => Application :: Widget , "application/wita" => Application :: Wita , "application/wordperfect5.1" => Application :: Wordperfect51 , "application/wsdl+xml" => Application :: WsdlXml , "application/wspolicy+xml" => Application :: WspolicyXml , "application/x-pki-message" => Application :: XPkiMessage , "application/x-www-form-urlencoded" => Application :: XWwwFormUrlencoded , "application/x-x509-ca-cert" => Application :: XX509CaCert , "application/x-x509-ca-ra-cert" => Application :: XX509CaRaCert , "application/x-x509-next-ca-cert" => Application :: XX509NextCaCert , "application/x400-bp" => Application :: X400Bp , "application/xacml+xml" => Application :: XacmlXml , "application/xcap-att+xml" => Application :: XcapAttXml , "application/xcap-caps+xml" => Application :: XcapCapsXml , "application/xcap-diff+xml" => Application :: XcapDiffXml , "application/xcap-el+xml" => Application :: XcapElXml , "application/xcap-error+xml" => Application :: XcapErrorXml , "application/xcap-ns+xml" => Application :: XcapNsXml , "application/xcon-conference-info-diff+xml" => Application :: XconConferenceInfoDiffXml , "application/xcon-conference-info+xml" => Application :: XconConferenceInfoXml , "application/xenc+xml" => Application :: XencXml , "application/xfdf" => Application :: Xfdf , "application/xhtml+xml" => Application :: XhtmlXml , "application/xliff+xml" => Application :: XliffXml , "application/xml" => Application :: Xml , "application/xml-dtd" => Application :: XmlDtd , "application/xml-external-parsed-entity" => Application :: XmlExternalParsedEntity , "application/xml-patch+xml" => Application :: XmlPatchXml , "application/xmpp+xml" => Application :: XmppXml , "application/xop+xml" => Application :: XopXml , "application/xslt+xml" => Application :: XsltXml , "application/xv+xml" => Application :: XvXml , "application/yang" => Application :: Yang , "application/yang-data+cbor" => Application :: YangDataCbor , "application/yang-data+json" => Application :: YangDataJson , "application/yang-data+xml" => Application :: YangDataXml , "application/yang-patch+json" => Application :: YangPatchJson , "application/yang-patch+xml" => Application :: YangPatchXml , "application/yin+xml" => Application :: YinXml , "application/zip" => Application :: Zip , "application/zlib" => Application :: Zlib , "application/zstd" => Application :: Zstd , _ => Application :: Other (input . to_string ()) , }
    }
}
