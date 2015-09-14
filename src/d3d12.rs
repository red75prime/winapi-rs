// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of d3d12.h

ENUM!{ enum D3D12_BLEND {
    D3D12_BLEND_ZERO = 1,
    D3D12_BLEND_ONE = 2,
    D3D12_BLEND_SRC_COLOR = 3,
    D3D12_BLEND_INV_SRC_COLOR = 4,
    D3D12_BLEND_SRC_ALPHA = 5,
    D3D12_BLEND_INV_SRC_ALPHA = 6,
    D3D12_BLEND_DEST_ALPHA = 7,
    D3D12_BLEND_INV_DEST_ALPHA = 8,
    D3D12_BLEND_DEST_COLOR = 9,
    D3D12_BLEND_INV_DEST_COLOR = 10,
    D3D12_BLEND_SRC_ALPHA_SAT = 11,
    D3D12_BLEND_BLEND_FACTOR = 14,
    D3D12_BLEND_INV_BLEND_FACTOR = 15,
    D3D12_BLEND_SRC1_COLOR = 16,
    D3D12_BLEND_INV_SRC1_COLOR = 17,
    D3D12_BLEND_SRC1_ALPHA = 18,
    D3D12_BLEND_INV_SRC1_ALPHA = 19,
}}

ENUM!{ enum D3D12_BLEND_OP {
    D3D12_BLEND_OP_ADD = 1,
    D3D12_BLEND_OP_SUBTRACT = 2,
    D3D12_BLEND_OP_REV_SUBTRACT = 3,
    D3D12_BLEND_OP_MIN = 4,
    D3D12_BLEND_OP_MAX = 5,
}}

FLAGS!{ enum D3D12_BUFFER_SRV_FLAGS {
    D3D12_BUFFER_SRV_FLAG_NONE = 0x0,
    D3D12_BUFFER_SRV_FLAG_RAW = 0x1,
}}

FLAGS!{ enum D3D12_BUFFER_UAV_FLAGS {
    D3D12_BUFFER_UAV_FLAG_NONE = 0x0,
    D3D12_BUFFER_UAV_FLAG_RAW = 0x1,
}}

FLAGS!{ enum D3D12_CLEAR_FLAGS {
    D3D12_CLEAR_FLAG_DEPTH = 0x1,
    D3D12_CLEAR_FLAG_STENCIL = 0x2,
}}

FLAGS!{ enum D3D12_COLOR_WRITE_ENABLE {
    D3D12_COLOR_WRITE_ENABLE_RED = 0x1,
    D3D12_COLOR_WRITE_ENABLE_GREEN = 0x2,
    D3D12_COLOR_WRITE_ENABLE_BLUE = 0x4,
    D3D12_COLOR_WRITE_ENABLE_ALPHA = 0x8,
    D3D12_COLOR_WRITE_ENABLE_ALL = 0xF,
}}

ENUM!{ enum D3D12_COMMAND_LIST_TYPE {
    D3D12_COMMAND_LIST_TYPE_DIRECT = 0,
    D3D12_COMMAND_LIST_TYPE_BUNDLE = 1,
    D3D12_COMMAND_LIST_TYPE_COMPUTE = 2,
    D3D12_COMMAND_LIST_TYPE_COPY = 3,
}}

FLAGS!{ enum D3D12_COMMAND_QUEUE_FLAGS {
    D3D12_COMMAND_QUEUE_FLAG_NONE = 0x0,
    D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT = 0x1,
}}

ENUM!{ enum D3D12_COMMAND_QUEUE_PRIORITY {
    D3D12_COMMAND_QUEUE_PRIORITY_NORMAL = 0,
    D3D12_COMMAND_QUEUE_PRIORITY_HIGH = 100,
}}

ENUM!{ enum D3D12_COMPARISON_FUNC {
    D3D12_COMPARISON_FUNC_NEVER = 1,
    D3D12_COMPARISON_FUNC_LESS = 2,
    D3D12_COMPARISON_FUNC_EQUAL = 3,
    D3D12_COMPARISON_FUNC_LESS_EQUAL = 4,
    D3D12_COMPARISON_FUNC_GREATER = 5,
    D3D12_COMPARISON_FUNC_NOT_EQUAL = 6,
    D3D12_COMPARISON_FUNC_GREATER_EQUAL = 7,
    D3D12_COMPARISON_FUNC_ALWAYS = 8,
}}

ENUM!{ enum D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF = 0,
    D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON = 1,
}}

ENUM!{ enum D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED = 0,
    D3D12_CONSERVATIVE_RASTERIZATION_TIER_1 = 1,
    D3D12_CONSERVATIVE_RASTERIZATION_TIER_2 = 2,
    D3D12_CONSERVATIVE_RASTERIZATION_TIER_3 = 3,
}}

ENUM!{ enum D3D12_CPU_PAGE_PROPERTY {
    D3D12_CPU_PAGE_PROPERTY_UNKNOWN = 0,
    D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE = 1,
    D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE = 2,
    D3D12_CPU_PAGE_PROPERTY_WRITE_BACK = 3,
}}

ENUM!{ enum D3D12_CROSS_NODE_SHARING_TIER {
    D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED = 0,
    D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED = 1,
    D3D12_CROSS_NODE_SHARING_TIER_1 = 2,
    D3D12_CROSS_NODE_SHARING_TIER_2 = 3,
}}

ENUM!{ enum D3D12_CULL_MODE {
    D3D12_CULL_MODE_NONE = 1,
    D3D12_CULL_MODE_FRONT = 2,
    D3D12_CULL_MODE_BACK = 3,
}}

ENUM!{ enum D3D12_DEPTH_WRITE_MASK {
    D3D12_DEPTH_WRITE_MASK_ZERO = 0,
    D3D12_DEPTH_WRITE_MASK_ALL = 1,
}}

FLAGS!{ enum D3D12_DESCRIPTOR_HEAP_FLAGS {
    D3D12_DESCRIPTOR_HEAP_FLAG_NONE = 0x0,
    D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE = 0x1,
}}

ENUM!{ enum D3D12_DESCRIPTOR_HEAP_TYPE {
    D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV = 0,
    D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER = 1,
    D3D12_DESCRIPTOR_HEAP_TYPE_RTV = 2,
    D3D12_DESCRIPTOR_HEAP_TYPE_DSV = 3,
    D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPES = 4,
}}

ENUM!{ enum D3D12_DESCRIPTOR_RANGE_TYPE {
    D3D12_DESCRIPTOR_RANGE_TYPE_SRV = 0,
    D3D12_DESCRIPTOR_RANGE_TYPE_UAV = 1,
    D3D12_DESCRIPTOR_RANGE_TYPE_CBV = 2,
    D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER = 3,
}}

ENUM!{ enum D3D12_DSV_DIMENSION {
    D3D12_DSV_DIMENSION_UNKNOWN = 0,
    D3D12_DSV_DIMENSION_TEXTURE1D = 1,
    D3D12_DSV_DIMENSION_TEXTURE1DARRAY = 2,
    D3D12_DSV_DIMENSION_TEXTURE2D = 3,
    D3D12_DSV_DIMENSION_TEXTURE2DARRAY = 4,
    D3D12_DSV_DIMENSION_TEXTURE2DMS = 5,
    D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY = 6,
}}

FLAGS!{ enum D3D12_DSV_FLAGS {
    D3D12_DSV_FLAG_NONE = 0x0,
    D3D12_DSV_FLAG_READ_ONLY_DEPTH = 0x1,
    D3D12_DSV_FLAG_READ_ONLY_STENCIL = 0x2,
}}

ENUM!{ enum D3D12_FEATURE {
    D3D12_FEATURE_D3D12_OPTIONS = 0,
    D3D12_FEATURE_ARCHITECTURE = 1,
    D3D12_FEATURE_FEATURE_LEVELS = 2,
    D3D12_FEATURE_FORMAT_SUPPORT = 3,
    D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS = 4,
    D3D12_FEATURE_FORMAT_INFO = 5,
    D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT = 6,
}}

FLAGS!{ enum D3D12_FENCE_FLAGS {
    D3D12_FENCE_FLAG_NONE = 0x0,
    D3D12_FENCE_FLAG_SHARED = 0x1,
    D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER = 0x2,
}}

ENUM!{ enum D3D12_FILL_MODE {
    D3D12_FILL_MODE_WIREFRAME = 2,
    D3D12_FILL_MODE_SOLID = 3,
}}

ENUM!{ enum D3D12_FILTER {
    D3D12_FILTER_MIN_MAG_MIP_POINT = 0,
    D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR = 1,
    D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT = 4,
    D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR = 5,
    D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT = 16,
    D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 17,
    D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT = 20,
    D3D12_FILTER_MIN_MAG_MIP_LINEAR = 21,
    D3D12_FILTER_ANISOTROPIC = 85,
    D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT = 128,
    D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR = 129,
    D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT = 132,
    D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR = 133,
    D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT = 144,
    D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 145,
    D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT = 148,
    D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR = 149,
    D3D12_FILTER_COMPARISON_ANISOTROPIC = 213,
    D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT = 256,
    D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR = 257,
    D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT = 260,
    D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR = 261,
    D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT = 272,
    D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 273,
    D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT = 276,
    D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR = 277,
    D3D12_FILTER_MINIMUM_ANISOTROPIC = 341,
    D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT = 384,
    D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR = 385,
    D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT = 388,
    D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR = 389,
    D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT = 400,
    D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 401,
    D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT = 404,
    D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR = 405,
    D3D12_FILTER_MAXIMUM_ANISOTROPIC = 469,
}}

ENUM!{ enum D3D12_FILTER_REDUCTION_TYPE {
    D3D12_FILTER_REDUCTION_TYPE_STANDARD = 0,
    D3D12_FILTER_REDUCTION_TYPE_COMPARISON = 1,
    D3D12_FILTER_REDUCTION_TYPE_MINIMUM = 2,
    D3D12_FILTER_REDUCTION_TYPE_MAXIMUM = 3,
}}

ENUM!{ enum D3D12_FILTER_TYPE {
    D3D12_FILTER_TYPE_POINT = 0,
    D3D12_FILTER_TYPE_LINEAR = 1,
}}

FLAGS!{ enum D3D12_FORMAT_SUPPORT1 {
    D3D12_FORMAT_SUPPORT1_NONE = 0x0,
    D3D12_FORMAT_SUPPORT1_BUFFER = 0x1,
    D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER = 0x2,
    D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER = 0x4,
    D3D12_FORMAT_SUPPORT1_SO_BUFFER = 0x8,
    D3D12_FORMAT_SUPPORT1_TEXTURE1D = 0x10,
    D3D12_FORMAT_SUPPORT1_TEXTURE2D = 0x20,
    D3D12_FORMAT_SUPPORT1_TEXTURE3D = 0x40,
    D3D12_FORMAT_SUPPORT1_TEXTURECUBE = 0x80,
    D3D12_FORMAT_SUPPORT1_SHADER_LOAD = 0x100,
    D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE = 0x200,
    D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON = 0x400,
    D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_MONO_TEXT = 0x800,
    D3D12_FORMAT_SUPPORT1_MIP = 0x1000,
    D3D12_FORMAT_SUPPORT1_RENDER_TARGET = 0x4000,
    D3D12_FORMAT_SUPPORT1_BLENDABLE = 0x8000,
    D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL = 0x10000,
    D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE = 0x40000,
    D3D12_FORMAT_SUPPORT1_DISPLAY = 0x80000,
    D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT = 0x100000,
    D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET = 0x200000,
    D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD = 0x400000,
    D3D12_FORMAT_SUPPORT1_SHADER_GATHER = 0x800000,
    D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST = 0x1000000,
    D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW = 0x2000000,
    D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON = 0x4000000,
    D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT = 0x8000000,
    D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT = 0x10000000,
    D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT = 0x20000000,
    D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER = 0x40000000,
}}

FLAGS!{ enum D3D12_FORMAT_SUPPORT2 {
    D3D12_FORMAT_SUPPORT2_NONE = 0x0,
    D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD = 0x1,
    D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS = 0x2,
    D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE = 0x4,
    D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE = 0x8,
    D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX = 0x10,
    D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX = 0x20,
    D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD = 0x40,
    D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE = 0x80,
    D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP = 0x100,
    D3D12_FORMAT_SUPPORT2_TILED = 0x200,
    D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY = 0x4000,
}}

