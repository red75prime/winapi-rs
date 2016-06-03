// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of d2d1_2.h

ENUM!{ enum D2D1_RENDERING_PRIORITY {
    D2D1_RENDERING_PRIORITY_NORMAL = 0,
    D2D1_RENDERING_PRIORITY_LOW = 1,
    D2D1_RENDERING_PRIORITY_FORCE_DWORD = 0xFFFFFFFF,
}}

RIDL!(
interface ID2D1CommandSink1(ID2D1CommandSink1Vtbl): ID2D1CommandSink(ID2D1CommandSinkVtbl) {
    fn SetPrimitiveBlend1(
        &mut self, primitiveBlend: ::D2D1_PRIMITIVE_BLEND
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1Device1(ID2D1Device1Vtbl): ID2D1Device(ID2D1DeviceVtbl) {
    fn GetRenderingPriority(&mut self) -> ::D2D1_RENDERING_PRIORITY,
    fn SetRenderingPriority(
        &mut self, renderingPriority: ::D2D1_RENDERING_PRIORITY
    ) -> (),
    fn CreateDeviceContext(
        &mut self, options: ::D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext1: *mut *mut ::ID2D1DeviceContext1
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1DeviceContext1(ID2D1DeviceContext1Vtbl): ID2D1DeviceContext(ID2D1DeviceContextVtbl) {
    fn CreateFilledGeometryRealization(
        &mut self, geometry: *mut ::ID2D1Geometry, flatteningTolerance: ::FLOAT,
        geometryRealization: *mut *mut ::ID2D1GeometryRealization
    ) -> ::HRESULT,
    fn CreateStrokedGeometryRealization(
        &mut self, geometry: *mut ::ID2D1Geometry, flatteningTolerance: ::FLOAT,
        strokeWidth: ::FLOAT, strokeStyle: *mut ::ID2D1StrokeStyle,
        geometryRealization: *mut *mut ::ID2D1GeometryRealization
    ) -> ::HRESULT,
    fn DrawGeometryRealization(
        &mut self, geometryRealization: *mut ::ID2D1GeometryRealization, brush: *mut ::ID2D1Brush
    ) -> ()
});

RIDL!(
interface ID2D1Factory2(ID2D1Factory2Vtbl): ID2D1Factory1(ID2D1Factory1Vtbl) {
    fn CreateDevice(
        &mut self, dxgiDevice: *mut ::IDXGIDevice, d2dDevice1: *mut *mut ::ID2D1Device1
    ) -> ::HRESULT
});

pub type ID2D1GeometryRealization = ::ID2D1Resource;
pub type ID2D1GeometryRealizationVtbl = ::ID2D1ResourceVtbl;
DEFINE_GUID!{IID_ID2D1CommandSink1, 0x9eb767fd, 0x4269, 0x4467, 0xb8, 0xc2, 0xeb, 0x30, 0xcb, 0x30, 0x57, 0x43}
DEFINE_GUID!{IID_ID2D1Device1, 0xd21768e1, 0x23a4, 0x4823, 0xa1, 0x4b, 0x7c, 0x3e, 0xba, 0x85, 0xd6, 0x58}
DEFINE_GUID!{IID_ID2D1DeviceContext1, 0xd37f57e4, 0x6908, 0x459f, 0xa1, 0x99, 0xe7, 0x2f, 0x24, 0xf7, 0x99, 0x87}
DEFINE_GUID!{IID_ID2D1Factory2, 0x94f81a73, 0x9212, 0x4376, 0x9c, 0x58, 0xb1, 0x6a, 0x3a, 0x0d, 0x39, 0x92}
DEFINE_GUID!{IID_ID2D1GeometryRealization, 0xa16907d7, 0xbc02, 0x4801, 0x99, 0xe8, 0x8c, 0xf7, 0xf4, 0x85, 0xf7, 0x74}
