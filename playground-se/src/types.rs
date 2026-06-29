use quick_xml::DeError;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DefinitionParseError {
    #[error("XML parse error [FILE:NONE]: {0}")]
    XmlError(String),
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<DeError> for DefinitionParseError {
    fn from(e: DeError) -> Self {
        DefinitionParseError::XmlError(e.to_string())
    }
}

impl From<quick_xml::Error> for DefinitionParseError {
    fn from(e: quick_xml::Error) -> Self {
        DefinitionParseError::XmlError(e.to_string())
    }
}

impl From<std::io::Error> for DefinitionParseError {
    fn from(e: std::io::Error) -> Self {
        DefinitionParseError::IoError(e.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    #[serde(rename = "CubeBlocks")]
    pub cube_blocks: CubeBlocks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CubeBlocks {
    #[serde(rename = "Definition", default)]
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    #[serde(rename = "Id")]
    pub id: BlockId,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "CubeSize")]
    pub cube_size: String,
    #[serde(rename = "BlockTopology")]
    pub block_topology: String,
    #[serde(rename = "Size")]
    pub size: Size,
    /// how much space the model can occupy (piston, hangar door, etc.)
    #[serde(rename = "ModelOffset")]
    pub model_offset: Coords,
    #[serde(rename = "CubeDefinition")]
    pub cube_definition: Option<CubeDefinition>,
    #[serde(rename = "BuildProgressModels")]
    pub build_progress_models: Option<BuildProgressModels>,
    #[serde(rename = "MountPoints")]
    pub mount_points: Option<MountPoints>,
    #[serde(rename = "Components")]
    pub components: Option<Components>,
    #[serde(rename = "CriticalComponent")]
    pub critical_component: Option<CriticalComponent>,
    #[serde(rename = "Skeleton")]
    pub skeleton: Option<Skeleton>,
    #[serde(rename = "BlockPairName")]
    pub block_pair_name: String,
    #[serde(rename = "PhysicsOption")]
    pub physics_option: Option<String>,
    #[serde(rename = "EdgeType")]
    pub edge_type: String,
    #[serde(rename = "BuildTimeSeconds")]
    pub build_time_seconds: f32,
    #[serde(rename = "DisassembleRatio")]
    pub disassemble_ratio: Option<f32>,
    #[serde(rename = "NavigationDefinition")]
    pub navigation_definition: Option<String>,
    #[serde(rename = "MirroringX")]
    pub mirroring_x: Option<String>,
    #[serde(rename = "MirroringY")]
    pub mirroring_y: Option<String>,
    #[serde(rename = "MirroringZ")]
    pub mirroring_z: Option<String>,
    #[serde(rename = "PCUConsole")]
    pub pcu_console: Option<i32>,
    #[serde(rename = "Public")]
    pub public: Option<bool>,
    #[serde(rename = "GuiVisible")]
    pub gui_visible: Option<bool>,
    #[serde(rename = "SilenceableByShipSoundSystem")]
    pub silenceable_by_ship_sound_system: Option<bool>,
    #[serde(rename = "DeformationRatio")]
    pub deformation_ratio: Option<f32>,
    #[serde(rename = "InventoryMaxVolume")]
    pub inventory_max_volume: Option<f32>,
    #[serde(rename = "UseModelIntersection")]
    pub use_model_intersection: Option<bool>,
    #[serde(rename = "GeneralDamageMultiplier")]
    pub general_damage_multiplier: Option<f32>,
    #[serde(rename = "Model")]
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockId {
    #[serde(rename = "TypeId")]
    pub type_id: String,
    #[serde(rename = "SubtypeId")]
    pub subtype_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    #[serde(rename = "@x")]
    pub x: i32,
    #[serde(rename = "@y")]
    pub y: i32,
    #[serde(rename = "@z")]
    pub z: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coords {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
    #[serde(rename = "@z")]
    pub z: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CubeDefinition {
    #[serde(rename = "CubeTopology")]
    pub cube_topology: String,
    #[serde(rename = "ShowEdges")]
    pub show_edges: bool,
    #[serde(rename = "Sides")]
    pub sides: Sides,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sides {
    #[serde(rename = "Side", default)]
    pub sides: Vec<Side>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Side {
    #[serde(rename = "@Model")]
    pub model: String,
    #[serde(rename = "@PatternHeight")]
    pub pattern_height: i32,
    #[serde(rename = "@PatternWidth")]
    pub pattern_width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildProgressModels {
    #[serde(rename = "Model", default)]
    pub models: Vec<BuildProgressModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildProgressModel {
    #[serde(rename = "@BuildPercentUpperBound")]
    pub build_percent_upper_bound: f32,
    #[serde(rename = "@File")]
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountPoints {
    #[serde(rename = "MountPoint", default)]
    pub mount_points: Vec<MountPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountPoint {
    #[serde(rename = "@EndX")]
    pub end_x: f32,
    #[serde(rename = "@EndY")]
    pub end_y: f32,
    #[serde(rename = "@Side")]
    pub side: String,
    #[serde(rename = "@StartX")]
    pub start_x: f32,
    #[serde(rename = "@StartY")]
    pub start_y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Components {
    #[serde(rename = "Component", default)]
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "@Count")]
    pub count: i32,
    #[serde(rename = "@Subtype")]
    pub subtype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CriticalComponent {
    #[serde(rename = "@Index")]
    pub index: i32,
    #[serde(rename = "@Subtype")]
    pub subtype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skeleton {
    #[serde(rename = "BoneInfo", default)]
    pub bone_infos: Vec<BoneInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoneInfo {
    #[serde(rename = "BonePosition")]
    pub bone_position: Coords,
    #[serde(rename = "BoneOffset")]
    pub bone_offset: Coords,
}

pub fn parse_sbc(content: &str) -> Result<Definitions, DefinitionParseError> {
    let definitions: Definitions = from_str(content)?;
    Ok(definitions)
}
