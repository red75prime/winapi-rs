// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of ocidl.h

ENUM!{ enum tagACTIVATEFLAGS {
    ACTIVATE_WINDOWLESS = 1,
}}

ENUM!{ enum tagAspectInfoFlag {
    DVASPECTINFOFLAG_CANOPTIMIZE = 1,
}}

ENUM!{ enum tagCTRLINFO {
    CTRLINFO_EATS_RETURN = 1,
    CTRLINFO_EATS_ESCAPE = 2,
}}

ENUM!{ enum tagDVASPECT2 {
    DVASPECT_OPAQUE = 16,
    DVASPECT_TRANSPARENT = 32,
}}

ENUM!{ enum tagExtentMode {
    DVEXTENT_CONTENT = 0,
    DVEXTENT_INTEGRAL = 1,
}}

ENUM!{ enum tagGUIDKIND {
    GUIDKIND_DEFAULT_SOURCE_DISP_IID = 1,
}}

ENUM!{ enum tagHITRESULT {
    HITRESULT_OUTSIDE = 0,
    HITRESULT_TRANSPARENT = 1,
    HITRESULT_CLOSE = 2,
    HITRESULT_HIT = 3,
}}

ENUM!{ enum tagOLEDCFLAGS {
    OLEDC_NODRAW = 1,
    OLEDC_PAINTBKGND = 2,
    OLEDC_OFFSCREEN = 4,
}}

ENUM!{ enum tagPOINTERINACTIVE {
    POINTERINACTIVE_ACTIVATEONENTRY = 1,
    POINTERINACTIVE_DEACTIVATEONLEAVE = 2,
    POINTERINACTIVE_ACTIVATEONDRAG = 4,
}}

ENUM!{ enum tagPROPBAG2_TYPE {
    PROPBAG2_TYPE_UNDEFINED = 0,
    PROPBAG2_TYPE_DATA = 1,
    PROPBAG2_TYPE_URL = 2,
    PROPBAG2_TYPE_OBJECT = 3,
    PROPBAG2_TYPE_STREAM = 4,
    PROPBAG2_TYPE_STORAGE = 5,
    PROPBAG2_TYPE_MONIKER = 6,
}}

ENUM!{ enum tagPROPPAGESTATUS {
    PROPPAGESTATUS_DIRTY = 1,
    PROPPAGESTATUS_VALIDATE = 2,
    PROPPAGESTATUS_CLEAN = 4,
}}

ENUM!{ enum tagPictureAttributes {
    PICTURE_SCALABLE = 1,
    PICTURE_TRANSPARENT = 2,
}}

ENUM!{ enum tagQACONTAINERFLAGS {
    QACONTAINER_SHOWHATCHING = 1,
    QACONTAINER_SHOWGRABHANDLES = 2,
    QACONTAINER_USERMODE = 4,
    QACONTAINER_DISPLAYASDEFAULT = 8,
    QACONTAINER_UIDEAD = 16,
    QACONTAINER_AUTOCLIP = 32,
    QACONTAINER_MESSAGEREFLECT = 64,
    QACONTAINER_SUPPORTSMNEMONICS = 128,
}}

ENUM!{ enum tagREADYSTATE {
    READYSTATE_UNINITIALIZED = 0,
    READYSTATE_LOADING = 1,
    READYSTATE_LOADED = 2,
    READYSTATE_INTERACTIVE = 3,
    READYSTATE_COMPLETE = 4,
}}

ENUM!{ enum tagUASFLAGS {
    UAS_NORMAL = 0,
    UAS_BLOCKED = 1,
    UAS_NOPARENTENABLE = 2,
    UAS_MASK = 3,
}}

ENUM!{ enum tagVIEWSTATUS {
    VIEWSTATUS_OPAQUE = 1,
    VIEWSTATUS_SOLIDBKGND = 2,
    VIEWSTATUS_DVASPECTOPAQUE = 4,
    VIEWSTATUS_DVASPECTTRANSPARENT = 8,
    VIEWSTATUS_SURFACE = 16,
    VIEWSTATUS_3DSURFACE = 32,
}}

ENUM!{ enum tagXFORMCOORDS {
    XFORMCOORDS_POSITION = 1,
    XFORMCOORDS_SIZE = 2,
    XFORMCOORDS_HIMETRICTOCONTAINER = 4,
    XFORMCOORDS_CONTAINERTOHIMETRIC = 8,
    XFORMCOORDS_EVENTCOMPAT = 16,
}}

STRUCT!{struct CADWORD {
    cElems: ::ULONG,
    pElems: *mut ::DWORD,
}}

STRUCT!{struct CALPOLESTR {
    cElems: ::ULONG,
    pElems: *mut ::LPOLESTR,
}}

STRUCT!{struct CAUUID {
    cElems: ::ULONG,
    pElems: *mut ::GUID,
}}

STRUCT!{struct CONNECTDATA {
    pUnk: *mut ::IUnknown,
    dwCookie: ::DWORD,
}}

STRUCT!{struct CONTROLINFO {
    cb: ::ULONG,
    hAccel: ::HACCEL,
    cAccel: ::USHORT,
    dwFlags: ::DWORD,
}}

STRUCT!{struct DVASPECTINFO {
    cb: ::ULONG,
    dwFlags: ::DWORD,
}}

