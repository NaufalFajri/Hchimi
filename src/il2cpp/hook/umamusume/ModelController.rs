use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    ptr::null_mut,
    sync::Mutex,
};

use once_cell::sync::Lazy;

use crate::{
    core::facial_override::{self, FacialOverrideConfig},
    il2cpp::{
        api::{il2cpp_class_get_field_from_name, il2cpp_class_get_name, il2cpp_runtime_class_init},
        ext::{Il2CppObjectExt, StringExt},
        symbols::{self, get_method_addr},
        types::*,
    },
};

static mut GET_OWNER_OBJECT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_OwnerObject, GET_OWNER_OBJECT_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

static mut SET_VISIBLE_ADDR: usize = 0;
impl_addr_wrapper_fn!(SetVisible, SET_VISIBLE_ADDR, (), this: *mut Il2CppObject, visible: bool, force: bool);

static mut SET_FACE_ADDR: usize = 0;
static mut SET_EAR_ADDR: usize = 0;
static mut SET_EAR_7_ADDR: usize = 0;
static mut FACE_TYPE_CLASS: *mut Il2CppClass = null_mut();

static FACE_TYPE_CACHE: Lazy<Mutex<HashMap<String, usize>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub unsafe fn get_chara_id(model_controller: *mut Il2CppObject) -> i32 {
    if model_controller.is_null() {
        return 0;
    }
    let build_info_ptr = (model_controller as *mut u8).add(0x90) as *mut *mut u8;
    if build_info_ptr.is_null() {
        return 0;
    }
    let build_info = *build_info_ptr;
    if build_info.is_null() {
        return 0;
    }
    *(build_info.add(0x14) as *mut i32)
}

pub unsafe fn get_character_index(model_controller: *mut Il2CppObject) -> i32 {
    if model_controller.is_null() {
        return -1;
    }
    let klass = (*model_controller).klass();
    if klass.is_null() {
        return -1;
    }
    let name_ptr = il2cpp_class_get_name(klass);
    if name_ptr.is_null() {
        return -1;
    }
    let class_name = CStr::from_ptr(name_ptr).to_string_lossy();
    if class_name.contains("LiveModelController") {
        *( (model_controller as *mut u8).add(0x660) as *mut i32 )
    } else {
        -1
    }
}

pub fn matches_target_character(model_controller: *mut Il2CppObject, cfg: &FacialOverrideConfig) -> bool {
    if cfg.target_character == -1 {
        return true;
    }
    let live_index = unsafe { get_character_index(model_controller) };
    if live_index != -1 && live_index == cfg.target_character {
        return true;
    }
    let chara_id = unsafe { get_chara_id(model_controller) };
    if chara_id > 0 && chara_id == cfg.target_character {
        return true;
    }
    false
}

pub fn get_face_type_by_name(name: &str) -> *mut Il2CppObject {
    if let Ok(cache) = FACE_TYPE_CACHE.lock() {
        if let Some(&ptr) = cache.get(name) {
            return ptr as *mut Il2CppObject;
        }
    }

    let klass = unsafe { FACE_TYPE_CLASS };
    if klass.is_null() {
        return null_mut();
    }

    il2cpp_runtime_class_init(klass);

    let mut found_obj: *mut Il2CppObject = null_mut();

    // 1. Try static field (e.g. FaceType.WaraiA)
    if let Ok(c_name) = CString::new(name) {
        let field = il2cpp_class_get_field_from_name(klass, c_name.as_ptr());
        if !field.is_null() {
            found_obj = symbols::get_static_field_object_value(field);
        }
    }

    // 2. Try op_Implicit(string) overload
    if found_obj.is_null() {
        let op_implicit_res = symbols::get_method_overload(
            klass,
            "op_Implicit",
            &[Il2CppTypeEnum_IL2CPP_TYPE_STRING],
        );
        if let Ok(method) = op_implicit_res {
            let method_ptr = unsafe { (*method).methodPointer };
            if method_ptr != 0 {
                let op_implicit: extern "C" fn(*mut Il2CppString) -> *mut Il2CppObject =
                    unsafe { std::mem::transmute(method_ptr) };
                let il2cpp_str = name.to_il2cpp_string();
                found_obj = op_implicit(il2cpp_str);
            }
        }
    }

    if !found_obj.is_null() {
        if let Ok(mut cache) = FACE_TYPE_CACHE.lock() {
            cache.insert(name.to_string(), found_obj as usize);
        }
    }

    found_obj
}

type SetFaceFn = extern "C" fn(
    this: *mut Il2CppObject,
    face_type: *mut Il2CppObject,
    weight: f32,
    face_group_set: i32,
    duration_time: f32,
    is_blend: bool,
    dont_clear_faces: bool,
    is_playing_ear_anim: bool,
);

