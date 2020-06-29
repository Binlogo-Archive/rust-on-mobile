use starwars_core::swapi::People;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct PeopleNativeWrapper {
    pub(crate) array: *mut PeopleNative,
    pub(crate) length: usize,
}

#[repr(C)]
pub struct PeopleNative {
    pub name: *const c_char,
    pub gender: *const c_char,
    pub mass: *const c_char,
}

impl From<Vec<People>> for PeopleNativeWrapper {
    fn from(people_vec: Vec<People>) -> Self {
        let mut native_vec: Vec<PeopleNative> = Vec::new();
        for p in people_vec {
            let native_people = PeopleNative {
                name: CString::new(p.name).unwrap().into_raw(),
                gender: CString::new(p.gender).unwrap().into_raw(),
                mass: CString::new(p.mass).unwrap().into_raw(),
            };
            native_vec.push(native_people);
        }

        PeopleNativeWrapper {
            array: native_vec.as_mut_ptr(),
            length: native_vec.len() as _,
        }
    }
}
