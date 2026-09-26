use crate::{
    core::Hachimi,
    il2cpp::{
        symbols::{get_method_addr, get_type_object_for_class},
        types::*,
    },
};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

static mut GET_ISPLAYING_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_IsPlaying, GET_ISPLAYING_ADDR, bool, this: *mut Il2CppObject);

type PlayIdleFn = extern "C" fn(this: *mut Il2CppObject, useSmoothFaceBlend: bool);
extern "C" fn PlayIdle(this: *mut Il2CppObject, useSmoothFaceBlend: bool) {
    if Hachimi::instance().config.load().chara_speak_home_idle {
        get_orig_fn!(PlayIdle, PlayIdleFn)(this, useSmoothFaceBlend);
    }
}

type PlaySetFn = extern "C" fn(this: *mut Il2CppObject, useSmoothFaceBlend: bool);
extern "C" fn PlaySet(this: *mut Il2CppObject, useSmoothFaceBlend: bool) {
    if Hachimi::instance().config.load().chara_speak_home_idle {
        get_orig_fn!(PlaySet, PlaySetFn)(this, useSmoothFaceBlend);
    }
}

type SetModelFn = extern "C" fn(this: *mut Il2CppObject, modelController: *mut Il2CppObject, isPlayVoice: bool);
extern "C" fn SetModel(this: *mut Il2CppObject, modelController: *mut Il2CppObject, isPlayVoice: bool) {
    let play_voice = isPlayVoice && Hachimi::instance().config.load().chara_speak_home_idle;
    get_orig_fn!(SetModel, SetModelFn)(this, modelController, play_voice);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsCharaMessageBase);

    unsafe {
        CLASS = PartsCharaMessageBase;
        TYPE_OBJECT = get_type_object_for_class(PartsCharaMessageBase);
        GET_ISPLAYING_ADDR = get_method_addr(PartsCharaMessageBase, c"get_IsPlaying", 0);
    }

    let PlayIdle_addr = get_method_addr(PartsCharaMessageBase, c"PlayIdle", 1);
    let PlaySet_addr = get_method_addr(PartsCharaMessageBase, c"PlaySet", 1);
    let SetModel_addr = get_method_addr(PartsCharaMessageBase, c"SetModel", 2);

    new_hook!(PlayIdle_addr, PlayIdle);
    new_hook!(PlaySet_addr, PlaySet);
    new_hook!(SetModel_addr, SetModel);
}