ENUM!{ enum D3D12_HEAP_FLAGS {
    D3D12_HEAP_FLAG_NONE = 0,
    D3D12_HEAP_FLAG_SHARED = 1,
    D3D12_HEAP_FLAG_DENY_BUFFERS = 4,
    D3D12_HEAP_FLAG_ALLOW_DISPLAY = 8,
    D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER = 32,
    D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES = 64,
    D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES = 128,
    D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES = 0,
    D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS = 192,
    D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES = 68,
    D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES = 132,
}}

ENUM!{ enum D3D12_HEAP_TYPE {
    D3D12_HEAP_TYPE_DEFAULT = 1,
    D3D12_HEAP_TYPE_UPLOAD = 2,
    D3D12_HEAP_TYPE_READBACK = 3,
    D3D12_HEAP_TYPE_CUSTOM = 4,
}}

ENUM!{ enum D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED = 0,
    D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF = 1,
    D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF = 2,
}}

ENUM!{ enum D3D12_INDIRECT_ARGUMENT_TYPE {
    D3D12_INDIRECT_ARGUMENT_TYPE_DRAW = 0,
    D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED = 1,
    D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH = 2,
    D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW = 3,
    D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW = 4,
    D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT = 5,
    D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW = 6,
    D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW = 7,
    D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW = 8,
}}

ENUM!{ enum D3D12_INPUT_CLASSIFICATION {
    D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA = 0,
    D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA = 1,
}}

ENUM!{ enum D3D12_LOGIC_OP {
    D3D12_LOGIC_OP_CLEAR = 0,
    D3D12_LOGIC_OP_SET = 1,
    D3D12_LOGIC_OP_COPY = 2,
    D3D12_LOGIC_OP_COPY_INVERTED = 3,
    D3D12_LOGIC_OP_NOOP = 4,
    D3D12_LOGIC_OP_INVERT = 5,
    D3D12_LOGIC_OP_AND = 6,
    D3D12_LOGIC_OP_NAND = 7,
    D3D12_LOGIC_OP_OR = 8,
    D3D12_LOGIC_OP_NOR = 9,
    D3D12_LOGIC_OP_XOR = 10,
    D3D12_LOGIC_OP_EQUIV = 11,
    D3D12_LOGIC_OP_AND_REVERSE = 12,
    D3D12_LOGIC_OP_AND_INVERTED = 13,
    D3D12_LOGIC_OP_OR_REVERSE = 14,
    D3D12_LOGIC_OP_OR_INVERTED = 15,
}}

ENUM!{ enum D3D12_MEMORY_POOL {
    D3D12_MEMORY_POOL_UNKNOWN = 0,
    D3D12_MEMORY_POOL_L0 = 1,
    D3D12_MEMORY_POOL_L1 = 2,
}}

FLAGS!{ enum D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_NONE = 0x0,
    D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_TILED_RESOURCE = 0x1,
}}

FLAGS!{ enum D3D12_PIPELINE_STATE_FLAGS {
    D3D12_PIPELINE_STATE_FLAG_NONE = 0x0,
    D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG = 0x1,
}}

ENUM!{ enum D3D12_PREDICATION_OP {
    D3D12_PREDICATION_OP_EQUAL_ZERO = 0,
    D3D12_PREDICATION_OP_NOT_EQUAL_ZERO = 1,
}}

ENUM!{ enum D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED = 0,
    D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT = 1,
    D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE = 2,
    D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE = 3,
    D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH = 4,
}}

ENUM!{ enum D3D12_QUERY_HEAP_TYPE {
    D3D12_QUERY_HEAP_TYPE_OCCLUSION = 0,
    D3D12_QUERY_HEAP_TYPE_TIMESTAMP = 1,
    D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS = 2,
    D3D12_QUERY_HEAP_TYPE_SO_STATISTICS = 3,
}}

ENUM!{ enum D3D12_QUERY_TYPE {
    D3D12_QUERY_TYPE_OCCLUSION = 0,
    D3D12_QUERY_TYPE_BINARY_OCCLUSION = 1,
    D3D12_QUERY_TYPE_TIMESTAMP = 2,
    D3D12_QUERY_TYPE_PIPELINE_STATISTICS = 3,
    D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0 = 4,
    D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1 = 5,
    D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2 = 6,
    D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3 = 7,
}}

FLAGS!{ enum D3D12_RESOURCE_BARRIER_FLAGS {
    D3D12_RESOURCE_BARRIER_FLAG_NONE = 0x0,
    D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY = 0x1,
    D3D12_RESOURCE_BARRIER_FLAG_END_ONLY = 0x2,
}}

ENUM!{ enum D3D12_RESOURCE_BARRIER_TYPE {
    D3D12_RESOURCE_BARRIER_TYPE_TRANSITION = 0,
    D3D12_RESOURCE_BARRIER_TYPE_ALIASING = 1,
    D3D12_RESOURCE_BARRIER_TYPE_UAV = 2,
}}

ENUM!{ enum D3D12_RESOURCE_BINDING_TIER {
    D3D12_RESOURCE_BINDING_TIER_1 = 1,
    D3D12_RESOURCE_BINDING_TIER_2 = 2,
    D3D12_RESOURCE_BINDING_TIER_3 = 3,
}}

ENUM!{ enum D3D12_RESOURCE_DIMENSION {
    D3D12_RESOURCE_DIMENSION_UNKNOWN = 0,
    D3D12_RESOURCE_DIMENSION_BUFFER = 1,
    D3D12_RESOURCE_DIMENSION_TEXTURE1D = 2,
    D3D12_RESOURCE_DIMENSION_TEXTURE2D = 3,
    D3D12_RESOURCE_DIMENSION_TEXTURE3D = 4,
}}

FLAGS!{ enum D3D12_RESOURCE_FLAGS {
    D3D12_RESOURCE_FLAG_NONE = 0x0,
    D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET = 0x1,
    D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL = 0x2,
    D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS = 0x4,
    D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE = 0x8,
    D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER = 0x10,
    D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS = 0x20,
}}

ENUM!{ enum D3D12_RESOURCE_HEAP_TIER {
    D3D12_RESOURCE_HEAP_TIER_1 = 1,
    D3D12_RESOURCE_HEAP_TIER_2 = 2,
}}

ENUM!{ enum D3D12_RESOURCE_STATES {
    D3D12_RESOURCE_STATE_COMMON = 0,
    D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER = 1,
    D3D12_RESOURCE_STATE_INDEX_BUFFER = 2,
    D3D12_RESOURCE_STATE_RENDER_TARGET = 4,
    D3D12_RESOURCE_STATE_UNORDERED_ACCESS = 8,
    D3D12_RESOURCE_STATE_DEPTH_WRITE = 16,
    D3D12_RESOURCE_STATE_DEPTH_READ = 32,
    D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE = 64,
    D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE = 128,
    D3D12_RESOURCE_STATE_STREAM_OUT = 256,
    D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT = 512,
    D3D12_RESOURCE_STATE_COPY_DEST = 1024,
    D3D12_RESOURCE_STATE_COPY_SOURCE = 2048,
    D3D12_RESOURCE_STATE_RESOLVE_DEST = 4096,
    D3D12_RESOURCE_STATE_RESOLVE_SOURCE = 8192,
    D3D12_RESOURCE_STATE_GENERIC_READ = 2755,
    D3D12_RESOURCE_STATE_PRESENT = 0,
    D3D12_RESOURCE_STATE_PREDICATION = 512,
}}

ENUM!{ enum D3D12_ROOT_PARAMETER_TYPE {
    D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE = 0,
    D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS = 1,
    D3D12_ROOT_PARAMETER_TYPE_CBV = 2,
    D3D12_ROOT_PARAMETER_TYPE_SRV = 3,
    D3D12_ROOT_PARAMETER_TYPE_UAV = 4,
}}

FLAGS!{ enum D3D12_ROOT_SIGNATURE_FLAGS {
    D3D12_ROOT_SIGNATURE_FLAG_NONE = 0x0,
    D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT = 0x1,
    D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS = 0x2,
    D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS = 0x4,
    D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS = 0x8,
    D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS = 0x10,
    D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS = 0x20,
    D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT = 0x40,
}}

ENUM!{ enum D3D12_RTV_DIMENSION {
    D3D12_RTV_DIMENSION_UNKNOWN = 0,
    D3D12_RTV_DIMENSION_BUFFER = 1,
    D3D12_RTV_DIMENSION_TEXTURE1D = 2,
    D3D12_RTV_DIMENSION_TEXTURE1DARRAY = 3,
    D3D12_RTV_DIMENSION_TEXTURE2D = 4,
    D3D12_RTV_DIMENSION_TEXTURE2DARRAY = 5,
    D3D12_RTV_DIMENSION_TEXTURE2DMS = 6,
    D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY = 7,
    D3D12_RTV_DIMENSION_TEXTURE3D = 8,
}}

ENUM!{ enum D3D12_SHADER_COMPONENT_MAPPING {
    D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0 = 0,
    D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1 = 1,
    D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2 = 2,
    D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3 = 3,
    D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_0 = 4,
    D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_1 = 5,
}}

ENUM!{ enum D3D12_SHADER_MIN_PRECISION_SUPPORT {
    D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE = 0,
    D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT = 1,
    D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT = 2,
}}

ENUM!{ enum D3D12_SHADER_VISIBILITY {
    D3D12_SHADER_VISIBILITY_ALL = 0,
    D3D12_SHADER_VISIBILITY_VERTEX = 1,
    D3D12_SHADER_VISIBILITY_HULL = 2,
    D3D12_SHADER_VISIBILITY_DOMAIN = 3,
    D3D12_SHADER_VISIBILITY_GEOMETRY = 4,
    D3D12_SHADER_VISIBILITY_PIXEL = 5,
}}

ENUM!{ enum D3D12_SRV_DIMENSION {
    D3D12_SRV_DIMENSION_UNKNOWN = 0,
    D3D12_SRV_DIMENSION_BUFFER = 1,
    D3D12_SRV_DIMENSION_TEXTURE1D = 2,
    D3D12_SRV_DIMENSION_TEXTURE1DARRAY = 3,
    D3D12_SRV_DIMENSION_TEXTURE2D = 4,
    D3D12_SRV_DIMENSION_TEXTURE2DARRAY = 5,
    D3D12_SRV_DIMENSION_TEXTURE2DMS = 6,
    D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY = 7,
    D3D12_SRV_DIMENSION_TEXTURE3D = 8,
    D3D12_SRV_DIMENSION_TEXTURECUBE = 9,
    D3D12_SRV_DIMENSION_TEXTURECUBEARRAY = 10,
}}

ENUM!{ enum D3D12_STATIC_BORDER_COLOR {
    D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK = 0,
    D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK = 1,
    D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE = 2,
}}

ENUM!{ enum D3D12_STENCIL_OP {
    D3D12_STENCIL_OP_KEEP = 1,
    D3D12_STENCIL_OP_ZERO = 2,
    D3D12_STENCIL_OP_REPLACE = 3,
    D3D12_STENCIL_OP_INCR_SAT = 4,
    D3D12_STENCIL_OP_DECR_SAT = 5,
    D3D12_STENCIL_OP_INVERT = 6,
    D3D12_STENCIL_OP_INCR = 7,
    D3D12_STENCIL_OP_DECR = 8,
}}

ENUM!{ enum D3D12_TEXTURE_ADDRESS_MODE {
    D3D12_TEXTURE_ADDRESS_MODE_WRAP = 1,
    D3D12_TEXTURE_ADDRESS_MODE_MIRROR = 2,
    D3D12_TEXTURE_ADDRESS_MODE_CLAMP = 3,
    D3D12_TEXTURE_ADDRESS_MODE_BORDER = 4,
    D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE = 5,
}}

ENUM!{ enum D3D12_TEXTURE_COPY_TYPE {
    D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX = 0,
    D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT = 1,
}}

