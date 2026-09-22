use serde::{Deserialize, Serialize};

use crate::{
    core::Hachimi,
    il2cpp::{
        symbols::{get_field_from_name, get_method_addr, set_field_value},
        types::*,
    },
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[repr(i32)]
pub enum SpringUpdateMode {
    ModeNormal,
    Mode60FPS,
    SkipFrame,
    SkipFramePostAlways,
}

static mut UPDATEMODE_FIELD: *mut FieldInfo = 0 as _;
static mut ENABLE_DUMMY_WIND_FIELD: *mut FieldInfo = 0 as _;
static mut DUMMY_WIND_DIR_FIELD: *mut FieldInfo = 0 as _;
static mut WIND_POWER_RATE_FIELD: *mut FieldInfo = 0 as _;

fn set_UpdateMode(this: *mut Il2CppObject, value: &SpringUpdateMode) {
    set_field_value(this, unsafe { UPDATEMODE_FIELD }, value);
}

type InitFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn Init(this: *mut Il2CppObject) {
    get_orig_fn!(Init, InitFn)(this);

    let config = Hachimi::instance().config.load();

    if let Some(mode) = config.physics_update_mode.as_ref() {
        set_UpdateMode(this, mode);
    }

    if config.cyspring_skirt_modifier.enabled && config.cyspring_skirt_modifier.dummy_wind {
        let enable = true;
        let dir = Vector3_t {
            x: config.cyspring_skirt_modifier.dummy_wind_dir[0],
            y: config.cyspring_skirt_modifier.dummy_wind_dir[1],
            z: config.cyspring_skirt_modifier.dummy_wind_dir[2],
        };
        unsafe {
            if !ENABLE_DUMMY_WIND_FIELD.is_null() {
                set_field_value(this, ENABLE_DUMMY_WIND_FIELD, &enable);
            }
            if !DUMMY_WIND_DIR_FIELD.is_null() {
                set_field_value(this, DUMMY_WIND_DIR_FIELD, &dir);
            }
            if !WIND_POWER_RATE_FIELD.is_null() {
                let current_rate: f32 = crate::il2cpp::symbols::get_field_value(this, WIND_POWER_RATE_FIELD);
                if current_rate <= 0.0 {
                    let rate = 1.0f32;
                    set_field_value(this, WIND_POWER_RATE_FIELD, &rate);
                }
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, CySpringController);

    let Init_addr = get_method_addr(CySpringController, c"Init", 0);

    new_hook!(Init_addr, Init);

    unsafe {
        UPDATEMODE_FIELD = get_field_from_name(CySpringController, c"<UpdateMode>k__BackingField");
        ENABLE_DUMMY_WIND_FIELD = get_field_from_name(CySpringController, c"_isEnableDummyWind");
        DUMMY_WIND_DIR_FIELD = get_field_from_name(CySpringController, c"_dummyWindDir");
        WIND_POWER_RATE_FIELD = get_field_from_name(CySpringController, c"_windPowerRate");
    }
}
