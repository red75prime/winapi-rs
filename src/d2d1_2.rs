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