ENUM!{ enum D3D12_TEXTURE_LAYOUT {
    D3D12_TEXTURE_LAYOUT_UNKNOWN = 0,
    D3D12_TEXTURE_LAYOUT_ROW_MAJOR = 1,
    D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE = 2,
    D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE = 3,
}}

ENUM!{ enum D3D12_TILED_RESOURCES_TIER {
    D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED = 0,
    D3D12_TILED_RESOURCES_TIER_1 = 1,
    D3D12_TILED_RESOURCES_TIER_2 = 2,
    D3D12_TILED_RESOURCES_TIER_3 = 3,
}}

FLAGS!{ enum D3D12_TILE_COPY_FLAGS {
    D3D12_TILE_COPY_FLAG_NONE = 0x0,
    D3D12_TILE_COPY_FLAG_NO_HAZARD = 0x1,
    D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE = 0x2,
    D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER = 0x4,
}}

FLAGS!{ enum D3D12_TILE_MAPPING_FLAGS {
    D3D12_TILE_MAPPING_FLAG_NONE = 0x0,
    D3D12_TILE_MAPPING_FLAG_NO_HAZARD = 0x1,
}}

FLAGS!{ enum D3D12_TILE_RANGE_FLAGS {
    D3D12_TILE_RANGE_FLAG_NONE = 0x0,
    D3D12_TILE_RANGE_FLAG_NULL = 0x1,
    D3D12_TILE_RANGE_FLAG_SKIP = 0x2,
    D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE = 0x4,
}}

ENUM!{ enum D3D12_UAV_DIMENSION {
    D3D12_UAV_DIMENSION_UNKNOWN = 0,
    D3D12_UAV_DIMENSION_BUFFER = 1,
    D3D12_UAV_DIMENSION_TEXTURE1D = 2,
    D3D12_UAV_DIMENSION_TEXTURE1DARRAY = 3,
    D3D12_UAV_DIMENSION_TEXTURE2D = 4,
    D3D12_UAV_DIMENSION_TEXTURE2DARRAY = 5,
    D3D12_UAV_DIMENSION_TEXTURE3D = 8,
}}

