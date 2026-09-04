use crate::il2cpp::{symbols::get_method_addr, types::*};
use crate::il2cpp::symbols::IEnumerator;

const TRAINING_RESULT_FAILURE: i32 = 1;

type OnSuccessSendCommandFn = extern "C" fn(
    this: *mut Il2CppObject,
    turn_info: *mut Il2CppObject,
    sub_id: i32,
    result_type: i32,
) -> IEnumerator;

extern "C" fn OnSuccessSendCommand(
    this: *mut Il2CppObject,
    turn_info: *mut Il2CppObject,
    sub_id: i32,
    result_type: i32,
) -> IEnumerator {
    let result_type = if result_type == TRAINING_RESULT_FAILURE {
        result_type
    }
    else {
        TRAINING_RESULT_FAILURE
    };

    get_orig_fn!(OnSuccessSendCommand, OnSuccessSendCommandFn)(
        this, turn_info, sub_id, result_type
    )
}

type PlayInResultFlashNormalFn = extern "C" fn(
    this: *mut Il2CppObject,
    result_type: i32,
);

extern "C" fn PlayInResultFlashNormal(
    this: *mut Il2CppObject,
    _result_type: i32,
) {
    get_orig_fn!(PlayInResultFlashNormal, PlayInResultFlashNormalFn)(
        this, TRAINING_RESULT_FAILURE
    )
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, SingleModeMainTrainingCuttController);

    let on_success_send_command_addr = get_method_addr(
        SingleModeMainTrainingCuttController,
        c"OnSuccessSendCommand",
        3,
    );
    new_hook!(on_success_send_command_addr, OnSuccessSendCommand);

    get_class_or_return!(umamusume, Gallop, AbstractTrainingCuttScenarioController);
    let play_in_result_flash_normal_addr = get_method_addr(
        AbstractTrainingCuttScenarioController,
        c"PlayInResultFlashNormal",
        1,
    );
    new_hook!(play_in_result_flash_normal_addr, PlayInResultFlashNormal);
}