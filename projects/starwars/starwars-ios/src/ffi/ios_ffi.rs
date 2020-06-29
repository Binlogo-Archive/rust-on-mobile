use super::callback::Callback;
use super::native_model::{PeopleNative, PeopleNativeWrapper};
use starwars_core::swapi::{self, SwapiClient};
use std::ffi::CString;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn create_swapi_client() -> *mut SwapiClient {
    Box::into_raw(Box::new(SwapiClient::new()))
}

#[no_mangle]
pub unsafe extern "C" fn free_swapi_client(client: *mut SwapiClient) {
    if client.is_null() {
        return;
    }
    Box::from_raw(client);
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct PeopleCallback {
    owner: *mut c_void,
    onSuccess: extern "C" fn(owner: *mut c_void, res: *const PeopleNativeWrapper),
    onError: extern "C" fn(owner: *mut c_void),
}

#[no_mangle]
pub unsafe extern "C" fn load_all_people(client: *mut SwapiClient, callback: PeopleCallback) {
    assert!(!client.is_null());

    let client = client.as_ref().unwrap();
    let cb = Callback {
        on_success: Box::new(move |res| {
            let native_res = PeopleNativeWrapper::from(res);
            (callback.onSuccess)(callback.owner, &native_res);
        }),
        on_error: Box::new(move |err| {}),
    };
    let cb = Box::new(cb);
    client.loadAllPeople(cb);
}