ENUM!{ enum D3D_ROOT_SIGNATURE_VERSION {
    D3D_ROOT_SIGNATURE_VERSION_1 = 1,
}}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_BLEND_DESC {
    pub AlphaToCoverageEnable: ::BOOL,
    pub IndependentBlendEnable: ::BOOL,
    pub RenderTarget: [::D3D12_RENDER_TARGET_BLEND_DESC; 8],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_BOX {
    pub left: ::UINT,
    pub top: ::UINT,
    pub front: ::UINT,
    pub right: ::UINT,
    pub bottom: ::UINT,
    pub back: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_BUFFER_RTV {
    pub FirstElement: ::UINT64,
    pub NumElements: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_BUFFER_SRV {
    pub FirstElement: ::UINT64,
    pub NumElements: ::UINT,
    pub StructureByteStride: ::UINT,
    pub Flags: ::D3D12_BUFFER_SRV_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_BUFFER_UAV {
    pub FirstElement: ::UINT64,
    pub NumElements: ::UINT,
    pub StructureByteStride: ::UINT,
    pub CounterOffsetInBytes: ::UINT64,
    pub Flags: ::D3D12_BUFFER_UAV_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_CACHED_PIPELINE_STATE {
    pub pCachedBlob: *const ::c_void,
    pub CachedBlobSizeInBytes: ::SIZE_T,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_CLEAR_VALUE {
    pub Format: ::DXGI_FORMAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_COMMAND_QUEUE_DESC {
    pub Type: ::D3D12_COMMAND_LIST_TYPE,
    pub Priority: ::INT,
    pub Flags: ::D3D12_COMMAND_QUEUE_FLAGS,
    pub NodeMask: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_COMMAND_SIGNATURE_DESC {
    pub ByteStride: ::UINT,
    pub NumArgumentDescs: ::UINT,
    pub pArgumentDescs: *const ::D3D12_INDIRECT_ARGUMENT_DESC,
    pub NodeMask: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_COMPUTE_PIPELINE_STATE_DESC {
    pub pRootSignature: *mut ::ID3D12RootSignature,
    pub CS: ::D3D12_SHADER_BYTECODE,
    pub NodeMask: ::UINT,
    pub CachedPSO: ::D3D12_CACHED_PIPELINE_STATE,
    pub Flags: ::D3D12_PIPELINE_STATE_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_CONSTANT_BUFFER_VIEW_DESC {
    pub BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_CPU_DESCRIPTOR_HANDLE {
    pub ptr: ::SIZE_T,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: ::D3D12_STENCIL_OP,
    pub StencilDepthFailOp: ::D3D12_STENCIL_OP,
    pub StencilPassOp: ::D3D12_STENCIL_OP,
    pub StencilFunc: ::D3D12_COMPARISON_FUNC,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DEPTH_STENCIL_DESC {
    pub DepthEnable: ::BOOL,
    pub DepthWriteMask: ::D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: ::D3D12_COMPARISON_FUNC,
    pub StencilEnable: ::BOOL,
    pub StencilReadMask: ::UINT8,
    pub StencilWriteMask: ::UINT8,
    pub FrontFace: ::D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: ::D3D12_DEPTH_STENCILOP_DESC,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DEPTH_STENCIL_VALUE {
    pub Depth: ::FLOAT,
    pub Stencil: ::UINT8,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DEPTH_STENCIL_VIEW_DESC {
    pub Format: ::DXGI_FORMAT,
    pub ViewDimension: ::D3D12_DSV_DIMENSION,
    pub Flags: ::D3D12_DSV_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DESCRIPTOR_HEAP_DESC {
    pub Type: ::D3D12_DESCRIPTOR_HEAP_TYPE,
    pub NumDescriptors: ::UINT,
    pub Flags: ::D3D12_DESCRIPTOR_HEAP_FLAGS,
    pub NodeMask: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DESCRIPTOR_RANGE {
    pub RangeType: ::D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: ::UINT,
    pub BaseShaderRegister: ::UINT,
    pub RegisterSpace: ::UINT,
    pub OffsetInDescriptorsFromTableStart: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DISCARD_REGION {
    pub NumRects: ::UINT,
    pub pRects: *const ::D3D12_RECT,
    pub FirstSubresource: ::UINT,
    pub NumSubresources: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DISPATCH_ARGUMENTS {
    pub ThreadGroupCountX: ::UINT,
    pub ThreadGroupCountY: ::UINT,
    pub ThreadGroupCountZ: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DRAW_ARGUMENTS {
    pub VertexCountPerInstance: ::UINT,
    pub InstanceCount: ::UINT,
    pub StartVertexLocation: ::UINT,
    pub StartInstanceLocation: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_DRAW_INDEXED_ARGUMENTS {
    pub IndexCountPerInstance: ::UINT,
    pub InstanceCount: ::UINT,
    pub StartIndexLocation: ::UINT,
    pub BaseVertexLocation: ::INT,
    pub StartInstanceLocation: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE {
    pub NodeIndex: ::UINT,
    pub TileBasedRenderer: ::BOOL,
    pub UMA: ::BOOL,
    pub CacheCoherentUMA: ::BOOL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS {
    pub DoublePrecisionFloatShaderOps: ::BOOL,
    pub OutputMergerLogicOp: ::BOOL,
    pub MinPrecisionSupport: ::D3D12_SHADER_MIN_PRECISION_SUPPORT,
    pub TiledResourcesTier: ::D3D12_TILED_RESOURCES_TIER,
    pub ResourceBindingTier: ::D3D12_RESOURCE_BINDING_TIER,
    pub PSSpecifiedStencilRefSupported: ::BOOL,
    pub TypedUAVLoadAdditionalFormats: ::BOOL,
    pub ROVsSupported: ::BOOL,
    pub ConservativeRasterizationTier: ::D3D12_CONSERVATIVE_RASTERIZATION_TIER,
    pub MaxGPUVirtualAddressBitsPerResource: ::UINT,
    pub StandardSwizzle64KBSupported: ::BOOL,
    pub CrossNodeSharingTier: ::D3D12_CROSS_NODE_SHARING_TIER,
    pub CrossAdapterRowMajorTextureSupported: ::BOOL,
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation: ::BOOL,
    pub ResourceHeapTier: ::D3D12_RESOURCE_HEAP_TIER,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_FEATURE_LEVELS {
    pub NumFeatureLevels: ::UINT,
    pub pFeatureLevelsRequested: *const ::D3D_FEATURE_LEVEL,
    pub MaxSupportedFeatureLevel: ::D3D_FEATURE_LEVEL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_FORMAT_INFO {
    pub Format: ::DXGI_FORMAT,
    pub PlaneCount: ::UINT8,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    pub Format: ::DXGI_FORMAT,
    pub Support1: ::D3D12_FORMAT_SUPPORT1,
    pub Support2: ::D3D12_FORMAT_SUPPORT2,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: ::UINT,
    pub MaxGPUVirtualAddressBitsPerProcess: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    pub Format: ::DXGI_FORMAT,
    pub SampleCount: ::UINT,
    pub Flags: ::D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS,
    pub NumQualityLevels: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_GPU_DESCRIPTOR_HANDLE {
    pub ptr: ::UINT64,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    pub pRootSignature: *mut ::ID3D12RootSignature,
    pub VS: ::D3D12_SHADER_BYTECODE,
    pub PS: ::D3D12_SHADER_BYTECODE,
    pub DS: ::D3D12_SHADER_BYTECODE,
    pub HS: ::D3D12_SHADER_BYTECODE,
    pub GS: ::D3D12_SHADER_BYTECODE,
    pub StreamOutput: ::D3D12_STREAM_OUTPUT_DESC,
    pub BlendState: ::D3D12_BLEND_DESC,
    pub SampleMask: ::UINT,
    pub RasterizerState: ::D3D12_RASTERIZER_DESC,
    pub DepthStencilState: ::D3D12_DEPTH_STENCIL_DESC,
    pub InputLayout: ::D3D12_INPUT_LAYOUT_DESC,
    pub IBStripCutValue: ::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    pub PrimitiveTopologyType: ::D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    pub NumRenderTargets: ::UINT,
    pub RTVFormats: [::DXGI_FORMAT; 8],
    pub DSVFormat: ::DXGI_FORMAT,
    pub SampleDesc: ::DXGI_SAMPLE_DESC,
    pub NodeMask: ::UINT,
    pub CachedPSO: ::D3D12_CACHED_PIPELINE_STATE,
    pub Flags: ::D3D12_PIPELINE_STATE_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_HEAP_DESC {
    pub SizeInBytes: ::UINT64,
    pub Properties: ::D3D12_HEAP_PROPERTIES,
    pub Alignment: ::UINT64,
    pub Flags: ::D3D12_HEAP_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_HEAP_PROPERTIES {
    pub Type: ::D3D12_HEAP_TYPE,
    pub CPUPageProperty: ::D3D12_CPU_PAGE_PROPERTY,
    pub MemoryPoolPreference: ::D3D12_MEMORY_POOL,
    pub CreationNodeMask: ::UINT,
    pub VisibleNodeMask: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_INDEX_BUFFER_VIEW {
    pub BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: ::UINT,
    pub Format: ::DXGI_FORMAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC {
    pub Type: ::D3D12_INDIRECT_ARGUMENT_TYPE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_INPUT_ELEMENT_DESC {
    pub SemanticName: ::LPCSTR,
    pub SemanticIndex: ::UINT,
    pub Format: ::DXGI_FORMAT,
    pub InputSlot: ::UINT,
    pub AlignedByteOffset: ::UINT,
    pub InputSlotClass: ::D3D12_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_INPUT_LAYOUT_DESC {
    pub pInputElementDescs: *const ::D3D12_INPUT_ELEMENT_DESC,
    pub NumElements: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_MEMCPY_DEST {
    pub pData: *mut ::c_void,
    pub RowPitch: ::SIZE_T,
    pub SlicePitch: ::SIZE_T,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_PACKED_MIP_INFO {
    pub NumStandardMips: ::UINT8,
    pub NumPackedMips: ::UINT8,
    pub NumTilesForPackedMips: ::UINT,
    pub StartTileIndexInOverallResource: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    pub Offset: ::UINT64,
    pub Footprint: ::D3D12_SUBRESOURCE_FOOTPRINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: ::UINT64,
    pub IAPrimitives: ::UINT64,
    pub VSInvocations: ::UINT64,
    pub GSInvocations: ::UINT64,
    pub GSPrimitives: ::UINT64,
    pub CInvocations: ::UINT64,
    pub CPrimitives: ::UINT64,
    pub PSInvocations: ::UINT64,
    pub HSInvocations: ::UINT64,
    pub DSInvocations: ::UINT64,
    pub CSInvocations: ::UINT64,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: ::UINT64,
    pub PrimitivesStorageNeeded: ::UINT64,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_QUERY_HEAP_DESC {
    pub Type: ::D3D12_QUERY_HEAP_TYPE,
    pub Count: ::UINT,
    pub NodeMask: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RANGE {
    pub Begin: ::SIZE_T,
    pub End: ::SIZE_T,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RASTERIZER_DESC {
    pub FillMode: ::D3D12_FILL_MODE,
    pub CullMode: ::D3D12_CULL_MODE,
    pub FrontCounterClockwise: ::BOOL,
    pub DepthBias: ::INT,
    pub DepthBiasClamp: ::FLOAT,
    pub SlopeScaledDepthBias: ::FLOAT,
    pub DepthClipEnable: ::BOOL,
    pub MultisampleEnable: ::BOOL,
    pub AntialiasedLineEnable: ::BOOL,
    pub ForcedSampleCount: ::UINT,
    pub ConservativeRaster: ::D3D12_CONSERVATIVE_RASTERIZATION_MODE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: ::BOOL,
    pub LogicOpEnable: ::BOOL,
    pub SrcBlend: ::D3D12_BLEND,
    pub DestBlend: ::D3D12_BLEND,
    pub BlendOp: ::D3D12_BLEND_OP,
    pub SrcBlendAlpha: ::D3D12_BLEND,
    pub DestBlendAlpha: ::D3D12_BLEND,
    pub BlendOpAlpha: ::D3D12_BLEND_OP,
    pub LogicOp: ::D3D12_LOGIC_OP,
    pub RenderTargetWriteMask: ::UINT8,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RENDER_TARGET_VIEW_DESC {
    pub Format: ::DXGI_FORMAT,
    pub ViewDimension: ::D3D12_RTV_DIMENSION,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RESOURCE_ALIASING_BARRIER {
    pub pResourceBefore: *mut ::ID3D12Resource,
    pub pResourceAfter: *mut ::ID3D12Resource,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RESOURCE_ALLOCATION_INFO {
    pub SizeInBytes: ::UINT64,
    pub Alignment: ::UINT64,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RESOURCE_BARRIER {
    pub Type: ::D3D12_RESOURCE_BARRIER_TYPE,
    pub Flags: ::D3D12_RESOURCE_BARRIER_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RESOURCE_DESC {
    pub Dimension: ::D3D12_RESOURCE_DIMENSION,
    pub Alignment: ::UINT64,
    pub Width: ::UINT64,
    pub Height: ::UINT,
    pub DepthOrArraySize: ::UINT16,
    pub MipLevels: ::UINT16,
    pub Format: ::DXGI_FORMAT,
    pub SampleDesc: ::DXGI_SAMPLE_DESC,
    pub Layout: ::D3D12_TEXTURE_LAYOUT,
    pub Flags: ::D3D12_RESOURCE_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RESOURCE_TRANSITION_BARRIER {
    pub pResource: *mut ::ID3D12Resource,
    pub Subresource: ::UINT,
    pub StateBefore: ::D3D12_RESOURCE_STATES,
    pub StateAfter: ::D3D12_RESOURCE_STATES,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: *mut ::ID3D12Resource,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_ROOT_CONSTANTS {
    pub ShaderRegister: ::UINT,
    pub RegisterSpace: ::UINT,
    pub Num32BitValues: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_ROOT_DESCRIPTOR {
    pub ShaderRegister: ::UINT,
    pub RegisterSpace: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE {
    pub NumDescriptorRanges: ::UINT,
    pub pDescriptorRanges: *const ::D3D12_DESCRIPTOR_RANGE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_ROOT_PARAMETER {
    pub ParameterType: ::D3D12_ROOT_PARAMETER_TYPE,
    pub ShaderVisibility: ::D3D12_SHADER_VISIBILITY,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_ROOT_SIGNATURE_DESC {
    pub NumParameters: ::UINT,
    pub pParameters: *const ::D3D12_ROOT_PARAMETER,
    pub NumStaticSamplers: ::UINT,
    pub pStaticSamplers: *const ::D3D12_STATIC_SAMPLER_DESC,
    pub Flags: ::D3D12_ROOT_SIGNATURE_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SAMPLER_DESC {
    pub Filter: ::D3D12_FILTER,
    pub AddressU: ::D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: ::D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: ::D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: ::FLOAT,
    pub MaxAnisotropy: ::UINT,
    pub ComparisonFunc: ::D3D12_COMPARISON_FUNC,
    pub BorderColor: [::FLOAT; 4],
    pub MinLOD: ::FLOAT,
    pub MaxLOD: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SHADER_BYTECODE {
    pub pShaderBytecode: *const ::c_void,
    pub BytecodeLength: ::SIZE_T,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SHADER_RESOURCE_VIEW_DESC {
    pub Format: ::DXGI_FORMAT,
    pub ViewDimension: ::D3D12_SRV_DIMENSION,
    pub Shader4ComponentMapping: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SO_DECLARATION_ENTRY {
    pub Stream: ::UINT,
    pub SemanticName: ::LPCSTR,
    pub SemanticIndex: ::UINT,
    pub StartComponent: ::BYTE,
    pub ComponentCount: ::BYTE,
    pub OutputSlot: ::BYTE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_STATIC_SAMPLER_DESC {
    pub Filter: ::D3D12_FILTER,
    pub AddressU: ::D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: ::D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: ::D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: ::FLOAT,
    pub MaxAnisotropy: ::UINT,
    pub ComparisonFunc: ::D3D12_COMPARISON_FUNC,
    pub BorderColor: ::D3D12_STATIC_BORDER_COLOR,
    pub MinLOD: ::FLOAT,
    pub MaxLOD: ::FLOAT,
    pub ShaderRegister: ::UINT,
    pub RegisterSpace: ::UINT,
    pub ShaderVisibility: ::D3D12_SHADER_VISIBILITY,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    pub BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: ::UINT64,
    pub BufferFilledSizeLocation: ::D3D12_GPU_VIRTUAL_ADDRESS,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_STREAM_OUTPUT_DESC {
    pub pSODeclaration: *const ::D3D12_SO_DECLARATION_ENTRY,
    pub NumEntries: ::UINT,
    pub pBufferStrides: *const ::UINT,
    pub NumStrides: ::UINT,
    pub RasterizedStream: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SUBRESOURCE_DATA {
    pub pData: *const ::c_void,
    pub RowPitch: ::LONG_PTR,
    pub SlicePitch: ::LONG_PTR,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SUBRESOURCE_FOOTPRINT {
    pub Format: ::DXGI_FORMAT,
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Depth: ::UINT,
    pub RowPitch: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SUBRESOURCE_INFO {
    pub Offset: ::UINT64,
    pub RowPitch: ::UINT,
    pub DepthPitch: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_SUBRESOURCE_TILING {
    pub WidthInTiles: ::UINT,
    pub HeightInTiles: ::UINT16,
    pub DepthInTiles: ::UINT16,
    pub StartTileIndexInOverallResource: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_ARRAY_DSV {
    pub MipSlice: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_ARRAY_RTV {
    pub MipSlice: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_ARRAY_UAV {
    pub MipSlice: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_DSV {
    pub MipSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_RTV {
    pub MipSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX1D_UAV {
    pub MipSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_ARRAY_DSV {
    pub MipSlice: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_ARRAY_RTV {
    pub MipSlice: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
    pub PlaneSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
    pub PlaneSlice: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_ARRAY_UAV {
    pub MipSlice: ::UINT,
    pub FirstArraySlice: ::UINT,
    pub ArraySize: ::UINT,
    pub PlaneSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_DSV {
    pub MipSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_RTV {
    pub MipSlice: ::UINT,
    pub PlaneSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub PlaneSlice: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX2D_UAV {
    pub MipSlice: ::UINT,
    pub PlaneSlice: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX3D_RTV {
    pub MipSlice: ::UINT,
    pub FirstWSlice: ::UINT,
    pub WSize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX3D_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEX3D_UAV {
    pub MipSlice: ::UINT,
    pub FirstWSlice: ::UINT,
    pub WSize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub First2DArrayFace: ::UINT,
    pub NumCubes: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEXCUBE_SRV {
    pub MostDetailedMip: ::UINT,
    pub MipLevels: ::UINT,
    pub ResourceMinLODClamp: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TEXTURE_COPY_LOCATION {
    pub pResource: *mut ::ID3D12Resource,
    pub Type: ::D3D12_TEXTURE_COPY_TYPE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TILED_RESOURCE_COORDINATE {
    pub X: ::UINT,
    pub Y: ::UINT,
    pub Z: ::UINT,
    pub Subresource: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TILE_REGION_SIZE {
    pub NumTiles: ::UINT,
    pub UseBox: ::BOOL,
    pub Width: ::UINT,
    pub Height: ::UINT16,
    pub Depth: ::UINT16,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_TILE_SHAPE {
    pub WidthInTexels: ::UINT,
    pub HeightInTexels: ::UINT,
    pub DepthInTexels: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: ::DXGI_FORMAT,
    pub ViewDimension: ::D3D12_UAV_DIMENSION,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_VERTEX_BUFFER_VIEW {
    pub BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: ::UINT,
    pub StrideInBytes: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D3D12_VIEWPORT {
    pub TopLeftX: ::FLOAT,
    pub TopLeftY: ::FLOAT,
    pub Width: ::FLOAT,
    pub Height: ::FLOAT,
    pub MinDepth: ::FLOAT,
    pub MaxDepth: ::FLOAT,
}

RIDL!(
interface ID3D12CommandAllocator(ID3D12CommandAllocatorVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn Reset(&mut self, This: *mut ::ID3D12CommandAllocator) -> ::HRESULT
});

RIDL!(
interface ID3D12CommandList(ID3D12CommandListVtbl): ID3D12DeviceChild(ID3D12DeviceChildVtbl) {
    fn GetType(
        &mut self, This: *mut ::ID3D12CommandList
    ) -> ::D3D12_COMMAND_LIST_TYPE
});

RIDL!(
interface ID3D12CommandQueue(ID3D12CommandQueueVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn UpdateTileMappings(
        &mut self, This: *mut ::ID3D12CommandQueue, pResource: *mut ::ID3D12Resource,
        NumResourceRegions: ::UINT,
        pResourceRegionStartCoordinates: *const ::D3D12_TILED_RESOURCE_COORDINATE,
        pResourceRegionSizes: *const ::D3D12_TILE_REGION_SIZE, pHeap: *mut ::ID3D12Heap,
        NumRanges: ::UINT, pRangeFlags: *const ::D3D12_TILE_RANGE_FLAGS,
        pHeapRangeStartOffsets: *const ::UINT, pRangeTileCounts: *const ::UINT,
        Flags: ::D3D12_TILE_MAPPING_FLAGS
    ) -> (),
    fn CopyTileMappings(
        &mut self, This: *mut ::ID3D12CommandQueue, pDstResource: *mut ::ID3D12Resource,
        pDstRegionStartCoordinate: *const ::D3D12_TILED_RESOURCE_COORDINATE,
        pSrcResource: *mut ::ID3D12Resource,
        pSrcRegionStartCoordinate: *const ::D3D12_TILED_RESOURCE_COORDINATE,
        pRegionSize: *const ::D3D12_TILE_REGION_SIZE, Flags: ::D3D12_TILE_MAPPING_FLAGS
    ) -> (),
    fn ExecuteCommandLists(
        &mut self, This: *mut ::ID3D12CommandQueue, NumCommandLists: ::UINT,
        ppCommandLists: *mut *mut ::ID3D12CommandList
    ) -> (),
    fn SetMarker(
        &mut self, This: *mut ::ID3D12CommandQueue, Metadata: ::UINT, pData: *const ::c_void,
        Size: ::UINT
    ) -> (),
    fn BeginEvent(
        &mut self, This: *mut ::ID3D12CommandQueue, Metadata: ::UINT, pData: *const ::c_void,
        Size: ::UINT
    ) -> (),
    fn EndEvent(&mut self, This: *mut ::ID3D12CommandQueue) -> (),
    fn Signal(
        &mut self, This: *mut ::ID3D12CommandQueue, pFence: *mut ::ID3D12Fence, Value: ::UINT64
    ) -> ::HRESULT,
    fn Wait(
        &mut self, This: *mut ::ID3D12CommandQueue, pFence: *mut ::ID3D12Fence, Value: ::UINT64
    ) -> ::HRESULT,
    fn GetTimestampFrequency(
        &mut self, This: *mut ::ID3D12CommandQueue, pFrequency: *mut ::UINT64
    ) -> ::HRESULT,
    fn GetClockCalibration(
        &mut self, This: *mut ::ID3D12CommandQueue, pGpuTimestamp: *mut ::UINT64,
        pCpuTimestamp: *mut ::UINT64
    ) -> ::HRESULT,
    fn GetDesc(
        &mut self, This: *mut ::ID3D12CommandQueue
    ) -> ::D3D12_COMMAND_QUEUE_DESC
});

RIDL!(
interface ID3D12CommandSignature(ID3D12CommandSignatureVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
});

RIDL!(
interface ID3D12DescriptorHeap(ID3D12DescriptorHeapVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn GetDesc(
        &mut self, This: *mut ::ID3D12DescriptorHeap
    ) -> ::D3D12_DESCRIPTOR_HEAP_DESC,
    fn GetCPUDescriptorHandleForHeapStart(
        &mut self, This: *mut ::ID3D12DescriptorHeap, __ret_val: *mut ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> *mut ::D3D12_CPU_DESCRIPTOR_HANDLE,
    fn GetGPUDescriptorHandleForHeapStart(
        &mut self, This: *mut ::ID3D12DescriptorHeap, __ret_val: *mut ::D3D12_GPU_DESCRIPTOR_HANDLE
    ) -> *mut ::D3D12_GPU_DESCRIPTOR_HANDLE
});

RIDL!(
interface ID3D12DeviceChild(ID3D12DeviceChildVtbl): ID3D12Object(ID3D12ObjectVtbl) {
    fn GetDevice(
        &mut self, This: *mut ::ID3D12DeviceChild, riid: ::REFGUID, ppvDevice: *mut *mut ::c_void
    ) -> ::HRESULT
});

RIDL!(
interface ID3D12Device(ID3D12DeviceVtbl): ID3D12Object(ID3D12ObjectVtbl) {
    fn GetNodeCount(&mut self, This: *mut ::ID3D12Device) -> ::UINT,
    fn CreateCommandQueue(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_COMMAND_QUEUE_DESC,
        riid: ::REFGUID, ppCommandQueue: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateCommandAllocator(
        &mut self, This: *mut ::ID3D12Device, type_: ::D3D12_COMMAND_LIST_TYPE, riid: ::REFGUID,
        ppCommandAllocator: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateGraphicsPipelineState(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
        riid: ::REFGUID, ppPipelineState: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateComputePipelineState(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_COMPUTE_PIPELINE_STATE_DESC,
        riid: ::REFGUID, ppPipelineState: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateCommandList(
        &mut self, This: *mut ::ID3D12Device, nodeMask: ::UINT, type_: ::D3D12_COMMAND_LIST_TYPE,
        pCommandAllocator: *mut ::ID3D12CommandAllocator,
        pInitialState: *mut ::ID3D12PipelineState, riid: ::REFGUID,
        ppCommandList: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CheckFeatureSupport(
        &mut self, This: *mut ::ID3D12Device, Feature: ::D3D12_FEATURE,
        pFeatureSupportData: *mut ::c_void, FeatureSupportDataSize: ::UINT
    ) -> ::HRESULT,
    fn CreateDescriptorHeap(
        &mut self, This: *mut ::ID3D12Device,
        pDescriptorHeapDesc: *const ::D3D12_DESCRIPTOR_HEAP_DESC, riid: ::REFGUID,
        ppvHeap: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn GetDescriptorHandleIncrementSize(
        &mut self, This: *mut ::ID3D12Device, DescriptorHeapType: ::D3D12_DESCRIPTOR_HEAP_TYPE
    ) -> ::UINT,
    fn CreateRootSignature(
        &mut self, This: *mut ::ID3D12Device, nodeMask: ::UINT,
        pBlobWithRootSignature: *const ::c_void, blobLengthInBytes: ::SIZE_T, riid: ::REFGUID,
        ppvRootSignature: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateConstantBufferView(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_CONSTANT_BUFFER_VIEW_DESC,
        DestDescriptor: ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn CreateShaderResourceView(
        &mut self, This: *mut ::ID3D12Device, pResource: *mut ::ID3D12Resource,
        pDesc: *const ::D3D12_SHADER_RESOURCE_VIEW_DESC,
        DestDescriptor: ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn CreateUnorderedAccessView(
        &mut self, This: *mut ::ID3D12Device, pResource: *mut ::ID3D12Resource,
        pCounterResource: *mut ::ID3D12Resource, pDesc: *const ::D3D12_UNORDERED_ACCESS_VIEW_DESC,
        DestDescriptor: ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn CreateRenderTargetView(
        &mut self, This: *mut ::ID3D12Device, pResource: *mut ::ID3D12Resource,
        pDesc: *const ::D3D12_RENDER_TARGET_VIEW_DESC,
        DestDescriptor: ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn CreateDepthStencilView(
        &mut self, This: *mut ::ID3D12Device, pResource: *mut ::ID3D12Resource,
        pDesc: *const ::D3D12_DEPTH_STENCIL_VIEW_DESC,
        DestDescriptor: ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn CreateSampler(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_SAMPLER_DESC,
        DestDescriptor: ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn CopyDescriptors(
        &mut self, This: *mut ::ID3D12Device, NumDestDescriptorRanges: ::UINT,
        pDestDescriptorRangeStarts: *const ::D3D12_CPU_DESCRIPTOR_HANDLE,
        pDestDescriptorRangeSizes: *const ::UINT, NumSrcDescriptorRanges: ::UINT,
        pSrcDescriptorRangeStarts: *const ::D3D12_CPU_DESCRIPTOR_HANDLE,
        pSrcDescriptorRangeSizes: *const ::UINT, DescriptorHeapsType: ::D3D12_DESCRIPTOR_HEAP_TYPE
    ) -> (),
    fn CopyDescriptorsSimple(
        &mut self, This: *mut ::ID3D12Device, NumDescriptors: ::UINT,
        DestDescriptorRangeStart: ::D3D12_CPU_DESCRIPTOR_HANDLE,
        SrcDescriptorRangeStart: ::D3D12_CPU_DESCRIPTOR_HANDLE,
        DescriptorHeapsType: ::D3D12_DESCRIPTOR_HEAP_TYPE
    ) -> (),
    fn GetResourceAllocationInfo(
        &mut self, This: *mut ::ID3D12Device, visibleMask: ::UINT, numResourceDescs: ::UINT,
        pResourceDescs: *const ::D3D12_RESOURCE_DESC
    ) -> ::D3D12_RESOURCE_ALLOCATION_INFO,
    fn GetCustomHeapProperties(
        &mut self, This: *mut ::ID3D12Device, nodeMask: ::UINT, heapType: ::D3D12_HEAP_TYPE
    ) -> ::D3D12_HEAP_PROPERTIES,
    fn CreateCommittedResource(
        &mut self, This: *mut ::ID3D12Device, pHeapProperties: *const ::D3D12_HEAP_PROPERTIES,
        HeapFlags: ::D3D12_HEAP_FLAGS, pResourceDesc: *const ::D3D12_RESOURCE_DESC,
        InitialResourceState: ::D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const ::D3D12_CLEAR_VALUE, riidResource: ::REFGUID,
        ppvResource: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateHeap(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_HEAP_DESC, riid: ::REFGUID,
        ppvHeap: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreatePlacedResource(
        &mut self, This: *mut ::ID3D12Device, pHeap: *mut ::ID3D12Heap, HeapOffset: ::UINT64,
        pDesc: *const ::D3D12_RESOURCE_DESC, InitialState: ::D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const ::D3D12_CLEAR_VALUE, riid: ::REFGUID,
        ppvResource: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateReservedResource(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_RESOURCE_DESC,
        InitialState: ::D3D12_RESOURCE_STATES, pOptimizedClearValue: *const ::D3D12_CLEAR_VALUE,
        riid: ::REFGUID, ppvResource: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn CreateSharedHandle(
        &mut self, This: *mut ::ID3D12Device, pObject: *mut ::ID3D12DeviceChild,
        pAttributes: *const ::SECURITY_ATTRIBUTES, Access: ::DWORD, Name: ::LPCWSTR,
        pHandle: *mut ::HANDLE
    ) -> ::HRESULT,
    fn OpenSharedHandle(
        &mut self, This: *mut ::ID3D12Device, NTHandle: ::HANDLE, riid: ::REFGUID,
        ppvObj: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn OpenSharedHandleByName(
        &mut self, This: *mut ::ID3D12Device, Name: ::LPCWSTR, Access: ::DWORD,
        pNTHandle: *mut ::HANDLE
    ) -> ::HRESULT,
    fn MakeResident(
        &mut self, This: *mut ::ID3D12Device, NumObjects: ::UINT,
        ppObjects: *mut *mut ::ID3D12Pageable
    ) -> ::HRESULT,
    fn Evict(
        &mut self, This: *mut ::ID3D12Device, NumObjects: ::UINT,
        ppObjects: *mut *mut ::ID3D12Pageable
    ) -> ::HRESULT,
    fn CreateFence(
        &mut self, This: *mut ::ID3D12Device, InitialValue: ::UINT64, Flags: ::D3D12_FENCE_FLAGS,
        riid: ::REFGUID, ppFence: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn GetDeviceRemovedReason(
        &mut self, This: *mut ::ID3D12Device
    ) -> ::HRESULT,
    fn GetCopyableFootprints(
        &mut self, This: *mut ::ID3D12Device, pResourceDesc: *const ::D3D12_RESOURCE_DESC,
        FirstSubresource: ::UINT, NumSubresources: ::UINT, BaseOffset: ::UINT64,
        pLayouts: *mut ::D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pNumRows: *mut ::UINT,
        pRowSizeInBytes: *mut ::UINT64, pTotalBytes: *mut ::UINT64
    ) -> (),
    fn CreateQueryHeap(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_QUERY_HEAP_DESC,
        riid: ::REFGUID, ppvHeap: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn SetStablePowerState(
        &mut self, This: *mut ::ID3D12Device, Enable: ::BOOL
    ) -> ::HRESULT,
    fn CreateCommandSignature(
        &mut self, This: *mut ::ID3D12Device, pDesc: *const ::D3D12_COMMAND_SIGNATURE_DESC,
        pRootSignature: *mut ::ID3D12RootSignature, riid: ::REFGUID,
        ppvCommandSignature: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn GetResourceTiling(
        &mut self, This: *mut ::ID3D12Device, pTiledResource: *mut ::ID3D12Resource,
        pNumTilesForEntireResource: *mut ::UINT, pPackedMipDesc: *mut ::D3D12_PACKED_MIP_INFO,
        pStandardTileShapeForNonPackedMips: *mut ::D3D12_TILE_SHAPE,
        pNumSubresourceTilings: *mut ::UINT, FirstSubresourceTilingToGet: ::UINT,
        pSubresourceTilingsForNonPackedMips: *mut ::D3D12_SUBRESOURCE_TILING
    ) -> (),
    fn GetAdapterLuid(
        &mut self, This: *mut ::ID3D12Device, __ret_val: *mut ::LUID
    ) -> *mut ::LUID
});

RIDL!(
interface ID3D12Fence(ID3D12FenceVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn GetCompletedValue(&mut self, This: *mut ::ID3D12Fence) -> ::UINT64,
    fn SetEventOnCompletion(
        &mut self, This: *mut ::ID3D12Fence, Value: ::UINT64, hEvent: ::HANDLE
    ) -> ::HRESULT,
    fn Signal(&mut self, This: *mut ::ID3D12Fence, Value: ::UINT64) -> ::HRESULT
});

RIDL!(
interface ID3D12GraphicsCommandList(ID3D12GraphicsCommandListVtbl): ID3D12CommandList(ID3D12CommandListVtbl) {
    fn Close(&mut self, This: *mut ::ID3D12GraphicsCommandList) -> ::HRESULT,
    fn Reset(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pAllocator: *mut ::ID3D12CommandAllocator, pInitialState: *mut ::ID3D12PipelineState
    ) -> ::HRESULT,
    fn ClearState(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pPipelineState: *mut ::ID3D12PipelineState
    ) -> (),
    fn DrawInstanced(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, VertexCountPerInstance: ::UINT,
        InstanceCount: ::UINT, StartVertexLocation: ::UINT, StartInstanceLocation: ::UINT
    ) -> (),
    fn DrawIndexedInstanced(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, IndexCountPerInstance: ::UINT,
        InstanceCount: ::UINT, StartIndexLocation: ::UINT, BaseVertexLocation: ::INT,
        StartInstanceLocation: ::UINT
    ) -> (),
    fn Dispatch(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, ThreadGroupCountX: ::UINT,
        ThreadGroupCountY: ::UINT, ThreadGroupCountZ: ::UINT
    ) -> (),
    fn CopyBufferRegion(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pDstBuffer: *mut ::ID3D12Resource,
        DstOffset: ::UINT64, pSrcBuffer: *mut ::ID3D12Resource, SrcOffset: ::UINT64,
        NumBytes: ::UINT64
    ) -> (),
    fn CopyTextureRegion(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pDst: *const ::D3D12_TEXTURE_COPY_LOCATION, DstX: ::UINT, DstY: ::UINT, DstZ: ::UINT,
        pSrc: *const ::D3D12_TEXTURE_COPY_LOCATION, pSrcBox: *const ::D3D12_BOX
    ) -> (),
    fn CopyResource(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pDstResource: *mut ::ID3D12Resource,
        pSrcResource: *mut ::ID3D12Resource
    ) -> (),
    fn CopyTiles(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pTiledResource: *mut ::ID3D12Resource,
        pTileRegionStartCoordinate: *const ::D3D12_TILED_RESOURCE_COORDINATE,
        pTileRegionSize: *const ::D3D12_TILE_REGION_SIZE, pBuffer: *mut ::ID3D12Resource,
        BufferStartOffsetInBytes: ::UINT64, Flags: ::D3D12_TILE_COPY_FLAGS
    ) -> (),
    fn ResolveSubresource(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pDstResource: *mut ::ID3D12Resource,
        DstSubresource: ::UINT, pSrcResource: *mut ::ID3D12Resource, SrcSubresource: ::UINT,
        Format: ::DXGI_FORMAT
    ) -> (),
    fn IASetPrimitiveTopology(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        PrimitiveTopology: ::D3D12_PRIMITIVE_TOPOLOGY
    ) -> (),
    fn RSSetViewports(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, NumViewports: ::UINT,
        pViewports: *const ::D3D12_VIEWPORT
    ) -> (),
    fn RSSetScissorRects(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, NumRects: ::UINT,
        pRects: *const ::D3D12_RECT
    ) -> (),
    fn OMSetBlendFactor(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, BlendFactor: *mut [::FLOAT; 4]
    ) -> (),
    fn OMSetStencilRef(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, StencilRef: ::UINT
    ) -> (),
    fn SetPipelineState(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pPipelineState: *mut ::ID3D12PipelineState
    ) -> (),
    fn ResourceBarrier(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, NumBarriers: ::UINT,
        pBarriers: *const ::D3D12_RESOURCE_BARRIER
    ) -> (),
    fn ExecuteBundle(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pCommandList: *mut ::ID3D12GraphicsCommandList
    ) -> (),
    fn SetDescriptorHeaps(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, NumDescriptorHeaps: ::UINT,
        ppDescriptorHeaps: *mut *mut ::ID3D12DescriptorHeap
    ) -> (),
    fn SetComputeRootSignature(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pRootSignature: *mut ::ID3D12RootSignature
    ) -> (),
    fn SetGraphicsRootSignature(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pRootSignature: *mut ::ID3D12RootSignature
    ) -> (),
    fn SetComputeRootDescriptorTable(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BaseDescriptor: ::D3D12_GPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn SetGraphicsRootDescriptorTable(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BaseDescriptor: ::D3D12_GPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn SetComputeRoot32BitConstant(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        SrcData: ::UINT, DestOffsetIn32BitValues: ::UINT
    ) -> (),
    fn SetGraphicsRoot32BitConstant(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        SrcData: ::UINT, DestOffsetIn32BitValues: ::UINT
    ) -> (),
    fn SetComputeRoot32BitConstants(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        Num32BitValuesToSet: ::UINT, pSrcData: *const ::c_void, DestOffsetIn32BitValues: ::UINT
    ) -> (),
    fn SetGraphicsRoot32BitConstants(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        Num32BitValuesToSet: ::UINT, pSrcData: *const ::c_void, DestOffsetIn32BitValues: ::UINT
    ) -> (),
    fn SetComputeRootConstantBufferView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS
    ) -> (),
    fn SetGraphicsRootConstantBufferView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS
    ) -> (),
    fn SetComputeRootShaderResourceView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS
    ) -> (),
    fn SetGraphicsRootShaderResourceView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS
    ) -> (),
    fn SetComputeRootUnorderedAccessView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS
    ) -> (),
    fn SetGraphicsRootUnorderedAccessView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, RootParameterIndex: ::UINT,
        BufferLocation: ::D3D12_GPU_VIRTUAL_ADDRESS
    ) -> (),
    fn IASetIndexBuffer(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pView: *const ::D3D12_INDEX_BUFFER_VIEW
    ) -> (),
    fn IASetVertexBuffers(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, StartSlot: ::UINT, NumViews: ::UINT,
        pViews: *const ::D3D12_VERTEX_BUFFER_VIEW
    ) -> (),
    fn SOSetTargets(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, StartSlot: ::UINT, NumViews: ::UINT,
        pViews: *const ::D3D12_STREAM_OUTPUT_BUFFER_VIEW
    ) -> (),
    fn OMSetRenderTargets(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, NumRenderTargetDescriptors: ::UINT,
        pRenderTargetDescriptors: *const ::D3D12_CPU_DESCRIPTOR_HANDLE,
        RTsSingleHandleToDescriptorRange: ::BOOL,
        pDepthStencilDescriptor: *const ::D3D12_CPU_DESCRIPTOR_HANDLE
    ) -> (),
    fn ClearDepthStencilView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        DepthStencilView: ::D3D12_CPU_DESCRIPTOR_HANDLE, ClearFlags: ::D3D12_CLEAR_FLAGS,
        Depth: ::FLOAT, Stencil: ::UINT8, NumRects: ::UINT, pRects: *const ::D3D12_RECT
    ) -> (),
    fn ClearRenderTargetView(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        RenderTargetView: ::D3D12_CPU_DESCRIPTOR_HANDLE, ColorRGBA: *mut [::FLOAT; 4],
        NumRects: ::UINT, pRects: *const ::D3D12_RECT
    ) -> (),
    fn ClearUnorderedAccessViewUint(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        ViewGPUHandleInCurrentHeap: ::D3D12_GPU_DESCRIPTOR_HANDLE,
        ViewCPUHandle: ::D3D12_CPU_DESCRIPTOR_HANDLE, pResource: *mut ::ID3D12Resource,
        Values: *mut [::UINT; 4], NumRects: ::UINT, pRects: *const ::D3D12_RECT
    ) -> (),
    fn ClearUnorderedAccessViewFloat(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        ViewGPUHandleInCurrentHeap: ::D3D12_GPU_DESCRIPTOR_HANDLE,
        ViewCPUHandle: ::D3D12_CPU_DESCRIPTOR_HANDLE, pResource: *mut ::ID3D12Resource,
        Values: *mut [::FLOAT; 4], NumRects: ::UINT, pRects: *const ::D3D12_RECT
    ) -> (),
    fn DiscardResource(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pResource: *mut ::ID3D12Resource,
        pRegion: *const ::D3D12_DISCARD_REGION
    ) -> (),
    fn BeginQuery(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pQueryHeap: *mut ::ID3D12QueryHeap,
        Type: ::D3D12_QUERY_TYPE, Index: ::UINT
    ) -> (),
    fn EndQuery(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pQueryHeap: *mut ::ID3D12QueryHeap,
        Type: ::D3D12_QUERY_TYPE, Index: ::UINT
    ) -> (),
    fn ResolveQueryData(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pQueryHeap: *mut ::ID3D12QueryHeap,
        Type: ::D3D12_QUERY_TYPE, StartIndex: ::UINT, NumQueries: ::UINT,
        pDestinationBuffer: *mut ::ID3D12Resource, AlignedDestinationBufferOffset: ::UINT64
    ) -> (),
    fn SetPredication(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, pBuffer: *mut ::ID3D12Resource,
        AlignedBufferOffset: ::UINT64, Operation: ::D3D12_PREDICATION_OP
    ) -> (),
    fn SetMarker(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, Metadata: ::UINT,
        pData: *const ::c_void, Size: ::UINT
    ) -> (),
    fn BeginEvent(
        &mut self, This: *mut ::ID3D12GraphicsCommandList, Metadata: ::UINT,
        pData: *const ::c_void, Size: ::UINT
    ) -> (),
    fn EndEvent(&mut self, This: *mut ::ID3D12GraphicsCommandList) -> (),
    fn ExecuteIndirect(
        &mut self, This: *mut ::ID3D12GraphicsCommandList,
        pCommandSignature: *mut ::ID3D12CommandSignature, MaxCommandCount: ::UINT,
        pArgumentBuffer: *mut ::ID3D12Resource, ArgumentBufferOffset: ::UINT64,
        pCountBuffer: *mut ::ID3D12Resource, CountBufferOffset: ::UINT64
    ) -> ()
});

RIDL!(
interface ID3D12Heap(ID3D12HeapVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn GetDesc(&mut self, This: *mut ::ID3D12Heap) -> ::D3D12_HEAP_DESC
});

RIDL!(
interface ID3D12Object(ID3D12ObjectVtbl): IUnknown(IUnknownVtbl) {
    fn GetPrivateData(
        &mut self, This: *mut ::ID3D12Object, guid: ::REFGUID, pDataSize: *mut ::UINT,
        pData: *mut ::c_void
    ) -> ::HRESULT,
    fn SetPrivateData(
        &mut self, This: *mut ::ID3D12Object, guid: ::REFGUID, DataSize: ::UINT,
        pData: *const ::c_void
    ) -> ::HRESULT,
    fn SetPrivateDataInterface(
        &mut self, This: *mut ::ID3D12Object, guid: ::REFGUID, pData: *const ::IUnknown
    ) -> ::HRESULT,
    fn SetName(
        &mut self, This: *mut ::ID3D12Object, Name: ::LPCWSTR
    ) -> ::HRESULT
});

RIDL!(
interface ID3D12Pageable(ID3D12PageableVtbl): ID3D12DeviceChild(ID3D12DeviceChildVtbl) {
});

RIDL!(
interface ID3D12PipelineState(ID3D12PipelineStateVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn GetCachedBlob(
        &mut self, This: *mut ::ID3D12PipelineState, ppBlob: *mut *mut ::ID3DBlob
    ) -> ::HRESULT
});

RIDL!(
interface ID3D12QueryHeap(ID3D12QueryHeapVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
});

RIDL!(
interface ID3D12Resource(ID3D12ResourceVtbl): ID3D12Pageable(ID3D12PageableVtbl) {
    fn Map(
        &mut self, This: *mut ::ID3D12Resource, Subresource: ::UINT,
        pReadRange: *const ::D3D12_RANGE, ppData: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn Unmap(
        &mut self, This: *mut ::ID3D12Resource, Subresource: ::UINT,
        pWrittenRange: *const ::D3D12_RANGE
    ) -> (),
    fn GetDesc(&mut self, This: *mut ::ID3D12Resource) -> ::D3D12_RESOURCE_DESC,
    fn GetGPUVirtualAddress(
        &mut self, This: *mut ::ID3D12Resource
    ) -> ::D3D12_GPU_VIRTUAL_ADDRESS,
    fn WriteToSubresource(
        &mut self, This: *mut ::ID3D12Resource, DstSubresource: ::UINT,
        pDstBox: *const ::D3D12_BOX, pSrcData: *const ::c_void, SrcRowPitch: ::UINT,
        SrcDepthPitch: ::UINT
    ) -> ::HRESULT,
    fn ReadFromSubresource(
        &mut self, This: *mut ::ID3D12Resource, pDstData: *mut ::c_void, DstRowPitch: ::UINT,
        DstDepthPitch: ::UINT, SrcSubresource: ::UINT, pSrcBox: *const ::D3D12_BOX
    ) -> ::HRESULT,
    fn GetHeapProperties(
        &mut self, This: *mut ::ID3D12Resource, pHeapProperties: *mut ::D3D12_HEAP_PROPERTIES,
        pHeapFlags: *mut ::D3D12_HEAP_FLAGS
    ) -> ::HRESULT
});

RIDL!(
interface ID3D12RootSignatureDeserializer(ID3D12RootSignatureDeserializerVtbl): IUnknown(IUnknownVtbl) {
    fn GetRootSignatureDesc(
        &mut self, This: *mut ::ID3D12RootSignatureDeserializer
    ) -> *const ::D3D12_ROOT_SIGNATURE_DESC
});

RIDL!(
interface ID3D12RootSignature(ID3D12RootSignatureVtbl): ID3D12DeviceChild(ID3D12DeviceChildVtbl) {
});

pub type D3D12_GPU_VIRTUAL_ADDRESS = ::UINT64;
pub type D3D12_PRIMITIVE = ::D3D_PRIMITIVE;
pub type D3D12_PRIMITIVE_TOPOLOGY = ::D3D_PRIMITIVE_TOPOLOGY;
pub type D3D12_RECT = ::RECT;
pub type PFN_D3D12_CREATE_DEVICE = extern "system" fn (_ : *mut ::IUnknown, _ : ::D3D_FEATURE_LEVEL, _ : ::REFGUID, _ : *mut *mut ::c_void) -> ::HRESULT;
pub type PFN_D3D12_CREATE_ROOT_SIGNATURE_DESERIALIZER = extern "system" fn (pSrcData: ::LPCVOID, SrcDataSizeInBytes: ::SIZE_T, pRootSignatureDeserializerInterface: ::REFGUID, ppRootSignatureDeserializer: *mut *mut ::c_void) -> ::HRESULT;
pub type PFN_D3D12_GET_DEBUG_INTERFACE = extern "system" fn (_ : ::REFGUID, _ : *mut *mut ::c_void) -> ::HRESULT;
pub type PFN_D3D12_SERIALIZE_ROOT_SIGNATURE = extern "system" fn (pRootSignature: *const ::D3D12_ROOT_SIGNATURE_DESC, Version: ::D3D_ROOT_SIGNATURE_VERSION, ppBlob: *mut *mut ::ID3DBlob, ppErrorBlob: *mut *mut ::ID3DBlob) -> ::HRESULT;
pub const D3D12_16BIT_INDEX_STRIP_CUT_VALUE: ::UINT = 0xffff;
pub const D3D12_32BIT_INDEX_STRIP_CUT_VALUE: ::UINT = 0xffffffff;
pub const D3D12_8BIT_INDEX_STRIP_CUT_VALUE: ::UINT = 0xff;
pub const D3D12_ANISOTROPIC_FILTERING_BIT: ::UINT = 0x40;
pub const D3D12_APPEND_ALIGNED_ELEMENT: ::UINT = 0xffffffff;
pub const D3D12_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: ::UINT = 9;
pub const D3D12_CLIP_OR_CULL_DISTANCE_COUNT: ::UINT = 8;
pub const D3D12_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: ::UINT = 2;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: ::UINT = 14;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: ::UINT = 4;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: ::UINT = 15;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: ::UINT = 16;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: ::UINT = 15;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: ::UINT = 1;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: ::UINT = 64;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: ::UINT = 1;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: ::UINT = 128;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: ::UINT = 1;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: ::UINT = 128;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COUNT: ::UINT = 16;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: ::UINT = 1;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_COMMONSHADER_SAMPLER_SLOT_COUNT: ::UINT = 16;
pub const D3D12_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: ::UINT = 32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COUNT: ::UINT = 4096;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: ::UINT = 3;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READ_PORTS: ::UINT = 3;
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: ::UINT = 10;
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: ::UINT = 7;
pub const D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT: ::UINT = 256;
pub const D3D12_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 256;
pub const D3D12_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: ::UINT = 64;
pub const D3D12_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 240;
pub const D3D12_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: ::UINT = 68;
pub const D3D12_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 224;
pub const D3D12_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: ::UINT = 72;
pub const D3D12_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 208;
pub const D3D12_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: ::UINT = 76;
pub const D3D12_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 192;
pub const D3D12_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: ::UINT = 84;
pub const D3D12_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 176;
pub const D3D12_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: ::UINT = 92;
pub const D3D12_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 160;
pub const D3D12_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: ::UINT = 100;
pub const D3D12_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 144;
pub const D3D12_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: ::UINT = 112;
pub const D3D12_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 128;
pub const D3D12_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: ::UINT = 128;
pub const D3D12_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 112;
pub const D3D12_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: ::UINT = 144;
pub const D3D12_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 96;
pub const D3D12_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: ::UINT = 168;
pub const D3D12_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 80;
pub const D3D12_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: ::UINT = 204;
pub const D3D12_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 64;
pub const D3D12_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: ::UINT = 256;
pub const D3D12_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 48;
pub const D3D12_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: ::UINT = 340;
pub const D3D12_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 32;
pub const D3D12_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: ::UINT = 512;
pub const D3D12_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: ::UINT = 16;
pub const D3D12_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: ::UINT = 768;
pub const D3D12_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: ::UINT = 1;
pub const D3D12_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: ::UINT = 256;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: ::UINT = 768;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_X: ::UINT = 768;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_Y: ::UINT = 768;
pub const D3D12_CS_4_X_UAV_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: ::UINT = 65535;
pub const D3D12_CS_TGSM_REGISTER_COUNT: ::UINT = 8192;
pub const D3D12_CS_TGSM_REGISTER_READS_PER_INST: ::UINT = 1;
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_CS_THREADGROUPID_REGISTER_COMPONENTS: ::UINT = 3;
pub const D3D12_CS_THREADGROUPID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COMPONENTS: ::UINT = 3;
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_CS_THREADID_REGISTER_COMPONENTS: ::UINT = 3;
pub const D3D12_CS_THREADID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: ::UINT = 1024;
pub const D3D12_CS_THREAD_GROUP_MAX_X: ::UINT = 1024;
pub const D3D12_CS_THREAD_GROUP_MAX_Y: ::UINT = 1024;
pub const D3D12_CS_THREAD_GROUP_MAX_Z: ::UINT = 64;
pub const D3D12_CS_THREAD_GROUP_MIN_X: ::UINT = 1;
pub const D3D12_CS_THREAD_GROUP_MIN_Y: ::UINT = 1;
pub const D3D12_CS_THREAD_GROUP_MIN_Z: ::UINT = 1;
pub const D3D12_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: ::UINT = 16384;
pub const D3D12_DEFAULT_BLEND_FACTOR_ALPHA: ::FLOAT = 1.0;
pub const D3D12_DEFAULT_BLEND_FACTOR_BLUE: ::FLOAT = 1.0;
pub const D3D12_DEFAULT_BLEND_FACTOR_GREEN: ::FLOAT = 1.0;
pub const D3D12_DEFAULT_BLEND_FACTOR_RED: ::FLOAT = 1.0;
pub const D3D12_DEFAULT_BORDER_COLOR_COMPONENT: ::FLOAT = 0.0;
pub const D3D12_DEFAULT_DEPTH_BIAS: ::UINT = 0;
pub const D3D12_DEFAULT_DEPTH_BIAS_CLAMP: ::FLOAT = 0.0;
pub const D3D12_DEFAULT_MAX_ANISOTROPY: ::UINT = 16;
pub const D3D12_DEFAULT_MIP_LOD_BIAS: ::FLOAT = 0.0;
pub const D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: ::UINT = 4194304;
pub const D3D12_DEFAULT_RENDER_TARGET_ARRAY_INDEX: ::UINT = 0;
pub const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: ::UINT = 65536;
pub const D3D12_DEFAULT_SAMPLE_MASK: ::UINT = 0xffffffff;
pub const D3D12_DEFAULT_SCISSOR_ENDX: ::UINT = 0;
pub const D3D12_DEFAULT_SCISSOR_ENDY: ::UINT = 0;
pub const D3D12_DEFAULT_SCISSOR_STARTX: ::UINT = 0;
pub const D3D12_DEFAULT_SCISSOR_STARTY: ::UINT = 0;
pub const D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: ::FLOAT = 0.0;
pub const D3D12_DEFAULT_STENCIL_READ_MASK: ::UINT = 0xff;
pub const D3D12_DEFAULT_STENCIL_REFERENCE: ::UINT = 0;
pub const D3D12_DEFAULT_STENCIL_WRITE_MASK: ::UINT = 0xff;
pub const D3D12_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: ::UINT = 0;
pub const D3D12_DEFAULT_VIEWPORT_HEIGHT: ::UINT = 0;
pub const D3D12_DEFAULT_VIEWPORT_MAX_DEPTH: ::FLOAT = 0.0;
pub const D3D12_DEFAULT_VIEWPORT_MIN_DEPTH: ::FLOAT = 0.0;
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTX: ::UINT = 0;
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTY: ::UINT = 0;
pub const D3D12_DEFAULT_VIEWPORT_WIDTH: ::UINT = 0;
pub const D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND: ::UINT = 0xffffffff;
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_END: ::UINT = 0xfffffff7;
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_START: ::UINT = 0xfffffff0;
pub const D3D12_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: ::UINT = 3968;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: ::UINT = 3;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_DS_OUTPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_FILTER_REDUCTION_TYPE_MASK: ::UINT = 0x3;
pub const D3D12_FILTER_REDUCTION_TYPE_SHIFT: ::UINT = 7;
pub const D3D12_FILTER_TYPE_MASK: ::UINT = 0x3;
pub const D3D12_FLOAT16_FUSED_TOLERANCE_IN_ULP: ::DOUBLE = 0.6;
pub const D3D12_FLOAT32_MAX: ::FLOAT = 3.402823466e+38;
pub const D3D12_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: ::FLOAT = 0.6;
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: ::FLOAT = 2.4;
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: ::FLOAT = 1.0;
pub const D3D12_FLOAT_TO_SRGB_OFFSET: ::FLOAT = 0.055;
pub const D3D12_FLOAT_TO_SRGB_SCALE_1: ::FLOAT = 12.92;
pub const D3D12_FLOAT_TO_SRGB_SCALE_2: ::FLOAT = 1.055;
pub const D3D12_FLOAT_TO_SRGB_THRESHOLD: ::FLOAT = 0.0031308;
pub const D3D12_FTOI_INSTRUCTION_MAX_INPUT: ::FLOAT = 2147483647.999;
pub const D3D12_FTOI_INSTRUCTION_MIN_INPUT: ::FLOAT = -2147483648.999;
pub const D3D12_FTOU_INSTRUCTION_MAX_INPUT: ::FLOAT = 4294967295.999;
pub const D3D12_FTOU_INSTRUCTION_MIN_INPUT: ::FLOAT = 0.0;
pub const D3D12_GS_INPUT_INSTANCE_ID_READS_PER_INST: ::UINT = 2;
pub const D3D12_GS_INPUT_INSTANCE_ID_READ_PORTS: ::UINT = 1;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_GS_INPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_GS_INPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_GS_INPUT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_GS_INPUT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_GS_INPUT_REGISTER_VERTICES: ::UINT = 32;
pub const D3D12_GS_MAX_INSTANCE_COUNT: ::UINT = 32;
pub const D3D12_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: ::UINT = 1024;
pub const D3D12_GS_OUTPUT_ELEMENTS: ::UINT = 32;
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_GS_OUTPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_HS_CONTROL_POINT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: ::UINT = 0xffffffff;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: ::UINT = 0xffffffff;
pub const D3D12_HS_MAXTESSFACTOR_LOWER_BOUND: ::FLOAT = 1.0;
pub const D3D12_HS_MAXTESSFACTOR_UPPER_BOUND: ::FLOAT = 64.0;
pub const D3D12_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: ::UINT = 3968;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: ::UINT = 128;
pub const D3D12_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: ::UINT = 0;
pub const D3D12_IA_DEFAULT_PRIMITIVE_TOPOLOGY: ::UINT = 0;
pub const D3D12_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: ::UINT = 0;
pub const D3D12_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: ::UINT = 1;
pub const D3D12_IA_INSTANCE_ID_BIT_COUNT: ::UINT = 32;
pub const D3D12_IA_INTEGER_ARITHMETIC_BIT_COUNT: ::UINT = 32;
pub const D3D12_IA_PATCH_MAX_CONTROL_POINT_COUNT: ::UINT = 32;
pub const D3D12_IA_PRIMITIVE_ID_BIT_COUNT: ::UINT = 32;
pub const D3D12_IA_VERTEX_ID_BIT_COUNT: ::UINT = 32;
pub const D3D12_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: ::UINT = 32;
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: ::UINT = 128;
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: ::UINT = 32;
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: ::UINT = 0xffffffff;
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_REMAINDER: ::UINT = 0xffffffff;
pub const D3D12_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: ::UINT = 0xffffffff;
pub const D3D12_KEEP_UNORDERED_ACCESS_VIEWS: ::UINT = 0xffffffff;
pub const D3D12_LINEAR_GAMMA: ::FLOAT = 1.0;
pub const D3D12_MAG_FILTER_SHIFT: ::UINT = 2;
pub const D3D12_MAJOR_VERSION: ::UINT = 12;
pub const D3D12_MAX_BORDER_COLOR_COMPONENT: ::FLOAT = 1.0;
pub const D3D12_MAX_DEPTH: ::FLOAT = 1.0;
pub const D3D12_MAX_LIVE_STATIC_SAMPLERS: ::UINT = 2032;
pub const D3D12_MAX_MAXANISOTROPY: ::UINT = 16;
pub const D3D12_MAX_MULTISAMPLE_SAMPLE_COUNT: ::UINT = 32;
pub const D3D12_MAX_POSITION_VALUE: ::FLOAT = 3.402823466e+34;
pub const D3D12_MAX_ROOT_COST: ::UINT = 64;
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_1: ::UINT = 1000000;
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_2: ::UINT = 1000000;
pub const D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE: ::UINT = 2048;
pub const D3D12_MAX_TEXTURE_DIMENSION_2_TO_EXP: ::UINT = 17;
pub const D3D12_MINOR_VERSION: ::UINT = 0;
pub const D3D12_MIN_BORDER_COLOR_COMPONENT: ::FLOAT = 0.0;
pub const D3D12_MIN_DEPTH: ::FLOAT = 0.0;
pub const D3D12_MIN_FILTER_SHIFT: ::UINT = 4;
pub const D3D12_MIN_MAXANISOTROPY: ::UINT = 0;
pub const D3D12_MIP_FILTER_SHIFT: ::UINT = 0;
pub const D3D12_MIP_LOD_BIAS_MAX: ::FLOAT = 15.99;
pub const D3D12_MIP_LOD_BIAS_MIN: ::FLOAT = -16.0;
pub const D3D12_MIP_LOD_FRACTIONAL_BIT_COUNT: ::UINT = 8;
pub const D3D12_MIP_LOD_RANGE_BIT_COUNT: ::UINT = 8;
pub const D3D12_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: ::FLOAT = 1.4;
pub const D3D12_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: ::UINT = 0;
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_END: ::UINT = 0xffffffff;
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_START: ::UINT = 0xfffffff8;
pub const D3D12_PACKED_TILE: ::UINT = 0xffffffff;
pub const D3D12_PIXEL_ADDRESS_RANGE_BIT_COUNT: ::UINT = 15;
pub const D3D12_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: ::UINT = 16;
pub const D3D12_PS_CS_UAV_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_PS_CS_UAV_REGISTER_COUNT: ::UINT = 8;
pub const D3D12_PS_CS_UAV_REGISTER_READS_PER_INST: ::UINT = 1;
pub const D3D12_PS_CS_UAV_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_PS_FRONTFACING_DEFAULT_VALUE: ::UINT = 0xffffffff;
pub const D3D12_PS_FRONTFACING_FALSE_VALUE: ::UINT = 0;
pub const D3D12_PS_FRONTFACING_TRUE_VALUE: ::UINT = 0xffffffff;
pub const D3D12_PS_INPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_PS_INPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_PS_INPUT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_PS_INPUT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: ::FLOAT = 0.0;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENTS: ::UINT = 1;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COUNT: ::UINT = 1;
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_PS_OUTPUT_REGISTER_COUNT: ::UINT = 8;
pub const D3D12_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: ::FLOAT = 0.5;
pub const D3D12_RAW_UAV_SRV_BYTE_ALIGNMENT: ::UINT = 16;
pub const D3D12_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: ::UINT = 4096;
pub const D3D12_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: ::UINT = 27;
pub const D3D12_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: ::UINT = 4096;
pub const D3D12_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: ::UINT = 4096;
pub const D3D12_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: ::UINT = 32;
pub const D3D12_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: ::UINT = 32;
pub const D3D12_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: ::UINT = 16384;
pub const D3D12_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: ::UINT = 1024;
pub const D3D12_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: ::UINT = 4096;
pub const D3D12_REQ_MAXANISOTROPY: ::UINT = 16;
pub const D3D12_REQ_MIP_LEVELS: ::UINT = 15;
pub const D3D12_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: ::UINT = 2048;
pub const D3D12_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: ::UINT = 4096;
pub const D3D12_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: ::UINT = 16384;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: ::UINT = 128;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: ::FLOAT = 0.25;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: ::UINT = 2048;
pub const D3D12_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: ::UINT = 20;
pub const D3D12_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: ::UINT = 4096;
pub const D3D12_REQ_SUBRESOURCES: ::UINT = 30720;
pub const D3D12_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: ::UINT = 2048;
pub const D3D12_REQ_TEXTURE1D_U_DIMENSION: ::UINT = 16384;
pub const D3D12_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: ::UINT = 2048;
pub const D3D12_REQ_TEXTURE2D_U_OR_V_DIMENSION: ::UINT = 16384;
pub const D3D12_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: ::UINT = 2048;
pub const D3D12_REQ_TEXTURECUBE_DIMENSION: ::UINT = 16384;
pub const D3D12_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: ::UINT = 0;
pub const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: ::UINT = 0xffffffff;
pub const D3D12_SHADER_COMPONENT_MAPPING_MASK: ::UINT = 0x7;
pub const D3D12_SHADER_COMPONENT_MAPPING_SHIFT: ::UINT = 3;
pub const D3D12_SHADER_MAJOR_VERSION: ::UINT = 5;
pub const D3D12_SHADER_MAX_INSTANCES: ::UINT = 65535;
pub const D3D12_SHADER_MAX_INTERFACES: ::UINT = 253;
pub const D3D12_SHADER_MAX_INTERFACE_CALL_SITES: ::UINT = 4096;
pub const D3D12_SHADER_MAX_TYPES: ::UINT = 65535;
pub const D3D12_SHADER_MINOR_VERSION: ::UINT = 1;
pub const D3D12_SHIFT_INSTRUCTION_PAD_VALUE: ::UINT = 0;
pub const D3D12_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: ::UINT = 5;
pub const D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT: ::UINT = 8;
pub const D3D12_SMALL_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: ::UINT = 65536;
pub const D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT: ::UINT = 4096;
pub const D3D12_SO_BUFFER_MAX_STRIDE_IN_BYTES: ::UINT = 2048;
pub const D3D12_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: ::UINT = 512;
pub const D3D12_SO_BUFFER_SLOT_COUNT: ::UINT = 4;
pub const D3D12_SO_DDI_REGISTER_INDEX_DENOTING_GAP: ::UINT = 0xffffffff;
pub const D3D12_SO_NO_RASTERIZED_STREAM: ::UINT = 0xffffffff;
pub const D3D12_SO_OUTPUT_COMPONENT_COUNT: ::UINT = 128;
pub const D3D12_SO_STREAM_COUNT: ::UINT = 4;
pub const D3D12_SPEC_DATE_DAY: ::UINT = 14;
pub const D3D12_SPEC_DATE_MONTH: ::UINT = 11;
pub const D3D12_SPEC_DATE_YEAR: ::UINT = 2014;
pub const D3D12_SPEC_VERSION: ::DOUBLE = 1.16;
pub const D3D12_SRGB_GAMMA: ::FLOAT = 2.2;
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_1: ::FLOAT = 12.92;
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_2: ::FLOAT = 1.055;
pub const D3D12_SRGB_TO_FLOAT_EXPONENT: ::FLOAT = 2.4;
pub const D3D12_SRGB_TO_FLOAT_OFFSET: ::FLOAT = 0.055;
pub const D3D12_SRGB_TO_FLOAT_THRESHOLD: ::FLOAT = 0.04045;
pub const D3D12_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: ::FLOAT = 0.5;
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: ::UINT = 64;
pub const D3D12_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: ::UINT = 4;
pub const D3D12_STANDARD_PIXEL_COMPONENT_COUNT: ::UINT = 128;
pub const D3D12_STANDARD_PIXEL_ELEMENT_COUNT: ::UINT = 32;
pub const D3D12_STANDARD_VECTOR_SIZE: ::UINT = 4;
pub const D3D12_STANDARD_VERTEX_ELEMENT_COUNT: ::UINT = 32;
pub const D3D12_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: ::UINT = 64;
pub const D3D12_SUBPIXEL_FRACTIONAL_BIT_COUNT: ::UINT = 8;
pub const D3D12_SUBTEXEL_FRACTIONAL_BIT_COUNT: ::UINT = 8;
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_END: ::UINT = 0xffffffff;
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_START: ::UINT = 0xfffffff0;
pub const D3D12_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: ::UINT = 64;
pub const D3D12_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: ::UINT = 64;
pub const D3D12_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: ::UINT = 63;
pub const D3D12_TESSELLATOR_MAX_TESSELLATION_FACTOR: ::UINT = 64;
pub const D3D12_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: ::UINT = 2;
pub const D3D12_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: ::UINT = 1;
pub const D3D12_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: ::UINT = 1;
pub const D3D12_TEXEL_ADDRESS_RANGE_BIT_COUNT: ::UINT = 16;
pub const D3D12_TEXTURE_DATA_PITCH_ALIGNMENT: ::UINT = 256;
pub const D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT: ::UINT = 512;
pub const D3D12_TILED_RESOURCE_TILE_SIZE_IN_BYTES: ::UINT = 65536;
pub const D3D12_UAV_COUNTER_PLACEMENT_ALIGNMENT: ::UINT = 4096;
pub const D3D12_UAV_SLOT_COUNT: ::UINT = 64;
pub const D3D12_UNBOUND_MEMORY_ACCESS_RESULT: ::UINT = 0;
pub const D3D12_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: ::UINT = 15;
pub const D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: ::UINT = 16;
pub const D3D12_VIEWPORT_BOUNDS_MAX: ::UINT = 32767;
pub const D3D12_VS_INPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_VS_INPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_VS_INPUT_REGISTER_READS_PER_INST: ::UINT = 2;
pub const D3D12_VS_INPUT_REGISTER_READ_PORTS: ::UINT = 1;
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENTS: ::UINT = 4;
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: ::UINT = 32;
pub const D3D12_VS_OUTPUT_REGISTER_COUNT: ::UINT = 32;
pub const D3D12_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: ::UINT = 10;
pub const D3D12_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: ::UINT = 25;
pub const D3D12_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: ::UINT = 25;
