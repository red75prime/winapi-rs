// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of urlmon.h

ENUM!{ enum IEObjectType {
    IE_EPM_OBJECT_EVENT = 0,
    IE_EPM_OBJECT_MUTEX = 1,
    IE_EPM_OBJECT_SEMAPHORE = 2,
    IE_EPM_OBJECT_SHARED_MEMORY = 3,
    IE_EPM_OBJECT_WAITABLE_TIMER = 4,
    IE_EPM_OBJECT_FILE = 5,
    IE_EPM_OBJECT_NAMED_PIPE = 6,
    IE_EPM_OBJECT_REGISTRY = 7,
}}

ENUM!{ enum _URLZONEREG {
    URLZONEREG_DEFAULT = 0,
    URLZONEREG_HKLM = 1,
    URLZONEREG_HKCU = 2,
}}

ENUM!{ enum __MIDL_IAuthenticateEx_0001 {
    AUTHENTICATEF_PROXY = 1,
    AUTHENTICATEF_BASIC = 2,
    AUTHENTICATEF_HTTP = 4,
}}

ENUM!{ enum __MIDL_IBindStatusCallbackEx_0001 {
    BINDF2_DISABLEBASICOVERHTTP = 1,
    BINDF2_DISABLEAUTOCOOKIEHANDLING = 2,
    BINDF2_READ_DATA_GREATER_THAN_4GB = 4,
    BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID = 8,
    BINDF2_SETDOWNLOADMODE = 32,
    BINDF2_DISABLE_HTTP_REDIRECT_CACHING = 64,
    BINDF2_KEEP_CALLBACK_MODULE_LOADED = 128,
    BINDF2_ALLOW_PROXY_CRED_PROMPT = 256,
    BINDF2_RESERVED_17 = 512,
    BINDF2_RESERVED_16 = 1024,
    BINDF2_RESERVED_15 = 2048,
    BINDF2_RESERVED_14 = 4096,
    BINDF2_RESERVED_13 = 8192,
    BINDF2_RESERVED_12 = 16384,
    BINDF2_RESERVED_11 = 32768,
    BINDF2_RESERVED_10 = 65536,
    BINDF2_RESERVED_F = 131072,
    BINDF2_RESERVED_E = 262144,
    BINDF2_RESERVED_D = 524288,
    BINDF2_RESERVED_C = 1048576,
    BINDF2_RESERVED_B = 2097152,
    BINDF2_RESERVED_A = 4194304,
    BINDF2_RESERVED_9 = 8388608,
    BINDF2_RESERVED_8 = 16777216,
    BINDF2_RESERVED_7 = 33554432,
    BINDF2_RESERVED_6 = 67108864,
    BINDF2_RESERVED_5 = 134217728,
    BINDF2_RESERVED_4 = 268435456,
    BINDF2_RESERVED_3 = 536870912,
    BINDF2_RESERVED_2 = 1073741824,
    BINDF2_RESERVED_1 = 2147483648,
}}

ENUM!{ enum __MIDL_IBindStatusCallback_0001 {
    BINDVERB_GET = 0,
    BINDVERB_POST = 1,
    BINDVERB_PUT = 2,
    BINDVERB_CUSTOM = 3,
    BINDVERB_RESERVED1 = 4,
}}

ENUM!{ enum __MIDL_IBindStatusCallback_0002 {
    BINDINFOF_URLENCODESTGMEDDATA = 1,
    BINDINFOF_URLENCODEDEXTRAINFO = 2,
}}

ENUM!{ enum __MIDL_IBindStatusCallback_0003 {
    BINDF_ASYNCHRONOUS = 1,
    BINDF_ASYNCSTORAGE = 2,
    BINDF_NOPROGRESSIVERENDERING = 4,
    BINDF_OFFLINEOPERATION = 8,
    BINDF_GETNEWESTVERSION = 16,
    BINDF_NOWRITECACHE = 32,
    BINDF_NEEDFILE = 64,
    BINDF_PULLDATA = 128,
    BINDF_IGNORESECURITYPROBLEM = 256,
    BINDF_RESYNCHRONIZE = 512,
    BINDF_HYPERLINK = 1024,
    BINDF_NO_UI = 2048,
    BINDF_SILENTOPERATION = 4096,
    BINDF_PRAGMA_NO_CACHE = 8192,
    BINDF_GETCLASSOBJECT = 16384,
    BINDF_RESERVED_1 = 32768,
    BINDF_FREE_THREADED = 65536,
    BINDF_DIRECT_READ = 131072,
    BINDF_FORMS_SUBMIT = 262144,
    BINDF_GETFROMCACHE_IF_NET_FAIL = 524288,
    BINDF_FROMURLMON = 1048576,
    BINDF_FWD_BACK = 2097152,
    BINDF_PREFERDEFAULTHANDLER = 4194304,
    BINDF_ENFORCERESTRICTED = 8388608,
    BINDF_RESERVED_2 = 2147483648,
    BINDF_RESERVED_3 = 16777216,
    BINDF_RESERVED_4 = 33554432,
    BINDF_RESERVED_5 = 67108864,
    BINDF_RESERVED_6 = 134217728,
    BINDF_RESERVED_7 = 1073741824,
    BINDF_RESERVED_8 = 536870912,
}}

ENUM!{ enum __MIDL_IBindStatusCallback_0004 {
    URL_ENCODING_NONE = 0,
    URL_ENCODING_ENABLE_UTF8 = 268435456,
    URL_ENCODING_DISABLE_UTF8 = 536870912,
}}

ENUM!{ enum __MIDL_IBindStatusCallback_0005 {
    BINDINFO_OPTIONS_WININETFLAG = 65536,
    BINDINFO_OPTIONS_ENABLE_UTF8 = 131072,
    BINDINFO_OPTIONS_DISABLE_UTF8 = 262144,
    BINDINFO_OPTIONS_USE_IE_ENCODING = 524288,
    BINDINFO_OPTIONS_BINDTOOBJECT = 1048576,
    BINDINFO_OPTIONS_SECURITYOPTOUT = 2097152,
    BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN = 4194304,
    BINDINFO_OPTIONS_USEBINDSTRINGCREDS = 8388608,
    BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS = 16777216,
    BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE = 33554432,
    BINDINFO_WPC_DOWNLOADBLOCKED = 134217728,
    BINDINFO_WPC_LOGGING_ENABLED = 268435456,
    BINDINFO_OPTIONS_ALLOWCONNECTDATA = 536870912,
    BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS = 1073741824,
    BINDINFO_OPTIONS_SHDOCVW_NAVIGATE = 2147483648,
}}

ENUM!{ enum __MIDL_IBindStatusCallback_0006 {
    BSCF_FIRSTDATANOTIFICATION = 1,
    BSCF_INTERMEDIATEDATANOTIFICATION = 2,
    BSCF_LASTDATANOTIFICATION = 4,
    BSCF_DATAFULLYAVAILABLE = 8,
    BSCF_AVAILABLEDATASIZEUNKNOWN = 16,
    BSCF_SKIPDRAINDATAFORFILEURLS = 32,
    BSCF_64BITLENGTHDOWNLOAD = 64,
}}

ENUM!{ enum __MIDL_ICodeInstall_0001 {
    CIP_DISK_FULL = 0,
    CIP_ACCESS_DENIED = 1,
    CIP_NEWER_VERSION_EXISTS = 2,
    CIP_OLDER_VERSION_EXISTS = 3,
    CIP_NAME_CONFLICT = 4,
    CIP_TRUST_VERIFICATION_COMPONENT_MISSING = 5,
    CIP_EXE_SELF_REGISTERATION_TIMEOUT = 6,
    CIP_UNSAFE_TO_ABORT = 7,
    CIP_NEED_REBOOT = 8,
    CIP_NEED_REBOOT_UI_PERMISSION = 9,
}}

ENUM!{ enum __MIDL_IGetBindHandle_0001 {
    BINDHANDLETYPES_APPCACHE = 0,
    BINDHANDLETYPES_DEPENDENCY = 1,
    BINDHANDLETYPES_COUNT = 2,
}}

ENUM!{ enum __MIDL_IInternetSecurityManager_0001 {
    PUAF_DEFAULT = 0,
    PUAF_NOUI = 1,
    PUAF_ISFILE = 2,
    PUAF_WARN_IF_DENIED = 4,
    PUAF_FORCEUI_FOREGROUND = 8,
    PUAF_CHECK_TIFS = 16,
    PUAF_DONTCHECKBOXINDIALOG = 32,
    PUAF_TRUSTED = 64,
    PUAF_ACCEPT_WILDCARD_SCHEME = 128,
    PUAF_ENFORCERESTRICTED = 256,
    PUAF_NOSAVEDFILECHECK = 512,
    PUAF_REQUIRESAVEDFILECHECK = 1024,
    PUAF_DONT_USE_CACHE = 4096,
    PUAF_RESERVED1 = 8192,
    PUAF_RESERVED2 = 16384,
    PUAF_LMZ_UNLOCKED = 65536,
    PUAF_LMZ_LOCKED = 131072,
    PUAF_DEFAULTZONEPOL = 262144,
    PUAF_NPL_USE_LOCKED_IF_RESTRICTED = 524288,
    PUAF_NOUIIFLOCKED = 1048576,
    PUAF_DRAGPROTOCOLCHECK = 2097152,
}}

ENUM!{ enum __MIDL_IInternetSecurityManager_0002 {
    PUAFOUT_DEFAULT = 0,
    PUAFOUT_ISLOCKZONEPOLICY = 1,
}}

ENUM!{ enum __MIDL_IInternetSecurityManager_0003 {
    SZM_CREATE = 0,
    SZM_DELETE = 1,
}}

ENUM!{ enum __MIDL_IInternetZoneManager_0001 {
    MAX_ZONE_PATH = 260,
    MAX_ZONE_DESCRIPTION = 200,
}}

ENUM!{ enum __MIDL_IInternetZoneManager_0002 {
    ZAFLAGS_CUSTOM_EDIT = 1,
    ZAFLAGS_ADD_SITES = 2,
    ZAFLAGS_REQUIRE_VERIFICATION = 4,
    ZAFLAGS_INCLUDE_PROXY_OVERRIDE = 8,
    ZAFLAGS_INCLUDE_INTRANET_SITES = 16,
    ZAFLAGS_NO_UI = 32,
    ZAFLAGS_SUPPORTS_VERIFICATION = 64,
    ZAFLAGS_UNC_AS_INTRANET = 128,
    ZAFLAGS_DETECT_INTRANET = 256,
    ZAFLAGS_USE_LOCKED_ZONES = 65536,
    ZAFLAGS_VERIFY_TEMPLATE_SETTINGS = 131072,
    ZAFLAGS_NO_CACHE = 262144,
}}

ENUM!{ enum __MIDL_IMonikerProp_0001 {
    MIMETYPEPROP = 0,
    USE_SRC_URL = 1,
    CLASSIDPROP = 2,
    TRUSTEDDOWNLOADPROP = 3,
    POPUPLEVELPROP = 4,
}}

