#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
#[cfg_attr(unix, repr(align(4)))]
pub struct VREvent_t {
    pub eventType: u32,
    pub trackedDeviceIndex: TrackedDeviceIndex_t,
    pub eventAgeSeconds: f32,
    pub data: VREvent_Data_t,
}

#[repr(C)]
#[cfg_attr(unix, repr(align(4)))]
pub struct VRControllerState001_t {
    /// If packet num matches that on your prior call, then the controller state hasn't been changed since
    /// your last call and there is no need to process it
    pub unPacketNum: u32,

    /// bit flags for each of the buttons. Use ButtonMaskFromId to turn an ID into a mask
    pub ulButtonPressed: u64,
    pub ulButtonTouched: u64,

    /// Axis data for the controller's analog inputs
    pub rAxis: [VRControllerAxis_t; k_unControllerStateAxisCount as usize],
}

#[repr(C)]
#[cfg_attr(unix, repr(align(4)))]
pub struct RenderModel_TextureMap_t {
    pub unWidth: u16,
    pub unHeight: u16,
    pub rubTextureMapData: *const u8,
}

#[repr(C)]
#[cfg_attr(unix, repr(align(4)))]
pub struct RenderModel_t {
    pub rVertexData: *const RenderModel_Vertex_t,
    pub unVertexCount: u32,
    pub rIndexData: *const u16,
    pub unTriangleCount: u32,
    pub diffuseTextureId: TextureID_t,
}
