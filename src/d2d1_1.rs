// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of d2d1_1.h

ENUM!{ enum D2D1_BITMAP_OPTIONS {
    D2D1_BITMAP_OPTIONS_NONE = 0,
    D2D1_BITMAP_OPTIONS_TARGET = 1,
    D2D1_BITMAP_OPTIONS_CANNOT_DRAW = 2,
    D2D1_BITMAP_OPTIONS_CPU_READ = 4,
    D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE = 8,
    D2D1_BITMAP_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_BUFFER_PRECISION {
    D2D1_BUFFER_PRECISION_UNKNOWN = 0,
    D2D1_BUFFER_PRECISION_8BPC_UNORM = 1,
    D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB = 2,
    D2D1_BUFFER_PRECISION_16BPC_UNORM = 3,
    D2D1_BUFFER_PRECISION_16BPC_FLOAT = 4,
    D2D1_BUFFER_PRECISION_32BPC_FLOAT = 5,
    D2D1_BUFFER_PRECISION_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_COLOR_INTERPOLATION_MODE {
    D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT = 0,
    D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED = 1,
    D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_COLOR_SPACE {
    D2D1_COLOR_SPACE_CUSTOM = 0,
    D2D1_COLOR_SPACE_SRGB = 1,
    D2D1_COLOR_SPACE_SCRGB = 2,
    D2D1_COLOR_SPACE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_COMPOSITE_MODE {
    D2D1_COMPOSITE_MODE_SOURCE_OVER = 0,
    D2D1_COMPOSITE_MODE_DESTINATION_OVER = 1,
    D2D1_COMPOSITE_MODE_SOURCE_IN = 2,
    D2D1_COMPOSITE_MODE_DESTINATION_IN = 3,
    D2D1_COMPOSITE_MODE_SOURCE_OUT = 4,
    D2D1_COMPOSITE_MODE_DESTINATION_OUT = 5,
    D2D1_COMPOSITE_MODE_SOURCE_ATOP = 6,
    D2D1_COMPOSITE_MODE_DESTINATION_ATOP = 7,
    D2D1_COMPOSITE_MODE_XOR = 8,
    D2D1_COMPOSITE_MODE_PLUS = 9,
    D2D1_COMPOSITE_MODE_SOURCE_COPY = 10,
    D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY = 11,
    D2D1_COMPOSITE_MODE_MASK_INVERT = 12,
    D2D1_COMPOSITE_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_DEVICE_CONTEXT_OPTIONS {
    D2D1_DEVICE_CONTEXT_OPTIONS_NONE = 0,
    D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS = 1,
    D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_INTERPOLATION_MODE {
    D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0,
    D2D1_INTERPOLATION_MODE_LINEAR = 1,
    D2D1_INTERPOLATION_MODE_CUBIC = 2,
    D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 3,
    D2D1_INTERPOLATION_MODE_ANISOTROPIC = 4,
    D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC = 5,
    D2D1_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_LAYER_OPTIONS1 {
    D2D1_LAYER_OPTIONS1_NONE = 0,
    D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND = 1,
    D2D1_LAYER_OPTIONS1_IGNORE_ALPHA = 2,
    D2D1_LAYER_OPTIONS1_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_MAP_OPTIONS {
    D2D1_MAP_OPTIONS_NONE = 0,
    D2D1_MAP_OPTIONS_READ = 1,
    D2D1_MAP_OPTIONS_WRITE = 2,
    D2D1_MAP_OPTIONS_DISCARD = 4,
    D2D1_MAP_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_PRIMITIVE_BLEND {
    D2D1_PRIMITIVE_BLEND_SOURCE_OVER = 0,
    D2D1_PRIMITIVE_BLEND_COPY = 1,
    D2D1_PRIMITIVE_BLEND_MIN = 2,
    D2D1_PRIMITIVE_BLEND_ADD = 3,
    D2D1_PRIMITIVE_BLEND_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_PRINT_FONT_SUBSET_MODE {
    D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT = 0,
    D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE = 1,
    D2D1_PRINT_FONT_SUBSET_MODE_NONE = 2,
    D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_PROPERTY {
    D2D1_PROPERTY_CLSID = 0x80000000,
    D2D1_PROPERTY_DISPLAYNAME = 0x80000001,
    D2D1_PROPERTY_AUTHOR = 0x80000002,
    D2D1_PROPERTY_CATEGORY = 0x80000003,
    D2D1_PROPERTY_DESCRIPTION = 0x80000004,
    D2D1_PROPERTY_INPUTS = 0x80000005,
    D2D1_PROPERTY_CACHED = 0x80000006,
    D2D1_PROPERTY_PRECISION = 0x80000007,
    D2D1_PROPERTY_MIN_INPUTS = 0x80000008,
    D2D1_PROPERTY_MAX_INPUTS = 0x80000009,
    D2D1_PROPERTY_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_PROPERTY_TYPE {
    D2D1_PROPERTY_TYPE_UNKNOWN = 0,
    D2D1_PROPERTY_TYPE_STRING = 1,
    D2D1_PROPERTY_TYPE_BOOL = 2,
    D2D1_PROPERTY_TYPE_UINT32 = 3,
    D2D1_PROPERTY_TYPE_INT32 = 4,
    D2D1_PROPERTY_TYPE_FLOAT = 5,
    D2D1_PROPERTY_TYPE_VECTOR2 = 6,
    D2D1_PROPERTY_TYPE_VECTOR3 = 7,
    D2D1_PROPERTY_TYPE_VECTOR4 = 8,
    D2D1_PROPERTY_TYPE_BLOB = 9,
    D2D1_PROPERTY_TYPE_IUNKNOWN = 10,
    D2D1_PROPERTY_TYPE_ENUM = 11,
    D2D1_PROPERTY_TYPE_ARRAY = 12,
    D2D1_PROPERTY_TYPE_CLSID = 13,
    D2D1_PROPERTY_TYPE_MATRIX_3X2 = 14,
    D2D1_PROPERTY_TYPE_MATRIX_4X3 = 15,
    D2D1_PROPERTY_TYPE_MATRIX_4X4 = 16,
    D2D1_PROPERTY_TYPE_MATRIX_5X4 = 17,
    D2D1_PROPERTY_TYPE_COLOR_CONTEXT = 18,
    D2D1_PROPERTY_TYPE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_STROKE_TRANSFORM_TYPE {
    D2D1_STROKE_TRANSFORM_TYPE_NORMAL = 0,
    D2D1_STROKE_TRANSFORM_TYPE_FIXED = 1,
    D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE = 2,
    D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_SUBPROPERTY {
    D2D1_SUBPROPERTY_DISPLAYNAME = 0x80000000,
    D2D1_SUBPROPERTY_ISREADONLY = 0x80000001,
    D2D1_SUBPROPERTY_MIN = 0x80000002,
    D2D1_SUBPROPERTY_MAX = 0x80000003,
    D2D1_SUBPROPERTY_DEFAULT = 0x80000004,
    D2D1_SUBPROPERTY_FIELDS = 0x80000005,
    D2D1_SUBPROPERTY_INDEX = 0x80000006,
    D2D1_SUBPROPERTY_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_THREADING_MODE {
    D2D1_THREADING_MODE_SINGLE_THREADED = 0,
    D2D1_THREADING_MODE_MULTI_THREADED = 1,
    D2D1_THREADING_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_UNIT_MODE {
    D2D1_UNIT_MODE_DIPS = 0,
    D2D1_UNIT_MODE_PIXELS = 1,
    D2D1_UNIT_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

STRUCT!{struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    extendModeX: ::D2D1_EXTEND_MODE,
    extendModeY: ::D2D1_EXTEND_MODE,
    interpolationMode: ::D2D1_INTERPOLATION_MODE,
}}

STRUCT!{struct D2D1_BITMAP_PROPERTIES1 {
    pixelFormat: ::D2D1_PIXEL_FORMAT,
    dpiX: ::FLOAT,
    dpiY: ::FLOAT,
    bitmapOptions: ::D2D1_BITMAP_OPTIONS,
    colorContext: *mut ::ID2D1ColorContext,
}}

STRUCT!{struct D2D1_CREATION_PROPERTIES {
    threadingMode: ::D2D1_THREADING_MODE,
    debugLevel: ::D2D1_DEBUG_LEVEL,
    options: ::D2D1_DEVICE_CONTEXT_OPTIONS,
}}

STRUCT!{struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    antialiasMode: ::D2D1_ANTIALIAS_MODE,
    textAntialiasMode: ::D2D1_TEXT_ANTIALIAS_MODE,
    tag1: ::D2D1_TAG,
    tag2: ::D2D1_TAG,
    transform: ::D2D1_MATRIX_3X2_F,
    primitiveBlend: ::D2D1_PRIMITIVE_BLEND,
    unitMode: ::D2D1_UNIT_MODE,
}}

STRUCT!{struct D2D1_EFFECT_INPUT_DESCRIPTION {
    effect: *mut ::ID2D1Effect,
    inputIndex: ::UINT32,
    inputRectangle: ::D2D1_RECT_F,
}}

STRUCT!{struct D2D1_IMAGE_BRUSH_PROPERTIES {
    sourceRectangle: ::D2D1_RECT_F,
    extendModeX: ::D2D1_EXTEND_MODE,
    extendModeY: ::D2D1_EXTEND_MODE,
    interpolationMode: ::D2D1_INTERPOLATION_MODE,
}}

STRUCT!{struct D2D1_LAYER_PARAMETERS1 {
    contentBounds: ::D2D1_RECT_F,
    geometricMask: *mut ::ID2D1Geometry,
    maskAntialiasMode: ::D2D1_ANTIALIAS_MODE,
    maskTransform: ::D2D1_MATRIX_3X2_F,
    opacity: ::FLOAT,
    opacityBrush: *mut ::ID2D1Brush,
    layerOptions: ::D2D1_LAYER_OPTIONS1,
}}

STRUCT!{struct D2D1_MAPPED_RECT {
    pitch: ::UINT32,
    bits: *mut ::BYTE,
}}

STRUCT!{struct D2D1_POINT_DESCRIPTION {
    point: ::D2D1_POINT_2F,
    unitTangentVector: ::D2D1_POINT_2F,
    endSegment: ::UINT32,
    endFigure: ::UINT32,
    lengthToEndSegment: ::FLOAT,
}}

STRUCT!{struct D2D1_PRINT_CONTROL_PROPERTIES {
    fontSubset: ::D2D1_PRINT_FONT_SUBSET_MODE,
    rasterDPI: ::FLOAT,
    colorSpace: ::D2D1_COLOR_SPACE,
}}

STRUCT!{struct D2D1_RENDERING_CONTROLS {
    bufferPrecision: ::D2D1_BUFFER_PRECISION,
    tileSize: ::D2D1_SIZE_U,
}}

STRUCT!{struct D2D1_STROKE_STYLE_PROPERTIES1 {
    startCap: ::D2D1_CAP_STYLE,
    endCap: ::D2D1_CAP_STYLE,
    dashCap: ::D2D1_CAP_STYLE,
    lineJoin: ::D2D1_LINE_JOIN,
    miterLimit: ::FLOAT,
    dashStyle: ::D2D1_DASH_STYLE,
    dashOffset: ::FLOAT,
    transformType: ::D2D1_STROKE_TRANSFORM_TYPE,
}}

RIDL!(
interface ID2D1Bitmap1(ID2D1Bitmap1Vtbl): ID2D1Bitmap(ID2D1BitmapVtbl) {
    fn GetColorContext(
        &mut self, colorContext: *mut *mut ::ID2D1ColorContext
    ) -> (),
    fn GetOptions(&mut self) -> ::D2D1_BITMAP_OPTIONS,
    fn GetSurface(
        &mut self, dxgiSurface: *mut *mut ::IDXGISurface
    ) -> ::HRESULT,
    fn Map(
        &mut self, options: ::D2D1_MAP_OPTIONS, mappedRect: *mut ::D2D1_MAPPED_RECT
    ) -> ::HRESULT,
    fn Unmap(&mut self) -> ::HRESULT
});

RIDL!(
interface ID2D1BitmapBrush1(ID2D1BitmapBrush1Vtbl): ID2D1BitmapBrush(ID2D1BitmapBrushVtbl) {
    fn SetInterpolationMode1(
        &mut self, interpolationMode: ::D2D1_INTERPOLATION_MODE
    ) -> (),
    fn GetInterpolationMode1(&mut self) -> ::D2D1_INTERPOLATION_MODE
});

RIDL!(
interface ID2D1ColorContext(ID2D1ColorContextVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetColorSpace(&mut self) -> ::D2D1_COLOR_SPACE,
    fn GetProfileSize(&mut self) -> ::UINT32,
    fn GetProfile(
        &mut self, profile: *mut ::BYTE, profileSize: ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1CommandList(ID2D1CommandListVtbl): ID2D1Image(ID2D1ImageVtbl) {
    fn Stream(&mut self, sink: *mut ::ID2D1CommandSink) -> ::HRESULT,
    fn Close(&mut self) -> ::HRESULT
});

RIDL!(
interface ID2D1CommandSink(ID2D1CommandSinkVtbl): IUnknown(IUnknownVtbl) {
    fn BeginDraw(&mut self) -> ::HRESULT,
    fn EndDraw(&mut self) -> ::HRESULT,
    fn SetAntialiasMode(
        &mut self, antialiasMode: ::D2D1_ANTIALIAS_MODE
    ) -> ::HRESULT,
    fn SetTags(&mut self, tag1: ::D2D1_TAG, tag2: ::D2D1_TAG) -> ::HRESULT,
    fn SetTextAntialiasMode(
        &mut self, textAntialiasMode: ::D2D1_TEXT_ANTIALIAS_MODE
    ) -> ::HRESULT,
    fn SetTextRenderingParams(
        &mut self, textRenderingParams: *mut ::IDWriteRenderingParams
    ) -> ::HRESULT,
    fn SetTransform(
        &mut self, transform: *const ::D2D1_MATRIX_3X2_F
    ) -> ::HRESULT,
    fn SetPrimitiveBlend(
        &mut self, primitiveBlend: ::D2D1_PRIMITIVE_BLEND
    ) -> ::HRESULT,
    fn SetUnitMode(&mut self, unitMode: ::D2D1_UNIT_MODE) -> ::HRESULT,
    fn Clear(&mut self, color: *const ::D2D1_COLOR_F) -> ::HRESULT,
    fn DrawGlyphRun(
        &mut self, baselineOrigin: ::D2D1_POINT_2F, glyphRun: *const ::DWRITE_GLYPH_RUN,
        glyphRunDescription: *const ::DWRITE_GLYPH_RUN_DESCRIPTION,
        foregroundBrush: *mut ::ID2D1Brush, measuringMode: ::DWRITE_MEASURING_MODE
    ) -> ::HRESULT,
    fn DrawLine(
        &mut self, point0: ::D2D1_POINT_2F, point1: ::D2D1_POINT_2F, brush: *mut ::ID2D1Brush,
        strokeWidth: ::FLOAT, strokeStyle: *mut ::ID2D1StrokeStyle
    ) -> ::HRESULT,
    fn DrawGeometry(
        &mut self, geometry: *mut ::ID2D1Geometry, brush: *mut ::ID2D1Brush, strokeWidth: ::FLOAT,
        strokeStyle: *mut ::ID2D1StrokeStyle
    ) -> ::HRESULT,
    fn DrawRectangle(
        &mut self, rect: *const ::D2D1_RECT_F, brush: *mut ::ID2D1Brush, strokeWidth: ::FLOAT,
        strokeStyle: *mut ::ID2D1StrokeStyle
    ) -> ::HRESULT,
    fn DrawBitmap(
        &mut self, bitmap: *mut ::ID2D1Bitmap, destinationRectangle: *const ::D2D1_RECT_F,
        opacity: ::FLOAT, interpolationMode: ::D2D1_INTERPOLATION_MODE,
        sourceRectangle: *const ::D2D1_RECT_F, perspectiveTransform: *const ::D2D1_MATRIX_4X4_F
    ) -> ::HRESULT,
    fn DrawImage(
        &mut self, image: *mut ::ID2D1Image, targetOffset: *const ::D2D1_POINT_2F,
        imageRectangle: *const ::D2D1_RECT_F, interpolationMode: ::D2D1_INTERPOLATION_MODE,
        compositeMode: ::D2D1_COMPOSITE_MODE
    ) -> ::HRESULT,
    fn DrawGdiMetafile(
        &mut self, gdiMetafile: *mut ::ID2D1GdiMetafile, targetOffset: *const ::D2D1_POINT_2F
    ) -> ::HRESULT,
    fn FillMesh(
        &mut self, mesh: *mut ::ID2D1Mesh, brush: *mut ::ID2D1Brush
    ) -> ::HRESULT,
    fn FillOpacityMask(
        &mut self, opacityMask: *mut ::ID2D1Bitmap, brush: *mut ::ID2D1Brush,
        destinationRectangle: *const ::D2D1_RECT_F, sourceRectangle: *const ::D2D1_RECT_F
    ) -> ::HRESULT,
    fn FillGeometry(
        &mut self, geometry: *mut ::ID2D1Geometry, brush: *mut ::ID2D1Brush,
        opacityBrush: *mut ::ID2D1Brush
    ) -> ::HRESULT,
    fn FillRectangle(
        &mut self, rect: *const ::D2D1_RECT_F, brush: *mut ::ID2D1Brush
    ) -> ::HRESULT,
    fn PushAxisAlignedClip(
        &mut self, clipRect: *const ::D2D1_RECT_F, antialiasMode: ::D2D1_ANTIALIAS_MODE
    ) -> ::HRESULT,
    fn PushLayer(
        &mut self, layerParameters1: *const ::D2D1_LAYER_PARAMETERS1, layer: *mut ::ID2D1Layer
    ) -> ::HRESULT,
    fn PopAxisAlignedClip(&mut self) -> ::HRESULT,
    fn PopLayer(&mut self) -> ::HRESULT
});

RIDL!(
interface ID2D1DeviceContext(ID2D1DeviceContextVtbl): ID2D1RenderTarget(ID2D1RenderTargetVtbl) {
    fn CreateBitmap(
        &mut self, size: ::D2D1_SIZE_U, sourceData: *const ::c_void, pitch: ::UINT32,
        bitmapProperties: *const ::D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::ID2D1Bitmap1
    ) -> ::HRESULT,
    fn CreateBitmapFromWicBitmap(
        &mut self, wicBitmapSource: *mut ::IWICBitmapSource,
        bitmapProperties: *const ::D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::ID2D1Bitmap1
    ) -> ::HRESULT,
    fn CreateColorContext(
        &mut self, space: ::D2D1_COLOR_SPACE, profile: *const ::BYTE, profileSize: ::UINT32,
        colorContext: *mut *mut ::ID2D1ColorContext
    ) -> ::HRESULT,
    fn CreateColorContextFromFilename(
        &mut self, filename: ::PCWSTR, colorContext: *mut *mut ::ID2D1ColorContext
    ) -> ::HRESULT,
    fn CreateColorContextFromWicColorContext(
        &mut self, wicColorContext: *mut ::IWICColorContext,
        colorContext: *mut *mut ::ID2D1ColorContext
    ) -> ::HRESULT,
    fn CreateBitmapFromDxgiSurface(
        &mut self, surface: *mut ::IDXGISurface,
        bitmapProperties: *const ::D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::ID2D1Bitmap1
    ) -> ::HRESULT,
    fn CreateEffect(
        &mut self, effectId: *const ::IID, effect: *mut *mut ::ID2D1Effect
    ) -> ::HRESULT,
    fn CreateGradientStopCollection(
        &mut self, straightAlphaGradientStops: *const ::D2D1_GRADIENT_STOP,
        straightAlphaGradientStopsCount: ::UINT32, preInterpolationSpace: ::D2D1_COLOR_SPACE,
        postInterpolationSpace: ::D2D1_COLOR_SPACE, bufferPrecision: ::D2D1_BUFFER_PRECISION,
        extendMode: ::D2D1_EXTEND_MODE, colorInterpolationMode: ::D2D1_COLOR_INTERPOLATION_MODE,
        gradientStopCollection1: *mut *mut ::ID2D1GradientStopCollection1
    ) -> ::HRESULT,
    fn CreateImageBrush(
        &mut self, image: *mut ::ID2D1Image,
        imageBrushProperties: *const ::D2D1_IMAGE_BRUSH_PROPERTIES,
        brushProperties: *const ::D2D1_BRUSH_PROPERTIES, imageBrush: *mut *mut ::ID2D1ImageBrush
    ) -> ::HRESULT,
    fn CreateBitmapBrush(
        &mut self, bitmap: *mut ::ID2D1Bitmap,
        bitmapBrushProperties: *const ::D2D1_BITMAP_BRUSH_PROPERTIES1,
        brushProperties: *const ::D2D1_BRUSH_PROPERTIES, bitmapBrush: *mut *mut ::ID2D1BitmapBrush1
    ) -> ::HRESULT,
    fn CreateCommandList(
        &mut self, commandList: *mut *mut ::ID2D1CommandList
    ) -> ::HRESULT,
    fn IsDxgiFormatSupported(&mut self, format: ::DXGI_FORMAT) -> ::BOOL,
    fn IsBufferPrecisionSupported(
        &mut self, bufferPrecision: ::D2D1_BUFFER_PRECISION
    ) -> ::BOOL,
    fn GetImageLocalBounds(
        &mut self, image: *mut ::ID2D1Image, localBounds: *mut ::D2D1_RECT_F
    ) -> ::HRESULT,
    fn GetImageWorldBounds(
        &mut self, image: *mut ::ID2D1Image, worldBounds: *mut ::D2D1_RECT_F
    ) -> ::HRESULT,
    fn GetGlyphRunWorldBounds(
        &mut self, baselineOrigin: ::D2D1_POINT_2F, glyphRun: *const ::DWRITE_GLYPH_RUN,
        measuringMode: ::DWRITE_MEASURING_MODE, bounds: *mut ::D2D1_RECT_F
    ) -> ::HRESULT,
    fn GetDevice(&mut self, device: *mut *mut ::ID2D1Device) -> (),
    fn SetTarget(&mut self, image: *mut ::ID2D1Image) -> (),
    fn GetTarget(&mut self, image: *mut *mut ::ID2D1Image) -> (),
    fn SetRenderingControls(
        &mut self, renderingControls: *const ::D2D1_RENDERING_CONTROLS
    ) -> (),
    fn GetRenderingControls(
        &mut self, renderingControls: *mut ::D2D1_RENDERING_CONTROLS
    ) -> (),
    fn SetPrimitiveBlend(
        &mut self, primitiveBlend: ::D2D1_PRIMITIVE_BLEND
    ) -> (),
    fn GetPrimitiveBlend(&mut self) -> ::D2D1_PRIMITIVE_BLEND,
    fn SetUnitMode(&mut self, unitMode: ::D2D1_UNIT_MODE) -> (),
    fn GetUnitMode(&mut self) -> ::D2D1_UNIT_MODE,
    fn DrawGlyphRun(
        &mut self, baselineOrigin: ::D2D1_POINT_2F, glyphRun: *const ::DWRITE_GLYPH_RUN,
        glyphRunDescription: *const ::DWRITE_GLYPH_RUN_DESCRIPTION,
        foregroundBrush: *mut ::ID2D1Brush, measuringMode: ::DWRITE_MEASURING_MODE
    ) -> (),
    fn DrawImage(
        &mut self, image: *mut ::ID2D1Image, targetOffset: *const ::D2D1_POINT_2F,
        imageRectangle: *const ::D2D1_RECT_F, interpolationMode: ::D2D1_INTERPOLATION_MODE,
        compositeMode: ::D2D1_COMPOSITE_MODE
    ) -> (),
    fn DrawGdiMetafile(
        &mut self, gdiMetafile: *mut ::ID2D1GdiMetafile, targetOffset: *const ::D2D1_POINT_2F
    ) -> (),
    fn DrawBitmap(
        &mut self, bitmap: *mut ::ID2D1Bitmap, destinationRectangle: *const ::D2D1_RECT_F,
        opacity: ::FLOAT, interpolationMode: ::D2D1_INTERPOLATION_MODE,
        sourceRectangle: *const ::D2D1_RECT_F, perspectiveTransform: *const ::D2D1_MATRIX_4X4_F
    ) -> (),
    fn PushLayer(
        &mut self, layerParameters: *const ::D2D1_LAYER_PARAMETERS1, layer: *mut ::ID2D1Layer
    ) -> (),
    fn InvalidateEffectInputRectangle(
        &mut self, effect: *mut ::ID2D1Effect, input: ::UINT32,
        inputRectangle: *const ::D2D1_RECT_F
    ) -> ::HRESULT,
    fn GetEffectInvalidRectangleCount(
        &mut self, effect: *mut ::ID2D1Effect, rectangleCount: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetEffectInvalidRectangles(
        &mut self, effect: *mut ::ID2D1Effect, rectangles: *mut ::D2D1_RECT_F,
        rectanglesCount: ::UINT32
    ) -> ::HRESULT,
    fn GetEffectRequiredInputRectangles(
        &mut self, renderEffect: *mut ::ID2D1Effect, renderImageRectangle: *const ::D2D1_RECT_F,
        inputDescriptions: *const ::D2D1_EFFECT_INPUT_DESCRIPTION,
        requiredInputRects: *mut ::D2D1_RECT_F, inputCount: ::UINT32
    ) -> ::HRESULT,
    fn FillOpacityMask(
        &mut self, opacityMask: *mut ::ID2D1Bitmap, brush: *mut ::ID2D1Brush,
        destinationRectangle: *const ::D2D1_RECT_F, sourceRectangle: *const ::D2D1_RECT_F
    ) -> ()
});

RIDL!(
interface ID2D1Device(ID2D1DeviceVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn CreateDeviceContext(
        &mut self, options: ::D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext: *mut *mut ::ID2D1DeviceContext
    ) -> ::HRESULT,
    fn CreatePrintControl(
        &mut self, wicFactory: *mut ::IWICImagingFactory,
        documentTarget: *mut ::IPrintDocumentPackageTarget,
        printControlProperties: *const ::D2D1_PRINT_CONTROL_PROPERTIES,
        printControl: *mut *mut ::ID2D1PrintControl
    ) -> ::HRESULT,
    fn SetMaximumTextureMemory(&mut self, maximumInBytes: ::UINT64) -> (),
    fn GetMaximumTextureMemory(&mut self) -> ::UINT64,
    fn ClearResources(&mut self, millisecondsSinceUse: ::UINT32) -> ()
});

RIDL!(
interface ID2D1DrawingStateBlock1(ID2D1DrawingStateBlock1Vtbl): ID2D1DrawingStateBlock(ID2D1DrawingStateBlockVtbl) {
    fn GetDescription(
        &mut self, stateDescription: *mut ::D2D1_DRAWING_STATE_DESCRIPTION1
    ) -> (),
    fn SetDescription(
        &mut self, stateDescription: *const ::D2D1_DRAWING_STATE_DESCRIPTION1
    ) -> ()
});

RIDL!(
interface ID2D1Effect(ID2D1EffectVtbl): ID2D1Properties(ID2D1PropertiesVtbl) {
    fn SetInput(
        &mut self, index: ::UINT32, input: *mut ::ID2D1Image, invalidate: ::BOOL
    ) -> (),
    fn SetInputCount(&mut self, inputCount: ::UINT32) -> ::HRESULT,
    fn GetInput(
        &mut self, index: ::UINT32, input: *mut *mut ::ID2D1Image
    ) -> (),
    fn GetInputCount(&mut self) -> ::UINT32,
    fn GetOutput(&mut self, outputImage: *mut *mut ::ID2D1Image) -> ()
});

RIDL!(
interface ID2D1Factory1(ID2D1Factory1Vtbl): ID2D1Factory(ID2D1FactoryVtbl) {
    fn CreateDevice(
        &mut self, dxgiDevice: *mut ::IDXGIDevice, d2dDevice: *mut *mut ::ID2D1Device
    ) -> ::HRESULT,
    fn CreateStrokeStyle(
        &mut self, strokeStyleProperties: *const ::D2D1_STROKE_STYLE_PROPERTIES1,
        dashes: *const ::FLOAT, dashesCount: ::UINT32, strokeStyle: *mut *mut ::ID2D1StrokeStyle1
    ) -> ::HRESULT,
    fn CreatePathGeometry(
        &mut self, pathGeometry: *mut *mut ::ID2D1PathGeometry1
    ) -> ::HRESULT,
    fn CreateDrawingStateBlock(
        &mut self, drawingStateDescription: *const ::D2D1_DRAWING_STATE_DESCRIPTION1,
        textRenderingParams: *mut ::IDWriteRenderingParams,
        drawingStateBlock: *mut *mut ::ID2D1DrawingStateBlock1
    ) -> ::HRESULT,
    fn CreateGdiMetafile(
        &mut self, metafileStream: *mut ::IStream, metafile: *mut *mut ::ID2D1GdiMetafile
    ) -> ::HRESULT,
    fn RegisterEffectFromStream(
        &mut self, classId: *const ::IID, propertyXml: *mut ::IStream,
        bindings: *const ::D2D1_PROPERTY_BINDING, bindingsCount: ::UINT32,
        effectFactory: ::PD2D1_EFFECT_FACTORY
    ) -> ::HRESULT,
    fn RegisterEffectFromString(
        &mut self, classId: *const ::IID, propertyXml: ::PCWSTR,
        bindings: *const ::D2D1_PROPERTY_BINDING, bindingsCount: ::UINT32,
        effectFactory: ::PD2D1_EFFECT_FACTORY
    ) -> ::HRESULT,
    fn UnregisterEffect(&mut self, classId: *const ::IID) -> ::HRESULT,
    fn GetRegisteredEffects(
        &mut self, effects: *mut ::CLSID, effectsCount: ::UINT32, effectsReturned: *mut ::UINT32,
        effectsRegistered: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetEffectProperties(
        &mut self, effectId: *const ::IID, properties: *mut *mut ::ID2D1Properties
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1GdiMetafileSink(ID2D1GdiMetafileSinkVtbl): IUnknown(IUnknownVtbl) {
    fn ProcessRecord(
        &mut self, recordType: ::DWORD, recordData: *const ::c_void, recordDataSize: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1GdiMetafile(ID2D1GdiMetafileVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn Stream(&mut self, sink: *mut ::ID2D1GdiMetafileSink) -> ::HRESULT,
    fn GetBounds(&mut self, bounds: *mut ::D2D1_RECT_F) -> ::HRESULT
});

RIDL!(
interface ID2D1GradientStopCollection1(ID2D1GradientStopCollection1Vtbl): ID2D1GradientStopCollection(ID2D1GradientStopCollectionVtbl) {
    fn GetGradientStops1(
        &mut self, gradientStops: *mut ::D2D1_GRADIENT_STOP, gradientStopsCount: ::UINT32
    ) -> (),
    fn GetPreInterpolationSpace(&mut self) -> ::D2D1_COLOR_SPACE,
    fn GetPostInterpolationSpace(&mut self) -> ::D2D1_COLOR_SPACE,
    fn GetBufferPrecision(&mut self) -> ::D2D1_BUFFER_PRECISION,
    fn GetColorInterpolationMode(&mut self) -> ::D2D1_COLOR_INTERPOLATION_MODE
});

RIDL!(
interface ID2D1ImageBrush(ID2D1ImageBrushVtbl): ID2D1Brush(ID2D1BrushVtbl) {
    fn SetImage(&mut self, image: *mut ::ID2D1Image) -> (),
    fn SetExtendModeX(&mut self, extendModeX: ::D2D1_EXTEND_MODE) -> (),
    fn SetExtendModeY(&mut self, extendModeY: ::D2D1_EXTEND_MODE) -> (),
    fn SetInterpolationMode(
        &mut self, interpolationMode: ::D2D1_INTERPOLATION_MODE
    ) -> (),
    fn SetSourceRectangle(
        &mut self, sourceRectangle: *const ::D2D1_RECT_F
    ) -> (),
    fn GetImage(&mut self, image: *mut *mut ::ID2D1Image) -> (),
    fn GetExtendModeX(&mut self) -> ::D2D1_EXTEND_MODE,
    fn GetExtendModeY(&mut self) -> ::D2D1_EXTEND_MODE,
    fn GetInterpolationMode(&mut self) -> ::D2D1_INTERPOLATION_MODE,
    fn GetSourceRectangle(&mut self, sourceRectangle: *mut ::D2D1_RECT_F) -> ()
});

RIDL!(
interface ID2D1Multithread(ID2D1MultithreadVtbl): IUnknown(IUnknownVtbl) {
    fn GetMultithreadProtected(&mut self) -> ::BOOL,
    fn Enter(&mut self) -> (),
    fn Leave(&mut self) -> ()
});

RIDL!(
interface ID2D1PathGeometry1(ID2D1PathGeometry1Vtbl): ID2D1PathGeometry(ID2D1PathGeometryVtbl) {
    fn ComputePointAndSegmentAtLength(
        &mut self, length: ::FLOAT, startSegment: ::UINT32,
        worldTransform: *const ::D2D1_MATRIX_3X2_F, flatteningTolerance: ::FLOAT,
        pointDescription: *mut ::D2D1_POINT_DESCRIPTION
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1PrintControl(ID2D1PrintControlVtbl): IUnknown(IUnknownVtbl) {
    fn AddPage(
        &mut self, commandList: *mut ::ID2D1CommandList, pageSize: ::D2D_SIZE_F,
        pagePrintTicketStream: *mut ::IStream, tag1: *mut ::D2D1_TAG, tag2: *mut ::D2D1_TAG
    ) -> ::HRESULT,
    fn Close(&mut self) -> ::HRESULT
});

RIDL!(
interface ID2D1Properties(ID2D1PropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyCount(&mut self) -> ::UINT32,
    fn GetPropertyName(
        &mut self, index: ::UINT32, name: ::PWSTR, nameCount: ::UINT32
    ) -> ::HRESULT,
    fn GetPropertyNameLength(&mut self, index: ::UINT32) -> ::UINT32,
    fn GetType(&mut self, index: ::UINT32) -> ::D2D1_PROPERTY_TYPE,
    fn GetPropertyIndex(&mut self, name: ::PCWSTR) -> ::UINT32,
    fn SetValueByName(
        &mut self, name: ::PCWSTR, type_: ::D2D1_PROPERTY_TYPE, data: *const ::BYTE,
        dataSize: ::UINT32
    ) -> ::HRESULT,
    fn SetValue(
        &mut self, index: ::UINT32, type_: ::D2D1_PROPERTY_TYPE, data: *const ::BYTE,
        dataSize: ::UINT32
    ) -> ::HRESULT,
    fn GetValueByName(
        &mut self, name: ::PCWSTR, type_: ::D2D1_PROPERTY_TYPE, data: *mut ::BYTE,
        dataSize: ::UINT32
    ) -> ::HRESULT,
    fn GetValue(
        &mut self, index: ::UINT32, type_: ::D2D1_PROPERTY_TYPE, data: *mut ::BYTE,
        dataSize: ::UINT32
    ) -> ::HRESULT,
    fn GetValueSize(&mut self, index: ::UINT32) -> ::UINT32,
    fn GetSubProperties(
        &mut self, index: ::UINT32, subProperties: *mut *mut ::ID2D1Properties
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1StrokeStyle1(ID2D1StrokeStyle1Vtbl): ID2D1StrokeStyle(ID2D1StrokeStyleVtbl) {
    fn GetStrokeTransformType(&mut self) -> ::D2D1_STROKE_TRANSFORM_TYPE
});

pub type D2D1_MATRIX_4X3_F = ::D2D_MATRIX_4X3_F;
pub type D2D1_MATRIX_4X4_F = ::D2D_MATRIX_4X4_F;
pub type D2D1_MATRIX_5X4_F = ::D2D_MATRIX_5X4_F;
pub type D2D1_POINT_2L = ::D2D_POINT_2L;
pub type D2D1_RECT_L = ::D2D_RECT_L;
pub type D2D1_VECTOR_2F = ::D2D_VECTOR_2F;
pub type D2D1_VECTOR_3F = ::D2D_VECTOR_3F;
pub type D2D1_VECTOR_4F = ::D2D_VECTOR_4F;
pub type PD2D1_EFFECT_FACTORY = extern "system" fn (effectImpl: *mut *mut ::IUnknown) -> ::HRESULT;