extern "C" fn SetFace(
    this: *mut Il2CppObject,
    mut face_type: *mut Il2CppObject,
    mut weight: f32,
    face_group_set: i32,
    duration_time: f32,
    is_blend: bool,
    dont_clear_faces: bool,
    is_playing_ear_anim: bool,
) {
    let cfg = facial_override::get_config();
    if cfg.enabled && matches_target_character(this, &cfg) {
        let emotion_name = if !cfg.active_emotion.is_empty() {
            cfg.active_emotion.as_str()
        } else if let Some(name) = facial_override::PRESET_EXPRESSIONS.get(cfg.preset_face) {
            *name
        } else {
            "Base"
        };
        let override_ft = get_face_type_by_name(emotion_name);
        if !override_ft.is_null() {
            face_type = override_ft;
            weight = cfg.active_emotion_weight;
        }
    }
    get_orig_fn!(SetFace, SetFaceFn)(
        this,
        face_type,
        weight,
        face_group_set,
        duration_time,
        is_blend,
        dont_clear_faces,
        is_playing_ear_anim,
    );
}

type SetEarFn = extern "C" fn(
    this: *mut Il2CppObject,
    ear_type: i32,
    weight: f32,
    duration_time: f32,
    is_blend: bool,
    dont_clear_faces: bool,
    is_force_set_prev_weight: bool,
);

extern "C" fn SetEar(
    this: *mut Il2CppObject,
    mut ear_type: i32,
    mut weight: f32,
    duration_time: f32,
    is_blend: bool,
    dont_clear_faces: bool,
    is_force_set_prev_weight: bool,
) {
    let cfg = facial_override::get_config();
    if cfg.enabled && cfg.override_ear && matches_target_character(this, &cfg) {
        ear_type = cfg.active_ear;
        weight = cfg.active_ear_weight;
    }
    get_orig_fn!(SetEar, SetEarFn)(
        this,
        ear_type,
        weight,
        duration_time,
        is_blend,
        dont_clear_faces,
        is_force_set_prev_weight,
    );
}

type SetEar7Fn = extern "C" fn(
    this: *mut Il2CppObject,
    ear_type: i32,
    ear_group: i32,
    weight: f32,
    duration_time: f32,
    is_blend: bool,
    dont_clear_faces: bool,
    is_force_set_prev_weight: bool,
);

extern "C" fn SetEar7(
    this: *mut Il2CppObject,
    mut ear_type: i32,
    ear_group: i32,
    mut weight: f32,
    duration_time: f32,
    is_blend: bool,
    dont_clear_faces: bool,
    is_force_set_prev_weight: bool,
) {
    let cfg = facial_override::get_config();
    if cfg.enabled && cfg.override_ear && matches_target_character(this, &cfg) {
        ear_type = cfg.active_ear;
        weight = cfg.active_ear_weight;
    }
    get_orig_fn!(SetEar7, SetEar7Fn)(
        this,
        ear_type,
        ear_group,
        weight,
        duration_time,
        is_blend,
        dont_clear_faces,
        is_force_set_prev_weight,
    );
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", ModelController);

    unsafe {
        GET_OWNER_OBJECT_ADDR = get_method_addr(ModelController, c"get_OwnerObject", 0);
        SET_VISIBLE_ADDR = get_method_addr(ModelController, c"SetVisible", 2);

        SET_FACE_ADDR = get_method_addr(ModelController, c"SetFace", 7);
        if SET_FACE_ADDR != 0 {
            new_hook!(SET_FACE_ADDR, SetFace);
        }

        SET_EAR_ADDR = get_method_addr(ModelController, c"SetEar", 6);
        if SET_EAR_ADDR != 0 {
            new_hook!(SET_EAR_ADDR, SetEar);
        }

        SET_EAR_7_ADDR = get_method_addr(ModelController, c"SetEar", 7);
        if SET_EAR_7_ADDR != 0 {
            new_hook!(SET_EAR_7_ADDR, SetEar7);
        }
    }

    if let Ok(face_type_class) = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"FaceType") {
        unsafe {
            FACE_TYPE_CLASS = face_type_class;
            il2cpp_runtime_class_init(face_type_class);
        }

        // Pre-populate cache with static fields
        if let Ok(mut cache) = FACE_TYPE_CACHE.lock() {
            let mut loaded = 0;
            for &preset_name in facial_override::PRESET_EXPRESSIONS {
                if let Ok(c_name) = CString::new(preset_name) {
                    let field = il2cpp_class_get_field_from_name(face_type_class, c_name.as_ptr());
                    if !field.is_null() {
                        let obj: *mut Il2CppObject = symbols::get_static_field_object_value(field);
                        if !obj.is_null() {
                            cache.insert(preset_name.to_string(), obj as usize);
                            loaded += 1;
                        }
                    }
                }
            }
            info!("Preloaded {}/{} FaceType presets into cache", loaded, facial_override::PRESET_EXPRESSIONS.len());
        }
    }
}
