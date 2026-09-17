use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*},
};

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

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsHomeCharaMessage);

    let PlayIdle_addr = get_method_addr(PartsHomeCharaMessage, c"PlayIdle", 1);
    let PlaySet_addr = get_method_addr(PartsHomeCharaMessage, c"PlaySet", 1);

    new_hook!(PlayIdle_addr, PlayIdle);
    new_hook!(PlaySet_addr, PlaySet);
}
