use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaceGroupType {
    EarR = 0,
    EarL = 1,
    EyeR = 2,
    EyeL = 3,
    EyebrowR = 4,
    EyebrowL = 5,
    Mouth = 6,
    Max = 7,
}

pub const EYEBROW_TYPES: &[&str] = &[
    "Base",
    "WaraiA",
    "WaraiB",
    "WaraiC",
    "WaraiD",
    "IkariA",
    "KanasiA",
    "DoyaA",
    "DereA",
    "OdorokiA",
    "OdorokiB",
    "JitoA",
    "KomariA",
    "KusyoA",
    "UreiA",
    "RunA",
    "RunB",
    "SeriousA",
    "SeriousB",
    "ShiwaA",
    "ShiwaB",
    "Offset_U",
    "Offset_D",
    "Offset_L",
    "Offset_R",
];

pub const EYE_TYPES: &[&str] = &[
    "Base",
    "HalfA",
    "CloseA",
    "HalfB",
    "HalfC",
    "WaraiA",
    "WaraiB",
    "WaraiC",
    "WaraiD",
    "IkariA",
    "KanasiA",
    "DereA",
    "OdorokiA",
    "OdorokiB",
    "OdorokiC",
    "JitoA",
    "KusyoA",
    "UreiA",
    "RunA",
    "DrivenA",
    "XRange",
    "YRange",
    "EyeHideA",
    "SeriousA",
    "PupilA",
    "PupilB",
    "PupilC",
    "EyelidHideA",
    "EyelidHideB",
];

pub const MOUTH_TYPES: &[&str] = &[
    "Base",
    "Normal",
    "CheekA_L",
    "CheekA_R",
    "WaraiA",
    "WaraiB",
    "WaraiC",
    "WaraiD",
    "WaraiE",
    "IkariA",
    "IkariB",
    "KanasiA",
    "DoyaA",
    "DereA",
    "OdorokiA",
    "OdorokiB",
    "JitoA",
    "KomariA",
    "KusyoA_L",
    "KusyoA_R",
    "KusyoB_L",
    "KusyoB_R",
    "UreiA",
    "TalkA_A_S",
    "TalkA_A_L",
    "TalkA_I_S",
    "TalkA_I_L",
    "TalkA_U_S",
    "TalkA_U_L",
    "TalkA_E_S",
    "TalkA_E_L",
    "TalkA_O_S",
    "TalkA_O_L",
    "TalkB_A_S",
    "TalkB_A_L",
    "TalkB_I_S",
    "TalkB_I_L",
    "TalkB_E_S",
    "TalkB_E_L",
    "RunA",
    "RunB",
    "DrivenA",
    "ToothHide",
    "TalkC_I",
    "TanA",
    "TanB",
    "TanC_L",
    "TanD_L",
    "TanC_R",
    "TanD_R",
    "Offset_U",
    "Offset_D",
    "Offset_L",
    "Offset_R",
    "Scale_U",
    "Scale_D",
    "LowAngle",
];

pub const EAR_TYPES: &[&str] = &[
    "Base",
    "Base_N",
    "Kanasi",
    "Dere_N",
    "Dere",
    "Yure",
    "Biku_N",
    "Biku",
    "Ikari",
    "Tanosi",
    "Up_N",
    "Up",
    "Down",
    "Front",
    "Side",
    "Back",
    "Roll",
];

pub const PRESET_EXPRESSIONS: &[&str] = &[
    "Base",
    "WinkL",
    "WinkR",
    "EyeHalfA",
    "EyeClose",
    "WaraiA",
    "WaraiB",
    "WaraiC",
    "WaraiD",
    "WaraiE",
    "IkariA",
    "IkariB",
    "IkariC",
    "IkariD",
    "KanasiA",
    "KanasiB",
    "KanasiC",
    "KanasiD",
    "DoyaA",
    "DoyaB",
    "FutuA",
    "FutuB",
    "OdorokiA",
    "OdorokiB",
    "OdorokiC",
    "OdorokiD",
    "JitomeA",
    "JitomeB",
    "KomariA",
    "KomariB",
    "KomariC",
    "KomariD",
    "DereA",
    "DereB",
    "KusyoAL",
    "KusyoBL",
    "KusyoCL",
    "KusyoDL",
    "UniqueA",
    "UniqueB",
    "UniqueC",
    "UniqueD",
    "UniqueE",
    "UniqueF",
    "UniqueG",
    "UniqueH",
    "UniqueI",
    "UniqueJ",
    "UniqueK",
    "MouthAS",
    "MouthAM",
    "MouthAL",
    "MouthIS",
    "MouthIM",
    "MouthIL",
    "MouthUS",
    "MouthUM",
    "MouthUL",
    "MouthES",
    "MouthEM",
    "MouthEL",
    "MouthOS",
    "MouthOM",
    "MouthOL",
    "RunSlowA",
    "RunNormalA",
    "RunFastA",
    "EyeHideA",
];

