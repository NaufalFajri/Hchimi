mod ApplicationSettingSaveLoader;
pub mod AudioManager;
#[cfg(target_os = "windows")]
mod BackKeyInputManager;
mod ButtonCommon;
mod CameraController;
pub mod CameraData;
mod CharacterNoteTopView;
mod CharacterNoteTopViewController;
pub mod CySpringController;
pub mod DialogCommon;
pub mod DialogCommonBase;
pub mod DialogManager;
pub mod DialogObject;
mod DialogRaceOrientation;
pub mod FlashActionPlayer;
#[cfg(target_os = "windows")]
mod GallopInput;
pub mod GallopUtil;
pub mod GameSystem;
pub mod GraphicSettings;
pub mod ImageCommon;
#[cfg(target_os = "windows")]
mod InputSystemManager;
mod JikkyoDisplay;
#[cfg(target_os = "windows")]
pub mod LandscapeUIManager;
mod LiveTheaterCharaSelect;
mod LiveTheaterViewController;
mod LiveUtil;
pub mod Localize;
mod LowResolutionCamera;
mod LyricsController;
pub mod MasterCharacterSystemText;
pub mod MasterDataUtil;
mod MasterSingleModeTurn;
pub mod Notification;
mod NowLoading;
pub mod PartsCharaMessageBase;
mod PartsSingleModeSkillLearningListItem;
pub mod PartsSingleModeSkillListItem;
pub mod RaceDefine;
pub mod RaceInfo;
pub mod RacePhaseCalculator;
mod RaceUtil;
mod SaveDataManager;
pub mod SceneManager;
pub mod Screen;
mod SingleModeMainTrainingCuttController;
pub mod SingleModeStartResultCharaViewer;
mod SingleModeUtils;
#[cfg(target_os = "windows")]
pub mod StandaloneWindowResize;
mod StoryChoiceController;
pub mod StoryRaceTextAsset;
pub mod StoryTimelineBlockData;
mod StoryTimelineCharaTrackData;
mod StoryTimelineClipData;
pub mod StoryTimelineController;
pub mod StoryTimelineData;
pub mod StoryTimelineTextClipData;
pub mod StoryTimelineTrackData;
mod StoryViewController;
pub mod StoryViewTextControllerBase;
mod StoryViewTextControllerLandscape;
mod StoryViewTextControllerSingleMode;
pub mod TapEffectController;
pub mod TextCommon;
pub mod TextDotData;
mod TextFontManager;
mod TextFormat;
pub mod TextFrame;
pub mod TextId;
mod TextMeshProUguiCommon;
pub mod TextRubyData;
mod TimeUtil;
mod TrainingParamChangeA2U;
mod TrainingParamChangePlate;
pub mod UIManager;
mod ViewControllerBase;
pub mod WebViewDefine;
pub mod WebViewManager;
#[cfg(target_os = "windows")]
pub mod WindowsGamepadControl;

#[cfg(target_os = "windows")]
mod CharacterObject;
pub mod HorseData;
pub mod HorseRaceInfo;
pub mod Jikkyo;
pub mod JikkyoControllerBase;
#[cfg(target_os = "windows")]
mod LiveModelController;
#[cfg(target_os = "windows")]
mod LiveTimelineControl;
#[cfg(target_os = "windows")]
mod LiveTimelineKeyCameraLookAtData;
#[cfg(target_os = "windows")]
pub mod LiveTimelineKeyCameraPositionData;
#[cfg(target_os = "windows")]
mod LiveTimelineKeyMultiCameraPositionData;
#[cfg(target_os = "windows")]
pub mod LiveTimelineKeyPostFilmDataList;
#[cfg(target_os = "windows")]
pub mod LiveTimelineWorkSheet;
#[cfg(target_os = "windows")]
pub mod ModelController;
#[cfg(target_os = "windows")]
mod PaymentUtility;
pub mod RaceBGMController;
#[cfg(target_os = "windows")]
mod RaceCameraEventBase;
#[cfg(target_os = "windows")]
mod RaceCameraManager;
#[cfg(target_os = "windows")]
mod RaceEffectManager;
pub mod RaceEventPlayer;
pub mod RaceHorseManagerBase;
pub mod RaceHorseManagerReplay;
pub mod RaceMainViewController;
pub mod RaceManager;
pub mod RaceManagerReplayBase;
#[cfg(target_os = "windows")]
mod RaceModelController;
pub mod RaceSimulateData;
pub mod RaceSimulateEventData;
pub mod RaceSimulateFrameData;
pub mod RaceSimulateHorseFrameData;
pub mod RaceSimulateReader;
pub mod RaceSoundReplay;
pub mod RaceUI;
pub mod RaceUIMiniMap;
#[cfg(target_os = "windows")]
pub mod RaceViewBase;
pub mod RaceViewReplay;