STRUCT!{struct DVEXTENTINFO {
    cb: ::ULONG,
    dwExtentMode: ::DWORD,
    sizelProposed: ::SIZEL,
}}

//TODO: Implement
/*
RIDL!(
interface IAdviseSinkEx(IAdviseSinkExVtbl): IAdviseSink(IAdviseSinkVtbl) {
    fn OnViewStatusChange(&mut self, dwViewStatus: ::DWORD) -> ()
});

RIDL!(
interface IClassFactory2(IClassFactory2Vtbl): IClassFactory(IClassFactoryVtbl) {
    fn GetLicInfo(&mut self, pLicInfo: *mut ::LICINFO) -> ::HRESULT,
    fn RequestLicKey(
        &mut self, dwReserved: ::DWORD, pBstrKey: *mut ::BSTR
    ) -> ::HRESULT,
    fn CreateInstanceLic(
        &mut self, pUnkOuter: *mut ::IUnknown, pUnkReserved: *mut ::IUnknown, riid: *const ::IID,
        bstrKey: ::BSTR, ppvObj: *mut ::PVOID
    ) -> ::HRESULT
});

RIDL!(
interface IConnectionPointContainer(IConnectionPointContainerVtbl): IUnknown(IUnknownVtbl) {
    fn EnumConnectionPoints(
        &mut self, ppEnum: *mut *mut ::IEnumConnectionPoints
    ) -> ::HRESULT,
    fn FindConnectionPoint(
        &mut self, riid: *const ::IID, ppCP: *mut *mut ::IConnectionPoint
    ) -> ::HRESULT
});

RIDL!(
interface IConnectionPoint(IConnectionPointVtbl): IUnknown(IUnknownVtbl) {
    fn GetConnectionInterface(&mut self, pIID: *mut ::IID) -> ::HRESULT,
    fn GetConnectionPointContainer(
        &mut self, ppCPC: *mut *mut ::IConnectionPointContainer
    ) -> ::HRESULT,
    fn Advise(
        &mut self, pUnkSink: *mut ::IUnknown, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn Unadvise(&mut self, dwCookie: ::DWORD) -> ::HRESULT,
    fn EnumConnections(
        &mut self, ppEnum: *mut *mut ::IEnumConnections
    ) -> ::HRESULT
});

RIDL!(
interface IEnumConnectionPoints(IEnumConnectionPointsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        &mut self, cConnections: ::ULONG, ppCP: *mut ::LPCONNECTIONPOINT, pcFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(&mut self, cConnections: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(&mut self, ppEnum: *mut *mut ::IEnumConnectionPoints) -> ::HRESULT
});

RIDL!(
interface IEnumConnections(IEnumConnectionsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        &mut self, cConnections: ::ULONG, rgcd: ::LPCONNECTDATA, pcFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(&mut self, cConnections: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(&mut self, ppEnum: *mut *mut ::IEnumConnections) -> ::HRESULT
});

RIDL!(
interface IEnumOleUndoUnits(IEnumOleUndoUnitsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        &mut self, cElt: ::ULONG, rgElt: *mut *mut ::IOleUndoUnit, pcEltFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(&mut self, cElt: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(&mut self, ppEnum: *mut *mut ::IEnumOleUndoUnits) -> ::HRESULT
});
*/

RIDL!(
interface IFont(IFontVtbl): IUnknown(IUnknownVtbl) {
    fn get_Name(&mut self, pName: *mut ::BSTR) -> ::HRESULT,
    fn put_Name(&mut self, name: ::BSTR) -> ::HRESULT,
    fn get_Size(&mut self, pSize: *mut ::CY) -> ::HRESULT,
    fn put_Size(&mut self, size: ::CY) -> ::HRESULT,
    fn get_Bold(&mut self, pBold: *mut ::BOOL) -> ::HRESULT,
    fn put_Bold(&mut self, bold: ::BOOL) -> ::HRESULT,
    fn get_Italic(&mut self, pItalic: *mut ::BOOL) -> ::HRESULT,
    fn put_Italic(&mut self, italic: ::BOOL) -> ::HRESULT,
    fn get_Underline(&mut self, pUnderline: *mut ::BOOL) -> ::HRESULT,
    fn put_Underline(&mut self, underline: ::BOOL) -> ::HRESULT,
    fn get_Strikethrough(&mut self, pStrikethrough: *mut ::BOOL) -> ::HRESULT,
    fn put_Strikethrough(&mut self, strikethrough: ::BOOL) -> ::HRESULT,
    fn get_Weight(&mut self, pWeight: *mut ::SHORT) -> ::HRESULT,
    fn put_Weight(&mut self, weight: ::SHORT) -> ::HRESULT,
    fn get_Charset(&mut self, pCharset: *mut ::SHORT) -> ::HRESULT,
    fn put_Charset(&mut self, charset: ::SHORT) -> ::HRESULT,
    fn get_hFont(&mut self, phFont: *mut ::HFONT) -> ::HRESULT,
    fn Clone(&mut self, ppFont: *mut *mut ::IFont) -> ::HRESULT,
    fn IsEqual(&mut self, pFontOther: *mut ::IFont) -> ::HRESULT,
    fn SetRatio(&mut self, cyLogical: ::LONG, cyHimetric: ::LONG) -> ::HRESULT,
    fn QueryTextMetrics(&mut self, pTM: *mut ::TEXTMETRICOLE) -> ::HRESULT,
    fn AddRefHfont(&mut self, hFont: ::HFONT) -> ::HRESULT,
    fn ReleaseHfont(&mut self, hFont: ::HFONT) -> ::HRESULT,
    fn SetHdc(&mut self, hDC: ::HDC) -> ::HRESULT
});

