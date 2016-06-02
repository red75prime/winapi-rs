// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! this ALWAYS GENERATED file contains the definitions for the interfaces
//8402
STRUCT!{struct BIND_OPTS {
    cbStruct: ::DWORD,
    grfFlags: ::DWORD,
    grfMode: ::DWORD,
    dwTickCountDeadline: ::DWORD,
}}
pub type LPBIND_OPTS = *mut BIND_OPTS;
//8479
RIDL!(
interface IBindCtx(IBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterObjectBound(&mut self, punk: *mut ::IUnknown) -> ::HRESULT,
    fn RevokeObjectBound(&mut self, punk: *mut ::IUnknown) -> ::HRESULT,
    fn ReleaseBoundObjects(&mut self) -> ::HRESULT,
    fn SetBindOptions(&mut self, pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetBindOptions(&mut self, pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetRunningObjectTable(&mut self, pprot: *mut *mut IRunningObjectTable) -> ::HRESULT,
    fn RegisterObjectParam(&mut self, pszKey: ::LPOLESTR, punk: *mut ::IUnknown) -> ::HRESULT,
    fn GetObjectParam(&mut self, pszKey: ::LPOLESTR, ppunk: *mut *mut ::IUnknown) -> ::HRESULT,
    fn EnumObjectParam(&mut self, ppenum: *mut *mut ::IEnumString) -> ::HRESULT,
    fn RevokeObjectParam(&mut self, pszKey: ::LPOLESTR) -> ::HRESULT
}
);
//8681
pub type IEnumMoniker = ::IUnknown; // TODO
//8958
RIDL!(
interface IRunningObjectTable(IRunningObjectTableVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        &mut self, grfFlags: ::DWORD, punkObject: *mut ::IUnknown, pmkObjectName: *mut IMoniker,
        pdwRegister: *mut ::DWORD
    ) -> ::HRESULT,
    fn Revoke(&mut self, dwRegister: ::DWORD) -> ::HRESULT,
    fn IsRunning(&mut self, pmkObjectName: *mut IMoniker) -> ::HRESULT,
    fn GetObject(
        &mut self, pmkObjectName: *mut IMoniker, ppunkObject: *mut *mut ::IUnknown
    ) -> ::HRESULT,
    fn NoteChangeTime(&mut self, dwRegister: ::DWORD, pfiletime: *mut ::FILETIME) -> ::HRESULT,
    fn GetTimeOfLastChange(
        &mut self, pmkObjectName: *mut IMoniker, pfiletime: *mut ::FILETIME
    ) -> ::HRESULT,
    fn EnumRunning(&mut self, ppenumMoniker: *mut *mut IEnumMoniker) -> ::HRESULT
}
);
//9350
pub type IMoniker = ::IUnknown; // TODO
pub type EOLE_AUTHENTICATION_CAPABILITIES = ::DWORD;
pub const EOAC_NONE: ::DWORD = 0;
pub const EOAC_MUTUAL_AUTH: ::DWORD = 0x1;
pub const EOAC_STATIC_CLOAKING: ::DWORD = 0x20;
pub const EOAC_DYNAMIC_CLOAKING: ::DWORD = 0x40;
pub const EOAC_ANY_AUTHORITY: ::DWORD = 0x80;
pub const EOAC_MAKE_FULLSIC: ::DWORD = 0x100;
pub const EOAC_DEFAULT: ::DWORD = 0x800;
pub const EOAC_SECURE_REFS: ::DWORD = 0x2;
pub const EOAC_ACCESS_CONTROL: ::DWORD = 0x4;
pub const EOAC_APPID: ::DWORD = 0x8;
pub const EOAC_DYNAMIC: ::DWORD = 0x10;
pub const EOAC_REQUIRE_FULLSIC: ::DWORD = 0x200;
pub const EOAC_AUTO_IMPERSONATE: ::DWORD = 0x400;
pub const EOAC_NO_CUSTOM_MARSHAL: ::DWORD = 0x2000;
pub const EOAC_DISABLE_AAA: ::DWORD = 0x1000;
STRUCT!{struct SOLE_AUTHENTICATION_SERVICE {
    dwAuthnSvc: ::DWORD,
    dwAuthzSvc: ::DWORD,
    pPrincipalName: *mut ::OLECHAR,
    hr: ::HRESULT,
}}
RIDL!(
interface IPersist(IPersistVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassID(&mut self, pClassID: *mut ::CLSID) -> ::HRESULT
});
RIDL!(
interface IAdviseSink2(IAdviseSink2Vtbl): IAdviseSink(IAdviseSinkVtbl) {
    fn OnLinkSrcChange(&mut self, pmk: *mut ::IMoniker) -> ()
});
RIDL!(
interface IAdviseSink(IAdviseSinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnDataChange(
        &mut self, pFormatetc: *mut ::FORMATETC, pStgmed: *mut ::STGMEDIUM
    ) -> (),
    fn OnViewChange(&mut self, dwAspect: ::DWORD, lindex: ::LONG) -> (),
    fn OnRename(&mut self, pmk: *mut ::IMoniker) -> (),
    fn OnSave(&mut self) -> (),
    fn OnClose(&mut self) -> ()
});
STRUCT!{struct FORMATETC {
    cfFormat: ::CLIPFORMAT,
    ptd: *mut ::DVTARGETDEVICE,
    dwAspect: ::DWORD,
    lindex: ::LONG,
    tymed: ::DWORD,
}}
STRUCT!{struct uSTGMEDIUM {
    tymed: ::DWORD,
    u: ::HBITMAP,
    pUnkForRelease: *mut ::IUnknown,
}}
UNION!(uSTGMEDIUM, u, pstg, pstg_mut, *mut ::IStorage);
UNION!(uSTGMEDIUM, u, pstm, pstm_mut, *mut ::IStream);
UNION!(uSTGMEDIUM, u, lpszFileName, lpszFileName_mut, ::LPOLESTR);
UNION!(uSTGMEDIUM, u, hGlobal, hGlobal_mut, ::HGLOBAL);
UNION!(uSTGMEDIUM, u, hEnhMetaFile, hEnhMetaFile_mut, ::HENHMETAFILE);
UNION!(uSTGMEDIUM, u, hMetaFilePict, hMetaFilePict_mut, ::HMETAFILEPICT);
UNION!(uSTGMEDIUM, u, hBitmap, hBitmap_mut, ::HBITMAP);
STRUCT!{struct userFLAG_STGMEDIUM {
    ContextFlags: ::LONG,
    fPassOwnership: ::LONG,
    Stgmed: ::userSTGMEDIUM,
}}
STRUCT!{struct userSTGMEDIUM {
    pUnkForRelease: *mut ::IUnknown,
}}
pub type STGMEDIUM = ::uSTGMEDIUM;
RIDL!(
interface IStorage(IStorageVtbl): IUnknown(IUnknownVtbl) {
    fn CreateStream(
        &mut self, pwcsName: *const ::OLECHAR, grfMode: ::DWORD, reserved1: ::DWORD,
        reserved2: ::DWORD, ppstm: *mut *mut ::IStream
    ) -> ::HRESULT,
    fn OpenStream(
        &mut self, pwcsName: *const ::OLECHAR, reserved1: *mut ::c_void, grfMode: ::DWORD,
        reserved2: ::DWORD, ppstm: *mut *mut ::IStream
    ) -> ::HRESULT,
    fn CreateStorage(
        &mut self, pwcsName: *const ::OLECHAR, grfMode: ::DWORD, reserved1: ::DWORD,
        reserved2: ::DWORD, ppstg: *mut *mut ::IStorage
    ) -> ::HRESULT,
    fn OpenStorage(
        &mut self, pwcsName: *const ::OLECHAR, pstgPriority: *mut ::IStorage, grfMode: ::DWORD,
        snbExclude: ::SNB, reserved: ::DWORD, ppstg: *mut *mut ::IStorage
    ) -> ::HRESULT,
    fn CopyTo(
        &mut self, ciidExclude: ::DWORD, rgiidExclude: *const ::IID, snbExclude: ::SNB,
        pstgDest: *mut ::IStorage
    ) -> ::HRESULT,
    fn MoveElementTo(
        &mut self, pwcsName: *const ::OLECHAR, pstgDest: *mut ::IStorage,
        pwcsNewName: *const ::OLECHAR, grfFlags: ::DWORD
    ) -> ::HRESULT,
    fn Commit(&mut self, grfCommitFlags: ::DWORD) -> ::HRESULT,
    fn Revert(&mut self) -> ::HRESULT,
    fn EnumElements(
        &mut self, reserved1: ::DWORD, reserved2: *mut ::c_void, reserved3: ::DWORD,
        ppenum: *mut *mut ::IEnumSTATSTG
    ) -> ::HRESULT,
    fn DestroyElement(&mut self, pwcsName: *const ::OLECHAR) -> ::HRESULT,
    fn RenameElement(
        &mut self, pwcsOldName: *const ::OLECHAR, pwcsNewName: *const ::OLECHAR
    ) -> ::HRESULT,
    fn SetElementTimes(
        &mut self, pwcsName: *const ::OLECHAR, pctime: *const ::FILETIME,
        patime: *const ::FILETIME, pmtime: *const ::FILETIME
    ) -> ::HRESULT,
    fn SetClass(&mut self, clsid: *const ::IID) -> ::HRESULT,
    fn SetStateBits(
        &mut self, grfStateBits: ::DWORD, grfMask: ::DWORD
    ) -> ::HRESULT,
    fn Stat(
        &mut self, pstatstg: *mut ::STATSTG, grfStatFlag: ::DWORD
    ) -> ::HRESULT
});
pub type LPBC = *mut ::IBindCtx;
STRUCT!{struct DVTARGETDEVICE {
    tdSize: ::DWORD,
    tdDriverNameOffset: ::WORD,
    tdDeviceNameOffset: ::WORD,
    tdPortNameOffset: ::WORD,
    tdExtDevmodeOffset: ::WORD,
    tdData: [::BYTE; 1],
}}
pub type SNB = *mut ::LPOLESTR;
RIDL!(
interface IEnumSTATSTG(IEnumSTATSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        &mut self, celt: ::ULONG, rgelt: *mut ::STATSTG, pceltFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(&mut self, celt: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(&mut self, ppenum: *mut *mut ::IEnumSTATSTG) -> ::HRESULT
});
