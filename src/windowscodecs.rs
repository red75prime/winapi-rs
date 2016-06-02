// Copyright © 2015; Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of wincodec.h

ENUM!{ enum WIC8BIMIptcDigestProperties {
    WIC8BIMIptcDigestPString = 1,
    WIC8BIMIptcDigestIptcDigest = 2,
    WIC8BIMIptcDigestProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WIC8BIMIptcProperties {
    WIC8BIMIptcPString = 0,
    WIC8BIMIptcEmbeddedIPTC = 1,
    WIC8BIMIptcProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WIC8BIMResolutionInfoProperties {
    WIC8BIMResolutionInfoPString = 1,
    WIC8BIMResolutionInfoHResolution = 2,
    WIC8BIMResolutionInfoHResolutionUnit = 3,
    WIC8BIMResolutionInfoWidthUnit = 4,
    WIC8BIMResolutionInfoVResolution = 5,
    WIC8BIMResolutionInfoVResolutionUnit = 6,
    WIC8BIMResolutionInfoHeightUnit = 7,
    WIC8BIMResolutionInfoProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapAlphaChannelOption {
    WICBitmapUseAlpha = 0,
    WICBitmapUsePremultipliedAlpha = 1,
    WICBitmapIgnoreAlpha = 2,
    WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapCreateCacheOption {
    WICBitmapNoCache = 0,
    WICBitmapCacheOnDemand = 1,
    WICBitmapCacheOnLoad = 2,
    WICBITMAPCREATECACHEOPTION_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapDecoderCapabilities {
    WICBitmapDecoderCapabilitySameEncoder = 1,
    WICBitmapDecoderCapabilityCanDecodeAllImages = 2,
    WICBitmapDecoderCapabilityCanDecodeSomeImages = 4,
    WICBitmapDecoderCapabilityCanEnumerateMetadata = 8,
    WICBitmapDecoderCapabilityCanDecodeThumbnail = 16,
    WICBITMAPDECODERCAPABILITIES_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapDitherType {
    WICBitmapDitherTypeNone = 0,
    WICBitmapDitherTypeSolid = 0,
    WICBitmapDitherTypeOrdered4x4 = 1,
    WICBitmapDitherTypeOrdered8x8 = 2,
    WICBitmapDitherTypeOrdered16x16 = 3,
    WICBitmapDitherTypeSpiral4x4 = 4,
    WICBitmapDitherTypeSpiral8x8 = 5,
    WICBitmapDitherTypeDualSpiral4x4 = 6,
    WICBitmapDitherTypeDualSpiral8x8 = 7,
    WICBitmapDitherTypeErrorDiffusion = 8,
    WICBITMAPDITHERTYPE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapEncoderCacheOption {
    WICBitmapEncoderCacheInMemory = 0,
    WICBitmapEncoderCacheTempFile = 1,
    WICBitmapEncoderNoCache = 2,
    WICBITMAPENCODERCACHEOPTION_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapInterpolationMode {
    WICBitmapInterpolationModeNearestNeighbor = 0,
    WICBitmapInterpolationModeLinear = 1,
    WICBitmapInterpolationModeCubic = 2,
    WICBitmapInterpolationModeFant = 3,
    WICBitmapInterpolationModeHighQualityCubic = 4,
    WICBITMAPINTERPOLATIONMODE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapLockFlags {
    WICBitmapLockRead = 1,
    WICBitmapLockWrite = 2,
    WICBITMAPLOCKFLAGS_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapPaletteType {
    WICBitmapPaletteTypeCustom = 0,
    WICBitmapPaletteTypeMedianCut = 1,
    WICBitmapPaletteTypeFixedBW = 2,
    WICBitmapPaletteTypeFixedHalftone8 = 3,
    WICBitmapPaletteTypeFixedHalftone27 = 4,
    WICBitmapPaletteTypeFixedHalftone64 = 5,
    WICBitmapPaletteTypeFixedHalftone125 = 6,
    WICBitmapPaletteTypeFixedHalftone216 = 7,
    WICBitmapPaletteTypeFixedWebPalette = 7,
    WICBitmapPaletteTypeFixedHalftone252 = 8,
    WICBitmapPaletteTypeFixedHalftone256 = 9,
    WICBitmapPaletteTypeFixedGray4 = 10,
    WICBitmapPaletteTypeFixedGray16 = 11,
    WICBitmapPaletteTypeFixedGray256 = 12,
    WICBITMAPPALETTETYPE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICBitmapTransformOptions {
    WICBitmapTransformRotate0 = 0,
    WICBitmapTransformRotate90 = 1,
    WICBitmapTransformRotate180 = 2,
    WICBitmapTransformRotate270 = 3,
    WICBitmapTransformFlipHorizontal = 8,
    WICBitmapTransformFlipVertical = 16,
    WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICColorContextType {
    WICColorContextUninitialized = 0,
    WICColorContextProfile = 1,
    WICColorContextExifColorSpace = 2,
}}

ENUM!{ enum WICComponentEnumerateOptions {
    WICComponentEnumerateDefault = 0,
    WICComponentEnumerateRefresh = 1,
    WICComponentEnumerateDisabled = 2147483648,
    WICComponentEnumerateUnsigned = 1073741824,
    WICComponentEnumerateBuiltInOnly = 536870912,
    WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICComponentSigning {
    WICComponentSigned = 1,
    WICComponentUnsigned = 2,
    WICComponentSafe = 4,
    WICComponentDisabled = 2147483648,
    WICCOMPONENTSIGNING_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICComponentType {
    WICDecoder = 1,
    WICEncoder = 2,
    WICPixelFormatConverter = 4,
    WICMetadataReader = 8,
    WICMetadataWriter = 16,
    WICPixelFormat = 32,
    WICAllComponents = 63,
    WICCOMPONENTTYPE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICDdsAlphaMode {
    WICDdsAlphaModeUnknown = 0,
    WICDdsAlphaModeStraight = 1,
    WICDdsAlphaModePremultiplied = 2,
    WICDdsAlphaModeOpaque = 3,
    WICDdsAlphaModeCustom = 4,
    WICDDSALPHAMODE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICDdsDimension {
    WICDdsTexture1D = 0,
    WICDdsTexture2D = 1,
    WICDdsTexture3D = 2,
    WICDdsTextureCube = 3,
    WICDDSTEXTURE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICDecodeOptions {
    WICDecodeMetadataCacheOnDemand = 0,
    WICDecodeMetadataCacheOnLoad = 1,
    WICMETADATACACHEOPTION_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICGifApplicationExtensionProperties {
    WICGifApplicationExtensionApplication = 1,
    WICGifApplicationExtensionData = 2,
    WICGifApplicationExtensionProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICGifCommentExtensionProperties {
    WICGifCommentExtensionText = 1,
    WICGifCommentExtensionProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICGifGraphicControlExtensionProperties {
    WICGifGraphicControlExtensionDisposal = 1,
    WICGifGraphicControlExtensionUserInputFlag = 2,
    WICGifGraphicControlExtensionTransparencyFlag = 3,
    WICGifGraphicControlExtensionDelay = 4,
    WICGifGraphicControlExtensionTransparentColorIndex = 5,
    WICGifGraphicControlExtensionProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICGifImageDescriptorProperties {
    WICGifImageDescriptorLeft = 1,
    WICGifImageDescriptorTop = 2,
    WICGifImageDescriptorWidth = 3,
    WICGifImageDescriptorHeight = 4,
    WICGifImageDescriptorLocalColorTableFlag = 5,
    WICGifImageDescriptorInterlaceFlag = 6,
    WICGifImageDescriptorSortFlag = 7,
    WICGifImageDescriptorLocalColorTableSize = 8,
    WICGifImageDescriptorProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICGifLogicalScreenDescriptorProperties {
    WICGifLogicalScreenSignature = 1,
    WICGifLogicalScreenDescriptorWidth = 2,
    WICGifLogicalScreenDescriptorHeight = 3,
    WICGifLogicalScreenDescriptorGlobalColorTableFlag = 4,
    WICGifLogicalScreenDescriptorColorResolution = 5,
    WICGifLogicalScreenDescriptorSortFlag = 6,
    WICGifLogicalScreenDescriptorGlobalColorTableSize = 7,
    WICGifLogicalScreenDescriptorBackgroundColorIndex = 8,
    WICGifLogicalScreenDescriptorPixelAspectRatio = 9,
    WICGifLogicalScreenDescriptorProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegChrominanceProperties {
    WICJpegChrominanceTable = 1,
    WICJpegChrominanceProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegCommentProperties {
    WICJpegCommentText = 1,
    WICJpegCommentProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegIndexingOptions {
    WICJpegIndexingOptionsGenerateOnDemand = 0,
    WICJpegIndexingOptionsGenerateOnLoad = 1,
    WICJpegIndexingOptions_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegLuminanceProperties {
    WICJpegLuminanceTable = 1,
    WICJpegLuminanceProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegScanType {
    WICJpegScanTypeInterleaved = 0,
    WICJpegScanTypePlanarComponents = 1,
    WICJpegScanTypeProgressive = 2,
    WICJpegScanType_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegTransferMatrix {
    WICJpegTransferMatrixIdentity = 0,
    WICJpegTransferMatrixBT601 = 1,
    WICJpegTransferMatrix_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICJpegYCrCbSubsamplingOption {
    WICJpegYCrCbSubsamplingDefault = 0,
    WICJpegYCrCbSubsampling420 = 1,
    WICJpegYCrCbSubsampling422 = 2,
    WICJpegYCrCbSubsampling444 = 3,
    WICJpegYCrCbSubsampling440 = 4,
    WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICNamedWhitePoint {
    WICWhitePointDefault = 1,
    WICWhitePointDaylight = 2,
    WICWhitePointCloudy = 4,
    WICWhitePointShade = 8,
    WICWhitePointTungsten = 16,
    WICWhitePointFluorescent = 32,
    WICWhitePointFlash = 64,
    WICWhitePointUnderwater = 128,
    WICWhitePointCustom = 256,
    WICWhitePointAutoWhiteBalance = 512,
    WICWhitePointAsShot = 1,
    WICNAMEDWHITEPOINT_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPixelFormatNumericRepresentation {
    WICPixelFormatNumericRepresentationUnspecified = 0,
    WICPixelFormatNumericRepresentationIndexed = 1,
    WICPixelFormatNumericRepresentationUnsignedInteger = 2,
    WICPixelFormatNumericRepresentationSignedInteger = 3,
    WICPixelFormatNumericRepresentationFixed = 4,
    WICPixelFormatNumericRepresentationFloat = 5,
    WICPixelFormatNumericRepresentation_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPlanarOptions {
    WICPlanarOptionsDefault = 0,
    WICPlanarOptionsPreserveSubsampling = 1,
    WICPLANAROPTIONS_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngBkgdProperties {
    WICPngBkgdBackgroundColor = 1,
    WICPngBkgdProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngChrmProperties {
    WICPngChrmWhitePointX = 1,
    WICPngChrmWhitePointY = 2,
    WICPngChrmRedX = 3,
    WICPngChrmRedY = 4,
    WICPngChrmGreenX = 5,
    WICPngChrmGreenY = 6,
    WICPngChrmBlueX = 7,
    WICPngChrmBlueY = 8,
    WICPngChrmProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngFilterOption {
    WICPngFilterUnspecified = 0,
    WICPngFilterNone = 1,
    WICPngFilterSub = 2,
    WICPngFilterUp = 3,
    WICPngFilterAverage = 4,
    WICPngFilterPaeth = 5,
    WICPngFilterAdaptive = 6,
    WICPNGFILTEROPTION_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngGamaProperties {
    WICPngGamaGamma = 1,
    WICPngGamaProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngHistProperties {
    WICPngHistFrequencies = 1,
    WICPngHistProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngIccpProperties {
    WICPngIccpProfileName = 1,
    WICPngIccpProfileData = 2,
    WICPngIccpProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngItxtProperties {
    WICPngItxtKeyword = 1,
    WICPngItxtCompressionFlag = 2,
    WICPngItxtLanguageTag = 3,
    WICPngItxtTranslatedKeyword = 4,
    WICPngItxtText = 5,
    WICPngItxtProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngSrgbProperties {
    WICPngSrgbRenderingIntent = 1,
    WICPngSrgbProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICPngTimeProperties {
    WICPngTimeYear = 1,
    WICPngTimeMonth = 2,
    WICPngTimeDay = 3,
    WICPngTimeHour = 4,
    WICPngTimeMinute = 5,
    WICPngTimeSecond = 6,
    WICPngTimeProperties_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICProgressNotification {
    WICProgressNotificationBegin = 65536,
    WICProgressNotificationEnd = 131072,
    WICProgressNotificationFrequent = 262144,
    WICProgressNotificationAll = 4294901760,
    WICPROGRESSNOTIFICATION_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICProgressOperation {
    WICProgressOperationCopyPixels = 1,
    WICProgressOperationWritePixels = 2,
    WICProgressOperationAll = 65535,
    WICPROGRESSOPERATION_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICRawCapabilities {
    WICRawCapabilityNotSupported = 0,
    WICRawCapabilityGetSupported = 1,
    WICRawCapabilityFullySupported = 2,
    WICRAWCAPABILITIES_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICRawParameterSet {
    WICAsShotParameterSet = 1,
    WICUserAdjustedParameterSet = 2,
    WICAutoAdjustedParameterSet = 3,
    WICRAWPARAMETERSET_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICRawRenderMode {
    WICRawRenderModeDraft = 1,
    WICRawRenderModeNormal = 2,
    WICRawRenderModeBestQuality = 3,
    WICRAWRENDERMODE_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICRawRotationCapabilities {
    WICRawRotationCapabilityNotSupported = 0,
    WICRawRotationCapabilityGetSupported = 1,
    WICRawRotationCapabilityNinetyDegreesSupported = 2,
    WICRawRotationCapabilityFullySupported = 3,
    WICRAWROTATIONCAPABILITIES_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICSectionAccessLevel {
    WICSectionAccessLevelRead = 1,
    WICSectionAccessLevelReadWrite = 3,
    WICSectionAccessLevel_FORCE_DWORD = 0x7FFFFFFF,
}}

ENUM!{ enum WICTiffCompressionOption {
    WICTiffCompressionDontCare = 0,
    WICTiffCompressionNone = 1,
    WICTiffCompressionCCITT3 = 2,
    WICTiffCompressionCCITT4 = 3,
    WICTiffCompressionLZW = 4,
    WICTiffCompressionRLE = 5,
    WICTiffCompressionZIP = 6,
    WICTiffCompressionLZWHDifferencing = 7,
    WICTIFFCOMPRESSIONOPTION_FORCE_DWORD = 0x7FFFFFFF,
}}

RIDL!(
interface IWICBitmapClipper(IWICBitmapClipperVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        &mut self, pISource: *mut ::IWICBitmapSource, prc: *const ::WICRect
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapCodecInfo(IWICBitmapCodecInfoVtbl): IWICComponentInfo(IWICComponentInfoVtbl) {
    fn GetContainerFormat(
        &mut self, pguidContainerFormat: *mut ::GUID
    ) -> ::HRESULT,
    fn GetPixelFormats(
        &mut self, cFormats: ::UINT, pguidPixelFormats: *mut ::GUID, pcActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetColorManagementVersion(
        &mut self, cchColorManagementVersion: ::UINT, wzColorManagementVersion: *mut ::WCHAR,
        pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetDeviceManufacturer(
        &mut self, cchDeviceManufacturer: ::UINT, wzDeviceManufacturer: *mut ::WCHAR,
        pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetDeviceModels(
        &mut self, cchDeviceModels: ::UINT, wzDeviceModels: *mut ::WCHAR, pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetMimeTypes(
        &mut self, cchMimeTypes: ::UINT, wzMimeTypes: *mut ::WCHAR, pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFileExtensions(
        &mut self, cchFileExtensions: ::UINT, wzFileExtensions: *mut ::WCHAR,
        pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn DoesSupportAnimation(
        &mut self, pfSupportAnimation: *mut ::BOOL
    ) -> ::HRESULT,
    fn DoesSupportChromakey(
        &mut self, pfSupportChromakey: *mut ::BOOL
    ) -> ::HRESULT,
    fn DoesSupportLossless(
        &mut self, pfSupportLossless: *mut ::BOOL
    ) -> ::HRESULT,
    fn DoesSupportMultiframe(
        &mut self, pfSupportMultiframe: *mut ::BOOL
    ) -> ::HRESULT,
    fn MatchesMimeType(
        &mut self, wzMimeType: ::LPCWSTR, pfMatches: *mut ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapCodecProgressNotification(IWICBitmapCodecProgressNotificationVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterProgressNotification(
        &mut self, pfnProgressNotification: ::PFNProgressNotification, pvData: ::LPVOID,
        dwProgressFlags: ::DWORD
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapDecoderInfo(IWICBitmapDecoderInfoVtbl): IWICBitmapCodecInfo(IWICBitmapCodecInfoVtbl) {
    fn GetPatterns(
        &mut self, cbSizePatterns: ::UINT, pPatterns: *mut ::WICBitmapPattern,
        pcPatterns: *mut ::UINT, pcbPatternsActual: *mut ::UINT
    ) -> ::HRESULT,
    fn MatchesPattern(
        &mut self, pIStream: *mut ::IStream, pfMatches: *mut ::BOOL
    ) -> ::HRESULT,
    fn CreateInstance(
        &mut self, ppIBitmapDecoder: *mut *mut ::IWICBitmapDecoder
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapDecoder(IWICBitmapDecoderVtbl): IUnknown(IUnknownVtbl) {
    fn QueryCapability(
        &mut self, pIStream: *mut ::IStream, pdwCapability: *mut ::DWORD
    ) -> ::HRESULT,
    fn Initialize(
        &mut self, pIStream: *mut ::IStream, cacheOptions: ::WICDecodeOptions
    ) -> ::HRESULT,
    fn GetContainerFormat(
        &mut self, pguidContainerFormat: *mut ::GUID
    ) -> ::HRESULT,
    fn GetDecoderInfo(
        &mut self, ppIDecoderInfo: *mut *mut ::IWICBitmapDecoderInfo
    ) -> ::HRESULT,
    fn CopyPalette(&mut self, pIPalette: *mut ::IWICPalette) -> ::HRESULT,
    fn GetMetadataQueryReader(
        &mut self, ppIMetadataQueryReader: *mut *mut ::IWICMetadataQueryReader
    ) -> ::HRESULT,
    fn GetPreview(
        &mut self, ppIBitmapSource: *mut *mut ::IWICBitmapSource
    ) -> ::HRESULT,
    fn GetColorContexts(
        &mut self, cCount: ::UINT, ppIColorContexts: *mut *mut ::IWICColorContext,
        pcActualCount: *mut ::UINT
    ) -> ::HRESULT,
    fn GetThumbnail(
        &mut self, ppIThumbnail: *mut *mut ::IWICBitmapSource
    ) -> ::HRESULT,
    fn GetFrameCount(&mut self, pCount: *mut ::UINT) -> ::HRESULT,
    fn GetFrame(
        &mut self, index: ::UINT, ppIBitmapFrame: *mut *mut ::IWICBitmapFrameDecode
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapEncoderInfo(IWICBitmapEncoderInfoVtbl): IWICBitmapCodecInfo(IWICBitmapCodecInfoVtbl) {
    fn CreateInstance(
        &mut self, ppIBitmapEncoder: *mut *mut ::IWICBitmapEncoder
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapEncoder(IWICBitmapEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        &mut self, pIStream: *mut ::IStream, cacheOption: ::WICBitmapEncoderCacheOption
    ) -> ::HRESULT,
    fn GetContainerFormat(
        &mut self, pguidContainerFormat: *mut ::GUID
    ) -> ::HRESULT,
    fn GetEncoderInfo(
        &mut self, ppIEncoderInfo: *mut *mut ::IWICBitmapEncoderInfo
    ) -> ::HRESULT,
    fn SetColorContexts(
        &mut self, cCount: ::UINT, ppIColorContext: *mut *mut ::IWICColorContext
    ) -> ::HRESULT,
    fn SetPalette(&mut self, pIPalette: *mut ::IWICPalette) -> ::HRESULT,
    fn SetThumbnail(
        &mut self, pIThumbnail: *mut ::IWICBitmapSource
    ) -> ::HRESULT,
    fn SetPreview(&mut self, pIPreview: *mut ::IWICBitmapSource) -> ::HRESULT,
    fn CreateNewFrame(
        &mut self, ppIFrameEncode: *mut *mut ::IWICBitmapFrameEncode,
        ppIEncoderOptions: *mut *mut ::IPropertyBag2
    ) -> ::HRESULT,
    fn Commit(&mut self) -> ::HRESULT,
    fn GetMetadataQueryWriter(
        &mut self, ppIMetadataQueryWriter: *mut *mut ::IWICMetadataQueryWriter
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapFlipRotator(IWICBitmapFlipRotatorVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        &mut self, pISource: *mut ::IWICBitmapSource, options: ::WICBitmapTransformOptions
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapFrameDecode(IWICBitmapFrameDecodeVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn GetMetadataQueryReader(
        &mut self, ppIMetadataQueryReader: *mut *mut ::IWICMetadataQueryReader
    ) -> ::HRESULT,
    fn GetColorContexts(
        &mut self, cCount: ::UINT, ppIColorContexts: *mut *mut ::IWICColorContext,
        pcActualCount: *mut ::UINT
    ) -> ::HRESULT,
    fn GetThumbnail(
        &mut self, ppIThumbnail: *mut *mut ::IWICBitmapSource
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapFrameEncode(IWICBitmapFrameEncodeVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        &mut self, pIEncoderOptions: *mut ::IPropertyBag2
    ) -> ::HRESULT,
    fn SetSize(&mut self, uiWidth: ::UINT, uiHeight: ::UINT) -> ::HRESULT,
    fn SetResolution(
        &mut self, dpiX: ::c_double, dpiY: ::c_double
    ) -> ::HRESULT,
    fn SetPixelFormat(
        &mut self, pPixelFormat: *mut ::WICPixelFormatGUID
    ) -> ::HRESULT,
    fn SetColorContexts(
        &mut self, cCount: ::UINT, ppIColorContext: *mut *mut ::IWICColorContext
    ) -> ::HRESULT,
    fn SetPalette(&mut self, pIPalette: *mut ::IWICPalette) -> ::HRESULT,
    fn SetThumbnail(
        &mut self, pIThumbnail: *mut ::IWICBitmapSource
    ) -> ::HRESULT,
    fn WritePixels(
        &mut self, lineCount: ::UINT, cbStride: ::UINT, cbBufferSize: ::UINT, pbPixels: *mut ::BYTE
    ) -> ::HRESULT,
    fn WriteSource(
        &mut self, pIBitmapSource: *mut ::IWICBitmapSource, prc: *mut ::WICRect
    ) -> ::HRESULT,
    fn Commit(&mut self) -> ::HRESULT,
    fn GetMetadataQueryWriter(
        &mut self, ppIMetadataQueryWriter: *mut *mut ::IWICMetadataQueryWriter
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapLock(IWICBitmapLockVtbl): IUnknown(IUnknownVtbl) {
    fn GetSize(
        &mut self, puiWidth: *mut ::UINT, puiHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn GetStride(&mut self, pcbStride: *mut ::UINT) -> ::HRESULT,
    fn GetDataPointer(
        &mut self, pcbBufferSize: *mut ::UINT, ppbData: *mut ::WICInProcPointer
    ) -> ::HRESULT,
    fn GetPixelFormat(
        &mut self, pPixelFormat: *mut ::WICPixelFormatGUID
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapScaler(IWICBitmapScalerVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        &mut self, pISource: *mut ::IWICBitmapSource, uiWidth: ::UINT, uiHeight: ::UINT,
        mode: ::WICBitmapInterpolationMode
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapSourceTransform(IWICBitmapSourceTransformVtbl): IUnknown(IUnknownVtbl) {
    fn CopyPixels(
        &mut self, prc: *const ::WICRect, uiWidth: ::UINT, uiHeight: ::UINT,
        pguidDstFormat: *mut ::WICPixelFormatGUID, dstTransform: ::WICBitmapTransformOptions,
        nStride: ::UINT, cbBufferSize: ::UINT, pbBuffer: *mut ::BYTE
    ) -> ::HRESULT,
    fn GetClosestSize(
        &mut self, puiWidth: *mut ::UINT, puiHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn GetClosestPixelFormat(
        &mut self, pguidDstFormat: *mut ::WICPixelFormatGUID
    ) -> ::HRESULT,
    fn DoesSupportTransform(
        &mut self, dstTransform: ::WICBitmapTransformOptions, pfIsSupported: *mut ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmapSource(IWICBitmapSourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetSize(
        &mut self, puiWidth: *mut ::UINT, puiHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn GetPixelFormat(
        &mut self, pPixelFormat: *mut ::WICPixelFormatGUID
    ) -> ::HRESULT,
    fn GetResolution(
        &mut self, pDpiX: *mut ::c_double, pDpiY: *mut ::c_double
    ) -> ::HRESULT,
    fn CopyPalette(&mut self, pIPalette: *mut ::IWICPalette) -> ::HRESULT,
    fn CopyPixels(
        &mut self, prc: *const ::WICRect, cbStride: ::UINT, cbBufferSize: ::UINT,
        pbBuffer: *mut ::BYTE
    ) -> ::HRESULT
});

RIDL!(
interface IWICBitmap(IWICBitmapVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Lock(
        &mut self, prcLock: *const ::WICRect, flags: ::DWORD, ppILock: *mut *mut ::IWICBitmapLock
    ) -> ::HRESULT,
    fn SetPalette(&mut self, pIPalette: *mut ::IWICPalette) -> ::HRESULT,
    fn SetResolution(&mut self, dpiX: ::c_double, dpiY: ::c_double) -> ::HRESULT
});

RIDL!(
interface IWICColorContext(IWICColorContextVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeFromFilename(&mut self, wzFilename: ::LPCWSTR) -> ::HRESULT,
    fn InitializeFromMemory(
        &mut self, pbBuffer: *const ::BYTE, cbBufferSize: ::UINT
    ) -> ::HRESULT,
    fn InitializeFromExifColorSpace(&mut self, value: ::UINT) -> ::HRESULT,
    fn GetType(&mut self, pType: *mut ::WICColorContextType) -> ::HRESULT,
    fn GetProfileBytes(
        &mut self, cbBuffer: ::UINT, pbBuffer: *mut ::BYTE, pcbActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetExifColorSpace(&mut self, pValue: *mut ::UINT) -> ::HRESULT
});

RIDL!(
interface IWICColorTransform(IWICColorTransformVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        &mut self, pIBitmapSource: *mut ::IWICBitmapSource,
        pIContextSource: *mut ::IWICColorContext, pIContextDest: *mut ::IWICColorContext,
        pixelFmtDest: ::REFWICPixelFormatGUID
    ) -> ::HRESULT
});

RIDL!(
interface IWICComponentInfo(IWICComponentInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetComponentType(&mut self, pType: *mut ::WICComponentType) -> ::HRESULT,
    fn GetCLSID(&mut self, pclsid: *mut ::CLSID) -> ::HRESULT,
    fn GetSigningStatus(&mut self, pStatus: *mut ::DWORD) -> ::HRESULT,
    fn GetAuthor(
        &mut self, cchAuthor: ::UINT, wzAuthor: *mut ::WCHAR, pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetVendorGUID(&mut self, pguidVendor: *mut ::GUID) -> ::HRESULT,
    fn GetVersion(
        &mut self, cchVersion: ::UINT, wzVersion: *mut ::WCHAR, pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetSpecVersion(
        &mut self, cchSpecVersion: ::UINT, wzSpecVersion: *mut ::WCHAR, pcchActual: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFriendlyName(
        &mut self, cchFriendlyName: ::UINT, wzFriendlyName: *mut ::WCHAR, pcchActual: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IWICDdsDecoder(IWICDdsDecoderVtbl): IUnknown(IUnknownVtbl) {
    fn GetParameters(
        &mut self, pParameters: *mut ::WICDdsParameters
    ) -> ::HRESULT,
    fn GetFrame(
        &mut self, arrayIndex: ::UINT, mipLevel: ::UINT, sliceIndex: ::UINT,
        ppIBitmapFrame: *mut *mut ::IWICBitmapFrameDecode
    ) -> ::HRESULT
});

RIDL!(
interface IWICDdsEncoder(IWICDdsEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn SetParameters(
        &mut self, pParameters: *mut ::WICDdsParameters
    ) -> ::HRESULT,
    fn GetParameters(
        &mut self, pParameters: *mut ::WICDdsParameters
    ) -> ::HRESULT,
    fn CreateNewFrame(
        &mut self, ppIFrameEncode: *mut *mut ::IWICBitmapFrameEncode, pArrayIndex: *mut ::UINT,
        pMipLevel: *mut ::UINT, pSliceIndex: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IWICDdsFrameDecode(IWICDdsFrameDecodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetSizeInBlocks(
        &mut self, pWidthInBlocks: *mut ::UINT, pHeightInBlocks: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFormatInfo(
        &mut self, pFormatInfo: *mut ::WICDdsFormatInfo
    ) -> ::HRESULT,
    fn CopyBlocks(
        &mut self, prcBoundsInBlocks: *const ::WICRect, cbStride: ::UINT, cbBufferSize: ::UINT,
        pbBuffer: *mut ::BYTE
    ) -> ::HRESULT
});

RIDL!(
interface IWICDevelopRawNotificationCallback(IWICDevelopRawNotificationCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Notify(&mut self, NotificationMask: ::UINT) -> ::HRESULT
});

RIDL!(
interface IWICDevelopRaw(IWICDevelopRawVtbl): IWICBitmapFrameDecode(IWICBitmapFrameDecodeVtbl) {
    fn QueryRawCapabilitiesInfo(
        &mut self, pInfo: *mut ::WICRawCapabilitiesInfo
    ) -> ::HRESULT,
    fn LoadParameterSet(
        &mut self, ParameterSet: ::WICRawParameterSet
    ) -> ::HRESULT,
    fn GetCurrentParameterSet(
        &mut self, ppCurrentParameterSet: *mut *mut ::IPropertyBag2
    ) -> ::HRESULT,
    fn SetExposureCompensation(&mut self, ev: ::c_double) -> ::HRESULT,
    fn GetExposureCompensation(&mut self, pEV: *mut ::c_double) -> ::HRESULT,
    fn SetWhitePointRGB(
        &mut self, Red: ::UINT, Green: ::UINT, Blue: ::UINT
    ) -> ::HRESULT,
    fn GetWhitePointRGB(
        &mut self, pRed: *mut ::UINT, pGreen: *mut ::UINT, pBlue: *mut ::UINT
    ) -> ::HRESULT,
    fn SetNamedWhitePoint(
        &mut self, WhitePoint: ::WICNamedWhitePoint
    ) -> ::HRESULT,
    fn GetNamedWhitePoint(
        &mut self, pWhitePoint: *mut ::WICNamedWhitePoint
    ) -> ::HRESULT,
    fn SetWhitePointKelvin(&mut self, WhitePointKelvin: ::UINT) -> ::HRESULT,
    fn GetWhitePointKelvin(
        &mut self, pWhitePointKelvin: *mut ::UINT
    ) -> ::HRESULT,
    fn GetKelvinRangeInfo(
        &mut self, pMinKelvinTemp: *mut ::UINT, pMaxKelvinTemp: *mut ::UINT,
        pKelvinTempStepValue: *mut ::UINT
    ) -> ::HRESULT,
    fn SetContrast(&mut self, Contrast: ::c_double) -> ::HRESULT,
    fn GetContrast(&mut self, pContrast: *mut ::c_double) -> ::HRESULT,
    fn SetGamma(&mut self, Gamma: ::c_double) -> ::HRESULT,
    fn GetGamma(&mut self, pGamma: *mut ::c_double) -> ::HRESULT,
    fn SetSharpness(&mut self, Sharpness: ::c_double) -> ::HRESULT,
    fn GetSharpness(&mut self, pSharpness: *mut ::c_double) -> ::HRESULT,
    fn SetSaturation(&mut self, Saturation: ::c_double) -> ::HRESULT,
    fn GetSaturation(&mut self, pSaturation: *mut ::c_double) -> ::HRESULT,
    fn SetTint(&mut self, Tint: ::c_double) -> ::HRESULT,
    fn GetTint(&mut self, pTint: *mut ::c_double) -> ::HRESULT,
    fn SetNoiseReduction(&mut self, NoiseReduction: ::c_double) -> ::HRESULT,
    fn GetNoiseReduction(
        &mut self, pNoiseReduction: *mut ::c_double
    ) -> ::HRESULT,
    fn SetDestinationColorContext(
        &mut self, pColorContext: *mut ::IWICColorContext
    ) -> ::HRESULT,
    fn SetToneCurve(
        &mut self, cbToneCurveSize: ::UINT, pToneCurve: *const ::WICRawToneCurve
    ) -> ::HRESULT,
    fn GetToneCurve(
        &mut self, cbToneCurveBufferSize: ::UINT, pToneCurve: *mut ::WICRawToneCurve,
        pcbActualToneCurveBufferSize: *mut ::UINT
    ) -> ::HRESULT,
    fn SetRotation(&mut self, Rotation: ::c_double) -> ::HRESULT,
    fn GetRotation(&mut self, pRotation: *mut ::c_double) -> ::HRESULT,
    fn SetRenderMode(&mut self, RenderMode: ::WICRawRenderMode) -> ::HRESULT,
    fn GetRenderMode(
        &mut self, pRenderMode: *mut ::WICRawRenderMode
    ) -> ::HRESULT,
    fn SetNotificationCallback(
        &mut self, pCallback: *mut ::IWICDevelopRawNotificationCallback
    ) -> ::HRESULT
});

RIDL!(
interface IWICEnumMetadataItem(IWICEnumMetadataItemVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        &mut self, celt: ::ULONG, rgeltSchema: *mut ::PROPVARIANT, rgeltId: *mut ::PROPVARIANT,
        rgeltValue: *mut ::PROPVARIANT, pceltFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(&mut self, celt: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(
        &mut self, ppIEnumMetadataItem: *mut *mut ::IWICEnumMetadataItem
    ) -> ::HRESULT
});

RIDL!(
interface IWICFastMetadataEncoder(IWICFastMetadataEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn Commit(&mut self) -> ::HRESULT,
    fn GetMetadataQueryWriter(
        &mut self, ppIMetadataQueryWriter: *mut *mut ::IWICMetadataQueryWriter
    ) -> ::HRESULT
});

RIDL!(
interface IWICFormatConverterInfo(IWICFormatConverterInfoVtbl): IWICComponentInfo(IWICComponentInfoVtbl) {
    fn GetPixelFormats(
        &mut self, cFormats: ::UINT, pPixelFormatGUIDs: *mut ::WICPixelFormatGUID,
        pcActual: *mut ::UINT
    ) -> ::HRESULT,
    fn CreateInstance(
        &mut self, ppIConverter: *mut *mut ::IWICFormatConverter
    ) -> ::HRESULT
});

RIDL!(
interface IWICFormatConverter(IWICFormatConverterVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        &mut self, pISource: *mut ::IWICBitmapSource, dstFormat: ::REFWICPixelFormatGUID,
        dither: ::WICBitmapDitherType, pIPalette: *mut ::IWICPalette,
        alphaThresholdPercent: ::c_double, paletteTranslate: ::WICBitmapPaletteType
    ) -> ::HRESULT,
    fn CanConvert(
        &mut self, srcPixelFormat: ::REFWICPixelFormatGUID,
        dstPixelFormat: ::REFWICPixelFormatGUID, pfCanConvert: *mut ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IWICImageEncoder(IWICImageEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn WriteFrame(
        &mut self, pImage: *mut ::ID2D1Image, pFrameEncode: *mut ::IWICBitmapFrameEncode,
        pImageParameters: *const ::WICImageParameters
    ) -> ::HRESULT,
    fn WriteFrameThumbnail(
        &mut self, pImage: *mut ::ID2D1Image, pFrameEncode: *mut ::IWICBitmapFrameEncode,
        pImageParameters: *const ::WICImageParameters
    ) -> ::HRESULT,
    fn WriteThumbnail(
        &mut self, pImage: *mut ::ID2D1Image, pEncoder: *mut ::IWICBitmapEncoder,
        pImageParameters: *const ::WICImageParameters
    ) -> ::HRESULT
});

RIDL!(
interface IWICImagingFactory2(IWICImagingFactory2Vtbl): IWICImagingFactory(IWICImagingFactoryVtbl) {
    fn CreateImageEncoder(
        &mut self, pD2DDevice: *mut ::ID2D1Device, ppWICImageEncoder: *mut *mut ::IWICImageEncoder
    ) -> ::HRESULT
});

RIDL!(
interface IWICImagingFactory(IWICImagingFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDecoderFromFilename(
        &mut self, wzFilename: ::LPCWSTR, pguidVendor: *const ::GUID, dwDesiredAccess: ::DWORD,
        metadataOptions: ::WICDecodeOptions, ppIDecoder: *mut *mut ::IWICBitmapDecoder
    ) -> ::HRESULT,
    fn CreateDecoderFromStream(
        &mut self, pIStream: *mut ::IStream, pguidVendor: *const ::GUID,
        metadataOptions: ::WICDecodeOptions, ppIDecoder: *mut *mut ::IWICBitmapDecoder
    ) -> ::HRESULT,
    fn CreateDecoderFromFileHandle(
        &mut self, hFile: ::ULONG_PTR, pguidVendor: *const ::GUID,
        metadataOptions: ::WICDecodeOptions, ppIDecoder: *mut *mut ::IWICBitmapDecoder
    ) -> ::HRESULT,
    fn CreateComponentInfo(
        &mut self, clsidComponent: *const ::IID, ppIInfo: *mut *mut ::IWICComponentInfo
    ) -> ::HRESULT,
    fn CreateDecoder(
        &mut self, guidContainerFormat: *const ::GUID, pguidVendor: *const ::GUID,
        ppIDecoder: *mut *mut ::IWICBitmapDecoder
    ) -> ::HRESULT,
    fn CreateEncoder(
        &mut self, guidContainerFormat: *const ::GUID, pguidVendor: *const ::GUID,
        ppIEncoder: *mut *mut ::IWICBitmapEncoder
    ) -> ::HRESULT,
    fn CreatePalette(
        &mut self, ppIPalette: *mut *mut ::IWICPalette
    ) -> ::HRESULT,
    fn CreateFormatConverter(
        &mut self, ppIFormatConverter: *mut *mut ::IWICFormatConverter
    ) -> ::HRESULT,
    fn CreateBitmapScaler(
        &mut self, ppIBitmapScaler: *mut *mut ::IWICBitmapScaler
    ) -> ::HRESULT,
    fn CreateBitmapClipper(
        &mut self, ppIBitmapClipper: *mut *mut ::IWICBitmapClipper
    ) -> ::HRESULT,
    fn CreateBitmapFlipRotator(
        &mut self, ppIBitmapFlipRotator: *mut *mut ::IWICBitmapFlipRotator
    ) -> ::HRESULT,
    fn CreateStream(
        &mut self, ppIWICStream: *mut *mut ::IWICStream
    ) -> ::HRESULT,
    fn CreateColorContext(
        &mut self, ppIWICColorContext: *mut *mut ::IWICColorContext
    ) -> ::HRESULT,
    fn CreateColorTransformer(
        &mut self, ppIWICColorTransform: *mut *mut ::IWICColorTransform
    ) -> ::HRESULT,
    fn CreateBitmap(
        &mut self, uiWidth: ::UINT, uiHeight: ::UINT, pixelFormat: ::REFWICPixelFormatGUID,
        option: ::WICBitmapCreateCacheOption, ppIBitmap: *mut *mut ::IWICBitmap
    ) -> ::HRESULT,
    fn CreateBitmapFromSource(
        &mut self, pIBitmapSource: *mut ::IWICBitmapSource, option: ::WICBitmapCreateCacheOption,
        ppIBitmap: *mut *mut ::IWICBitmap
    ) -> ::HRESULT,
    fn CreateBitmapFromSourceRect(
        &mut self, pIBitmapSource: *mut ::IWICBitmapSource, x: ::UINT, y: ::UINT,
        width: ::UINT, height: ::UINT, ppIBitmap: *mut *mut ::IWICBitmap
    ) -> ::HRESULT,
    fn CreateBitmapFromMemory(
        &mut self, uiWidth: ::UINT, uiHeight: ::UINT, pixelFormat: ::REFWICPixelFormatGUID,
        cbStride: ::UINT, cbBufferSize: ::UINT, pbBuffer: *mut ::BYTE,
        ppIBitmap: *mut *mut ::IWICBitmap
    ) -> ::HRESULT,
    fn CreateBitmapFromHBITMAP(
        &mut self, hBitmap: ::HBITMAP, hPalette: ::HPALETTE,
        options: ::WICBitmapAlphaChannelOption, ppIBitmap: *mut *mut ::IWICBitmap
    ) -> ::HRESULT,
    fn CreateBitmapFromHICON(
        &mut self, hIcon: ::HICON, ppIBitmap: *mut *mut ::IWICBitmap
    ) -> ::HRESULT,
    fn CreateComponentEnumerator(
        &mut self, componentTypes: ::DWORD, options: ::DWORD,
        ppIEnumUnknown: *mut *mut ::IEnumUnknown
    ) -> ::HRESULT,
    fn CreateFastMetadataEncoderFromDecoder(
        &mut self, pIDecoder: *mut ::IWICBitmapDecoder,
        ppIFastEncoder: *mut *mut ::IWICFastMetadataEncoder
    ) -> ::HRESULT,
    fn CreateFastMetadataEncoderFromFrameDecode(
        &mut self, pIFrameDecoder: *mut ::IWICBitmapFrameDecode,
        ppIFastEncoder: *mut *mut ::IWICFastMetadataEncoder
    ) -> ::HRESULT,
    fn CreateQueryWriter(
        &mut self, guidMetadataFormat: *const ::GUID, pguidVendor: *const ::GUID,
        ppIQueryWriter: *mut *mut ::IWICMetadataQueryWriter
    ) -> ::HRESULT,
    fn CreateQueryWriterFromReader(
        &mut self, pIQueryReader: *mut ::IWICMetadataQueryReader, pguidVendor: *const ::GUID,
        ppIQueryWriter: *mut *mut ::IWICMetadataQueryWriter
    ) -> ::HRESULT
});

RIDL!(
interface IWICJpegFrameDecode(IWICJpegFrameDecodeVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportIndexing(
        &mut self, pfIndexingSupported: *mut ::BOOL
    ) -> ::HRESULT,
    fn SetIndexing(
        &mut self, options: ::WICJpegIndexingOptions, horizontalIntervalSize: ::UINT
    ) -> ::HRESULT,
    fn ClearIndexing(&mut self) -> ::HRESULT,
    fn GetAcHuffmanTable(
        &mut self, scanIndex: ::UINT, tableIndex: ::UINT,
        pAcHuffmanTable: *mut ::DXGI_JPEG_AC_HUFFMAN_TABLE
    ) -> ::HRESULT,
    fn GetDcHuffmanTable(
        &mut self, scanIndex: ::UINT, tableIndex: ::UINT,
        pDcHuffmanTable: *mut ::DXGI_JPEG_DC_HUFFMAN_TABLE
    ) -> ::HRESULT,
    fn GetQuantizationTable(
        &mut self, scanIndex: ::UINT, tableIndex: ::UINT,
        pQuantizationTable: *mut ::DXGI_JPEG_QUANTIZATION_TABLE
    ) -> ::HRESULT,
    fn GetFrameHeader(
        &mut self, pFrameHeader: *mut ::WICJpegFrameHeader
    ) -> ::HRESULT,
    fn GetScanHeader(
        &mut self, scanIndex: ::UINT, pScanHeader: *mut ::WICJpegScanHeader
    ) -> ::HRESULT,
    fn CopyScan(
        &mut self, scanIndex: ::UINT, scanOffset: ::UINT, cbScanData: ::UINT,
        pbScanData: *mut ::BYTE, pcbScanDataActual: *mut ::UINT
    ) -> ::HRESULT,
    fn CopyMinimalStream(
        &mut self, streamOffset: ::UINT, cbStreamData: ::UINT, pbStreamData: *mut ::BYTE,
        pcbStreamDataActual: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IWICJpegFrameEncode(IWICJpegFrameEncodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetAcHuffmanTable(
        &mut self, scanIndex: ::UINT, tableIndex: ::UINT,
        pAcHuffmanTable: *mut ::DXGI_JPEG_AC_HUFFMAN_TABLE
    ) -> ::HRESULT,
    fn GetDcHuffmanTable(
        &mut self, scanIndex: ::UINT, tableIndex: ::UINT,
        pDcHuffmanTable: *mut ::DXGI_JPEG_DC_HUFFMAN_TABLE
    ) -> ::HRESULT,
    fn GetQuantizationTable(
        &mut self, scanIndex: ::UINT, tableIndex: ::UINT,
        pQuantizationTable: *mut ::DXGI_JPEG_QUANTIZATION_TABLE
    ) -> ::HRESULT,
    fn WriteScan(
        &mut self, cbScanData: ::UINT, pbScanData: *const ::BYTE
    ) -> ::HRESULT
});

RIDL!(
interface IWICMetadataQueryReader(IWICMetadataQueryReaderVtbl): IUnknown(IUnknownVtbl) {
    fn GetContainerFormat(
        &mut self, pguidContainerFormat: *mut ::GUID
    ) -> ::HRESULT,
    fn GetLocation(
        &mut self, cchMaxLength: ::UINT, wzNamespace: *mut ::WCHAR, pcchActualLength: *mut ::UINT
    ) -> ::HRESULT,
    fn GetMetadataByName(
        &mut self, wzName: ::LPCWSTR, pvarValue: *mut ::PROPVARIANT
    ) -> ::HRESULT,
    fn GetEnumerator(
        &mut self, ppIEnumString: *mut *mut ::IEnumString
    ) -> ::HRESULT
});

RIDL!(
interface IWICMetadataQueryWriter(IWICMetadataQueryWriterVtbl): IWICMetadataQueryReader(IWICMetadataQueryReaderVtbl) {
    fn SetMetadataByName(
        &mut self, wzName: ::LPCWSTR, pvarValue: *const ::PROPVARIANT
    ) -> ::HRESULT,
    fn RemoveMetadataByName(&mut self, wzName: ::LPCWSTR) -> ::HRESULT
});

RIDL!(
interface IWICPalette(IWICPaletteVtbl): IUnknown(IUnknownVtbl) {
    fn InitializePredefined(
        &mut self, ePaletteType: ::WICBitmapPaletteType, fAddTransparentColor: ::BOOL
    ) -> ::HRESULT,
    fn InitializeCustom(
        &mut self, pColors: *mut ::WICColor, cCount: ::UINT
    ) -> ::HRESULT,
    fn InitializeFromBitmap(
        &mut self, pISurface: *mut ::IWICBitmapSource, cCount: ::UINT, fAddTransparentColor: ::BOOL
    ) -> ::HRESULT,
    fn InitializeFromPalette(
        &mut self, pIPalette: *mut ::IWICPalette
    ) -> ::HRESULT,
    fn GetType(
        &mut self, pePaletteType: *mut ::WICBitmapPaletteType
    ) -> ::HRESULT,
    fn GetColorCount(&mut self, pcCount: *mut ::UINT) -> ::HRESULT,
    fn GetColors(
        &mut self, cCount: ::UINT, pColors: *mut ::WICColor, pcActualColors: *mut ::UINT
    ) -> ::HRESULT,
    fn IsBlackWhite(&mut self, pfIsBlackWhite: *mut ::BOOL) -> ::HRESULT,
    fn IsGrayscale(&mut self, pfIsGrayscale: *mut ::BOOL) -> ::HRESULT,
    fn HasAlpha(&mut self, pfHasAlpha: *mut ::BOOL) -> ::HRESULT
});

RIDL!(
interface IWICPixelFormatInfo2(IWICPixelFormatInfo2Vtbl): IWICPixelFormatInfo(IWICPixelFormatInfoVtbl) {
    fn SupportsTransparency(
        &mut self, pfSupportsTransparency: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetNumericRepresentation(
        &mut self, pNumericRepresentation: *mut ::WICPixelFormatNumericRepresentation
    ) -> ::HRESULT
});

RIDL!(
interface IWICPixelFormatInfo(IWICPixelFormatInfoVtbl): IWICComponentInfo(IWICComponentInfoVtbl) {
    fn GetFormatGUID(&mut self, pFormat: *mut ::GUID) -> ::HRESULT,
    fn GetColorContext(
        &mut self, ppIColorContext: *mut *mut ::IWICColorContext
    ) -> ::HRESULT,
    fn GetBitsPerPixel(&mut self, puiBitsPerPixel: *mut ::UINT) -> ::HRESULT,
    fn GetChannelCount(&mut self, puiChannelCount: *mut ::UINT) -> ::HRESULT,
    fn GetChannelMask(
        &mut self, uiChannelIndex: ::UINT, cbMaskBuffer: ::UINT, pbMaskBuffer: *mut ::BYTE,
        pcbActual: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IWICPlanarBitmapFrameEncode(IWICPlanarBitmapFrameEncodeVtbl): IUnknown(IUnknownVtbl) {
    fn WritePixels(
        &mut self, lineCount: ::UINT, pPlanes: *mut ::WICBitmapPlane, cPlanes: ::UINT
    ) -> ::HRESULT,
    fn WriteSource(
        &mut self, ppPlanes: *mut *mut ::IWICBitmapSource, cPlanes: ::UINT,
        prcSource: *mut ::WICRect
    ) -> ::HRESULT
});

RIDL!(
interface IWICPlanarBitmapSourceTransform(IWICPlanarBitmapSourceTransformVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportTransform(
        &mut self, puiWidth: *mut ::UINT, puiHeight: *mut ::UINT,
        dstTransform: ::WICBitmapTransformOptions, dstPlanarOptions: ::WICPlanarOptions,
        pguidDstFormats: *const ::WICPixelFormatGUID,
        pPlaneDescriptions: *mut ::WICBitmapPlaneDescription, cPlanes: ::UINT,
        pfIsSupported: *mut ::BOOL
    ) -> ::HRESULT,
    fn CopyPixels(
        &mut self, prcSource: *const ::WICRect, uiWidth: ::UINT, uiHeight: ::UINT,
        dstTransform: ::WICBitmapTransformOptions, dstPlanarOptions: ::WICPlanarOptions,
        pDstPlanes: *const ::WICBitmapPlane, cPlanes: ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IWICPlanarFormatConverter(IWICPlanarFormatConverterVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        &mut self, ppPlanes: *mut *mut ::IWICBitmapSource, cPlanes: ::UINT,
        dstFormat: ::REFWICPixelFormatGUID, dither: ::WICBitmapDitherType,
        pIPalette: *mut ::IWICPalette, alphaThresholdPercent: ::c_double,
        paletteTranslate: ::WICBitmapPaletteType
    ) -> ::HRESULT,
    fn CanConvert(
        &mut self, pSrcPixelFormats: *const ::WICPixelFormatGUID, cSrcPlanes: ::UINT,
        dstPixelFormat: ::REFWICPixelFormatGUID, pfCanConvert: *mut ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IWICProgressCallback(IWICProgressCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Notify(
        &mut self, uFrameNum: ::ULONG, operation: ::WICProgressOperation, dblProgress: ::c_double
    ) -> ::HRESULT
});

RIDL!(
interface IWICProgressiveLevelControl(IWICProgressiveLevelControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetLevelCount(&mut self, pcLevels: *mut ::UINT) -> ::HRESULT,
    fn GetCurrentLevel(&mut self, pnLevel: *mut ::UINT) -> ::HRESULT,
    fn SetCurrentLevel(&mut self, nLevel: ::UINT) -> ::HRESULT
});

RIDL!(
interface IWICStream(IWICStreamVtbl): IStream(IStreamVtbl) {
    fn InitializeFromIStream(&mut self, pIStream: *mut ::IStream) -> ::HRESULT,
    fn InitializeFromFilename(
        &mut self, wzFileName: ::LPCWSTR, dwDesiredAccess: ::DWORD
    ) -> ::HRESULT,
    fn InitializeFromMemory(
        &mut self, pbBuffer: ::WICInProcPointer, cbBufferSize: ::DWORD
    ) -> ::HRESULT,
    fn InitializeFromIStreamRegion(
        &mut self, pIStream: *mut ::IStream, ulOffset: ::ULARGE_INTEGER,
        ulMaxSize: ::ULARGE_INTEGER
    ) -> ::HRESULT
});

STRUCT!{struct WICBitmapPattern {
    Position: ::ULARGE_INTEGER,
    Length: ::ULONG,
    Pattern: *mut ::BYTE,
    Mask: *mut ::BYTE,
    EndOfStream: ::BOOL,
}}

STRUCT!{struct WICBitmapPlane {
    Format: ::WICPixelFormatGUID,
    pbBuffer: *mut ::BYTE,
    cbStride: ::UINT,
    cbBufferSize: ::UINT,
}}

STRUCT!{struct WICBitmapPlaneDescription {
    Format: ::WICPixelFormatGUID,
    Width: ::UINT,
    Height: ::UINT,
}}

STRUCT!{struct WICDdsFormatInfo {
    DxgiFormat: ::DXGI_FORMAT,
    BytesPerBlock: ::UINT,
    BlockWidth: ::UINT,
    BlockHeight: ::UINT,
}}

STRUCT!{struct WICDdsParameters {
    Width: ::UINT,
    Height: ::UINT,
    Depth: ::UINT,
    MipLevels: ::UINT,
    ArraySize: ::UINT,
    DxgiFormat: ::DXGI_FORMAT,
    Dimension: ::WICDdsDimension,
    AlphaMode: ::WICDdsAlphaMode,
}}

STRUCT!{struct WICImageParameters {
    PixelFormat: ::D2D1_PIXEL_FORMAT,
    DpiX: ::FLOAT,
    DpiY: ::FLOAT,
    Top: ::FLOAT,
    Left: ::FLOAT,
    PixelWidth: ::UINT32,
    PixelHeight: ::UINT32,
}}

STRUCT!{struct WICJpegFrameHeader {
    Width: ::UINT,
    Height: ::UINT,
    TransferMatrix: ::WICJpegTransferMatrix,
    ScanType: ::WICJpegScanType,
    cComponents: ::UINT,
    ComponentIdentifiers: ::DWORD,
    SampleFactors: ::DWORD,
    QuantizationTableIndices: ::DWORD,
}}

STRUCT!{struct WICJpegScanHeader {
    cComponents: ::UINT,
    RestartInterval: ::UINT,
    ComponentSelectors: ::DWORD,
    HuffmanTableIndices: ::DWORD,
    StartSpectralSelection: ::BYTE,
    EndSpectralSelection: ::BYTE,
    SuccessiveApproximationHigh: ::BYTE,
    SuccessiveApproximationLow: ::BYTE,
}}

STRUCT!{struct WICRawCapabilitiesInfo {
    cbSize: ::UINT,
    CodecMajorVersion: ::UINT,
    CodecMinorVersion: ::UINT,
    ExposureCompensationSupport: ::WICRawCapabilities,
    ContrastSupport: ::WICRawCapabilities,
    RGBWhitePointSupport: ::WICRawCapabilities,
    NamedWhitePointSupport: ::WICRawCapabilities,
    NamedWhitePointSupportMask: ::UINT,
    KelvinWhitePointSupport: ::WICRawCapabilities,
    GammaSupport: ::WICRawCapabilities,
    TintSupport: ::WICRawCapabilities,
    SaturationSupport: ::WICRawCapabilities,
    SharpnessSupport: ::WICRawCapabilities,
    NoiseReductionSupport: ::WICRawCapabilities,
    DestinationColorProfileSupport: ::WICRawCapabilities,
    ToneCurveSupport: ::WICRawCapabilities,
    RotationSupport: ::WICRawRotationCapabilities,
    RenderModeSupport: ::WICRawCapabilities,
}}

STRUCT!{struct WICRawToneCurve {
    cPoints: ::UINT,
    aPoints: [::WICRawToneCurvePoint; 1],
}}

STRUCT!{struct WICRawToneCurvePoint {
    Input: ::c_double,
    Output: ::c_double,
}}

STRUCT!{struct WICRect {
    X: ::INT,
    Y: ::INT,
    Width: ::INT,
    Height: ::INT,
}}

pub type PFNProgressNotification = extern "system" fn (pvData: ::LPVOID, uFrameNum: ::ULONG, operation: ::WICProgressOperation, dblProgress: ::c_double) -> ::HRESULT;
pub type REFWICPixelFormatGUID = *const ::GUID;
pub type WICColor = ::UINT32;
pub type WICInProcPointer = *mut ::BYTE;
pub type WICPixelFormatGUID = ::GUID;
pub const FACILITY_WINCODEC_ERR: ::UINT = 0x898;
pub const WICRawChangeNotification_Contrast: ::UINT = 0x00000010;
pub const WICRawChangeNotification_DestinationColorContext: ::UINT = 0x00000400;
pub const WICRawChangeNotification_ExposureCompensation: ::UINT = 0x00000001;
pub const WICRawChangeNotification_Gamma: ::UINT = 0x00000020;
pub const WICRawChangeNotification_KelvinWhitePoint: ::UINT = 0x00000004;
pub const WICRawChangeNotification_NamedWhitePoint: ::UINT = 0x00000002;
pub const WICRawChangeNotification_NoiseReduction: ::UINT = 0x00000200;
pub const WICRawChangeNotification_RGBWhitePoint: ::UINT = 0x00000008;
pub const WICRawChangeNotification_RenderMode: ::UINT = 0x00002000;
pub const WICRawChangeNotification_Rotation: ::UINT = 0x00001000;
pub const WICRawChangeNotification_Saturation: ::UINT = 0x00000080;
pub const WICRawChangeNotification_Sharpness: ::UINT = 0x00000040;
pub const WICRawChangeNotification_Tint: ::UINT = 0x00000100;
pub const WICRawChangeNotification_ToneCurve: ::UINT = 0x00000800;
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: ::UINT = 0;
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: ::UINT = 0x111100;
pub const WIC_JPEG_MAX_COMPONENT_COUNT: ::UINT = 4;
pub const WIC_JPEG_MAX_TABLE_INDEX: ::UINT = 3;
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: ::UINT = 0;
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: ::UINT = 0x10100;
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: ::UINT = 0x11;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: ::UINT = 0x111122;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: ::UINT = 0x111121;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: ::UINT = 0x111112;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: ::UINT = 0x111111;
pub const WINCODEC_ERR_BASE: ::UINT = 0x2000;
pub const WINCODEC_SDK_VERSION1: ::UINT = 0x0236;
pub const WINCODEC_SDK_VERSION2: ::UINT = 0x0237;
