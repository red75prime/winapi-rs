// Copyright © 2015, Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate d3d12;
use d3d12::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
//    bb(D3D12CreateDevice);
//    bb(D3D12CreateRootSignatureDeserializer);
//    bb(D3D12GetDebugInterface);
//    bb(D3D12SerializeRootSignature);
}
#[cfg(target_env = "msvc")]
#[test]
fn msvc_functions() {
    bb(D3D12CreateDevice);
    bb(D3D12CreateRootSignatureDeserializer);
    bb(D3D12GetDebugInterface);
    bb(D3D12SerializeRootSignature);
}