//TODO: Implement
/*
RIDL!(
interface IObjectWithSite(IObjectWithSiteVtbl): IUnknown(IUnknownVtbl) {
    fn SetSite(&mut self, pUnkSite: *mut ::IUnknown) -> ::HRESULT,
    fn GetSite(
        &mut self, riid: *const ::IID, ppvSite: *mut *mut ::c_void
    ) -> ::HRESULT
});

RIDL!(
interface IOleControlSite(IOleControlSiteVtbl): IUnknown(IUnknownVtbl) {
    fn OnControlInfoChanged(&mut self) -> ::HRESULT,
    fn LockInPlaceActive(&mut self, fLock: ::BOOL) -> ::HRESULT,
    fn GetExtendedControl(
        &mut self, ppDisp: *mut *mut ::IDispatch
    ) -> ::HRESULT,
    fn TransformCoords(
        &mut self, pPtlHimetric: *mut ::POINTL, pPtfContainer: *mut ::POINTF, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn TranslateAcceleratorA(
        &mut self, pMsg: *mut ::MSG, grfModifiers: ::DWORD
    ) -> ::HRESULT,
    fn OnFocus(&mut self, fGotFocus: ::BOOL) -> ::HRESULT,
    fn ShowPropertyFrame(&mut self) -> ::HRESULT
});

RIDL!(
interface IOleControl(IOleControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetControlInfo(&mut self, pCI: *mut ::CONTROLINFO) -> ::HRESULT,
    fn OnMnemonic(&mut self, pMsg: *mut ::MSG) -> ::HRESULT,
    fn OnAmbientPropertyChange(&mut self, dispID: ::DISPID) -> ::HRESULT,
    fn FreezeEvents(&mut self, bFreeze: ::BOOL) -> ::HRESULT
});

RIDL!(
interface IOleInPlaceObjectWindowless(IOleInPlaceObjectWindowlessVtbl): IOleInPlaceObject(IOleInPlaceObjectVtbl) {
    fn OnWindowMessage(
        &mut self, msg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM, plResult: *mut ::LRESULT
    ) -> ::HRESULT,
    fn GetDropTarget(
        &mut self, ppDropTarget: *mut *mut ::IDropTarget
    ) -> ::HRESULT
});

RIDL!(
interface IOleInPlaceSiteEx(IOleInPlaceSiteExVtbl): IOleInPlaceSite(IOleInPlaceSiteVtbl) {
    fn OnInPlaceActivateEx(
        &mut self, pfNoRedraw: *mut ::BOOL, dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn OnInPlaceDeactivateEx(&mut self, fNoRedraw: ::BOOL) -> ::HRESULT,
    fn RequestUIActivate(&mut self) -> ::HRESULT
});

RIDL!(
interface IOleInPlaceSiteWindowless(IOleInPlaceSiteWindowlessVtbl): IOleInPlaceSiteEx(IOleInPlaceSiteExVtbl) {
    fn CanWindowlessActivate(&mut self) -> ::HRESULT,
    fn GetCapture(&mut self) -> ::HRESULT,
    fn SetCapture(&mut self, fCapture: ::BOOL) -> ::HRESULT,
    fn GetFocus(&mut self) -> ::HRESULT,
    fn SetFocus(&mut self, fFocus: ::BOOL) -> ::HRESULT,
    fn GetDC(
        &mut self, pRect: ::LPCRECT, grfFlags: ::DWORD, phDC: *mut ::HDC
    ) -> ::HRESULT,
    fn ReleaseDC(&mut self, hDC: ::HDC) -> ::HRESULT,
    fn InvalidateRect(&mut self, pRect: ::LPCRECT, fErase: ::BOOL) -> ::HRESULT,
    fn InvalidateRgn(&mut self, hRGN: ::HRGN, fErase: ::BOOL) -> ::HRESULT,
    fn ScrollRect(
        &mut self, dx: ::INT, dy: ::INT, pRectScroll: ::LPCRECT, pRectClip: ::LPCRECT
    ) -> ::HRESULT,
    fn AdjustRect(&mut self, prc: ::LPRECT) -> ::HRESULT,
    fn OnDefWindowMessage(
        &mut self, msg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM, plResult: *mut ::LRESULT
    ) -> ::HRESULT
});

RIDL!(
interface IOleParentUndoUnit(IOleParentUndoUnitVtbl): IOleUndoUnit(IOleUndoUnitVtbl) {
    fn Open(&mut self, pPUU: *mut ::IOleParentUndoUnit) -> ::HRESULT,
    fn Close(
        &mut self, pPUU: *mut ::IOleParentUndoUnit, fCommit: ::BOOL
    ) -> ::HRESULT,
    fn Add(&mut self, pUU: *mut ::IOleUndoUnit) -> ::HRESULT,
    fn FindUnit(&mut self, pUU: *mut ::IOleUndoUnit) -> ::HRESULT,
    fn GetParentState(&mut self, pdwState: *mut ::DWORD) -> ::HRESULT
});

RIDL!(
interface IOleUndoManager(IOleUndoManagerVtbl): IUnknown(IUnknownVtbl) {
    fn Open(&mut self, pPUU: *mut ::IOleParentUndoUnit) -> ::HRESULT,
    fn Close(
        &mut self, pPUU: *mut ::IOleParentUndoUnit, fCommit: ::BOOL
    ) -> ::HRESULT,
    fn Add(&mut self, pUU: *mut ::IOleUndoUnit) -> ::HRESULT,
    fn GetOpenParentState(&mut self, pdwState: *mut ::DWORD) -> ::HRESULT,
    fn DiscardFrom(&mut self, pUU: *mut ::IOleUndoUnit) -> ::HRESULT,
    fn UndoTo(&mut self, pUU: *mut ::IOleUndoUnit) -> ::HRESULT,
    fn RedoTo(&mut self, pUU: *mut ::IOleUndoUnit) -> ::HRESULT,
    fn EnumUndoable(
        &mut self, ppEnum: *mut *mut ::IEnumOleUndoUnits
    ) -> ::HRESULT,
    fn EnumRedoable(
        &mut self, ppEnum: *mut *mut ::IEnumOleUndoUnits
    ) -> ::HRESULT,
    fn GetLastUndoDescription(&mut self, pBstr: *mut ::BSTR) -> ::HRESULT,
    fn GetLastRedoDescription(&mut self, pBstr: *mut ::BSTR) -> ::HRESULT,
    fn Enable(&mut self, fEnable: ::BOOL) -> ::HRESULT
});

RIDL!(
interface IOleUndoUnit(IOleUndoUnitVtbl): IUnknown(IUnknownVtbl) {
    fn Do(&mut self, pUndoManager: *mut ::IOleUndoManager) -> ::HRESULT,
    fn GetDescription(&mut self, pBstr: *mut ::BSTR) -> ::HRESULT,
    fn GetUnitType(
        &mut self, pClsid: *mut ::CLSID, plID: *mut ::LONG
    ) -> ::HRESULT,
    fn OnNextAdd(&mut self) -> ::HRESULT
});


RIDL!(
interface IPerPropertyBrowsing(IPerPropertyBrowsingVtbl): IUnknown(IUnknownVtbl) {
    fn GetDisplayString(
        &mut self, dispID: ::DISPID, pBstr: *mut ::BSTR
    ) -> ::HRESULT,
    fn MapPropertyToPage(
        &mut self, dispID: ::DISPID, pClsid: *mut ::CLSID
    ) -> ::HRESULT,
    fn GetPredefinedStrings(
        &mut self, dispID: ::DISPID, pCaStringsOut: *mut ::CALPOLESTR,
        pCaCookiesOut: *mut ::CADWORD
    ) -> ::HRESULT,
    fn GetPredefinedValue(
        &mut self, dispID: ::DISPID, dwCookie: ::DWORD, pVarOut: *mut ::VARIANT
    ) -> ::HRESULT
});

RIDL!(
interface IPersistMemory(IPersistMemoryVtbl): IPersist(IPersistVtbl) {
    fn IsDirty(&mut self) -> ::HRESULT,
    fn Load(&mut self, pMem: ::LPVOID, cbSize: ::ULONG) -> ::HRESULT,
    fn Save(
        &mut self, pMem: ::LPVOID, fClearDirty: ::BOOL, cbSize: ::ULONG
    ) -> ::HRESULT,
    fn GetSizeMax(&mut self, pCbSize: *mut ::ULONG) -> ::HRESULT,
    fn InitNew(&mut self) -> ::HRESULT
});

RIDL!(
interface IPersistPropertyBag2(IPersistPropertyBag2Vtbl): IPersist(IPersistVtbl) {
    fn InitNew(&mut self) -> ::HRESULT,
    fn Load(
        &mut self, pPropBag: *mut ::IPropertyBag2, pErrLog: *mut ::IErrorLog
    ) -> ::HRESULT,
    fn Save(
        &mut self, pPropBag: *mut ::IPropertyBag2, fClearDirty: ::BOOL, fSaveAllProperties: ::BOOL
    ) -> ::HRESULT,
    fn IsDirty(&mut self) -> ::HRESULT
});

RIDL!(
interface IPersistPropertyBag(IPersistPropertyBagVtbl): IPersist(IPersistVtbl) {
    fn InitNew(&mut self) -> ::HRESULT,
    fn Load(
        &mut self, pPropBag: *mut ::IPropertyBag, pErrorLog: *mut ::IErrorLog
    ) -> ::HRESULT,
    fn Save(
        &mut self, pPropBag: *mut ::IPropertyBag, fClearDirty: ::BOOL, fSaveAllProperties: ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IPersistStreamInit(IPersistStreamInitVtbl): IPersist(IPersistVtbl) {
    fn IsDirty(&mut self) -> ::HRESULT,
    fn Load(&mut self, pStm: ::LPSTREAM) -> ::HRESULT,
    fn Save(&mut self, pStm: ::LPSTREAM, fClearDirty: ::BOOL) -> ::HRESULT,
    fn GetSizeMax(&mut self, pCbSize: *mut ::ULARGE_INTEGER) -> ::HRESULT,
    fn InitNew(&mut self) -> ::HRESULT
});

RIDL!(
interface IPicture2(IPicture2Vtbl): IUnknown(IUnknownVtbl) {
    fn get_Handle(&mut self, pHandle: *mut ::HHANDLE) -> ::HRESULT,
    fn get_hPal(&mut self, phPal: *mut ::HHANDLE) -> ::HRESULT,
    fn get_Type(&mut self, pType: *mut ::SHORT) -> ::HRESULT,
    fn get_Width(&mut self, pWidth: *mut ::OLE_XSIZE_HIMETRIC) -> ::HRESULT,
    fn get_Height(&mut self, pHeight: *mut ::OLE_YSIZE_HIMETRIC) -> ::HRESULT,
    fn Render(
        &mut self, hDC: ::HDC, x: ::LONG, y: ::LONG, cx: ::LONG, cy: ::LONG,
        xSrc: ::OLE_XPOS_HIMETRIC, ySrc: ::OLE_YPOS_HIMETRIC, cxSrc: ::OLE_XSIZE_HIMETRIC,
        cySrc: ::OLE_YSIZE_HIMETRIC, pRcWBounds: ::LPCRECT
    ) -> ::HRESULT,
    fn set_hPal(&mut self, hPal: ::HHANDLE) -> ::HRESULT,
    fn get_CurDC(&mut self, phDC: *mut ::HDC) -> ::HRESULT,
    fn SelectPicture(
        &mut self, hDCIn: ::HDC, phDCOut: *mut ::HDC, phBmpOut: *mut ::HHANDLE
    ) -> ::HRESULT,
    fn get_KeepOriginalFormat(&mut self, pKeep: *mut ::BOOL) -> ::HRESULT,
    fn put_KeepOriginalFormat(&mut self, keep: ::BOOL) -> ::HRESULT,
    fn PictureChanged(&mut self) -> ::HRESULT,
    fn SaveAsFile(
        &mut self, pStream: ::LPSTREAM, fSaveMemCopy: ::BOOL, pCbSize: *mut ::LONG
    ) -> ::HRESULT,
    fn get_Attributes(&mut self, pDwAttr: *mut ::DWORD) -> ::HRESULT
});

RIDL!(
interface IPicture(IPictureVtbl): IUnknown(IUnknownVtbl) {
    fn get_Handle(&mut self, pHandle: *mut ::OLE_HANDLE) -> ::HRESULT,
    fn get_hPal(&mut self, phPal: *mut ::OLE_HANDLE) -> ::HRESULT,
    fn get_Type(&mut self, pType: *mut ::SHORT) -> ::HRESULT,
    fn get_Width(&mut self, pWidth: *mut ::OLE_XSIZE_HIMETRIC) -> ::HRESULT,
    fn get_Height(&mut self, pHeight: *mut ::OLE_YSIZE_HIMETRIC) -> ::HRESULT,
    fn Render(
        &mut self, hDC: ::HDC, x: ::LONG, y: ::LONG, cx: ::LONG, cy: ::LONG,
        xSrc: ::OLE_XPOS_HIMETRIC, ySrc: ::OLE_YPOS_HIMETRIC, cxSrc: ::OLE_XSIZE_HIMETRIC,
        cySrc: ::OLE_YSIZE_HIMETRIC, pRcWBounds: ::LPCRECT
    ) -> ::HRESULT,
    fn set_hPal(&mut self, hPal: ::OLE_HANDLE) -> ::HRESULT,
    fn get_CurDC(&mut self, phDC: *mut ::HDC) -> ::HRESULT,
    fn SelectPicture(
        &mut self, hDCIn: ::HDC, phDCOut: *mut ::HDC, phBmpOut: *mut ::OLE_HANDLE
    ) -> ::HRESULT,
    fn get_KeepOriginalFormat(&mut self, pKeep: *mut ::BOOL) -> ::HRESULT,
    fn put_KeepOriginalFormat(&mut self, keep: ::BOOL) -> ::HRESULT,
    fn PictureChanged(&mut self) -> ::HRESULT,
    fn SaveAsFile(
        &mut self, pStream: ::LPSTREAM, fSaveMemCopy: ::BOOL, pCbSize: *mut ::LONG
    ) -> ::HRESULT,
    fn get_Attributes(&mut self, pDwAttr: *mut ::DWORD) -> ::HRESULT
});

RIDL!(
interface IPointerInactive(IPointerInactiveVtbl): IUnknown(IUnknownVtbl) {
    fn GetActivationPolicy(&mut self, pdwPolicy: *mut ::DWORD) -> ::HRESULT,
    fn OnInactiveMouseMove(
        &mut self, pRectBounds: ::LPCRECT, x: ::LONG, y: ::LONG, grfKeyState: ::DWORD
    ) -> ::HRESULT,
    fn OnInactiveSetCursor(
        &mut self, pRectBounds: ::LPCRECT, x: ::LONG, y: ::LONG, dwMouseMsg: ::DWORD,
        fSetAlways: ::BOOL
    ) -> ::HRESULT
});
*/

