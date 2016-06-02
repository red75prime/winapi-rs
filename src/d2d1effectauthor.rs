// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of d2d1effectauthor.h

ENUM!{ enum D2D1_BLEND {
    D2D1_BLEND_ZERO = 1,
    D2D1_BLEND_ONE = 2,
    D2D1_BLEND_SRC_COLOR = 3,
    D2D1_BLEND_INV_SRC_COLOR = 4,
    D2D1_BLEND_SRC_ALPHA = 5,
    D2D1_BLEND_INV_SRC_ALPHA = 6,
    D2D1_BLEND_DEST_ALPHA = 7,
    D2D1_BLEND_INV_DEST_ALPHA = 8,
    D2D1_BLEND_DEST_COLOR = 9,
    D2D1_BLEND_INV_DEST_COLOR = 10,
    D2D1_BLEND_SRC_ALPHA_SAT = 11,
    D2D1_BLEND_BLEND_FACTOR = 14,
    D2D1_BLEND_INV_BLEND_FACTOR = 15,
    D2D1_BLEND_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_BLEND_OPERATION {
    D2D1_BLEND_OPERATION_ADD = 1,
    D2D1_BLEND_OPERATION_SUBTRACT = 2,
    D2D1_BLEND_OPERATION_REV_SUBTRACT = 3,
    D2D1_BLEND_OPERATION_MIN = 4,
    D2D1_BLEND_OPERATION_MAX = 5,
    D2D1_BLEND_OPERATION_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_CHANGE_TYPE {
    D2D1_CHANGE_TYPE_NONE = 0,
    D2D1_CHANGE_TYPE_PROPERTIES = 1,
    D2D1_CHANGE_TYPE_CONTEXT = 2,
    D2D1_CHANGE_TYPE_GRAPH = 3,
    D2D1_CHANGE_TYPE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_CHANNEL_DEPTH {
    D2D1_CHANNEL_DEPTH_DEFAULT = 0,
    D2D1_CHANNEL_DEPTH_1 = 1,
    D2D1_CHANNEL_DEPTH_4 = 4,
    D2D1_CHANNEL_DEPTH_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_FEATURE {
    D2D1_FEATURE_DOUBLES = 0,
    D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS = 1,
    D2D1_FEATURE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_FILTER {
    D2D1_FILTER_MIN_MAG_MIP_POINT = 0,
    D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR = 1,
    D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT = 4,
    D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR = 5,
    D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT = 16,
    D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 17,
    D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT = 20,
    D2D1_FILTER_MIN_MAG_MIP_LINEAR = 21,
    D2D1_FILTER_ANISOTROPIC = 85,
    D2D1_FILTER_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_PIXEL_OPTIONS {
    D2D1_PIXEL_OPTIONS_NONE = 0,
    D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING = 1,
    D2D1_PIXEL_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_VERTEX_OPTIONS {
    D2D1_VERTEX_OPTIONS_NONE = 0,
    D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR = 1,
    D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER = 2,
    D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP = 4,
    D2D1_VERTEX_OPTIONS_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum D2D1_VERTEX_USAGE {
    D2D1_VERTEX_USAGE_STATIC = 0,
    D2D1_VERTEX_USAGE_DYNAMIC = 1,
    D2D1_VERTEX_USAGE_FORCE_DWORD = 0xFFFFFFFF,
}}

STRUCT!{struct D2D1_BLEND_DESCRIPTION {
    sourceBlend: ::D2D1_BLEND,
    destinationBlend: ::D2D1_BLEND,
    blendOperation: ::D2D1_BLEND_OPERATION,
    sourceBlendAlpha: ::D2D1_BLEND,
    destinationBlendAlpha: ::D2D1_BLEND,
    blendOperationAlpha: ::D2D1_BLEND_OPERATION,
    blendFactor: [::FLOAT; 4],
}}

STRUCT!{struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    shaderBufferWithInputSignature: *const ::BYTE,
    shaderBufferSize: ::UINT32,
    inputElements: *const ::D2D1_INPUT_ELEMENT_DESC,
    elementCount: ::UINT32,
    stride: ::UINT32,
}}

STRUCT!{struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: ::BOOL,
}}

STRUCT!{struct D2D1_FEATURE_DATA_DOUBLES {
    doublePrecisionFloatShaderOps: ::BOOL,
}}

STRUCT!{struct D2D1_INPUT_DESCRIPTION {
    filter: ::D2D1_FILTER,
    levelOfDetailCount: ::UINT32,
}}

STRUCT!{struct D2D1_INPUT_ELEMENT_DESC {
    semanticName: ::PCSTR,
    semanticIndex: ::UINT32,
    format: ::DXGI_FORMAT,
    inputSlot: ::UINT32,
    alignedByteOffset: ::UINT32,
}}

STRUCT!{nodebug struct D2D1_PROPERTY_BINDING {
    propertyName: ::PCWSTR,
    setFunction: ::PD2D1_PROPERTY_SET_FUNCTION,
    getFunction: ::PD2D1_PROPERTY_GET_FUNCTION,
}}

STRUCT!{struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    extents: *const ::UINT32,
    dimensions: ::UINT32,
    bufferPrecision: ::D2D1_BUFFER_PRECISION,
    channelDepth: ::D2D1_CHANNEL_DEPTH,
    filter: ::D2D1_FILTER,
    extendModes: *const ::D2D1_EXTEND_MODE,
}}

STRUCT!{struct D2D1_VERTEX_BUFFER_PROPERTIES {
    inputCount: ::UINT32,
    usage: ::D2D1_VERTEX_USAGE,
    data: *const ::BYTE,
    byteWidth: ::UINT32,
}}

STRUCT!{struct D2D1_VERTEX_RANGE {
    startVertex: ::UINT32,
    vertexCount: ::UINT32,
}}

RIDL!(
interface ID2D1AnalysisTransform(ID2D1AnalysisTransformVtbl): IUnknown(IUnknownVtbl) {
    fn ProcessAnalysisResults(
        &mut self, analysisData: *const ::BYTE, analysisDataCount: ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1BlendTransform(ID2D1BlendTransformVtbl): ID2D1ConcreteTransform(ID2D1ConcreteTransformVtbl) {
    fn SetDescription(
        &mut self, description: *const ::D2D1_BLEND_DESCRIPTION
    ) -> (),
    fn GetDescription(
        &mut self, description: *mut ::D2D1_BLEND_DESCRIPTION
    ) -> ()
});

RIDL!(
interface ID2D1BorderTransform(ID2D1BorderTransformVtbl): ID2D1ConcreteTransform(ID2D1ConcreteTransformVtbl) {
    fn SetExtendModeX(&mut self, extendMode: ::D2D1_EXTEND_MODE) -> (),
    fn SetExtendModeY(&mut self, extendMode: ::D2D1_EXTEND_MODE) -> (),
    fn GetExtendModeX(&mut self) -> ::D2D1_EXTEND_MODE,
    fn GetExtendModeY(&mut self) -> ::D2D1_EXTEND_MODE
});

RIDL!(
interface ID2D1BoundsAdjustmentTransform(ID2D1BoundsAdjustmentTransformVtbl): ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn SetOutputBounds(&mut self, outputBounds: *const ::D2D1_RECT_L) -> (),
    fn GetOutputBounds(&mut self, outputBounds: *mut ::D2D1_RECT_L) -> ()
});

RIDL!(
interface ID2D1ComputeInfo(ID2D1ComputeInfoVtbl): ID2D1RenderInfo(ID2D1RenderInfoVtbl) {
    fn SetComputeShaderConstantBuffer(
        &mut self, buffer: *const ::BYTE, bufferCount: ::UINT32
    ) -> ::HRESULT,
    fn SetComputeShader(&mut self, shaderId: *const ::GUID) -> ::HRESULT,
    fn SetResourceTexture(
        &mut self, textureIndex: ::UINT32, resourceTexture: *mut ::ID2D1ResourceTexture
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1ComputeTransform(ID2D1ComputeTransformVtbl): ID2D1Transform(ID2D1TransformVtbl) {
    fn SetComputeInfo(
        &mut self, computeInfo: *mut ::ID2D1ComputeInfo
    ) -> ::HRESULT,
    fn CalculateThreadgroups(
        &mut self, outputRect: *const ::D2D1_RECT_L, dimensionX: *mut ::UINT32,
        dimensionY: *mut ::UINT32, dimensionZ: *mut ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1ConcreteTransform(ID2D1ConcreteTransformVtbl): ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn SetOutputBuffer(
        &mut self, bufferPrecision: ::D2D1_BUFFER_PRECISION, channelDepth: ::D2D1_CHANNEL_DEPTH
    ) -> ::HRESULT,
    fn SetCached(&mut self, isCached: ::BOOL) -> ()
});

RIDL!(
interface ID2D1DrawInfo(ID2D1DrawInfoVtbl): ID2D1RenderInfo(ID2D1RenderInfoVtbl) {
    fn SetPixelShaderConstantBuffer(
        &mut self, buffer: *const ::BYTE, bufferCount: ::UINT32
    ) -> ::HRESULT,
    fn SetResourceTexture(
        &mut self, textureIndex: ::UINT32, resourceTexture: *mut ::ID2D1ResourceTexture
    ) -> ::HRESULT,
    fn SetVertexShaderConstantBuffer(
        &mut self, buffer: *const ::BYTE, bufferCount: ::UINT32
    ) -> ::HRESULT,
    fn SetPixelShader(
        &mut self, shaderId: *const ::GUID, pixelOptions: ::D2D1_PIXEL_OPTIONS
    ) -> ::HRESULT,
    fn SetVertexProcessing(
        &mut self, vertexBuffer: *mut ::ID2D1VertexBuffer, vertexOptions: ::D2D1_VERTEX_OPTIONS,
        blendDescription: *const ::D2D1_BLEND_DESCRIPTION, vertexRange: *const ::D2D1_VERTEX_RANGE,
        vertexShader: *const ::GUID
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1DrawTransform(ID2D1DrawTransformVtbl): ID2D1Transform(ID2D1TransformVtbl) {
    fn SetDrawInfo(&mut self, drawInfo: *mut ::ID2D1DrawInfo) -> ::HRESULT
});

RIDL!(
interface ID2D1EffectContext(ID2D1EffectContextVtbl): IUnknown(IUnknownVtbl) {
    fn GetDpi(&mut self, dpiX: *mut ::FLOAT, dpiY: *mut ::FLOAT) -> (),
    fn CreateEffect(
        &mut self, effectId: *const ::IID, effect: *mut *mut ::ID2D1Effect
    ) -> ::HRESULT,
    fn GetMaximumSupportedFeatureLevel(
        &mut self, featureLevels: *const ::D3D_FEATURE_LEVEL, featureLevelsCount: ::UINT32,
        maximumSupportedFeatureLevel: *mut ::D3D_FEATURE_LEVEL
    ) -> ::HRESULT,
    fn CreateTransformNodeFromEffect(
        &mut self, effect: *mut ::ID2D1Effect, transformNode: *mut *mut ::ID2D1TransformNode
    ) -> ::HRESULT,
    fn CreateBlendTransform(
        &mut self, numInputs: ::UINT32, blendDescription: *const ::D2D1_BLEND_DESCRIPTION,
        transform: *mut *mut ::ID2D1BlendTransform
    ) -> ::HRESULT,
    fn CreateBorderTransform(
        &mut self, extendModeX: ::D2D1_EXTEND_MODE, extendModeY: ::D2D1_EXTEND_MODE,
        transform: *mut *mut ::ID2D1BorderTransform
    ) -> ::HRESULT,
    fn CreateOffsetTransform(
        &mut self, offset: ::D2D1_POINT_2L, transform: *mut *mut ::ID2D1OffsetTransform
    ) -> ::HRESULT,
    fn CreateBoundsAdjustmentTransform(
        &mut self, outputRectangle: *const ::D2D1_RECT_L,
        transform: *mut *mut ::ID2D1BoundsAdjustmentTransform
    ) -> ::HRESULT,
    fn LoadPixelShader(
        &mut self, shaderId: *const ::GUID, shaderBuffer: *const ::BYTE,
        shaderBufferCount: ::UINT32
    ) -> ::HRESULT,
    fn LoadVertexShader(
        &mut self, resourceId: *const ::GUID, shaderBuffer: *const ::BYTE,
        shaderBufferCount: ::UINT32
    ) -> ::HRESULT,
    fn LoadComputeShader(
        &mut self, resourceId: *const ::GUID, shaderBuffer: *const ::BYTE,
        shaderBufferCount: ::UINT32
    ) -> ::HRESULT,
    fn IsShaderLoaded(&mut self, shaderId: *const ::GUID) -> ::BOOL,
    fn CreateResourceTexture(
        &mut self, resourceId: *const ::GUID,
        resourceTextureProperties: *const ::D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const ::BYTE,
        strides: *const ::UINT32, dataSize: ::UINT32,
        resourceTexture: *mut *mut ::ID2D1ResourceTexture
    ) -> ::HRESULT,
    fn FindResourceTexture(
        &mut self, resourceId: *const ::GUID, resourceTexture: *mut *mut ::ID2D1ResourceTexture
    ) -> ::HRESULT,
    fn CreateVertexBuffer(
        &mut self, vertexBufferProperties: *const ::D2D1_VERTEX_BUFFER_PROPERTIES,
        resourceId: *const ::GUID,
        customVertexBufferProperties: *const ::D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES,
        buffer: *mut *mut ::ID2D1VertexBuffer
    ) -> ::HRESULT,
    fn FindVertexBuffer(
        &mut self, resourceId: *const ::GUID, buffer: *mut *mut ::ID2D1VertexBuffer
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
    fn CheckFeatureSupport(
        &mut self, feature: ::D2D1_FEATURE, featureSupportData: *mut ::c_void,
        featureSupportDataSize: ::UINT32
    ) -> ::HRESULT,
    fn IsBufferPrecisionSupported(
        &mut self, bufferPrecision: ::D2D1_BUFFER_PRECISION
    ) -> ::BOOL
});

RIDL!(
interface ID2D1EffectImpl(ID2D1EffectImplVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        &mut self, effectContext: *mut ::ID2D1EffectContext,
        transformGraph: *mut ::ID2D1TransformGraph
    ) -> ::HRESULT,
    fn PrepareForRender(&mut self, changeType: ::D2D1_CHANGE_TYPE) -> ::HRESULT,
    fn SetGraph(
        &mut self, transformGraph: *mut ::ID2D1TransformGraph
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1OffsetTransform(ID2D1OffsetTransformVtbl): ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn SetOffset(&mut self, offset: ::D2D1_POINT_2L) -> (),
    fn GetOffset(
        &mut self, __ret_val: *mut ::D2D1_POINT_2L
    ) -> *mut ::D2D1_POINT_2L
});

RIDL!(
interface ID2D1RenderInfo(ID2D1RenderInfoVtbl): IUnknown(IUnknownVtbl) {
    fn SetInputDescription(
        &mut self, inputIndex: ::UINT32, inputDescription: ::D2D1_INPUT_DESCRIPTION
    ) -> ::HRESULT,
    fn SetOutputBuffer(
        &mut self, bufferPrecision: ::D2D1_BUFFER_PRECISION, channelDepth: ::D2D1_CHANNEL_DEPTH
    ) -> ::HRESULT,
    fn SetCached(&mut self, isCached: ::BOOL) -> (),
    fn SetInstructionCountHint(&mut self, instructionCount: ::UINT32) -> ()
});

RIDL!(
interface ID2D1ResourceTexture(ID2D1ResourceTextureVtbl): IUnknown(IUnknownVtbl) {
    fn Update(
        &mut self, minimumExtents: *const ::UINT32, maximimumExtents: *const ::UINT32,
        strides: *const ::UINT32, dimensions: ::UINT32, data: *const ::BYTE, dataCount: ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1SourceTransform(ID2D1SourceTransformVtbl): ID2D1Transform(ID2D1TransformVtbl) {
    fn SetRenderInfo(
        &mut self, renderInfo: *mut ::ID2D1RenderInfo
    ) -> ::HRESULT,
    fn Draw(
        &mut self, target: *mut ::ID2D1Bitmap1, drawRect: *const ::D2D1_RECT_L,
        targetOrigin: ::D2D1_POINT_2U
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1TransformGraph(ID2D1TransformGraphVtbl): IUnknown(IUnknownVtbl) {
    fn GetInputCount(&mut self) -> ::UINT32,
    fn SetSingleTransformNode(
        &mut self, node: *mut ::ID2D1TransformNode
    ) -> ::HRESULT,
    fn AddNode(&mut self, node: *mut ::ID2D1TransformNode) -> ::HRESULT,
    fn RemoveNode(&mut self, node: *mut ::ID2D1TransformNode) -> ::HRESULT,
    fn SetOutputNode(&mut self, node: *mut ::ID2D1TransformNode) -> ::HRESULT,
    fn ConnectNode(
        &mut self, fromNode: *mut ::ID2D1TransformNode, toNode: *mut ::ID2D1TransformNode,
        toNodeInputIndex: ::UINT32
    ) -> ::HRESULT,
    fn ConnectToEffectInput(
        &mut self, toEffectInputIndex: ::UINT32, node: *mut ::ID2D1TransformNode,
        toNodeInputIndex: ::UINT32
    ) -> ::HRESULT,
    fn Clear(&mut self) -> (),
    fn SetPassthroughGraph(&mut self, effectInputIndex: ::UINT32) -> ::HRESULT
});

RIDL!(
interface ID2D1TransformNode(ID2D1TransformNodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetInputCount(&mut self) -> ::UINT32
});

RIDL!(
interface ID2D1Transform(ID2D1TransformVtbl): ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn MapOutputRectToInputRects(
        &mut self, outputRect: *const ::D2D1_RECT_L, inputRects: *mut ::D2D1_RECT_L,
        inputRectsCount: ::UINT32
    ) -> ::HRESULT,
    fn MapInputRectsToOutputRect(
        &mut self, inputRects: *const ::D2D1_RECT_L, inputOpaqueSubRects: *const ::D2D1_RECT_L,
        inputRectCount: ::UINT32, outputRect: *mut ::D2D1_RECT_L,
        outputOpaqueSubRect: *mut ::D2D1_RECT_L
    ) -> ::HRESULT,
    fn MapInvalidRect(
        &mut self, inputIndex: ::UINT32, invalidInputRect: ::D2D1_RECT_L,
        invalidOutputRect: *mut ::D2D1_RECT_L
    ) -> ::HRESULT
});

RIDL!(
interface ID2D1VertexBuffer(ID2D1VertexBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Map(
        &mut self, data: *mut *mut ::BYTE, bufferSize: ::UINT32
    ) -> ::HRESULT,
    fn Unmap(&mut self) -> ::HRESULT
});

pub type PD2D1_PROPERTY_GET_FUNCTION = extern "system" fn (effect: *const ::IUnknown, data: *mut ::BYTE, dataSize: ::UINT32, actualSize: *mut ::UINT32) -> ::HRESULT;
pub type PD2D1_PROPERTY_SET_FUNCTION = extern "system" fn (effect: *mut ::IUnknown, data: *const ::BYTE, dataSize: ::UINT32) -> ::HRESULT;
pub const D2D1_APPEND_ALIGNED_ELEMENT: ::UINT = 0xffffffff;
