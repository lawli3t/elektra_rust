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

#[repr(C)]
pub enum elektraNamespace {
    KEY_NS_NONE = 0,
    KEY_NS_CASCADING = 1,
    KEY_NS_META = 2,
    KEY_NS_SPEC = 3,
    KEY_NS_PROC = 4,
    KEY_NS_DIR = 5,
    KEY_NS_USER = 6,
    KEY_NS_SYSTEM = 7,
    KEY_NS_DEFAULT = 8,
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
pub struct CKey {
    pub data: *mut c_void,
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
    pub fn overwrite(key: *mut CKey, rustKey: Key) {
        unsafe {
            let ukeyPtr = (*key).ukey;
            let keyPtr = (*key).key;
            let dataPtr = (*key).data;

            let c_key: CKey = rustKey.into();
            std::ptr::write(key, c_key);

            drop(
                CString::from_raw(
                    ukeyPtr
                )
            );

            drop(
                CString::from_raw(
                    keyPtr
                )
            );

            drop(
                Box::from_raw(
                    dataPtr
                )
            );
        }
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
                    (*key).key
                )
            );

            drop(
                Box::from_raw(
                    (*key).data
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
        let key = CString::new(self.name().to_string())
            .expect("test");

        let keySize = key.as_bytes_with_nul().len();

        let uKey = CString::new("test")
            .expect("test");

        let keyUSize = uKey.as_bytes_with_nul().len();

        let data = match self.value() {
            None => {
                ptr::null_mut()
            }
            Some(value) => {
                let mut buf = value
                    .to_vec()
                    .into_boxed_slice();

                let ptr = buf.as_mut_ptr();
                std::mem::forget(buf);

                ptr as *mut c_void
            }
        };

        let dataSize = match self.value() {
            None => { 0 }
            Some(value) => { value.len() }
        };

        CKey {
            data,
            dataSize,
            key: key.into_raw(),
            keySize,
            ukey: uKey.into_raw(),
            keyUSize,
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

        let mut builder = KeyBuilder::from_str(key_name_cstr)?;

        if !value.data.is_null() {
            let newValue = unsafe {
                slice::from_raw_parts_mut(value.data as *mut u8, value.dataSize)
            };

            builder = builder.value(newValue);
        }

        builder.build()
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