use std::collections::HashMap;

fn default_weight() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacialOverrideConfig {
    pub enabled: bool,
    pub target_character: i32, // -1 = All, 0..=18 = Live Position index, or chara_id

    // Morphs weights matching UmaViewer lists
    #[serde(default)]
    pub emotion_weights: HashMap<String, f32>,
    #[serde(default)]
    pub eye_weights: HashMap<String, f32>,
    #[serde(default)]
    pub eyebrow_weights: HashMap<String, f32>,
    #[serde(default)]
    pub mouth_weights: HashMap<String, f32>,
    #[serde(default)]
    pub ear_weights: HashMap<String, f32>,

    // Quick active state for engine hooks
    #[serde(default)]
    pub active_emotion: String,
    #[serde(default = "default_weight")]
    pub active_emotion_weight: f32,

    #[serde(default)]
    pub override_ear: bool,
    #[serde(default)]
    pub active_ear: i32,
    #[serde(default = "default_weight")]
    pub active_ear_weight: f32,

    // Legacy fields for backward compatibility
    #[serde(default)]
    pub use_preset: bool,
    #[serde(default)]
    pub preset_face: usize,
    #[serde(default = "default_weight")]
    pub preset_weight: f32,

    #[serde(default)]
    pub link_lr: bool,

    #[serde(default)]
    pub eyebrow_r: usize,
    #[serde(default)]
    pub eyebrow_l: usize,
    #[serde(default = "default_weight")]
    pub eyebrow_weight: f32,

    #[serde(default)]
    pub eye_r: usize,
    #[serde(default)]
    pub eye_l: usize,
    #[serde(default = "default_weight")]
    pub eye_weight: f32,

    #[serde(default)]
    pub mouth: usize,
    #[serde(default = "default_weight")]
    pub mouth_weight: f32,

    #[serde(default)]
    pub ear_r: usize,
    #[serde(default)]
    pub ear_l: usize,
    #[serde(default = "default_weight")]
    pub ear_weight: f32,
}

impl Default for FacialOverrideConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            target_character: -1,
            emotion_weights: HashMap::new(),
            eye_weights: HashMap::new(),
            eyebrow_weights: HashMap::new(),
            mouth_weights: HashMap::new(),
            ear_weights: HashMap::new(),
            active_emotion: "Base".to_string(),
            active_emotion_weight: 1.0,
            override_ear: false,
            active_ear: 0,
            active_ear_weight: 1.0,
            use_preset: true,
            preset_face: 0, // "Base"
            preset_weight: 1.0,

            link_lr: true,

            eyebrow_r: 0,
            eyebrow_l: 0,
            eyebrow_weight: 1.0,

            eye_r: 0,
            eye_l: 0,
            eye_weight: 1.0,

            mouth: 0,
            mouth_weight: 1.0,

            ear_r: 0,
            ear_l: 0,
            ear_weight: 1.0,
        }
    }
}

pub static FACIAL_OVERRIDE_CONFIG: Lazy<Mutex<FacialOverrideConfig>> =
    Lazy::new(|| Mutex::new(FacialOverrideConfig::default()));

pub fn get_config() -> FacialOverrideConfig {
    FACIAL_OVERRIDE_CONFIG.lock().unwrap().clone()
}

pub fn update_config<F: FnOnce(&mut FacialOverrideConfig)>(f: F) {
    let mut lock = FACIAL_OVERRIDE_CONFIG.lock().unwrap();
    f(&mut lock);
}
