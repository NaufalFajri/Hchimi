use crate::{
    core::Hachimi,
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
static mut WORKING_IS_LIMIT_OFFSET: usize = 0;
static mut WORKING_VERT_WIND_SLOW_OFFSET: usize = 0;
static mut WORKING_VERT_WIND_FAST_OFFSET: usize = 0;
static mut WORKING_HORIZ_WIND_SLOW_OFFSET: usize = 0;
static mut WORKING_HORIZ_WIND_FAST_OFFSET: usize = 0;
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
    let skirt_cfg = &config.cyspring_skirt_modifier;
    if !bust_cfg.enabled && !skirt_cfg.enabled {
        return;
    }

    if unsafe { NATIVE_ARRAY_FIELD.is_null() || WORKING_ELEM_SIZE == 0 } {
        return;
    }

    // Check bone name from CySpringRootBone or CySpringParamDataElement
    let mut bone_name = String::new();

    if unsafe { !ROOT_BONE_NAME_FIELD.is_null() } {
        let bone_name_ptr: *mut Il2CppString =
            get_field_value(this, unsafe { ROOT_BONE_NAME_FIELD });
        if !bone_name_ptr.is_null() {
            bone_name = unsafe { (*bone_name_ptr).as_utf16str().to_string() };
        }
    }

    if bone_name.is_empty() && !element.is_null() && unsafe { !ELEMENT_BONE_NAME_FIELD.is_null() } {
        let elem_bone_name_ptr: *mut Il2CppString =
            get_field_value(element, unsafe { ELEMENT_BONE_NAME_FIELD });
        if !elem_bone_name_ptr.is_null() {
            bone_name = unsafe { (*elem_bone_name_ptr).as_utf16str().to_string() };
        }
    }

    let is_bust = bust_cfg.enabled && bone_name.starts_with("Sp_Ch_Bust0_");
    let is_skirt = skirt_cfg.enabled && (bone_name.contains("Skirt") || bone_name.contains("skirt"));

    if !is_bust && !is_skirt {
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

            if is_bust {
                let stiffness_ptr = elem_ptr.add(WORKING_STIFFNESS_OFFSET) as *mut f32;
                let drag_ptr = elem_ptr.add(WORKING_DRAG_OFFSET) as *mut f32;
                let gravity_ptr = elem_ptr.add(WORKING_GRAVITY_OFFSET) as *mut f32;

                *stiffness_ptr *= bust_cfg.stiffness_multiplier;
                *drag_ptr *= bust_cfg.drag_multiplier;
                *gravity_ptr *= bust_cfg.gravity_multiplier;

                if bust_cfg.disable_limit_angle && WORKING_IS_LIMIT_OFFSET != 0 {
                    let is_limit_ptr = elem_ptr.add(WORKING_IS_LIMIT_OFFSET) as *mut bool;
                    *is_limit_ptr = false;
                }
            } else if is_skirt {
                let stiffness_ptr = elem_ptr.add(WORKING_STIFFNESS_OFFSET) as *mut f32;
                let drag_ptr = elem_ptr.add(WORKING_DRAG_OFFSET) as *mut f32;
                let gravity_ptr = elem_ptr.add(WORKING_GRAVITY_OFFSET) as *mut f32;

                *stiffness_ptr *= skirt_cfg.stiffness_multiplier;
                *drag_ptr *= skirt_cfg.drag_multiplier;
                *gravity_ptr *= skirt_cfg.gravity_multiplier;

                if skirt_cfg.disable_limit_angle && WORKING_IS_LIMIT_OFFSET != 0 {
                    let is_limit_ptr = elem_ptr.add(WORKING_IS_LIMIT_OFFSET) as *mut bool;
                    *is_limit_ptr = false;
                }

                if skirt_cfg.wind_multiplier != 1.0 {
                    if WORKING_VERT_WIND_SLOW_OFFSET != 0 {
                        let ptr = elem_ptr.add(WORKING_VERT_WIND_SLOW_OFFSET) as *mut f32;
                        *ptr *= skirt_cfg.wind_multiplier;
                    }
                    if WORKING_VERT_WIND_FAST_OFFSET != 0 {
                        let ptr = elem_ptr.add(WORKING_VERT_WIND_FAST_OFFSET) as *mut f32;
                        *ptr *= skirt_cfg.wind_multiplier;
                    }
                    if WORKING_HORIZ_WIND_SLOW_OFFSET != 0 {
                        let ptr = elem_ptr.add(WORKING_HORIZ_WIND_SLOW_OFFSET) as *mut f32;
                        *ptr *= skirt_cfg.wind_multiplier;
                    }
                    if WORKING_HORIZ_WIND_FAST_OFFSET != 0 {
                        let ptr = elem_ptr.add(WORKING_HORIZ_WIND_FAST_OFFSET) as *mut f32;
                        *ptr *= skirt_cfg.wind_multiplier;
                    }
                }
            }
        }
    }

    if is_bust {
        debug!(
            "Applied bust physics multipliers to {}: stiffness x{}, drag x{}, gravity x{}, disable_limit={} ({} chain bones)",
            bone_name,
            bust_cfg.stiffness_multiplier,
            bust_cfg.drag_multiplier,
            bust_cfg.gravity_multiplier,
            bust_cfg.disable_limit_angle,
            len,
        );
    } else if is_skirt {
        debug!(
            "Applied skirt physics multipliers to {}: stiffness x{}, drag x{}, gravity x{}, wind x{}, disable_limit={} ({} chain bones)",
            bone_name,
            skirt_cfg.stiffness_multiplier,
            skirt_cfg.drag_multiplier,
            skirt_cfg.gravity_multiplier,
            skirt_cfg.wind_multiplier,
            skirt_cfg.disable_limit_angle,
            len,
        );
    }
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
        let is_limit_field = get_field_from_name(NativeClothWorking, c"IsLimit");
        let vert_slow_field = get_field_from_name(NativeClothWorking, c"VerticalWindRateSlow");
        let vert_fast_field = get_field_from_name(NativeClothWorking, c"VerticalWindRateFast");
        let horiz_slow_field = get_field_from_name(NativeClothWorking, c"HorizontalWindRateSlow");
        let horiz_fast_field = get_field_from_name(NativeClothWorking, c"HorizontalWindRateFast");

        if !stiffness_field.is_null() && !drag_field.is_null() && !gravity_field.is_null() {
            WORKING_STIFFNESS_OFFSET = (*stiffness_field).offset as usize - 0x10;
            WORKING_DRAG_OFFSET = (*drag_field).offset as usize - 0x10;
            WORKING_GRAVITY_OFFSET = (*gravity_field).offset as usize - 0x10;
            WORKING_ELEM_SIZE =
                il2cpp_class_value_size(NativeClothWorking, std::ptr::null_mut()) as usize;
        }

        if !is_limit_field.is_null() {
            WORKING_IS_LIMIT_OFFSET = (*is_limit_field).offset as usize - 0x10;
        }
        if !vert_slow_field.is_null() {
            WORKING_VERT_WIND_SLOW_OFFSET = (*vert_slow_field).offset as usize - 0x10;
        }
        if !vert_fast_field.is_null() {
            WORKING_VERT_WIND_FAST_OFFSET = (*vert_fast_field).offset as usize - 0x10;
        }
        if !horiz_slow_field.is_null() {
            WORKING_HORIZ_WIND_SLOW_OFFSET = (*horiz_slow_field).offset as usize - 0x10;
        }
        if !horiz_fast_field.is_null() {
            WORKING_HORIZ_WIND_FAST_OFFSET = (*horiz_fast_field).offset as usize - 0x10;
        }
    }
}
