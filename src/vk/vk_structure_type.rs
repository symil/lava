// Generated by `scripts/generate_type.js`

use std::convert::From;

pub type RawVkStructureType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkStructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    LoaderInstanceCreateInfo = 47,
    LoaderDeviceCreateInfo = 48,
    PhysicalDeviceSubgroupProperties = 1000094000,
    BindBufferMemoryInfo = 1000157000,
    BindImageMemoryInfo = 1000157001,
    MemoryDedicatedRequirements = 1000127000,
    MemoryDedicatedAllocateInfo = 1000127001,
    MemoryAllocateFlagsInfo = 1000060000,
    DeviceGroupRenderPassBeginInfo = 1000060003,
    DeviceGroupCommandBufferBeginInfo = 1000060004,
    DeviceGroupSubmitInfo = 1000060005,
    DeviceGroupBindSparseInfo = 1000060006,
    BindBufferMemoryDeviceGroupInfo = 1000060013,
    BindImageMemoryDeviceGroupInfo = 1000060014,
    PhysicalDeviceGroupProperties = 1000070000,
    DeviceGroupDeviceCreateInfo = 1000070001,
    PhysicalDevicePointClippingProperties = 1000117000,
    RenderPassInputAttachmentAspectCreateInfo = 1000117001,
    ImageViewUsageCreateInfo = 1000117002,
    PipelineTessellationDomainOriginStateCreateInfo = 1000117003,
    RenderPassMultiviewCreateInfo = 1000053000,
    PhysicalDeviceMultiviewFeatures = 1000053001,
    PhysicalDeviceMultiviewProperties = 1000053002,
    PhysicalDeviceVariablePointerFeatures = 1000120000,
    ProtectedSubmitInfo = 1000145000,
    PhysicalDeviceProtectedMemoryFeatures = 1000145001,
    PhysicalDeviceProtectedMemoryProperties = 1000145002,
    SamplerYcbcrConversionCreateInfo = 1000156000,
    SamplerYcbcrConversionInfo = 1000156001,
    BindImagePlaneMemoryInfo = 1000156002,
    ImagePlaneMemoryRequirementsInfo = 1000156003,
    PhysicalDeviceSamplerYcbcrConversionFeatures = 1000156004,
    SamplerYcbcrConversionImageFormatProperties = 1000156005,
    DescriptorUpdateTemplateCreateInfo = 1000085000,
    PhysicalDeviceExternalImageFormatInfo = 1000071000,
    ExternalImageFormatProperties = 1000071001,
    PhysicalDeviceExternalBufferInfo = 1000071002,
    ExternalBufferProperties = 1000071003,
    PhysicalDeviceIdProperties = 1000071004,
    ExternalMemoryBufferCreateInfo = 1000072000,
    ExternalMemoryImageCreateInfo = 1000072001,
    ExportMemoryAllocateInfo = 1000072002,
    PhysicalDeviceExternalFenceInfo = 1000112000,
    ExternalFenceProperties = 1000112001,
    ExportFenceCreateInfo = 1000113000,
    ExportSemaphoreCreateInfo = 1000077000,
    PhysicalDeviceExternalSemaphoreInfo = 1000076000,
    ExternalSemaphoreProperties = 1000076001,
    DescriptorSetLayoutSupport = 1000168001,
    PhysicalDeviceShaderDrawParameterFeatures = 1000063000,
    SwapchainCreateInfoKhr = 1000001000,
    PresentInfoKhr = 1000001001,
    DeviceGroupPresentCapabilitiesKhr = 1000060007,
    ImageSwapchainCreateInfoKhr = 1000060008,
    BindImageMemorySwapchainInfoKhr = 1000060009,
    AcquireNextImageInfoKhr = 1000060010,
    DeviceGroupPresentInfoKhr = 1000060011,
    DeviceGroupSwapchainCreateInfoKhr = 1000060012,
    DisplayModeCreateInfoKhr = 1000002000,
    DisplaySurfaceCreateInfoKhr = 1000002001,
    DisplayPresentInfoKhr = 1000003000,
    XlibSurfaceCreateInfoKhr = 1000004000,
    XcbSurfaceCreateInfoKhr = 1000005000,
    WaylandSurfaceCreateInfoKhr = 1000006000,
    MirSurfaceCreateInfoKhr = 1000007000,
    AndroidSurfaceCreateInfoKhr = 1000008000,
    DebugReportCallbackCreateInfoExt = 1000011000,
    PipelineRasterizationStateRasterizationOrderAmd = 1000018000,
    DebugMarkerObjectNameInfoExt = 1000022000,
    DebugMarkerObjectTagInfoExt = 1000022001,
    DebugMarkerMarkerInfoExt = 1000022002,
    DedicatedAllocationImageCreateInfoNv = 1000026000,
    DedicatedAllocationBufferCreateInfoNv = 1000026001,
    DedicatedAllocationMemoryAllocateInfoNv = 1000026002,
    TextureLodGatherFormatPropertiesAmd = 1000041000,
    ExternalMemoryImageCreateInfoNv = 1000056000,
    ExportMemoryAllocateInfoNv = 1000056001,
    ValidationFlagsExt = 1000061000,
    ViSurfaceCreateInfoNn = 1000062000,
    ImportMemoryFdInfoKhr = 1000074000,
    MemoryFdPropertiesKhr = 1000074001,
    MemoryGetFdInfoKhr = 1000074002,
    ImportSemaphoreFdInfoKhr = 1000079000,
    SemaphoreGetFdInfoKhr = 1000079001,
    PhysicalDevicePushDescriptorPropertiesKhr = 1000080000,
    PresentRegionsKhr = 1000084000,
    ObjectTableCreateInfoNvx = 1000086000,
    IndirectCommandsLayoutCreateInfoNvx = 1000086001,
    CmdProcessCommandsInfoNvx = 1000086002,
    CmdReserveSpaceForCommandsInfoNvx = 1000086003,
    DeviceGeneratedCommandsLimitsNvx = 1000086004,
    DeviceGeneratedCommandsFeaturesNvx = 1000086005,
    PipelineViewportWScalingStateCreateInfoNv = 1000087000,
    DisplayPowerInfoExt = 1000091000,
    DeviceEventInfoExt = 1000091001,
    DisplayEventInfoExt = 1000091002,
    SwapchainCounterCreateInfoExt = 1000091003,
    PresentTimesInfoGoogle = 1000092000,
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx = 1000097000,
    PipelineViewportSwizzleStateCreateInfoNv = 1000098000,
    PhysicalDeviceDiscardRectanglePropertiesExt = 1000099000,
    PipelineDiscardRectangleStateCreateInfoExt = 1000099001,
    PhysicalDeviceConservativeRasterizationPropertiesExt = 1000101000,
    PipelineRasterizationConservativeStateCreateInfoExt = 1000101001,
    HdrMetadataExt = 1000105000,
    SharedPresentSurfaceCapabilitiesKhr = 1000111000,
    ImportFenceFdInfoKhr = 1000115000,
    FenceGetFdInfoKhr = 1000115001,
    IosSurfaceCreateInfoMvk = 1000122000,
    MacosSurfaceCreateInfoMvk = 1000123000,
    DebugUtilsObjectNameInfoExt = 1000128000,
    DebugUtilsObjectTagInfoExt = 1000128001,
    DebugUtilsLabelExt = 1000128002,
    DebugUtilsMessengerCallbackDataExt = 1000128003,
    DebugUtilsMessengerCreateInfoExt = 1000128004,
    AndroidHardwareBufferUsageAndroid = 1000129000,
    AndroidHardwareBufferPropertiesAndroid = 1000129001,
    AndroidHardwareBufferFormatPropertiesAndroid = 1000129002,
    ImportAndroidHardwareBufferInfoAndroid = 1000129003,
    MemoryGetAndroidHardwareBufferInfoAndroid = 1000129004,
    ExternalFormatAndroid = 1000129005,
    PhysicalDeviceSamplerFilterMinmaxPropertiesExt = 1000130000,
    SamplerReductionModeCreateInfoExt = 1000130001,
    SampleLocationsInfoExt = 1000143000,
    RenderPassSampleLocationsBeginInfoExt = 1000143001,
    PipelineSampleLocationsStateCreateInfoExt = 1000143002,
    PhysicalDeviceSampleLocationsPropertiesExt = 1000143003,
    MultisamplePropertiesExt = 1000143004,
    ImageFormatListCreateInfoKhr = 1000147000,
    PhysicalDeviceBlendOperationAdvancedFeaturesExt = 1000148000,
    PhysicalDeviceBlendOperationAdvancedPropertiesExt = 1000148001,
    PipelineColorBlendAdvancedStateCreateInfoExt = 1000148002,
    PipelineCoverageToColorStateCreateInfoNv = 1000149000,
    PipelineCoverageModulationStateCreateInfoNv = 1000152000,
    ValidationCacheCreateInfoExt = 1000160000,
    ShaderModuleValidationCacheCreateInfoExt = 1000160001,
    DescriptorSetLayoutBindingFlagsCreateInfoExt = 1000161000,
    PhysicalDeviceDescriptorIndexingFeaturesExt = 1000161001,
    PhysicalDeviceDescriptorIndexingPropertiesExt = 1000161002,
    DescriptorSetVariableDescriptorCountAllocateInfoExt = 1000161003,
    DescriptorSetVariableDescriptorCountLayoutSupportExt = 1000161004,
    DeviceQueueGlobalPriorityCreateInfoExt = 1000174000,
    ImportMemoryHostPointerInfoExt = 1000178000,
    MemoryHostPointerPropertiesExt = 1000178001,
    PhysicalDeviceExternalMemoryHostPropertiesExt = 1000178002,
    PhysicalDeviceShaderCorePropertiesAmd = 1000185000,
    PhysicalDeviceVertexAttributeDivisorPropertiesExt = 1000190000,
    PipelineVertexInputDivisorStateCreateInfoExt = 1000190001
}

impl<'a> From<&'a i32> for VkStructureType {
    fn from(value: &'a i32) -> Self {
        unsafe { *((value as *const i32) as *const VkStructureType) }
    }
}

impl<'a> From<&'a VkStructureType> for i32 {
    fn from(value: &'a VkStructureType) -> Self {
        *value as i32
    }
}