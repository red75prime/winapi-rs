// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of DocumentTarget.h

ENUM!{ enum PrintDocumentPackageCompletion {
    PrintDocumentPackageCompletion_InProgress = 0,
    PrintDocumentPackageCompletion_Completed = 1,
    PrintDocumentPackageCompletion_Canceled = 2,
    PrintDocumentPackageCompletion_Failed = 3,
}}

RIDL!(
interface IPrintDocumentPackageStatusEvent(IPrintDocumentPackageStatusEventVtbl): IDispatch(IDispatchVtbl) {
    fn PackageStatusUpdated(
        &mut self, packageStatus: *mut ::PrintDocumentPackageStatus
    ) -> ::HRESULT
});

RIDL!(
interface IPrintDocumentPackageTargetFactory(IPrintDocumentPackageTargetFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDocumentPackageTargetForPrintJob(
        &mut self, printerName: ::LPCWSTR, jobName: ::LPCWSTR, jobOutputStream: *mut ::IStream,
        jobPrintTicketStream: *mut ::IStream,
        docPackageTarget: *mut *mut ::IPrintDocumentPackageTarget
    ) -> ::HRESULT
});

RIDL!(
interface IPrintDocumentPackageTarget(IPrintDocumentPackageTargetVtbl): IUnknown(IUnknownVtbl) {
    fn GetPackageTargetTypes(
        &mut self, targetCount: *mut ::UINT32, targetTypes: *mut *mut ::GUID
    ) -> ::HRESULT,
    fn GetPackageTarget(
        &mut self, guidTargetType: *const ::GUID, riid: *const ::IID, ppvTarget: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn Cancel(&mut self) -> ::HRESULT
});

STRUCT!{struct PrintDocumentPackageStatus {
    JobId: ::UINT32,
    CurrentDocument: ::INT32,
    CurrentPage: ::INT32,
    CurrentPageTotal: ::INT32,
    Completion: ::PrintDocumentPackageCompletion,
    PackageStatus: ::HRESULT,
}}
