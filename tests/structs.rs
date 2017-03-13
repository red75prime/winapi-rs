// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#![cfg(windows)]
extern crate winapi;
use std::mem::{size_of, align_of};
#[cfg(all(target_arch = "x86", feature = "shared-d3d9caps"))] #[test]
fn shared_d3d9caps() {
    use winapi::shared::d3d9caps::*;
    assert_eq!(size_of::<D3DVSHADERCAPS2_0>(), 16);
    assert_eq!(align_of::<D3DVSHADERCAPS2_0>(), 4);
    assert_eq!(size_of::<D3DPSHADERCAPS2_0>(), 20);
    assert_eq!(align_of::<D3DPSHADERCAPS2_0>(), 4);
    assert_eq!(size_of::<D3DOVERLAYCAPS>(), 12);
    assert_eq!(align_of::<D3DOVERLAYCAPS>(), 4);
    assert_eq!(size_of::<D3DCONTENTPROTECTIONCAPS>(), 36);
    // FIXME packed(4)
    // assert_eq!(align_of::<D3DCONTENTPROTECTIONCAPS>(), 4);
    assert_eq!(size_of::<D3DCAPS9>(), 304);
    assert_eq!(align_of::<D3DCAPS9>(), 4);
}
#[cfg(all(target_arch = "x86_64", feature = "shared-d3d9caps"))] #[test]
fn shared_d3d9caps() {
    use winapi::shared::d3d9caps::*;
    assert_eq!(size_of::<D3DVSHADERCAPS2_0>(), 16);
    assert_eq!(align_of::<D3DVSHADERCAPS2_0>(), 4);
    assert_eq!(size_of::<D3DPSHADERCAPS2_0>(), 20);
    assert_eq!(align_of::<D3DPSHADERCAPS2_0>(), 4);
    assert_eq!(size_of::<D3DOVERLAYCAPS>(), 12);
    assert_eq!(align_of::<D3DOVERLAYCAPS>(), 4);
    assert_eq!(size_of::<D3DCONTENTPROTECTIONCAPS>(), 40);
    assert_eq!(align_of::<D3DCONTENTPROTECTIONCAPS>(), 8);
    assert_eq!(size_of::<D3DCAPS9>(), 304);
    assert_eq!(align_of::<D3DCAPS9>(), 4);
}
#[cfg(all(target_arch = "x86", feature = "shared-d3d9types"))] #[test]
fn shared_d3d9types() {
    use winapi::shared::d3d9types::*;
    assert_eq!(size_of::<D3DVECTOR>(), 12);
    assert_eq!(align_of::<D3DVECTOR>(), 4);
    assert_eq!(size_of::<D3DCOLORVALUE>(), 16);
    assert_eq!(align_of::<D3DCOLORVALUE>(), 4);
    assert_eq!(size_of::<D3DRECT>(), 16);
    assert_eq!(align_of::<D3DRECT>(), 4);
    assert_eq!(size_of::<D3DMATRIX>(), 64);
    assert_eq!(align_of::<D3DMATRIX>(), 4);
    assert_eq!(size_of::<D3DVIEWPORT9>(), 24);
    assert_eq!(align_of::<D3DVIEWPORT9>(), 4);
    assert_eq!(size_of::<D3DCLIPSTATUS9>(), 8);
    assert_eq!(align_of::<D3DCLIPSTATUS9>(), 4);
    assert_eq!(size_of::<D3DMATERIAL9>(), 68);
    assert_eq!(align_of::<D3DMATERIAL9>(), 4);
    assert_eq!(size_of::<D3DLIGHT9>(), 104);
    assert_eq!(align_of::<D3DLIGHT9>(), 4);
    assert_eq!(size_of::<D3DVERTEXELEMENT9>(), 8);
    assert_eq!(align_of::<D3DVERTEXELEMENT9>(), 2);
    assert_eq!(size_of::<D3DDISPLAYMODE>(), 16);
    assert_eq!(align_of::<D3DDISPLAYMODE>(), 4);
    assert_eq!(size_of::<D3DDEVICE_CREATION_PARAMETERS>(), 16);
    assert_eq!(align_of::<D3DDEVICE_CREATION_PARAMETERS>(), 4);
    assert_eq!(size_of::<D3DPRESENT_PARAMETERS>(), 56);
    assert_eq!(align_of::<D3DPRESENT_PARAMETERS>(), 4);
    assert_eq!(size_of::<D3DGAMMARAMP>(), 1536);
    assert_eq!(align_of::<D3DGAMMARAMP>(), 2);
    assert_eq!(size_of::<D3DVERTEXBUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3DVERTEXBUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3DINDEXBUFFER_DESC>(), 20);
    assert_eq!(align_of::<D3DINDEXBUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3DSURFACE_DESC>(), 32);
    assert_eq!(align_of::<D3DSURFACE_DESC>(), 4);
    assert_eq!(size_of::<D3DVOLUME_DESC>(), 28);
    assert_eq!(align_of::<D3DVOLUME_DESC>(), 4);
    assert_eq!(size_of::<D3DLOCKED_RECT>(), 8);
    assert_eq!(align_of::<D3DLOCKED_RECT>(), 4);
    assert_eq!(size_of::<D3DBOX>(), 24);
    assert_eq!(align_of::<D3DBOX>(), 4);
    assert_eq!(size_of::<D3DLOCKED_BOX>(), 12);
    assert_eq!(align_of::<D3DLOCKED_BOX>(), 4);
    assert_eq!(size_of::<D3DRANGE>(), 8);
    assert_eq!(align_of::<D3DRANGE>(), 4);
    assert_eq!(size_of::<D3DRECTPATCH_INFO>(), 28);
    assert_eq!(align_of::<D3DRECTPATCH_INFO>(), 4);
    assert_eq!(size_of::<D3DTRIPATCH_INFO>(), 16);
    assert_eq!(align_of::<D3DTRIPATCH_INFO>(), 4);
    assert_eq!(size_of::<D3DADAPTER_IDENTIFIER9>(), 1100);
    // FIXME packed(4)
    // assert_eq!(align_of::<D3DADAPTER_IDENTIFIER9>(), 4);
    assert_eq!(size_of::<D3DRASTER_STATUS>(), 8);
    assert_eq!(align_of::<D3DRASTER_STATUS>(), 4);
    assert_eq!(size_of::<D3DRESOURCESTATS>(), 44);
    assert_eq!(align_of::<D3DRESOURCESTATS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_RESOURCEMANAGER>(), 352);
    assert_eq!(align_of::<D3DDEVINFO_RESOURCEMANAGER>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3DVERTEXSTATS>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3DVERTEXSTATS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_VCACHE>(), 16);
    assert_eq!(align_of::<D3DDEVINFO_VCACHE>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>(), 16);
    assert_eq!(align_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>(), 20);
    assert_eq!(align_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9STAGETIMINGS>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3D9STAGETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>(), 20);
    assert_eq!(align_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>(), 4);
    assert_eq!(size_of::<D3DMEMORYPRESSURE>(), 20);
    // FIXME packed(4)
    // assert_eq!(align_of::<D3DMEMORYPRESSURE>(), 4);
    assert_eq!(size_of::<D3DCOMPOSERECTDESC>(), 8);
    assert_eq!(align_of::<D3DCOMPOSERECTDESC>(), 2);
    assert_eq!(size_of::<D3DCOMPOSERECTDESTINATION>(), 8);
    assert_eq!(align_of::<D3DCOMPOSERECTDESTINATION>(), 2);
    assert_eq!(size_of::<D3DPRESENTSTATS>(), 28);
    // FIXME packed(4)
    // assert_eq!(align_of::<D3DPRESENTSTATS>(), 4);
    assert_eq!(size_of::<D3DDISPLAYMODEEX>(), 24);
    assert_eq!(align_of::<D3DDISPLAYMODEEX>(), 4);
    assert_eq!(size_of::<D3DDISPLAYMODEFILTER>(), 12);
    assert_eq!(align_of::<D3DDISPLAYMODEFILTER>(), 4);
    assert_eq!(size_of::<D3D_OMAC>(), 16);
    assert_eq!(align_of::<D3D_OMAC>(), 1);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>(), 24);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>(), 44);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>(), 4);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>(), 28);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>(), 28);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>(), 32);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>(), 36);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>(), 64);
    // FIXME packed(4)
    // assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>(), 28);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>(), 64);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>(), 60);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>(), 40);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>(), 44);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>(), 44);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>(), 52);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>(), 52);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>(), 4);
    assert_eq!(size_of::<D3DENCRYPTED_BLOCK_INFO>(), 12);
    assert_eq!(align_of::<D3DENCRYPTED_BLOCK_INFO>(), 4);
    assert_eq!(size_of::<D3DAES_CTR_IV>(), 16);
    // FIXME packed(4)
    // assert_eq!(align_of::<D3DAES_CTR_IV>(), 4);
}
#[cfg(all(target_arch = "x86_64", feature = "shared-d3d9types"))] #[test]
fn shared_d3d9types() {
    use winapi::shared::d3d9types::*;
    assert_eq!(size_of::<D3DVECTOR>(), 12);
    assert_eq!(align_of::<D3DVECTOR>(), 4);
    assert_eq!(size_of::<D3DCOLORVALUE>(), 16);
    assert_eq!(align_of::<D3DCOLORVALUE>(), 4);
    assert_eq!(size_of::<D3DRECT>(), 16);
    assert_eq!(align_of::<D3DRECT>(), 4);
    assert_eq!(size_of::<D3DMATRIX>(), 64);
    assert_eq!(align_of::<D3DMATRIX>(), 4);
    assert_eq!(size_of::<D3DVIEWPORT9>(), 24);
    assert_eq!(align_of::<D3DVIEWPORT9>(), 4);
    assert_eq!(size_of::<D3DCLIPSTATUS9>(), 8);
    assert_eq!(align_of::<D3DCLIPSTATUS9>(), 4);
    assert_eq!(size_of::<D3DMATERIAL9>(), 68);
    assert_eq!(align_of::<D3DMATERIAL9>(), 4);
    assert_eq!(size_of::<D3DLIGHT9>(), 104);
    assert_eq!(align_of::<D3DLIGHT9>(), 4);
    assert_eq!(size_of::<D3DVERTEXELEMENT9>(), 8);
    assert_eq!(align_of::<D3DVERTEXELEMENT9>(), 2);
    assert_eq!(size_of::<D3DDISPLAYMODE>(), 16);
    assert_eq!(align_of::<D3DDISPLAYMODE>(), 4);
    assert_eq!(size_of::<D3DDEVICE_CREATION_PARAMETERS>(), 24);
    assert_eq!(align_of::<D3DDEVICE_CREATION_PARAMETERS>(), 8);
    assert_eq!(size_of::<D3DPRESENT_PARAMETERS>(), 64);
    assert_eq!(align_of::<D3DPRESENT_PARAMETERS>(), 8);
    assert_eq!(size_of::<D3DGAMMARAMP>(), 1536);
    assert_eq!(align_of::<D3DGAMMARAMP>(), 2);
    assert_eq!(size_of::<D3DVERTEXBUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3DVERTEXBUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3DINDEXBUFFER_DESC>(), 20);
    assert_eq!(align_of::<D3DINDEXBUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3DSURFACE_DESC>(), 32);
    assert_eq!(align_of::<D3DSURFACE_DESC>(), 4);
    assert_eq!(size_of::<D3DVOLUME_DESC>(), 28);
    assert_eq!(align_of::<D3DVOLUME_DESC>(), 4);
    assert_eq!(size_of::<D3DLOCKED_RECT>(), 16);
    assert_eq!(align_of::<D3DLOCKED_RECT>(), 8);
    assert_eq!(size_of::<D3DBOX>(), 24);
    assert_eq!(align_of::<D3DBOX>(), 4);
    assert_eq!(size_of::<D3DLOCKED_BOX>(), 16);
    assert_eq!(align_of::<D3DLOCKED_BOX>(), 8);
    assert_eq!(size_of::<D3DRANGE>(), 8);
    assert_eq!(align_of::<D3DRANGE>(), 4);
    assert_eq!(size_of::<D3DRECTPATCH_INFO>(), 28);
    assert_eq!(align_of::<D3DRECTPATCH_INFO>(), 4);
    assert_eq!(size_of::<D3DTRIPATCH_INFO>(), 16);
    assert_eq!(align_of::<D3DTRIPATCH_INFO>(), 4);
    assert_eq!(size_of::<D3DADAPTER_IDENTIFIER9>(), 1104);
    assert_eq!(align_of::<D3DADAPTER_IDENTIFIER9>(), 8);
    assert_eq!(size_of::<D3DRASTER_STATUS>(), 8);
    assert_eq!(align_of::<D3DRASTER_STATUS>(), 4);
    assert_eq!(size_of::<D3DRESOURCESTATS>(), 44);
    assert_eq!(align_of::<D3DRESOURCESTATS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_RESOURCEMANAGER>(), 352);
    assert_eq!(align_of::<D3DDEVINFO_RESOURCEMANAGER>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3DVERTEXSTATS>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3DVERTEXSTATS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_VCACHE>(), 16);
    assert_eq!(align_of::<D3DDEVINFO_VCACHE>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>(), 16);
    assert_eq!(align_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>(), 20);
    assert_eq!(align_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9STAGETIMINGS>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3D9STAGETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>(), 20);
    assert_eq!(align_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>(), 4);
    assert_eq!(size_of::<D3DMEMORYPRESSURE>(), 24);
    assert_eq!(align_of::<D3DMEMORYPRESSURE>(), 8);
    assert_eq!(size_of::<D3DCOMPOSERECTDESC>(), 8);
    assert_eq!(align_of::<D3DCOMPOSERECTDESC>(), 2);
    assert_eq!(size_of::<D3DCOMPOSERECTDESTINATION>(), 8);
    assert_eq!(align_of::<D3DCOMPOSERECTDESTINATION>(), 2);
    assert_eq!(size_of::<D3DPRESENTSTATS>(), 32);
    assert_eq!(align_of::<D3DPRESENTSTATS>(), 8);
    assert_eq!(size_of::<D3DDISPLAYMODEEX>(), 24);
    assert_eq!(align_of::<D3DDISPLAYMODEEX>(), 4);
    assert_eq!(size_of::<D3DDISPLAYMODEFILTER>(), 12);
    assert_eq!(align_of::<D3DDISPLAYMODEFILTER>(), 4);
    assert_eq!(size_of::<D3D_OMAC>(), 16);
    assert_eq!(align_of::<D3D_OMAC>(), 1);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>(), 32);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>(), 4);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>(), 4);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>(), 40);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>(), 72);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>(), 40);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>(), 64);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>(), 72);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>(), 80);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>(), 64);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>(), 40);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>(), 72);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>(), 64);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>(), 72);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>(), 72);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>(), 64);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>(), 8);
    assert_eq!(size_of::<D3DENCRYPTED_BLOCK_INFO>(), 12);
    assert_eq!(align_of::<D3DENCRYPTED_BLOCK_INFO>(), 4);
    assert_eq!(size_of::<D3DAES_CTR_IV>(), 16);
    assert_eq!(align_of::<D3DAES_CTR_IV>(), 8);
}
#[cfg(all(target_arch = "x86", feature = "shared-guiddef"))] #[test]
fn shared_guiddef() {
    use winapi::shared::guiddef::*;
    assert_eq!(size_of::<GUID>(), 16);
    assert_eq!(align_of::<GUID>(), 4);
}
#[cfg(all(target_arch = "x86_64", feature = "shared-guiddef"))] #[test]
fn shared_guiddef() {
    use winapi::shared::guiddef::*;
    assert_eq!(size_of::<GUID>(), 16);
    assert_eq!(align_of::<GUID>(), 4);
}
#[cfg(all(target_arch = "x86", feature = "shared-minwindef"))] #[test]
fn shared_minwindef() {
    use winapi::shared::minwindef::*;
    assert_eq!(size_of::<FILETIME>(), 8);
    assert_eq!(align_of::<FILETIME>(), 4);
}
#[cfg(all(target_arch = "x86_64", feature = "shared-minwindef"))] #[test]
fn shared_minwindef() {
    use winapi::shared::minwindef::*;
    assert_eq!(size_of::<FILETIME>(), 8);
    assert_eq!(align_of::<FILETIME>(), 4);
}
#[cfg(all(target_arch = "x86", feature = "shared-windef"))] #[test]
fn shared_windef() {
    use winapi::shared::windef::*;
    assert_eq!(size_of::<RECT>(), 16);
    assert_eq!(align_of::<RECT>(), 4);
    assert_eq!(size_of::<RECTL>(), 16);
    assert_eq!(align_of::<RECTL>(), 4);
    assert_eq!(size_of::<POINT>(), 8);
    assert_eq!(align_of::<POINT>(), 4);
    assert_eq!(size_of::<POINTL>(), 8);
    assert_eq!(align_of::<POINTL>(), 4);
    assert_eq!(size_of::<SIZE>(), 8);
    assert_eq!(align_of::<SIZE>(), 4);
    assert_eq!(size_of::<POINTS>(), 4);
    assert_eq!(align_of::<POINTS>(), 2);
}
#[cfg(all(target_arch = "x86_64", feature = "shared-windef"))] #[test]
fn shared_windef() {
    use winapi::shared::windef::*;
    assert_eq!(size_of::<RECT>(), 16);
    assert_eq!(align_of::<RECT>(), 4);
    assert_eq!(size_of::<RECTL>(), 16);
    assert_eq!(align_of::<RECTL>(), 4);
    assert_eq!(size_of::<POINT>(), 8);
    assert_eq!(align_of::<POINT>(), 4);
    assert_eq!(size_of::<POINTL>(), 8);
    assert_eq!(align_of::<POINTL>(), 4);
    assert_eq!(size_of::<SIZE>(), 8);
    assert_eq!(align_of::<SIZE>(), 4);
    assert_eq!(size_of::<POINTS>(), 4);
    assert_eq!(align_of::<POINTS>(), 2);
}
#[cfg(all(target_arch = "x86", feature = "shared-wtypesbase"))] #[test]
fn shared_wtypesbase() {
    use winapi::shared::wtypesbase::*;
    assert_eq!(size_of::<COAUTHIDENTITY>(), 28);
    assert_eq!(align_of::<COAUTHIDENTITY>(), 4);
    assert_eq!(size_of::<COAUTHINFO>(), 28);
    assert_eq!(align_of::<COAUTHINFO>(), 4);
    assert_eq!(size_of::<BYTE_BLOB>(), 8);
    assert_eq!(align_of::<BYTE_BLOB>(), 4);
    assert_eq!(size_of::<WORD_BLOB>(), 8);
    assert_eq!(align_of::<WORD_BLOB>(), 4);
    assert_eq!(size_of::<WORD_BLOB>(), 8);
    assert_eq!(align_of::<WORD_BLOB>(), 4);
    assert_eq!(size_of::<FLAGGED_BYTE_BLOB>(), 12);
    assert_eq!(align_of::<FLAGGED_BYTE_BLOB>(), 4);
    assert_eq!(size_of::<FLAGGED_WORD_BLOB>(), 12);
    assert_eq!(align_of::<FLAGGED_WORD_BLOB>(), 4);
    assert_eq!(size_of::<BYTE_SIZEDARR>(), 8);
    assert_eq!(align_of::<BYTE_SIZEDARR>(), 4);
    assert_eq!(size_of::<WORD_SIZEDARR>(), 8);
    assert_eq!(align_of::<WORD_SIZEDARR>(), 4);
    assert_eq!(size_of::<DWORD_SIZEDARR>(), 8);
    assert_eq!(align_of::<DWORD_SIZEDARR>(), 4);
    assert_eq!(size_of::<HYPER_SIZEDARR>(), 8);
    assert_eq!(align_of::<HYPER_SIZEDARR>(), 4);
    assert_eq!(size_of::<BLOB>(), 8);
    assert_eq!(align_of::<BLOB>(), 4);
}
#[cfg(all(target_arch = "x86_64", feature = "shared-wtypesbase"))] #[test]
fn shared_wtypesbase() {
    use winapi::shared::wtypesbase::*;
    assert_eq!(size_of::<COAUTHIDENTITY>(), 48);
    assert_eq!(align_of::<COAUTHIDENTITY>(), 8);
    assert_eq!(size_of::<COAUTHINFO>(), 40);
    assert_eq!(align_of::<COAUTHINFO>(), 8);
    assert_eq!(size_of::<BYTE_BLOB>(), 8);
    assert_eq!(align_of::<BYTE_BLOB>(), 4);
    assert_eq!(size_of::<WORD_BLOB>(), 8);
    assert_eq!(align_of::<WORD_BLOB>(), 4);
    assert_eq!(size_of::<WORD_BLOB>(), 8);
    assert_eq!(align_of::<WORD_BLOB>(), 4);
    assert_eq!(size_of::<FLAGGED_BYTE_BLOB>(), 12);
    assert_eq!(align_of::<FLAGGED_BYTE_BLOB>(), 4);
    assert_eq!(size_of::<FLAGGED_WORD_BLOB>(), 12);
    assert_eq!(align_of::<FLAGGED_WORD_BLOB>(), 4);
    assert_eq!(size_of::<BYTE_SIZEDARR>(), 16);
    assert_eq!(align_of::<BYTE_SIZEDARR>(), 8);
    assert_eq!(size_of::<WORD_SIZEDARR>(), 16);
    assert_eq!(align_of::<WORD_SIZEDARR>(), 8);
    assert_eq!(size_of::<DWORD_SIZEDARR>(), 16);
    assert_eq!(align_of::<DWORD_SIZEDARR>(), 8);
    assert_eq!(size_of::<HYPER_SIZEDARR>(), 16);
    assert_eq!(align_of::<HYPER_SIZEDARR>(), 8);
    assert_eq!(size_of::<BLOB>(), 16);
    assert_eq!(align_of::<BLOB>(), 8);
}
