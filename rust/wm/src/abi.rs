// Shared native layouts and callback signatures for X11, Xt, Motif and EMWM.

pub type WmGlobalData = _WmGlobalData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmGlobalData {
    pub dataType: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub environ: *mut *mut ::core::ffi::c_char,
    pub mwmName: *mut ::core::ffi::c_char,
    pub topLevelW: Widget,
    pub topLevelW1: Widget,
    pub confirmDialogMapped: Boolean,
    pub mwmAppContext: XtAppContext,
    pub windowContextType: XContext,
    pub screenContextType: XContext,
    pub cmapWindowContextType: XContext,
    pub mwmWindowContextType: XContext,
    pub commandWindow: Window,
    pub displayString: String_0,
    pub display: *mut Display,
    pub display1: *mut Display,
    pub statusColorServer: ::core::ffi::c_int,
    pub numScreens: ::core::ffi::c_int,
    pub screenNames: *mut *mut ::core::ffi::c_uchar,
    pub Screens: *mut WmScreenData,
    pub workspaceCursor: Cursor,
    pub stretchCursors: [Cursor; 8],
    pub configCursor: Cursor,
    pub movePlacementCursor: Cursor,
    pub sizePlacementCursor: Cursor,
    pub pActiveSD: *mut WmScreenData,
    pub useStandardBehavior: Boolean,
    pub wmRestarted: Boolean,
    pub errorFlag: Boolean,
    pub errorResource: XID,
    pub errorRequestCode: ::core::ffi::c_uchar,
    pub menuActive: *mut MenuSpec,
    pub menuUnpostKeySpec: *mut KeySpec,
    pub menuClient: *mut ClientData,
    pub F_NextKeySpec: *mut KeySpec,
    pub F_PrevKeySpec: *mut KeySpec,
    pub grabContext: Context,
    pub clickData: ClickData,
    pub configAction: ::core::ffi::c_int,
    pub configButton: ::core::ffi::c_uint,
    pub configPart: ::core::ffi::c_uint,
    pub configSet: Boolean,
    pub movingIcon: Boolean,
    pub preMove: Boolean,
    pub preMoveX: ::core::ffi::c_int,
    pub preMoveY: ::core::ffi::c_int,
    pub gadgetClient: *mut ClientData,
    pub gadgetDepressed: ::core::ffi::c_int,
    pub checkHotspot: Boolean,
    pub hotspotRectangle: XRectangle,
    pub wmTimers: *mut WmTimer,
    pub passKeysActive: Boolean,
    pub passKeysKeySpec: *mut KeySpec,
    pub activeIconTextDisplayed: Boolean,
    pub queryScreen: Boolean,
    pub keyboardFocus: *mut ClientData,
    pub nextKeyboardFocus: *mut ClientData,
    pub systemModalActive: Boolean,
    pub systemModalClient: *mut ClientData,
    pub systemModalWindow: Window,
    pub clientResourceDB: XrmDatabase,
    pub dbFileName: *mut ::core::ffi::c_char,
    pub sessionClientDB: String_0,
    pub xa_WM_STATE: Atom,
    pub xa_WM_PROTOCOLS: Atom,
    pub xa_WM_CHANGE_STATE: Atom,
    pub xa_WM_SAVE_YOURSELF: Atom,
    pub xa_WM_DELETE_WINDOW: Atom,
    pub xa_WM_TAKE_FOCUS: Atom,
    pub xa_WM_COLORMAP_WINDOWS: Atom,
    pub xa_MWM_HINTS: Atom,
    pub xa_MWM_MESSAGES: Atom,
    pub xa_MWM_MENU: Atom,
    pub xa_MWM_INFO: Atom,
    pub xa_MWM_OFFSET: Atom,
    pub xa_MWM_CLIENT_LIST: Atom,
    pub xa_MOTIF_BINDINGS: Atom,
    pub xa_COMPOUND_TEXT: Atom,
    pub xa_SM_CLIENT_ID: Atom,
    pub xa_WMSAVE_HINT: Atom,
    pub xa_MWM_WM_REQUEST: Atom,
    pub xa_MWM_WORKSPACE_HINTS: Atom,
    pub xa_MWM_WORKSPACE_PRESENCE: Atom,
    pub xa_MWM_WORKSPACE_INFO: Atom,
    pub xa_MWM_WORKSPACE_LIST: Atom,
    pub xa_MWM_WORKSPACE_CURRENT: Atom,
    pub xa_ALL_WORKSPACES: Atom,
    pub autoKeyFocus: Boolean,
    pub autoRaiseDelay: ::core::ffi::c_int,
    pub bitmapDirectory: String_0,
    pub backdropDirs: String_0,
    pub clientAutoPlace: Boolean,
    pub colormapFocusPolicy: ::core::ffi::c_int,
    pub configFile: String_0,
    pub cppCommand: String_0,
    pub deiconifyKeyFocus: Boolean,
    pub doubleClickTime: ::core::ffi::c_int,
    pub enableWarp: Boolean,
    pub enforceKeyFocus: Boolean,
    pub useWindowOutline: Boolean,
    pub outlineWidth: Dimension,
    pub iconAutoPlace: Boolean,
    pub iconClick: Boolean,
    pub interactivePlacement: Boolean,
    pub keyboardFocusPolicy: ::core::ffi::c_int,
    pub lowerOnIconify: Boolean,
    pub moveThreshold: ::core::ffi::c_int,
    pub passButtonsCheck: Boolean,
    pub passButtons: Boolean,
    pub passSelectButton: Boolean,
    pub positionOnScreen: Boolean,
    pub quitTimeout: ::core::ffi::c_int,
    pub raiseKeyFocus: Boolean,
    pub multiScreen: Boolean,
    pub screenList: String_0,
    pub showFeedback: ::core::ffi::c_int,
    pub rootButtonClick: Boolean,
    pub refreshByClearing: Boolean,
    pub startupKeyFocus: Boolean,
    pub systemButtonClick: Boolean,
    pub systemButtonClick2: Boolean,
    pub useLargeCursors: Boolean,
    pub requestContextWin: Window,
    pub waitForClicks: Boolean,
    pub frameStyle: FrameStyle,
    pub evLastButton: XButtonEvent,
    pub bReplayedButton: Boolean,
    pub iconExternalShadowWidth: Dimension,
    pub frameExternalShadowWidth: Dimension,
    pub marqueeSelectGranularity: ::core::ffi::c_int,
    pub bSuspendSecondaryRestack: Boolean,
    pub clientDefaultTitle: XmString,
    pub iconDefaultTitle: XmString,
    pub attributesWindow: Window,
    pub windowAttributes: XWindowAttributes,
    pub hasShape: Boolean,
    pub shapeEventBase: ::core::ffi::c_int,
    pub shapeErrorBase: ::core::ffi::c_int,
    pub replayEnterEvent: ::core::ffi::c_int,
    pub savedEnterEvent: XEnterWindowEvent,
    pub lockingModMask: ::core::ffi::c_uint,
    pub pLockMaskSequence: *mut ::core::ffi::c_uint,
    pub tmpBuffer: [::core::ffi::c_uchar; 1024],
    pub numMouseButtons: ::core::ffi::c_int,
    pub bMenuButton: ::core::ffi::c_uint,
    pub primaryXineramaScreen: ::core::ffi::c_int,
    pub xineramaScreenFocus: ::core::ffi::c_int,
    pub xineramaIconifyToPrimary: Boolean,
    pub xrandr_present: Boolean,
    pub xrandr_base_evt: ::core::ffi::c_int,
    pub xrandr_base_err: ::core::ffi::c_int,
}

pub type FrameStyle = _FrameStyle;

pub type _FrameStyle = ::core::ffi::c_uint;

pub type WmTimer = _WmTimer;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmTimer {
    pub timerId: XtIntervalId,
    pub timerCD: *mut _ClientData,
    pub timerType: ::core::ffi::c_uint,
    pub nextWmTimer: *mut _WmTimer,
}

