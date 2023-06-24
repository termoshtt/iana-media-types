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
pub enum Model {
    #[doc = "model/3mf"]
    #[serde(alias = "model/3mf")]
    _3Mf,
    #[doc = "model/e57"]
    #[serde(alias = "model/e57")]
    E57,
    #[doc = "model/example"]
    #[serde(alias = "model/example")]
    Example,
    #[doc = "model/gltf-binary"]
    #[serde(alias = "model/gltf-binary")]
    #[serde(alias = "glb")]
    GltfBinary,
    #[doc = "model/gltf+json"]
    #[serde(alias = "model/gltf+json")]
    #[serde(alias = "gltf")]
    GltfJson,
    #[doc = "model/JT"]
    #[serde(alias = "model/JT")]
    #[serde(alias = "jt")]
    Jt,
    #[doc = "model/iges"]
    #[serde(alias = "model/iges")]
    #[serde(alias = "igs")]
    #[serde(alias = "iges")]
    Iges,
    #[doc = "model/mtl"]
    #[serde(alias = "model/mtl")]
    #[serde(alias = "mtl")]
    Mesh,
    #[doc = "model/obj"]
    #[serde(alias = "model/obj")]
    #[serde(alias = "obj")]
    Mtl,
    #[doc = "model/prc"]
    #[serde(alias = "model/prc")]
    Obj,
    #[doc = "model/step"]
    #[serde(alias = "model/step")]
    Prc,
    #[doc = "model/step+xml"]
    #[serde(alias = "model/step+xml")]
    #[serde(alias = "stpx")]
    Step,
    #[doc = "model/step+zip"]
    #[serde(alias = "model/step+zip")]
    StepXml,
    #[doc = "model/step-xml+zip"]
    #[serde(alias = "model/step-xml+zip")]
    #[serde(alias = "stpxz")]
    StepZip,
    #[doc = "model/stl"]
    #[serde(alias = "model/stl")]
    #[serde(alias = "stl")]
    StepXmlZip,
    #[doc = "model/u3d"]
    #[serde(alias = "model/u3d")]
    #[serde(alias = "u3d")]
    Stl,
    #[doc = "model/vnd.bary"]
    #[serde(alias = "model/vnd.bary")]
    #[serde(alias = "bary")]
    U3D,
    #[doc = "model/vnd.cld"]
    #[serde(alias = "model/vnd.cld")]
    #[serde(alias = "cld")]
    VndBary,
    #[doc = "model/vnd.collada+xml"]
    #[serde(alias = "model/vnd.collada+xml")]
    #[serde(alias = "dae")]
    VndCld,
    #[doc = "model/vnd.dwf"]
    #[serde(alias = "model/vnd.dwf")]
    #[serde(alias = "dwf")]
    VndColladaXml,
    #[doc = "model/vnd.flatland.3dml"]
    #[serde(alias = "model/vnd.flatland.3dml")]
    VndDwf,
    #[doc = "model/vnd.gdl"]
    #[serde(alias = "model/vnd.gdl")]
    #[serde(alias = "gdl")]
    #[serde(alias = "gsm")]
    #[serde(alias = "win")]
    #[serde(alias = "dor")]
    #[serde(alias = "lmp")]
    #[serde(alias = "rsm")]
    #[serde(alias = "msm")]
    #[serde(alias = "ism")]
    VndFlatland3Dml,
    #[doc = "model/vnd.gs-gdl"]
    #[serde(alias = "model/vnd.gs-gdl")]
    VndGdl,
    #[doc = "model/vnd.gtw"]
    #[serde(alias = "model/vnd.gtw")]
    #[serde(alias = "gtw")]
    VndGsGdl,
    #[doc = "model/vnd.moml+xml"]
    #[serde(alias = "model/vnd.moml+xml")]
    #[serde(alias = "moml")]
    VndGtw,
    #[doc = "model/vnd.mts"]
    #[serde(alias = "model/vnd.mts")]
    #[serde(alias = "mts")]
    VndMomlXml,
    #[doc = "model/vnd.opengex"]
    #[serde(alias = "model/vnd.opengex")]
    #[serde(alias = "ogex")]
    VndMts,
    #[doc = "model/vnd.parasolid.transmit.binary"]
    #[serde(alias = "model/vnd.parasolid.transmit.binary")]
    #[serde(alias = "x_b")]
    #[serde(alias = "xmt_bin")]
    VndOpengex,
    #[doc = "model/vnd.parasolid.transmit.text"]
    #[serde(alias = "model/vnd.parasolid.transmit.text")]
    #[serde(alias = "x_t")]
    #[serde(alias = "xmt_txt")]
    VndParasolidTransmitBinary,
    #[doc = "model/vnd.pytha.pyox"]
    #[serde(alias = "model/vnd.pytha.pyox")]
    #[serde(alias = "pyo")]
    #[serde(alias = "pyox")]
    VndParasolidTransmitText,
    #[doc = "model/vnd.rosette.annotated-data-model"]
    #[serde(alias = "model/vnd.rosette.annotated-data-model")]
    VndPythaPyox,
    #[doc = "model/vnd.sap.vds"]
    #[serde(alias = "model/vnd.sap.vds")]
    #[serde(alias = "vds")]
    VndRosetteAnnotatedDataModel,
    #[doc = "model/vnd.usda"]
    #[serde(alias = "model/vnd.usda")]
    #[serde(alias = "usda")]
    VndSapVds,
    #[doc = "model/vnd.usdz+zip"]
    #[serde(alias = "model/vnd.usdz+zip")]
    #[serde(alias = "usdz")]
    VndUsda,
    #[doc = "model/vnd.valve.source.compiled-map"]
    #[serde(alias = "model/vnd.valve.source.compiled-map")]
    #[serde(alias = "bsp")]
    VndUsdzZip,
    #[doc = "model/vnd.vtu"]
    #[serde(alias = "model/vnd.vtu")]
    #[serde(alias = "vtu")]
    VndValveSourceCompiledMap,
    #[doc = "model/x3d-vrml"]
    #[serde(alias = "model/x3d-vrml")]
    #[serde(alias = "x3dv")]
    #[serde(alias = "x3dvz")]
    VndVtu,
    #[doc = "model/x3d+fastinfoset"]
    #[serde(alias = "model/x3d+fastinfoset")]
    Vrml,
    #[doc = "model/x3d+xml"]
    #[serde(alias = "model/x3d+xml")]
    #[serde(alias = "x3db")]
    X3DVrml,
}
impl ::std::fmt::Display for Model {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Model::_3Mf => write!(f, "model/3mf")?,
            Model::E57 => write!(f, "model/e57")?,
            Model::Example => write!(f, "model/example")?,
            Model::GltfBinary => write!(f, "model/gltf-binary")?,
            Model::GltfJson => write!(f, "model/gltf+json")?,
            Model::Jt => write!(f, "model/JT")?,
            Model::Iges => write!(f, "model/iges")?,
            Model::Mesh => write!(f, "model/mtl")?,
            Model::Mtl => write!(f, "model/obj")?,
            Model::Obj => write!(f, "model/prc")?,
            Model::Prc => write!(f, "model/step")?,
            Model::Step => write!(f, "model/step+xml")?,
            Model::StepXml => write!(f, "model/step+zip")?,
            Model::StepZip => write!(f, "model/step-xml+zip")?,
            Model::StepXmlZip => write!(f, "model/stl")?,
            Model::Stl => write!(f, "model/u3d")?,
            Model::U3D => write!(f, "model/vnd.bary")?,
            Model::VndBary => write!(f, "model/vnd.cld")?,
            Model::VndCld => write!(f, "model/vnd.collada+xml")?,
            Model::VndColladaXml => write!(f, "model/vnd.dwf")?,
            Model::VndDwf => write!(f, "model/vnd.flatland.3dml")?,
            Model::VndFlatland3Dml => write!(f, "model/vnd.gdl")?,
            Model::VndGdl => write!(f, "model/vnd.gs-gdl")?,
            Model::VndGsGdl => write!(f, "model/vnd.gtw")?,
            Model::VndGtw => write!(f, "model/vnd.moml+xml")?,
            Model::VndMomlXml => write!(f, "model/vnd.mts")?,
            Model::VndMts => write!(f, "model/vnd.opengex")?,
            Model::VndOpengex => write!(f, "model/vnd.parasolid.transmit.binary")?,
            Model::VndParasolidTransmitBinary => write!(f, "model/vnd.parasolid.transmit.text")?,
            Model::VndParasolidTransmitText => write!(f, "model/vnd.pytha.pyox")?,
            Model::VndPythaPyox => write!(f, "model/vnd.rosette.annotated-data-model")?,
            Model::VndRosetteAnnotatedDataModel => write!(f, "model/vnd.sap.vds")?,
            Model::VndSapVds => write!(f, "model/vnd.usda")?,
            Model::VndUsda => write!(f, "model/vnd.usdz+zip")?,
            Model::VndUsdzZip => write!(f, "model/vnd.valve.source.compiled-map")?,
            Model::VndValveSourceCompiledMap => write!(f, "model/vnd.vtu")?,
            Model::VndVtu => write!(f, "model/x3d-vrml")?,
            Model::Vrml => write!(f, "model/x3d+fastinfoset")?,
            Model::X3DVrml => write!(f, "model/x3d+xml")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Model {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "model/3mf" => Ok(Model::_3Mf),
            "model/e57" => Ok(Model::E57),
            "model/example" => Ok(Model::Example),
            "model/gltf-binary" => Ok(Model::GltfBinary),
            "model/gltf+json" => Ok(Model::GltfJson),
            "model/JT" => Ok(Model::Jt),
            "model/iges" => Ok(Model::Iges),
            "model/mtl" => Ok(Model::Mesh),
            "model/obj" => Ok(Model::Mtl),
            "model/prc" => Ok(Model::Obj),
            "model/step" => Ok(Model::Prc),
            "model/step+xml" => Ok(Model::Step),
            "model/step+zip" => Ok(Model::StepXml),
            "model/step-xml+zip" => Ok(Model::StepZip),
            "model/stl" => Ok(Model::StepXmlZip),
            "model/u3d" => Ok(Model::Stl),
            "model/vnd.bary" => Ok(Model::U3D),
            "model/vnd.cld" => Ok(Model::VndBary),
            "model/vnd.collada+xml" => Ok(Model::VndCld),
            "model/vnd.dwf" => Ok(Model::VndColladaXml),
            "model/vnd.flatland.3dml" => Ok(Model::VndDwf),
            "model/vnd.gdl" => Ok(Model::VndFlatland3Dml),
            "model/vnd.gs-gdl" => Ok(Model::VndGdl),
            "model/vnd.gtw" => Ok(Model::VndGsGdl),
            "model/vnd.moml+xml" => Ok(Model::VndGtw),
            "model/vnd.mts" => Ok(Model::VndMomlXml),
            "model/vnd.opengex" => Ok(Model::VndMts),
            "model/vnd.parasolid.transmit.binary" => Ok(Model::VndOpengex),
            "model/vnd.parasolid.transmit.text" => Ok(Model::VndParasolidTransmitBinary),
            "model/vnd.pytha.pyox" => Ok(Model::VndParasolidTransmitText),
            "model/vnd.rosette.annotated-data-model" => Ok(Model::VndPythaPyox),
            "model/vnd.sap.vds" => Ok(Model::VndRosetteAnnotatedDataModel),
            "model/vnd.usda" => Ok(Model::VndSapVds),
            "model/vnd.usdz+zip" => Ok(Model::VndUsda),
            "model/vnd.valve.source.compiled-map" => Ok(Model::VndUsdzZip),
            "model/vnd.vtu" => Ok(Model::VndValveSourceCompiledMap),
            "model/x3d-vrml" => Ok(Model::VndVtu),
            "model/x3d+fastinfoset" => Ok(Model::Vrml),
            "model/x3d+xml" => Ok(Model::X3DVrml),
            _ => Err(()),
        }
    }
}
