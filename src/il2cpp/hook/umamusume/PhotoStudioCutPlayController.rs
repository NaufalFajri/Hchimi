use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use crate::{
    il2cpp::{
        ext::Il2CppStringExt,
        hook::UnityEngine_CoreModule::{Camera, Component, Object, Transform},
        symbols::{get_method_addr, Array},
        types::*,
    },
    windows::free_camera::{self, CameraScene},
};

type NoArgsFn = extern "C" fn(this: *mut Il2CppObject);
type EndViewFn = extern "C" fn(this: *mut Il2CppObject) -> *mut Il2CppObject;
type SetTargetCameraFn = extern "C" fn(this: *mut Il2CppObject, target_camera: *mut Il2CppObject);
type AlterLateUpdateMotionCameraFn = extern "C" fn(
    this: *mut Il2CppObject,
    active_sheet: *mut Il2CppObject,
    update_pos: bool,
    update_rot: bool,
);

static VIEW_CONTROLLER: AtomicPtr<Il2CppObject> = AtomicPtr::new(std::ptr::null_mut());
static CUT_TIMELINE_CONTROLLER: AtomicPtr<Il2CppObject> = AtomicPtr::new(std::ptr::null_mut());
static CUT_MOTION_CAMERA: AtomicPtr<Il2CppObject> = AtomicPtr::new(std::ptr::null_mut());
static CUT_CAMERA: AtomicPtr<Il2CppObject> = AtomicPtr::new(std::ptr::null_mut());
static CUT_CAMERA_TRANSFORM: AtomicPtr<Il2CppObject> = AtomicPtr::new(std::ptr::null_mut());

static ORIGINAL_CAMERA_PARENT: AtomicPtr<Il2CppObject> = AtomicPtr::new(std::ptr::null_mut());
static WAS_FREE_CAM_ACTIVE: AtomicBool = AtomicBool::new(false);

