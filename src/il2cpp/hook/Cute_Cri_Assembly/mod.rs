pub mod AtomSourceEx;
pub mod AudioControllerBase;
pub mod AudioManager;
pub mod AudioPlayback;
pub mod CuteAudioSource;
pub mod CuteAudioSourcePool;
mod MovieManager;

pub fn init() {
    get_assembly_image_or_return!(image, "Cute.Cri.Assembly.dll");

    MovieManager::init(image);
    AudioControllerBase::init(image);
    AtomSourceEx::init(image);
    AudioPlayback::init(image);
    AudioManager::init(image);
    CuteAudioSource::init(image);
    CuteAudioSourcePool::init(image);
}