#[path = "SimulateEventType.rs"]
mod simulate_event_type;
pub use simulate_event_type::SimulateEventType;
#[path = "TemptationMode.rs"]
mod temptation_mode;
pub use temptation_mode::TemptationMode;

pub mod AssetManager;
#[cfg(target_os = "windows")]
mod Connecting;
mod CySpringNative;
mod DialogMissionListItem;
pub mod Director;
#[cfg(target_os = "windows")]
mod DownloadErrorProcessor;
#[cfg(target_os = "windows")]
mod DownloadManager;
pub mod DownloadPathRegister;
#[cfg(target_os = "windows")]
mod DownloadView;
pub mod GameDefine;
#[cfg(target_os = "windows")]
pub mod HomeCameraController;
pub mod HomeViewController;
pub mod HorseRaceInfoReplay;
pub mod HubViewControllerBase;
pub mod JukeboxBgmSelector;
pub mod JukeboxHomeTopUI;
mod LiveTheaterInfo;
pub mod LiveTimeController;
pub mod LiveViewController;
#[cfg(target_os = "windows")]
pub mod MainGameInitializer;
pub mod MasterDataManager;
pub mod MasterItemExchangeTop;
pub mod MasterJukeboxSetlistMusicData;
mod PartsCommonHeaderTitle;
mod PartsGetSkillPlate;
mod PartsHomeCharaMessage;
mod PartsNamePlateBase;
mod PartsNickNameListItem;
pub mod PartsNickNameRibbon;
mod PartsRaceAnalyzeRaceEventListItem;
#[cfg(target_os = "windows")]
mod PartsScheduleBookAutoPlayScreen;
mod PartsSingleModeChoiceRewardTextElementViewModel;
mod PartsSupportCardImproveDetail;
#[cfg(target_os = "windows")]
pub mod PhotoStudioCutPlayController;
pub mod SceneDefine;
pub mod SkillBase;
pub mod SkillManager;
mod StoryChoiceButton;
pub mod StoryParamChangeEffect;
pub mod TempData;
#[cfg(target_os = "windows")]
mod TitleViewController;
pub mod TweenAnimationTimelineComponent;
pub mod TweenAnimationTimelineData;
pub mod TweenAnimationTimelineSheetData;
pub mod WorkDataManager;
pub mod WorkJukeboxData;

