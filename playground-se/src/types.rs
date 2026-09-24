use std::fmt::Display;

use quick_xml::DeError;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DefinitionParseError {
    #[error("XML parse error [FILE:UNSUPPORT]: {0}")]
    XmlError(String),
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<DeError> for DefinitionParseError {
    fn from(e: DeError) -> Self {
        dbg!(&e);
        DefinitionParseError::XmlError(e.to_string())
    }
}

impl From<quick_xml::Error> for DefinitionParseError {
    fn from(e: quick_xml::Error) -> Self {
        dbg!(&e);
        DefinitionParseError::XmlError(e.to_string())
    }
}

impl From<std::io::Error> for DefinitionParseError {
    fn from(e: std::io::Error) -> Self {
        dbg!(&e);
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

impl Display for MountPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mount_point in &self.mount_points {
            write!(f, "{}\n", mount_point);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountPoint {
    #[serde(rename = "@Side")]
    pub side: String,
    #[serde(rename = "@EndX")]
    pub end_x: f32,
    #[serde(rename = "@EndY")]
    pub end_y: f32,
    #[serde(rename = "@StartX")]
    pub start_x: f32,
    #[serde(rename = "@StartY")]
    pub start_y: f32,
    #[serde(rename = "@Default")]
    pub default: Option<bool>,
}

impl Display for MountPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Side: {}, EndX: {}, EndY: {}, StartX: {}, StartY: {}",
            self.side, self.end_x, self.end_y, self.start_x, self.start_y
        );
        if self.default.unwrap_or(false) {
            write!(f, " (default)");
        }
        Ok(())
    }
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

pub mod inspect {
    use std::collections::{BTreeMap, BTreeSet};

    use quick_xml::Reader;
    use quick_xml::events::Event;

    #[derive(Debug, Default)]
    pub struct NodeStats {
        pub count: usize,                    // Сколько раз встречается путь
        pub sample_values: BTreeSet<String>, // Образцы значений (например, до 5 уникальных)
        pub attributes: BTreeSet<String>,    // Какие атрибуты встречались у этого элемента
    }

    pub fn debug_parse_sbc(content: &str) {
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        let mut schema: BTreeMap<String, NodeStats> = BTreeMap::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) => {
                    let tag_name = e.name().as_ref().to_string();

                    // Регистрируем только если мы уже внутри или заходим в Definition
                    if tag_name == "Definition" || stack.contains(&"Definition".to_string()) {
                        let path = format_path(&stack, &tag_name);
                        let stats = schema.entry(path).or_default();
                        stats.count += 1;

                        {
                            for attr in e.attributes().flatten() {
                                let key = attr.key.as_ref();
                                let val = &attr.value;
                                stats.attributes.insert(format!("@{}={}", key, val));
                            }
                        }
                    }
                }

                // Открывающие теги <Definition>, <Id>, <Components>
                Ok(Event::Start(ref e)) => {
                    let tag_name = e.name().as_ref().to_string();
                    stack.push(tag_name.clone());

                    // Фиксируем путь, только если мы внутри Definition (или это сам Definition)
                    if stack.contains(&"Definition".to_string()) {
                        let path = format_path(&stack[..stack.len() - 1], &tag_name);
                        let stats = schema.entry(path).or_default();
                        stats.count += 1;

                        {
                            for attr in e.attributes().flatten() {
                                let key = attr.key.as_ref();
                                let val = &attr.value;
                                stats.attributes.insert(format!("@{}={}", key, val));
                            }
                        }
                    }
                }

                // Текстовые узлы
                Ok(Event::Text(ref e)) => {
                    if stack.contains(&"Definition".to_string()) {
                        let text = e.trim().to_string();
                        if !text.is_empty() {
                            let path = format!("{}/@text", build_relative_path(&stack));
                            let stats = schema.entry(path).or_default();
                            stats.count += 1;
                            if stats.sample_values.len() < 5 {
                                stats.sample_values.insert(text);
                            }
                        }
                    }
                }

                // Закрывающие теги
                Ok(Event::End(_)) => {
                    stack.pop();
                }

                Ok(Event::Eof) => break println!("[LOG] Completed {}", stack.join("/")),
                Err(e) => {
                    eprintln!(
                        "[ERROR] Ошибка парсинга на позиции {}: {:?}",
                        reader.error_position(),
                        e
                    );
                    eprintln!("Текущий путь (стек): {}", stack.join("/"));
                    break;
                }
                _ => (),
            }
            buf.clear(); // КРИТИЧНО: очищаем буфер, чтобы память не утекала
        }
    }

    // Хелпер: срезает внешнюю обертку (Definitions/CubeBlocks/...) оставляя относительный путь от Definition
    fn build_relative_path(stack: &[String]) -> String {
        if let Some(pos) = stack.iter().position(|s| s == "Definition") {
            stack[pos..].join("/")
        } else {
            stack.join("/")
        }
    }

    fn format_path(parent_stack: &[String], current_tag: &str) -> String {
        let base = build_relative_path(parent_stack);
        if base.is_empty() {
            current_tag.to_string()
        } else {
            format!("{}/{}", base, current_tag)
        }
    }
}
