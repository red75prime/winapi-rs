// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of d2d1_3.h

ENUM!{ enum D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE = 0,
    D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION = 1,
    D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE = 0,
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE = 1,
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND = 2,
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_INK_NIB_SHAPE {
    D2D1_INK_NIB_SHAPE_ROUND = 0,
    D2D1_INK_NIB_SHAPE_SQUARE = 1,
    D2D1_INK_NIB_SHAPE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_ORIENTATION {
    D2D1_ORIENTATION_DEFAULT = 1,
    D2D1_ORIENTATION_FLIP_HORIZONTAL = 2,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE180 = 3,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL = 4,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL = 5,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE270 = 6,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL = 7,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE90 = 8,
    D2D1_ORIENTATION_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_PATCH_EDGE_MODE {
    D2D1_PATCH_EDGE_MODE_ALIASED = 0,
    D2D1_PATCH_EDGE_MODE_ANTIALIASED = 1,
    D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED = 2,
    D2D1_PATCH_EDGE_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_SPRITE_OPTIONS {
    D2D1_SPRITE_OPTIONS_NONE = 0,
    D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE = 1,
    D2D1_SPRITE_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE = 0,
    D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE = 1,
    D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

STRUCT!{struct D2D1_GRADIENT_MESH_PATCH {
    point00: ::D2D1_POINT_2F,
    point01: ::D2D1_POINT_2F,
    point02: ::D2D1_POINT_2F,
    point03: ::D2D1_POINT_2F,
    point10: ::D2D1_POINT_2F,
    point11: ::D2D1_POINT_2F,
    point12: ::D2D1_POINT_2F,
    point13: ::D2D1_POINT_2F,
    point20: ::D2D1_POINT_2F,
    point21: ::D2D1_POINT_2F,
    point22: ::D2D1_POINT_2F,
    point23: ::D2D1_POINT_2F,
    point30: ::D2D1_POINT_2F,
    point31: ::D2D1_POINT_2F,
    point32: ::D2D1_POINT_2F,
    point33: ::D2D1_POINT_2F,
    color00: ::D2D1_COLOR_F,
    color03: ::D2D1_COLOR_F,
    color30: ::D2D1_COLOR_F,
    color33: ::D2D1_COLOR_F,
    topEdgeMode: ::D2D1_PATCH_EDGE_MODE,
    leftEdgeMode: ::D2D1_PATCH_EDGE_MODE,
    bottomEdgeMode: ::D2D1_PATCH_EDGE_MODE,
    rightEdgeMode: ::D2D1_PATCH_EDGE_MODE,
}}

STRUCT!{struct D2D1_INK_BEZIER_SEGMENT {
    point1: ::D2D1_INK_POINT,
    point2: ::D2D1_INK_POINT,
    point3: ::D2D1_INK_POINT,
}}

STRUCT!{struct D2D1_INK_POINT {
    x: ::FLOAT,
    y: ::FLOAT,
    radius: ::FLOAT,
}}

STRUCT!{struct D2D1_INK_STYLE_PROPERTIES {
    nibShape: ::D2D1_INK_NIB_SHAPE,
    nibTransform: ::D2D1_MATRIX_3X2_F,
}}

STRUCT!{struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    orientation: ::D2D1_ORIENTATION,
    scaleX: ::FLOAT,
    scaleY: ::FLOAT,
    interpolationMode: ::D2D1_INTERPOLATION_MODE,
    options: ::D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}}

RIDL!(
interface ID2D1CommandSink2(ID2D1CommandSink2Vtbl): ID2D1CommandSink1(ID2D1CommandSink1Vtbl) {
    fn DrawInk(
        &mut self, ink: *mut ::ID2D1Ink, brush: *mut ::ID2D1Brush, inkStyle: *mut ::ID2D1InkStyle
    ) -> ::HRESULT,
    fn DrawGradientMesh(
        &mut self, gradientMesh: *mut ::ID2D1GradientMesh
    ) -> ::HRESULT,
    fn DrawGdiMetafile(
        &mut self, gdiMetafile: *mut ::ID2D1GdiMetafile,
        destinationRectangle: *const ::D2D1_RECT_F, sourceRectangle: *const ::D2D1_RECT_F
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1Device2(ID2D1Device2Vtbl): ID2D1Device1(ID2D1Device1Vtbl) {
    fn CreateDeviceContext(
        &mut self, options: ::D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext2: *mut *mut ::ID2D1DeviceContext2
    ) -> ::HRESULT,
    fn FlushDeviceContexts(&mut self, bitmap: *mut ::ID2D1Bitmap) -> (),
    fn GetDxgiDevice(
        &mut self, dxgiDevice: *mut *mut ::IDXGIDevice
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1DeviceContext2(ID2D1DeviceContext2Vtbl): ID2D1DeviceContext1(ID2D1DeviceContext1Vtbl) {
    fn CreateInk(
        &mut self, startPoint: *const ::D2D1_INK_POINT, ink: *mut *mut ::ID2D1Ink
    ) -> ::HRESULT,
    fn CreateInkStyle(
        &mut self, inkStyleProperties: *const ::D2D1_INK_STYLE_PROPERTIES,
        inkStyle: *mut *mut ::ID2D1InkStyle
    ) -> ::HRESULT,
    fn CreateGradientMesh(
        &mut self, patches: *const ::D2D1_GRADIENT_MESH_PATCH, patchesCount: ::UINT32,
        gradientMesh: *mut *mut ::ID2D1GradientMesh
    ) -> ::HRESULT,
    fn CreateImageSourceFromWic(
        &mut self, wicBitmapSource: *mut ::IWICBitmapSource,
        loadingOptions: ::D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphaMode: ::D2D1_ALPHA_MODE,
        imageSource: *mut *mut ::ID2D1ImageSourceFromWic
    ) -> ::HRESULT,
    fn CreateLookupTable3D(
        &mut self, precision: ::D2D1_BUFFER_PRECISION, extents: *const ::UINT32,
        data: *const ::BYTE, dataCount: ::UINT32, strides: *const ::UINT32,
        lookupTable: *mut *mut ::ID2D1LookupTable3D
    ) -> ::HRESULT,
    fn CreateImageSourceFromDxgi(
        &mut self, surfaces: *mut *mut ::IDXGISurface, surfaceCount: ::UINT32,
        colorSpace: ::DXGI_COLOR_SPACE_TYPE, options: ::D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS,
        imageSource: *mut *mut ::ID2D1ImageSource
    ) -> ::HRESULT,
    fn GetGradientMeshWorldBounds(
        &mut self, gradientMesh: *mut ::ID2D1GradientMesh, pBounds: *mut ::D2D1_RECT_F
    ) -> ::HRESULT,
    fn DrawInk(
        &mut self, ink: *mut ::ID2D1Ink, brush: *mut ::ID2D1Brush, inkStyle: *mut ::ID2D1InkStyle
    ) -> (),
    fn DrawGradientMesh(
        &mut self, gradientMesh: *mut ::ID2D1GradientMesh
    ) -> (),
    fn DrawGdiMetafile(
        &mut self, gdiMetafile: *mut ::ID2D1GdiMetafile,
        destinationRectangle: *const ::D2D1_RECT_F, sourceRectangle: *const ::D2D1_RECT_F
    ) -> (),
    fn CreateTransformedImageSource(
        &mut self, imageSource: *mut ::ID2D1ImageSource,
        properties: *const ::D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES,
        transformedImageSource: *mut *mut ::ID2D1TransformedImageSource
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1Factory3(ID2D1Factory3Vtbl): ID2D1Factory2(ID2D1Factory2Vtbl) {
    fn CreateDevice(
        &mut self, dxgiDevice: *mut ::IDXGIDevice, d2dDevice2: *mut *mut ::ID2D1Device2
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1GdiMetafile1(ID2D1GdiMetafile1Vtbl): ID2D1GdiMetafile(ID2D1GdiMetafileVtbl) {
    fn GetDpi(&mut self, dpiX: *mut ::FLOAT, dpiY: *mut ::FLOAT) -> ::HRESULT,
    fn GetSourceBounds(&mut self, bounds: *mut ::D2D1_RECT_F) -> ::HRESULT
});

RIDL!(
interface ID2D1GdiMetafileSink1(ID2D1GdiMetafileSink1Vtbl): ID2D1GdiMetafileSink(ID2D1GdiMetafileSinkVtbl) {
    fn ProcessRecord(
        &mut self, recordType: ::DWORD, recordData: *const ::c_void, recordDataSize: ::DWORD,
        flags: ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1GradientMesh(ID2D1GradientMeshVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetPatchCount(&mut self) -> ::UINT32,
    fn GetPatches(
        &mut self, startIndex: ::UINT32, patches: *mut ::D2D1_GRADIENT_MESH_PATCH,
        patchesCount: ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1ImageSourceFromWic(ID2D1ImageSourceFromWicVtbl): ID2D1ImageSource(ID2D1ImageSourceVtbl) {
    fn EnsureCached(
        &mut self, rectangleToFill: *const ::D2D1_RECT_U
    ) -> ::HRESULT,
    fn TrimCache(
        &mut self, rectangleToPreserve: *const ::D2D1_RECT_U
    ) -> ::HRESULT,
    fn GetSource(&mut self, wicBitmapSource: *mut *mut ::IWICBitmapSource) -> ()
});

RIDL!(
interface ID2D1ImageSource(ID2D1ImageSourceVtbl): ID2D1Image(ID2D1ImageVtbl) {
    fn OfferResources(&mut self) -> ::HRESULT,
    fn TryReclaimResources(
        &mut self, resourcesDiscarded: *mut ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1InkStyle(ID2D1InkStyleVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetNibTransform(&mut self, transform: *const ::D2D1_MATRIX_3X2_F) -> (),
    fn GetNibTransform(&mut self, transform: *mut ::D2D1_MATRIX_3X2_F) -> (),
    fn SetNibShape(&mut self, nibShape: ::D2D1_INK_NIB_SHAPE) -> (),
    fn GetNibShape(&mut self) -> ::D2D1_INK_NIB_SHAPE
});

RIDL!(
interface ID2D1Ink(ID2D1InkVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetStartPoint(&mut self, startPoint: *const ::D2D1_INK_POINT) -> (),
    fn GetStartPoint(
        &mut self, __ret_val: *mut ::D2D1_INK_POINT
    ) -> *mut ::D2D1_INK_POINT,
    fn AddSegments(
        &mut self, segments: *const ::D2D1_INK_BEZIER_SEGMENT, segmentsCount: ::UINT32
    ) -> ::HRESULT,
    fn RemoveSegmentsAtEnd(&mut self, segmentsCount: ::UINT32) -> ::HRESULT,
    fn SetSegments(
        &mut self, startSegment: ::UINT32, segments: *const ::D2D1_INK_BEZIER_SEGMENT,
        segmentsCount: ::UINT32
    ) -> ::HRESULT,
    fn SetSegmentAtEnd(
        &mut self, segment: *const ::D2D1_INK_BEZIER_SEGMENT
    ) -> ::HRESULT,
    fn GetSegmentCount(&mut self) -> ::UINT32,
    fn GetSegments(
        &mut self, startSegment: ::UINT32, segments: *mut ::D2D1_INK_BEZIER_SEGMENT,
        segmentsCount: ::UINT32
    ) -> ::HRESULT,
    fn StreamAsGeometry(
        &mut self, inkStyle: *mut ::ID2D1InkStyle, worldTransform: *const ::D2D1_MATRIX_3X2_F,
        flatteningTolerance: ::FLOAT, geometrySink: *mut ::ID2D1SimplifiedGeometrySink
    ) -> ::HRESULT,
    fn GetBounds(
        &mut self, inkStyle: *mut ::ID2D1InkStyle, worldTransform: *const ::D2D1_MATRIX_3X2_F,
        bounds: *mut ::D2D1_RECT_F
    ) -> ::HRESULT
});

pub type ID2D1LookupTable3DVtbl = ::ID2D1ResourceVtbl;
pub type ID2D1LookupTable3D = ::ID2D1Resource;

RIDL!(
interface ID2D1TransformedImageSource(ID2D1TransformedImageSourceVtbl): ID2D1Image(ID2D1ImageVtbl) {
    fn GetSource(&mut self, imageSource: *mut *mut ::ID2D1ImageSource) -> (),
    fn GetProperties(
        &mut self, properties: *mut ::D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES
    ) -> ()
});