RIDL!(
interface IPropertyBag2(IPropertyBag2Vtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        &mut self, cProperties: ::ULONG, pPropBag: *mut ::PROPBAG2, pErrLog: *mut ::IErrorLog,
        pvarValue: *mut ::VARIANT, phrError: *mut ::HRESULT
    ) -> ::HRESULT,
    fn Write(
        &mut self, cProperties: ::ULONG, pPropBag: *mut ::PROPBAG2, pvarValue: *mut ::VARIANT
    ) -> ::HRESULT,
    fn CountProperties(&mut self, pcProperties: *mut ::ULONG) -> ::HRESULT,
    fn GetPropertyInfo(
        &mut self, iProperty: ::ULONG, cProperties: ::ULONG, pPropBag: *mut ::PROPBAG2,
        pcProperties: *mut ::ULONG
    ) -> ::HRESULT,
    fn LoadObject(
        &mut self, pstrName: ::LPCOLESTR, dwHint: ::DWORD, pUnkObject: *mut ::IUnknown,
        pErrLog: *mut ::IErrorLog
    ) -> ::HRESULT
});

//TODO: Implement
/*
RIDL!(
interface IPropertyNotifySink(IPropertyNotifySinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnChanged(&mut self, dispID: ::DISPID) -> ::HRESULT,
    fn OnRequestEdit(&mut self, dispID: ::DISPID) -> ::HRESULT
});

RIDL!(
interface IPropertyPage2(IPropertyPage2Vtbl): IPropertyPage(IPropertyPageVtbl) {
    fn EditProperty(&mut self, dispID: ::DISPID) -> ::HRESULT
});

RIDL!(
interface IPropertyPageSite(IPropertyPageSiteVtbl): IUnknown(IUnknownVtbl) {
    fn OnStatusChange(&mut self, dwFlags: ::DWORD) -> ::HRESULT,
    fn GetLocaleID(&mut self, pLocaleID: *mut ::LCID) -> ::HRESULT,
    fn GetPageContainer(&mut self, ppUnk: *mut *mut ::IUnknown) -> ::HRESULT,
    fn TranslateAcceleratorA(&mut self, pMsg: *mut ::MSG) -> ::HRESULT
});

RIDL!(
interface IPropertyPage(IPropertyPageVtbl): IUnknown(IUnknownVtbl) {
    fn SetPageSite(&mut self, pPageSite: *mut ::IPropertyPageSite) -> ::HRESULT,
    fn Activate(
        &mut self, hWndParent: ::HWND, pRect: ::LPCRECT, bModal: ::BOOL
    ) -> ::HRESULT,
    fn Deactivate(&mut self) -> ::HRESULT,
    fn GetPageInfo(&mut self, pPageInfo: *mut ::PROPPAGEINFO) -> ::HRESULT,
    fn SetObjects(
        &mut self, cObjects: ::ULONG, ppUnk: *mut *mut ::IUnknown
    ) -> ::HRESULT,
    fn Show(&mut self, nCmdShow: ::UINT) -> ::HRESULT,
    fn Move(&mut self, pRect: ::LPCRECT) -> ::HRESULT,
    fn IsPageDirty(&mut self) -> ::HRESULT,
    fn Apply(&mut self) -> ::HRESULT,
    fn Help(&mut self, pszHelpDir: ::LPCOLESTR) -> ::HRESULT,
    fn TranslateAcceleratorA(&mut self, pMsg: *mut ::MSG) -> ::HRESULT
});

RIDL!(
interface IProvideClassInfo2(IProvideClassInfo2Vtbl): IProvideClassInfo(IProvideClassInfoVtbl) {
    fn GetGUID(&mut self, dwGuidKind: ::DWORD, pGUID: *mut ::GUID) -> ::HRESULT
});

RIDL!(
interface IProvideClassInfo(IProvideClassInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassInfoA(&mut self, ppTI: *mut *mut ::ITypeInfo) -> ::HRESULT
});

RIDL!(
interface IProvideMultipleClassInfo(IProvideMultipleClassInfoVtbl): IProvideClassInfo2(IProvideClassInfo2Vtbl) {
    fn GetMultiTypeInfoCount(&mut self, pcti: *mut ::ULONG) -> ::HRESULT,
    fn GetInfoOfIndex(
        &mut self, iti: ::ULONG, dwFlags: ::DWORD, pptiCoClass: *mut *mut ::ITypeInfo,
        pdwTIFlags: *mut ::DWORD, pcdispidReserved: *mut ::ULONG, piidPrimary: *mut ::IID,
        piidSource: *mut ::IID
    ) -> ::HRESULT
});

RIDL!(
interface IQuickActivate(IQuickActivateVtbl): IUnknown(IUnknownVtbl) {
    fn QuickActivate(
        &mut self, pQaContainer: *mut ::QACONTAINER, pQaControl: *mut ::QACONTROL
    ) -> ::HRESULT,
    fn SetContentExtent(&mut self, pSizel: ::LPSIZEL) -> ::HRESULT,
    fn GetContentExtent(&mut self, pSizel: ::LPSIZEL) -> ::HRESULT
});

RIDL!(
interface ISimpleFrameSite(ISimpleFrameSiteVtbl): IUnknown(IUnknownVtbl) {
    fn PreMessageFilter(
        &mut self, hWnd: ::HWND, msg: ::UINT, wp: ::WPARAM, lp: ::LPARAM, plResult: *mut ::LRESULT,
        pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn PostMessageFilter(
        &mut self, hWnd: ::HWND, msg: ::UINT, wp: ::WPARAM, lp: ::LPARAM, plResult: *mut ::LRESULT,
        dwCookie: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface ISpecifyPropertyPages(ISpecifyPropertyPagesVtbl): IUnknown(IUnknownVtbl) {
    fn GetPages(&mut self, pPages: *mut ::CAUUID) -> ::HRESULT
});

RIDL!(
interface IViewObjectEx(IViewObjectExVtbl): IViewObject2(IViewObject2Vtbl) {
    fn GetRect(&mut self, dwAspect: ::DWORD, pRect: ::LPRECTL) -> ::HRESULT,
    fn GetViewStatus(&mut self, pdwStatus: *mut ::DWORD) -> ::HRESULT,
    fn QueryHitPoint(
        &mut self, dwAspect: ::DWORD, pRectBounds: ::LPCRECT, ptlLoc: ::POINT, lCloseHint: ::LONG,
        pHitResult: *mut ::DWORD
    ) -> ::HRESULT,
    fn QueryHitRect(
        &mut self, dwAspect: ::DWORD, pRectBounds: ::LPCRECT, pRectLoc: ::LPCRECT,
        lCloseHint: ::LONG, pHitResult: *mut ::DWORD
    ) -> ::HRESULT,
    fn GetNaturalExtent(
        &mut self, dwAspect: ::DWORD, lindex: ::LONG, ptd: *mut ::DVTARGETDEVICE,
        hicTargetDev: ::HDC, pExtentInfo: *mut ::DVEXTENTINFO, pSizel: ::LPSIZEL
    ) -> ::HRESULT
});
*/

