use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*},
};

type SetModelFn = extern "C" fn(this: *mut Il2CppObject, modelController: *mut Il2CppObject, isPlayVoice: bool);
extern "C" fn SetModel(this: *mut Il2CppObject, modelController: *mut Il2CppObject, isPlayVoice: bool) {
    let play_voice = isPlayVoice && Hachimi::instance().config.load().chara_speak_home_idle;
    get_orig_fn!(SetModel, SetModelFn)(this, modelController, play_voice);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsHomeCharaMessage);

    let SetModel_addr = get_method_addr(PartsHomeCharaMessage, c"SetModel", 2);

    new_hook!(SetModel_addr, SetModel);
}
