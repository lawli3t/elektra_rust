#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::ffi::{CStr, CString};
use std::ptr;
use libc::{size_t, c_char, c_int, c_uint};

use bitflags::bitflags;

use elektra_rust::key::Key;

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
pub type elektraNamespace = c_int;

#[repr(C)]
pub union CDataUnion {
    pub c: *const c_char,
    pub v: *const libc::c_void,
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
}

#[repr(C)]
pub struct CKeySet
{
    pub size: size_t,
    pub alloc: size_t,

    pub cursor: *mut CKey,
    pub current: size_t,

    pub flags: elektraKeySetFlags,
}

impl CKeySet {
    pub fn default() -> CKeySet {
        CKeySet {
            size: 0,
            alloc: 0,
            cursor: ptr::null_mut(),
            current: 0,
            flags: 0
        }
    }
}

pub trait CKeyConvertable {
    fn to_ckey(&self) -> CKey;
    fn from_ckey(c_key: CKey) -> Key;
}

impl CKeyConvertable for Key {
    fn to_ckey(&self) -> CKey {
        CKey {
            data: CDataUnion { c: CString::new("qq").expect("qq").into_raw() },
            dataSize: 0,
            key: CString::new(self.name().clone()).expect("qq").into_raw(),
            keySize: 0,
            ukey: CString::new("qq").expect("qq").into_raw(),
            keyUSize: 0,
            ksReference: 0,
            flags: 0,
            meta: &mut CKeySet::default(),
        }
    }

    fn from_ckey(c_key: CKey) -> Key {
        let cstr = unsafe { CStr::from_ptr(c_key.key) };
        let keyNameStr = cstr.to_str()
            .expect("key name cannot be cast to string")
            .to_string();

        Key::new(String::from(keyNameStr))
    }
}