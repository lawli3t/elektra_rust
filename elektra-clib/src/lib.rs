#![feature(c_variadic)]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ffi::{CStr, CString, VaList, VaListImpl};
use std::{ptr, slice};
use std::str::FromStr;
use std::convert::TryFrom;
use libc::{ssize_t, size_t, c_char, c_int, c_void};

mod structs;

use crate::structs::{
    CKey, CKeySet,
    KeyNewFlags, elektraNamespace, elektraCopyFlags, elektraLockFlags, elektraLookupFlags,
};

use crate::elektraNamespace::KEY_NS_NONE;

use elektra_rust::key::{Key, KeyBuilder, KeyName, KeyNamespace, KeySet};

#[no_mangle]
pub unsafe extern "C" fn keyNew(keyname: *const c_char, args: ...) -> *const CKey {
    let mut va_list: VaListImpl;
    va_list = args.clone();
    keyVNew(keyname, va_list.as_va_list())
}

#[no_mangle]
pub extern "C" fn keyVNew(keyname: *const c_char, mut ap: VaList) -> *const CKey {
    if keyname.is_null() {
        return ptr::null_mut();
    }

    let cstr = unsafe { CStr::from_ptr(keyname) };
    let keyNameStr = cstr.to_str()
        .expect("key name cannot be cast to string");

    if let Ok(builder) = KeyBuilder::from_str(keyNameStr) {
        let keyResult = builder.build();

        if let Ok(key) = keyResult {
            loop {
                let flag_argument = unsafe { ap.arg::<c_int>() };

                let flags = KeyNewFlags::from_bits(flag_argument)
                    .expect("Cannot create Flags from va_list args");

                if flags.contains(KeyNewFlags::KEY_NAME) {
                    println!("KEY_NAME");
                } else if flag_argument == 0 {
                    println!("KEY_END");
                    break;
                } else {
                    break;
                }
            }

            return Box::into_raw(
                Box::new(key.into())
            );
        }
    }

    return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn keyCopy(dest: *mut CKey, source: *const CKey, flags: elektraCopyFlags) -> *mut CKey {
    &mut CKey::default()
}

#[no_mangle]
#[deprecated]
pub extern "C" fn keyClear(key: *mut CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyDel(key: *mut CKey) -> c_int {
    if key.is_null() {
        return -1;
    }

    CKey::destroy(key);

    return 0;
}

#[no_mangle]
pub extern "C" fn keyCopyMeta(
    dest: *mut CKey,
    source: *const CKey,
    metaName: *const c_char,
) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyCopyAllMeta(dest: *mut CKey, source: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyMeta(key: *mut CKey) -> *mut CKeySet {
    unsafe {
        (*key).meta
    }
}

#[no_mangle]
pub extern "C" fn keyCmp(k1: *const CKey, k2: *const CKey) -> c_int {
    if k1.is_null() || k2.is_null() {
        return -1;
    }

    let k1 = unsafe { &*k1 };
    let k2 = unsafe { &*k2 };

    let that_key = match Key::try_from(k1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let other_key = match Key::try_from(k2) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    return that_key.cmp(&other_key) as c_int;
}


#[no_mangle]
pub extern "C" fn keyNeedSync(key: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsBelow(key: *mut CKey, check: *mut CKey) -> c_int {
    if key.is_null() || check.is_null() {
        return -1;
    }

    let key1 = unsafe { &*key };
    let key2 = unsafe { &*check };

    let that_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let other_key = match Key::try_from(key2) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    return match that_key.cmp(&other_key) {
        Ordering::Equal | Ordering::Greater => 0,
        Ordering::Less => 1,
    };
}

#[no_mangle]
pub extern "C" fn keyIsBelowOrSame(key: *mut CKey, check: *mut CKey) -> c_int {
    if key.is_null() || check.is_null() {
        return -1;
    }

    let key1 = unsafe { &*key };
    let key2 = unsafe { &*check };

    let that_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let other_key = match Key::try_from(key2) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    return match that_key.cmp(&other_key) {
        Ordering::Greater => 0,
        Ordering::Less | Ordering::Equal => 1,
    };
}

#[no_mangle]
pub extern "C" fn keyIsDirectlyBelow(key: *const CKey, check: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyName(key: *const CKey) -> *const c_char {
    unsafe {
        (*key).key
    }
}

#[no_mangle]
pub extern "C" fn keyGetNameSize(key: *const CKey) -> ssize_t {
    let cstr = unsafe { CStr::from_ptr((*key).key) };

    cstr.to_bytes_with_nul().len() as ssize_t
}

#[no_mangle]
pub extern "C" fn keySetName(key: *mut CKey, newname: *const c_char) -> ssize_t {
    if key.is_null() || newname.is_null() {
        return -1;
    }

    let cstr = unsafe { CStr::from_ptr(newname) };
    let newNameStr = match cstr.to_str() {
        Ok(x) => x,
        Err(_) => return -1,
    };

    if let Ok(key_name) = KeyName::from_str(newNameStr) {
        let key1 = unsafe { &*key };

        let mut rust_key = match Key::try_from(key1) {
            Ok(x) => x,
            Err(_) => return -1,
        };

        rust_key.set_name(key_name);
        CKey::overwrite(key, rust_key);
        return keyGetNameSize(key);
    }

    return -1;
}

#[no_mangle]
pub extern "C" fn keyAddName(key: *mut CKey, addName: *const c_char) -> ssize_t {
    let cstr = unsafe { CStr::from_ptr(addName) };
    let addNameStr = match cstr.to_str() {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let key1 = unsafe { &*key };
    let mut rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    rust_key.append_name(addNameStr);
    CKey::overwrite(key, rust_key);
    return keyGetNameSize(key);
}

#[no_mangle]
pub extern "C" fn keyUnescapedName(key: *const CKey) -> *const c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn keyGetUnescapedNameSize(key: *const CKey) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn keyBaseName(key: *const CKey) -> *const c_char {
    CString::new("qq").expect("CString new failed").into_raw()
}

#[no_mangle]
pub extern "C" fn keySetBaseName(key: *mut CKey, baseName: *const c_char) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn keyAddBaseName(key: *mut CKey, baseName: *const c_char) -> ssize_t {
    let cstr = unsafe { CStr::from_ptr(baseName) };
    let addNameStr = match cstr.to_str() {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let key1 = unsafe { &*key };
    let mut rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    rust_key.append_name(addNameStr);
    CKey::overwrite(key, rust_key);
    return keyGetNameSize(key);
}

#[no_mangle]
pub extern "C" fn keyGetNamespace(key: *const CKey) -> elektraNamespace {
    if key.is_null() {
        return elektraNamespace::KEY_NS_NONE;
    }

    let key1 = unsafe { &*key };
    let rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return KEY_NS_NONE,
    };

    elektraNamespace::from(rust_key.namespace())
}

#[no_mangle]
pub extern "C" fn keySetNamespace(key: *mut CKey, ns: elektraNamespace) -> ssize_t {
    let key1 = unsafe { &*key };
    let mut rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let namespace = KeyNamespace::from(ns);
    rust_key.set_namespace(namespace);

    namespace.to_string().len() as ssize_t
}

#[no_mangle]
pub extern "C" fn keyValue(key: *const CKey) -> *const c_void {
    let key1 = unsafe { &*key };
    let rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return ptr::null_mut(),
    };

    if let Some(value) = rust_key.value() {
        println!("{:?}", value);
        return value.as_ptr() as *const c_void;
    }

    return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn keySetBinary(key: *mut CKey, newBinary: *const c_void, size: size_t) -> ssize_t {
    let key1 = unsafe { &*key };
    let mut rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let newValue = unsafe {
        slice::from_raw_parts(newBinary as *const u8, size)
    };

    rust_key.set_value(newValue.to_vec());

    println!("{:?}", rust_key.value().unwrap());

    size as ssize_t
}

#[no_mangle]
pub extern "C" fn keyValueSize(key: *const CKey) -> ssize_t {
    let key1 = unsafe { &*key };
    let rust_key = match Key::try_from(key1) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    if let Some(value) = rust_key.value() {
        return value.len() as ssize_t
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn keyString(key: *const CKey) -> *const c_char {
    CString::new("qq").expect("CString new failed").into_raw()
}

#[no_mangle]
pub extern "C" fn keyLock(key: *mut CKey, what: elektraLockFlags) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsLocked(key: *const CKey, what: elektraLockFlags) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn ksNew(alloc: size_t) -> *mut CKeySet {
    let ks = KeySet::default();
    &mut ks.into()
}

#[no_mangle]
pub extern "C" fn ksDup(source: *const CKeySet) -> *mut CKeySet {
    &mut CKeySet::default()
}

#[no_mangle]
pub extern "C" fn ksCopy(dest: *mut CKeySet, source: *const CKeySet) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn ksClear(ks: *mut CKeySet) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn ksDel(ks: *mut CKeySet) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn ksGetSize(ks: *const CKeySet) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn ksAppendKey(ks: *mut CKeySet, toAppend: *mut CKey) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn ksAppend(ks: *mut CKeySet, toAppend: *const CKeySet) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn ksCut(ks: *mut CKeySet, cutpoint: *const CKey) -> *mut CKeySet {
    &mut CKeySet::default()
}

#[no_mangle]
pub extern "C" fn ksPop(ks: *mut CKeySet) -> *mut CKey {
    &mut CKey::default()
}

#[no_mangle]
pub extern "C" fn ksLookup(ks: *mut CKeySet, k: *mut CKey, options: elektraLookupFlags) -> *mut CKey {
    &mut CKey::default()
}

#[no_mangle]
pub extern "C" fn ksLookupByName(
    ks: *mut CKeySet,
    name: *const c_char,
    options: elektraLookupFlags,
) -> *mut CKey {
    &mut CKey::default()
}
