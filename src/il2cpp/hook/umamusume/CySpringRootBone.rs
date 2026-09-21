use crate::{
    core::{ext::Utf16StringExt, Hachimi},
    il2cpp::{
        api::il2cpp_class_value_size,
        ext::Il2CppStringExt,
        symbols::{get_field_from_name, get_field_value, get_method_addr},
        types::*,
    },
};

static mut NATIVE_ARRAY_FIELD: *mut FieldInfo = 0 as _;
static mut ROOT_BONE_NAME_FIELD: *mut FieldInfo = 0 as _;
static mut ELEMENT_BONE_NAME_FIELD: *mut FieldInfo = 0 as _;

static mut WORKING_STIFFNESS_OFFSET: usize = 0;
static mut WORKING_DRAG_OFFSET: usize = 0;
static mut WORKING_GRAVITY_OFFSET: usize = 0;
static mut WORKING_ELEM_SIZE: usize = 0;

type SetNativeClothFn =
    extern "C" fn(this: *mut Il2CppObject, element: *mut Il2CppObject, legacy_scale: f32);

extern "C" fn SetNativeCloth(
    this: *mut Il2CppObject,
    element: *mut Il2CppObject,
    legacy_scale: f32,
) {
    get_orig_fn!(SetNativeCloth, SetNativeClothFn)(this, element, legacy_scale);

    let config = Hachimi::instance().config.load();
    let bust_cfg = &config.cyspring_bust_modifier;
    if !bust_cfg.enabled {
        return;
    }

    if unsafe { NATIVE_ARRAY_FIELD.is_null() || WORKING_ELEM_SIZE == 0 } {
        return;
    }

    // Check bone name from CySpringRootBone or CySpringParamDataElement
    let mut is_bust = false;
    let mut matched_name = String::new();

    if unsafe { !ROOT_BONE_NAME_FIELD.is_null() } {
        let bone_name_ptr: *mut Il2CppString =
            get_field_value(this, unsafe { ROOT_BONE_NAME_FIELD });
        if !bone_name_ptr.is_null() {
            let utf = unsafe { (*bone_name_ptr).as_utf16str() };
            if utf.starts_with("Sp_Ch_Bust0_") {
                is_bust = true;
                matched_name = utf.to_string();
            }
        }
    }

    if !is_bust && !element.is_null() && unsafe { !ELEMENT_BONE_NAME_FIELD.is_null() } {
        let elem_bone_name_ptr: *mut Il2CppString =
            get_field_value(element, unsafe { ELEMENT_BONE_NAME_FIELD });
        if !elem_bone_name_ptr.is_null() {
            let utf = unsafe { (*elem_bone_name_ptr).as_utf16str() };
            if utf.starts_with("Sp_Ch_Bust0_") {
                is_bust = true;
                matched_name = utf.to_string();
            }
        }
    }

    if !is_bust {
        return;
    }

    let native_array: *mut Il2CppArray = get_field_value(this, unsafe { NATIVE_ARRAY_FIELD });
    if native_array.is_null() {
        return;
    }

    let len = unsafe { (*native_array).max_length as usize };
    if len == 0 {
        return;
    }

    let data_ptr = unsafe { native_array.add(1) as *mut u8 };

    for i in 0..len {
        unsafe {
            let elem_ptr = data_ptr.add(i * WORKING_ELEM_SIZE);
            let stiffness_ptr = elem_ptr.add(WORKING_STIFFNESS_OFFSET) as *mut f32;
            let drag_ptr = elem_ptr.add(WORKING_DRAG_OFFSET) as *mut f32;
            let gravity_ptr = elem_ptr.add(WORKING_GRAVITY_OFFSET) as *mut f32;

            *stiffness_ptr *= bust_cfg.stiffness_multiplier;
            *drag_ptr *= bust_cfg.drag_multiplier;
            *gravity_ptr *= bust_cfg.gravity_multiplier;
        }
    }

    debug!(
        "Applied bust physics multipliers to {}: stiffness x{}, drag x{}, gravity x{} ({} chain bones)",
        matched_name,
        bust_cfg.stiffness_multiplier,
        bust_cfg.drag_multiplier,
        bust_cfg.gravity_multiplier,
        len,
    );
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, CySpringRootBone);
    get_class_or_return!(umamusume, Gallop, CySpringBoneBase);
    get_class_or_return!(umamusume, Gallop, CySpringParamDataElement);
    get_class_or_return!(umamusume, Gallop, NativeClothWorking);

    let set_native_cloth_addr = get_method_addr(CySpringRootBone, c"SetNativeCloth", 2);
    new_hook!(set_native_cloth_addr, SetNativeCloth);

    unsafe {
        NATIVE_ARRAY_FIELD = get_field_from_name(CySpringRootBone, c"NativeArray");
        ROOT_BONE_NAME_FIELD = get_field_from_name(CySpringBoneBase, c"_boneName");
        ELEMENT_BONE_NAME_FIELD = get_field_from_name(CySpringParamDataElement, c"_boneName");

        let stiffness_field = get_field_from_name(NativeClothWorking, c"StiffnessForce");
        let drag_field = get_field_from_name(NativeClothWorking, c"DragForce");
        let gravity_field = get_field_from_name(NativeClothWorking, c"Gravity");

        if !stiffness_field.is_null() && !drag_field.is_null() && !gravity_field.is_null() {
            WORKING_STIFFNESS_OFFSET = (*stiffness_field).offset as usize - 0x10;
            WORKING_DRAG_OFFSET = (*drag_field).offset as usize - 0x10;
            WORKING_GRAVITY_OFFSET = (*gravity_field).offset as usize - 0x10;
            WORKING_ELEM_SIZE =
                il2cpp_class_value_size(NativeClothWorking, std::ptr::null_mut()) as usize;
        }
    }
}