STRUCT!{struct LICINFO {
    cbLicInfo: ::LONG,
    fRuntimeKeyAvail: ::BOOL,
    fLicVerified: ::BOOL,
}}

STRUCT!{struct POINTF {
    x: ::FLOAT,
    y: ::FLOAT,
}}

STRUCT!{struct PROPBAG2 {
    dwType: ::DWORD,
    vt: ::VARTYPE,
    cfType: ::CLIPFORMAT,
    dwHint: ::DWORD,
    pstrName: ::LPOLESTR,
    clsid: ::CLSID,
}}

//TODO: Implement
/*
STRUCT!{struct PROPPAGEINFO {
    cb: ::ULONG,
    pszTitle: ::LPOLESTR,
    size: ::SIZE,
    pszDocString: ::LPOLESTR,
    pszHelpFile: ::LPOLESTR,
    dwHelpContext: ::DWORD,
}}

STRUCT!{struct QACONTAINER {
    cbSize: ::ULONG,
    pClientSite: *mut ::IOleClientSite,
    pAdviseSink: *mut ::IAdviseSinkEx,
    pPropertyNotifySink: *mut ::IPropertyNotifySink,
    pUnkEventSink: *mut ::IUnknown,
    dwAmbientFlags: ::DWORD,
    colorFore: ::OLE_COLOR,
    colorBack: ::OLE_COLOR,
    pFont: *mut ::IFont,
    pUndoMgr: *mut ::IOleUndoManager,
    dwAppearance: ::DWORD,
    lcid: ::LONG,
    hpal: ::HPALETTE,
    pBindHost: *mut ::IBindHost,
    pOleControlSite: *mut ::IOleControlSite,
    pServiceProvider: *mut ::IServiceProvider,
}}

STRUCT!{struct QACONTROL {
    cbSize: ::ULONG,
    dwMiscStatus: ::DWORD,
    dwViewStatus: ::DWORD,
    dwEventCookie: ::DWORD,
    dwPropNotifyCookie: ::DWORD,
    dwPointerActivationPolicy: ::DWORD,
}}
*/
pub type ACTIVATEFLAGS = ::tagACTIVATEFLAGS;
pub type CTRLINFO = ::tagCTRLINFO;
pub type DVASPECT2 = ::tagDVASPECT2;
pub type DVASPECTINFOFLAG = ::tagAspectInfoFlag;
pub type DVEXTENTMODE = ::tagExtentMode;
pub type GUIDKIND = ::tagGUIDKIND;
pub type HHANDLE = ::UINT_PTR;
pub type HITRESULT = ::tagHITRESULT;
pub type LPCADWORD = *mut ::CADWORD;
pub type LPCALPOLESTR = *mut ::CALPOLESTR;
pub type LPCAUUID = *mut ::CAUUID;
pub type LPCONNECTDATA = *mut ::CONNECTDATA;
pub type LPCONTROLINFO = *mut ::CONTROLINFO;
pub type LPFONT = *mut ::IFont;
//TODO: Implement
/*
pub type LPADVISESINKEX = *mut ::IAdviseSinkEx;
pub type LPCLASSFACTORY2 = *mut ::IClassFactory2;
pub type LPCONNECTIONPOINT = *mut ::IConnectionPoint;
pub type LPCONNECTIONPOINTCONTAINER = *mut ::IConnectionPointContainer;
pub type LPENUMCONNECTIONPOINTS = *mut ::IEnumConnectionPoints;
pub type LPENUMCONNECTIONS = *mut ::IEnumConnections;
pub type LPENUMOLEUNDOUNITS = *mut ::IEnumOleUndoUnits;
pub type LPFONTDISP = *mut ::IFontDisp;
pub type LPFONTEVENTS = *mut ::IFontEventsDisp;
pub type LPLICINFO = *mut ::LICINFO;
pub type LPOBJECTWITHSITE = *mut ::IObjectWithSite;
pub type LPOLECONTROL = *mut ::IOleControl;
pub type LPOLECONTROLSITE = *mut ::IOleControlSite;
pub type LPOLEINPLACEOBJECTWINDOWLESS = *mut ::IOleInPlaceObjectWindowless; 
pub type LPOLEINPLACESITEEX = *mut ::IOleInPlaceSiteEx;
pub type LPOLEINPLACESITEWINDOWLESS = *mut ::IOleInPlaceSiteWindowless;
pub type LPOLEPARENTUNDOUNIT = *mut ::IOleParentUndoUnit;
pub type LPOLEUNDOMANAGER = *mut ::IOleUndoManager;
pub type LPOLEUNDOUNIT = *mut ::IOleUndoUnit;
pub type LPPERPROPERTYBROWSING = *mut ::IPerPropertyBrowsing;
pub type LPPERSISTMEMORY = *mut ::IPersistMemory;
pub type LPPERSISTPROPERTYBAG = *mut ::IPersistPropertyBag;
pub type LPPERSISTPROPERTYBAG2 = *mut ::IPersistPropertyBag2;
pub type LPPERSISTSTREAMINIT = *mut ::IPersistStreamInit;
pub type LPPICTURE = *mut ::IPicture;
pub type LPPICTURE2 = *mut ::IPicture2;
pub type LPPICTUREDISP = *mut ::IPictureDisp;
pub type LPPOINTERINACTIVE = *mut ::IPointerInactive;
pub type LPPOINTF = *mut ::POINTF;
pub type LPPROPERTYBAG2 = *mut ::IPropertyBag2;
pub type LPPROPERTYNOTIFYSINK = *mut ::IPropertyNotifySink;
pub type LPPROPERTYPAGE = *mut ::IPropertyPage;
pub type LPPROPERTYPAGE2 = *mut ::IPropertyPage2;
pub type LPPROPERTYPAGESITE = *mut ::IPropertyPageSite;
pub type LPPROPPAGEINFO = *mut ::PROPPAGEINFO;
pub type LPPROVIDECLASSINFO = *mut ::IProvideClassInfo;
pub type LPPROVIDECLASSINFO2 = *mut ::IProvideClassInfo2;
pub type LPPROVIDEMULTIPLECLASSINFO = *mut ::IProvideMultipleClassInfo;
pub type LPQUICKACTIVATE = *mut ::IQuickActivate;
pub type LPSIMPLEFRAMESITE = *mut ::ISimpleFrameSite;
pub type LPSPECIFYPROPERTYPAGES = *mut ::ISpecifyPropertyPages;
pub type LPTEXTMETRICOLE = *mut ::TEXTMETRICOLE;
pub type LPVIEWOBJECTEX = *mut ::IViewObjectEx;
pub type OLEDCFLAGS = ::tagOLEDCFLAGS;
pub type OLE_COLOR = ::DWORD;
pub type OLE_HANDLE = ::UINT;
pub type OLE_XPOS_HIMETRIC = ::LONG;
pub type OLE_XSIZE_HIMETRIC = ::LONG;
pub type OLE_YPOS_HIMETRIC = ::LONG;
pub type OLE_YSIZE_HIMETRIC = ::LONG;
pub type PCONNECTDATA = *mut ::CONNECTDATA;
pub type PCONNECTIONPOINT = *mut ::IConnectionPoint;
pub type PCONNECTIONPOINTCONTAINER = *mut ::IConnectionPointContainer;
pub type PENUMCONNECTIONPOINTS = *mut ::IEnumConnectionPoints;
pub type PENUMCONNECTIONS = *mut ::IEnumConnections;
pub type PICTUREATTRIBUTES = ::tagPictureAttributes;
pub type POINTERINACTIVE = ::tagPOINTERINACTIVE;
pub type PROPBAG2_TYPE = ::tagPROPBAG2_TYPE;
pub type PROPPAGESTATUS = ::tagPROPPAGESTATUS;
pub type QACONTAINERFLAGS = ::tagQACONTAINERFLAGS;
*/
pub type READYSTATE = ::tagREADYSTATE;
pub type TEXTMETRICOLE = ::TEXTMETRICW;
pub type UASFLAGS = ::tagUASFLAGS;
pub type VIEWSTATUS = ::tagVIEWSTATUS;
pub type XFORMCOORDS = ::tagXFORMCOORDS;

pub const MULTICLASSINFO_GETIIDPRIMARY: ::UINT = 0x00000004;
pub const MULTICLASSINFO_GETIIDSOURCE: ::UINT = 0x00000008;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: ::UINT = 0x00000002;
pub const MULTICLASSINFO_GETTYPEINFO: ::UINT = 0x00000001;
pub const TIFLAGS_EXTENDDISPATCHONLY: ::UINT = 0x00000001;