ENUM!{ enum __MIDL_IUri_0001 {
    Uri_PROPERTY_ABSOLUTE_URI = 0,
    Uri_PROPERTY_STRING_START = 0,
    Uri_PROPERTY_AUTHORITY = 1,
    Uri_PROPERTY_DISPLAY_URI = 2,
    Uri_PROPERTY_DOMAIN = 3,
    Uri_PROPERTY_EXTENSION = 4,
    Uri_PROPERTY_FRAGMENT = 5,
    Uri_PROPERTY_HOST = 6,
    Uri_PROPERTY_PASSWORD = 7,
    Uri_PROPERTY_PATH = 8,
    Uri_PROPERTY_PATH_AND_QUERY = 9,
    Uri_PROPERTY_QUERY = 10,
    Uri_PROPERTY_RAW_URI = 11,
    Uri_PROPERTY_SCHEME_NAME = 12,
    Uri_PROPERTY_USER_INFO = 13,
    Uri_PROPERTY_USER_NAME = 14,
    Uri_PROPERTY_STRING_LAST = 14,
    Uri_PROPERTY_HOST_TYPE = 15,
    Uri_PROPERTY_DWORD_START = 15,
    Uri_PROPERTY_PORT = 16,
    Uri_PROPERTY_SCHEME = 17,
    Uri_PROPERTY_ZONE = 18,
    Uri_PROPERTY_DWORD_LAST = 18,
}}

ENUM!{ enum __MIDL_IUri_0002 {
    Uri_HOST_UNKNOWN = 0,
    Uri_HOST_DNS = 1,
    Uri_HOST_IPV4 = 2,
    Uri_HOST_IPV6 = 3,
    Uri_HOST_IDN = 4,
}}

ENUM!{ enum _tagINTERNETFEATURELIST {
    FEATURE_OBJECT_CACHING = 0,
    FEATURE_ZONE_ELEVATION = 1,
    FEATURE_MIME_HANDLING = 2,
    FEATURE_MIME_SNIFFING = 3,
    FEATURE_WINDOW_RESTRICTIONS = 4,
    FEATURE_WEBOC_POPUPMANAGEMENT = 5,
    FEATURE_BEHAVIORS = 6,
    FEATURE_DISABLE_MK_PROTOCOL = 7,
    FEATURE_LOCALMACHINE_LOCKDOWN = 8,
    FEATURE_SECURITYBAND = 9,
    FEATURE_RESTRICT_ACTIVEXINSTALL = 10,
    FEATURE_VALIDATE_NAVIGATE_URL = 11,
    FEATURE_RESTRICT_FILEDOWNLOAD = 12,
    FEATURE_ADDON_MANAGEMENT = 13,
    FEATURE_PROTOCOL_LOCKDOWN = 14,
    FEATURE_HTTP_USERNAME_PASSWORD_DISABLE = 15,
    FEATURE_SAFE_BINDTOOBJECT = 16,
    FEATURE_UNC_SAVEDFILECHECK = 17,
    FEATURE_GET_URL_DOM_FILEPATH_UNENCODED = 18,
    FEATURE_TABBED_BROWSING = 19,
    FEATURE_SSLUX = 20,
    FEATURE_DISABLE_NAVIGATION_SOUNDS = 21,
    FEATURE_DISABLE_LEGACY_COMPRESSION = 22,
    FEATURE_FORCE_ADDR_AND_STATUS = 23,
    FEATURE_XMLHTTP = 24,
    FEATURE_DISABLE_TELNET_PROTOCOL = 25,
    FEATURE_FEEDS = 26,
    FEATURE_BLOCK_INPUT_PROMPTS = 27,
    FEATURE_ENTRY_COUNT = 28,
}}

ENUM!{ enum _tagOIBDG_FLAGS {
    OIBDG_APARTMENTTHREADED = 256,
    OIBDG_DATAONLY = 4096,
}}

ENUM!{ enum _tagPARSEACTION {
    PARSE_CANONICALIZE = 1,
    PARSE_FRIENDLY = 2,
    PARSE_SECURITY_URL = 3,
    PARSE_ROOTDOCUMENT = 4,
    PARSE_DOCUMENT = 5,
    PARSE_ANCHOR = 6,
    PARSE_ENCODE_IS_UNESCAPE = 7,
    PARSE_DECODE_IS_ESCAPE = 8,
    PARSE_PATH_FROM_URL = 9,
    PARSE_URL_FROM_PATH = 10,
    PARSE_MIME = 11,
    PARSE_SERVER = 12,
    PARSE_SCHEMA = 13,
    PARSE_SITE = 14,
    PARSE_DOMAIN = 15,
    PARSE_LOCATION = 16,
    PARSE_SECURITY_DOMAIN = 17,
    PARSE_ESCAPE = 18,
    PARSE_UNESCAPE = 19,
}}

ENUM!{ enum _tagPI_FLAGS {
    PI_PARSE_URL = 1,
    PI_FILTER_MODE = 2,
    PI_FORCE_ASYNC = 4,
    PI_USE_WORKERTHREAD = 8,
    PI_MIMEVERIFICATION = 16,
    PI_CLSIDLOOKUP = 32,
    PI_DATAPROGRESS = 64,
    PI_SYNCHRONOUS = 128,
    PI_APARTMENTTHREADED = 256,
    PI_CLASSINSTALL = 512,
    PI_PASSONBINDCTX = 8192,
    PI_NOMIMEHANDLER = 32768,
    PI_LOADAPPDIRECT = 16384,
    PD_FORCE_SWITCH = 65536,
    PI_PREFERDEFAULTHANDLER = 131072,
}}

ENUM!{ enum _tagPSUACTION {
    PSU_DEFAULT = 1,
    PSU_SECURITY_URL_ONLY = 2,
}}

ENUM!{ enum _tagQUERYOPTION {
    QUERY_EXPIRATION_DATE = 1,
    QUERY_TIME_OF_LAST_CHANGE = 2,
    QUERY_CONTENT_ENCODING = 3,
    QUERY_CONTENT_TYPE = 4,
    QUERY_REFRESH = 5,
    QUERY_RECOMBINE = 6,
    QUERY_CAN_NAVIGATE = 7,
    QUERY_USES_NETWORK = 8,
    QUERY_IS_CACHED = 9,
    QUERY_IS_INSTALLEDENTRY = 10,
    QUERY_IS_CACHED_OR_MAPPED = 11,
    QUERY_USES_CACHE = 12,
    QUERY_IS_SECURE = 13,
    QUERY_IS_SAFE = 14,
    QUERY_USES_HISTORYFOLDER = 15,
    QUERY_IS_CACHED_AND_USABLE_OFFLINE = 16,
}}

ENUM!{ enum tagBINDSTATUS {
    BINDSTATUS_FINDINGRESOURCE = 1,
    BINDSTATUS_CONNECTING = 2,
    BINDSTATUS_REDIRECTING = 3,
    BINDSTATUS_BEGINDOWNLOADDATA = 4,
    BINDSTATUS_DOWNLOADINGDATA = 5,
    BINDSTATUS_ENDDOWNLOADDATA = 6,
    BINDSTATUS_BEGINDOWNLOADCOMPONENTS = 7,
    BINDSTATUS_INSTALLINGCOMPONENTS = 8,
    BINDSTATUS_ENDDOWNLOADCOMPONENTS = 9,
    BINDSTATUS_USINGCACHEDCOPY = 10,
    BINDSTATUS_SENDINGREQUEST = 11,
    BINDSTATUS_CLASSIDAVAILABLE = 12,
    BINDSTATUS_MIMETYPEAVAILABLE = 13,
    BINDSTATUS_CACHEFILENAMEAVAILABLE = 14,
    BINDSTATUS_BEGINSYNCOPERATION = 15,
    BINDSTATUS_ENDSYNCOPERATION = 16,
    BINDSTATUS_BEGINUPLOADDATA = 17,
    BINDSTATUS_UPLOADINGDATA = 18,
    BINDSTATUS_ENDUPLOADDATA = 19,
    BINDSTATUS_PROTOCOLCLASSID = 20,
    BINDSTATUS_ENCODING = 21,
    BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE = 22,
    BINDSTATUS_CLASSINSTALLLOCATION = 23,
    BINDSTATUS_DECODING = 24,
    BINDSTATUS_LOADINGMIMEHANDLER = 25,
    BINDSTATUS_CONTENTDISPOSITIONATTACH = 26,
    BINDSTATUS_FILTERREPORTMIMETYPE = 27,
    BINDSTATUS_CLSIDCANINSTANTIATE = 28,
    BINDSTATUS_IUNKNOWNAVAILABLE = 29,
    BINDSTATUS_DIRECTBIND = 30,
    BINDSTATUS_RAWMIMETYPE = 31,
    BINDSTATUS_PROXYDETECTING = 32,
    BINDSTATUS_ACCEPTRANGES = 33,
    BINDSTATUS_COOKIE_SENT = 34,
    BINDSTATUS_COMPACT_POLICY_RECEIVED = 35,
    BINDSTATUS_COOKIE_SUPPRESSED = 36,
    BINDSTATUS_COOKIE_STATE_UNKNOWN = 37,
    BINDSTATUS_COOKIE_STATE_ACCEPT = 38,
    BINDSTATUS_COOKIE_STATE_REJECT = 39,
    BINDSTATUS_COOKIE_STATE_PROMPT = 40,
    BINDSTATUS_COOKIE_STATE_LEASH = 41,
    BINDSTATUS_COOKIE_STATE_DOWNGRADE = 42,
    BINDSTATUS_POLICY_HREF = 43,
    BINDSTATUS_P3P_HEADER = 44,
    BINDSTATUS_SESSION_COOKIE_RECEIVED = 45,
    BINDSTATUS_PERSISTENT_COOKIE_RECEIVED = 46,
    BINDSTATUS_SESSION_COOKIES_ALLOWED = 47,
    BINDSTATUS_CACHECONTROL = 48,
    BINDSTATUS_CONTENTDISPOSITIONFILENAME = 49,
    BINDSTATUS_MIMETEXTPLAINMISMATCH = 50,
    BINDSTATUS_PUBLISHERAVAILABLE = 51,
    BINDSTATUS_DISPLAYNAMEAVAILABLE = 52,
    BINDSTATUS_SSLUX_NAVBLOCKED = 53,
    BINDSTATUS_SERVER_MIMETYPEAVAILABLE = 54,
    BINDSTATUS_SNIFFED_CLASSIDAVAILABLE = 55,
    BINDSTATUS_64BIT_PROGRESS = 56,
    BINDSTATUS_LAST = 56,
    BINDSTATUS_RESERVED_0 = 57,
    BINDSTATUS_RESERVED_1 = 58,
    BINDSTATUS_RESERVED_2 = 59,
    BINDSTATUS_RESERVED_3 = 60,
    BINDSTATUS_RESERVED_4 = 61,
    BINDSTATUS_RESERVED_5 = 62,
    BINDSTATUS_RESERVED_6 = 63,
    BINDSTATUS_RESERVED_7 = 64,
    BINDSTATUS_RESERVED_8 = 65,
    BINDSTATUS_RESERVED_9 = 66,
    BINDSTATUS_RESERVED_A = 67,
    BINDSTATUS_RESERVED_B = 68,
    BINDSTATUS_RESERVED_C = 69,
    BINDSTATUS_RESERVED_D = 70,
    BINDSTATUS_RESERVED_E = 71,
    BINDSTATUS_RESERVED_F = 72,
    BINDSTATUS_RESERVED_10 = 73,
    BINDSTATUS_RESERVED_11 = 74,
    BINDSTATUS_LAST_PRIVATE = 74,
}}

