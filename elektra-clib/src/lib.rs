#![feature(c_variadic)]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::ffi::{CStr, CString, VaList, VaListImpl};
use std::ptr;
use libc::{ssize_t, size_t, c_char, c_int, c_void};

mod structs;

use crate::structs::{
    CKey, CKeySet,
    KeyNewFlags, elektraNamespace, elektraCopyFlags, elektraLockFlags, elektraLookupFlags,
    CKeyEquivalent, CKeySetEquivalent
};

use elektra_rust::key::{Key, KeySet};

/*
#[no_mangle]
pub extern "C" fn kdbOpen(contract: *const CKeySet, parentKey: *mut CKey) -> *mut CKDB;
#[no_mangle]
pub extern "C" fn kdbClose(handle: *mut CKDB, errorKey: *mut CKey) -> ::std::os::raw::c_int;
#[no_mangle]
pub extern "C" fn kdbGet(
    handle: *mut CKDB,
    returned: *mut KeySet,
    parentKey: *mut Key,
) -> ::std::os::raw::c_int;
#[no_mangle]
pub extern "C" fn kdbSet(
    handle: *mut CKDB,
    returned: *mut KeySet,
    parentKey: *mut Key,
) -> ::std::os::raw::c_int;
*/

fn cleanup_c_key(key: *mut CKey) {
    unsafe {
        CString::from_raw(
            (*key).ukey
        );

        CString::from_raw(
            (*key).data.c
        );

        CString::from_raw(
            (*key).key
        );

        Box::from_raw(key)
    };
}

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
        .expect("key name cannot be cast to string")
        .to_string();

    let key: Key = Key::new(keyNameStr);

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

    Box::into_raw(
        Box::new(key.to_ckey())
    )
}

#[no_mangle]
pub extern "C" fn keyCopy(dest: *mut CKey, source: *const CKey, flags: elektraCopyFlags) -> *mut CKey {
    &mut CKey::default()

}

#[no_mangle]
pub extern "C" fn keyClear(key: *mut CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyDel(key: *mut CKey) -> c_int {
    if key.is_null() {
        return -1;
    }

    cleanup_c_key(key);

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
    &CKey::default()
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
    &mut CKeySet::default()
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
pub extern "C" fn keyIsBelow(key: *const CKey, check: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsBelowOrSame(key: *const CKey, check: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsDirectlyBelow(key: *const CKey, check: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsBinary(key: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyIsString(key: *const CKey) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn keyName(key: *const CKey) -> *const c_char {
    unsafe { (*key).key }

    /*
    let mut key = unsafe { Key::from_ckey(&*key) };

    let name = CString::new(key.name().clone())
        .unwrap();

    name.into_raw() as *mut u8
    */
}

#[no_mangle]
pub extern "C" fn keyGetNameSize(key: *const CKey) -> ssize_t {
    1

}

#[no_mangle]
pub extern "C" fn keyGetName(
    key: *const CKey,
    returnedName: *mut c_char,
    maxSize: size_t,
) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn keySetName(key: *mut CKey, newname: *const c_char) -> ssize_t {
    let mut rustKey = unsafe { Key::from_ckey(&*key) };

    let cstr = unsafe { CStr::from_ptr(newname) };
    let keyNameStr = cstr.to_str()
        .unwrap()
        .to_string();

    rustKey.set_name(keyNameStr);

    unsafe {
        let ukeyPtr = (*key).ukey;
        let dataPtr = (*key).data.c;
        let keyPtr = (*key).key;

        std::ptr::write(key, rustKey.to_ckey());

        CString::from_raw(
            ukeyPtr
        );

        CString::from_raw(
            dataPtr
        );

        CString::from_raw(
            keyPtr
        );
    };

    cstr.to_bytes_with_nul().len() as ssize_t
}

#[no_mangle]
pub extern "C" fn keyAddName(key: *mut CKey, addName: *const c_char) -> ssize_t {
    1

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
pub extern "C" fn keyGetBaseNameSize(key: *const CKey) -> ssize_t {
    1

}

#[no_mangle]
pub extern "C" fn keyGetBaseName(
    key: *const CKey,
    returned: *mut c_char,
    maxSize: size_t,
) -> ssize_t {
    1
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
pub extern "C" fn keyGetString(
    key: *const CKey,
    returnedString: *mut c_char,
    maxSize: size_t,
) -> ssize_t {
    1
}

#[no_mangle]
pub extern "C" fn keySetString(key: *mut CKey, newString: *const c_char) -> ssize_t {
    1

}

#[no_mangle]
pub extern "C" fn keyGetBinary(
    key: *const CKey,
    returnedBinary: *mut c_void,
    maxSize: size_t,
) -> ssize_t {
    1

}

#[no_mangle]
pub extern "C" fn keySetBinary(
    key: *mut CKey,
    newBinary: *const c_void,
    dataSize: size_t,
) -> ssize_t {
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
    let ks = KeySet::new();
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