pub type ClickData = _ClickData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClickData {
    pub clickPending: Boolean,
    pub doubleClickPending: Boolean,
    pub bReplayed: Boolean,
    pub button: ::core::ffi::c_uint,
    pub state: ::core::ffi::c_uint,
    pub releaseState: ::core::ffi::c_uint,
    pub pCD: *mut _ClientData,
    pub context: Context,
    pub subContext: Context,
    pub clickContext: Context,
    pub doubleClickContext: Context,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FrameInfo {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub upperBorderWidth: ::core::ffi::c_uint,
    pub lowerBorderWidth: ::core::ffi::c_uint,
    pub cornerWidth: ::core::ffi::c_uint,
    pub cornerHeight: ::core::ffi::c_uint,
    pub titleBarHeight: ::core::ffi::c_uint,
}

pub type FrameInfo = _FrameInfo;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Gadget_Rectangle {
    pub id: ::core::ffi::c_short,
    pub rect: XRectangle,
}

pub type GadgetRectangle = _Gadget_Rectangle;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsPresenceData {
    pub title: XmString,
    pub shellW: Widget,
    pub formW: Widget,
    pub windowLabelW: Widget,
    pub windowNameW: Widget,
    pub workspaceLabelW: Widget,
    pub workspaceScrolledListW: Widget,
    pub workspaceListW: Widget,
    pub allWsW: Widget,
    pub sepW: Widget,
    pub OkW: Widget,
    pub CancelW: Widget,
    pub HelpW: Widget,
    pub pCDforClient: *mut _ClientData,
    pub contextForClient: Context,
    pub ItemSelected: *mut Boolean,
    pub ItemStrings: XmStringTable,
    pub currentWsItem: ::core::ffi::c_int,
    pub onScreen: Boolean,
    pub userDismissed: Boolean,
    pub numWorkspaces: ::core::ffi::c_int,
}

pub type WsPresenceData = _WsPresenceData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PixmapCache {
    pub pixmapType: ::core::ffi::c_uint,
    pub foreground: Pixel,
    pub background: Pixel,
    pub pixmap: Pixmap,
    pub next: *mut _PixmapCache,
}

pub type PixmapCache = _PixmapCache;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BitmapCache {
    pub path: *mut ::core::ffi::c_char,
    pub bitmap: Pixmap,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub pixmapCache: *mut PixmapCache,
}

pub type BitmapCache = _BitmapCache;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ButtonSpec {
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub eventType: ::core::ffi::c_uint,
    pub click: Boolean,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction,
    pub wmFuncArgs: String_0,
    pub nextButtonSpec: *mut _ButtonSpec,
}

pub type ButtonSpec = _ButtonSpec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuItem {
    pub labelType: ::core::ffi::c_int,
    pub label: String_0,
    pub labelBitmapIndex: ::core::ffi::c_int,
    pub mnemonic: KeySym,
    pub accelState: ::core::ffi::c_uint,
    pub accelKeyCode: KeyCode,
    pub accelText: String_0,
    pub wmFunction: WmFunction,
    pub wmFuncArgs: String_0,
    pub greyedContext: Context,
    pub mgtMask: ::core::ffi::c_long,
    pub nextMenuItem: *mut _MenuItem,
}

pub type MenuItem = _MenuItem;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuButton {
    pub menuItem: *mut MenuItem,
    pub buttonWidget: Widget,
    pub managed: Boolean,
}

pub type MenuButton = _MenuButton;

pub type ClientData = _ClientData;

pub type WmFunction =
    Option<unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _KeySpec {
    pub state: ::core::ffi::c_uint,
    pub keycode: KeyCode,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction,
    pub wmFuncArgs: String_0,
    pub nextKeySpec: *mut _KeySpec,
}

pub type KeySpec = _KeySpec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuSpec {
    pub name: String_0,
    pub currentContext: Context,
    pub menuWidget: Widget,
    pub whichButton: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub menuItems: *mut MenuItem,
    pub menuButtons: *mut MenuButton,
    pub menuButtonSize: ::core::ffi::c_uint,
    pub menuButtonCount: ::core::ffi::c_uint,
    pub accelContext: Context,
    pub accelKeySpecs: *mut KeySpec,
    pub nextMenuSpec: *mut _MenuSpec,
}

pub type MenuSpec = _MenuSpec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AppearanceData {
    pub renderTable: XmRenderTable,
    pub fontHeight: ::core::ffi::c_uint,
    pub titleHeight: ::core::ffi::c_uint,
    pub saveUnder: Boolean,
    pub background: Pixel,
    pub foreground: Pixel,
    pub backgroundPStr: String_0,
    pub backgroundPixmap: Pixmap,
    pub bottomShadowColor: Pixel,
    pub bottomShadowPStr: String_0,
    pub bottomShadowPixmap: Pixmap,
    pub topShadowColor: Pixel,
    pub topShadowPStr: String_0,
    pub topShadowPixmap: Pixmap,
    pub activeBackground: Pixel,
    pub activeBackgroundPStr: String_0,
    pub activeBackgroundPixmap: Pixmap,
    pub activeBottomShadowColor: Pixel,
    pub activeBottomShadowPStr: String_0,
    pub activeBottomShadowPixmap: Pixmap,
    pub activeForeground: Pixel,
    pub activeTopShadowColor: Pixel,
    pub activeTopShadowPStr: String_0,
    pub activeTopShadowPixmap: Pixmap,
    pub inactiveGC: GC,
    pub inactiveTopShadowGC: GC,
    pub inactiveBottomShadowGC: GC,
    pub activeGC: GC,
    pub activeTopShadowGC: GC,
    pub activeBottomShadowGC: GC,
}

pub type AppearanceData = _AppearanceData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientListEntry {
    pub nextSibling: *mut _ClientListEntry,
    pub prevSibling: *mut _ClientListEntry,
    pub type_0: ::core::ffi::c_int,
    pub pCD: *mut _ClientData,
}

pub type ClientListEntry = _ClientListEntry;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SessionGeom {
    pub flags: ::core::ffi::c_int,
    pub clientX: ::core::ffi::c_int,
    pub clientY: ::core::ffi::c_int,
    pub clientWidth: ::core::ffi::c_int,
    pub clientHeight: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmSessionItem {
    pub processed: Boolean,
    pub clientState: ::core::ffi::c_int,
    pub workspaces: String_0,
    pub clientMachine: String_0,
    pub command: String_0,
    pub commandArgc: ::core::ffi::c_int,
    pub commandArgv: *mut *mut ::core::ffi::c_char,
    pub sessionGeom: *mut _SessionGeom,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmBackdropData {
    pub image: String_0,
    pub nameAtom: Atom,
    pub imagePixmap: Pixmap,
    pub colorSet: ::core::ffi::c_int,
    pub background: Pixel,
    pub foreground: Pixel,
    pub defBackground: Pixel,
    pub flags: ::core::ffi::c_uint,
    pub window: Window,
}

pub type BackdropData = _WmBackdropData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmWorkspaceData {
    pub dataType: ::core::ffi::c_int,
    pub id: WorkspaceID,
    pub map_state: ::core::ffi::c_int,
    pub backdrop: BackdropData,
    pub buttonW: Widget,
    pub title: XmString,
    pub iconBoxGeometry: String_0,
    pub name: String_0,
    pub pSD: *mut WmScreenData,
    pub pIconBox: *mut IconBoxData,
    pub IPData: *mut IconPlacementData,
    pub workspaceTopLevelW: Widget,
    pub keyFocus: *mut _ClientData,
    pub nextKeyFocus: *mut _ClientData,
    pub ppClients: *mut *mut _ClientData,
    pub numClients: ::core::ffi::c_uint,
    pub sizeClientList: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmScreenData {
    pub dataType: ::core::ffi::c_int,
    pub screen: ::core::ffi::c_int,
    pub managed: Boolean,
    pub rootWindow: Window,
    pub screenTopLevelW: Widget,
    pub screenTopLevelW1: Widget,
    pub confirmboxW: [Widget; 4],
    pub presence: WsPresenceData,
    pub wmWorkspaceWin: Window,
    pub feedbackWin: Window,
    pub activeIconTextWin: Window,
    pub activeLabelParent: Window,
    pub displayString: String_0,
    pub displayResolutionType: ::core::ffi::c_int,
    pub clientCounter: ::core::ffi::c_ulong,
    pub focusPriority: ::core::ffi::c_long,
    pub inputScreenWindow: Window,
    pub colormapFocus: *mut _ClientData,
    pub workspaceColormap: Colormap,
    pub lastInstalledColormap: Colormap,
    pub pActiveWS: *mut _WmWorkspaceData,
    pub pLastWS: *mut _WmWorkspaceData,
    pub bitmapCache: *mut BitmapCache,
    pub bitmapCacheSize: ::core::ffi::c_uint,
    pub bitmapCacheCount: ::core::ffi::c_uint,
    pub fadeNormalIcon: Boolean,
    pub iconPlacement: ::core::ffi::c_int,
    pub iconPlacementMargin: ::core::ffi::c_int,
    pub iconDecoration: ::core::ffi::c_int,
    pub iconImageMaximum: WHSize,
    pub iconImageMinimum: WHSize,
    pub builtinIconPixmap: Pixmap,
    pub iconWidth: ::core::ffi::c_int,
    pub iconHeight: ::core::ffi::c_int,
    pub iconImageHeight: ::core::ffi::c_int,
    pub iconLabelHeight: ::core::ffi::c_int,
    pub shrinkWrapGC: GC,
    pub fadeIconGC: GC,
    pub fadeIconTextGC: GC,
    pub woN: Window,
    pub woS: Window,
    pub woE: Window,
    pub woW: Window,
    pub fbStyle: ::core::ffi::c_ulong,
    pub fbWinWidth: ::core::ffi::c_uint,
    pub fbWinHeight: ::core::ffi::c_uint,
    pub fbLocation: XmString,
    pub fbSize: XmString,
    pub fbLocX: ::core::ffi::c_int,
    pub fbLocY: ::core::ffi::c_int,
    pub fbSizeX: ::core::ffi::c_int,
    pub fbSizeY: ::core::ffi::c_int,
    pub fbLastX: ::core::ffi::c_int,
    pub fbLastY: ::core::ffi::c_int,
    pub fbLastWidth: ::core::ffi::c_uint,
    pub fbLastHeight: ::core::ffi::c_uint,
    pub fbTop: *mut RList,
    pub fbBottom: *mut RList,
    pub actionNbr: ::core::ffi::c_int,
    pub buttonBindings: String_0,
    pub buttonSpecs: *mut ButtonSpec,
    pub keyBindings: String_0,
    pub keySpecs: *mut KeySpec,
    pub acceleratorMenuSpecs: *mut *mut MenuSpec,
    pub acceleratorMenuCount: ::core::ffi::c_uint,
    pub menuSpecs: *mut MenuSpec,
    pub defaultSystemMenuUseBuiltin: Boolean,
    pub defaultPixmap: Pixmap,
    pub xorGC: GC,
    pub cleanText: Boolean,
    pub decoupleTitleAppearance: Boolean,
    pub frameBorderWidth: ::core::ffi::c_int,
    pub feedbackGeometry: String_0,
    pub iconBoxName: String_0,
    pub iconBoxSBDisplayPolicy: String_0,
    pub iconBoxScheme: ::core::ffi::c_int,
    pub iconBoxTitle: XmString,
    pub externalBevel: ::core::ffi::c_int,
    pub joinBevel: ::core::ffi::c_int,
    pub limitResize: Boolean,
    pub maximumMaximumSize: WHSize,
    pub resizeBorderWidth: ::core::ffi::c_int,
    pub resizeCursors: Boolean,
    pub transientDecoration: ::core::ffi::c_int,
    pub transientFunctions: ::core::ffi::c_int,
    pub utilityDecoration: ::core::ffi::c_int,
    pub utilityFunctions: ::core::ffi::c_int,
    pub useIconBox: Boolean,
    pub moveOpaque: Boolean,
    pub titleLeft: Boolean,
    pub pPixelData: *mut XmPixelSet,
    pub pActivePixelSet: *mut XmPixelSet,
    pub pInactivePixelSet: *mut XmPixelSet,
    pub pPrimaryPixelSet: *mut XmPixelSet,
    pub pSecondaryPixelSet: *mut XmPixelSet,
    pub pTextPixelSet: *mut XmPixelSet,
    pub colorUse: ::core::ffi::c_int,
    pub clientAppearance: AppearanceData,
    pub clientTitleAppearance: AppearanceData,
    pub transientOffset: XPoint,
    pub Num_Resize_Ts_Elements: ::core::ffi::c_int,
    pub Num_Resize_Bs_Elements: ::core::ffi::c_int,
    pub Num_Title_Ts_Elements: ::core::ffi::c_int,
    pub Num_Title_Bs_Elements: ::core::ffi::c_int,
    pub iconAppearance: AppearanceData,
    pub feedbackAppearance: AppearanceData,
    pub clientList: *mut ClientListEntry,
    pub lastClient: *mut ClientListEntry,
    pub pSessionItems: *mut _WmSessionItem,
    pub totalSessionItems: ::core::ffi::c_int,
    pub remainingSessionItems: ::core::ffi::c_int,
    pub initialWorkspace: String_0,
    pub workspaceList: String_0,
    pub numWorkspaces: ::core::ffi::c_int,
    pub numWsDataAllocated: ::core::ffi::c_int,
    pub pWS: *mut _WmWorkspaceData,
    pub lastBackdropWin: Window,
    pub bMarqueeSelectionInitialized: Boolean,
}

pub type WmScreenData = _WmScreenData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RList {
    pub allocated: ::core::ffi::c_int,
    pub used: ::core::ffi::c_int,
    pub prect: *mut XRectangle,
}

pub type RList = _RList;

pub type IconPlacementData = _IconPlacementData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconInfo {
    pub theWidget: Widget,
    pub pCD: *mut _ClientData,
}

pub type IconInfo = _IconInfo;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconPlacementData {
    pub onRootWindow: Boolean,
    pub iconPlacement: ::core::ffi::c_uint,
    pub placementRows: ::core::ffi::c_int,
    pub placementCols: ::core::ffi::c_int,
    pub totalPlaces: ::core::ffi::c_int,
    pub iPlaceW: ::core::ffi::c_int,
    pub iPlaceH: ::core::ffi::c_int,
    pub placeList: *mut IconInfo,
    pub placeIconX: ::core::ffi::c_int,
    pub placeIconY: ::core::ffi::c_int,
    pub placementRowY: *mut ::core::ffi::c_int,
    pub placementColX: *mut ::core::ffi::c_int,
    pub xiOrgX: ::core::ffi::c_int,
    pub xiOrgY: ::core::ffi::c_int,
    pub xiScreen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconBoxData {
    pub shellWidget: Widget,
    pub frameWidget: Widget,
    pub scrolledWidget: Widget,
    pub vScrollBar: Widget,
    pub hScrollBar: Widget,
    pub bBoardWidget: Widget,
    pub clipWidget: Widget,
    pub numberOfIcons: ::core::ffi::c_int,
    pub currentRow: ::core::ffi::c_int,
    pub currentCol: ::core::ffi::c_int,
    pub lastRow: ::core::ffi::c_int,
    pub lastCol: ::core::ffi::c_int,
    pub wsID: WorkspaceID,
    pub pCD_iconBox: *mut _ClientData,
    pub pNextIconBox: *mut _IconBoxData,
    pub IPD: _IconPlacementData,
}

pub type IconBoxData = _IconBoxData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsClientData {
    pub wsID: WorkspaceID,
    pub IPData: *mut IconPlacementData,
    pub iconPlace: ::core::ffi::c_int,
    pub iconX: ::core::ffi::c_int,
    pub iconY: ::core::ffi::c_int,
    pub iconFrameWin: Window,
    pub pIconBox: *mut IconBoxData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientData {
    pub dataType: ::core::ffi::c_int,
    pub client: Window,
    pub clientFlags: ::core::ffi::c_long,
    pub icccVersion: ::core::ffi::c_int,
    pub clientState: ::core::ffi::c_int,
    pub inputFocusModel: ::core::ffi::c_int,
    pub inputMode: ::core::ffi::c_int,
    pub focusPriority: ::core::ffi::c_long,
    pub clientID: ::core::ffi::c_ulong,
    pub wmUnmapCount: ::core::ffi::c_int,
    pub paInitialProperties: *mut Atom,
    pub numInitialProperties: ::core::ffi::c_int,
    pub clientProtocols: *mut Atom,
    pub clientProtocolCount: ::core::ffi::c_int,
    pub protocolFlags: ::core::ffi::c_long,
    pub mwmMessages: *mut ::core::ffi::c_long,
    pub mwmMessagesCount: ::core::ffi::c_int,
    pub clientColormap: Colormap,
    pub cmapWindows: *mut Window,
    pub clientCmapList: *mut Colormap,
    pub clientCmapCount: ::core::ffi::c_int,
    pub clientCmapIndex: ::core::ffi::c_int,
    pub clientCmapFlags: *mut ::core::ffi::c_int,
    pub clientCmapFlagsInitialized: ::core::ffi::c_int,
    pub clientEntry: ClientListEntry,
    pub iconEntry: ClientListEntry,
    pub windowGroup: XID,
    pub pIconBox: *mut IconBoxData,
    pub thisIconBox: *mut IconBoxData,
    pub grabContext: Context,
    pub transientFor: Window,
    pub transientLeader: *mut _ClientData,
    pub transientChildren: *mut _ClientData,
    pub transientSiblings: *mut _ClientData,
    pub primaryStackPosition: ::core::ffi::c_int,
    pub secondariesOnTop: Boolean,
    pub primaryModalCount: ::core::ffi::c_int,
    pub fullModalCount: ::core::ffi::c_int,
    pub clientClass: String_0,
    pub clientName: String_0,
    pub clientDecoration: ::core::ffi::c_int,
    pub clientFunctions: ::core::ffi::c_int,
    pub focusAutoRaise: Boolean,
    pub focusAutoRaiseDisabled: Boolean,
    pub focusAutoRaiseDisablePending: Boolean,
    pub iconImage: String_0,
    pub iconImageBackground: Pixel,
    pub iconImageBottomShadowColor: Pixel,
    pub iconImageBottomShadowPStr: String_0,
    pub iconImageBottomShadowPixmap: Pixmap,
    pub iconImageForeground: Pixel,
    pub iconImageTopShadowColor: Pixel,
    pub iconImageTopShadowPStr: String_0,
    pub iconImageTopShadowPixmap: Pixmap,
    pub ignoreWMSaveHints: Boolean,
    pub internalBevel: ::core::ffi::c_int,
    pub matteBackground: Pixel,
    pub matteBottomShadowColor: Pixel,
    pub matteBottomShadowPStr: String_0,
    pub matteBottomShadowPixmap: Pixmap,
    pub matteForeground: Pixel,
    pub matteTopShadowColor: Pixel,
    pub matteTopShadowPStr: String_0,
    pub matteTopShadowPixmap: Pixmap,
    pub matteWidth: ::core::ffi::c_int,
    pub maximumClientSize: WHSize,
    pub smClientID: String_0,
    pub systemMenu: String_0,
    pub mwmMenuItems: *mut MenuItem,
    pub systemMenuSpec: *mut MenuSpec,
    pub useClientIcon: Boolean,
    pub wmSaveHintFlags: ::core::ffi::c_int,
    pub overrideGeometry: String_0,
    pub occupyWorkspaces: String_0,
    pub sizeFlags: ::core::ffi::c_long,
    pub decor: ::core::ffi::c_long,
    pub decorFlags: ::core::ffi::c_long,
    pub minWidth: ::core::ffi::c_int,
    pub minHeight: ::core::ffi::c_int,
    pub maxConfig: Boolean,
    pub maxX: ::core::ffi::c_int,
    pub maxY: ::core::ffi::c_int,
    pub maxWidthLimit: ::core::ffi::c_int,
    pub maxWidth: ::core::ffi::c_int,
    pub maxHeightLimit: ::core::ffi::c_int,
    pub maxHeight: ::core::ffi::c_int,
    pub oldMaxWidth: ::core::ffi::c_int,
    pub oldMaxHeight: ::core::ffi::c_int,
    pub widthInc: ::core::ffi::c_int,
    pub heightInc: ::core::ffi::c_int,
    pub minAspect: AspectRatio,
    pub maxAspect: AspectRatio,
    pub baseWidth: ::core::ffi::c_int,
    pub baseHeight: ::core::ffi::c_int,
    pub windowGravity: ::core::ffi::c_int,
    pub clientX: ::core::ffi::c_int,
    pub clientY: ::core::ffi::c_int,
    pub clientWidth: ::core::ffi::c_int,
    pub clientHeight: ::core::ffi::c_int,
    pub tileState: ::core::ffi::c_int,
    pub tileApplying: Boolean,
    pub tileX: ::core::ffi::c_int,
    pub tileY: ::core::ffi::c_int,
    pub tileWidth: ::core::ffi::c_int,
    pub tileHeight: ::core::ffi::c_int,
    pub clientOffset: XPoint,
    pub clientTitle: XmString,
    pub ewmhClientTitle: XmString,
    pub desktopSurface: Boolean,
    pub argbFrame: *mut _ArgbFrame,
    pub clientFrameWin: Window,
    pub clientStretchWin: [Window; 8],
    pub clientTitleWin: Window,
    pub clientBaseWin: Window,
    pub xBorderWidth: ::core::ffi::c_int,
    pub frameInfo: FrameInfo,
    pub fullScreen: Boolean,
    pub normalClientFunctions: ::core::ffi::c_int,
    pub fullScreenX: ::core::ffi::c_int,
    pub fullScreenY: ::core::ffi::c_int,
    pub fullScreenWidth: ::core::ffi::c_int,
    pub fullScreenHeight: ::core::ffi::c_int,
    pub fullScreenXineramaIndices: [::core::ffi::c_int; 4],
    pub pclientTopShadows: *mut RList,
    pub pclientBottomShadows: *mut RList,
    pub pclientTitleTopShadows: *mut RList,
    pub pclientTitleBottomShadows: *mut RList,
    pub pclientMatteTopShadows: *mut RList,
    pub pclientMatteBottomShadows: *mut RList,
    pub titleRectangle: XRectangle,
    pub pTitleGadgets: *mut GadgetRectangle,
    pub cTitleGadgets: ::core::ffi::c_int,
    pub pResizeGadgets: *mut GadgetRectangle,
    pub matteRectangle: XRectangle,
    pub clientMatteTopShadowGC: GC,
    pub clientMatteBottomShadowGC: GC,
    pub pSD: *mut WmScreenData,
    pub iconFlags: ::core::ffi::c_long,
    pub iconTitle: XmString,
    pub ewmhIconTitle: XmString,
    pub iconPixmap: Pixmap,
    pub iconMask: Pixmap,
    pub iconWindow: Window,
    pub ewmhIconPixmap: Pixmap,
    pub piconTopShadows: *mut RList,
    pub piconBottomShadows: *mut RList,
    pub absentMapBehavior: ::core::ffi::c_int,
    pub numInhabited: ::core::ffi::c_int,
    pub sizeWsList: ::core::ffi::c_int,
    pub pWsList: *mut _WsClientData,
    pub currentWsc: ::core::ffi::c_int,
    pub pWorkspaceHints: *mut WorkspaceID,
    pub numWorkspaceHints: ::core::ffi::c_int,
    pub putInAll: Boolean,
    pub wsmFunctions: ::core::ffi::c_long,
    pub wShaped: ::core::ffi::c_short,
    pub usePPosition: ::core::ffi::c_int,
    pub window_status: ::core::ffi::c_long,
}

pub type Context = ::core::ffi::c_ulong;

pub type WorkspaceID = Atom;

pub type AspectRatio = _AspectRatio;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AspectRatio {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}

pub type WHSize = _WHSize;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WHSize {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmPixelSet {
    pub fg: Pixel,
    pub bg: Pixel,
    pub ts: Pixel,
    pub bs: Pixel,
    pub sc: Pixel,
}

pub type XmRenderTable = *mut *mut __XmRenderTableRec;

pub type XmStringTable = *mut XmString;

pub type XmString = *mut __XmStringRec;

pub type XtIntervalId = ::core::ffi::c_ulong;

pub type XtAppContext = *mut _XtAppStruct;

pub type XtProc = Option<unsafe extern "C" fn() -> ()>;

pub type XtWidgetClassProc = Option<unsafe extern "C" fn(WidgetClass) -> ()>;

pub type XtInitProc = Option<unsafe extern "C" fn(Widget, Widget, ArgList, *mut Cardinal) -> ()>;

pub type XtValueMask = ::core::ffi::c_ulong;

pub type XtRealizeProc =
    Option<unsafe extern "C" fn(Widget, *mut XtValueMask, *mut XSetWindowAttributes) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtActionsRec {
    pub string: String_0,
    pub proc_0: XtActionProc,
}

pub type XtActionList = *mut _XtActionsRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtResource {
    pub resource_name: String_0,
    pub resource_class: String_0,
    pub resource_type: String_0,
    pub resource_size: Cardinal,
    pub resource_offset: Cardinal,
    pub default_type: String_0,
    pub default_addr: XtPointer,
}

pub type XtResourceList = *mut _XtResource;

pub type XtEnum = ::core::ffi::c_uchar;

pub type XtWidgetProc = Option<unsafe extern "C" fn(Widget) -> ()>;

pub type XtExposeProc = Option<unsafe extern "C" fn(Widget, *mut XEvent, Region) -> ()>;

pub type XtSetValuesFunc =
    Option<unsafe extern "C" fn(Widget, Widget, Widget, ArgList, *mut Cardinal) -> Boolean>;

pub type XtArgsFunc = Option<unsafe extern "C" fn(Widget, ArgList, *mut Cardinal) -> Boolean>;

pub type XtAlmostProc = Option<
    unsafe extern "C" fn(Widget, Widget, *mut XtWidgetGeometry, *mut XtWidgetGeometry) -> (),
>;

pub type XtArgVal = ::core::ffi::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1_WmArgb {
    pub name: String_0,
    pub value: XtArgVal,
}

pub type ArgList = *mut C2RustUnnamed_1_WmArgb;

pub type XtArgsProc = Option<unsafe extern "C" fn(Widget, ArgList, *mut Cardinal) -> ()>;

pub type XtAcceptFocusProc = Option<unsafe extern "C" fn(Widget, *mut Time) -> Boolean>;

pub type XtVersionType = ::core::ffi::c_ulong;

pub type XtGeometryResult = ::core::ffi::c_uint;

pub type XtGeometryMask = ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XtWidgetGeometry {
    pub request_mode: XtGeometryMask,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub sibling: Widget,
    pub stack_mode: ::core::ffi::c_int,
}

pub type XtGeometryHandler = Option<
    unsafe extern "C" fn(Widget, *mut XtWidgetGeometry, *mut XtWidgetGeometry) -> XtGeometryResult,
>;

pub type XtStringProc = Option<unsafe extern "C" fn(Widget, String_0) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CoreClassPart {
    pub superclass: WidgetClass,
    pub class_name: String_0,
    pub widget_size: Cardinal,
    pub class_initialize: XtProc,
    pub class_part_initialize: XtWidgetClassProc,
    pub class_inited: XtEnum,
    pub initialize: XtInitProc,
    pub initialize_hook: XtArgsProc,
    pub realize: XtRealizeProc,
    pub actions: XtActionList,
    pub num_actions: Cardinal,
    pub resources: XtResourceList,
    pub num_resources: Cardinal,
    pub xrm_class: XrmClass,
    pub compress_motion: Boolean,
    pub compress_exposure: XtEnum,
    pub compress_enterleave: Boolean,
    pub visible_interest: Boolean,
    pub destroy: XtWidgetProc,
    pub resize: XtWidgetProc,
    pub expose: XtExposeProc,
    pub set_values: XtSetValuesFunc,
    pub set_values_hook: XtArgsFunc,
    pub set_values_almost: XtAlmostProc,
    pub get_values_hook: XtArgsProc,
    pub accept_focus: XtAcceptFocusProc,
    pub version: XtVersionType,
    pub callback_private: XtPointer,
    pub tm_table: String_0,
    pub query_geometry: XtGeometryHandler,
    pub display_accelerator: XtStringProc,
    pub extension: XtPointer,
}

pub type CoreClassPart = _CoreClassPart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetClassRec {
    pub core_class: CoreClassPart,
}

pub type WidgetClass = *mut _WidgetClassRec;

pub type XtCallbackProc = Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtCallbackRec {
    pub callback: XtCallbackProc,
    pub closure: XtPointer,
}

pub type XtCallbackList = *mut _XtCallbackRec;

pub type XtPointer = *mut ::core::ffi::c_void;

pub type Position = ::core::ffi::c_short;

pub type Dimension = ::core::ffi::c_ushort;

pub type XtEventTable = *mut _XtEventRec;

pub type XtActionProc =
    Option<unsafe extern "C" fn(Widget, *mut XEvent, *mut String_0, *mut Cardinal) -> ()>;

pub type XtBoundActions = *mut XtActionProc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtTMRec {
    pub translations: XtTranslations,
    pub proc_table: XtBoundActions,
    pub current_state: *mut _XtStateRec,
    pub lastEventTime: ::core::ffi::c_ulong,
}

pub type XtTMRec = _XtTMRec;

pub type XtTranslations = *mut _TranslationData;

pub type Widget = *mut _WidgetRec;

pub type WidgetList = *mut Widget;

pub type Cardinal = ::core::ffi::c_uint;

pub type Pixel = ::core::ffi::c_ulong;

pub type Boolean = ::core::ffi::c_char;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CorePart {
    pub self_0: Widget,
    pub widget_class: WidgetClass,
    pub parent: Widget,
    pub xrm_name: XrmName,
    pub being_destroyed: Boolean,
    pub destroy_callbacks: XtCallbackList,
    pub constraints: XtPointer,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub managed: Boolean,
    pub sensitive: Boolean,
    pub ancestor_sensitive: Boolean,
    pub event_table: XtEventTable,
    pub tm: XtTMRec,
    pub accelerators: XtTranslations,
    pub border_pixel: Pixel,
    pub border_pixmap: Pixmap,
    pub popup_list: WidgetList,
    pub num_popups: Cardinal,
    pub name: String_0,
    pub screen: *mut Screen,
    pub colormap: Colormap,
    pub window: Window,
    pub depth: Cardinal,
    pub background_pixel: Pixel,
    pub background_pixmap: Pixmap,
    pub visible: Boolean,
    pub mapped_when_managed: Boolean,
}

pub type CorePart = _CorePart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetRec {
    pub core: CorePart,
}

pub type String_0 = *mut ::core::ffi::c_char;

pub type XrmDatabase = *mut _XrmHashBucketRec;

pub type XrmClass = XrmQuark;

pub type XrmName = XrmQuark;

pub type XrmQuark = ::core::ffi::c_int;

pub type XContext = ::core::ffi::c_int;

pub type Region = *mut _XRegion;

pub type XEvent = _XEvent;

#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: ::core::ffi::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [::core::ffi::c_long; 24],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub extension: ::core::ffi::c_int,
    pub evtype: ::core::ffi::c_int,
    pub cookie: ::core::ffi::c_uint,
    pub data: *mut ::core::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub extension: ::core::ffi::c_int,
    pub evtype: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: ::core::ffi::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: ::core::ffi::c_ulong,
    pub error_code: ::core::ffi::c_uchar,
    pub request_code: ::core::ffi::c_uchar,
    pub minor_code: ::core::ffi::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: ::core::ffi::c_int,
    pub first_keycode: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0_WmArgb {
    pub b: [::core::ffi::c_char; 20],
    pub s: [::core::ffi::c_short; 10],
    pub l: [::core::ffi::c_long; 5],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: ::core::ffi::c_int,
    pub data: C2RustUnnamed_0_WmArgb,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: ::core::ffi::c_int,
    pub state: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub above: Window,
    pub detail: ::core::ffi::c_int,
    pub value_mask: ::core::ffi::c_ulong,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub above: Window,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: ::core::ffi::c_int,
    pub minor_code: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub major_code: ::core::ffi::c_int,
    pub minor_code: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [::core::ffi::c_char; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: ::core::ffi::c_int,
    pub detail: ::core::ffi::c_int,
}

pub type XEnterWindowEvent = XCrossingEvent;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub mode: ::core::ffi::c_int,
    pub detail: ::core::ffi::c_int,
    pub same_screen: ::core::ffi::c_int,
    pub focus: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub is_hint: ::core::ffi::c_char,
    pub same_screen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub same_screen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub keycode: ::core::ffi::c_uint,
    pub same_screen: ::core::ffi::c_int,
}

pub type Display = _XDisplay;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRectangle {
    pub x: ::core::ffi::c_short,
    pub y: ::core::ffi::c_short,
    pub width: ::core::ffi::c_ushort,
    pub height: ::core::ffi::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPoint {
    pub x: ::core::ffi::c_short,
    pub y: ::core::ffi::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: ::core::ffi::c_int,
    pub bit_gravity: ::core::ffi::c_int,
    pub win_gravity: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub backing_planes: ::core::ffi::c_ulong,
    pub backing_pixel: ::core::ffi::c_ulong,
    pub save_under: ::core::ffi::c_int,
    pub colormap: Colormap,
    pub map_installed: ::core::ffi::c_int,
    pub map_state: ::core::ffi::c_int,
    pub all_event_masks: ::core::ffi::c_long,
    pub your_event_mask: ::core::ffi::c_long,
    pub do_not_propagate_mask: ::core::ffi::c_long,
    pub override_redirect: ::core::ffi::c_int,
    pub screen: *mut Screen,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: ::core::ffi::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: ::core::ffi::c_ulong,
    pub bit_gravity: ::core::ffi::c_int,
    pub win_gravity: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub backing_planes: ::core::ffi::c_ulong,
    pub backing_pixel: ::core::ffi::c_ulong,
    pub save_under: ::core::ffi::c_int,
    pub event_mask: ::core::ffi::c_long,
    pub do_not_propagate_mask: ::core::ffi::c_long,
    pub override_redirect: ::core::ffi::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub mwidth: ::core::ffi::c_int,
    pub mheight: ::core::ffi::c_int,
    pub ndepths: ::core::ffi::c_int,
    pub depths: *mut Depth,
    pub root_depth: ::core::ffi::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: ::core::ffi::c_ulong,
    pub black_pixel: ::core::ffi::c_ulong,
    pub max_maps: ::core::ffi::c_int,
    pub min_maps: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub save_unders: ::core::ffi::c_int,
    pub root_input_mask: ::core::ffi::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: ::core::ffi::c_int,
    pub nvisuals: ::core::ffi::c_int,
    pub visuals: *mut Visual,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: ::core::ffi::c_int,
    pub red_mask: ::core::ffi::c_ulong,
    pub green_mask: ::core::ffi::c_ulong,
    pub blue_mask: ::core::ffi::c_ulong,
    pub bits_per_rgb: ::core::ffi::c_int,
    pub map_entries: ::core::ffi::c_int,
}

pub type GC = *mut _XGC;

pub type XExtData = _XExtData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: ::core::ffi::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> ::core::ffi::c_int>,
    pub private_data: XPointer,
}

pub type XPointer = *mut ::core::ffi::c_char;

pub type KeyCode = ::core::ffi::c_uchar;

pub type KeySym = XID;

pub type Colormap = XID;

pub type Cursor = XID;

pub type Pixmap = XID;

pub type Drawable = XID;

pub type Window = XID;

pub type Time = ::core::ffi::c_ulong;

pub type VisualID = ::core::ffi::c_ulong;

pub type Atom = ::core::ffi::c_ulong;

pub type XID = ::core::ffi::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ArgbFrame {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __XmRenderTableRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __XmStringRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XtAppStruct {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XtEventRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XtStateRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _TranslationData {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XRegion {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XrmHashBucketRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XDisplay {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XGC {
    _private: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ProxyClientInfo {
    pub screen: ::core::ffi::c_int,
    pub wmCommand: *mut ::core::ffi::c_char,
    pub wmClientMachine: *mut ::core::ffi::c_char,
    pub clientID: *mut ::core::ffi::c_char,
}

pub type ProxyClientInfo = _ProxyClientInfo;

pub type WmGlobalData_WmFeedback = _WmGlobalData_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmGlobalData_WmFeedback {
    pub dataType: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub environ: *mut *mut ::core::ffi::c_char,
    pub mwmName: *mut ::core::ffi::c_char,
    pub topLevelW: Widget_WmFeedback,
    pub topLevelW1: Widget_WmFeedback,
    pub confirmDialogMapped: Boolean,
    pub mwmAppContext: XtAppContext,
    pub windowContextType: XContext,
    pub screenContextType: XContext,
    pub cmapWindowContextType: XContext,
    pub mwmWindowContextType: XContext,
    pub commandWindow: Window,
    pub displayString: String_0,
    pub display: *mut Display,
    pub display1: *mut Display,
    pub statusColorServer: ::core::ffi::c_int,
    pub numScreens: ::core::ffi::c_int,
    pub screenNames: *mut *mut ::core::ffi::c_uchar,
    pub Screens: *mut WmScreenData_WmFeedback,
    pub workspaceCursor: Cursor,
    pub stretchCursors: [Cursor; 8],
    pub configCursor: Cursor,
    pub movePlacementCursor: Cursor,
    pub sizePlacementCursor: Cursor,
    pub pActiveSD: *mut WmScreenData_WmFeedback,
    pub useStandardBehavior: Boolean,
    pub wmRestarted: Boolean,
    pub errorFlag: Boolean,
    pub errorResource: XID,
    pub errorRequestCode: ::core::ffi::c_uchar,
    pub menuActive: *mut MenuSpec_WmFeedback,
    pub menuUnpostKeySpec: *mut KeySpec_WmFeedback,
    pub menuClient: *mut ClientData_WmFeedback,
    pub F_NextKeySpec: *mut KeySpec_WmFeedback,
    pub F_PrevKeySpec: *mut KeySpec_WmFeedback,
    pub grabContext: Context,
    pub clickData: ClickData_WmFeedback,
    pub configAction: ::core::ffi::c_int,
    pub configButton: ::core::ffi::c_uint,
    pub configPart: ::core::ffi::c_uint,
    pub configSet: Boolean,
    pub movingIcon: Boolean,
    pub preMove: Boolean,
    pub preMoveX: ::core::ffi::c_int,
    pub preMoveY: ::core::ffi::c_int,
    pub gadgetClient: *mut ClientData_WmFeedback,
    pub gadgetDepressed: ::core::ffi::c_int,
    pub checkHotspot: Boolean,
    pub hotspotRectangle: XRectangle,
    pub wmTimers: *mut WmTimer_WmFeedback,
    pub passKeysActive: Boolean,
    pub passKeysKeySpec: *mut KeySpec_WmFeedback,
    pub activeIconTextDisplayed: Boolean,
    pub queryScreen: Boolean,
    pub keyboardFocus: *mut ClientData_WmFeedback,
    pub nextKeyboardFocus: *mut ClientData_WmFeedback,
    pub systemModalActive: Boolean,
    pub systemModalClient: *mut ClientData_WmFeedback,
    pub systemModalWindow: Window,
    pub clientResourceDB: XrmDatabase,
    pub dbFileName: *mut ::core::ffi::c_char,
    pub sessionClientDB: String_0,
    pub xa_WM_STATE: Atom,
    pub xa_WM_PROTOCOLS: Atom,
    pub xa_WM_CHANGE_STATE: Atom,
    pub xa_WM_SAVE_YOURSELF: Atom,
    pub xa_WM_DELETE_WINDOW: Atom,
    pub xa_WM_TAKE_FOCUS: Atom,
    pub xa_WM_COLORMAP_WINDOWS: Atom,
    pub xa_MWM_HINTS: Atom,
    pub xa_MWM_MESSAGES: Atom,
    pub xa_MWM_MENU: Atom,
    pub xa_MWM_INFO: Atom,
    pub xa_MWM_OFFSET: Atom,
    pub xa_MWM_CLIENT_LIST: Atom,
    pub xa_MOTIF_BINDINGS: Atom,
    pub xa_COMPOUND_TEXT: Atom,
    pub xa_SM_CLIENT_ID: Atom,
    pub xa_WMSAVE_HINT: Atom,
    pub xa_MWM_WM_REQUEST: Atom,
    pub xa_MWM_WORKSPACE_HINTS: Atom,
    pub xa_MWM_WORKSPACE_PRESENCE: Atom,
    pub xa_MWM_WORKSPACE_INFO: Atom,
    pub xa_MWM_WORKSPACE_LIST: Atom,
    pub xa_MWM_WORKSPACE_CURRENT: Atom,
    pub xa_ALL_WORKSPACES: Atom,
    pub autoKeyFocus: Boolean,
    pub autoRaiseDelay: ::core::ffi::c_int,
    pub bitmapDirectory: String_0,
    pub backdropDirs: String_0,
    pub clientAutoPlace: Boolean,
    pub colormapFocusPolicy: ::core::ffi::c_int,
    pub configFile: String_0,
    pub cppCommand: String_0,
    pub deiconifyKeyFocus: Boolean,
    pub doubleClickTime: ::core::ffi::c_int,
    pub enableWarp: Boolean,
    pub enforceKeyFocus: Boolean,
    pub useWindowOutline: Boolean,
    pub outlineWidth: Dimension,
    pub iconAutoPlace: Boolean,
    pub iconClick: Boolean,
    pub interactivePlacement: Boolean,
    pub keyboardFocusPolicy: ::core::ffi::c_int,
    pub lowerOnIconify: Boolean,
    pub moveThreshold: ::core::ffi::c_int,
    pub passButtonsCheck: Boolean,
    pub passButtons: Boolean,
    pub passSelectButton: Boolean,
    pub positionOnScreen: Boolean,
    pub quitTimeout: ::core::ffi::c_int,
    pub raiseKeyFocus: Boolean,
    pub multiScreen: Boolean,
    pub screenList: String_0,
    pub showFeedback: ::core::ffi::c_int,
    pub rootButtonClick: Boolean,
    pub refreshByClearing: Boolean,
    pub startupKeyFocus: Boolean,
    pub systemButtonClick: Boolean,
    pub systemButtonClick2: Boolean,
    pub useLargeCursors: Boolean,
    pub requestContextWin: Window,
    pub waitForClicks: Boolean,
    pub frameStyle: FrameStyle,
    pub evLastButton: XButtonEvent,
    pub bReplayedButton: Boolean,
    pub iconExternalShadowWidth: Dimension,
    pub frameExternalShadowWidth: Dimension,
    pub marqueeSelectGranularity: ::core::ffi::c_int,
    pub bSuspendSecondaryRestack: Boolean,
    pub clientDefaultTitle: XmString,
    pub iconDefaultTitle: XmString,
    pub attributesWindow: Window,
    pub windowAttributes: XWindowAttributes,
    pub hasShape: Boolean,
    pub shapeEventBase: ::core::ffi::c_int,
    pub shapeErrorBase: ::core::ffi::c_int,
    pub replayEnterEvent: ::core::ffi::c_int,
    pub savedEnterEvent: XEnterWindowEvent,
    pub lockingModMask: ::core::ffi::c_uint,
    pub pLockMaskSequence: *mut ::core::ffi::c_uint,
    pub tmpBuffer: [::core::ffi::c_uchar; 1024],
    pub numMouseButtons: ::core::ffi::c_int,
    pub bMenuButton: ::core::ffi::c_uint,
    pub primaryXineramaScreen: ::core::ffi::c_int,
    pub xineramaScreenFocus: ::core::ffi::c_int,
    pub xineramaIconifyToPrimary: Boolean,
    pub xrandr_present: Boolean,
    pub xrandr_base_evt: ::core::ffi::c_int,
    pub xrandr_base_err: ::core::ffi::c_int,
}

pub type WmWorkspaceData = _WmWorkspaceData_WmFeedback;

pub type WmTimer_WmFeedback = _WmTimer_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmTimer_WmFeedback {
    pub timerId: XtIntervalId,
    pub timerCD: *mut _ClientData_WmFeedback,
    pub timerType: ::core::ffi::c_uint,
    pub nextWmTimer: *mut _WmTimer_WmFeedback,
}

pub type ClickData_WmFeedback = _ClickData_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClickData_WmFeedback {
    pub clickPending: Boolean,
    pub doubleClickPending: Boolean,
    pub bReplayed: Boolean,
    pub button: ::core::ffi::c_uint,
    pub state: ::core::ffi::c_uint,
    pub releaseState: ::core::ffi::c_uint,
    pub pCD: *mut _ClientData_WmFeedback,
    pub context: Context,
    pub subContext: Context,
    pub clickContext: Context,
    pub doubleClickContext: Context,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsPresenceData_WmFeedback {
    pub title: XmString,
    pub shellW: Widget_WmFeedback,
    pub formW: Widget_WmFeedback,
    pub windowLabelW: Widget_WmFeedback,
    pub windowNameW: Widget_WmFeedback,
    pub workspaceLabelW: Widget_WmFeedback,
    pub workspaceScrolledListW: Widget_WmFeedback,
    pub workspaceListW: Widget_WmFeedback,
    pub allWsW: Widget_WmFeedback,
    pub sepW: Widget_WmFeedback,
    pub OkW: Widget_WmFeedback,
    pub CancelW: Widget_WmFeedback,
    pub HelpW: Widget_WmFeedback,
    pub pCDforClient: *mut _ClientData_WmFeedback,
    pub contextForClient: Context,
    pub ItemSelected: *mut Boolean,
    pub ItemStrings: XmStringTable,
    pub currentWsItem: ::core::ffi::c_int,
    pub onScreen: Boolean,
    pub userDismissed: Boolean,
    pub numWorkspaces: ::core::ffi::c_int,
}

pub type WsPresenceData_WmFeedback = _WsPresenceData_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ButtonSpec_WmFeedback {
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub eventType: ::core::ffi::c_uint,
    pub click: Boolean,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction_WmFeedback,
    pub wmFuncArgs: String_0,
    pub nextButtonSpec: *mut _ButtonSpec_WmFeedback,
}

pub type ButtonSpec_WmFeedback = _ButtonSpec_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuItem_WmFeedback {
    pub labelType: ::core::ffi::c_int,
    pub label: String_0,
    pub labelBitmapIndex: ::core::ffi::c_int,
    pub mnemonic: KeySym,
    pub accelState: ::core::ffi::c_uint,
    pub accelKeyCode: KeyCode,
    pub accelText: String_0,
    pub wmFunction: WmFunction_WmFeedback,
    pub wmFuncArgs: String_0,
    pub greyedContext: Context,
    pub mgtMask: ::core::ffi::c_long,
    pub nextMenuItem: *mut _MenuItem_WmFeedback,
}

pub type MenuItem_WmFeedback = _MenuItem_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuButton_WmFeedback {
    pub menuItem: *mut MenuItem_WmFeedback,
    pub buttonWidget: Widget_WmFeedback,
    pub managed: Boolean,
}

pub type MenuButton_WmFeedback = _MenuButton_WmFeedback;

pub type ClientData_WmFeedback = _ClientData_WmFeedback;

pub type WmFunction_WmFeedback =
    Option<unsafe extern "C" fn(String_0, *mut ClientData_WmFeedback, *mut XEvent) -> Boolean>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _KeySpec_WmFeedback {
    pub state: ::core::ffi::c_uint,
    pub keycode: KeyCode,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction_WmFeedback,
    pub wmFuncArgs: String_0,
    pub nextKeySpec: *mut _KeySpec_WmFeedback,
}

pub type KeySpec_WmFeedback = _KeySpec_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuSpec_WmFeedback {
    pub name: String_0,
    pub currentContext: Context,
    pub menuWidget: Widget_WmFeedback,
    pub whichButton: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub menuItems: *mut MenuItem_WmFeedback,
    pub menuButtons: *mut MenuButton_WmFeedback,
    pub menuButtonSize: ::core::ffi::c_uint,
    pub menuButtonCount: ::core::ffi::c_uint,
    pub accelContext: Context,
    pub accelKeySpecs: *mut KeySpec_WmFeedback,
    pub nextMenuSpec: *mut _MenuSpec_WmFeedback,
}

pub type MenuSpec_WmFeedback = _MenuSpec_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientListEntry_WmFeedback {
    pub nextSibling: *mut _ClientListEntry_WmFeedback,
    pub prevSibling: *mut _ClientListEntry_WmFeedback,
    pub type_0: ::core::ffi::c_int,
    pub pCD: *mut _ClientData_WmFeedback,
}

pub type ClientListEntry_WmFeedback = _ClientListEntry_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmWorkspaceData_WmFeedback {
    pub dataType: ::core::ffi::c_int,
    pub id: WorkspaceID,
    pub map_state: ::core::ffi::c_int,
    pub backdrop: BackdropData,
    pub buttonW: Widget_WmFeedback,
    pub title: XmString,
    pub iconBoxGeometry: String_0,
    pub name: String_0,
    pub pSD: *mut WmScreenData_WmFeedback,
    pub pIconBox: *mut IconBoxData_WmFeedback,
    pub IPData: *mut IconPlacementData_WmFeedback,
    pub workspaceTopLevelW: Widget_WmFeedback,
    pub keyFocus: *mut _ClientData_WmFeedback,
    pub nextKeyFocus: *mut _ClientData_WmFeedback,
    pub ppClients: *mut *mut _ClientData_WmFeedback,
    pub numClients: ::core::ffi::c_uint,
    pub sizeClientList: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmScreenData_WmFeedback {
    pub dataType: ::core::ffi::c_int,
    pub screen: ::core::ffi::c_int,
    pub managed: Boolean,
    pub rootWindow: Window,
    pub screenTopLevelW: Widget_WmFeedback,
    pub screenTopLevelW1: Widget_WmFeedback,
    pub confirmboxW: [Widget_WmFeedback; 4],
    pub presence: WsPresenceData_WmFeedback,
    pub wmWorkspaceWin: Window,
    pub feedbackWin: Window,
    pub activeIconTextWin: Window,
    pub activeLabelParent: Window,
    pub displayString: String_0,
    pub displayResolutionType: ::core::ffi::c_int,
    pub clientCounter: ::core::ffi::c_ulong,
    pub focusPriority: ::core::ffi::c_long,
    pub inputScreenWindow: Window,
    pub colormapFocus: *mut _ClientData_WmFeedback,
    pub workspaceColormap: Colormap,
    pub lastInstalledColormap: Colormap,
    pub pActiveWS: *mut _WmWorkspaceData_WmFeedback,
    pub pLastWS: *mut _WmWorkspaceData_WmFeedback,
    pub bitmapCache: *mut BitmapCache,
    pub bitmapCacheSize: ::core::ffi::c_uint,
    pub bitmapCacheCount: ::core::ffi::c_uint,
    pub fadeNormalIcon: Boolean,
    pub iconPlacement: ::core::ffi::c_int,
    pub iconPlacementMargin: ::core::ffi::c_int,
    pub iconDecoration: ::core::ffi::c_int,
    pub iconImageMaximum: WHSize,
    pub iconImageMinimum: WHSize,
    pub builtinIconPixmap: Pixmap,
    pub iconWidth: ::core::ffi::c_int,
    pub iconHeight: ::core::ffi::c_int,
    pub iconImageHeight: ::core::ffi::c_int,
    pub iconLabelHeight: ::core::ffi::c_int,
    pub shrinkWrapGC: GC,
    pub fadeIconGC: GC,
    pub fadeIconTextGC: GC,
    pub woN: Window,
    pub woS: Window,
    pub woE: Window,
    pub woW: Window,
    pub fbStyle: ::core::ffi::c_ulong,
    pub fbWinWidth: ::core::ffi::c_uint,
    pub fbWinHeight: ::core::ffi::c_uint,
    pub fbLocation: XmString,
    pub fbSize: XmString,
    pub fbLocX: ::core::ffi::c_int,
    pub fbLocY: ::core::ffi::c_int,
    pub fbSizeX: ::core::ffi::c_int,
    pub fbSizeY: ::core::ffi::c_int,
    pub fbLastX: ::core::ffi::c_int,
    pub fbLastY: ::core::ffi::c_int,
    pub fbLastWidth: ::core::ffi::c_uint,
    pub fbLastHeight: ::core::ffi::c_uint,
    pub fbTop: *mut RList,
    pub fbBottom: *mut RList,
    pub actionNbr: ::core::ffi::c_int,
    pub buttonBindings: String_0,
    pub buttonSpecs: *mut ButtonSpec_WmFeedback,
    pub keyBindings: String_0,
    pub keySpecs: *mut KeySpec_WmFeedback,
    pub acceleratorMenuSpecs: *mut *mut MenuSpec_WmFeedback,
    pub acceleratorMenuCount: ::core::ffi::c_uint,
    pub menuSpecs: *mut MenuSpec_WmFeedback,
    pub defaultSystemMenuUseBuiltin: Boolean,
    pub defaultPixmap: Pixmap,
    pub xorGC: GC,
    pub cleanText: Boolean,
    pub decoupleTitleAppearance: Boolean,
    pub frameBorderWidth: ::core::ffi::c_int,
    pub feedbackGeometry: String_0,
    pub iconBoxName: String_0,
    pub iconBoxSBDisplayPolicy: String_0,
    pub iconBoxScheme: ::core::ffi::c_int,
    pub iconBoxTitle: XmString,
    pub externalBevel: ::core::ffi::c_int,
    pub joinBevel: ::core::ffi::c_int,
    pub limitResize: Boolean,
    pub maximumMaximumSize: WHSize,
    pub resizeBorderWidth: ::core::ffi::c_int,
    pub resizeCursors: Boolean,
    pub transientDecoration: ::core::ffi::c_int,
    pub transientFunctions: ::core::ffi::c_int,
    pub utilityDecoration: ::core::ffi::c_int,
    pub utilityFunctions: ::core::ffi::c_int,
    pub useIconBox: Boolean,
    pub moveOpaque: Boolean,
    pub titleLeft: Boolean,
    pub pPixelData: *mut XmPixelSet,
    pub pActivePixelSet: *mut XmPixelSet,
    pub pInactivePixelSet: *mut XmPixelSet,
    pub pPrimaryPixelSet: *mut XmPixelSet,
    pub pSecondaryPixelSet: *mut XmPixelSet,
    pub pTextPixelSet: *mut XmPixelSet,
    pub colorUse: ::core::ffi::c_int,
    pub clientAppearance: AppearanceData,
    pub clientTitleAppearance: AppearanceData,
    pub transientOffset: XPoint,
    pub Num_Resize_Ts_Elements: ::core::ffi::c_int,
    pub Num_Resize_Bs_Elements: ::core::ffi::c_int,
    pub Num_Title_Ts_Elements: ::core::ffi::c_int,
    pub Num_Title_Bs_Elements: ::core::ffi::c_int,
    pub iconAppearance: AppearanceData,
    pub feedbackAppearance: AppearanceData,
    pub clientList: *mut ClientListEntry_WmFeedback,
    pub lastClient: *mut ClientListEntry_WmFeedback,
    pub pSessionItems: *mut _WmSessionItem,
    pub totalSessionItems: ::core::ffi::c_int,
    pub remainingSessionItems: ::core::ffi::c_int,
    pub initialWorkspace: String_0,
    pub workspaceList: String_0,
    pub numWorkspaces: ::core::ffi::c_int,
    pub numWsDataAllocated: ::core::ffi::c_int,
    pub pWS: *mut _WmWorkspaceData_WmFeedback,
    pub lastBackdropWin: Window,
    pub bMarqueeSelectionInitialized: Boolean,
}

pub type WmScreenData_WmFeedback = _WmScreenData_WmFeedback;

pub type IconPlacementData_WmFeedback = _IconPlacementData_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconInfo_WmFeedback {
    pub theWidget: Widget_WmFeedback,
    pub pCD: *mut _ClientData_WmFeedback,
}

pub type IconInfo_WmFeedback = _IconInfo_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconPlacementData_WmFeedback {
    pub onRootWindow: Boolean,
    pub iconPlacement: ::core::ffi::c_uint,
    pub placementRows: ::core::ffi::c_int,
    pub placementCols: ::core::ffi::c_int,
    pub totalPlaces: ::core::ffi::c_int,
    pub iPlaceW: ::core::ffi::c_int,
    pub iPlaceH: ::core::ffi::c_int,
    pub placeList: *mut IconInfo_WmFeedback,
    pub placeIconX: ::core::ffi::c_int,
    pub placeIconY: ::core::ffi::c_int,
    pub placementRowY: *mut ::core::ffi::c_int,
    pub placementColX: *mut ::core::ffi::c_int,
    pub xiOrgX: ::core::ffi::c_int,
    pub xiOrgY: ::core::ffi::c_int,
    pub xiScreen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconBoxData_WmFeedback {
    pub shellWidget: Widget_WmFeedback,
    pub frameWidget: Widget_WmFeedback,
    pub scrolledWidget: Widget_WmFeedback,
    pub vScrollBar: Widget_WmFeedback,
    pub hScrollBar: Widget_WmFeedback,
    pub bBoardWidget: Widget_WmFeedback,
    pub clipWidget: Widget_WmFeedback,
    pub numberOfIcons: ::core::ffi::c_int,
    pub currentRow: ::core::ffi::c_int,
    pub currentCol: ::core::ffi::c_int,
    pub lastRow: ::core::ffi::c_int,
    pub lastCol: ::core::ffi::c_int,
    pub wsID: WorkspaceID,
    pub pCD_iconBox: *mut _ClientData_WmFeedback,
    pub pNextIconBox: *mut _IconBoxData_WmFeedback,
    pub IPD: _IconPlacementData_WmFeedback,
}

pub type IconBoxData_WmFeedback = _IconBoxData_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsClientData_WmFeedback {
    pub wsID: WorkspaceID,
    pub IPData: *mut IconPlacementData_WmFeedback,
    pub iconPlace: ::core::ffi::c_int,
    pub iconX: ::core::ffi::c_int,
    pub iconY: ::core::ffi::c_int,
    pub iconFrameWin: Window,
    pub pIconBox: *mut IconBoxData_WmFeedback,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientData_WmFeedback {
    pub dataType: ::core::ffi::c_int,
    pub client: Window,
    pub clientFlags: ::core::ffi::c_long,
    pub icccVersion: ::core::ffi::c_int,
    pub clientState: ::core::ffi::c_int,
    pub inputFocusModel: ::core::ffi::c_int,
    pub inputMode: ::core::ffi::c_int,
    pub focusPriority: ::core::ffi::c_long,
    pub clientID: ::core::ffi::c_ulong,
    pub wmUnmapCount: ::core::ffi::c_int,
    pub paInitialProperties: *mut Atom,
    pub numInitialProperties: ::core::ffi::c_int,
    pub clientProtocols: *mut Atom,
    pub clientProtocolCount: ::core::ffi::c_int,
    pub protocolFlags: ::core::ffi::c_long,
    pub mwmMessages: *mut ::core::ffi::c_long,
    pub mwmMessagesCount: ::core::ffi::c_int,
    pub clientColormap: Colormap,
    pub cmapWindows: *mut Window,
    pub clientCmapList: *mut Colormap,
    pub clientCmapCount: ::core::ffi::c_int,
    pub clientCmapIndex: ::core::ffi::c_int,
    pub clientCmapFlags: *mut ::core::ffi::c_int,
    pub clientCmapFlagsInitialized: ::core::ffi::c_int,
    pub clientEntry: ClientListEntry_WmFeedback,
    pub iconEntry: ClientListEntry_WmFeedback,
    pub windowGroup: XID,
    pub pIconBox: *mut IconBoxData_WmFeedback,
    pub thisIconBox: *mut IconBoxData_WmFeedback,
    pub grabContext: Context,
    pub transientFor: Window,
    pub transientLeader: *mut _ClientData_WmFeedback,
    pub transientChildren: *mut _ClientData_WmFeedback,
    pub transientSiblings: *mut _ClientData_WmFeedback,
    pub primaryStackPosition: ::core::ffi::c_int,
    pub secondariesOnTop: Boolean,
    pub primaryModalCount: ::core::ffi::c_int,
    pub fullModalCount: ::core::ffi::c_int,
    pub clientClass: String_0,
    pub clientName: String_0,
    pub clientDecoration: ::core::ffi::c_int,
    pub clientFunctions: ::core::ffi::c_int,
    pub focusAutoRaise: Boolean,
    pub focusAutoRaiseDisabled: Boolean,
    pub focusAutoRaiseDisablePending: Boolean,
    pub iconImage: String_0,
    pub iconImageBackground: Pixel,
    pub iconImageBottomShadowColor: Pixel,
    pub iconImageBottomShadowPStr: String_0,
    pub iconImageBottomShadowPixmap: Pixmap,
    pub iconImageForeground: Pixel,
    pub iconImageTopShadowColor: Pixel,
    pub iconImageTopShadowPStr: String_0,
    pub iconImageTopShadowPixmap: Pixmap,
    pub ignoreWMSaveHints: Boolean,
    pub internalBevel: ::core::ffi::c_int,
    pub matteBackground: Pixel,
    pub matteBottomShadowColor: Pixel,
    pub matteBottomShadowPStr: String_0,
    pub matteBottomShadowPixmap: Pixmap,
    pub matteForeground: Pixel,
    pub matteTopShadowColor: Pixel,
    pub matteTopShadowPStr: String_0,
    pub matteTopShadowPixmap: Pixmap,
    pub matteWidth: ::core::ffi::c_int,
    pub maximumClientSize: WHSize,
    pub smClientID: String_0,
    pub systemMenu: String_0,
    pub mwmMenuItems: *mut MenuItem_WmFeedback,
    pub systemMenuSpec: *mut MenuSpec_WmFeedback,
    pub useClientIcon: Boolean,
    pub wmSaveHintFlags: ::core::ffi::c_int,
    pub overrideGeometry: String_0,
    pub occupyWorkspaces: String_0,
    pub sizeFlags: ::core::ffi::c_long,
    pub decor: ::core::ffi::c_long,
    pub decorFlags: ::core::ffi::c_long,
    pub minWidth: ::core::ffi::c_int,
    pub minHeight: ::core::ffi::c_int,
    pub maxConfig: Boolean,
    pub maxX: ::core::ffi::c_int,
    pub maxY: ::core::ffi::c_int,
    pub maxWidthLimit: ::core::ffi::c_int,
    pub maxWidth: ::core::ffi::c_int,
    pub maxHeightLimit: ::core::ffi::c_int,
    pub maxHeight: ::core::ffi::c_int,
    pub oldMaxWidth: ::core::ffi::c_int,
    pub oldMaxHeight: ::core::ffi::c_int,
    pub widthInc: ::core::ffi::c_int,
    pub heightInc: ::core::ffi::c_int,
    pub minAspect: AspectRatio,
    pub maxAspect: AspectRatio,
    pub baseWidth: ::core::ffi::c_int,
    pub baseHeight: ::core::ffi::c_int,
    pub windowGravity: ::core::ffi::c_int,
    pub clientX: ::core::ffi::c_int,
    pub clientY: ::core::ffi::c_int,
    pub clientWidth: ::core::ffi::c_int,
    pub clientHeight: ::core::ffi::c_int,
    pub tileState: ::core::ffi::c_int,
    pub tileApplying: Boolean,
    pub tileX: ::core::ffi::c_int,
    pub tileY: ::core::ffi::c_int,
    pub tileWidth: ::core::ffi::c_int,
    pub tileHeight: ::core::ffi::c_int,
    pub clientOffset: XPoint,
    pub clientTitle: XmString,
    pub ewmhClientTitle: XmString,
    pub desktopSurface: Boolean,
    pub argbFrame: *mut _ArgbFrame,
    pub clientFrameWin: Window,
    pub clientStretchWin: [Window; 8],
    pub clientTitleWin: Window,
    pub clientBaseWin: Window,
    pub xBorderWidth: ::core::ffi::c_int,
    pub frameInfo: FrameInfo,
    pub fullScreen: Boolean,
    pub normalClientFunctions: ::core::ffi::c_int,
    pub fullScreenX: ::core::ffi::c_int,
    pub fullScreenY: ::core::ffi::c_int,
    pub fullScreenWidth: ::core::ffi::c_int,
    pub fullScreenHeight: ::core::ffi::c_int,
    pub fullScreenXineramaIndices: [::core::ffi::c_int; 4],
    pub pclientTopShadows: *mut RList,
    pub pclientBottomShadows: *mut RList,
    pub pclientTitleTopShadows: *mut RList,
    pub pclientTitleBottomShadows: *mut RList,
    pub pclientMatteTopShadows: *mut RList,
    pub pclientMatteBottomShadows: *mut RList,
    pub titleRectangle: XRectangle,
    pub pTitleGadgets: *mut GadgetRectangle,
    pub cTitleGadgets: ::core::ffi::c_int,
    pub pResizeGadgets: *mut GadgetRectangle,
    pub matteRectangle: XRectangle,
    pub clientMatteTopShadowGC: GC,
    pub clientMatteBottomShadowGC: GC,
    pub pSD: *mut WmScreenData_WmFeedback,
    pub iconFlags: ::core::ffi::c_long,
    pub iconTitle: XmString,
    pub ewmhIconTitle: XmString,
    pub iconPixmap: Pixmap,
    pub iconMask: Pixmap,
    pub iconWindow: Window,
    pub ewmhIconPixmap: Pixmap,
    pub piconTopShadows: *mut RList,
    pub piconBottomShadows: *mut RList,
    pub absentMapBehavior: ::core::ffi::c_int,
    pub numInhabited: ::core::ffi::c_int,
    pub sizeWsList: ::core::ffi::c_int,
    pub pWsList: *mut _WsClientData_WmFeedback,
    pub currentWsc: ::core::ffi::c_int,
    pub pWorkspaceHints: *mut WorkspaceID,
    pub numWorkspaceHints: ::core::ffi::c_int,
    pub putInAll: Boolean,
    pub wsmFunctions: ::core::ffi::c_long,
    pub wShaped: ::core::ffi::c_short,
    pub usePPosition: ::core::ffi::c_int,
    pub window_status: ::core::ffi::c_long,
}

pub type XtCheckpointToken = *mut _XtCheckpointTokenRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtCheckpointTokenRec {
    pub save_type: ::core::ffi::c_int,
    pub interact_style: ::core::ffi::c_int,
    pub shutdown: Boolean,
    pub fast: Boolean,
    pub cancel_shutdown: Boolean,
    pub phase: ::core::ffi::c_int,
    pub interact_dialog_type: ::core::ffi::c_int,
    pub request_cancel: Boolean,
    pub request_next_phase: Boolean,
    pub save_success: Boolean,
    pub type_0: ::core::ffi::c_int,
    pub widget: Widget_WmFeedback,
}

pub type XtWidgetClassProc_WmFeedback = Option<unsafe extern "C" fn(WidgetClass_WmFeedback) -> ()>;

pub type XtInitProc_WmFeedback = Option<
    unsafe extern "C" fn(
        Widget_WmFeedback,
        Widget_WmFeedback,
        ArgList_WmFeedback,
        *mut Cardinal,
    ) -> (),
>;

pub type XtRealizeProc_WmFeedback = Option<
    unsafe extern "C" fn(Widget_WmFeedback, *mut XtValueMask, *mut XSetWindowAttributes) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtActionsRec_WmFeedback {
    pub string: String_0,
    pub proc_0: XtActionProc_WmFeedback,
}

pub type XtActionList_WmFeedback = *mut _XtActionsRec_WmFeedback;

pub type XtWidgetProc_WmFeedback = Option<unsafe extern "C" fn(Widget_WmFeedback) -> ()>;

pub type XtExposeProc_WmFeedback =
    Option<unsafe extern "C" fn(Widget_WmFeedback, *mut XEvent, Region) -> ()>;

pub type XtSetValuesFunc_WmFeedback = Option<
    unsafe extern "C" fn(
        Widget_WmFeedback,
        Widget_WmFeedback,
        Widget_WmFeedback,
        ArgList_WmFeedback,
        *mut Cardinal,
    ) -> Boolean,
>;

pub type XtArgsFunc_WmFeedback =
    Option<unsafe extern "C" fn(Widget_WmFeedback, ArgList_WmFeedback, *mut Cardinal) -> Boolean>;

pub type XtAlmostProc_WmFeedback = Option<
    unsafe extern "C" fn(
        Widget_WmFeedback,
        Widget_WmFeedback,
        *mut XtWidgetGeometry_WmFeedback,
        *mut XtWidgetGeometry_WmFeedback,
    ) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg {
    pub name: String_0,
    pub value: XtArgVal,
}

pub type ArgList_WmFeedback = *mut Arg;

pub type XtArgsProc_WmFeedback =
    Option<unsafe extern "C" fn(Widget_WmFeedback, ArgList_WmFeedback, *mut Cardinal) -> ()>;

pub type XtAcceptFocusProc_WmFeedback =
    Option<unsafe extern "C" fn(Widget_WmFeedback, *mut Time) -> Boolean>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XtWidgetGeometry_WmFeedback {
    pub request_mode: XtGeometryMask,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub sibling: Widget_WmFeedback,
    pub stack_mode: ::core::ffi::c_int,
}

pub type XtGeometryHandler_WmFeedback = Option<
    unsafe extern "C" fn(
        Widget_WmFeedback,
        *mut XtWidgetGeometry_WmFeedback,
        *mut XtWidgetGeometry_WmFeedback,
    ) -> XtGeometryResult,
>;

pub type XtStringProc_WmFeedback = Option<unsafe extern "C" fn(Widget_WmFeedback, String_0) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CoreClassPart_WmFeedback {
    pub superclass: WidgetClass_WmFeedback,
    pub class_name: String_0,
    pub widget_size: Cardinal,
    pub class_initialize: XtProc,
    pub class_part_initialize: XtWidgetClassProc_WmFeedback,
    pub class_inited: XtEnum,
    pub initialize: XtInitProc_WmFeedback,
    pub initialize_hook: XtArgsProc_WmFeedback,
    pub realize: XtRealizeProc_WmFeedback,
    pub actions: XtActionList_WmFeedback,
    pub num_actions: Cardinal,
    pub resources: XtResourceList,
    pub num_resources: Cardinal,
    pub xrm_class: XrmClass,
    pub compress_motion: Boolean,
    pub compress_exposure: XtEnum,
    pub compress_enterleave: Boolean,
    pub visible_interest: Boolean,
    pub destroy: XtWidgetProc_WmFeedback,
    pub resize: XtWidgetProc_WmFeedback,
    pub expose: XtExposeProc_WmFeedback,
    pub set_values: XtSetValuesFunc_WmFeedback,
    pub set_values_hook: XtArgsFunc_WmFeedback,
    pub set_values_almost: XtAlmostProc_WmFeedback,
    pub get_values_hook: XtArgsProc_WmFeedback,
    pub accept_focus: XtAcceptFocusProc_WmFeedback,
    pub version: XtVersionType,
    pub callback_private: XtPointer,
    pub tm_table: String_0,
    pub query_geometry: XtGeometryHandler_WmFeedback,
    pub display_accelerator: XtStringProc_WmFeedback,
    pub extension: XtPointer,
}

pub type CoreClassPart_WmFeedback = _CoreClassPart_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetClassRec_WmFeedback {
    pub core_class: CoreClassPart_WmFeedback,
}

pub type WidgetClass_WmFeedback = *mut _WidgetClassRec_WmFeedback;

pub type XtCallbackProc_WmFeedback =
    Option<unsafe extern "C" fn(Widget_WmFeedback, XtPointer, XtPointer) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtCallbackRec_WmFeedback {
    pub callback: XtCallbackProc_WmFeedback,
    pub closure: XtPointer,
}

pub type XtCallbackList_WmFeedback = *mut _XtCallbackRec_WmFeedback;

pub type XtActionProc_WmFeedback = Option<
    unsafe extern "C" fn(Widget_WmFeedback, *mut XEvent, *mut String_0, *mut Cardinal) -> (),
>;

pub type XtBoundActions_WmFeedback = *mut XtActionProc_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtTMRec_WmFeedback {
    pub translations: XtTranslations,
    pub proc_table: XtBoundActions_WmFeedback,
    pub current_state: *mut _XtStateRec,
    pub lastEventTime: ::core::ffi::c_ulong,
}

pub type XtTMRec_WmFeedback = _XtTMRec_WmFeedback;

pub type Widget_WmFeedback = *mut _WidgetRec_WmFeedback;

pub type WidgetList_WmFeedback = *mut Widget_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CorePart_WmFeedback {
    pub self_0: Widget_WmFeedback,
    pub widget_class: WidgetClass_WmFeedback,
    pub parent: Widget_WmFeedback,
    pub xrm_name: XrmName,
    pub being_destroyed: Boolean,
    pub destroy_callbacks: XtCallbackList_WmFeedback,
    pub constraints: XtPointer,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub managed: Boolean,
    pub sensitive: Boolean,
    pub ancestor_sensitive: Boolean,
    pub event_table: XtEventTable,
    pub tm: XtTMRec_WmFeedback,
    pub accelerators: XtTranslations,
    pub border_pixel: Pixel,
    pub border_pixmap: Pixmap,
    pub popup_list: WidgetList_WmFeedback,
    pub num_popups: Cardinal,
    pub name: String_0,
    pub screen: *mut Screen,
    pub colormap: Colormap,
    pub window: Window,
    pub depth: Cardinal,
    pub background_pixel: Pixel,
    pub background_pixmap: Pixmap,
    pub visible: Boolean,
    pub mapped_when_managed: Boolean,
}

pub type CorePart_WmFeedback = _CorePart_WmFeedback;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetRec_WmFeedback {
    pub core: CorePart_WmFeedback,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmValue {
    pub size: ::core::ffi::c_uint,
    pub addr: XPointer,
}

pub type XrmRepresentation = XrmQuark;

pub type XrmClassList = XrmQuarkList;

pub type XrmNameList = XrmQuarkList;

pub type XrmBindingList = *mut C2RustUnnamed_3_WmEwmh;

pub type C2RustUnnamed_3_WmEwmh = ::core::ffi::c_uint;

pub type XrmQuarkList = *mut ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut ::core::ffi::c_uchar,
    pub encoding: Atom,
    pub format: ::core::ffi::c_int,
    pub nitems: ::core::ffi::c_ulong,
}

pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

pub type FILE = _IO_FILE;

pub type _IO_lock_t = ();

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}

pub type __off64_t = ::core::ffi::c_long;

pub type __off_t = ::core::ffi::c_long;

pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_marker {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_codecvt {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_wide_data {
    _private: [u8; 0],
}

pub type WsClientData = _WsClientData;

pub type WmWorkspaceData_WmBackdrop = _WmWorkspaceData;

pub type _XPrivDisplay = *mut C2RustUnnamed_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_WmArgb {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: ::core::ffi::c_int,
    pub private2: ::core::ffi::c_int,
    pub proto_major_version: ::core::ffi::c_int,
    pub proto_minor_version: ::core::ffi::c_int,
    pub vendor: *mut ::core::ffi::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: ::core::ffi::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: ::core::ffi::c_int,
    pub bitmap_unit: ::core::ffi::c_int,
    pub bitmap_pad: ::core::ffi::c_int,
    pub bitmap_bit_order: ::core::ffi::c_int,
    pub nformats: ::core::ffi::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: ::core::ffi::c_int,
    pub release: ::core::ffi::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: ::core::ffi::c_int,
    pub last_request_read: ::core::ffi::c_ulong,
    pub request: ::core::ffi::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: ::core::ffi::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> ::core::ffi::c_int>,
    pub display_name: *mut ::core::ffi::c_char,
    pub default_screen: ::core::ffi::c_int,
    pub nscreens: ::core::ffi::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: ::core::ffi::c_ulong,
    pub private16: ::core::ffi::c_ulong,
    pub min_keycode: ::core::ffi::c_int,
    pub max_keycode: ::core::ffi::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: ::core::ffi::c_int,
    pub xdefaults: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowChanges {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub sibling: Window,
    pub stack_mode: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: ::core::ffi::c_int,
    pub bits_per_pixel: ::core::ffi::c_int,
    pub scanline_pad: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XPrivate {
    _private: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: ::core::ffi::c_ulong,
    pub red: ::core::ffi::c_ushort,
    pub green: ::core::ffi::c_ushort,
    pub blue: ::core::ffi::c_ushort,
    pub flags: ::core::ffi::c_char,
    pub pad: ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XineramaScreenInfo {
    pub screen_number: ::core::ffi::c_int,
    pub x_org: ::core::ffi::c_short,
    pub y_org: ::core::ffi::c_short,
    pub width: ::core::ffi::c_short,
    pub height: ::core::ffi::c_short,
}

pub type SizeHints = _SizeHints;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SizeHints {
    pub icccVersion: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_long,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
    pub width_inc: ::core::ffi::c_int,
    pub height_inc: ::core::ffi::c_int,
    pub min_aspect: AspectRatio,
    pub max_aspect: AspectRatio,
    pub base_width: ::core::ffi::c_int,
    pub base_height: ::core::ffi::c_int,
    pub win_gravity: ::core::ffi::c_int,
}

pub type XmFontList = *mut *mut __XmRenderTableRec;

pub type PropMwmHints = PropMotifWmHints;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropMotifWmHints {
    pub flags: ::core::ffi::c_ulong,
    pub functions: ::core::ffi::c_ulong,
    pub decorations: ::core::ffi::c_ulong,
    pub inputMode: ::core::ffi::c_long,
    pub status: ::core::ffi::c_ulong,
}

pub type BITS32 = CARD32;

pub type CARD32 = ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut ::core::ffi::c_char,
    pub res_class: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: ::core::ffi::c_long,
    pub input: ::core::ffi::c_int,
    pub initial_state: ::core::ffi::c_int,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: ::core::ffi::c_int,
    pub icon_y: ::core::ffi::c_int,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ButtonAssoc {
    pub button: ::core::ffi::c_uint,
    pub maskbit: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSegment {
    pub x1: ::core::ffi::c_short,
    pub y1: ::core::ffi::c_short,
    pub x2: ::core::ffi::c_short,
    pub y2: ::core::ffi::c_short,
}

pub type RROutput = XID;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRRMonitorInfo {
    pub name: Atom,
    pub primary: ::core::ffi::c_int,
    pub automatic: ::core::ffi::c_int,
    pub noutput: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub mwidth: ::core::ffi::c_int,
    pub mheight: ::core::ffi::c_int,
    pub outputs: *mut RROutput,
}

pub type XRRMonitorInfo = _XRRMonitorInfo;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileRect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11_WmSignal {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_11_WmSignal,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10_WmSignal {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9_WmSignal {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8_WmSignal {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7_WmSignal {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6_WmSignal {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5_WmSignal {
    pub _addr_bnd: C2RustUnnamed_6_WmSignal,
    pub _pkey: __uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4_WmSignal {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2RustUnnamed_5_WmSignal,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3_WmSignal {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2_WmSignal {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1_WmSignal {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2RustUnnamed_10_WmSignal,
    pub _timer: C2RustUnnamed_9_WmSignal,
    pub _rt: C2RustUnnamed_8_WmSignal,
    pub _sigchld: C2RustUnnamed_7_WmSignal,
    pub _sigfault: C2RustUnnamed_4_WmSignal,
    pub _sigpoll: C2RustUnnamed_3_WmSignal,
    pub _sigsys: C2RustUnnamed_2_WmSignal,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2RustUnnamed_1_WmSignal,
}

pub type __sigval_t = sigval;

#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}

pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}

pub type __clock_t = ::core::ffi::c_long;

pub type __pid_t = ::core::ffi::c_int;

pub type __uid_t = ::core::ffi::c_uint;

pub type __uint32_t = u32;

pub type XmScreen = *mut _XmScreenRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmDragCursorRec {
    pub next: *mut _XmDragCursorRec,
    pub cursor: Cursor,
    pub stateIcon: XmDragIconObject,
    pub opIcon: XmDragIconObject,
    pub sourceIcon: XmDragIconObject,
    pub dirty: Boolean,
}

pub type XmDragCursorRec = _XmDragCursorRec;

pub type XmScreenColorProc = Option<
    unsafe extern "C" fn(
        *mut Screen,
        *mut XColor,
        *mut XColor,
        *mut XColor,
        *mut XColor,
        *mut XColor,
    ) -> (),
>;

pub type XmAllocColorProc =
    Option<unsafe extern "C" fn(*mut Display, Colormap, *mut XColor) -> ::core::ffi::c_int>;

pub type XmDragIconObject = *mut _XmDragIconRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmScreenPart {
    pub mwmPresent: Boolean,
    pub numReparented: ::core::ffi::c_ushort,
    pub darkThreshold: ::core::ffi::c_int,
    pub foregroundThreshold: ::core::ffi::c_int,
    pub lightThreshold: ::core::ffi::c_int,
    pub defaultNoneCursorIcon: XmDragIconObject,
    pub defaultValidCursorIcon: XmDragIconObject,
    pub defaultInvalidCursorIcon: XmDragIconObject,
    pub defaultMoveCursorIcon: XmDragIconObject,
    pub defaultCopyCursorIcon: XmDragIconObject,
    pub defaultLinkCursorIcon: XmDragIconObject,
    pub defaultSourceCursorIcon: XmDragIconObject,
    pub nullCursor: Cursor,
    pub cursorCache: *mut XmDragCursorRec,
    pub maxCursorWidth: Cardinal,
    pub maxCursorHeight: Cardinal,
    pub menuCursor: Cursor,
    pub unpostBehavior: ::core::ffi::c_uchar,
    pub font_struct: *mut XFontStruct,
    pub h_unit: ::core::ffi::c_int,
    pub v_unit: ::core::ffi::c_int,
    pub scratchPixmaps: XtPointer,
    pub moveOpaque: ::core::ffi::c_uchar,
    pub color_calc_proc: XmScreenColorProc,
    pub color_alloc_proc: XmAllocColorProc,
    pub bitmap_conversion_model: XtEnum,
    pub xmStateCursorIcon: XmDragIconObject,
    pub xmMoveCursorIcon: XmDragIconObject,
    pub xmCopyCursorIcon: XmDragIconObject,
    pub xmLinkCursorIcon: XmDragIconObject,
    pub xmSourceCursorIcon: XmDragIconObject,
    pub imageGC: GC,
    pub imageGCDepth: ::core::ffi::c_int,
    pub imageForeground: Pixel,
    pub imageBackground: Pixel,
    pub screenInfo: XtPointer,
    pub user_data: XtPointer,
    pub insensitive_stipple_bitmap: Pixmap,
    pub inUsePixmaps: XtPointer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmScreenRec {
    pub core: CorePart_WmFeedback,
    pub desktop: XmDesktopPart,
    pub screen: XmScreenPart,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmDesktopPart {
    pub parent: Widget_WmFeedback,
    pub children: *mut Widget_WmFeedback,
    pub num_children: Cardinal,
    pub num_slots: Cardinal,
}

pub type XmColorData = _XmColorData;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmColorData {
    pub screen: *mut Screen,
    pub color_map: Colormap,
    pub allocated: ::core::ffi::c_uchar,
    pub background: XColor,
    pub foreground: XColor,
    pub top_shadow: XColor,
    pub bottom_shadow: XColor,
    pub select: XColor,
}

pub type XmMergeMode = ::core::ffi::c_uint;

pub type XmRendition = *mut *mut __XmRenditionRec;

pub type XmStringTag = *mut ::core::ffi::c_char;

pub type XtResource = _XtResource;

pub type XtGCMask = ::core::ffi::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: ::core::ffi::c_uint,
    pub min_char_or_byte2: ::core::ffi::c_uint,
    pub max_char_or_byte2: ::core::ffi::c_uint,
    pub min_byte1: ::core::ffi::c_uint,
    pub max_byte1: ::core::ffi::c_uint,
    pub all_chars_exist: ::core::ffi::c_int,
    pub default_char: ::core::ffi::c_uint,
    pub n_properties: ::core::ffi::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: ::core::ffi::c_int,
    pub descent: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: ::core::ffi::c_ulong,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: ::core::ffi::c_short,
    pub rbearing: ::core::ffi::c_short,
    pub width: ::core::ffi::c_short,
    pub ascent: ::core::ffi::c_short,
    pub descent: ::core::ffi::c_short,
    pub attributes: ::core::ffi::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: ::core::ffi::c_int,
    pub plane_mask: ::core::ffi::c_ulong,
    pub foreground: ::core::ffi::c_ulong,
    pub background: ::core::ffi::c_ulong,
    pub line_width: ::core::ffi::c_int,
    pub line_style: ::core::ffi::c_int,
    pub cap_style: ::core::ffi::c_int,
    pub join_style: ::core::ffi::c_int,
    pub fill_style: ::core::ffi::c_int,
    pub fill_rule: ::core::ffi::c_int,
    pub arc_mode: ::core::ffi::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: ::core::ffi::c_int,
    pub ts_y_origin: ::core::ffi::c_int,
    pub font: Font,
    pub subwindow_mode: ::core::ffi::c_int,
    pub graphics_exposures: ::core::ffi::c_int,
    pub clip_x_origin: ::core::ffi::c_int,
    pub clip_y_origin: ::core::ffi::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: ::core::ffi::c_int,
    pub dashes: ::core::ffi::c_char,
}

pub type Font = XID;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XmDragIconRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __XmRenditionRec {
    _private: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _keySubs {
    pub pchFrom: *mut ::core::ffi::c_uchar,
    pub lenFrom: ::core::ffi::c_int,
    pub pchTo: *mut ::core::ffi::c_uchar,
}

pub type KeySub = _keySubs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MaskTableEntry {
    pub name: *mut ::core::ffi::c_char,
    pub mask: ::core::ffi::c_uint,
}

pub type EventTableParseProcT = Option<
    unsafe extern "C" fn(
        *mut *mut ::core::ffi::c_uchar,
        ::core::ffi::c_uint,
        *mut ::core::ffi::c_uint,
    ) -> Boolean,
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventTableEntry {
    pub event: *mut ::core::ffi::c_char,
    pub eventType: ::core::ffi::c_uint,
    pub parseProc: EventTableParseProcT,
    pub closure: ::core::ffi::c_uint,
    pub fClick: Boolean,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionTableEntry {
    pub funcName: *mut ::core::ffi::c_char,
    pub greyedContext: Context,
    pub resource: ::core::ffi::c_uint,
    pub mgtMask: ::core::ffi::c_long,
    pub wmFunction: WmFunction,
    pub parseProc: FunctionTableParseProcT,
}

pub type FunctionTableParseProcT = Option<
    unsafe extern "C" fn(
        *mut *mut ::core::ffi::c_uchar,
        WmFunction,
        *mut ::core::ffi::c_void,
    ) -> Boolean,
>;

pub type SessionGeom = _SessionGeom;

pub type WmSessionItem = _WmSessionItem;

pub type GroupArg = ::core::ffi::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: ::core::ffi::c_int,
    pub modifiermap: *mut KeyCode,
}

pub type wchar_t = ::libc::wchar_t;

pub type __int32_t = i32;

pub type XtConverter =
    Option<unsafe extern "C" fn(*mut XrmValue, *mut Cardinal, *mut XrmValue, *mut XrmValue) -> ()>;

pub type XtConvertArgList = *mut C2RustUnnamed_1_WmResCvt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1_WmResCvt {
    pub address_mode: XtAddressMode,
    pub address_id: XtPointer,
    pub size: Cardinal,
}

pub type XtAddressMode = ::core::ffi::c_uint;

pub type XtTimerCallbackProc = Option<unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIconSize {
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
    pub width_inc: ::core::ffi::c_int,
    pub height_inc: ::core::ffi::c_int,
}

pub type PropWMState = _PropWMState;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PropWMState {
    pub state: ::core::ffi::c_ulong,
    pub icon: ::core::ffi::c_ulong,
}

pub type PropSizeHints = _PropSizeHints;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PropSizeHints {
    pub flags: ::core::ffi::c_ulong,
    pub x: ::core::ffi::c_long,
    pub y: ::core::ffi::c_long,
    pub width: ::core::ffi::c_long,
    pub height: ::core::ffi::c_long,
    pub minWidth: ::core::ffi::c_long,
    pub minHeight: ::core::ffi::c_long,
    pub maxWidth: ::core::ffi::c_long,
    pub maxHeight: ::core::ffi::c_long,
    pub widthInc: ::core::ffi::c_long,
    pub heightInc: ::core::ffi::c_long,
    pub minAspectX: ::core::ffi::c_long,
    pub minAspectY: ::core::ffi::c_long,
    pub maxAspectX: ::core::ffi::c_long,
    pub maxAspectY: ::core::ffi::c_long,
    pub baseWidth: ::core::ffi::c_long,
    pub baseHeight: ::core::ffi::c_long,
    pub winGravity: ::core::ffi::c_long,
}

pub type PropMwmInfo = PropMotifWmInfo;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropMotifWmInfo {
    pub flags: ::core::ffi::c_long,
    pub wmWindow: Window,
}

pub type XICCEncodingStyle = ::core::ffi::c_uint;

pub type PtrWsPresenceData = *mut _WsPresenceData_WmFeedback;

pub type XmTraversalDirection = ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmListCallbackStruct {
    pub reason: ::core::ffi::c_int,
    pub event: *mut XEvent,
    pub item: XmString,
    pub item_length: ::core::ffi::c_int,
    pub item_position: ::core::ffi::c_int,
    pub selected_items: *mut XmString,
    pub selected_item_count: ::core::ffi::c_int,
    pub selected_item_positions: *mut ::core::ffi::c_int,
    pub selection_type: ::core::ffi::c_char,
    pub auto_selection_type: ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmToggleButtonCallbackStruct {
    pub reason: ::core::ffi::c_int,
    pub event: *mut XEvent,
    pub set: ::core::ffi::c_int,
}

pub type XmParseTable = *mut XmParseMapping;

pub type XmParseMapping = *mut __XmParseMappingRec;

pub type XmParseModel = ::core::ffi::c_uint;

pub type XmTextType = ::core::ffi::c_uint;

pub type XtGrabKind = ::core::ffi::c_uint;

pub type Object = *mut _ObjectRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ObjectPart {
    pub self_0: Widget_WmFeedback,
    pub widget_class: WidgetClass_WmFeedback,
    pub parent: Widget_WmFeedback,
    pub xrm_name: XrmName,
    pub being_destroyed: Boolean,
    pub destroy_callbacks: XtCallbackList_WmFeedback,
    pub constraints: XtPointer,
}

pub type ObjectPart = _ObjectPart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ObjectRec {
    pub object: ObjectPart,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __XmParseMappingRec {
    _private: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _StrList {
    pub string: XmString,
    pub next: *mut _StrList,
}

pub type StrList = _StrList;

pub type XmRowColumnWidgetClass = *mut _XmRowColumnClassRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmRowColumnClassPart {
    pub menuProcedures: XmMenuProc,
    pub armAndActivate: XtActionProc_WmFeedback,
    pub traversalHandler: XmMenuTraversalProc,
    pub extension: XtPointer,
}

pub type XmRowColumnClassPart = _XmRowColumnClassPart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmRowColumnClassRec {
    pub core_class: CoreClassPart_WmFeedback,
    pub composite_class: CompositeClassPart,
    pub constraint_class: ConstraintClassPart,
    pub manager_class: XmManagerClassPart,
    pub row_column_class: XmRowColumnClassPart,
}

pub type XmBaseClassExt = *mut _XmBaseClassExtRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmBaseClassExtRec {
    pub next_extension: XtPointer,
    pub record_type: XrmQuark,
    pub version: ::core::ffi::c_long,
    pub record_size: Cardinal,
    pub initializePrehook: XtInitProc_WmFeedback,
    pub setValuesPrehook: XtSetValuesFunc_WmFeedback,
    pub initializePosthook: XtInitProc_WmFeedback,
    pub setValuesPosthook: XtSetValuesFunc_WmFeedback,
    pub secondaryObjectClass: WidgetClass_WmFeedback,
    pub secondaryObjectCreate: XtInitProc_WmFeedback,
    pub getSecResData: XmGetSecResDataFunc,
    pub flags: [::core::ffi::c_uchar; 32],
    pub getValuesPrehook: XtArgsProc_WmFeedback,
    pub getValuesPosthook: XtArgsProc_WmFeedback,
    pub classPartInitPrehook: XtWidgetClassProc_WmFeedback,
    pub classPartInitPosthook: XtWidgetClassProc_WmFeedback,
    pub ext_resources: XtResourceList,
    pub compiled_ext_resources: XtResourceList,
    pub num_ext_resources: Cardinal,
    pub use_sub_resources: Boolean,
    pub widgetNavigable: XmWidgetNavigableProc,
    pub focusChange: XmFocusChangeProc,
    pub wrapperData: XmWrapperData,
}

pub type XmWrapperData = *mut _XmWrapperDataRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmWrapperDataRec {
    pub next: *mut _XmWrapperDataRec,
    pub widgetClass: WidgetClass_WmFeedback,
    pub initializeLeaf: XtInitProc_WmFeedback,
    pub setValuesLeaf: XtSetValuesFunc_WmFeedback,
    pub getValuesLeaf: XtArgsProc_WmFeedback,
    pub realize: XtRealizeProc_WmFeedback,
    pub classPartInitLeaf: XtWidgetClassProc_WmFeedback,
    pub resize: XtWidgetProc_WmFeedback,
    pub geometry_manager: XtGeometryHandler_WmFeedback,
    pub init_depth: Cardinal,
    pub initializeLeafCount: ::core::ffi::c_int,
    pub setValuesLeafCount: ::core::ffi::c_int,
    pub getValuesLeafCount: ::core::ffi::c_int,
    pub constraintInitializeLeaf: XtInitProc_WmFeedback,
    pub constraintSetValuesLeaf: XtSetValuesFunc_WmFeedback,
    pub constraintInitializeLeafCount: ::core::ffi::c_int,
    pub constraintSetValuesLeafCount: ::core::ffi::c_int,
}

pub type XmGenericClassExt = *mut _XmGenericClassExtRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmGenericClassExtRec {
    pub next_extension: XtPointer,
    pub record_type: XrmQuark,
    pub version: ::core::ffi::c_long,
    pub record_size: Cardinal,
}

pub type XmGetSecResDataFunc = Option<
    unsafe extern "C" fn(WidgetClass_WmFeedback, *mut *mut XmSecondaryResourceData) -> Cardinal,
>;

pub type XmFocusChangeProc = Option<unsafe extern "C" fn(Widget_WmFeedback, XmFocusChange) -> ()>;

pub type XmWidgetNavigableProc = Option<unsafe extern "C" fn(Widget_WmFeedback) -> XmNavigability>;

pub type XmMenuTraversalProc =
    Option<unsafe extern "C" fn(Widget_WmFeedback, Widget_WmFeedback, XmTraversalDirection) -> ()>;

pub type XmNavigability = ::core::ffi::c_uint;

pub type XmFocusChange = ::core::ffi::c_uint;

pub type XmMenuProc =
    Option<unsafe extern "C" fn(::core::ffi::c_int, Widget_WmFeedback, ...) -> ()>;

pub type XmSecondaryResourceData = *mut _XmSecondaryResourceDataRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmSecondaryResourceDataRec {
    pub base_proc: XmResourceBaseProc,
    pub client_data: XtPointer,
    pub name: String_0,
    pub res_class: String_0,
    pub resources: XtResourceList,
    pub num_resources: Cardinal,
}

pub type XmResourceBaseProc =
    Option<unsafe extern "C" fn(Widget_WmFeedback, XtPointer) -> XtPointer>;

pub type XmExportProc =
    Option<unsafe extern "C" fn(Widget_WmFeedback, ::core::ffi::c_int, *mut XtArgVal) -> ()>;

pub type XmImportOperator = ::core::ffi::c_uint;

pub type XmImportProc = Option<
    unsafe extern "C" fn(Widget_WmFeedback, ::core::ffi::c_int, *mut XtArgVal) -> XmImportOperator,
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmSyntheticResource {
    pub resource_name: String_0,
    pub resource_size: Cardinal,
    pub resource_offset: Cardinal,
    pub export_proc: XmExportProc,
    pub import_proc: XmImportProc,
}

pub type XmSyntheticResource = _XmSyntheticResource;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmParentProcessAnyRec {
    pub process_type: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmParentInputActionRec {
    pub process_type: ::core::ffi::c_int,
    pub event: *mut XEvent,
    pub action: ::core::ffi::c_int,
    pub params: *mut String_0,
    pub num_params: *mut Cardinal,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1_WmMenu {
    pub any: XmParentProcessAnyRec,
    pub input_action: XmParentInputActionRec,
}

pub type XmParentProcessData = *mut C2RustUnnamed_1_WmMenu;

pub type XmParentProcessProc =
    Option<unsafe extern "C" fn(Widget_WmFeedback, XmParentProcessData) -> Boolean>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmManagerClassPart {
    pub translations: String_0,
    pub syn_resources: *mut XmSyntheticResource,
    pub num_syn_resources: ::core::ffi::c_int,
    pub syn_constraint_resources: *mut XmSyntheticResource,
    pub num_syn_constraint_resources: ::core::ffi::c_int,
    pub parent_process: XmParentProcessProc,
    pub extension: XtPointer,
}

pub type XmManagerClassPart = _XmManagerClassPart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ConstraintClassPart {
    pub resources: XtResourceList,
    pub num_resources: Cardinal,
    pub constraint_size: Cardinal,
    pub initialize: XtInitProc_WmFeedback,
    pub destroy: XtWidgetProc_WmFeedback,
    pub set_values: XtSetValuesFunc_WmFeedback,
    pub extension: XtPointer,
}

pub type ConstraintClassPart = _ConstraintClassPart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CompositeClassPart {
    pub geometry_manager: XtGeometryHandler_WmFeedback,
    pub change_managed: XtWidgetProc_WmFeedback,
    pub insert_child: XtWidgetProc_WmFeedback,
    pub delete_child: XtWidgetProc_WmFeedback,
    pub extension: XtPointer,
}

pub type CompositeClassPart = _CompositeClassPart;

pub type CompositeWidget = *mut _CompositeRec;

pub type XtOrderProc = Option<unsafe extern "C" fn(Widget_WmFeedback) -> Cardinal>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CompositePart {
    pub children: WidgetList_WmFeedback,
    pub num_children: Cardinal,
    pub num_slots: Cardinal,
    pub insert_position: XtOrderProc,
}

pub type CompositePart = _CompositePart;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CompositeRec {
    pub core: CorePart_WmFeedback,
    pub composite: CompositePart,
}

pub type XButtonPressedEvent = XButtonEvent;

pub type WmGlobalData_WmArgb = _WmGlobalData_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmGlobalData_WmArgb {
    pub dataType: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub environ: *mut *mut ::core::ffi::c_char,
    pub mwmName: *mut ::core::ffi::c_char,
    pub topLevelW: Widget,
    pub topLevelW1: Widget,
    pub confirmDialogMapped: Boolean,
    pub mwmAppContext: XtAppContext,
    pub windowContextType: XContext,
    pub screenContextType: XContext,
    pub cmapWindowContextType: XContext,
    pub mwmWindowContextType: XContext,
    pub commandWindow: Window,
    pub displayString: String_0,
    pub display: *mut Display,
    pub display1: *mut Display,
    pub statusColorServer: ::core::ffi::c_int,
    pub numScreens: ::core::ffi::c_int,
    pub screenNames: *mut *mut ::core::ffi::c_uchar,
    pub Screens: *mut WmScreenData_WmArgb,
    pub workspaceCursor: Cursor,
    pub stretchCursors: [Cursor; 8],
    pub configCursor: Cursor,
    pub movePlacementCursor: Cursor,
    pub sizePlacementCursor: Cursor,
    pub pActiveSD: *mut WmScreenData_WmArgb,
    pub useStandardBehavior: Boolean,
    pub wmRestarted: Boolean,
    pub errorFlag: Boolean,
    pub errorResource: XID,
    pub errorRequestCode: ::core::ffi::c_uchar,
    pub menuActive: *mut MenuSpec_WmArgb,
    pub menuUnpostKeySpec: *mut KeySpec_WmArgb,
    pub menuClient: *mut ClientData_WmArgb,
    pub F_NextKeySpec: *mut KeySpec_WmArgb,
    pub F_PrevKeySpec: *mut KeySpec_WmArgb,
    pub grabContext: Context,
    pub clickData: ClickData_WmArgb,
    pub configAction: ::core::ffi::c_int,
    pub configButton: ::core::ffi::c_uint,
    pub configPart: ::core::ffi::c_uint,
    pub configSet: Boolean,
    pub movingIcon: Boolean,
    pub preMove: Boolean,
    pub preMoveX: ::core::ffi::c_int,
    pub preMoveY: ::core::ffi::c_int,
    pub gadgetClient: *mut ClientData_WmArgb,
    pub gadgetDepressed: ::core::ffi::c_int,
    pub checkHotspot: Boolean,
    pub hotspotRectangle: XRectangle,
    pub wmTimers: *mut WmTimer_WmArgb,
    pub passKeysActive: Boolean,
    pub passKeysKeySpec: *mut KeySpec_WmArgb,
    pub activeIconTextDisplayed: Boolean,
    pub queryScreen: Boolean,
    pub keyboardFocus: *mut ClientData_WmArgb,
    pub nextKeyboardFocus: *mut ClientData_WmArgb,
    pub systemModalActive: Boolean,
    pub systemModalClient: *mut ClientData_WmArgb,
    pub systemModalWindow: Window,
    pub clientResourceDB: XrmDatabase,
    pub dbFileName: *mut ::core::ffi::c_char,
    pub sessionClientDB: String_0,
    pub xa_WM_STATE: Atom,
    pub xa_WM_PROTOCOLS: Atom,
    pub xa_WM_CHANGE_STATE: Atom,
    pub xa_WM_SAVE_YOURSELF: Atom,
    pub xa_WM_DELETE_WINDOW: Atom,
    pub xa_WM_TAKE_FOCUS: Atom,
    pub xa_WM_COLORMAP_WINDOWS: Atom,
    pub xa_MWM_HINTS: Atom,
    pub xa_MWM_MESSAGES: Atom,
    pub xa_MWM_MENU: Atom,
    pub xa_MWM_INFO: Atom,
    pub xa_MWM_OFFSET: Atom,
    pub xa_MWM_CLIENT_LIST: Atom,
    pub xa_MOTIF_BINDINGS: Atom,
    pub xa_COMPOUND_TEXT: Atom,
    pub xa_SM_CLIENT_ID: Atom,
    pub xa_WMSAVE_HINT: Atom,
    pub xa_MWM_WM_REQUEST: Atom,
    pub xa_MWM_WORKSPACE_HINTS: Atom,
    pub xa_MWM_WORKSPACE_PRESENCE: Atom,
    pub xa_MWM_WORKSPACE_INFO: Atom,
    pub xa_MWM_WORKSPACE_LIST: Atom,
    pub xa_MWM_WORKSPACE_CURRENT: Atom,
    pub xa_ALL_WORKSPACES: Atom,
    pub autoKeyFocus: Boolean,
    pub autoRaiseDelay: ::core::ffi::c_int,
    pub bitmapDirectory: String_0,
    pub backdropDirs: String_0,
    pub clientAutoPlace: Boolean,
    pub colormapFocusPolicy: ::core::ffi::c_int,
    pub configFile: String_0,
    pub cppCommand: String_0,
    pub deiconifyKeyFocus: Boolean,
    pub doubleClickTime: ::core::ffi::c_int,
    pub enableWarp: Boolean,
    pub enforceKeyFocus: Boolean,
    pub useWindowOutline: Boolean,
    pub outlineWidth: Dimension,
    pub iconAutoPlace: Boolean,
    pub iconClick: Boolean,
    pub interactivePlacement: Boolean,
    pub keyboardFocusPolicy: ::core::ffi::c_int,
    pub lowerOnIconify: Boolean,
    pub moveThreshold: ::core::ffi::c_int,
    pub passButtonsCheck: Boolean,
    pub passButtons: Boolean,
    pub passSelectButton: Boolean,
    pub positionOnScreen: Boolean,
    pub quitTimeout: ::core::ffi::c_int,
    pub raiseKeyFocus: Boolean,
    pub multiScreen: Boolean,
    pub screenList: String_0,
    pub showFeedback: ::core::ffi::c_int,
    pub rootButtonClick: Boolean,
    pub refreshByClearing: Boolean,
    pub startupKeyFocus: Boolean,
    pub systemButtonClick: Boolean,
    pub systemButtonClick2: Boolean,
    pub useLargeCursors: Boolean,
    pub requestContextWin: Window,
    pub waitForClicks: Boolean,
    pub frameStyle: FrameStyle,
    pub evLastButton: XButtonEvent,
    pub bReplayedButton: Boolean,
    pub iconExternalShadowWidth: Dimension,
    pub frameExternalShadowWidth: Dimension,
    pub marqueeSelectGranularity: ::core::ffi::c_int,
    pub bSuspendSecondaryRestack: Boolean,
    pub clientDefaultTitle: XmString,
    pub iconDefaultTitle: XmString,
    pub attributesWindow: Window,
    pub windowAttributes: XWindowAttributes,
    pub hasShape: Boolean,
    pub shapeEventBase: ::core::ffi::c_int,
    pub shapeErrorBase: ::core::ffi::c_int,
    pub replayEnterEvent: ::core::ffi::c_int,
    pub savedEnterEvent: XEnterWindowEvent,
    pub lockingModMask: ::core::ffi::c_uint,
    pub pLockMaskSequence: *mut ::core::ffi::c_uint,
    pub tmpBuffer: [::core::ffi::c_uchar; 1024],
    pub numMouseButtons: ::core::ffi::c_int,
    pub bMenuButton: ::core::ffi::c_uint,
    pub primaryXineramaScreen: ::core::ffi::c_int,
    pub xineramaScreenFocus: ::core::ffi::c_int,
    pub xineramaIconifyToPrimary: Boolean,
    pub xrandr_present: Boolean,
    pub xrandr_base_evt: ::core::ffi::c_int,
    pub xrandr_base_err: ::core::ffi::c_int,
}

pub type WsClientData_WmManage = _WsClientData_WmArgb;

pub type WmWorkspaceData_WmManage = _WmWorkspaceData_WmArgb;

pub type WmTimer_WmArgb = _WmTimer_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmTimer_WmArgb {
    pub timerId: XtIntervalId,
    pub timerCD: *mut _ClientData_WmArgb,
    pub timerType: ::core::ffi::c_uint,
    pub nextWmTimer: *mut _WmTimer_WmArgb,
}

pub type ClickData_WmArgb = _ClickData_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClickData_WmArgb {
    pub clickPending: Boolean,
    pub doubleClickPending: Boolean,
    pub bReplayed: Boolean,
    pub button: ::core::ffi::c_uint,
    pub state: ::core::ffi::c_uint,
    pub releaseState: ::core::ffi::c_uint,
    pub pCD: *mut _ClientData_WmArgb,
    pub context: Context,
    pub subContext: Context,
    pub clickContext: Context,
    pub doubleClickContext: Context,
    pub time: Time,
}

pub type Picture = XID;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArgbBuffer {
    pub pixmap: Pixmap,
    pub source: Picture,
    pub target: Picture,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ArgbFrame_WmArgb {
    pub visual: *mut Visual,
    pub depth: ::core::ffi::c_int,
    pub colormap: Colormap,
    pub backgroundGC: GC,
    pub frame: ArgbBuffer,
    pub base: ArgbBuffer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsPresenceData_WmArgb {
    pub title: XmString,
    pub shellW: Widget,
    pub formW: Widget,
    pub windowLabelW: Widget,
    pub windowNameW: Widget,
    pub workspaceLabelW: Widget,
    pub workspaceScrolledListW: Widget,
    pub workspaceListW: Widget,
    pub allWsW: Widget,
    pub sepW: Widget,
    pub OkW: Widget,
    pub CancelW: Widget,
    pub HelpW: Widget,
    pub pCDforClient: *mut _ClientData_WmArgb,
    pub contextForClient: Context,
    pub ItemSelected: *mut Boolean,
    pub ItemStrings: XmStringTable,
    pub currentWsItem: ::core::ffi::c_int,
    pub onScreen: Boolean,
    pub userDismissed: Boolean,
    pub numWorkspaces: ::core::ffi::c_int,
}

pub type WsPresenceData_WmArgb = _WsPresenceData_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ButtonSpec_WmArgb {
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub eventType: ::core::ffi::c_uint,
    pub click: Boolean,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction_WmArgb,
    pub wmFuncArgs: String_0,
    pub nextButtonSpec: *mut _ButtonSpec_WmArgb,
}

pub type ButtonSpec_WmArgb = _ButtonSpec_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuItem_WmArgb {
    pub labelType: ::core::ffi::c_int,
    pub label: String_0,
    pub labelBitmapIndex: ::core::ffi::c_int,
    pub mnemonic: KeySym,
    pub accelState: ::core::ffi::c_uint,
    pub accelKeyCode: KeyCode,
    pub accelText: String_0,
    pub wmFunction: WmFunction_WmArgb,
    pub wmFuncArgs: String_0,
    pub greyedContext: Context,
    pub mgtMask: ::core::ffi::c_long,
    pub nextMenuItem: *mut _MenuItem_WmArgb,
}

pub type MenuItem_WmArgb = _MenuItem_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuButton_WmArgb {
    pub menuItem: *mut MenuItem_WmArgb,
    pub buttonWidget: Widget,
    pub managed: Boolean,
}

pub type MenuButton_WmArgb = _MenuButton_WmArgb;

pub type ClientData_WmArgb = _ClientData_WmArgb;

pub type WmFunction_WmArgb =
    Option<unsafe extern "C" fn(String_0, *mut ClientData_WmArgb, *mut XEvent) -> Boolean>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _KeySpec_WmArgb {
    pub state: ::core::ffi::c_uint,
    pub keycode: KeyCode,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction_WmArgb,
    pub wmFuncArgs: String_0,
    pub nextKeySpec: *mut _KeySpec_WmArgb,
}

pub type KeySpec_WmArgb = _KeySpec_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuSpec_WmArgb {
    pub name: String_0,
    pub currentContext: Context,
    pub menuWidget: Widget,
    pub whichButton: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub menuItems: *mut MenuItem_WmArgb,
    pub menuButtons: *mut MenuButton_WmArgb,
    pub menuButtonSize: ::core::ffi::c_uint,
    pub menuButtonCount: ::core::ffi::c_uint,
    pub accelContext: Context,
    pub accelKeySpecs: *mut KeySpec_WmArgb,
    pub nextMenuSpec: *mut _MenuSpec_WmArgb,
}

pub type MenuSpec_WmArgb = _MenuSpec_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientListEntry_WmArgb {
    pub nextSibling: *mut _ClientListEntry_WmArgb,
    pub prevSibling: *mut _ClientListEntry_WmArgb,
    pub type_0: ::core::ffi::c_int,
    pub pCD: *mut _ClientData_WmArgb,
}

pub type ClientListEntry_WmArgb = _ClientListEntry_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmWorkspaceData_WmArgb {
    pub dataType: ::core::ffi::c_int,
    pub id: WorkspaceID,
    pub map_state: ::core::ffi::c_int,
    pub backdrop: BackdropData,
    pub buttonW: Widget,
    pub title: XmString,
    pub iconBoxGeometry: String_0,
    pub name: String_0,
    pub pSD: *mut WmScreenData_WmArgb,
    pub pIconBox: *mut IconBoxData_WmArgb,
    pub IPData: *mut IconPlacementData_WmArgb,
    pub workspaceTopLevelW: Widget,
    pub keyFocus: *mut _ClientData_WmArgb,
    pub nextKeyFocus: *mut _ClientData_WmArgb,
    pub ppClients: *mut *mut _ClientData_WmArgb,
    pub numClients: ::core::ffi::c_uint,
    pub sizeClientList: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmScreenData_WmArgb {
    pub dataType: ::core::ffi::c_int,
    pub screen: ::core::ffi::c_int,
    pub managed: Boolean,
    pub rootWindow: Window,
    pub screenTopLevelW: Widget,
    pub screenTopLevelW1: Widget,
    pub confirmboxW: [Widget; 4],
    pub presence: WsPresenceData_WmArgb,
    pub wmWorkspaceWin: Window,
    pub feedbackWin: Window,
    pub activeIconTextWin: Window,
    pub activeLabelParent: Window,
    pub displayString: String_0,
    pub displayResolutionType: ::core::ffi::c_int,
    pub clientCounter: ::core::ffi::c_ulong,
    pub focusPriority: ::core::ffi::c_long,
    pub inputScreenWindow: Window,
    pub colormapFocus: *mut _ClientData_WmArgb,
    pub workspaceColormap: Colormap,
    pub lastInstalledColormap: Colormap,
    pub pActiveWS: *mut _WmWorkspaceData_WmArgb,
    pub pLastWS: *mut _WmWorkspaceData_WmArgb,
    pub bitmapCache: *mut BitmapCache,
    pub bitmapCacheSize: ::core::ffi::c_uint,
    pub bitmapCacheCount: ::core::ffi::c_uint,
    pub fadeNormalIcon: Boolean,
    pub iconPlacement: ::core::ffi::c_int,
    pub iconPlacementMargin: ::core::ffi::c_int,
    pub iconDecoration: ::core::ffi::c_int,
    pub iconImageMaximum: WHSize,
    pub iconImageMinimum: WHSize,
    pub builtinIconPixmap: Pixmap,
    pub iconWidth: ::core::ffi::c_int,
    pub iconHeight: ::core::ffi::c_int,
    pub iconImageHeight: ::core::ffi::c_int,
    pub iconLabelHeight: ::core::ffi::c_int,
    pub shrinkWrapGC: GC,
    pub fadeIconGC: GC,
    pub fadeIconTextGC: GC,
    pub woN: Window,
    pub woS: Window,
    pub woE: Window,
    pub woW: Window,
    pub fbStyle: ::core::ffi::c_ulong,
    pub fbWinWidth: ::core::ffi::c_uint,
    pub fbWinHeight: ::core::ffi::c_uint,
    pub fbLocation: XmString,
    pub fbSize: XmString,
    pub fbLocX: ::core::ffi::c_int,
    pub fbLocY: ::core::ffi::c_int,
    pub fbSizeX: ::core::ffi::c_int,
    pub fbSizeY: ::core::ffi::c_int,
    pub fbLastX: ::core::ffi::c_int,
    pub fbLastY: ::core::ffi::c_int,
    pub fbLastWidth: ::core::ffi::c_uint,
    pub fbLastHeight: ::core::ffi::c_uint,
    pub fbTop: *mut RList,
    pub fbBottom: *mut RList,
    pub actionNbr: ::core::ffi::c_int,
    pub buttonBindings: String_0,
    pub buttonSpecs: *mut ButtonSpec_WmArgb,
    pub keyBindings: String_0,
    pub keySpecs: *mut KeySpec_WmArgb,
    pub acceleratorMenuSpecs: *mut *mut MenuSpec_WmArgb,
    pub acceleratorMenuCount: ::core::ffi::c_uint,
    pub menuSpecs: *mut MenuSpec_WmArgb,
    pub defaultSystemMenuUseBuiltin: Boolean,
    pub defaultPixmap: Pixmap,
    pub xorGC: GC,
    pub cleanText: Boolean,
    pub decoupleTitleAppearance: Boolean,
    pub frameBorderWidth: ::core::ffi::c_int,
    pub feedbackGeometry: String_0,
    pub iconBoxName: String_0,
    pub iconBoxSBDisplayPolicy: String_0,
    pub iconBoxScheme: ::core::ffi::c_int,
    pub iconBoxTitle: XmString,
    pub externalBevel: ::core::ffi::c_int,
    pub joinBevel: ::core::ffi::c_int,
    pub limitResize: Boolean,
    pub maximumMaximumSize: WHSize,
    pub resizeBorderWidth: ::core::ffi::c_int,
    pub resizeCursors: Boolean,
    pub transientDecoration: ::core::ffi::c_int,
    pub transientFunctions: ::core::ffi::c_int,
    pub utilityDecoration: ::core::ffi::c_int,
    pub utilityFunctions: ::core::ffi::c_int,
    pub useIconBox: Boolean,
    pub moveOpaque: Boolean,
    pub titleLeft: Boolean,
    pub pPixelData: *mut XmPixelSet,
    pub pActivePixelSet: *mut XmPixelSet,
    pub pInactivePixelSet: *mut XmPixelSet,
    pub pPrimaryPixelSet: *mut XmPixelSet,
    pub pSecondaryPixelSet: *mut XmPixelSet,
    pub pTextPixelSet: *mut XmPixelSet,
    pub colorUse: ::core::ffi::c_int,
    pub clientAppearance: AppearanceData,
    pub clientTitleAppearance: AppearanceData,
    pub transientOffset: XPoint,
    pub Num_Resize_Ts_Elements: ::core::ffi::c_int,
    pub Num_Resize_Bs_Elements: ::core::ffi::c_int,
    pub Num_Title_Ts_Elements: ::core::ffi::c_int,
    pub Num_Title_Bs_Elements: ::core::ffi::c_int,
    pub iconAppearance: AppearanceData,
    pub feedbackAppearance: AppearanceData,
    pub clientList: *mut ClientListEntry_WmArgb,
    pub lastClient: *mut ClientListEntry_WmArgb,
    pub pSessionItems: *mut _WmSessionItem,
    pub totalSessionItems: ::core::ffi::c_int,
    pub remainingSessionItems: ::core::ffi::c_int,
    pub initialWorkspace: String_0,
    pub workspaceList: String_0,
    pub numWorkspaces: ::core::ffi::c_int,
    pub numWsDataAllocated: ::core::ffi::c_int,
    pub pWS: *mut _WmWorkspaceData_WmArgb,
    pub lastBackdropWin: Window,
    pub bMarqueeSelectionInitialized: Boolean,
}

pub type WmScreenData_WmArgb = _WmScreenData_WmArgb;

pub type IconPlacementData_WmArgb = _IconPlacementData_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconInfo_WmArgb {
    pub theWidget: Widget,
    pub pCD: *mut _ClientData_WmArgb,
}

pub type IconInfo_WmArgb = _IconInfo_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconPlacementData_WmArgb {
    pub onRootWindow: Boolean,
    pub iconPlacement: ::core::ffi::c_uint,
    pub placementRows: ::core::ffi::c_int,
    pub placementCols: ::core::ffi::c_int,
    pub totalPlaces: ::core::ffi::c_int,
    pub iPlaceW: ::core::ffi::c_int,
    pub iPlaceH: ::core::ffi::c_int,
    pub placeList: *mut IconInfo_WmArgb,
    pub placeIconX: ::core::ffi::c_int,
    pub placeIconY: ::core::ffi::c_int,
    pub placementRowY: *mut ::core::ffi::c_int,
    pub placementColX: *mut ::core::ffi::c_int,
    pub xiOrgX: ::core::ffi::c_int,
    pub xiOrgY: ::core::ffi::c_int,
    pub xiScreen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconBoxData_WmArgb {
    pub shellWidget: Widget,
    pub frameWidget: Widget,
    pub scrolledWidget: Widget,
    pub vScrollBar: Widget,
    pub hScrollBar: Widget,
    pub bBoardWidget: Widget,
    pub clipWidget: Widget,
    pub numberOfIcons: ::core::ffi::c_int,
    pub currentRow: ::core::ffi::c_int,
    pub currentCol: ::core::ffi::c_int,
    pub lastRow: ::core::ffi::c_int,
    pub lastCol: ::core::ffi::c_int,
    pub wsID: WorkspaceID,
    pub pCD_iconBox: *mut _ClientData_WmArgb,
    pub pNextIconBox: *mut _IconBoxData_WmArgb,
    pub IPD: _IconPlacementData_WmArgb,
}

pub type IconBoxData_WmArgb = _IconBoxData_WmArgb;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsClientData_WmArgb {
    pub wsID: WorkspaceID,
    pub IPData: *mut IconPlacementData_WmArgb,
    pub iconPlace: ::core::ffi::c_int,
    pub iconX: ::core::ffi::c_int,
    pub iconY: ::core::ffi::c_int,
    pub iconFrameWin: Window,
    pub pIconBox: *mut IconBoxData_WmArgb,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientData_WmArgb {
    pub dataType: ::core::ffi::c_int,
    pub client: Window,
    pub clientFlags: ::core::ffi::c_long,
    pub icccVersion: ::core::ffi::c_int,
    pub clientState: ::core::ffi::c_int,
    pub inputFocusModel: ::core::ffi::c_int,
    pub inputMode: ::core::ffi::c_int,
    pub focusPriority: ::core::ffi::c_long,
    pub clientID: ::core::ffi::c_ulong,
    pub wmUnmapCount: ::core::ffi::c_int,
    pub paInitialProperties: *mut Atom,
    pub numInitialProperties: ::core::ffi::c_int,
    pub clientProtocols: *mut Atom,
    pub clientProtocolCount: ::core::ffi::c_int,
    pub protocolFlags: ::core::ffi::c_long,
    pub mwmMessages: *mut ::core::ffi::c_long,
    pub mwmMessagesCount: ::core::ffi::c_int,
    pub clientColormap: Colormap,
    pub cmapWindows: *mut Window,
    pub clientCmapList: *mut Colormap,
    pub clientCmapCount: ::core::ffi::c_int,
    pub clientCmapIndex: ::core::ffi::c_int,
    pub clientCmapFlags: *mut ::core::ffi::c_int,
    pub clientCmapFlagsInitialized: ::core::ffi::c_int,
    pub clientEntry: ClientListEntry_WmArgb,
    pub iconEntry: ClientListEntry_WmArgb,
    pub windowGroup: XID,
    pub pIconBox: *mut IconBoxData_WmArgb,
    pub thisIconBox: *mut IconBoxData_WmArgb,
    pub grabContext: Context,
    pub transientFor: Window,
    pub transientLeader: *mut _ClientData_WmArgb,
    pub transientChildren: *mut _ClientData_WmArgb,
    pub transientSiblings: *mut _ClientData_WmArgb,
    pub primaryStackPosition: ::core::ffi::c_int,
    pub secondariesOnTop: Boolean,
    pub primaryModalCount: ::core::ffi::c_int,
    pub fullModalCount: ::core::ffi::c_int,
    pub clientClass: String_0,
    pub clientName: String_0,
    pub clientDecoration: ::core::ffi::c_int,
    pub clientFunctions: ::core::ffi::c_int,
    pub focusAutoRaise: Boolean,
    pub focusAutoRaiseDisabled: Boolean,
    pub focusAutoRaiseDisablePending: Boolean,
    pub iconImage: String_0,
    pub iconImageBackground: Pixel,
    pub iconImageBottomShadowColor: Pixel,
    pub iconImageBottomShadowPStr: String_0,
    pub iconImageBottomShadowPixmap: Pixmap,
    pub iconImageForeground: Pixel,
    pub iconImageTopShadowColor: Pixel,
    pub iconImageTopShadowPStr: String_0,
    pub iconImageTopShadowPixmap: Pixmap,
    pub ignoreWMSaveHints: Boolean,
    pub internalBevel: ::core::ffi::c_int,
    pub matteBackground: Pixel,
    pub matteBottomShadowColor: Pixel,
    pub matteBottomShadowPStr: String_0,
    pub matteBottomShadowPixmap: Pixmap,
    pub matteForeground: Pixel,
    pub matteTopShadowColor: Pixel,
    pub matteTopShadowPStr: String_0,
    pub matteTopShadowPixmap: Pixmap,
    pub matteWidth: ::core::ffi::c_int,
    pub maximumClientSize: WHSize,
    pub smClientID: String_0,
    pub systemMenu: String_0,
    pub mwmMenuItems: *mut MenuItem_WmArgb,
    pub systemMenuSpec: *mut MenuSpec_WmArgb,
    pub useClientIcon: Boolean,
    pub wmSaveHintFlags: ::core::ffi::c_int,
    pub overrideGeometry: String_0,
    pub occupyWorkspaces: String_0,
    pub sizeFlags: ::core::ffi::c_long,
    pub decor: ::core::ffi::c_long,
    pub decorFlags: ::core::ffi::c_long,
    pub minWidth: ::core::ffi::c_int,
    pub minHeight: ::core::ffi::c_int,
    pub maxConfig: Boolean,
    pub maxX: ::core::ffi::c_int,
    pub maxY: ::core::ffi::c_int,
    pub maxWidthLimit: ::core::ffi::c_int,
    pub maxWidth: ::core::ffi::c_int,
    pub maxHeightLimit: ::core::ffi::c_int,
    pub maxHeight: ::core::ffi::c_int,
    pub oldMaxWidth: ::core::ffi::c_int,
    pub oldMaxHeight: ::core::ffi::c_int,
    pub widthInc: ::core::ffi::c_int,
    pub heightInc: ::core::ffi::c_int,
    pub minAspect: AspectRatio,
    pub maxAspect: AspectRatio,
    pub baseWidth: ::core::ffi::c_int,
    pub baseHeight: ::core::ffi::c_int,
    pub windowGravity: ::core::ffi::c_int,
    pub clientX: ::core::ffi::c_int,
    pub clientY: ::core::ffi::c_int,
    pub clientWidth: ::core::ffi::c_int,
    pub clientHeight: ::core::ffi::c_int,
    pub tileState: ::core::ffi::c_int,
    pub tileApplying: Boolean,
    pub tileX: ::core::ffi::c_int,
    pub tileY: ::core::ffi::c_int,
    pub tileWidth: ::core::ffi::c_int,
    pub tileHeight: ::core::ffi::c_int,
    pub clientOffset: XPoint,
    pub clientTitle: XmString,
    pub ewmhClientTitle: XmString,
    pub desktopSurface: Boolean,
    pub argbFrame: *mut _ArgbFrame_WmArgb,
    pub clientFrameWin: Window,
    pub clientStretchWin: [Window; 8],
    pub clientTitleWin: Window,
    pub clientBaseWin: Window,
    pub xBorderWidth: ::core::ffi::c_int,
    pub frameInfo: FrameInfo,
    pub fullScreen: Boolean,
    pub normalClientFunctions: ::core::ffi::c_int,
    pub fullScreenX: ::core::ffi::c_int,
    pub fullScreenY: ::core::ffi::c_int,
    pub fullScreenWidth: ::core::ffi::c_int,
    pub fullScreenHeight: ::core::ffi::c_int,
    pub fullScreenXineramaIndices: [::core::ffi::c_int; 4],
    pub pclientTopShadows: *mut RList,
    pub pclientBottomShadows: *mut RList,
    pub pclientTitleTopShadows: *mut RList,
    pub pclientTitleBottomShadows: *mut RList,
    pub pclientMatteTopShadows: *mut RList,
    pub pclientMatteBottomShadows: *mut RList,
    pub titleRectangle: XRectangle,
    pub pTitleGadgets: *mut GadgetRectangle,
    pub cTitleGadgets: ::core::ffi::c_int,
    pub pResizeGadgets: *mut GadgetRectangle,
    pub matteRectangle: XRectangle,
    pub clientMatteTopShadowGC: GC,
    pub clientMatteBottomShadowGC: GC,
    pub pSD: *mut WmScreenData_WmArgb,
    pub iconFlags: ::core::ffi::c_long,
    pub iconTitle: XmString,
    pub ewmhIconTitle: XmString,
    pub iconPixmap: Pixmap,
    pub iconMask: Pixmap,
    pub iconWindow: Window,
    pub ewmhIconPixmap: Pixmap,
    pub piconTopShadows: *mut RList,
    pub piconBottomShadows: *mut RList,
    pub absentMapBehavior: ::core::ffi::c_int,
    pub numInhabited: ::core::ffi::c_int,
    pub sizeWsList: ::core::ffi::c_int,
    pub pWsList: *mut _WsClientData_WmArgb,
    pub currentWsc: ::core::ffi::c_int,
    pub pWorkspaceHints: *mut WorkspaceID,
    pub numWorkspaceHints: ::core::ffi::c_int,
    pub putInAll: Boolean,
    pub wsmFunctions: ::core::ffi::c_long,
    pub wShaped: ::core::ffi::c_short,
    pub usePPosition: ::core::ffi::c_int,
    pub window_status: ::core::ffi::c_long,
}

pub type Object_WmEvent = *mut _ObjectRec_WmEvent;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ObjectPart_WmEvent {
    pub self_0: Widget,
    pub widget_class: WidgetClass,
    pub parent: Widget,
    pub xrm_name: XrmName,
    pub being_destroyed: Boolean,
    pub destroy_callbacks: XtCallbackList,
    pub constraints: XtPointer,
}

pub type ObjectPart_WmEvent = _ObjectPart_WmEvent;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ObjectRec_WmEvent {
    pub object: ObjectPart_WmEvent,
}

pub type XtLanguageProc =
    Option<unsafe extern "C" fn(*mut Display, String_0, XtPointer) -> String_0>;

pub type WmGlobalData_WmInitWs = _WmGlobalData_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmGlobalData_WmInitWs {
    pub dataType: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub environ: *mut *mut ::core::ffi::c_char,
    pub mwmName: *mut ::core::ffi::c_char,
    pub topLevelW: Widget_WmInitWs,
    pub topLevelW1: Widget_WmInitWs,
    pub confirmDialogMapped: Boolean,
    pub mwmAppContext: XtAppContext,
    pub windowContextType: XContext,
    pub screenContextType: XContext,
    pub cmapWindowContextType: XContext,
    pub mwmWindowContextType: XContext,
    pub commandWindow: Window,
    pub displayString: String_0,
    pub display: *mut Display_WmInitWs,
    pub display1: *mut Display_WmInitWs,
    pub statusColorServer: ::core::ffi::c_int,
    pub numScreens: ::core::ffi::c_int,
    pub screenNames: *mut *mut ::core::ffi::c_uchar,
    pub Screens: *mut WmScreenData_WmInitWs,
    pub workspaceCursor: Cursor,
    pub stretchCursors: [Cursor; 8],
    pub configCursor: Cursor,
    pub movePlacementCursor: Cursor,
    pub sizePlacementCursor: Cursor,
    pub pActiveSD: *mut WmScreenData_WmInitWs,
    pub useStandardBehavior: Boolean,
    pub wmRestarted: Boolean,
    pub errorFlag: Boolean,
    pub errorResource: XID,
    pub errorRequestCode: ::core::ffi::c_uchar,
    pub menuActive: *mut MenuSpec_WmInitWs,
    pub menuUnpostKeySpec: *mut KeySpec_WmInitWs,
    pub menuClient: *mut ClientData_WmInitWs,
    pub F_NextKeySpec: *mut KeySpec_WmInitWs,
    pub F_PrevKeySpec: *mut KeySpec_WmInitWs,
    pub grabContext: Context,
    pub clickData: ClickData_WmInitWs,
    pub configAction: ::core::ffi::c_int,
    pub configButton: ::core::ffi::c_uint,
    pub configPart: ::core::ffi::c_uint,
    pub configSet: Boolean,
    pub movingIcon: Boolean,
    pub preMove: Boolean,
    pub preMoveX: ::core::ffi::c_int,
    pub preMoveY: ::core::ffi::c_int,
    pub gadgetClient: *mut ClientData_WmInitWs,
    pub gadgetDepressed: ::core::ffi::c_int,
    pub checkHotspot: Boolean,
    pub hotspotRectangle: XRectangle,
    pub wmTimers: *mut WmTimer_WmInitWs,
    pub passKeysActive: Boolean,
    pub passKeysKeySpec: *mut KeySpec_WmInitWs,
    pub activeIconTextDisplayed: Boolean,
    pub queryScreen: Boolean,
    pub keyboardFocus: *mut ClientData_WmInitWs,
    pub nextKeyboardFocus: *mut ClientData_WmInitWs,
    pub systemModalActive: Boolean,
    pub systemModalClient: *mut ClientData_WmInitWs,
    pub systemModalWindow: Window,
    pub clientResourceDB: XrmDatabase,
    pub dbFileName: *mut ::core::ffi::c_char,
    pub sessionClientDB: String_0,
    pub xa_WM_STATE: Atom,
    pub xa_WM_PROTOCOLS: Atom,
    pub xa_WM_CHANGE_STATE: Atom,
    pub xa_WM_SAVE_YOURSELF: Atom,
    pub xa_WM_DELETE_WINDOW: Atom,
    pub xa_WM_TAKE_FOCUS: Atom,
    pub xa_WM_COLORMAP_WINDOWS: Atom,
    pub xa_MWM_HINTS: Atom,
    pub xa_MWM_MESSAGES: Atom,
    pub xa_MWM_MENU: Atom,
    pub xa_MWM_INFO: Atom,
    pub xa_MWM_OFFSET: Atom,
    pub xa_MWM_CLIENT_LIST: Atom,
    pub xa_MOTIF_BINDINGS: Atom,
    pub xa_COMPOUND_TEXT: Atom,
    pub xa_SM_CLIENT_ID: Atom,
    pub xa_WMSAVE_HINT: Atom,
    pub xa_MWM_WM_REQUEST: Atom,
    pub xa_MWM_WORKSPACE_HINTS: Atom,
    pub xa_MWM_WORKSPACE_PRESENCE: Atom,
    pub xa_MWM_WORKSPACE_INFO: Atom,
    pub xa_MWM_WORKSPACE_LIST: Atom,
    pub xa_MWM_WORKSPACE_CURRENT: Atom,
    pub xa_ALL_WORKSPACES: Atom,
    pub autoKeyFocus: Boolean,
    pub autoRaiseDelay: ::core::ffi::c_int,
    pub bitmapDirectory: String_0,
    pub backdropDirs: String_0,
    pub clientAutoPlace: Boolean,
    pub colormapFocusPolicy: ::core::ffi::c_int,
    pub configFile: String_0,
    pub cppCommand: String_0,
    pub deiconifyKeyFocus: Boolean,
    pub doubleClickTime: ::core::ffi::c_int,
    pub enableWarp: Boolean,
    pub enforceKeyFocus: Boolean,
    pub useWindowOutline: Boolean,
    pub outlineWidth: Dimension,
    pub iconAutoPlace: Boolean,
    pub iconClick: Boolean,
    pub interactivePlacement: Boolean,
    pub keyboardFocusPolicy: ::core::ffi::c_int,
    pub lowerOnIconify: Boolean,
    pub moveThreshold: ::core::ffi::c_int,
    pub passButtonsCheck: Boolean,
    pub passButtons: Boolean,
    pub passSelectButton: Boolean,
    pub positionOnScreen: Boolean,
    pub quitTimeout: ::core::ffi::c_int,
    pub raiseKeyFocus: Boolean,
    pub multiScreen: Boolean,
    pub screenList: String_0,
    pub showFeedback: ::core::ffi::c_int,
    pub rootButtonClick: Boolean,
    pub refreshByClearing: Boolean,
    pub startupKeyFocus: Boolean,
    pub systemButtonClick: Boolean,
    pub systemButtonClick2: Boolean,
    pub useLargeCursors: Boolean,
    pub requestContextWin: Window,
    pub waitForClicks: Boolean,
    pub frameStyle: FrameStyle,
    pub evLastButton: XButtonEvent_WmInitWs,
    pub bReplayedButton: Boolean,
    pub iconExternalShadowWidth: Dimension,
    pub frameExternalShadowWidth: Dimension,
    pub marqueeSelectGranularity: ::core::ffi::c_int,
    pub bSuspendSecondaryRestack: Boolean,
    pub clientDefaultTitle: XmString,
    pub iconDefaultTitle: XmString,
    pub attributesWindow: Window,
    pub windowAttributes: XWindowAttributes_WmInitWs,
    pub hasShape: Boolean,
    pub shapeEventBase: ::core::ffi::c_int,
    pub shapeErrorBase: ::core::ffi::c_int,
    pub replayEnterEvent: ::core::ffi::c_int,
    pub savedEnterEvent: XEnterWindowEvent_WmInitWs,
    pub lockingModMask: ::core::ffi::c_uint,
    pub pLockMaskSequence: *mut ::core::ffi::c_uint,
    pub tmpBuffer: [::core::ffi::c_uchar; 1024],
    pub numMouseButtons: ::core::ffi::c_int,
    pub bMenuButton: ::core::ffi::c_uint,
    pub primaryXineramaScreen: ::core::ffi::c_int,
    pub xineramaScreenFocus: ::core::ffi::c_int,
    pub xineramaIconifyToPrimary: Boolean,
    pub xrandr_present: Boolean,
    pub xrandr_base_evt: ::core::ffi::c_int,
    pub xrandr_base_err: ::core::ffi::c_int,
}

pub type WmWorkspaceData_WmInitWs = _WmWorkspaceData_WmInitWs;

pub type WmTimer_WmInitWs = _WmTimer_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmTimer_WmInitWs {
    pub timerId: XtIntervalId,
    pub timerCD: *mut _ClientData_WmInitWs,
    pub timerType: ::core::ffi::c_uint,
    pub nextWmTimer: *mut _WmTimer_WmInitWs,
}

pub type ClickData_WmInitWs = _ClickData_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClickData_WmInitWs {
    pub clickPending: Boolean,
    pub doubleClickPending: Boolean,
    pub bReplayed: Boolean,
    pub button: ::core::ffi::c_uint,
    pub state: ::core::ffi::c_uint,
    pub releaseState: ::core::ffi::c_uint,
    pub pCD: *mut _ClientData_WmInitWs,
    pub context: Context,
    pub subContext: Context,
    pub clickContext: Context,
    pub doubleClickContext: Context,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsPresenceData_WmInitWs {
    pub title: XmString,
    pub shellW: Widget_WmInitWs,
    pub formW: Widget_WmInitWs,
    pub windowLabelW: Widget_WmInitWs,
    pub windowNameW: Widget_WmInitWs,
    pub workspaceLabelW: Widget_WmInitWs,
    pub workspaceScrolledListW: Widget_WmInitWs,
    pub workspaceListW: Widget_WmInitWs,
    pub allWsW: Widget_WmInitWs,
    pub sepW: Widget_WmInitWs,
    pub OkW: Widget_WmInitWs,
    pub CancelW: Widget_WmInitWs,
    pub HelpW: Widget_WmInitWs,
    pub pCDforClient: *mut _ClientData_WmInitWs,
    pub contextForClient: Context,
    pub ItemSelected: *mut Boolean,
    pub ItemStrings: XmStringTable,
    pub currentWsItem: ::core::ffi::c_int,
    pub onScreen: Boolean,
    pub userDismissed: Boolean,
    pub numWorkspaces: ::core::ffi::c_int,
}

pub type WsPresenceData_WmInitWs = _WsPresenceData_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ButtonSpec_WmInitWs {
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub eventType: ::core::ffi::c_uint,
    pub click: Boolean,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction_WmInitWs,
    pub wmFuncArgs: String_0,
    pub nextButtonSpec: *mut _ButtonSpec_WmInitWs,
}

pub type ButtonSpec_WmInitWs = _ButtonSpec_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuItem_WmInitWs {
    pub labelType: ::core::ffi::c_int,
    pub label: String_0,
    pub labelBitmapIndex: ::core::ffi::c_int,
    pub mnemonic: KeySym,
    pub accelState: ::core::ffi::c_uint,
    pub accelKeyCode: KeyCode,
    pub accelText: String_0,
    pub wmFunction: WmFunction_WmInitWs,
    pub wmFuncArgs: String_0,
    pub greyedContext: Context,
    pub mgtMask: ::core::ffi::c_long,
    pub nextMenuItem: *mut _MenuItem_WmInitWs,
}

pub type MenuItem_WmInitWs = _MenuItem_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuButton_WmInitWs {
    pub menuItem: *mut MenuItem_WmInitWs,
    pub buttonWidget: Widget_WmInitWs,
    pub managed: Boolean,
}

pub type MenuButton_WmInitWs = _MenuButton_WmInitWs;

pub type ClientData_WmInitWs = _ClientData_WmInitWs;

pub type WmFunction_WmInitWs = Option<
    unsafe extern "C" fn(String_0, *mut ClientData_WmInitWs, *mut XEvent_WmInitWs) -> Boolean,
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _KeySpec_WmInitWs {
    pub state: ::core::ffi::c_uint,
    pub keycode: KeyCode,
    pub context: Context,
    pub subContext: Context,
    pub wmFunction: WmFunction_WmInitWs,
    pub wmFuncArgs: String_0,
    pub nextKeySpec: *mut _KeySpec_WmInitWs,
}

pub type KeySpec_WmInitWs = _KeySpec_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MenuSpec_WmInitWs {
    pub name: String_0,
    pub currentContext: Context,
    pub menuWidget: Widget_WmInitWs,
    pub whichButton: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub menuItems: *mut MenuItem_WmInitWs,
    pub menuButtons: *mut MenuButton_WmInitWs,
    pub menuButtonSize: ::core::ffi::c_uint,
    pub menuButtonCount: ::core::ffi::c_uint,
    pub accelContext: Context,
    pub accelKeySpecs: *mut KeySpec_WmInitWs,
    pub nextMenuSpec: *mut _MenuSpec_WmInitWs,
}

pub type MenuSpec_WmInitWs = _MenuSpec_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AppearanceData_WmInitWs {
    pub renderTable: XmRenderTable,
    pub fontHeight: ::core::ffi::c_uint,
    pub titleHeight: ::core::ffi::c_uint,
    pub saveUnder: Boolean,
    pub background: Pixel,
    pub foreground: Pixel,
    pub backgroundPStr: String_0,
    pub backgroundPixmap: Pixmap,
    pub bottomShadowColor: Pixel,
    pub bottomShadowPStr: String_0,
    pub bottomShadowPixmap: Pixmap,
    pub topShadowColor: Pixel,
    pub topShadowPStr: String_0,
    pub topShadowPixmap: Pixmap,
    pub activeBackground: Pixel,
    pub activeBackgroundPStr: String_0,
    pub activeBackgroundPixmap: Pixmap,
    pub activeBottomShadowColor: Pixel,
    pub activeBottomShadowPStr: String_0,
    pub activeBottomShadowPixmap: Pixmap,
    pub activeForeground: Pixel,
    pub activeTopShadowColor: Pixel,
    pub activeTopShadowPStr: String_0,
    pub activeTopShadowPixmap: Pixmap,
    pub inactiveGC: GC_WmInitWs,
    pub inactiveTopShadowGC: GC_WmInitWs,
    pub inactiveBottomShadowGC: GC_WmInitWs,
    pub activeGC: GC_WmInitWs,
    pub activeTopShadowGC: GC_WmInitWs,
    pub activeBottomShadowGC: GC_WmInitWs,
}

pub type AppearanceData_WmInitWs = _AppearanceData_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientListEntry_WmInitWs {
    pub nextSibling: *mut _ClientListEntry_WmInitWs,
    pub prevSibling: *mut _ClientListEntry_WmInitWs,
    pub type_0: ::core::ffi::c_int,
    pub pCD: *mut _ClientData_WmInitWs,
}

pub type ClientListEntry_WmInitWs = _ClientListEntry_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmWorkspaceData_WmInitWs {
    pub dataType: ::core::ffi::c_int,
    pub id: WorkspaceID,
    pub map_state: ::core::ffi::c_int,
    pub backdrop: BackdropData,
    pub buttonW: Widget_WmInitWs,
    pub title: XmString,
    pub iconBoxGeometry: String_0,
    pub name: String_0,
    pub pSD: *mut WmScreenData_WmInitWs,
    pub pIconBox: *mut IconBoxData_WmInitWs,
    pub IPData: *mut IconPlacementData_WmInitWs,
    pub workspaceTopLevelW: Widget_WmInitWs,
    pub keyFocus: *mut _ClientData_WmInitWs,
    pub nextKeyFocus: *mut _ClientData_WmInitWs,
    pub ppClients: *mut *mut _ClientData_WmInitWs,
    pub numClients: ::core::ffi::c_uint,
    pub sizeClientList: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WmScreenData_WmInitWs {
    pub dataType: ::core::ffi::c_int,
    pub screen: ::core::ffi::c_int,
    pub managed: Boolean,
    pub rootWindow: Window,
    pub screenTopLevelW: Widget_WmInitWs,
    pub screenTopLevelW1: Widget_WmInitWs,
    pub confirmboxW: [Widget_WmInitWs; 4],
    pub presence: WsPresenceData_WmInitWs,
    pub wmWorkspaceWin: Window,
    pub feedbackWin: Window,
    pub activeIconTextWin: Window,
    pub activeLabelParent: Window,
    pub displayString: String_0,
    pub displayResolutionType: ::core::ffi::c_int,
    pub clientCounter: ::core::ffi::c_ulong,
    pub focusPriority: ::core::ffi::c_long,
    pub inputScreenWindow: Window,
    pub colormapFocus: *mut _ClientData_WmInitWs,
    pub workspaceColormap: Colormap,
    pub lastInstalledColormap: Colormap,
    pub pActiveWS: *mut _WmWorkspaceData_WmInitWs,
    pub pLastWS: *mut _WmWorkspaceData_WmInitWs,
    pub bitmapCache: *mut BitmapCache,
    pub bitmapCacheSize: ::core::ffi::c_uint,
    pub bitmapCacheCount: ::core::ffi::c_uint,
    pub fadeNormalIcon: Boolean,
    pub iconPlacement: ::core::ffi::c_int,
    pub iconPlacementMargin: ::core::ffi::c_int,
    pub iconDecoration: ::core::ffi::c_int,
    pub iconImageMaximum: WHSize,
    pub iconImageMinimum: WHSize,
    pub builtinIconPixmap: Pixmap,
    pub iconWidth: ::core::ffi::c_int,
    pub iconHeight: ::core::ffi::c_int,
    pub iconImageHeight: ::core::ffi::c_int,
    pub iconLabelHeight: ::core::ffi::c_int,
    pub shrinkWrapGC: GC_WmInitWs,
    pub fadeIconGC: GC_WmInitWs,
    pub fadeIconTextGC: GC_WmInitWs,
    pub woN: Window,
    pub woS: Window,
    pub woE: Window,
    pub woW: Window,
    pub fbStyle: ::core::ffi::c_ulong,
    pub fbWinWidth: ::core::ffi::c_uint,
    pub fbWinHeight: ::core::ffi::c_uint,
    pub fbLocation: XmString,
    pub fbSize: XmString,
    pub fbLocX: ::core::ffi::c_int,
    pub fbLocY: ::core::ffi::c_int,
    pub fbSizeX: ::core::ffi::c_int,
    pub fbSizeY: ::core::ffi::c_int,
    pub fbLastX: ::core::ffi::c_int,
    pub fbLastY: ::core::ffi::c_int,
    pub fbLastWidth: ::core::ffi::c_uint,
    pub fbLastHeight: ::core::ffi::c_uint,
    pub fbTop: *mut RList,
    pub fbBottom: *mut RList,
    pub actionNbr: ::core::ffi::c_int,
    pub buttonBindings: String_0,
    pub buttonSpecs: *mut ButtonSpec_WmInitWs,
    pub keyBindings: String_0,
    pub keySpecs: *mut KeySpec_WmInitWs,
    pub acceleratorMenuSpecs: *mut *mut MenuSpec_WmInitWs,
    pub acceleratorMenuCount: ::core::ffi::c_uint,
    pub menuSpecs: *mut MenuSpec_WmInitWs,
    pub defaultSystemMenuUseBuiltin: Boolean,
    pub defaultPixmap: Pixmap,
    pub xorGC: GC_WmInitWs,
    pub cleanText: Boolean,
    pub decoupleTitleAppearance: Boolean,
    pub frameBorderWidth: ::core::ffi::c_int,
    pub feedbackGeometry: String_0,
    pub iconBoxName: String_0,
    pub iconBoxSBDisplayPolicy: String_0,
    pub iconBoxScheme: ::core::ffi::c_int,
    pub iconBoxTitle: XmString,
    pub externalBevel: ::core::ffi::c_int,
    pub joinBevel: ::core::ffi::c_int,
    pub limitResize: Boolean,
    pub maximumMaximumSize: WHSize,
    pub resizeBorderWidth: ::core::ffi::c_int,
    pub resizeCursors: Boolean,
    pub transientDecoration: ::core::ffi::c_int,
    pub transientFunctions: ::core::ffi::c_int,
    pub utilityDecoration: ::core::ffi::c_int,
    pub utilityFunctions: ::core::ffi::c_int,
    pub useIconBox: Boolean,
    pub moveOpaque: Boolean,
    pub titleLeft: Boolean,
    pub pPixelData: *mut XmPixelSet,
    pub pActivePixelSet: *mut XmPixelSet,
    pub pInactivePixelSet: *mut XmPixelSet,
    pub pPrimaryPixelSet: *mut XmPixelSet,
    pub pSecondaryPixelSet: *mut XmPixelSet,
    pub pTextPixelSet: *mut XmPixelSet,
    pub colorUse: ::core::ffi::c_int,
    pub clientAppearance: AppearanceData_WmInitWs,
    pub clientTitleAppearance: AppearanceData_WmInitWs,
    pub transientOffset: XPoint,
    pub Num_Resize_Ts_Elements: ::core::ffi::c_int,
    pub Num_Resize_Bs_Elements: ::core::ffi::c_int,
    pub Num_Title_Ts_Elements: ::core::ffi::c_int,
    pub Num_Title_Bs_Elements: ::core::ffi::c_int,
    pub iconAppearance: AppearanceData_WmInitWs,
    pub feedbackAppearance: AppearanceData_WmInitWs,
    pub clientList: *mut ClientListEntry_WmInitWs,
    pub lastClient: *mut ClientListEntry_WmInitWs,
    pub pSessionItems: *mut _WmSessionItem,
    pub totalSessionItems: ::core::ffi::c_int,
    pub remainingSessionItems: ::core::ffi::c_int,
    pub initialWorkspace: String_0,
    pub workspaceList: String_0,
    pub numWorkspaces: ::core::ffi::c_int,
    pub numWsDataAllocated: ::core::ffi::c_int,
    pub pWS: *mut _WmWorkspaceData_WmInitWs,
    pub lastBackdropWin: Window,
    pub bMarqueeSelectionInitialized: Boolean,
}

pub type WmScreenData_WmInitWs = _WmScreenData_WmInitWs;

pub type IconPlacementData_WmInitWs = _IconPlacementData_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconInfo_WmInitWs {
    pub theWidget: Widget_WmInitWs,
    pub pCD: *mut _ClientData_WmInitWs,
}

pub type IconInfo_WmInitWs = _IconInfo_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconPlacementData_WmInitWs {
    pub onRootWindow: Boolean,
    pub iconPlacement: ::core::ffi::c_uint,
    pub placementRows: ::core::ffi::c_int,
    pub placementCols: ::core::ffi::c_int,
    pub totalPlaces: ::core::ffi::c_int,
    pub iPlaceW: ::core::ffi::c_int,
    pub iPlaceH: ::core::ffi::c_int,
    pub placeList: *mut IconInfo_WmInitWs,
    pub placeIconX: ::core::ffi::c_int,
    pub placeIconY: ::core::ffi::c_int,
    pub placementRowY: *mut ::core::ffi::c_int,
    pub placementColX: *mut ::core::ffi::c_int,
    pub xiOrgX: ::core::ffi::c_int,
    pub xiOrgY: ::core::ffi::c_int,
    pub xiScreen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IconBoxData_WmInitWs {
    pub shellWidget: Widget_WmInitWs,
    pub frameWidget: Widget_WmInitWs,
    pub scrolledWidget: Widget_WmInitWs,
    pub vScrollBar: Widget_WmInitWs,
    pub hScrollBar: Widget_WmInitWs,
    pub bBoardWidget: Widget_WmInitWs,
    pub clipWidget: Widget_WmInitWs,
    pub numberOfIcons: ::core::ffi::c_int,
    pub currentRow: ::core::ffi::c_int,
    pub currentCol: ::core::ffi::c_int,
    pub lastRow: ::core::ffi::c_int,
    pub lastCol: ::core::ffi::c_int,
    pub wsID: WorkspaceID,
    pub pCD_iconBox: *mut _ClientData_WmInitWs,
    pub pNextIconBox: *mut _IconBoxData_WmInitWs,
    pub IPD: _IconPlacementData_WmInitWs,
}

pub type IconBoxData_WmInitWs = _IconBoxData_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WsClientData_WmInitWs {
    pub wsID: WorkspaceID,
    pub IPData: *mut IconPlacementData_WmInitWs,
    pub iconPlace: ::core::ffi::c_int,
    pub iconX: ::core::ffi::c_int,
    pub iconY: ::core::ffi::c_int,
    pub iconFrameWin: Window,
    pub pIconBox: *mut IconBoxData_WmInitWs,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClientData_WmInitWs {
    pub dataType: ::core::ffi::c_int,
    pub client: Window,
    pub clientFlags: ::core::ffi::c_long,
    pub icccVersion: ::core::ffi::c_int,
    pub clientState: ::core::ffi::c_int,
    pub inputFocusModel: ::core::ffi::c_int,
    pub inputMode: ::core::ffi::c_int,
    pub focusPriority: ::core::ffi::c_long,
    pub clientID: ::core::ffi::c_ulong,
    pub wmUnmapCount: ::core::ffi::c_int,
    pub paInitialProperties: *mut Atom,
    pub numInitialProperties: ::core::ffi::c_int,
    pub clientProtocols: *mut Atom,
    pub clientProtocolCount: ::core::ffi::c_int,
    pub protocolFlags: ::core::ffi::c_long,
    pub mwmMessages: *mut ::core::ffi::c_long,
    pub mwmMessagesCount: ::core::ffi::c_int,
    pub clientColormap: Colormap,
    pub cmapWindows: *mut Window,
    pub clientCmapList: *mut Colormap,
    pub clientCmapCount: ::core::ffi::c_int,
    pub clientCmapIndex: ::core::ffi::c_int,
    pub clientCmapFlags: *mut ::core::ffi::c_int,
    pub clientCmapFlagsInitialized: ::core::ffi::c_int,
    pub clientEntry: ClientListEntry_WmInitWs,
    pub iconEntry: ClientListEntry_WmInitWs,
    pub windowGroup: XID,
    pub pIconBox: *mut IconBoxData_WmInitWs,
    pub thisIconBox: *mut IconBoxData_WmInitWs,
    pub grabContext: Context,
    pub transientFor: Window,
    pub transientLeader: *mut _ClientData_WmInitWs,
    pub transientChildren: *mut _ClientData_WmInitWs,
    pub transientSiblings: *mut _ClientData_WmInitWs,
    pub primaryStackPosition: ::core::ffi::c_int,
    pub secondariesOnTop: Boolean,
    pub primaryModalCount: ::core::ffi::c_int,
    pub fullModalCount: ::core::ffi::c_int,
    pub clientClass: String_0,
    pub clientName: String_0,
    pub clientDecoration: ::core::ffi::c_int,
    pub clientFunctions: ::core::ffi::c_int,
    pub focusAutoRaise: Boolean,
    pub focusAutoRaiseDisabled: Boolean,
    pub focusAutoRaiseDisablePending: Boolean,
    pub iconImage: String_0,
    pub iconImageBackground: Pixel,
    pub iconImageBottomShadowColor: Pixel,
    pub iconImageBottomShadowPStr: String_0,
    pub iconImageBottomShadowPixmap: Pixmap,
    pub iconImageForeground: Pixel,
    pub iconImageTopShadowColor: Pixel,
    pub iconImageTopShadowPStr: String_0,
    pub iconImageTopShadowPixmap: Pixmap,
    pub ignoreWMSaveHints: Boolean,
    pub internalBevel: ::core::ffi::c_int,
    pub matteBackground: Pixel,
    pub matteBottomShadowColor: Pixel,
    pub matteBottomShadowPStr: String_0,
    pub matteBottomShadowPixmap: Pixmap,
    pub matteForeground: Pixel,
    pub matteTopShadowColor: Pixel,
    pub matteTopShadowPStr: String_0,
    pub matteTopShadowPixmap: Pixmap,
    pub matteWidth: ::core::ffi::c_int,
    pub maximumClientSize: WHSize,
    pub smClientID: String_0,
    pub systemMenu: String_0,
    pub mwmMenuItems: *mut MenuItem_WmInitWs,
    pub systemMenuSpec: *mut MenuSpec_WmInitWs,
    pub useClientIcon: Boolean,
    pub wmSaveHintFlags: ::core::ffi::c_int,
    pub overrideGeometry: String_0,
    pub occupyWorkspaces: String_0,
    pub sizeFlags: ::core::ffi::c_long,
    pub decor: ::core::ffi::c_long,
    pub decorFlags: ::core::ffi::c_long,
    pub minWidth: ::core::ffi::c_int,
    pub minHeight: ::core::ffi::c_int,
    pub maxConfig: Boolean,
    pub maxX: ::core::ffi::c_int,
    pub maxY: ::core::ffi::c_int,
    pub maxWidthLimit: ::core::ffi::c_int,
    pub maxWidth: ::core::ffi::c_int,
    pub maxHeightLimit: ::core::ffi::c_int,
    pub maxHeight: ::core::ffi::c_int,
    pub oldMaxWidth: ::core::ffi::c_int,
    pub oldMaxHeight: ::core::ffi::c_int,
    pub widthInc: ::core::ffi::c_int,
    pub heightInc: ::core::ffi::c_int,
    pub minAspect: AspectRatio,
    pub maxAspect: AspectRatio,
    pub baseWidth: ::core::ffi::c_int,
    pub baseHeight: ::core::ffi::c_int,
    pub windowGravity: ::core::ffi::c_int,
    pub clientX: ::core::ffi::c_int,
    pub clientY: ::core::ffi::c_int,
    pub clientWidth: ::core::ffi::c_int,
    pub clientHeight: ::core::ffi::c_int,
    pub tileState: ::core::ffi::c_int,
    pub tileApplying: Boolean,
    pub tileX: ::core::ffi::c_int,
    pub tileY: ::core::ffi::c_int,
    pub tileWidth: ::core::ffi::c_int,
    pub tileHeight: ::core::ffi::c_int,
    pub clientOffset: XPoint,
    pub clientTitle: XmString,
    pub ewmhClientTitle: XmString,
    pub desktopSurface: Boolean,
    pub argbFrame: *mut _ArgbFrame,
    pub clientFrameWin: Window,
    pub clientStretchWin: [Window; 8],
    pub clientTitleWin: Window,
    pub clientBaseWin: Window,
    pub xBorderWidth: ::core::ffi::c_int,
    pub frameInfo: FrameInfo,
    pub fullScreen: Boolean,
    pub normalClientFunctions: ::core::ffi::c_int,
    pub fullScreenX: ::core::ffi::c_int,
    pub fullScreenY: ::core::ffi::c_int,
    pub fullScreenWidth: ::core::ffi::c_int,
    pub fullScreenHeight: ::core::ffi::c_int,
    pub fullScreenXineramaIndices: [::core::ffi::c_int; 4],
    pub pclientTopShadows: *mut RList,
    pub pclientBottomShadows: *mut RList,
    pub pclientTitleTopShadows: *mut RList,
    pub pclientTitleBottomShadows: *mut RList,
    pub pclientMatteTopShadows: *mut RList,
    pub pclientMatteBottomShadows: *mut RList,
    pub titleRectangle: XRectangle,
    pub pTitleGadgets: *mut GadgetRectangle,
    pub cTitleGadgets: ::core::ffi::c_int,
    pub pResizeGadgets: *mut GadgetRectangle,
    pub matteRectangle: XRectangle,
    pub clientMatteTopShadowGC: GC_WmInitWs,
    pub clientMatteBottomShadowGC: GC_WmInitWs,
    pub pSD: *mut WmScreenData_WmInitWs,
    pub iconFlags: ::core::ffi::c_long,
    pub iconTitle: XmString,
    pub ewmhIconTitle: XmString,
    pub iconPixmap: Pixmap,
    pub iconMask: Pixmap,
    pub iconWindow: Window,
    pub ewmhIconPixmap: Pixmap,
    pub piconTopShadows: *mut RList,
    pub piconBottomShadows: *mut RList,
    pub absentMapBehavior: ::core::ffi::c_int,
    pub numInhabited: ::core::ffi::c_int,
    pub sizeWsList: ::core::ffi::c_int,
    pub pWsList: *mut _WsClientData_WmInitWs,
    pub currentWsc: ::core::ffi::c_int,
    pub pWorkspaceHints: *mut WorkspaceID,
    pub numWorkspaceHints: ::core::ffi::c_int,
    pub putInAll: Boolean,
    pub wsmFunctions: ::core::ffi::c_long,
    pub wShaped: ::core::ffi::c_short,
    pub usePPosition: ::core::ffi::c_int,
    pub window_status: ::core::ffi::c_long,
}

pub type MwmInfo = MotifWmInfo;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotifWmInfo {
    pub flags: ::core::ffi::c_long,
    pub wm_window: Window,
}

pub type EventMask = ::core::ffi::c_ulong;

pub type XtEventHandler = Option<
    unsafe extern "C" fn(Widget_WmInitWs, XtPointer, *mut XEvent_WmInitWs, *mut Boolean) -> (),
>;

pub type Modifiers = ::core::ffi::c_uint;

pub type XtWidgetClassProc_WmInitWs = Option<unsafe extern "C" fn(WidgetClass_WmInitWs) -> ()>;

pub type XtInitProc_WmInitWs = Option<
    unsafe extern "C" fn(Widget_WmInitWs, Widget_WmInitWs, ArgList_WmFeedback, *mut Cardinal) -> (),
>;

pub type XtRealizeProc_WmInitWs = Option<
    unsafe extern "C" fn(Widget_WmInitWs, *mut XtValueMask, *mut XSetWindowAttributes) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtActionsRec_WmInitWs {
    pub string: String_0,
    pub proc_0: XtActionProc_WmInitWs,
}

pub type XtActionList_WmInitWs = *mut _XtActionsRec_WmInitWs;

pub type XtWidgetProc_WmInitWs = Option<unsafe extern "C" fn(Widget_WmInitWs) -> ()>;

pub type XtExposeProc_WmInitWs =
    Option<unsafe extern "C" fn(Widget_WmInitWs, *mut XEvent_WmInitWs, Region) -> ()>;

pub type XtSetValuesFunc_WmInitWs = Option<
    unsafe extern "C" fn(
        Widget_WmInitWs,
        Widget_WmInitWs,
        Widget_WmInitWs,
        ArgList_WmFeedback,
        *mut Cardinal,
    ) -> Boolean,
>;

pub type XtArgsFunc_WmInitWs =
    Option<unsafe extern "C" fn(Widget_WmInitWs, ArgList_WmFeedback, *mut Cardinal) -> Boolean>;

pub type XtAlmostProc_WmInitWs = Option<
    unsafe extern "C" fn(
        Widget_WmInitWs,
        Widget_WmInitWs,
        *mut XtWidgetGeometry_WmInitWs,
        *mut XtWidgetGeometry_WmInitWs,
    ) -> (),
>;

pub type XtArgsProc_WmInitWs =
    Option<unsafe extern "C" fn(Widget_WmInitWs, ArgList_WmFeedback, *mut Cardinal) -> ()>;

pub type XtAcceptFocusProc_WmInitWs =
    Option<unsafe extern "C" fn(Widget_WmInitWs, *mut Time) -> Boolean>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XtWidgetGeometry_WmInitWs {
    pub request_mode: XtGeometryMask,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub sibling: Widget_WmInitWs,
    pub stack_mode: ::core::ffi::c_int,
}

pub type XtGeometryHandler_WmInitWs = Option<
    unsafe extern "C" fn(
        Widget_WmInitWs,
        *mut XtWidgetGeometry_WmInitWs,
        *mut XtWidgetGeometry_WmInitWs,
    ) -> XtGeometryResult,
>;

pub type XtStringProc_WmInitWs = Option<unsafe extern "C" fn(Widget_WmInitWs, String_0) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CoreClassPart_WmInitWs {
    pub superclass: WidgetClass_WmInitWs,
    pub class_name: String_0,
    pub widget_size: Cardinal,
    pub class_initialize: XtProc,
    pub class_part_initialize: XtWidgetClassProc_WmInitWs,
    pub class_inited: XtEnum,
    pub initialize: XtInitProc_WmInitWs,
    pub initialize_hook: XtArgsProc_WmInitWs,
    pub realize: XtRealizeProc_WmInitWs,
    pub actions: XtActionList_WmInitWs,
    pub num_actions: Cardinal,
    pub resources: XtResourceList,
    pub num_resources: Cardinal,
    pub xrm_class: XrmClass,
    pub compress_motion: Boolean,
    pub compress_exposure: XtEnum,
    pub compress_enterleave: Boolean,
    pub visible_interest: Boolean,
    pub destroy: XtWidgetProc_WmInitWs,
    pub resize: XtWidgetProc_WmInitWs,
    pub expose: XtExposeProc_WmInitWs,
    pub set_values: XtSetValuesFunc_WmInitWs,
    pub set_values_hook: XtArgsFunc_WmInitWs,
    pub set_values_almost: XtAlmostProc_WmInitWs,
    pub get_values_hook: XtArgsProc_WmInitWs,
    pub accept_focus: XtAcceptFocusProc_WmInitWs,
    pub version: XtVersionType,
    pub callback_private: XtPointer,
    pub tm_table: String_0,
    pub query_geometry: XtGeometryHandler_WmInitWs,
    pub display_accelerator: XtStringProc_WmInitWs,
    pub extension: XtPointer,
}

pub type CoreClassPart_WmInitWs = _CoreClassPart_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetClassRec_WmInitWs {
    pub core_class: CoreClassPart_WmInitWs,
}

pub type WidgetClass_WmInitWs = *mut _WidgetClassRec_WmInitWs;

pub type XtCallbackProc_WmInitWs =
    Option<unsafe extern "C" fn(Widget_WmInitWs, XtPointer, XtPointer) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtCallbackRec_WmInitWs {
    pub callback: XtCallbackProc_WmInitWs,
    pub closure: XtPointer,
}

pub type XtCallbackList_WmInitWs = *mut _XtCallbackRec_WmInitWs;

pub type XtActionProc_WmInitWs = Option<
    unsafe extern "C" fn(Widget_WmInitWs, *mut XEvent_WmInitWs, *mut String_0, *mut Cardinal) -> (),
>;

pub type XtBoundActions_WmInitWs = *mut XtActionProc_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtTMRec_WmInitWs {
    pub translations: XtTranslations,
    pub proc_table: XtBoundActions_WmInitWs,
    pub current_state: *mut _XtStateRec,
    pub lastEventTime: ::core::ffi::c_ulong,
}

pub type XtTMRec_WmInitWs = _XtTMRec_WmInitWs;

pub type Widget_WmInitWs = *mut _WidgetRec_WmInitWs;

pub type WidgetList_WmInitWs = *mut Widget_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CorePart_WmInitWs {
    pub self_0: Widget_WmInitWs,
    pub widget_class: WidgetClass_WmInitWs,
    pub parent: Widget_WmInitWs,
    pub xrm_name: XrmName,
    pub being_destroyed: Boolean,
    pub destroy_callbacks: XtCallbackList_WmInitWs,
    pub constraints: XtPointer,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub managed: Boolean,
    pub sensitive: Boolean,
    pub ancestor_sensitive: Boolean,
    pub event_table: XtEventTable,
    pub tm: XtTMRec_WmInitWs,
    pub accelerators: XtTranslations,
    pub border_pixel: Pixel,
    pub border_pixmap: Pixmap,
    pub popup_list: WidgetList_WmInitWs,
    pub num_popups: Cardinal,
    pub name: String_0,
    pub screen: *mut Screen_WmInitWs,
    pub colormap: Colormap,
    pub window: Window,
    pub depth: Cardinal,
    pub background_pixel: Pixel,
    pub background_pixmap: Pixmap,
    pub visible: Boolean,
    pub mapped_when_managed: Boolean,
}

pub type CorePart_WmInitWs = _CorePart_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetRec_WmInitWs {
    pub core: CorePart_WmInitWs,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmOptionDescRec {
    pub option: *mut ::core::ffi::c_char,
    pub specifier: *mut ::core::ffi::c_char,
    pub argKind: XrmOptionKind,
    pub value: XPointer,
}

pub type XrmOptionKind = ::core::ffi::c_uint;

pub type XEnterWindowEvent_WmInitWs = XCrossingEvent_WmInitWs;

pub type _XPrivDisplay_WmInitWs = *mut C2RustUnnamed_32_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32_WmInitWs {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: ::core::ffi::c_int,
    pub private2: ::core::ffi::c_int,
    pub proto_major_version: ::core::ffi::c_int,
    pub proto_minor_version: ::core::ffi::c_int,
    pub vendor: *mut ::core::ffi::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: ::core::ffi::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay_WmInitWs) -> XID>,
    pub byte_order: ::core::ffi::c_int,
    pub bitmap_unit: ::core::ffi::c_int,
    pub bitmap_pad: ::core::ffi::c_int,
    pub bitmap_bit_order: ::core::ffi::c_int,
    pub nformats: ::core::ffi::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: ::core::ffi::c_int,
    pub release: ::core::ffi::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: ::core::ffi::c_int,
    pub last_request_read: ::core::ffi::c_ulong,
    pub request: ::core::ffi::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: ::core::ffi::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay_WmInitWs) -> ::core::ffi::c_int>,
    pub display_name: *mut ::core::ffi::c_char,
    pub default_screen: ::core::ffi::c_int,
    pub nscreens: ::core::ffi::c_int,
    pub screens: *mut Screen_WmInitWs,
    pub motion_buffer: ::core::ffi::c_ulong,
    pub private16: ::core::ffi::c_ulong,
    pub min_keycode: ::core::ffi::c_int,
    pub max_keycode: ::core::ffi::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: ::core::ffi::c_int,
    pub xdefaults: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes_WmInitWs {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: ::core::ffi::c_int,
    pub bit_gravity: ::core::ffi::c_int,
    pub win_gravity: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub backing_planes: ::core::ffi::c_ulong,
    pub backing_pixel: ::core::ffi::c_ulong,
    pub save_under: ::core::ffi::c_int,
    pub colormap: Colormap,
    pub map_installed: ::core::ffi::c_int,
    pub map_state: ::core::ffi::c_int,
    pub all_event_masks: ::core::ffi::c_long,
    pub your_event_mask: ::core::ffi::c_long,
    pub do_not_propagate_mask: ::core::ffi::c_long,
    pub override_redirect: ::core::ffi::c_int,
    pub screen: *mut Screen_WmInitWs,
}

pub type FreeModmapType = Option<unsafe extern "C" fn(*mut XModifierKeymap) -> ::core::ffi::c_int>;

pub type FreeFuncType = Option<unsafe extern "C" fn(*mut Display_WmInitWs) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XFreeFuncs {
    pub atoms: FreeFuncType,
    pub modifiermap: FreeModmapType,
    pub key_bindings: FreeFuncType,
    pub context_db: FreeFuncType,
    pub defaultCCCs: FreeFuncType,
    pub clientCmaps: FreeFuncType,
    pub intensityMaps: FreeFuncType,
    pub im_filters: FreeFuncType,
    pub xkb: FreeFuncType,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen_WmInitWs {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay_WmInitWs,
    pub root: Window,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub mwidth: ::core::ffi::c_int,
    pub mheight: ::core::ffi::c_int,
    pub ndepths: ::core::ffi::c_int,
    pub depths: *mut Depth,
    pub root_depth: ::core::ffi::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC_WmInitWs,
    pub cmap: Colormap,
    pub white_pixel: ::core::ffi::c_ulong,
    pub black_pixel: ::core::ffi::c_ulong,
    pub max_maps: ::core::ffi::c_int,
    pub min_maps: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub save_unders: ::core::ffi::c_int,
    pub root_input_mask: ::core::ffi::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGenericReply {
    pub type_0: BYTE,
    pub data1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub data00: CARD32,
    pub data01: CARD32,
    pub data02: CARD32,
    pub data03: CARD32,
    pub data04: CARD32,
    pub data05: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetGeometryReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub pad1: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTreeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub parent: CARD32,
    pub nChildren: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xInternAtomReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub atom: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetAtomNameReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nameLength: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPropertyReply {
    pub type_0: BYTE,
    pub format: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub propertyType: CARD32,
    pub bytesAfter: CARD32,
    pub nItems: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListPropertiesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nProperties: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetSelectionOwnerReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub owner: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGrabKeyboardReply {
    pub type_0: BYTE,
    pub status: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

pub type xGrabPointerReply = xGrabKeyboardReply;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryPointerReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub winX: INT16,
    pub winY: INT16,
    pub mask: CARD16,
    pub pad1: CARD16,
    pub pad: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetMotionEventsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nEvents: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xTranslateCoordsReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub child: CARD32,
    pub dstX: INT16,
    pub dstY: INT16,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetInputFocusReply {
    pub type_0: BYTE,
    pub revertTo: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub focus: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTextExtentsReply {
    pub type_0: BYTE,
    pub drawDirection: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub fontAscent: INT16,
    pub fontDescent: INT16,
    pub overallAscent: INT16,
    pub overallDescent: INT16,
    pub overallWidth: INT32,
    pub overallLeft: INT32,
    pub overallRight: INT32,
    pub pad: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListFontsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nFonts: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetFontPathReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPaths: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetImageReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub visual: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListInstalledColormapsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColormaps: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub red: CARD16,
    pub green: CARD16,
    pub blue: CARD16,
    pub pad2: CARD16,
    pub pixel: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocNamedColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pixel: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorCellsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub nMasks: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorPlanesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub pad2: CARD16,
    pub redMask: CARD32,
    pub greenMask: CARD32,
    pub blueMask: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryColorsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColors: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xLookupColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryBestSizeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub width: CARD16,
    pub height: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryExtensionReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub present: BOOL,
    pub major_opcode: CARD8,
    pub first_event: CARD8,
    pub first_error: CARD8,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListExtensionsReply {
    pub type_0: BYTE,
    pub nExtensions: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

pub type xSetModifierMappingReply = xSetMappingReply;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetModifierMappingReply {
    pub type_0: BYTE,
    pub numKeyPerModifier: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xSetPointerMappingReply {
    pub type_0: BYTE,
    pub success: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

pub type xSetMappingReply = xSetPointerMappingReply;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetKeyboardMappingReply {
    pub type_0: BYTE,
    pub keySymsPerKeyCode: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerMappingReply {
    pub type_0: BYTE,
    pub nElts: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerControlReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub accelNumerator: CARD16,
    pub accelDenominator: CARD16,
    pub threshold: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetScreenSaverReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub timeout: CARD16,
    pub interval: CARD16,
    pub preferBlanking: BOOL,
    pub allowExposures: BOOL,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListHostsReply {
    pub type_0: BYTE,
    pub enabled: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nHosts: CARD16,
    pub pad1: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union xReply {
    pub generic: xGenericReply,
    pub geom: xGetGeometryReply,
    pub tree: xQueryTreeReply,
    pub atom: xInternAtomReply,
    pub atomName: xGetAtomNameReply,
    pub property: xGetPropertyReply,
    pub listProperties: xListPropertiesReply,
    pub selection: xGetSelectionOwnerReply,
    pub grabPointer: xGrabPointerReply,
    pub grabKeyboard: xGrabKeyboardReply,
    pub pointer: xQueryPointerReply,
    pub motionEvents: xGetMotionEventsReply,
    pub coords: xTranslateCoordsReply,
    pub inputFocus: xGetInputFocusReply,
    pub textExtents: xQueryTextExtentsReply,
    pub fonts: xListFontsReply,
    pub fontPath: xGetFontPathReply,
    pub image: xGetImageReply,
    pub colormaps: xListInstalledColormapsReply,
    pub allocColor: xAllocColorReply,
    pub allocNamedColor: xAllocNamedColorReply,
    pub colorCells: xAllocColorCellsReply,
    pub colorPlanes: xAllocColorPlanesReply,
    pub colors: xQueryColorsReply,
    pub lookupColor: xLookupColorReply,
    pub bestSize: xQueryBestSizeReply,
    pub extension: xQueryExtensionReply,
    pub extensions: xListExtensionsReply,
    pub setModifierMapping: xSetModifierMappingReply,
    pub getModifierMapping: xGetModifierMappingReply,
    pub setPointerMapping: xSetPointerMappingReply,
    pub getKeyboardMapping: xGetKeyboardMappingReply,
    pub getPointerMapping: xGetPointerMappingReply,
    pub pointerControl: xGetPointerControlReply,
    pub screenSaver: xGetScreenSaverReply,
    pub hosts: xListHostsReply,
    pub error: xError,
    pub event: xEvent,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XInternalAsync {
    pub next: *mut _XInternalAsync,
    pub handler: Option<
        unsafe extern "C" fn(
            *mut Display_WmInitWs,
            *mut xReply,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
            XPointer,
        ) -> ::core::ffi::c_int,
    >,
    pub data: XPointer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XLockPtrs {
    pub lock_display: Option<unsafe extern "C" fn(*mut Display_WmInitWs) -> ()>,
    pub unlock_display: Option<unsafe extern "C" fn(*mut Display_WmInitWs) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31_WmInitWs {
    pub defaultCCCs: XPointer,
    pub clientCmaps: XPointer,
    pub perVisualIntensityMaps: XPointer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub keycode: ::core::ffi::c_uint,
    pub same_screen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub same_screen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub is_hint: ::core::ffi::c_char,
    pub same_screen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub mode: ::core::ffi::c_int,
    pub detail: ::core::ffi::c_int,
    pub same_screen: ::core::ffi::c_int,
    pub focus: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub mode: ::core::ffi::c_int,
    pub detail: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub drawable: Drawable,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub major_code: ::core::ffi::c_int,
    pub minor_code: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub drawable: Drawable,
    pub major_code: ::core::ffi::c_int,
    pub minor_code: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub state: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub parent: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
    pub from_configure: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub parent: Window,
    pub window: Window,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub above: Window,
    pub override_redirect: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub parent: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub above: Window,
    pub detail: ::core::ffi::c_int,
    pub value_mask: ::core::ffi::c_ulong,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub event: Window,
    pub window: Window,
    pub place: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub parent: Window,
    pub window: Window,
    pub place: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub colormap: Colormap,
    pub new: ::core::ffi::c_int,
    pub state: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub message_type: Atom,
    pub format: ::core::ffi::c_int,
    pub data: C2RustUnnamed_0_WmArgb,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub request: ::core::ffi::c_int,
    pub first_keycode: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub window: Window,
    pub key_vector: [::core::ffi::c_char; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub extension: ::core::ffi::c_int,
    pub evtype: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub xany: XAnyEvent_WmInitWs,
    pub xkey: XKeyEvent_WmInitWs,
    pub xbutton: XButtonEvent_WmInitWs,
    pub xmotion: XMotionEvent_WmInitWs,
    pub xcrossing: XCrossingEvent_WmInitWs,
    pub xfocus: XFocusChangeEvent_WmInitWs,
    pub xexpose: XExposeEvent_WmInitWs,
    pub xgraphicsexpose: XGraphicsExposeEvent_WmInitWs,
    pub xnoexpose: XNoExposeEvent_WmInitWs,
    pub xvisibility: XVisibilityEvent_WmInitWs,
    pub xcreatewindow: XCreateWindowEvent_WmInitWs,
    pub xdestroywindow: XDestroyWindowEvent_WmInitWs,
    pub xunmap: XUnmapEvent_WmInitWs,
    pub xmap: XMapEvent_WmInitWs,
    pub xmaprequest: XMapRequestEvent_WmInitWs,
    pub xreparent: XReparentEvent_WmInitWs,
    pub xconfigure: XConfigureEvent_WmInitWs,
    pub xgravity: XGravityEvent_WmInitWs,
    pub xresizerequest: XResizeRequestEvent_WmInitWs,
    pub xconfigurerequest: XConfigureRequestEvent_WmInitWs,
    pub xcirculate: XCirculateEvent_WmInitWs,
    pub xcirculaterequest: XCirculateRequestEvent_WmInitWs,
    pub xproperty: XPropertyEvent_WmInitWs,
    pub xselectionclear: XSelectionClearEvent_WmInitWs,
    pub xselectionrequest: XSelectionRequestEvent_WmInitWs,
    pub xselection: XSelectionEvent_WmInitWs,
    pub xcolormap: XColormapEvent_WmInitWs,
    pub xclient: XClientMessageEvent_WmInitWs,
    pub xmapping: XMappingEvent_WmInitWs,
    pub xerror: XErrorEvent_WmInitWs,
    pub xkeymap: XKeymapEvent_WmInitWs,
    pub xgeneric: XGenericEvent_WmInitWs,
    pub xcookie: XGenericEventCookie_WmInitWs,
    pub pad: [::core::ffi::c_long; 24],
}

pub type XEvent_WmInitWs = _XEvent_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XSQEvent {
    pub next: *mut _XSQEvent,
    pub event: XEvent_WmInitWs,
    pub qserial_num: ::core::ffi::c_ulong,
}

pub type CreateGCType = Option<
    unsafe extern "C" fn(*mut Display_WmInitWs, GC_WmInitWs, *mut XExtCodes) -> ::core::ffi::c_int,
>;

pub type CopyGCType = Option<
    unsafe extern "C" fn(*mut Display_WmInitWs, GC_WmInitWs, *mut XExtCodes) -> ::core::ffi::c_int,
>;

pub type FlushGCType = Option<
    unsafe extern "C" fn(*mut Display_WmInitWs, GC_WmInitWs, *mut XExtCodes) -> ::core::ffi::c_int,
>;

pub type FreeGCType = Option<
    unsafe extern "C" fn(*mut Display_WmInitWs, GC_WmInitWs, *mut XExtCodes) -> ::core::ffi::c_int,
>;

pub type CreateFontType = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        *mut XFontStruct,
        *mut XExtCodes,
    ) -> ::core::ffi::c_int,
>;

pub type FreeFontType = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        *mut XFontStruct,
        *mut XExtCodes,
    ) -> ::core::ffi::c_int,
>;

pub type CloseDisplayType =
    Option<unsafe extern "C" fn(*mut Display_WmInitWs, *mut XExtCodes) -> ::core::ffi::c_int>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xError {
    pub type_0: BYTE,
    pub errorCode: BYTE,
    pub sequenceNumber: CARD16,
    pub resourceID: CARD32,
    pub minorCode: CARD16,
    pub majorCode: CARD8,
    pub pad1: BYTE,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}

pub type ErrorType = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        *mut xError,
        *mut XExtCodes,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;

pub type ErrorStringType = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        ::core::ffi::c_int,
        *mut XExtCodes,
        *mut ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char,
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub resourceid: XID,
    pub serial: ::core::ffi::c_ulong,
    pub error_code: ::core::ffi::c_uchar,
    pub request_code: ::core::ffi::c_uchar,
    pub minor_code: ::core::ffi::c_uchar,
}

pub type PrintErrorType = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        *mut XErrorEvent_WmInitWs,
        *mut ::core::ffi::c_void,
    ) -> (),
>;

pub type BeforeFlushType = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        *mut XExtCodes,
        *const ::core::ffi::c_char,
        ::core::ffi::c_long,
    ) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExten {
    pub next: *mut _XExten,
    pub codes: XExtCodes,
    pub create_GC: CreateGCType,
    pub copy_GC: CopyGCType,
    pub flush_GC: FlushGCType,
    pub free_GC: FreeGCType,
    pub create_Font: CreateFontType,
    pub free_Font: FreeFontType,
    pub close_display: CloseDisplayType,
    pub error: ErrorType,
    pub error_string: ErrorStringType,
    pub name: *mut ::core::ffi::c_char,
    pub error_values: PrintErrorType,
    pub before_flush: BeforeFlushType,
    pub next_flush: *mut _XExten,
}

pub type _XInternalConnectionProc =
    Option<unsafe extern "C" fn(*mut Display_WmInitWs, ::core::ffi::c_int, XPointer) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnectionInfo {
    pub fd: ::core::ffi::c_int,
    pub read_callback: _XInternalConnectionProc,
    pub call_data: XPointer,
    pub watch_data: *mut XPointer,
    pub next: *mut _XConnectionInfo,
}

pub type XConnectionWatchProc = Option<
    unsafe extern "C" fn(
        *mut Display_WmInitWs,
        XPointer,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut XPointer,
    ) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnWatchInfo {
    pub fn_0: XConnectionWatchProc,
    pub client_data: XPointer,
    pub next: *mut _XConnWatchInfo,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29_WmInitWs {
    pub type_0: BYTE,
    pub detail: BYTE,
    pub sequenceNumber: CARD16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28_WmInitWs {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub sameScreen: BOOL,
    pub pad1: BYTE,
}

pub type KeyButMask = CARD16;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27_WmInitWs {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub mode: BYTE,
    pub flags: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub mode: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub count: CARD16,
    pub pad2: CARD16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24_WmInitWs {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub minorEvent: CARD16,
    pub count: CARD16,
    pub majorEvent: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23_WmInitWs {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub minorEvent: CARD16,
    pub majorEvent: BYTE,
    pub bpad: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub state: CARD8,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21_WmInitWs {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub fromConfigure: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17_WmInitWs {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub aboveSibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14_WmInitWs {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub sibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub valueMask: CARD16,
    pub pad1: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub width: CARD16,
    pub height: CARD16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11_WmInitWs {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub place: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}

pub type CARD16 = ::core::ffi::c_ushort;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
    pub time: CARD32,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: CARD16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9_WmInitWs {
    pub pad00: CARD32,
    pub time: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8_WmInitWs {
    pub pad00: CARD32,
    pub time: CARD32,
    pub owner: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7_WmInitWs {
    pub pad00: CARD32,
    pub time: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}

pub type BOOL = CARD8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub colormap: CARD32,
    pub new: BOOL,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
}

pub type CARD8 = ::core::ffi::c_uchar;

pub type BYTE = CARD8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5_WmInitWs {
    pub pad00: CARD32,
    pub request: CARD8,
    pub firstKeyCode: CARD8,
    pub count: CARD8,
    pub pad1: BYTE,
}

pub type INT32 = ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4_WmInitWs {
    pub type_0: CARD32,
    pub longs0: INT32,
    pub longs1: INT32,
    pub longs2: INT32,
    pub longs3: INT32,
    pub longs4: INT32,
}

pub type INT16 = ::core::ffi::c_short;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3_WmInitWs {
    pub type_0: CARD32,
    pub shorts0: INT16,
    pub shorts1: INT16,
    pub shorts2: INT16,
    pub shorts3: INT16,
    pub shorts4: INT16,
    pub shorts5: INT16,
    pub shorts6: INT16,
    pub shorts7: INT16,
    pub shorts8: INT16,
    pub shorts9: INT16,
}

pub type INT8 = ::core::ffi::c_schar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2_WmInitWs {
    pub type_0: CARD32,
    pub bytes: [INT8; 20],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1_WmInitWs {
    pub l: C2RustUnnamed_4_WmInitWs,
    pub s: C2RustUnnamed_3_WmInitWs,
    pub b: C2RustUnnamed_2_WmInitWs,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0_WmInitWs {
    pub pad00: CARD32,
    pub window: CARD32,
    pub u: C2RustUnnamed_1_WmInitWs,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_WmInitWs {
    pub u: C2RustUnnamed_29_WmInitWs,
    pub keyButtonPointer: C2RustUnnamed_28_WmInitWs,
    pub enterLeave: C2RustUnnamed_27_WmInitWs,
    pub focus: C2RustUnnamed_26_WmInitWs,
    pub expose: C2RustUnnamed_25_WmInitWs,
    pub graphicsExposure: C2RustUnnamed_24_WmInitWs,
    pub noExposure: C2RustUnnamed_23_WmInitWs,
    pub visibility: C2RustUnnamed_22_WmInitWs,
    pub createNotify: C2RustUnnamed_21_WmInitWs,
    pub destroyNotify: C2RustUnnamed_20_WmInitWs,
    pub unmapNotify: C2RustUnnamed_19_WmInitWs,
    pub mapNotify: C2RustUnnamed_18_WmInitWs,
    pub mapRequest: C2RustUnnamed_17_WmInitWs,
    pub reparent: C2RustUnnamed_16_WmInitWs,
    pub configureNotify: C2RustUnnamed_15_WmInitWs,
    pub configureRequest: C2RustUnnamed_14_WmInitWs,
    pub gravity: C2RustUnnamed_13_WmInitWs,
    pub resizeRequest: C2RustUnnamed_12_WmInitWs,
    pub circulate: C2RustUnnamed_11_WmInitWs,
    pub property: C2RustUnnamed_10_WmInitWs,
    pub selectionClear: C2RustUnnamed_9_WmInitWs,
    pub selectionRequest: C2RustUnnamed_8_WmInitWs,
    pub selectionNotify: C2RustUnnamed_7_WmInitWs,
    pub colormap: C2RustUnnamed_6_WmInitWs,
    pub mappingNotify: C2RustUnnamed_5_WmInitWs,
    pub clientMessage: C2RustUnnamed_0_WmInitWs,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xEvent {
    pub u: C2RustUnnamed_WmInitWs,
}

pub type xEvent = _xEvent;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie_WmInitWs {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display_WmInitWs,
    pub extension: ::core::ffi::c_int,
    pub evtype: ::core::ffi::c_int,
    pub cookie: ::core::ffi::c_uint,
    pub data: *mut ::core::ffi::c_void,
}

pub type Display_WmInitWs = _XDisplay_WmInitWs;

pub type XIOErrorExitHandler =
    Option<unsafe extern "C" fn(*mut Display_WmInitWs, *mut ::core::ffi::c_void) -> ()>;

pub type xthread_t = pthread_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XDisplay_WmInitWs {
    pub ext_data: *mut XExtData,
    pub free_funcs: *mut _XFreeFuncs,
    pub fd: ::core::ffi::c_int,
    pub conn_checker: ::core::ffi::c_int,
    pub proto_major_version: ::core::ffi::c_int,
    pub proto_minor_version: ::core::ffi::c_int,
    pub vendor: *mut ::core::ffi::c_char,
    pub resource_base: XID,
    pub resource_mask: XID,
    pub resource_id: XID,
    pub resource_shift: ::core::ffi::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay_WmInitWs) -> XID>,
    pub byte_order: ::core::ffi::c_int,
    pub bitmap_unit: ::core::ffi::c_int,
    pub bitmap_pad: ::core::ffi::c_int,
    pub bitmap_bit_order: ::core::ffi::c_int,
    pub nformats: ::core::ffi::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub vnumber: ::core::ffi::c_int,
    pub release: ::core::ffi::c_int,
    pub head: *mut _XSQEvent,
    pub tail: *mut _XSQEvent,
    pub qlen: ::core::ffi::c_int,
    pub last_request_read: ::core::ffi::c_ulong,
    pub request: ::core::ffi::c_ulong,
    pub last_req: *mut ::core::ffi::c_char,
    pub buffer: *mut ::core::ffi::c_char,
    pub bufptr: *mut ::core::ffi::c_char,
    pub bufmax: *mut ::core::ffi::c_char,
    pub max_request_size: ::core::ffi::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub synchandler: Option<unsafe extern "C" fn(*mut _XDisplay_WmInitWs) -> ::core::ffi::c_int>,
    pub display_name: *mut ::core::ffi::c_char,
    pub default_screen: ::core::ffi::c_int,
    pub nscreens: ::core::ffi::c_int,
    pub screens: *mut Screen_WmInitWs,
    pub motion_buffer: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_ulong,
    pub min_keycode: ::core::ffi::c_int,
    pub max_keycode: ::core::ffi::c_int,
    pub keysyms: *mut KeySym,
    pub modifiermap: *mut XModifierKeymap,
    pub keysyms_per_keycode: ::core::ffi::c_int,
    pub xdefaults: *mut ::core::ffi::c_char,
    pub scratch_buffer: *mut ::core::ffi::c_char,
    pub scratch_length: ::core::ffi::c_ulong,
    pub ext_number: ::core::ffi::c_int,
    pub ext_procs: *mut _XExten,
    pub event_vec: [Option<
        unsafe extern "C" fn(
            *mut Display_WmInitWs,
            *mut XEvent_WmInitWs,
            *mut xEvent,
        ) -> ::core::ffi::c_int,
    >; 128],
    pub wire_vec: [Option<
        unsafe extern "C" fn(
            *mut Display_WmInitWs,
            *mut XEvent_WmInitWs,
            *mut xEvent,
        ) -> ::core::ffi::c_int,
    >; 128],
    pub lock_meaning: KeySym,
    pub lock: *mut _XLockInfo,
    pub async_handlers: *mut _XInternalAsync,
    pub bigreq_size: ::core::ffi::c_ulong,
    pub lock_fns: *mut _XLockPtrs,
    pub idlist_alloc:
        Option<unsafe extern "C" fn(*mut Display_WmInitWs, *mut XID, ::core::ffi::c_int) -> ()>,
    pub key_bindings: *mut _XKeytrans,
    pub cursor_font: Font,
    pub atoms: *mut _XDisplayAtoms,
    pub mode_switch: ::core::ffi::c_uint,
    pub num_lock: ::core::ffi::c_uint,
    pub context_db: *mut _XContextDB,
    pub error_vec: *mut Option<
        unsafe extern "C" fn(
            *mut Display_WmInitWs,
            *mut XErrorEvent_WmInitWs,
            *mut xError,
        ) -> ::core::ffi::c_int,
    >,
    pub cms: C2RustUnnamed_31_WmInitWs,
    pub im_filters: *mut _XIMFilter,
    pub qfree: *mut _XSQEvent,
    pub next_event_serial_num: ::core::ffi::c_ulong,
    pub flushes: *mut _XExten,
    pub im_fd_info: *mut _XConnectionInfo,
    pub im_fd_length: ::core::ffi::c_int,
    pub conn_watchers: *mut _XConnWatchInfo,
    pub watcher_count: ::core::ffi::c_int,
    pub filedes: XPointer,
    pub savedsynchandler: Option<unsafe extern "C" fn(*mut Display_WmInitWs) -> ::core::ffi::c_int>,
    pub resource_max: XID,
    pub xcmisc_opcode: ::core::ffi::c_int,
    pub xkb_info: *mut _XkbInfoRec,
    pub trans_conn: *mut _XtransConnInfo,
    pub xcb: *mut _X11XCBPrivate,
    pub next_cookie: ::core::ffi::c_uint,
    pub generic_event_vec: [Option<
        unsafe extern "C" fn(
            *mut Display_WmInitWs,
            *mut XGenericEventCookie_WmInitWs,
            *mut xEvent,
        ) -> ::core::ffi::c_int,
    >; 128],
    pub generic_event_copy_vec: [Option<
        unsafe extern "C" fn(
            *mut Display_WmInitWs,
            *mut XGenericEventCookie_WmInitWs,
            *mut XGenericEventCookie_WmInitWs,
        ) -> ::core::ffi::c_int,
    >; 128],
    pub cookiejar: *mut ::core::ffi::c_void,
    pub error_threads: *mut _XErrorThreadInfo,
    pub exit_handler: XIOErrorExitHandler,
    pub exit_handler_data: *mut ::core::ffi::c_void,
    pub in_ifevent: CARD32,
    pub ifevent_thread: xthread_t,
}

pub type GC_WmInitWs = *mut _XGC_WmInitWs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGC_WmInitWs {
    pub ext_data: *mut XExtData,
    pub gid: GContext,
    pub rects: ::core::ffi::c_int,
    pub dashes: ::core::ffi::c_int,
    pub dirty: ::core::ffi::c_ulong,
    pub values: XGCValues,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExtCodes {
    pub extension: ::core::ffi::c_int,
    pub major_opcode: ::core::ffi::c_int,
    pub first_event: ::core::ffi::c_int,
    pub first_error: ::core::ffi::c_int,
}

pub type GContext = XID;

pub type pthread_t = ::core::ffi::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XLockInfo {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XKeytrans {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XDisplayAtoms {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XContextDB {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XIMFilter {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XkbInfoRec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XtransConnInfo {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _X11XCBPrivate {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XErrorThreadInfo {
    _private: [u8; 0],
}

pub type XtFilePredicate = Option<unsafe extern "C" fn(String_0) -> Boolean>;

pub type Substitution = *mut SubstitutionRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubstitutionRec {
    pub match_0: ::core::ffi::c_char,
    pub substitution: *mut ::core::ffi::c_char,
}

pub type WsClientData_WmIDecor = _WsClientData_WmFeedback;

pub type XmTraverseObscuredCallbackStruct = _XmTraverseObscuredCallbackStruct;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmTraverseObscuredCallbackStruct {
    pub reason: ::core::ffi::c_int,
    pub event: *mut XEvent,
    pub traversal_destination: Widget_WmFeedback,
    pub direction: XmTraversalDirection,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmAnyCallbackStruct {
    pub reason: ::core::ffi::c_int,
    pub event: *mut XEvent,
}

pub type XtEventHandler_WmIconBox =
    Option<unsafe extern "C" fn(Widget_WmFeedback, XtPointer, *mut XEvent, *mut Boolean) -> ()>;

pub type XImage = _XImage;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(*mut _XImage) -> ::core::ffi::c_int>,
    pub get_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong,
    >,
    pub put_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int,
    >,
    pub sub_image: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel:
        Option<unsafe extern "C" fn(*mut _XImage, ::core::ffi::c_long) -> ::core::ffi::c_int>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub xoffset: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_char,
    pub byte_order: ::core::ffi::c_int,
    pub bitmap_unit: ::core::ffi::c_int,
    pub bitmap_bit_order: ::core::ffi::c_int,
    pub bitmap_pad: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub bytes_per_line: ::core::ffi::c_int,
    pub bits_per_pixel: ::core::ffi::c_int,
    pub red_mask: ::core::ffi::c_ulong,
    pub green_mask: ::core::ffi::c_ulong,
    pub blue_mask: ::core::ffi::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}

pub type XmStringCharSet = *mut ::core::ffi::c_char;

pub type ConfirmFunc = Option<unsafe extern "C" fn(Boolean) -> ()>;

pub type ewmh_atom = ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2_WmEwmh {
    pub i: *mut ::core::ffi::c_ulong,
    pub b: *mut ::core::ffi::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmButtonEventStatusRec {
    pub time: Time,
    pub verified: Boolean,
    pub waiting_to_be_managed: Boolean,
    pub event: XButtonEvent,
}

pub type XmButtonEventStatusRec = _XmButtonEventStatusRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmReplayInfoRec {
    pub time: Time,
    pub toplevel_menu: Widget,
}

pub type XmReplayInfoRec = _XmReplayInfoRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmMenuFocusRec {
    pub oldWidget: Widget,
    pub oldFocus: Window,
    pub oldRevert: ::core::ffi::c_int,
    pub oldTime: Time,
}

pub type XmMenuFocusRec = _XmMenuFocusRec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmMenuStateRec {
    pub RC_LastSelectToplevel: Widget,
    pub RC_ButtonEventStatus: XmButtonEventStatusRec,
    pub RC_ReplayInfo: XmReplayInfoRec,
    pub RC_activeItem: Widget,
    pub RC_menuFocus: XmMenuFocusRec,
    pub RC_allowAcceleratedInsensitiveUnmanagedMenuItems: Boolean,
    pub MS_LastManagedMenuTime: Time,
    pub MU_InDragMode: Boolean,
    pub MU_CurrentMenuChild: Widget,
    pub MU_InPMMode: Boolean,
}

pub type XmMenuState = *mut _XmMenuStateRec;

pub type XFocusInEvent = XFocusChangeEvent;

pub type XLeaveWindowEvent = XCrossingEvent;

pub type XtErrorHandler = Option<unsafe extern "C" fn(String_0) -> ()>;

pub type XIOErrorHandler = Option<unsafe extern "C" fn(*mut Display) -> ::core::ffi::c_int>;

pub type XErrorHandler =
    Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> ::core::ffi::c_int>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XShapeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub kind: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub time: Time,
    pub shaped: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bevel_Count {
    pub top: Single_Bevel_Count,
    pub bottom: Single_Bevel_Count,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Single_Bevel_Count {
    pub external: ::core::ffi::c_int,
    pub join: ::core::ffi::c_int,
    pub internal: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

pub type __syscall_slong_t = ::core::ffi::c_long;

pub type __blkcnt_t = ::core::ffi::c_long;

pub type __blksize_t = ::core::ffi::c_long;

pub type __time_t = ::core::ffi::c_long;

pub type __nlink_t = ::core::ffi::c_ulong;

pub type __mode_t = ::core::ffi::c_uint;

pub type __ino_t = ::core::ffi::c_ulong;

pub type __gid_t = ::core::ffi::c_uint;

pub type __dev_t = ::core::ffi::c_ulong;

pub type ArgbFrame = _ArgbFrame_WmArgb;

pub type XRenderPictureAttributes = _XRenderPictureAttributes;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRenderPictureAttributes {
    pub repeat: ::core::ffi::c_int,
    pub alpha_map: Picture,
    pub alpha_x_origin: ::core::ffi::c_int,
    pub alpha_y_origin: ::core::ffi::c_int,
    pub clip_x_origin: ::core::ffi::c_int,
    pub clip_y_origin: ::core::ffi::c_int,
    pub clip_mask: Pixmap,
    pub graphics_exposures: ::core::ffi::c_int,
    pub subwindow_mode: ::core::ffi::c_int,
    pub poly_edge: ::core::ffi::c_int,
    pub poly_mode: ::core::ffi::c_int,
    pub dither: Atom,
    pub component_alpha: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_0: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: ::core::ffi::c_short,
    pub redMask: ::core::ffi::c_short,
    pub green: ::core::ffi::c_short,
    pub greenMask: ::core::ffi::c_short,
    pub blue: ::core::ffi::c_short,
    pub blueMask: ::core::ffi::c_short,
    pub alpha: ::core::ffi::c_short,
    pub alphaMask: ::core::ffi::c_short,
}

pub type PictFormat = XID;

// Sizes and offsets recorded from the patched WmGlobal.h and Xt public headers
// with the distribution's Debian amd64 toolchain. These records cross module
// and library boundaries, including the Xt resource callback tables.
const _: () = {
    use std::mem::{align_of, offset_of, size_of};
    assert!(size_of::<WmGlobalData>() == 2312 && align_of::<WmGlobalData>() == 8);
    assert!(size_of::<WmScreenData>() == 1816 && align_of::<WmScreenData>() == 8);
    assert!(size_of::<WmWorkspaceData>() == 192 && align_of::<WmWorkspaceData>() == 8);
    assert!(size_of::<ClientData>() == 1168 && align_of::<ClientData>() == 8);
    assert!(size_of::<XEvent>() == 192 && align_of::<XEvent>() == 8);
    assert!(size_of::<XtResource>() == 48 && align_of::<XtResource>() == 8);
    assert!(offset_of!(WmGlobalData, display) == 96);
    assert!(offset_of!(WmGlobalData, keyboardFocus) == 456);
    assert!(offset_of!(ClientData, client) == 8);
    assert!(offset_of!(ClientData, clientX) == 664);
    assert!(offset_of!(ClientData, tileState) == 680);
    assert!(offset_of!(WmWorkspaceData, id) == 8);
    assert!(offset_of!(WmScreenData, pActiveWS) == 320);
    assert!(offset_of!(WmScreenData, numWorkspaces) == 1784);
};
