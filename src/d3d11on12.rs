// Copyright © 2016; Dmitry Roschin
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Mappings for the content of d3d11on12.h

STRUCT!{struct D3D11_RESOURCE_FLAGS {
    BindFlags: ::UINT,
    MiscFlags: ::UINT,
    CPUAccessFlags: ::UINT,
    StructureByteStride: ::UINT,
}}

RIDL!(
interface ID3D11On12Device(ID3D11On12DeviceVtbl): IUnknown(IUnknownVtbl) {
    fn CreateWrappedResource(
        &mut self, pResource12: *mut ::IUnknown, pFlags11: *const ::D3D11_RESOURCE_FLAGS,
        InState: ::D3D12_RESOURCE_STATES, OutState: ::D3D12_RESOURCE_STATES, riid: *const ::IID,
        ppResource11: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn ReleaseWrappedResources(
        &mut self, ppResources: *mut *mut ::ID3D11Resource, NumResources: ::UINT
    ) -> (),
    fn AcquireWrappedResources(
        &mut self, ppResources: *mut *mut ::ID3D11Resource, NumResources: ::UINT
    ) -> ()
});

pub type PFN_D3D11ON12_CREATE_DEVICE = 
  extern "system" fn (_ : *mut ::IUnknown, _ : ::UINT, _ : *const ::D3D_FEATURE_LEVEL, 
                      FeatureLevels: ::UINT, _ : *mut *mut ::IUnknown, NumQueues: ::UINT, 
                      _ : ::UINT, _ : *mut *mut ::ID3D11Device, _ : *mut *mut ::ID3D11DeviceContext, 
                      _ : *mut ::D3D_FEATURE_LEVEL) -> ::HRESULT;

#[link(name="d3d11")]
extern "system" {
    pub fn D3D11On12CreateDevice(pDevice: *mut ::IUnknown, Flags: ::UINT,
        pFeatureLevels: *const ::D3D_FEATURE_LEVEL, FeatureLevels: ::UINT, 
        ppCommandQueues: *mut *mut ::IUnknown, NumQueues: ::UINT, NodeMask: ::UINT, 
        ppDevice: *mut *mut ::ID3D11Device, ppImmediateContext: *mut *mut ::ID3D11DeviceContext, 
        pChosenFeatureLevel: *mut ::D3D_FEATURE_LEVEL) -> ::HRESULT;
}
