#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Model {
    #[doc = "model/3mf"]
    _3Mf,
    #[doc = "model/e57"]
    E57,
    #[doc = "model/example"]
    Example,
    #[doc = "model/gltf-binary"]
    GltfBinary,
    #[doc = "model/gltf+json"]
    GltfJson,
    #[doc = "model/iges"]
    Iges,
    #[doc = "model/mtl"]
    Mtl,
    #[doc = "model/obj"]
    Obj,
    #[doc = "model/prc"]
    Prc,
    #[doc = "model/step"]
    Step,
    #[doc = "model/step+xml"]
    StepXml,
    #[doc = "model/step+zip"]
    StepZip,
    #[doc = "model/step-xml+zip"]
    StepXmlZip,
    #[doc = "model/stl"]
    Stl,
    #[doc = "model/u3d"]
    U3D,
    #[doc = "model/vnd.collada+xml"]
    VndColladaXml,
    #[doc = "model/vnd.dwf"]
    VndDwf,
    #[doc = "model/vnd.flatland.3dml"]
    VndFlatland3Dml,
    #[doc = "model/vnd.gdl"]
    VndGdl,
    #[doc = "model/vnd.gs-gdl"]
    VndGsGdl,
    #[doc = "model/vnd.gtw"]
    VndGtw,
    #[doc = "model/vnd.moml+xml"]
    VndMomlXml,
    #[doc = "model/vnd.mts"]
    VndMts,
    #[doc = "model/vnd.opengex"]
    VndOpengex,
    #[doc = "model/vnd.parasolid.transmit.binary"]
    VndParasolidTransmitBinary,
    #[doc = "model/vnd.parasolid.transmit.text"]
    VndParasolidTransmitText,
    #[doc = "model/vnd.pytha.pyox"]
    VndPythaPyox,
    #[doc = "model/vnd.rosette.annotated-data-model"]
    VndRosetteAnnotatedDataModel,
    #[doc = "model/vnd.sap.vds"]
    VndSapVds,
    #[doc = "model/vnd.usda"]
    VndUsda,
    #[doc = "model/vnd.usdz+zip"]
    VndUsdzZip,
    #[doc = "model/vnd.valve.source.compiled-map"]
    VndValveSourceCompiledMap,
    #[doc = "model/vnd.vtu"]
    VndVtu,
    #[doc = "model/x3d-vrml"]
    X3DVrml,
    #[doc = "model/x3d+fastinfoset"]
    X3DFastinfoset,
    #[doc = "model/x3d+xml"]
    X3DXml,
}
impl ::std::fmt::Display for Model {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Model::_3Mf => write!(f, "model/3mf")?,
            Model::E57 => write!(f, "model/e57")?,
            Model::Example => write!(f, "model/example")?,
            Model::GltfBinary => write!(f, "model/gltf-binary")?,
            Model::GltfJson => write!(f, "model/gltf+json")?,
            Model::Iges => write!(f, "model/iges")?,
            Model::Mtl => write!(f, "model/mtl")?,
            Model::Obj => write!(f, "model/obj")?,
            Model::Prc => write!(f, "model/prc")?,
            Model::Step => write!(f, "model/step")?,
            Model::StepXml => write!(f, "model/step+xml")?,
            Model::StepZip => write!(f, "model/step+zip")?,
            Model::StepXmlZip => write!(f, "model/step-xml+zip")?,
            Model::Stl => write!(f, "model/stl")?,
            Model::U3D => write!(f, "model/u3d")?,
            Model::VndColladaXml => write!(f, "model/vnd.collada+xml")?,
            Model::VndDwf => write!(f, "model/vnd.dwf")?,
            Model::VndFlatland3Dml => write!(f, "model/vnd.flatland.3dml")?,
            Model::VndGdl => write!(f, "model/vnd.gdl")?,
            Model::VndGsGdl => write!(f, "model/vnd.gs-gdl")?,
            Model::VndGtw => write!(f, "model/vnd.gtw")?,
            Model::VndMomlXml => write!(f, "model/vnd.moml+xml")?,
            Model::VndMts => write!(f, "model/vnd.mts")?,
            Model::VndOpengex => write!(f, "model/vnd.opengex")?,
            Model::VndParasolidTransmitBinary => write!(f, "model/vnd.parasolid.transmit.binary")?,
            Model::VndParasolidTransmitText => write!(f, "model/vnd.parasolid.transmit.text")?,
            Model::VndPythaPyox => write!(f, "model/vnd.pytha.pyox")?,
            Model::VndRosetteAnnotatedDataModel => {
                write!(f, "model/vnd.rosette.annotated-data-model")?
            }
            Model::VndSapVds => write!(f, "model/vnd.sap.vds")?,
            Model::VndUsda => write!(f, "model/vnd.usda")?,
            Model::VndUsdzZip => write!(f, "model/vnd.usdz+zip")?,
            Model::VndValveSourceCompiledMap => write!(f, "model/vnd.valve.source.compiled-map")?,
            Model::VndVtu => write!(f, "model/vnd.vtu")?,
            Model::X3DVrml => write!(f, "model/x3d-vrml")?,
            Model::X3DFastinfoset => write!(f, "model/x3d+fastinfoset")?,
            Model::X3DXml => write!(f, "model/x3d+xml")?,
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
            "model/iges" => Ok(Model::Iges),
            "model/mtl" => Ok(Model::Mtl),
            "model/obj" => Ok(Model::Obj),
            "model/prc" => Ok(Model::Prc),
            "model/step" => Ok(Model::Step),
            "model/step+xml" => Ok(Model::StepXml),
            "model/step+zip" => Ok(Model::StepZip),
            "model/step-xml+zip" => Ok(Model::StepXmlZip),
            "model/stl" => Ok(Model::Stl),
            "model/u3d" => Ok(Model::U3D),
            "model/vnd.collada+xml" => Ok(Model::VndColladaXml),
            "model/vnd.dwf" => Ok(Model::VndDwf),
            "model/vnd.flatland.3dml" => Ok(Model::VndFlatland3Dml),
            "model/vnd.gdl" => Ok(Model::VndGdl),
            "model/vnd.gs-gdl" => Ok(Model::VndGsGdl),
            "model/vnd.gtw" => Ok(Model::VndGtw),
            "model/vnd.moml+xml" => Ok(Model::VndMomlXml),
            "model/vnd.mts" => Ok(Model::VndMts),
            "model/vnd.opengex" => Ok(Model::VndOpengex),
            "model/vnd.parasolid.transmit.binary" => Ok(Model::VndParasolidTransmitBinary),
            "model/vnd.parasolid.transmit.text" => Ok(Model::VndParasolidTransmitText),
            "model/vnd.pytha.pyox" => Ok(Model::VndPythaPyox),
            "model/vnd.rosette.annotated-data-model" => Ok(Model::VndRosetteAnnotatedDataModel),
            "model/vnd.sap.vds" => Ok(Model::VndSapVds),
            "model/vnd.usda" => Ok(Model::VndUsda),
            "model/vnd.usdz+zip" => Ok(Model::VndUsdzZip),
            "model/vnd.valve.source.compiled-map" => Ok(Model::VndValveSourceCompiledMap),
            "model/vnd.vtu" => Ok(Model::VndVtu),
            "model/x3d-vrml" => Ok(Model::X3DVrml),
            "model/x3d+fastinfoset" => Ok(Model::X3DFastinfoset),
            "model/x3d+xml" => Ok(Model::X3DXml),
            _ => Err(()),
        }
    }
}