pub fn init() {
    get_assembly_image_or_return!(image, "umamusume.dll");

    Localize::init(image);
    TextId::init(image);
    StoryRaceTextAsset::init(image);
    LyricsController::init(image);
    StoryTimelineData::init(image);
    StoryTimelineBlockData::init(image);
    StoryTimelineTrackData::init(image);
    StoryTimelineTextClipData::init(image);
    GallopUtil::init(image);
    UIManager::init(image);
    GraphicSettings::init(image);
    CameraController::init(image);
    SingleModeStartResultCharaViewer::init(image);
    WebViewManager::init(image);
    DialogCommon::init(image);
    PartsSingleModeSkillLearningListItem::init(image);
    TrainingParamChangeA2U::init(image);
    SingleModeMainTrainingCuttController::init(image);
    TextFrame::init(image);
    PartsSingleModeSkillListItem::init(image);
    FlashActionPlayer::init(image);
    TextRubyData::init(image);
    TextDotData::init(image);
    GameSystem::init(image);
    StoryViewTextControllerBase::init(image);
    StoryViewTextControllerLandscape::init(image);
    StoryViewTextControllerSingleMode::init(image);
    JikkyoDisplay::init(image);
    Screen::init(image);
    TrainingParamChangePlate::init(image);
    SingleModeUtils::init(image);
    MasterSingleModeTurn::init(image);
    TextFontManager::init(image);
    TextFormat::init(image);
    TextCommon::init(image);
    TextMeshProUguiCommon::init(image);
    StoryChoiceController::init(image);
    StoryViewController::init(image);
    StoryTimelineClipData::init(image);
    StoryTimelineCharaTrackData::init(image);
    CharacterNoteTopView::init(image);
    CharacterNoteTopViewController::init(image);
    ViewControllerBase::init(image);
    ButtonCommon::init(image);
    NowLoading::init(image);
    StoryTimelineController::init(image);
    DialogRaceOrientation::init(image);
    RaceInfo::init(image);
    RacePhaseCalculator::init(image);
    RaceUtil::init(image);
    SaveDataManager::init(image);
    ApplicationSettingSaveLoader::init(image);
    LiveTheaterCharaSelect::init(image);
    LiveTheaterViewController::init(image);
    CySpringController::init(image);
    LiveUtil::init(image);
    MasterDataUtil::init(image);
    DialogCommonBase::init(image);
    DialogObject::init(image);
    AudioManager::init(image);
    MasterCharacterSystemText::init(image);
    ImageCommon::init(image);
    Notification::init(image);
    TimeUtil::init(image);
    DialogManager::init(image);
    PartsCharaMessageBase::init(image);
    SceneManager::init(image);
    LowResolutionCamera::init(image);
    TapEffectController::init(image);

    #[cfg(target_os = "windows")]
    {
        LandscapeUIManager::init(image);
        StandaloneWindowResize::init(image);
        GallopInput::init(image);
        InputSystemManager::init(image);
        BackKeyInputManager::init(image);
        WindowsGamepadControl::init(image);
        PaymentUtility::init(image);
        Connecting::init(image);
        DownloadManager::init(image);
        DownloadView::init(image);
        DownloadErrorProcessor::init(image);
        MainGameInitializer::init(image);
        LiveTimelineControl::init(image);
        LiveTimelineWorkSheet::init(image);
        LiveTimelineKeyPostFilmDataList::init(image);
        LiveTimelineKeyCameraPositionData::init(image);
        LiveTimelineKeyCameraLookAtData::init(image);
        LiveTimelineKeyMultiCameraPositionData::init(image);
        CharacterObject::init(image);
        LiveModelController::init(image);
        ModelController::init(image);
        RaceCameraManager::init(image);
        RaceCameraEventBase::init(image);
        RaceModelController::init(image);
        RaceViewBase::init(image);
        RaceEffectManager::init(image);
        TitleViewController::init(image);
        PartsScheduleBookAutoPlayScreen::init(image);
    }
    HorseData::init(image);
    HorseRaceInfo::init(image);
    JikkyoControllerBase::init(image);
    Jikkyo::init(image);
    RaceBGMController::init(image);
    RaceMainViewController::init(image);
    RaceManager::init(image);
    RaceManagerReplayBase::init(image);
    RaceEventPlayer::init(image);
    RaceSoundReplay::init(image);
    RaceUI::init(image);
    RaceUIMiniMap::init(image);
    RaceViewReplay::init(image);
    RaceHorseManagerBase::init(image);
    RaceSimulateData::init(image);
    RaceSimulateEventData::init(image);
    RaceSimulateReader::init(image);
    RaceHorseManagerReplay::init(image);
    RaceSimulateFrameData::init(image);
    RaceSimulateHorseFrameData::init(image);
    HorseRaceInfoReplay::init(image);
    SkillManager::init(image);
    SkillBase::init(image);
    CameraData::init(image);
    TweenAnimationTimelineComponent::init(image);
    TweenAnimationTimelineData::init(image);
    TweenAnimationTimelineSheetData::init(image);
    PartsSingleModeChoiceRewardTextElementViewModel::init(image);
    PartsCommonHeaderTitle::init(image);
    StoryParamChangeEffect::init(image);
    PartsRaceAnalyzeRaceEventListItem::init(image);
    PartsNickNameRibbon::init(image);
    PartsNickNameListItem::init(image);
    PartsGetSkillPlate::init(image);
    StoryChoiceButton::init(image);
    DialogMissionListItem::init(image);
    PartsNamePlateBase::init(image);
    PartsSupportCardImproveDetail::init(image);
    Director::init(image);
    CySpringNative::init(image);
    PartsHomeCharaMessage::init(image);
    LiveViewController::init(image);
    LiveTimeController::init(image);
    HomeViewController::init(image);
    #[cfg(target_os = "windows")]
    HomeCameraController::init(image);
    #[cfg(target_os = "windows")]
    PhotoStudioCutPlayController::init(image);
    WorkDataManager::init(image);
    AssetManager::init(image);
    WorkJukeboxData::init(image);
    JukeboxBgmSelector::init(image);
    JukeboxHomeTopUI::init(image);
    TempData::init(image);
    MasterJukeboxSetlistMusicData::init(image);
    HubViewControllerBase::init(image);
    LiveTheaterInfo::init(image);
    DownloadPathRegister::init(image);
    MasterDataManager::init(image);
    MasterItemExchangeTop::init(image);
}
