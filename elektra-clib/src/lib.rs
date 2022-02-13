#![feature(c_variadic)]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ffi::{CStr, CString, VaList, VaListImpl};
use std::ptr;
use std::str::FromStr;
use libc::{ssize_t, size_t, c_char, c_int, c_void};

mod structs;

use crate::structs::{
    CKey, CKeySet,
    KeyNewFlags, elektraNamespace, elektraCopyFlags, elektraLockFlags, elektraLookupFlags,
    CKeyEquivalent, CKeySetEquivalent
};

use elektra_rust::key::{Key, KeyBuilder, KeyName, KeySet};

#[no_mangle]
pub unsafe extern "C" fn keyNew(keyname: *const c_char, args: ...) -> *const CKey {
    let mut va_list: VaListImpl;
    va_list = args.clone();
    keyVNew(keyname, va_list.as_va_list())
}

#[no_mangle]
pub extern "C" fn keyVNew(keyname: *const c_char, mut ap: VaList) -> *const CKey {
    if keyname.is_null() {
        return ptr::null_mut()
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
                    break
                } else {
                    break
                }
            }

            return Box::into_raw(
                Box::new(key.to_ckey())
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
pub extern "C" fn keyGetMeta(key: *const CKey, metaName: *const c_char) -> *const CKey {
    &mut CKey::default()
}

#[no_mangle]
pub extern "C" fn keySetMeta(
    key: *mut CKey,
    metaName: *const c_char,
    newMetaString: *const c_char,
) -> ssize_t {
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
    1
}

#[no_mangle]
pub extern "C" fn keyNeedSync(key: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsBelow(key: *mut CKey, check: *mut CKey) -> c_int {
    let that_key = match Key::from_ckey(key) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let other_key = match Key::from_ckey(check) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    return match that_key.cmp(&other_key) {
        Ordering::Equal | Ordering::Greater => 0,
        Ordering::Less => 1,
    }
}

#[no_mangle]
pub extern "C" fn keyIsBelowOrSame(key: *mut CKey, check: *mut CKey) -> c_int {
    let that_key = match Key::from_ckey(key) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let other_key = match Key::from_ckey(check) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    return match that_key.cmp(&other_key) {
        Ordering::Greater => 0,
        Ordering::Less | Ordering::Equal => 1,
    }
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
    let cstr = unsafe { CStr::from_ptr(newname) };
    let newNameStr = match cstr.to_str() {
        Ok(x) => x,
        Err(_) => return -1,
    };


    if let Ok(key_name) = KeyName::from_str(newNameStr) {
        let mut rustKey = match Key::from_ckey(key) {
            Ok(x) => x,
            Err(_) => return -1,
        };

        rustKey.set_name(key_name);
        CKey::overwrite(key, rustKey);
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

    let mut rustKey = match Key::from_ckey(key) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    rustKey.append_name(addNameStr);
    CKey::overwrite(key, rustKey);
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
    1

}

#[no_mangle]
pub extern "C" fn keyGetNamespace(key: *const CKey) -> elektraNamespace {
    1
}

#[no_mangle]
pub extern "C" fn keySetNamespace(key: *mut CKey, ns: elektraNamespace) -> ssize_t {
    1

}

#[no_mangle]
pub extern "C" fn keyValue(key: *const CKey) -> *const c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn keyGetValueSize(key: *const CKey) -> ssize_t {
    1

}

#[no_mangle]
pub extern "C" fn keyString(key: *const CKey) -> *const c_char {
    CString::new("qq").expect("CString new failed").into_raw()
}

#[no_mangle]
pub extern "C" fn keySetString(key: *mut CKey, newString: *const c_char) -> ssize_t {
    1

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
    &mut ks.to_ckeyset()
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
