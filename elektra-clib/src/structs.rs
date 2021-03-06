#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::{ptr, slice};
use std::str::FromStr;
use libc::{size_t, c_char, c_int, c_uint, c_void};

use bitflags::bitflags;

use elektra_rust::key::{Key, KeyBuilder, KeyError, KeyNamespace, KeySet};
use crate::elektraNamespace::{KEY_NS_CASCADING, KEY_NS_DEFAULT, KEY_NS_DIR, KEY_NS_META, KEY_NS_PROC, KEY_NS_SPEC, KEY_NS_SYSTEM, KEY_NS_USER};
use crate::KEY_NS_NONE;

bitflags! {
    pub struct KeyNewFlags: i32 {
        const KEY_NAME = 1;
        const KEY_VALUE = 1<<1;
        const KEY_FLAGS = 3;
    }
}

pub type elektraLockFlags = c_int;
pub type elektraKeyFlags = c_int;
pub type elektraKeySetFlags = c_int;
pub type elektraCopyFlags = c_uint;
pub type elektraLookupFlags = c_int;

#[repr(C)]
pub enum elektraNamespace {
    KEY_NS_NONE=0,
    KEY_NS_CASCADING=1,
    KEY_NS_META=2,
    KEY_NS_SPEC=3,
    KEY_NS_PROC=4,
    KEY_NS_DIR=5,
    KEY_NS_USER=6,
    KEY_NS_SYSTEM=7,
    KEY_NS_DEFAULT=8
}

impl From<KeyNamespace> for elektraNamespace {
    fn from(namespace: KeyNamespace) -> Self {
        match namespace {
            KeyNamespace::None => KEY_NS_NONE,
            KeyNamespace::Cascading => KEY_NS_CASCADING,
            KeyNamespace::Meta => KEY_NS_META,
            KeyNamespace::Spec => KEY_NS_SPEC,
            KeyNamespace::Proc => KEY_NS_PROC,
            KeyNamespace::Dir => KEY_NS_DIR,
            KeyNamespace::User => KEY_NS_USER,
            KeyNamespace::System => KEY_NS_SYSTEM,
            KeyNamespace::Default => KEY_NS_DEFAULT,
        }
    }
}

impl From<elektraNamespace> for KeyNamespace {
    fn from(namespace: elektraNamespace) -> Self {
        match namespace {
            KEY_NS_NONE => KeyNamespace::None,
            KEY_NS_CASCADING => KeyNamespace::Cascading,
            KEY_NS_META => KeyNamespace::Meta,
            KEY_NS_SPEC => KeyNamespace::Spec,
            KEY_NS_PROC => KeyNamespace::Proc,
            KEY_NS_DIR => KeyNamespace::Dir,
            KEY_NS_USER => KeyNamespace::User,
            KEY_NS_SYSTEM => KeyNamespace::System,
            KEY_NS_DEFAULT => KeyNamespace::Default,
        }
    }
}

#[repr(C)]
pub union CDataUnion {
    pub c: *mut c_char,
    pub v: *mut libc::c_void,
}

#[repr(C)]
pub struct CKey {
    pub data: CDataUnion,
    pub dataSize: size_t,

    pub key: *mut c_char,
    pub keySize: size_t,

    pub ukey: *mut c_char,
    pub keyUSize: size_t,

    pub ksReference: size_t,

    pub flags: elektraKeyFlags,
    pub meta: *mut CKeySet,
}

impl CKey {
    pub fn default() -> CKey {
        CKey {
            data: CDataUnion { c: CString::new("qq").expect("qq").into_raw() },
            dataSize: 0,
            key: CString::new("qq").expect("qq").into_raw(),
            keySize: 0,
            ukey: CString::new("qq").expect("qq").into_raw(),
            keyUSize: 0,
            ksReference: 0,
            flags: 0,
            meta: &mut CKeySet::default(),
        }
    }

    pub fn overwrite(key: *mut CKey, rustKey: Key) {
        unsafe {
            let ukeyPtr = (*key).ukey;
            let dataPtr = (*key).data.c;
            let dataBinaryPtr = (*key).data.v;
            let keyPtr = (*key).key;

            let c_key: CKey = rustKey.into();
            std::ptr::write(key, c_key);

            drop(
                CString::from_raw(
                    ukeyPtr
                )
            );

            drop(
                ptr::from_raw(
                    dataBinaryPtr
                )
            );

            drop(
                CString::from_raw(
                    dataPtr
                )
            );

            drop(
                CString::from_raw(
                    keyPtr
                )
            );
        };
    }

    pub fn destroy_fields(key: *mut CKey) {
        unsafe {
            drop(
                CString::from_raw(
                    (*key).ukey
                )
            );

            drop(
                CString::from_raw(
                    (*key).data.v
                )
            );

            drop(
                CString::from_raw(
                    (*key).data.c
                )
            );

            drop(
                CString::from_raw(
                    (*key).key
                )
            );
        }
    }

    pub fn destroy(key: *mut CKey) {
        unsafe {
            // TODO might need to swap so no accesses to free'd memory is possible
            Self::destroy_fields(key);
            Box::from_raw(key);
        };
    }
}

impl Into<CKey> for Key {
    fn into(self) -> CKey {
        let name = CString::new(self.name().to_string().clone())
            .expect("qq")
            .into_raw();

        let mut data = vec![97, 98, 99, 0];

        let uKey = CString::new("qq")
            .expect("qq")
            .into_raw();

        CKey {
            data: CDataUnion { v: data.as_mut_ptr() as *mut c_void },
            dataSize: 0,
            key: name,
            keySize: 0,
            ukey: uKey,
            keyUSize: 0,
            ksReference: 0,
            flags: 0,
            meta: &mut CKeySet::default(),
        }
    }
}

impl TryFrom<&CKey> for Key {
    type Error = KeyError;

    fn try_from(value: &CKey) -> Result<Self, Self::Error> {
        let cstr = unsafe { CStr::from_ptr(value.key) };

        let key_name_cstr = cstr.to_str()
            .expect("key name cannot be cast to string");

        let newValue = unsafe {
            slice::from_raw_parts(value.data.v as *const u8, value.dataSize)
        };

        KeyBuilder::from_str(key_name_cstr)?
            .value(newValue.to_vec())
            .build()
    }
}

#[repr(C)]
pub struct CKeySet
{
    pub array: *mut *const CKey,

    pub size: size_t,
    pub alloc: size_t,

    pub cursor: *mut CKey,
    pub current: size_t,

    pub flags: elektraKeySetFlags,

    pub refs: u16,
    pub reserved: u16,
}

impl CKeySet {
    pub fn default() -> CKeySet {
        CKeySet {
            array: ptr::null_mut(),
            size: 0,
            alloc: 0,
            cursor: ptr::null_mut(),
            current: 0,
            flags: 0,
            refs: 0,
            reserved: 0,
        }
    }
}

impl Into<CKeySet> for KeySet {
    fn into(self) -> CKeySet {
        todo!()
    }
}

impl TryFrom<&CKeySet> for KeySet {
    type Error = KeyError;

    fn try_from(value: &CKeySet) -> Result<Self, Self::Error> {
        todo!()
    }
}