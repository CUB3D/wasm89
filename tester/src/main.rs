#![feature(c_str_literals)]

use std::ffi::CString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct T {
    commands: Vec<C>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum A {
    #[serde(rename = "invoke")]
    Invoke {
        field: String,
    }

}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum C {
    #[serde(rename = "module")]
    Module{
        filename: String,
    },
    #[serde(rename = "assert_return")]
    AssertReturn{
        action: A,
    },
    #[serde(rename = "assert_invalid")]
    AssertInvalid{}
}

#[repr(C)]
struct O {
    a: bool,
    b: bool,
    c: bool,
}

#[repr(C)]
struct Module {}

fn main() {

    let (load_module, get_export_fidx, invoke) = unsafe {
        let l = libc::dlopen(c"../bin/libwasm89.so".as_ptr(), libc::RTLD_NOW);
        let load_module = libc::dlsym(l, c"load_module".as_ptr());
        let load_module = core::mem::transmute::<_, extern "C" fn(*const u8, usize, O)->*mut Module>(load_module);
        let get_export_fidx = libc::dlsym(l, c"get_export_fidx".as_ptr());
        let get_export_fidx = core::mem::transmute::<_, extern "C" fn(*mut Module, *const i8)->usize>(get_export_fidx);
        let invoke = libc::dlsym(l, c"invoke".as_ptr());
        let invoke = core::mem::transmute::<_, extern "C" fn(*mut Module, usize)->usize>(invoke);
        println!("{:x}", invoke as usize);

        (load_module, get_export_fidx, invoke)
    };

    let t: T = serde_json::from_str(include_str!("../res/nop/nop.json")).unwrap();
    println!("{:?}", t);
    let mut m = core::ptr::null_mut();
    for c in t.commands {
        match c {
            C::Module { .. } => {
                let mm = include_bytes!("../res/nop/nop.0.wasm");
                m = load_module(mm.as_ptr(), mm.len(), O {
                    a: false,
                    b: false,
                    c: false,
                });
            }
            C::AssertReturn { action } => {
                match action {
                    A::Invoke { field } => {
                        println!("I: {field}");
                        let fs = CString::new(field).unwrap();
                        let f = get_export_fidx(m, fs.as_ptr());
                        invoke(m, f);
                    }
                }
            }
            C::AssertInvalid { .. } => {}
        }
    }
}
