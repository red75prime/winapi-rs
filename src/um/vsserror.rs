// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! VSS Error header file
use um::winnt::HRESULT;
pub const VSS_E_BAD_STATE: HRESULT = 0x80042301u32 as i32;
pub const VSS_E_UNEXPECTED: HRESULT = 0x80042302u32 as i32;
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: HRESULT = 0x80042303u32 as i32;
pub const VSS_E_PROVIDER_NOT_REGISTERED: HRESULT = 0x80042304u32 as i32;
pub const VSS_E_PROVIDER_VETO: HRESULT = 0x80042306u32 as i32;
pub const VSS_E_PROVIDER_IN_USE: HRESULT = 0x80042307u32 as i32;
pub const VSS_E_OBJECT_NOT_FOUND: HRESULT = 0x80042308u32 as i32;
pub const VSS_S_ASYNC_PENDING: HRESULT = 0x00042309u32 as i32;
pub const VSS_S_ASYNC_FINISHED: HRESULT = 0x0004230Au32 as i32;
pub const VSS_S_ASYNC_CANCELLED: HRESULT = 0x0004230Bu32 as i32;
pub const VSS_E_VOLUME_NOT_SUPPORTED: HRESULT = 0x8004230Cu32 as i32;
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: HRESULT = 0x8004230Eu32 as i32;
pub const VSS_E_OBJECT_ALREADY_EXISTS: HRESULT = 0x8004230Du32 as i32;
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: HRESULT = 0x8004230Fu32 as i32;
pub const VSS_E_CORRUPT_XML_DOCUMENT: HRESULT = 0x80042310u32 as i32;
pub const VSS_E_INVALID_XML_DOCUMENT: HRESULT = 0x80042311u32 as i32;
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: HRESULT = 0x80042312u32 as i32;
pub const VSS_E_FLUSH_WRITES_TIMEOUT: HRESULT = 0x80042313u32 as i32;
pub const VSS_E_HOLD_WRITES_TIMEOUT: HRESULT = 0x80042314u32 as i32;
pub const VSS_E_UNEXPECTED_WRITER_ERROR: HRESULT = 0x80042315u32 as i32;
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: HRESULT = 0x80042316u32 as i32;
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: HRESULT = 0x80042317u32 as i32;
pub const VSS_E_WRITER_INFRASTRUCTURE: HRESULT = 0x80042318u32 as i32;
pub const VSS_E_WRITER_NOT_RESPONDING: HRESULT = 0x80042319u32 as i32;
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: HRESULT = 0x8004231Au32 as i32;
pub const VSS_E_UNSUPPORTED_CONTEXT: HRESULT = 0x8004231Bu32 as i32;
pub const VSS_E_VOLUME_IN_USE: HRESULT = 0x8004231Du32 as i32;
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: HRESULT = 0x8004231Eu32 as i32;
pub const VSS_E_INSUFFICIENT_STORAGE: HRESULT = 0x8004231Fu32 as i32;
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: HRESULT = 0x80042320u32 as i32;
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: HRESULT = 0x00042321u32 as i32;
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: HRESULT = 0x80042321u32 as i32;
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: HRESULT = 0x80042322u32 as i32;
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: HRESULT = 0x80042323u32 as i32;
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: HRESULT = 0x80042324u32 as i32;
pub const VSS_E_REVERT_IN_PROGRESS: HRESULT = 0x80042325u32 as i32;
pub const VSS_E_REVERT_VOLUME_LOST: HRESULT = 0x80042326u32 as i32;
pub const VSS_E_REBOOT_REQUIRED: HRESULT = 0x80042327u32 as i32;
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: HRESULT = 0x80042328u32 as i32;
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: HRESULT = 0x80042329u32 as i32;
pub const VSS_E_VOLUME_NOT_LOCAL: HRESULT = 0x8004232Du32 as i32;
pub const VSS_E_CLUSTER_TIMEOUT: HRESULT = 0x8004232Eu32 as i32;
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: HRESULT = 0x800423F0u32 as i32;
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: HRESULT = 0x800423F1u32 as i32;
pub const VSS_E_WRITERERROR_TIMEOUT: HRESULT = 0x800423F2u32 as i32;
pub const VSS_E_WRITERERROR_RETRYABLE: HRESULT = 0x800423F3u32 as i32;
pub const VSS_E_WRITERERROR_NONRETRYABLE: HRESULT = 0x800423F4u32 as i32;
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: HRESULT = 0x800423F5u32 as i32;
pub const VSS_E_BREAK_REVERT_ID_FAILED: HRESULT = 0x800423F6u32 as i32;
pub const VSS_E_LEGACY_PROVIDER: HRESULT = 0x800423F7u32 as i32;
pub const VSS_E_MISSING_DISK: HRESULT = 0x800423F8u32 as i32;
pub const VSS_E_MISSING_HIDDEN_VOLUME: HRESULT = 0x800423F9u32 as i32;
pub const VSS_E_MISSING_VOLUME: HRESULT = 0x800423FAu32 as i32;
pub const VSS_E_AUTORECOVERY_FAILED: HRESULT = 0x800423FBu32 as i32;
pub const VSS_E_DYNAMIC_DISK_ERROR: HRESULT = 0x800423FCu32 as i32;
pub const VSS_E_NONTRANSPORTABLE_BCD: HRESULT = 0x800423FDu32 as i32;
pub const VSS_E_CANNOT_REVERT_DISKID: HRESULT = 0x800423FEu32 as i32;
pub const VSS_E_RESYNC_IN_PROGRESS: HRESULT = 0x800423FFu32 as i32;
pub const VSS_E_CLUSTER_ERROR: HRESULT = 0x80042400u32 as i32;
pub const VSS_E_UNSELECTED_VOLUME: HRESULT = 0x8004232Au32 as i32;
pub const VSS_E_SNAPSHOT_NOT_IN_SET: HRESULT = 0x8004232Bu32 as i32;
pub const VSS_E_NESTED_VOLUME_LIMIT: HRESULT = 0x8004232Cu32 as i32;
pub const VSS_E_NOT_SUPPORTED: HRESULT = 0x8004232Fu32 as i32;
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: HRESULT = 0x80042336u32 as i32;
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: HRESULT = 0x80042401u32 as i32;
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: HRESULT = 0x80042402u32 as i32;
pub const VSS_E_ASRERROR_NO_ARCPATH: HRESULT = 0x80042403u32 as i32;
pub const VSS_E_ASRERROR_MISSING_DYNDISK: HRESULT = 0x80042404u32 as i32;
pub const VSS_E_ASRERROR_SHARED_CRIDISK: HRESULT = 0x80042405u32 as i32;
pub const VSS_E_ASRERROR_DATADISK_RDISK0: HRESULT = 0x80042406u32 as i32;
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: HRESULT = 0x80042407u32 as i32;
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: HRESULT = 0x80042408u32 as i32;
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: HRESULT = 0x80042409u32 as i32;
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: HRESULT = 0x8004240Au32 as i32;
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: HRESULT = 0x80042411u32 as i32;
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: HRESULT = 0x80042412u32 as i32;
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: HRESULT = 0x80042413u32 as i32;
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: HRESULT =
    0x80042414u32 as i32;
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: HRESULT = 0x80042415u32 as i32;
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: HRESULT = 0x80042416u32 as i32;
pub const VSS_E_FSS_TIMEOUT: HRESULT = 0x80042417u32 as i32;