ENUM!{ enum tagBINDSTRING {
    BINDSTRING_HEADERS = 1,
    BINDSTRING_ACCEPT_MIMES = 2,
    BINDSTRING_EXTRA_URL = 3,
    BINDSTRING_LANGUAGE = 4,
    BINDSTRING_USERNAME = 5,
    BINDSTRING_PASSWORD = 6,
    BINDSTRING_UA_PIXELS = 7,
    BINDSTRING_UA_COLOR = 8,
    BINDSTRING_OS = 9,
    BINDSTRING_USER_AGENT = 10,
    BINDSTRING_ACCEPT_ENCODINGS = 11,
    BINDSTRING_POST_COOKIE = 12,
    BINDSTRING_POST_DATA_MIME = 13,
    BINDSTRING_URL = 14,
    BINDSTRING_IID = 15,
    BINDSTRING_FLAG_BIND_TO_OBJECT = 16,
    BINDSTRING_PTR_BIND_CONTEXT = 17,
    BINDSTRING_XDR_ORIGIN = 18,
    BINDSTRING_DOWNLOADPATH = 19,
    BINDSTRING_ROOTDOC_URL = 20,
    BINDSTRING_INITIAL_FILENAME = 21,
    BINDSTRING_PROXY_USERNAME = 22,
    BINDSTRING_PROXY_PASSWORD = 23,
    BINDSTRING_ENTERPRISE_ID = 24,
}}

ENUM!{ enum tagURLTEMPLATE {
    URLTEMPLATE_CUSTOM = 0,
    URLTEMPLATE_PREDEFINED_MIN = 65536,
    URLTEMPLATE_LOW = 65536,
    URLTEMPLATE_MEDLOW = 66816,
    URLTEMPLATE_MEDIUM = 69632,
    URLTEMPLATE_MEDHIGH = 70912,
    URLTEMPLATE_HIGH = 73728,
    URLTEMPLATE_PREDEFINED_MAX = 131072,
}}

ENUM!{ enum tagURLZONE {
    URLZONE_INVALID = 4294967295,
    URLZONE_PREDEFINED_MIN = 0,
    URLZONE_LOCAL_MACHINE = 0,
    URLZONE_INTRANET = 1,
    URLZONE_TRUSTED = 2,
    URLZONE_INTERNET = 3,
    URLZONE_UNTRUSTED = 4,
    URLZONE_PREDEFINED_MAX = 999,
    URLZONE_USER_MIN = 1000,
    URLZONE_USER_MAX = 10000,
}}

STRUCT!{struct AUTHENTICATEINFO {
    dwFlags: ::DWORD,
    dwReserved: ::DWORD,
}}

STRUCT!{struct BINDINFO {
    cbSize: ::ULONG,
    szExtraInfo: ::LPWSTR,
    stgmedData: ::STGMEDIUM,
    grfBindInfoF: ::DWORD,
    dwBindVerb: ::DWORD,
    szCustomVerb: ::LPWSTR,
    cbstgmedData: ::DWORD,
    dwOptions: ::DWORD,
    dwOptionsFlags: ::DWORD,
    dwCodePage: ::DWORD,
    securityAttributes: ::SECURITY_ATTRIBUTES,
    iid: ::IID,
    pUnk: *mut ::IUnknown,
    dwReserved: ::DWORD,
}}

STRUCT!{struct CODEBASEHOLD {
    cbSize: ::ULONG,
    szDistUnit: ::LPWSTR,
    szCodeBase: ::LPWSTR,
    dwVersionMS: ::DWORD,
    dwVersionLS: ::DWORD,
    dwStyle: ::DWORD,
}}

STRUCT!{struct CONFIRMSAFETY {
    clsid: ::CLSID,
    pUnk: *mut ::IUnknown,
    dwFlags: ::DWORD,
}}

STRUCT!{struct DATAINFO {
    ulTotalSize: ::ULONG,
    ulavrPacketSize: ::ULONG,
    ulConnectSpeed: ::ULONG,
    ulProcessorSpeed: ::ULONG,
}}

STRUCT!{struct HIT_LOGGING_INFO {
    dwStructSize: ::DWORD,
    lpszLoggedUrlName: ::LPSTR,
    StartTime: ::SYSTEMTIME,
    EndTime: ::SYSTEMTIME,
    lpszExtendedInfo: ::LPSTR,
}}

RIDL!(
interface IAuthenticateEx(IAuthenticateExVtbl): IAuthenticate(IAuthenticateVtbl) {
    fn AuthenticateEx(
        &mut self, phwnd: *mut ::HWND, pszUsername: *mut ::LPWSTR, pszPassword: *mut ::LPWSTR,
        pauthinfo: *mut ::AUTHENTICATEINFO
    ) -> ::HRESULT
});

RIDL!(
interface IAuthenticate(IAuthenticateVtbl): IUnknown(IUnknownVtbl) {
    fn Authenticate(
        &mut self, phwnd: *mut ::HWND, pszUsername: *mut ::LPWSTR, pszPassword: *mut ::LPWSTR
    ) -> ::HRESULT
});