static mut LAST_CUT_CAMERA_POS: Vector3_t = Vector3_t {
    x: 0.0,
    y: 1.2,
    z: -2.0,
};
static mut LAST_CUT_CAMERA_ROT: Quaternion_t = Quaternion_t {
    w: 1.0,
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

fn detach_cut_cameras() {
    let cut_transform = CUT_CAMERA_TRANSFORM.load(Ordering::Relaxed);
    if !cut_transform.is_null()
        && Object::op_Implicit(cut_transform)
        && ORIGINAL_CAMERA_PARENT.load(Ordering::Relaxed).is_null()
    {
        let parent = Transform::get_parent(cut_transform);
        if !parent.is_null() && Object::op_Implicit(parent) {
            ORIGINAL_CAMERA_PARENT.store(parent, Ordering::Relaxed);
            Transform::SetParent(cut_transform, std::ptr::null_mut(), true);
        }
    }
}

fn restore_cut_camera_parents() {
    let orig_parent = ORIGINAL_CAMERA_PARENT.swap(std::ptr::null_mut(), Ordering::Relaxed);
    if !orig_parent.is_null() && Object::op_Implicit(orig_parent) {
        let cut_transform = CUT_CAMERA_TRANSFORM.load(Ordering::Relaxed);
        if !cut_transform.is_null() && Object::op_Implicit(cut_transform) {
            Transform::SetParent(cut_transform, orig_parent, true);
        }
    }
}

pub fn apply_photo_studio_free_camera() {
    if !free_camera::is_scene_enabled(CameraScene::PhotoStudioCutPlay) {
        return;
    }

    detach_cut_cameras();

    let mut position = free_camera::camera_pos();
    let rotation_opt = free_camera::camera_rotation();
    let look_at = free_camera::camera_look_at();
    let mut world_up = Vector3_t {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let mut apply_transform = |transform: *mut Il2CppObject| {
        if transform.is_null() {
            return;
        }
        Transform::set_position_Injected(transform, &mut position);
        if let Some(mut rot) = rotation_opt {
            Transform::set_rotation_Injected(transform, &mut rot);
        } else {
            let mut l_at = look_at;
            Transform::Internal_LookAt_Injected(transform, &mut l_at, &mut world_up);
        }
    };

    let cut_transform = CUT_CAMERA_TRANSFORM.load(Ordering::Relaxed);
    if !cut_transform.is_null() {
        apply_transform(cut_transform);
    }

    // Apply free camera to all active cameras named "CutInCamera" (multi-camera passes)
    let all_cams = Camera::get_allCameras();
    if !all_cams.is_null() {
        let arr = Array::<*mut Il2CppObject>::from(all_cams);
        for cam in unsafe { arr.as_slice() }.iter().copied() {
            if cam.is_null() {
                continue;
            }
            let name = Object::get_name(cam);
            let name_str = if !name.is_null() {
                unsafe { (*name).as_utf16str().to_string() }
            } else {
                String::new()
            };
            if name_str.contains("CutInCamera") {
                let t = Component::get_transform(cam);
                if !t.is_null() && t != cut_transform {
                    apply_transform(t);
                }
                if let Some(fov) = free_camera::fov_for_scene(CameraScene::PhotoStudioCutPlay) {
                    Camera::set_fieldOfView(cam, fov);
                }
            }
        }
    }

    // Keep camera FoV in sync on the primary cut camera
    if let Some(fov) = free_camera::fov_for_scene(CameraScene::PhotoStudioCutPlay) {
        let cut_cam = CUT_CAMERA.load(Ordering::Relaxed);
        if !cut_cam.is_null() {
            Camera::set_fieldOfView(cut_cam, fov);
        }
    }
}

fn cache_cameras_from_controller(controller: *mut Il2CppObject) {
    if controller.is_null() {
        return;
    }

    // Extract _motionCamera (offset 0xf8)
    let motion_cam = unsafe { *(controller.cast::<u8>().add(0xf8) as *const *mut Il2CppObject) };
    if !motion_cam.is_null() {
        CUT_MOTION_CAMERA.store(motion_cam, Ordering::Relaxed);

        // _targetCamera (offset 0x88)
        let target_cam =
            unsafe { *(motion_cam.cast::<u8>().add(0x88) as *const *mut Il2CppObject) };
        if !target_cam.is_null() {
            CUT_CAMERA.store(target_cam, Ordering::Relaxed);

            // _targetTransform (offset 0x98)
            let target_trans =
                unsafe { *(motion_cam.cast::<u8>().add(0x98) as *const *mut Il2CppObject) };
            if !target_trans.is_null() {
                CUT_CAMERA_TRANSFORM.store(target_trans, Ordering::Relaxed);
            } else {
                CUT_CAMERA_TRANSFORM
                    .store(Component::get_transform(target_cam), Ordering::Relaxed);
            }
        }
    }
}

extern "C" fn CutInTimelineMotionCamera_SetTargetCamera(
    this: *mut Il2CppObject,
    target_camera: *mut Il2CppObject,
) {
    let current_cam = CUT_CAMERA.load(Ordering::Relaxed);
    if !current_cam.is_null() && current_cam != target_camera {
        restore_cut_camera_parents();
    }

    CUT_MOTION_CAMERA.store(this, Ordering::Relaxed);
    if !target_camera.is_null() {
        CUT_CAMERA.store(target_camera, Ordering::Relaxed);
        let transform = Component::get_transform(target_camera);
        if !transform.is_null() {
            CUT_CAMERA_TRANSFORM.store(transform, Ordering::Relaxed);
        }
    }
    get_orig_fn!(
        CutInTimelineMotionCamera_SetTargetCamera,
        SetTargetCameraFn
    )(this, target_camera);
}

extern "C" fn CutInTimelineMotionCamera_OnDestroy(this: *mut Il2CppObject) {
    if CUT_MOTION_CAMERA.load(Ordering::Relaxed) == this {
        restore_cut_camera_parents();
        CUT_MOTION_CAMERA.store(std::ptr::null_mut(), Ordering::Relaxed);
        CUT_CAMERA.store(std::ptr::null_mut(), Ordering::Relaxed);
        CUT_CAMERA_TRANSFORM.store(std::ptr::null_mut(), Ordering::Relaxed);
    }
    get_orig_fn!(CutInTimelineMotionCamera_OnDestroy, NoArgsFn)(this);
}

extern "C" fn CutInTimelineController_AlterLateUpdate_MotionCamera(
    this: *mut Il2CppObject,
    active_sheet: *mut Il2CppObject,
    mut update_pos: bool,
    mut update_rot: bool,
) {
    CUT_TIMELINE_CONTROLLER.store(this, Ordering::Relaxed);
    cache_cameras_from_controller(this);

    let is_free_cam = free_camera::is_scene_enabled(CameraScene::PhotoStudioCutPlay);
    if is_free_cam {
        // Stop timeline animation tracks from overwriting free camera position & rotation
        update_pos = false;
        update_rot = false;
    }

    get_orig_fn!(
        CutInTimelineController_AlterLateUpdate_MotionCamera,
        AlterLateUpdateMotionCameraFn
    )(this, active_sheet, update_pos, update_rot);

    if is_free_cam {
        apply_photo_studio_free_camera();
    } else {
        // Track the current cut camera position when free camera is OFF
        let transform = CUT_CAMERA_TRANSFORM.load(Ordering::Relaxed);
        if !transform.is_null() {
            let mut pos = Vector3_t::default();
            let mut rot = Quaternion_t::default();
            Transform::get_position_Injected(transform, &mut pos);
            Transform::get_rotation_Injected(transform, &mut rot);
            unsafe {
                LAST_CUT_CAMERA_POS = pos;
                LAST_CUT_CAMERA_ROT = rot;
            }
        }
    }
}

extern "C" fn PhotoStudioPlayCutViewController_LateUpdateView(this: *mut Il2CppObject) {
    VIEW_CONTROLLER.store(this, Ordering::Relaxed);

    unsafe {
        free_camera::set_photo_studio_cut_play_active_with_transform(
            LAST_CUT_CAMERA_POS,
            LAST_CUT_CAMERA_ROT,
        );
    }
    free_camera::tick();

    get_orig_fn!(PhotoStudioPlayCutViewController_LateUpdateView, NoArgsFn)(this);

    let is_free_cam = free_camera::is_scene_enabled(CameraScene::PhotoStudioCutPlay);
    let was_free_cam = WAS_FREE_CAM_ACTIVE.swap(is_free_cam, Ordering::Relaxed);

    if is_free_cam {
        if !was_free_cam {
            detach_cut_cameras();
            unsafe {
                free_camera::reseed_photo_studio_camera_transform(
                    LAST_CUT_CAMERA_POS,
                    LAST_CUT_CAMERA_ROT,
                );
            }
        } else {
            detach_cut_cameras();
        }
        apply_photo_studio_free_camera();
    } else {
        if was_free_cam {
            restore_cut_camera_parents();
        }
        let transform = CUT_CAMERA_TRANSFORM.load(Ordering::Relaxed);
        if !transform.is_null() && Object::op_Implicit(transform) {
            let mut pos = Vector3_t::default();
            let mut rot = Quaternion_t::default();
            Transform::get_position_Injected(transform, &mut pos);
            Transform::get_rotation_Injected(transform, &mut rot);
            unsafe {
                LAST_CUT_CAMERA_POS = pos;
                LAST_CUT_CAMERA_ROT = rot;
            }
        }
    }
}

extern "C" fn PhotoStudioPlayCutViewController_EndView(
    this: *mut Il2CppObject,
) -> *mut Il2CppObject {
    restore_cut_camera_parents();
    WAS_FREE_CAM_ACTIVE.store(false, Ordering::Relaxed);
    VIEW_CONTROLLER.store(std::ptr::null_mut(), Ordering::Relaxed);
    CUT_TIMELINE_CONTROLLER.store(std::ptr::null_mut(), Ordering::Relaxed);
    CUT_MOTION_CAMERA.store(std::ptr::null_mut(), Ordering::Relaxed);
    CUT_CAMERA.store(std::ptr::null_mut(), Ordering::Relaxed);
    CUT_CAMERA_TRANSFORM.store(std::ptr::null_mut(), Ordering::Relaxed);
    free_camera::end_scene(CameraScene::PhotoStudioCutPlay);
    get_orig_fn!(PhotoStudioPlayCutViewController_EndView, EndViewFn)(this)
}

pub fn init(umamusume: *const Il2CppImage) {
    // Hook View Controller
    get_class_or_return!(umamusume, Gallop, PhotoStudioPlayCutViewController);

    let late_update_view =
        get_method_addr(PhotoStudioPlayCutViewController, c"LateUpdateView", 0);
    new_hook!(
        late_update_view,
        PhotoStudioPlayCutViewController_LateUpdateView
    );

    let end_view = get_method_addr(PhotoStudioPlayCutViewController, c"EndView", 0);
    new_hook!(end_view, PhotoStudioPlayCutViewController_EndView);

    // Hook CUTT Motion Camera
    if let Ok(motion_cam_class) =
        crate::il2cpp::symbols::get_class(umamusume, c"Gallop.CutIn.Cutt", c"CutInTimelineMotionCamera")
    {
        let set_target_cam =
            get_method_addr(motion_cam_class, c"SetTargetCamera", 1);
        new_hook!(
            set_target_cam,
            CutInTimelineMotionCamera_SetTargetCamera
        );

        let on_destroy = get_method_addr(motion_cam_class, c"OnDestroy", 0);
        new_hook!(on_destroy, CutInTimelineMotionCamera_OnDestroy);
    }

    // Hook CUTT Controller
    if let Ok(timeline_ctrl_class) =
        crate::il2cpp::symbols::get_class(umamusume, c"Gallop.CutIn.Cutt", c"CutInTimelineController")
    {
        let alter_late_update_motion_camera =
            get_method_addr(timeline_ctrl_class, c"AlterLateUpdate_MotionCamera", 3);
        new_hook!(
            alter_late_update_motion_camera,
            CutInTimelineController_AlterLateUpdate_MotionCamera
        );
    }
}