RIDL!(
interface IBindCallbackRedirect(IBindCallbackRedirectVtbl): IUnknown(IUnknownVtbl) {
    fn Redirect(
        &mut self, lpcUrl: ::LPCWSTR, vbCancel: *mut ::VARIANT_BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IBindHost(IBindHostVtbl): IUnknown(IUnknownVtbl) {
    fn CreateMoniker(
        &mut self, szName: ::LPOLESTR, pBC: *mut ::IBindCtx, ppmk: *mut *mut ::IMoniker,
        dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn MonikerBindToStorage(
        &mut self, pMk: *mut ::IMoniker, pBC: *mut ::IBindCtx, pBSC: *mut ::IBindStatusCallback,
        riid: *const ::IID, ppvObj: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn MonikerBindToObject(
        &mut self, pMk: *mut ::IMoniker, pBC: *mut ::IBindCtx, pBSC: *mut ::IBindStatusCallback,
        riid: *const ::IID, ppvObj: *mut *mut ::c_void
    ) -> ::HRESULT
});

RIDL!(
interface IBindHttpSecurity(IBindHttpSecurityVtbl): IUnknown(IUnknownVtbl) {
    fn GetIgnoreCertMask(
        &mut self, pdwIgnoreCertMask: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IBindProtocol(IBindProtocolVtbl): IUnknown(IUnknownVtbl) {
    fn CreateBinding(
        &mut self, szUrl: ::LPCWSTR, pbc: *mut ::IBindCtx, ppb: *mut *mut ::IBinding
    ) -> ::HRESULT
});

RIDL!(
interface IBindStatusCallbackEx(IBindStatusCallbackExVtbl): IBindStatusCallback(IBindStatusCallbackVtbl) {
    fn GetBindInfoEx(
        &mut self, grfBINDF: *mut ::DWORD, pbindinfo: *mut ::BINDINFO, grfBINDF2: *mut ::DWORD,
        pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IBindStatusCallback(IBindStatusCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnStartBinding(
        &mut self, dwReserved: ::DWORD, pib: *mut ::IBinding
    ) -> ::HRESULT,
    fn GetPriority(&mut self, pnPriority: *mut ::LONG) -> ::HRESULT,
    fn OnLowResource(&mut self, reserved: ::DWORD) -> ::HRESULT,
    fn OnProgress(
        &mut self, ulProgress: ::ULONG, ulProgressMax: ::ULONG, ulStatusCode: ::ULONG,
        szStatusText: ::LPCWSTR
    ) -> ::HRESULT,
    fn OnStopBinding(
        &mut self, hresult: ::HRESULT, szError: ::LPCWSTR
    ) -> ::HRESULT,
    fn GetBindInfo(
        &mut self, grfBINDF: *mut ::DWORD, pbindinfo: *mut ::BINDINFO
    ) -> ::HRESULT,
    fn OnDataAvailable(
        &mut self, grfBSCF: ::DWORD, dwSize: ::DWORD, pformatetc: *mut ::FORMATETC,
        pstgmed: *mut ::STGMEDIUM
    ) -> ::HRESULT,
    fn OnObjectAvailable(
        &mut self, riid: *const ::IID, punk: *mut ::IUnknown
    ) -> ::HRESULT
});

RIDL!(
interface IBinding(IBindingVtbl): IUnknown(IUnknownVtbl) {
    fn Abort(&mut self) -> ::HRESULT,
    fn Suspend(&mut self) -> ::HRESULT,
    fn Resume(&mut self) -> ::HRESULT,
    fn SetPriority(&mut self, nPriority: ::LONG) -> ::HRESULT,
    fn GetPriority(&mut self, pnPriority: *mut ::LONG) -> ::HRESULT,
    fn GetBindResult(
        &mut self, pclsidProtocol: *mut ::CLSID, pdwResult: *mut ::DWORD,
        pszResult: *mut ::LPOLESTR, pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface ICatalogFileInfo(ICatalogFileInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetCatalogFile(&mut self, ppszCatalogFile: *mut ::LPSTR) -> ::HRESULT,
    fn GetJavaTrust(&mut self, ppJavaTrust: *mut *mut ::c_void) -> ::HRESULT
});

RIDL!(
interface ICodeInstall(ICodeInstallVtbl): IWindowForBindingUI(IWindowForBindingUIVtbl) {
    fn OnCodeInstallProblem(
        &mut self, ulStatusCode: ::ULONG, szDestination: ::LPCWSTR, szSource: ::LPCWSTR,
        dwReserved: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IDataFilter(IDataFilterVtbl): IUnknown(IUnknownVtbl) {
    fn DoEncode(
        &mut self, dwFlags: ::DWORD, lInBufferSize: ::LONG, pbInBuffer: *mut ::BYTE,
        lOutBufferSize: ::LONG, pbOutBuffer: *mut ::BYTE, lInBytesAvailable: ::LONG,
        plInBytesRead: *mut ::LONG, plOutBytesWritten: *mut ::LONG, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn DoDecode(
        &mut self, dwFlags: ::DWORD, lInBufferSize: ::LONG, pbInBuffer: *mut ::BYTE,
        lOutBufferSize: ::LONG, pbOutBuffer: *mut ::BYTE, lInBytesAvailable: ::LONG,
        plInBytesRead: *mut ::LONG, plOutBytesWritten: *mut ::LONG, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn SetEncodingLevel(&mut self, dwEncLevel: ::DWORD) -> ::HRESULT
});

RIDL!(
interface IEncodingFilterFactory(IEncodingFilterFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn FindBestFilter(
        &mut self, pwzCodeIn: ::LPCWSTR, pwzCodeOut: ::LPCWSTR, info: ::DATAINFO,
        ppDF: *mut *mut ::IDataFilter
    ) -> ::HRESULT,
    fn GetDefaultFilter(
        &mut self, pwzCodeIn: ::LPCWSTR, pwzCodeOut: ::LPCWSTR, ppDF: *mut *mut ::IDataFilter
    ) -> ::HRESULT
});

RIDL!(
interface IGetBindHandle(IGetBindHandleVtbl): IUnknown(IUnknownVtbl) {
    fn GetBindHandle(
        &mut self, enumRequestedHandle: ::BINDHANDLETYPES, pRetHandle: *mut ::HANDLE
    ) -> ::HRESULT
});

RIDL!(
interface IHttpNegotiate2(IHttpNegotiate2Vtbl): IHttpNegotiate(IHttpNegotiateVtbl) {
    fn GetRootSecurityId(
        &mut self, pbSecurityId: *mut ::BYTE, pcbSecurityId: *mut ::DWORD, dwReserved: ::DWORD_PTR
    ) -> ::HRESULT
});

RIDL!(
interface IHttpNegotiate3(IHttpNegotiate3Vtbl): IHttpNegotiate2(IHttpNegotiate2Vtbl) {
    fn GetSerializedClientCertContext(
        &mut self, ppbCert: *mut *mut ::BYTE, pcbCert: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IHttpNegotiate(IHttpNegotiateVtbl): IUnknown(IUnknownVtbl) {
    fn BeginningTransaction(
        &mut self, szURL: ::LPCWSTR, szHeaders: ::LPCWSTR, dwReserved: ::DWORD,
        pszAdditionalHeaders: *mut ::LPWSTR
    ) -> ::HRESULT,
    fn OnResponse(
        &mut self, dwResponseCode: ::DWORD, szResponseHeaders: ::LPCWSTR,
        szRequestHeaders: ::LPCWSTR, pszAdditionalRequestHeaders: *mut ::LPWSTR
    ) -> ::HRESULT
});

RIDL!(
interface IHttpSecurity(IHttpSecurityVtbl): IWindowForBindingUI(IWindowForBindingUIVtbl) {
    fn OnSecurityProblem(&mut self, dwProblem: ::DWORD) -> ::HRESULT
});

RIDL!(
interface IInternetBindInfoEx(IInternetBindInfoExVtbl): IInternetBindInfo(IInternetBindInfoVtbl) {
    fn GetBindInfoEx(
        &mut self, grfBINDF: *mut ::DWORD, pbindinfo: *mut ::BINDINFO, grfBINDF2: *mut ::DWORD,
        pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetBindInfo(IInternetBindInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetBindInfo(
        &mut self, grfBINDF: *mut ::DWORD, pbindinfo: *mut ::BINDINFO
    ) -> ::HRESULT,
    fn GetBindString(
        &mut self, ulStringType: ::ULONG, ppwzStr: *mut ::LPOLESTR, cEl: ::ULONG,
        pcElFetched: *mut ::ULONG
    ) -> ::HRESULT
});

RIDL!(
interface IInternetHostSecurityManager(IInternetHostSecurityManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetSecurityId(
        &mut self, pbSecurityId: *mut ::BYTE, pcbSecurityId: *mut ::DWORD, dwReserved: ::DWORD_PTR
    ) -> ::HRESULT,
    fn ProcessUrlAction(
        &mut self, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        pContext: *mut ::BYTE, cbContext: ::DWORD, dwFlags: ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn QueryCustomPolicy(
        &mut self, guidKey: *const ::GUID, ppPolicy: *mut *mut ::BYTE, pcbPolicy: *mut ::DWORD,
        pContext: *mut ::BYTE, cbContext: ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetPriority(IInternetPriorityVtbl): IUnknown(IUnknownVtbl) {
    fn SetPriority(&mut self, nPriority: ::LONG) -> ::HRESULT,
    fn GetPriority(&mut self, pnPriority: *mut ::LONG) -> ::HRESULT
});

RIDL!(
interface IInternetProtocolEx(IInternetProtocolExVtbl): IInternetProtocol(IInternetProtocolVtbl) {
    fn StartEx(
        &mut self, pUri: *mut ::IUri, pOIProtSink: *mut ::IInternetProtocolSink,
        pOIBindInfo: *mut ::IInternetBindInfo, grfPI: ::DWORD, dwReserved: ::HANDLE_PTR
    ) -> ::HRESULT
});

RIDL!(
interface IInternetProtocolInfo(IInternetProtocolInfoVtbl): IUnknown(IUnknownVtbl) {
    fn ParseUrl(
        &mut self, pwzUrl: ::LPCWSTR, ParseAction: ::PARSEACTION, dwParseFlags: ::DWORD,
        pwzResult: ::LPWSTR, cchResult: ::DWORD, pcchResult: *mut ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn CombineUrl(
        &mut self, pwzBaseUrl: ::LPCWSTR, pwzRelativeUrl: ::LPCWSTR, dwCombineFlags: ::DWORD,
        pwzResult: ::LPWSTR, cchResult: ::DWORD, pcchResult: *mut ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn CompareUrl(
        &mut self, pwzUrl1: ::LPCWSTR, pwzUrl2: ::LPCWSTR, dwCompareFlags: ::DWORD
    ) -> ::HRESULT,
    fn QueryInfo(
        &mut self, pwzUrl: ::LPCWSTR, OueryOption: ::QUERYOPTION, dwQueryFlags: ::DWORD,
        pBuffer: ::LPVOID, cbBuffer: ::DWORD, pcbBuf: *mut ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetProtocolRoot(IInternetProtocolRootVtbl): IUnknown(IUnknownVtbl) {
    fn Start(
        &mut self, szUrl: ::LPCWSTR, pOIProtSink: *mut ::IInternetProtocolSink,
        pOIBindInfo: *mut ::IInternetBindInfo, grfPI: ::DWORD, dwReserved: ::HANDLE_PTR
    ) -> ::HRESULT,
    fn Continue(&mut self, pProtocolData: *mut ::PROTOCOLDATA) -> ::HRESULT,
    fn Abort(&mut self, hrReason: ::HRESULT, dwOptions: ::DWORD) -> ::HRESULT,
    fn Terminate(&mut self, dwOptions: ::DWORD) -> ::HRESULT,
    fn Suspend(&mut self) -> ::HRESULT,
    fn Resume(&mut self) -> ::HRESULT
});

RIDL!(
interface IInternetProtocolSinkStackable(IInternetProtocolSinkStackableVtbl): IUnknown(IUnknownVtbl) {
    fn SwitchSink(
        &mut self, pOIProtSink: *mut ::IInternetProtocolSink
    ) -> ::HRESULT,
    fn CommitSwitch(&mut self) -> ::HRESULT,
    fn RollbackSwitch(&mut self) -> ::HRESULT
});

RIDL!(
interface IInternetProtocolSink(IInternetProtocolSinkVtbl): IUnknown(IUnknownVtbl) {
    fn Switch(&mut self, pProtocolData: *mut ::PROTOCOLDATA) -> ::HRESULT,
    fn ReportProgress(
        &mut self, ulStatusCode: ::ULONG, szStatusText: ::LPCWSTR
    ) -> ::HRESULT,
    fn ReportData(
        &mut self, grfBSCF: ::DWORD, ulProgress: ::ULONG, ulProgressMax: ::ULONG
    ) -> ::HRESULT,
    fn ReportResult(
        &mut self, hrResult: ::HRESULT, dwError: ::DWORD, szResult: ::LPCWSTR
    ) -> ::HRESULT
});

RIDL!(
interface IInternetProtocol(IInternetProtocolVtbl): IInternetProtocolRoot(IInternetProtocolRootVtbl) {
    fn Read(
        &mut self, pv: *mut ::c_void, cb: ::ULONG, pcbRead: *mut ::ULONG
    ) -> ::HRESULT,
    fn Seek(
        &mut self, dlibMove: ::LARGE_INTEGER, dwOrigin: ::DWORD,
        plibNewPosition: *mut ::ULARGE_INTEGER
    ) -> ::HRESULT,
    fn LockRequest(&mut self, dwOptions: ::DWORD) -> ::HRESULT,
    fn UnlockRequest(&mut self) -> ::HRESULT
});

RIDL!(
interface IInternetSecurityManagerEx2(IInternetSecurityManagerEx2Vtbl): IInternetSecurityManagerEx(IInternetSecurityManagerExVtbl) {
    fn MapUrlToZoneEx2(
        &mut self, pUri: *mut ::IUri, pdwZone: *mut ::DWORD, dwFlags: ::DWORD,
        ppwszMappedUrl: *mut ::LPWSTR, pdwOutFlags: *mut ::DWORD
    ) -> ::HRESULT,
    fn ProcessUrlActionEx2(
        &mut self, pUri: *mut ::IUri, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        pContext: *mut ::BYTE, cbContext: ::DWORD, dwFlags: ::DWORD, dwReserved: ::DWORD_PTR,
        pdwOutFlags: *mut ::DWORD
    ) -> ::HRESULT,
    fn GetSecurityIdEx2(
        &mut self, pUri: *mut ::IUri, pbSecurityId: *mut ::BYTE, pcbSecurityId: *mut ::DWORD,
        dwReserved: ::DWORD_PTR
    ) -> ::HRESULT,
    fn QueryCustomPolicyEx2(
        &mut self, pUri: *mut ::IUri, guidKey: *const ::GUID, ppPolicy: *mut *mut ::BYTE,
        pcbPolicy: *mut ::DWORD, pContext: *mut ::BYTE, cbContext: ::DWORD, dwReserved: ::DWORD_PTR
    ) -> ::HRESULT
});

RIDL!(
interface IInternetSecurityManagerEx(IInternetSecurityManagerExVtbl): IInternetSecurityManager(IInternetSecurityManagerVtbl) {
    fn ProcessUrlActionEx(
        &mut self, pwszUrl: ::LPCWSTR, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        pContext: *mut ::BYTE, cbContext: ::DWORD, dwFlags: ::DWORD, dwReserved: ::DWORD,
        pdwOutFlags: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetSecurityManager(IInternetSecurityManagerVtbl): IUnknown(IUnknownVtbl) {
    fn SetSecuritySite(
        &mut self, pSite: *mut ::IInternetSecurityMgrSite
    ) -> ::HRESULT,
    fn GetSecuritySite(
        &mut self, ppSite: *mut *mut ::IInternetSecurityMgrSite
    ) -> ::HRESULT,
    fn MapUrlToZone(
        &mut self, pwszUrl: ::LPCWSTR, pdwZone: *mut ::DWORD, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn GetSecurityId(
        &mut self, pwszUrl: ::LPCWSTR, pbSecurityId: *mut ::BYTE, pcbSecurityId: *mut ::DWORD,
        dwReserved: ::DWORD_PTR
    ) -> ::HRESULT,
    fn ProcessUrlAction(
        &mut self, pwszUrl: ::LPCWSTR, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        pContext: *mut ::BYTE, cbContext: ::DWORD, dwFlags: ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn QueryCustomPolicy(
        &mut self, pwszUrl: ::LPCWSTR, guidKey: *const ::GUID, ppPolicy: *mut *mut ::BYTE,
        pcbPolicy: *mut ::DWORD, pContext: *mut ::BYTE, cbContext: ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn SetZoneMapping(
        &mut self, dwZone: ::DWORD, lpszPattern: ::LPCWSTR, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn GetZoneMappings(
        &mut self, dwZone: ::DWORD, ppenumString: *mut *mut ::IEnumString, dwFlags: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetSecurityMgrSite(IInternetSecurityMgrSiteVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindow(&mut self, phwnd: *mut ::HWND) -> ::HRESULT,
    fn EnableModeless(&mut self, fEnable: ::BOOL) -> ::HRESULT
});

RIDL!(
interface IInternetSession(IInternetSessionVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterNameSpace(
        &mut self, pCF: *mut ::IClassFactory, rclsid: *const ::IID, pwzProtocol: ::LPCWSTR,
        cPatterns: ::ULONG, ppwzPatterns: *const ::LPCWSTR, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn UnregisterNameSpace(
        &mut self, pCF: *mut ::IClassFactory, pszProtocol: ::LPCWSTR
    ) -> ::HRESULT,
    fn RegisterMimeFilter(
        &mut self, pCF: *mut ::IClassFactory, rclsid: *const ::IID, pwzType: ::LPCWSTR
    ) -> ::HRESULT,
    fn UnregisterMimeFilter(
        &mut self, pCF: *mut ::IClassFactory, pwzType: ::LPCWSTR
    ) -> ::HRESULT,
    fn CreateBinding(
        &mut self, pBC: ::LPBC, szUrl: ::LPCWSTR, pUnkOuter: *mut ::IUnknown,
        ppUnk: *mut *mut ::IUnknown, ppOInetProt: *mut *mut ::IInternetProtocol, dwOption: ::DWORD
    ) -> ::HRESULT,
    fn SetSessionOption(
        &mut self, dwOption: ::DWORD, pBuffer: ::LPVOID, dwBufferLength: ::DWORD,
        dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn GetSessionOption(
        &mut self, dwOption: ::DWORD, pBuffer: ::LPVOID, pdwBufferLength: *mut ::DWORD,
        dwReserved: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetThreadSwitch(IInternetThreadSwitchVtbl): IUnknown(IUnknownVtbl) {
    fn Prepare(&mut self) -> ::HRESULT,
    fn Continue(&mut self) -> ::HRESULT
});

RIDL!(
interface IInternetZoneManagerEx2(IInternetZoneManagerEx2Vtbl): IInternetZoneManagerEx(IInternetZoneManagerExVtbl) {
    fn GetZoneAttributesEx(
        &mut self, dwZone: ::DWORD, pZoneAttributes: *mut ::ZONEATTRIBUTES, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn GetZoneSecurityState(
        &mut self, dwZoneIndex: ::DWORD, fRespectPolicy: ::BOOL, pdwState: ::LPDWORD,
        pfPolicyEncountered: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetIESecurityState(
        &mut self, fRespectPolicy: ::BOOL, pdwState: ::LPDWORD, pfPolicyEncountered: *mut ::BOOL,
        fNoCache: ::BOOL
    ) -> ::HRESULT,
    fn FixUnsecureSettings(&mut self) -> ::HRESULT
});

RIDL!(
interface IInternetZoneManagerEx(IInternetZoneManagerExVtbl): IInternetZoneManager(IInternetZoneManagerVtbl) {
    fn GetZoneActionPolicyEx(
        &mut self, dwZone: ::DWORD, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        urlZoneReg: ::URLZONEREG, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn SetZoneActionPolicyEx(
        &mut self, dwZone: ::DWORD, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        urlZoneReg: ::URLZONEREG, dwFlags: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IInternetZoneManager(IInternetZoneManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetZoneAttributes(
        &mut self, dwZone: ::DWORD, pZoneAttributes: *mut ::ZONEATTRIBUTES
    ) -> ::HRESULT,
    fn SetZoneAttributes(
        &mut self, dwZone: ::DWORD, pZoneAttributes: *mut ::ZONEATTRIBUTES
    ) -> ::HRESULT,
    fn GetZoneCustomPolicy(
        &mut self, dwZone: ::DWORD, guidKey: *const ::GUID, ppPolicy: *mut *mut ::BYTE,
        pcbPolicy: *mut ::DWORD, urlZoneReg: ::URLZONEREG
    ) -> ::HRESULT,
    fn SetZoneCustomPolicy(
        &mut self, dwZone: ::DWORD, guidKey: *const ::GUID, pPolicy: *mut ::BYTE,
        cbPolicy: ::DWORD, urlZoneReg: ::URLZONEREG
    ) -> ::HRESULT,
    fn GetZoneActionPolicy(
        &mut self, dwZone: ::DWORD, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        urlZoneReg: ::URLZONEREG
    ) -> ::HRESULT,
    fn SetZoneActionPolicy(
        &mut self, dwZone: ::DWORD, dwAction: ::DWORD, pPolicy: *mut ::BYTE, cbPolicy: ::DWORD,
        urlZoneReg: ::URLZONEREG
    ) -> ::HRESULT,
    fn PromptAction(
        &mut self, dwAction: ::DWORD, hwndParent: ::HWND, pwszUrl: ::LPCWSTR, pwszText: ::LPCWSTR,
        dwPromptFlags: ::DWORD
    ) -> ::HRESULT,
    fn LogAction(
        &mut self, dwAction: ::DWORD, pwszUrl: ::LPCWSTR, pwszText: ::LPCWSTR, dwLogFlags: ::DWORD
    ) -> ::HRESULT,
    fn CreateZoneEnumerator(
        &mut self, pdwEnum: *mut ::DWORD, pdwCount: *mut ::DWORD, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn GetZoneAt(
        &mut self, dwEnum: ::DWORD, dwIndex: ::DWORD, pdwZone: *mut ::DWORD
    ) -> ::HRESULT,
    fn DestroyZoneEnumerator(&mut self, dwEnum: ::DWORD) -> ::HRESULT,
    fn CopyTemplatePoliciesToZone(
        &mut self, dwTemplate: ::DWORD, dwZone: ::DWORD, dwReserved: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IMonikerProp(IMonikerPropVtbl): IUnknown(IUnknownVtbl) {
    fn PutProperty(
        &mut self, mkp: ::MONIKERPROPERTY, val: ::LPCWSTR
    ) -> ::HRESULT
});

RIDL!(
interface IPersistMoniker(IPersistMonikerVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassID(&mut self, pClassID: *mut ::CLSID) -> ::HRESULT,
    fn IsDirty(&mut self) -> ::HRESULT,
    fn Load(
        &mut self, fFullyAvailable: ::BOOL, pimkName: *mut ::IMoniker, pibc: ::LPBC,
        grfMode: ::DWORD
    ) -> ::HRESULT,
    fn Save(
        &mut self, pimkName: *mut ::IMoniker, pbc: ::LPBC, fRemember: ::BOOL
    ) -> ::HRESULT,
    fn SaveCompleted(
        &mut self, pimkName: *mut ::IMoniker, pibc: ::LPBC
    ) -> ::HRESULT,
    fn GetCurMoniker(&mut self, ppimkName: *mut *mut ::IMoniker) -> ::HRESULT
});

/*
RIDL!(
interface ISoftDistExt(ISoftDistExtVtbl): IUnknown(IUnknownVtbl) {
    fn ProcessSoftDist(
        &mut self, szCDFURL: ::LPCWSTR, pSoftDistElement: *mut ::IXMLElement,
        lpsdi: ::LPSOFTDISTINFO
    ) -> ::HRESULT,
    fn GetFirstCodeBase(
        &mut self, szCodeBase: *mut ::LPWSTR, dwMaxSize: ::LPDWORD
    ) -> ::HRESULT,
    fn GetNextCodeBase(
        &mut self, szCodeBase: *mut ::LPWSTR, dwMaxSize: ::LPDWORD
    ) -> ::HRESULT,
    fn AsyncInstallDistributionUnit(
        &mut self, pbc: *mut ::IBindCtx, pvReserved: ::LPVOID, flags: ::DWORD,
        lpcbh: ::LPCODEBASEHOLD
    ) -> ::HRESULT
});
*/

RIDL!(
interface IUriBuilderFactory(IUriBuilderFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateIUriBuilder(
        &mut self, dwFlags: ::DWORD, dwReserved: ::DWORD_PTR,
        ppIUriBuilder: *mut *mut ::IUriBuilder
    ) -> ::HRESULT,
    fn CreateInitializedIUriBuilder(
        &mut self, dwFlags: ::DWORD, dwReserved: ::DWORD_PTR,
        ppIUriBuilder: *mut *mut ::IUriBuilder
    ) -> ::HRESULT
});

RIDL!(
interface IUriBuilder(IUriBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn CreateUriSimple(
        &mut self, dwAllowEncodingPropertyMask: ::DWORD, dwReserved: ::DWORD_PTR,
        ppIUri: *mut *mut ::IUri
    ) -> ::HRESULT,
    fn CreateUri(
        &mut self, dwCreateFlags: ::DWORD, dwAllowEncodingPropertyMask: ::DWORD,
        dwReserved: ::DWORD_PTR, ppIUri: *mut *mut ::IUri
    ) -> ::HRESULT,
    fn CreateUriWithFlags(
        &mut self, dwCreateFlags: ::DWORD, dwUriBuilderFlags: ::DWORD,
        dwAllowEncodingPropertyMask: ::DWORD, dwReserved: ::DWORD_PTR, ppIUri: *mut *mut ::IUri
    ) -> ::HRESULT,
    fn GetIUri(&mut self, ppIUri: *mut *mut ::IUri) -> ::HRESULT,
    fn SetIUri(&mut self, pIUri: *mut ::IUri) -> ::HRESULT,
    fn GetFragment(
        &mut self, pcchFragment: *mut ::DWORD, ppwzFragment: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn GetHost(
        &mut self, pcchHost: *mut ::DWORD, ppwzHost: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn GetPassword(
        &mut self, pcchPassword: *mut ::DWORD, ppwzPassword: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn GetPath(
        &mut self, pcchPath: *mut ::DWORD, ppwzPath: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn GetPort(
        &mut self, pfHasPort: *mut ::BOOL, pdwPort: *mut ::DWORD
    ) -> ::HRESULT,
    fn GetQuery(
        &mut self, pcchQuery: *mut ::DWORD, ppwzQuery: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn GetSchemeName(
        &mut self, pcchSchemeName: *mut ::DWORD, ppwzSchemeName: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn GetUserNameA(
        &mut self, pcchUserName: *mut ::DWORD, ppwzUserName: *mut ::LPCWSTR
    ) -> ::HRESULT,
    fn SetFragment(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn SetHost(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn SetPassword(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn SetPath(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn SetPort(&mut self, fHasPort: ::BOOL, dwNewValue: ::DWORD) -> ::HRESULT,
    fn SetQuery(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn SetSchemeName(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn SetUserName(&mut self, pwzNewValue: ::LPCWSTR) -> ::HRESULT,
    fn RemoveProperties(&mut self, dwPropertyMask: ::DWORD) -> ::HRESULT,
    fn HasBeenModified(&mut self, pfModified: *mut ::BOOL) -> ::HRESULT
});

RIDL!(
interface IUriContainer(IUriContainerVtbl): IUnknown(IUnknownVtbl) {
    fn GetIUri(&mut self, ppIUri: *mut *mut ::IUri) -> ::HRESULT
});

RIDL!(
interface IUri(IUriVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyBSTR(
        &mut self, uriProp: ::Uri_PROPERTY, pbstrProperty: *mut ::BSTR, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn GetPropertyLength(
        &mut self, uriProp: ::Uri_PROPERTY, pcchProperty: *mut ::DWORD, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn GetPropertyDWORD(
        &mut self, uriProp: ::Uri_PROPERTY, pdwProperty: *mut ::DWORD, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn HasProperty(
        &mut self, uriProp: ::Uri_PROPERTY, pfHasProperty: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetAbsoluteUri(&mut self, pbstrAbsoluteUri: *mut ::BSTR) -> ::HRESULT,
    fn GetAuthority(&mut self, pbstrAuthority: *mut ::BSTR) -> ::HRESULT,
    fn GetDisplayUri(&mut self, pbstrDisplayString: *mut ::BSTR) -> ::HRESULT,
    fn GetDomain(&mut self, pbstrDomain: *mut ::BSTR) -> ::HRESULT,
    fn GetExtension(&mut self, pbstrExtension: *mut ::BSTR) -> ::HRESULT,
    fn GetFragment(&mut self, pbstrFragment: *mut ::BSTR) -> ::HRESULT,
    fn GetHost(&mut self, pbstrHost: *mut ::BSTR) -> ::HRESULT,
    fn GetPassword(&mut self, pbstrPassword: *mut ::BSTR) -> ::HRESULT,
    fn GetPath(&mut self, pbstrPath: *mut ::BSTR) -> ::HRESULT,
    fn GetPathAndQuery(&mut self, pbstrPathAndQuery: *mut ::BSTR) -> ::HRESULT,
    fn GetQuery(&mut self, pbstrQuery: *mut ::BSTR) -> ::HRESULT,
    fn GetRawUri(&mut self, pbstrRawUri: *mut ::BSTR) -> ::HRESULT,
    fn GetSchemeName(&mut self, pbstrSchemeName: *mut ::BSTR) -> ::HRESULT,
    fn GetUserInfo(&mut self, pbstrUserInfo: *mut ::BSTR) -> ::HRESULT,
    fn GetUserNameA(&mut self, pbstrUserName: *mut ::BSTR) -> ::HRESULT,
    fn GetHostType(&mut self, pdwHostType: *mut ::DWORD) -> ::HRESULT,
    fn GetPort(&mut self, pdwPort: *mut ::DWORD) -> ::HRESULT,
    fn GetScheme(&mut self, pdwScheme: *mut ::DWORD) -> ::HRESULT,
    fn GetZone(&mut self, pdwZone: *mut ::DWORD) -> ::HRESULT,
    fn GetProperties(&mut self, pdwFlags: ::LPDWORD) -> ::HRESULT,
    fn IsEqual(&mut self, pUri: *mut ::IUri, pfEqual: *mut ::BOOL) -> ::HRESULT
});

RIDL!(
interface IWinInetCacheHints2(IWinInetCacheHints2Vtbl): IWinInetCacheHints(IWinInetCacheHintsVtbl) {
    fn SetCacheExtension2(
        &mut self, pwzExt: ::LPCWSTR, pwzCacheFile: *mut ::WCHAR, pcchCacheFile: *mut ::DWORD,
        pdwWinInetError: *mut ::DWORD, pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IWinInetCacheHints(IWinInetCacheHintsVtbl): IUnknown(IUnknownVtbl) {
    fn SetCacheExtension(
        &mut self, pwzExt: ::LPCWSTR, pszCacheFile: ::LPVOID, pcbCacheFile: *mut ::DWORD,
        pdwWinInetError: *mut ::DWORD, pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IWinInetFileStream(IWinInetFileStreamVtbl): IUnknown(IUnknownVtbl) {
    fn SetHandleForUnlock(
        &mut self, hWinInetLockHandle: ::DWORD_PTR, dwReserved: ::DWORD_PTR
    ) -> ::HRESULT,
    fn SetDeleteFile(&mut self, dwReserved: ::DWORD_PTR) -> ::HRESULT
});

RIDL!(
interface IWinInetHttpInfo(IWinInetHttpInfoVtbl): IWinInetInfo(IWinInetInfoVtbl) {
    fn QueryInfo(
        &mut self, dwOption: ::DWORD, pBuffer: ::LPVOID, pcbBuf: *mut ::DWORD,
        pdwFlags: *mut ::DWORD, pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IWinInetHttpTimeouts(IWinInetHttpTimeoutsVtbl): IUnknown(IUnknownVtbl) {
    fn GetRequestTimeouts(
        &mut self, pdwConnectTimeout: *mut ::DWORD, pdwSendTimeout: *mut ::DWORD,
        pdwReceiveTimeout: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IWinInetInfo(IWinInetInfoVtbl): IUnknown(IUnknownVtbl) {
    fn QueryOption(
        &mut self, dwOption: ::DWORD, pBuffer: ::LPVOID, pcbBuf: *mut ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IWindowForBindingUI(IWindowForBindingUIVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindow(
        &mut self, rguidReason: *const ::GUID, phwnd: *mut ::HWND
    ) -> ::HRESULT
});

RIDL!(
interface IWrappedProtocol(IWrappedProtocolVtbl): IUnknown(IUnknownVtbl) {
    fn GetWrapperCode(
        &mut self, pnCode: *mut ::LONG, dwReserved: ::DWORD_PTR
    ) -> ::HRESULT
});

RIDL!(
interface IZoneIdentifier2(IZoneIdentifier2Vtbl): IZoneIdentifier(IZoneIdentifierVtbl) {
    fn GetLastWriterPackageFamilyName(
        &mut self, packageFamilyName: *mut ::LPWSTR
    ) -> ::HRESULT,
    fn SetLastWriterPackageFamilyName(
        &mut self, packageFamilyName: ::LPCWSTR
    ) -> ::HRESULT,
    fn RemoveLastWriterPackageFamilyName(&mut self) -> ::HRESULT,
    fn GetAppZoneId(&mut self, zone: *mut ::DWORD) -> ::HRESULT,
    fn SetAppZoneId(&mut self, zone: ::DWORD) -> ::HRESULT,
    fn RemoveAppZoneId(&mut self) -> ::HRESULT
});

RIDL!(
interface IZoneIdentifier(IZoneIdentifierVtbl): IUnknown(IUnknownVtbl) {
    fn GetId(&mut self, pdwZone: *mut ::DWORD) -> ::HRESULT,
    fn SetId(&mut self, dwZone: ::DWORD) -> ::HRESULT,
    fn Remove(&mut self) -> ::HRESULT
});

STRUCT!{struct PROTOCOLDATA {
    grfFlags: ::DWORD,
    dwState: ::DWORD,
    pData: ::LPVOID,
    cbData: ::ULONG,
}}

STRUCT!{struct PROTOCOLFILTERDATA {
    cbSize: ::DWORD,
    pProtocolSink: *mut ::IInternetProtocolSink,
    pProtocol: *mut ::IInternetProtocol,
    pUnk: *mut ::IUnknown,
    dwFilterFlags: ::DWORD,
}}

STRUCT!{struct PROTOCOL_ARGUMENT {
    szMethod: ::LPCWSTR,
    szTargetUrl: ::LPCWSTR,
}}

STRUCT!{struct REMSECURITY_ATTRIBUTES {
    nLength: ::DWORD,
    lpSecurityDescriptor: ::DWORD,
    bInheritHandle: ::BOOL,
}}

STRUCT!{struct RemBINDINFO {
    cbSize: ::ULONG,
    szExtraInfo: ::LPWSTR,
    grfBindInfoF: ::DWORD,
    dwBindVerb: ::DWORD,
    szCustomVerb: ::LPWSTR,
    cbstgmedData: ::DWORD,
    dwOptions: ::DWORD,
    dwOptionsFlags: ::DWORD,
    dwCodePage: ::DWORD,
    securityAttributes: ::REMSECURITY_ATTRIBUTES,
    iid: ::IID,
    pUnk: *mut ::IUnknown,
    dwReserved: ::DWORD,
}}

STRUCT!{struct RemFORMATETC {
    cfFormat: ::DWORD,
    ptd: ::DWORD,
    dwAspect: ::DWORD,
    lindex: ::LONG,
    tymed: ::DWORD,
}}

STRUCT!{struct SOFTDISTINFO {
    cbSize: ::ULONG,
    dwFlags: ::DWORD,
    dwAdState: ::DWORD,
    szTitle: ::LPWSTR,
    szAbstract: ::LPWSTR,
    szHREF: ::LPWSTR,
    dwInstalledVersionMS: ::DWORD,
    dwInstalledVersionLS: ::DWORD,
    dwUpdateVersionMS: ::DWORD,
    dwUpdateVersionLS: ::DWORD,
    dwAdvertisedVersionMS: ::DWORD,
    dwAdvertisedVersionLS: ::DWORD,
    dwReserved: ::DWORD,
}}

STRUCT!{struct StartParam {
    iid: ::IID,
    pIBindCtx: *mut ::IBindCtx,
    pItf: *mut ::IUnknown,
}}

STRUCT!{nodebug struct ZONEATTRIBUTES {
    cbSize: ::ULONG,
    szDisplayName: [::WCHAR; 260],
    szDescription: [::WCHAR; 200],
    szIconPath: [::WCHAR; 260],
    dwTemplateMinLevel: ::DWORD,
    dwTemplateRecommended: ::DWORD,
    dwTemplateCurrentLevel: ::DWORD,
    dwFlags: ::DWORD,
}}

pub type AUTHENTICATEF = ::__MIDL_IAuthenticateEx_0001;
pub type BINDF = ::__MIDL_IBindStatusCallback_0003;
pub type BINDF2 = ::__MIDL_IBindStatusCallbackEx_0001;
pub type BINDHANDLETYPES = ::__MIDL_IGetBindHandle_0001;
pub type BINDINFOF = ::__MIDL_IBindStatusCallback_0002;
pub type BINDINFO_OPTIONS = ::__MIDL_IBindStatusCallback_0005;
pub type BINDSTATUS = ::tagBINDSTATUS;
pub type BINDSTRING = ::tagBINDSTRING;
pub type BINDVERB = ::__MIDL_IBindStatusCallback_0001;
pub type BSCF = ::__MIDL_IBindStatusCallback_0006;
pub type CIP_STATUS = ::__MIDL_ICodeInstall_0001;
pub type INTERNETFEATURELIST = ::_tagINTERNETFEATURELIST;
pub type LPAUTHENTICATION = *mut ::IAuthenticate;
pub type LPAUTHENTICATIONEX = *mut ::IAuthenticateEx;
pub type LPBINDCALLBACKREDIRECT = *mut ::IBindCallbackRedirect;
pub type LPBINDHOST = *mut ::IBindHost;
pub type LPBINDING = *mut ::IBinding;
pub type LPBINDPROTOCOL = *mut ::IBindProtocol;
pub type LPBINDSTATUSCALLBACK = *mut ::IBindStatusCallback;
pub type LPBINDSTATUSCALLBACKEX = *mut ::IBindStatusCallbackEx;
pub type LPCATALOGFILEINFO = *mut ::ICatalogFileInfo;
pub type LPCODEBASEHOLD = *mut ::CODEBASEHOLD;
pub type LPCODEINSTALL = *mut ::ICodeInstall;
pub type LPDATAFILTER = *mut ::IDataFilter;
pub type LPENCODINGFILTERFACTORY = *mut ::IEncodingFilterFactory;
pub type LPGETBINDHANDLE = *mut ::IGetBindHandle;
pub type LPHIT_LOGGING_INFO = *mut ::HIT_LOGGING_INFO;
pub type LPHTTPNEGOTIATE = *mut ::IHttpNegotiate;
pub type LPHTTPNEGOTIATE2 = *mut ::IHttpNegotiate2;
pub type LPHTTPNEGOTIATE3 = *mut ::IHttpNegotiate3;
pub type LPHTTPSECURITY = *mut ::IHttpSecurity;
pub type LPIINTERNET = *mut ::IInternet;
pub type LPIINTERNETBINDINFO = *mut ::IInternetBindInfo;
pub type LPIINTERNETBINDINFOEX = *mut ::IInternetBindInfoEx;
pub type LPIINTERNETPRIORITY = *mut ::IInternetPriority;
pub type LPIINTERNETPROTOCOL = *mut ::IInternetProtocol;
pub type LPIINTERNETPROTOCOLINFO = *mut ::IInternetProtocolInfo;
pub type LPIINTERNETPROTOCOLROOT = *mut ::IInternetProtocolRoot;
pub type LPIINTERNETPROTOCOLSINK = *mut ::IInternetProtocolSink;
pub type LPIINTERNETPROTOCOLSINKStackable = *mut ::IInternetProtocolSinkStackable;
pub type LPIINTERNETSESSION = *mut ::IInternetSession;
pub type LPIINTERNETTHREADSWITCH = *mut ::IInternetThreadSwitch;
pub type LPIWRAPPEDPROTOCOL = *mut ::IWrappedProtocol;
pub type LPMONIKERPROP = *mut ::IMonikerProp;
pub type LPPERSISTMONIKER = *mut ::IPersistMoniker;
pub type LPPROTOCOL_ARGUMENT = *mut ::PROTOCOL_ARGUMENT;
pub type LPREMFORMATETC = *mut ::RemFORMATETC;
pub type LPREMSECURITY_ATTRIBUTES = *mut ::REMSECURITY_ATTRIBUTES;
pub type LPSOFTDISTINFO = *mut ::SOFTDISTINFO;
pub type LPURLZONEMANAGER = *mut ::IInternetZoneManager;
pub type LPWINDOWFORBINDINGUI = *mut ::IWindowForBindingUI;
pub type LPWININETCACHEHINTS = *mut ::IWinInetCacheHints;
pub type LPWININETCACHEHINTS2 = *mut ::IWinInetCacheHints2;
pub type LPWININETFILESTREAM = *mut ::IWinInetFileStream;
pub type LPWININETHTTPINFO = *mut ::IWinInetHttpInfo;
pub type LPWININETINFO = *mut ::IWinInetInfo;
pub type LPZONEATTRIBUTES = *mut ::ZONEATTRIBUTES;
pub type MONIKERPROPERTY = ::__MIDL_IMonikerProp_0001;
pub type OIBDG_FLAGS = ::_tagOIBDG_FLAGS;
pub type PARSEACTION = ::_tagPARSEACTION;
pub type PI_FLAGS = ::_tagPI_FLAGS;
pub type PREMSECURITY_ATTRIBUTES = *mut ::REMSECURITY_ATTRIBUTES;
pub type PSUACTION = ::_tagPSUACTION;
pub type PUAF = ::__MIDL_IInternetSecurityManager_0001;
pub type PUAFOUT = ::__MIDL_IInternetSecurityManager_0002;
pub type QUERYOPTION = ::_tagQUERYOPTION;
pub type SZM_FLAGS = ::__MIDL_IInternetSecurityManager_0003;
pub type URLTEMPLATE = ::tagURLTEMPLATE;
pub type URLZONE = ::tagURLZONE;
pub type URLZONEREG = ::_URLZONEREG;
pub type URL_ENCODING = ::__MIDL_IBindStatusCallback_0004;
pub type Uri_HOST_TYPE = ::__MIDL_IUri_0002;
pub type Uri_PROPERTY = ::__MIDL_IUri_0001;
pub type ZAFLAGS = ::__MIDL_IInternetZoneManager_0002;
pub type IInternet = ::IUnknown;
pub type IInternetVtbl = ::IUnknownVtbl;
pub const CF_NULL: ::UINT = 0;
pub const CONFIRMSAFETYACTION_LOADOBJECT: ::UINT = 0x00000001;
pub const FIEF_FLAG_FORCE_JITUI: ::UINT = 0x1;
pub const FIEF_FLAG_PEEK: ::UINT = 0x2;
pub const FIEF_FLAG_RESERVED_0: ::UINT = 0x8;
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: ::UINT = 0x4;
pub const FMFD_DEFAULT: ::UINT = 0x00000000;
pub const FMFD_ENABLEMIMESNIFFING: ::UINT = 0x00000002;
pub const FMFD_IGNOREMIMETEXTPLAIN: ::UINT = 0x00000004;
pub const FMFD_RESERVED_1: ::UINT = 0x00000040;
pub const FMFD_RESPECTTEXTPLAIN: ::UINT = 0x00000010;
pub const FMFD_RETURNUPDATEDIMGMIMES: ::UINT = 0x00000020;
pub const FMFD_SERVERMIME: ::UINT = 0x00000008;
pub const FMFD_URLASFILENAME: ::UINT = 0x00000001;
pub const GET_FEATURE_FROM_PROCESS: ::UINT = 0x00000002;
pub const GET_FEATURE_FROM_REGISTRY: ::UINT = 0x00000004;
pub const GET_FEATURE_FROM_THREAD: ::UINT = 0x00000001;
pub const GET_FEATURE_FROM_THREAD_INTERNET: ::UINT = 0x00000040;
pub const GET_FEATURE_FROM_THREAD_INTRANET: ::UINT = 0x00000010;
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: ::UINT = 0x00000008;
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: ::UINT = 0x00000080;
pub const GET_FEATURE_FROM_THREAD_TRUSTED: ::UINT = 0x00000020;
pub const MAX_SIZE_SECURITY_ID: ::UINT = 512;
pub const MKSYS_URLMONIKER: ::UINT = 6;
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: ::UINT = 0x00000080;
pub const MUTZ_DONT_UNESCAPE: ::UINT = 0x00000800;
pub const MUTZ_DONT_USE_CACHE: ::UINT = 0x00001000;
pub const MUTZ_ENFORCERESTRICTED: ::UINT = 0x00000100;
pub const MUTZ_FORCE_INTRANET_FLAGS: ::UINT = 0x00002000;
pub const MUTZ_IGNORE_ZONE_MAPPINGS: ::UINT = 0x00004000;
pub const MUTZ_ISFILE: ::UINT = 0x00000002;
pub const MUTZ_NOSAVEDFILECHECK: ::UINT = 0x00000001;
pub const MUTZ_REQUIRESAVEDFILECHECK: ::UINT = 0x00000400;
pub const MUTZ_RESERVED: ::UINT = 0x00000200;
pub const PROTOCOLFLAG_NO_PICS_CHECK: ::UINT = 0x00000001;
pub const SECURITY_IE_STATE_GREEN: ::UINT = 0x00000000;
pub const SECURITY_IE_STATE_RED: ::UINT = 0x00000001;
pub const SET_FEATURE_IN_REGISTRY: ::UINT = 0x00000004;
pub const SET_FEATURE_ON_PROCESS: ::UINT = 0x00000002;
pub const SET_FEATURE_ON_THREAD: ::UINT = 0x00000001;
pub const SET_FEATURE_ON_THREAD_INTERNET: ::UINT = 0x00000040;
pub const SET_FEATURE_ON_THREAD_INTRANET: ::UINT = 0x00000010;
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: ::UINT = 0x00000008;
pub const SET_FEATURE_ON_THREAD_RESTRICTED: ::UINT = 0x00000080;
pub const SET_FEATURE_ON_THREAD_TRUSTED: ::UINT = 0x00000020;
pub const SOFTDIST_ADSTATE_AVAILABLE: ::UINT = 0x00000001;
pub const SOFTDIST_ADSTATE_DOWNLOADED: ::UINT = 0x00000002;
pub const SOFTDIST_ADSTATE_INSTALLED: ::UINT = 0x00000003;
pub const SOFTDIST_ADSTATE_NONE: ::UINT = 0x00000000;
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: ::UINT = 0x00000008;
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: ::UINT = 0x00000004;
pub const SOFTDIST_FLAG_USAGE_EMAIL: ::UINT = 0x00000001;
pub const SOFTDIST_FLAG_USAGE_PRECACHE: ::UINT = 0x00000002;
pub const UAS_EXACTLEGACY: ::UINT = 0x00001000;
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: ::UINT = 0x00001204;
pub const URLACTION_ACTIVEX_CURR_MAX: ::UINT = 0x0000120B;
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: ::UINT = 0x0000120A;
pub const URLACTION_ACTIVEX_MAX: ::UINT = 0x000013ff;
pub const URLACTION_ACTIVEX_MIN: ::UINT = 0x00001200;
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: ::UINT = 0x00001206;
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: ::UINT = 0x00001202;
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: ::UINT = 0x0000120B;
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: ::UINT = 0x00001201;
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: ::UINT = 0x00001208;
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: ::UINT = 0x00001207;
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: ::UINT = 0x00001203;
pub const URLACTION_ACTIVEX_RUN: ::UINT = 0x00001200;
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: ::UINT = 0x00001209;
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: ::UINT = 0x00001205;
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: ::UINT = 0x00002702;
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: ::UINT = 0x0000270C;
pub const URLACTION_ALLOW_APEVALUATION: ::UINT = 0x00002301;
pub const URLACTION_ALLOW_AUDIO_VIDEO: ::UINT = 0x00002701;
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: ::UINT = 0x00002704;
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: ::UINT = 0x0000270A;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: ::UINT = 0x00002709;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: ::UINT = 0x00002708;
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: ::UINT = 0x0000270D;
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: ::UINT = 0x0000270B;
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: ::UINT = 0x00002300;
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: ::UINT = 0x00002703;
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: ::UINT = 0x00001408;
pub const URLACTION_ALLOW_XHR_EVALUATION: ::UINT = 0x00002302;
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: ::UINT = 0x00002706;
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: ::UINT = 0x00002705;
pub const URLACTION_AUTHENTICATE_CLIENT: ::UINT = 0x00001A01;
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: ::UINT = 0x00002201;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: ::UINT = 0x00002200;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: ::UINT = 0x00002200;
pub const URLACTION_BEHAVIOR_MIN: ::UINT = 0x00002000;
pub const URLACTION_BEHAVIOR_RUN: ::UINT = 0x00002000;
pub const URLACTION_CHANNEL_SOFTDIST_MAX: ::UINT = 0x00001Eff;
pub const URLACTION_CHANNEL_SOFTDIST_MIN: ::UINT = 0x00001E00;
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: ::UINT = 0x00001E05;
pub const URLACTION_CLIENT_CERT_PROMPT: ::UINT = 0x00001A04;
pub const URLACTION_COOKIES: ::UINT = 0x00001A02;
pub const URLACTION_COOKIES_ENABLED: ::UINT = 0x00001A10;
pub const URLACTION_COOKIES_SESSION: ::UINT = 0x00001A03;
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: ::UINT = 0x00001A06;
pub const URLACTION_COOKIES_THIRD_PARTY: ::UINT = 0x00001A05;
pub const URLACTION_CREDENTIALS_USE: ::UINT = 0x00001A00;
pub const URLACTION_CROSS_DOMAIN_DATA: ::UINT = 0x00001406;
pub const URLACTION_DOTNET_USERCONTROLS: ::UINT = 0x00002005;
pub const URLACTION_DOWNLOAD_CURR_MAX: ::UINT = 0x00001004;
pub const URLACTION_DOWNLOAD_MAX: ::UINT = 0x000011FF;
pub const URLACTION_DOWNLOAD_MIN: ::UINT = 0x00001000;
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: ::UINT = 0x00001001;
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: ::UINT = 0x00001004;
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: ::UINT = 0x00002105;
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: ::UINT = 0x00002107;
pub const URLACTION_FEATURE_DATA_BINDING: ::UINT = 0x00002106;
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: ::UINT = 0x00002104;
pub const URLACTION_FEATURE_MIME_SNIFFING: ::UINT = 0x00002100;
pub const URLACTION_FEATURE_MIN: ::UINT = 0x00002100;
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: ::UINT = 0x00002103;
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: ::UINT = 0x00002102;
pub const URLACTION_FEATURE_ZONE_ELEVATION: ::UINT = 0x00002101;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: ::UINT = 0x0000160D;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: ::UINT = 0x00001610;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: ::UINT = 0x0000160F;
pub const URLACTION_HTML_ALLOW_INDEXEDDB: ::UINT = 0x00001611;
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: ::UINT = 0x0000160B;
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: ::UINT = 0x0000160E;
pub const URLACTION_HTML_FONT_DOWNLOAD: ::UINT = 0x00001604;
pub const URLACTION_HTML_INCLUDE_FILE_PATH: ::UINT = 0x0000160A;
pub const URLACTION_HTML_JAVA_RUN: ::UINT = 0x00001605;
pub const URLACTION_HTML_MAX: ::UINT = 0x000017ff;
pub const URLACTION_HTML_META_REFRESH: ::UINT = 0x00001608;
pub const URLACTION_HTML_MIN: ::UINT = 0x00001600;
pub const URLACTION_HTML_MIXED_CONTENT: ::UINT = 0x00001609;
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: ::UINT = 0x0000160C;
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: ::UINT = 0x00001607;
pub const URLACTION_HTML_SUBMIT_FORMS: ::UINT = 0x00001601;
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: ::UINT = 0x00001602;
pub const URLACTION_HTML_SUBMIT_FORMS_TO: ::UINT = 0x00001603;
pub const URLACTION_HTML_USERDATA_SAVE: ::UINT = 0x00001606;
pub const URLACTION_INFODELIVERY_CURR_MAX: ::UINT = 0x00001D06;
pub const URLACTION_INFODELIVERY_MAX: ::UINT = 0x00001Dff;
pub const URLACTION_INFODELIVERY_MIN: ::UINT = 0x00001D00;
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: ::UINT = 0x00001D00;
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: ::UINT = 0x00001D03;
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: ::UINT = 0x00001D06;
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: ::UINT = 0x00001D01;
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: ::UINT = 0x00001D04;
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: ::UINT = 0x00001D02;
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: ::UINT = 0x00001D05;
pub const URLACTION_INPRIVATE_BLOCKING: ::UINT = 0x00002700;
pub const URLACTION_JAVA_CURR_MAX: ::UINT = 0x00001C00;
pub const URLACTION_JAVA_MAX: ::UINT = 0x00001Cff;
pub const URLACTION_JAVA_MIN: ::UINT = 0x00001C00;
pub const URLACTION_JAVA_PERMISSIONS: ::UINT = 0x00001C00;
pub const URLACTION_LOOSE_XAML: ::UINT = 0x00002402;
pub const URLACTION_LOWRIGHTS: ::UINT = 0x00002500;
pub const URLACTION_MIN: ::UINT = 0x00001000;
pub const URLACTION_NETWORK_CURR_MAX: ::UINT = 0x00001A10;
pub const URLACTION_NETWORK_MAX: ::UINT = 0x00001Bff;
pub const URLACTION_NETWORK_MIN: ::UINT = 0x00001A00;
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: ::UINT = 0x0000140B;
pub const URLACTION_SCRIPT_CURR_MAX: ::UINT = 0x0000140B;
pub const URLACTION_SCRIPT_JAVA_USE: ::UINT = 0x00001402;
pub const URLACTION_SCRIPT_MAX: ::UINT = 0x000015ff;
pub const URLACTION_SCRIPT_MIN: ::UINT = 0x00001400;
pub const URLACTION_SCRIPT_NAVIGATE: ::UINT = 0x0000140A;
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: ::UINT = 0x00001401;
pub const URLACTION_SCRIPT_PASTE: ::UINT = 0x00001407;
pub const URLACTION_SCRIPT_RUN: ::UINT = 0x00001400;
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: ::UINT = 0x00001405;
pub const URLACTION_SCRIPT_XSSFILTER: ::UINT = 0x00001409;
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: ::UINT = 0x00001811;
pub const URLACTION_SHELL_CURR_MAX: ::UINT = 0x00001812;
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: ::UINT = 0x0000180B;
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: ::UINT = 0x00001806;
pub const URLACTION_SHELL_EXECUTE_LOWRISK: ::UINT = 0x00001808;
pub const URLACTION_SHELL_EXECUTE_MODRISK: ::UINT = 0x00001807;
pub const URLACTION_SHELL_EXTENSIONSECURITY: ::UINT = 0x0000180C;
pub const URLACTION_SHELL_FILE_DOWNLOAD: ::UINT = 0x00001803;
pub const URLACTION_SHELL_INSTALL_DTITEMS: ::UINT = 0x00001800;
pub const URLACTION_SHELL_MAX: ::UINT = 0x000019ff;
pub const URLACTION_SHELL_MIN: ::UINT = 0x00001800;
pub const URLACTION_SHELL_MOVE_OR_COPY: ::UINT = 0x00001802;
pub const URLACTION_SHELL_POPUPMGR: ::UINT = 0x00001809;
pub const URLACTION_SHELL_PREVIEW: ::UINT = 0x0000180F;
pub const URLACTION_SHELL_REMOTEQUERY: ::UINT = 0x0000180E;
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: ::UINT = 0x0000180A;
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: ::UINT = 0x0000180D;
pub const URLACTION_SHELL_SHARE: ::UINT = 0x00001810;
pub const URLACTION_SHELL_SHELLEXECUTE: ::UINT = 0x00001806;
pub const URLACTION_SHELL_TOCTOU_RISK: ::UINT = 0x00001812;
pub const URLACTION_SHELL_VERB: ::UINT = 0x00001804;
pub const URLACTION_SHELL_WEBVIEW_VERB: ::UINT = 0x00001805;
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: ::UINT = 0x00002400;
pub const URLACTION_WINFX_SETUP: ::UINT = 0x00002600;
pub const URLACTION_XPS_DOCUMENTS: ::UINT = 0x00002401;
pub const URLMON_OPTION_URL_ENCODING: ::UINT = 0x10000004;
pub const URLMON_OPTION_USERAGENT: ::UINT = 0x10000001;
pub const URLMON_OPTION_USERAGENT_REFRESH: ::UINT = 0x10000002;
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: ::UINT = 0x10000008;
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: ::UINT = 0x10000010;
pub const URLOSTRM_GETNEWESTVERSION: ::UINT = 0x3;
pub const URLOSTRM_USECACHEDCOPY: ::UINT = 0x2;
pub const URLOSTRM_USECACHEDCOPY_ONLY: ::UINT = 0x1;
pub const URLPOLICY_ACTIVEX_CHECK_LIST: ::UINT = 0x00010000;
pub const URLPOLICY_ALLOW: ::UINT = 0x00;
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: ::UINT = 0x00010000;
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: ::UINT = 0x00000000;
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: ::UINT = 0x00030000;
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: ::UINT = 0x00010000;
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: ::UINT = 0x00030000;
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: ::UINT = 0x00020000;
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: ::UINT = 0x00010000;
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: ::UINT = 0x00030000;
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: ::UINT = 0x00020000;
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: ::UINT = 0x00010000;
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: ::UINT = 0x00000000;
pub const URLPOLICY_DISALLOW: ::UINT = 0x03;
pub const URLPOLICY_DONTCHECKDLGBOX: ::UINT = 0x100;
pub const URLPOLICY_JAVA_CUSTOM: ::UINT = 0x00800000;
pub const URLPOLICY_JAVA_HIGH: ::UINT = 0x00010000;
pub const URLPOLICY_JAVA_LOW: ::UINT = 0x00030000;
pub const URLPOLICY_JAVA_MEDIUM: ::UINT = 0x00020000;
pub const URLPOLICY_JAVA_PROHIBIT: ::UINT = 0x00000000;
pub const URLPOLICY_LOG_ON_ALLOW: ::UINT = 0x40;
pub const URLPOLICY_LOG_ON_DISALLOW: ::UINT = 0x80;
pub const URLPOLICY_MASK_PERMISSIONS: ::UINT = 0x0f;
pub const URLPOLICY_NOTIFY_ON_ALLOW: ::UINT = 0x10;
pub const URLPOLICY_NOTIFY_ON_DISALLOW: ::UINT = 0x20;
pub const URLPOLICY_QUERY: ::UINT = 0x01;
pub const URLZONE_ESC_FLAG: ::UINT = 0x100;
pub const URL_MK_LEGACY: ::UINT = 0;
pub const URL_MK_NO_CANONICALIZE: ::UINT = 2;
pub const URL_MK_UNIFORM: ::UINT = 1;
pub const UriBuilder_USE_ORIGINAL_FLAGS: ::UINT = 0x00000001;
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: ::UINT = 0x00000004;
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: ::UINT = 0x00000002;
pub const Uri_CREATE_ALLOW_RELATIVE: ::UINT = 0x00000001;
pub const Uri_CREATE_CANONICALIZE: ::UINT = 0x00000100;
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: ::UINT = 0x00020000;
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: ::UINT = 0x00000200;
pub const Uri_CREATE_DECODE_EXTRA_INFO: ::UINT = 0x00000040;
pub const Uri_CREATE_FILE_USE_DOS_PATH: ::UINT = 0x00000020;
pub const Uri_CREATE_IE_SETTINGS: ::UINT = 0x00002000;
pub const Uri_CREATE_NOFRAG: ::UINT = 0x00000008;
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: ::UINT = 0x00010000;
pub const Uri_CREATE_NO_CANONICALIZE: ::UINT = 0x00000010;
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: ::UINT = 0x00000400;
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: ::UINT = 0x00000080;
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: ::UINT = 0x00008000;
pub const Uri_CREATE_NO_IE_SETTINGS: ::UINT = 0x00004000;
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: ::UINT = 0x00001000;
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: ::UINT = 0x00000800;
pub const Uri_DISPLAY_IDN_HOST: ::UINT = 0x00000004;
pub const Uri_DISPLAY_NO_FRAGMENT: ::UINT = 0x00000001;
pub const Uri_DISPLAY_NO_PUNYCODE: ::UINT = 0x00000008;
pub const Uri_ENCODING_HOST_IS_IDN: ::UINT = 0x00000004;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: ::UINT = 0x00000010;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: ::UINT = 0x00000008;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: ::UINT = 0x00000040;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: ::UINT = 0x00000020;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: ::UINT = 0x00000002;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: ::UINT = 0x00000001;
pub const Uri_PUNYCODE_IDN_HOST: ::UINT = 0x00000002;
pub const WININETINFO_OPTION_LOCK_HANDLE: ::UINT = 65534;
