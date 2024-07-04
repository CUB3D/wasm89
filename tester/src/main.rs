#![feature(c_str_literals)]

use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::str::FromStr;
use serde::Deserialize;
#[repr(C)]
#[derive(Debug)]
pub enum S {
    Ok = 0,
    Err = 1,
}

#[repr(C)]
pub struct R {
    status: S,
    msg: *const i8,
}

impl R {
    pub fn safe_r(&self) -> SafeR {
        unsafe {
            match self.status {
                S::Ok => SafeR::Ok,
                S::Err => SafeR::Err(CStr::from_ptr(self.msg).to_str().unwrap().to_string())
            }
        }
    }
}
#[derive(Debug)]
pub enum SafeR {
    Ok,
    Err(String)
}

#[derive(Debug, Deserialize)]
struct T {
    commands: Vec<C>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Arg {
    #[serde(rename = "i32")]
    I32{
        value: String,
    },
    #[serde(rename = "i64")]
    I64{
        value: String,
    },
    #[serde(rename = "f32")]
    F32{
        value: String,
    },
    #[serde(rename = "f64")]
    F64{
        value: String,
    },
}

impl Arg {
    fn sv(&self) -> SV {
        match self {
            Arg::I32 { value } => {
                SV {
                    value_ty: 0x7f,
                    v: SVV {
                        i32: u32::from_str(&value).unwrap() as i32,
                    }
                }
            }
            Arg::I64 { value } => {
                SV {
                    value_ty: 0x7e,
                    v: SVV {
                        i64: u64::from_str(&value).unwrap() as i64,
                    }
                }
            }
            Arg::F32 { value } => {
                SV {
                    value_ty: 0x7d,
                    v: SVV {
                        f32: f32::from_str(&value).unwrap(),
                    }
                }
            }
            Arg::F64 { value } => {
                SV {
                    value_ty: 0x7c,
                    v: SVV {
                        f64: f64::from_str(&value).unwrap(),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum A {
    #[serde(rename = "invoke")]
    Invoke {
        field: String,
        args: Vec<Arg>,
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
        line: u64,
        expected: Vec<Arg>,
    },
    #[serde(rename = "assert_invalid")]
    AssertInvalid {
        // action: A,
        // expected: Vec<Arg>,
    },
    #[serde(rename = "assert_malformed")]
    AssertMalformed {
        // action: A,
        // expected: Vec<Arg>,
    },
    #[serde(rename = "assert_trap")]
    AssertTrap {
        // action: A,
        // expected: Vec<Arg>,
    },
    #[serde(rename = "assert_exhaustion")]
    AssertExhaustion {
        // action: A,
        // expected: Vec<Arg>,
    }
}

#[repr(C)]
struct O {
    a: bool,
    b: bool,
    c: bool,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SafeSV {
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64)
}

#[repr(C)]
#[derive(Copy, Clone)]
union  SVV {
    u32: u32,
    i32: i32,
    u64: u64,
    i64: i64,
    f32: f32,
    f64: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SV {
    value_ty: u8,
    v: SVV,
}

impl SV {
    fn safe(&self) ->SafeSV {
        match self.value_ty {
            0x7e => SafeSV::I64(unsafe { self.v.i64 }),
            0x7f => SafeSV::I32(unsafe { self.v.i32 }),
            0x7d => SafeSV::F32(unsafe { self.v.f32 }),
            0x7c => SafeSV::F64(unsafe { self.v.f64 }),
            _ => panic!()
        }
    }
}

impl PartialEq<Self> for SV {
    fn eq(&self, other: &Self) -> bool {
        if self.value_ty != other.value_ty {
            return false;
        }

        match self.value_ty {
            0x7f => unsafe {
                self.v.i32 == other.v.i32
            }
            0x7d => unsafe {
                self.v.f32 == other.v.f32
            }
            0x7c => unsafe {
                self.v.f64 == other.v.f64
            }
            _ => {panic!("{:x}", self.value_ty)}
        }
    }
}

impl Eq for SV {

}

#[repr(C)]
struct Module {
    pc: u32,
    sp: u32,
    fp: u32,
    stack: [SV; 4*1024],
}


macro_rules! test {
    ($([$name: ident, $path: expr]),*,) => {
        $(
            #[test]
            pub fn $name() {
                run_test($path)
            }
        )*
    };
}

test! {
    [test_i32, "i32"],
    [test_i64, "i64"],
    [test_comments, "comments"],
    [test_int_exprs, "int_exprs"],
    [test_int_literals, "int_literals"],
    [test_labels, "labels"],
    [test_br, "br"],
    // [test_align, "align"],
   // [test_load, "load"],
   //  [test_call, "call"],

   // [test_if, "if"],
    // floats:
    // [test_address, "address"],
    // [test_const, "const"],

    // opc:
   // [test_nop, "nop"],
}

fn main() {

}

pub fn run_test(testset: &'static str) {
    let (load_module, get_export_fidx, invoke) = unsafe {
        let l = libc::dlopen(c"../bin/libwasm89.so".as_ptr(), libc::RTLD_NOW);
        let load_module = libc::dlsym(l, c"load_module".as_ptr());
        let load_module = core::mem::transmute::<_, extern "C" fn(*const u8, usize, O)->*mut Module>(load_module);
        let get_export_fidx = libc::dlsym(l, c"get_export_fidx".as_ptr());
        let get_export_fidx = core::mem::transmute::<_, extern "C" fn(*mut Module, *const i8)->usize>(get_export_fidx);
        let invoke = libc::dlsym(l, c"invoke".as_ptr());
        let invoke = core::mem::transmute::<_, extern "C" fn(*mut Module, usize)->R>(invoke);
        println!("{:x}", invoke as usize);

        (load_module, get_export_fidx, invoke)
    };

    // let testset = "i32";
    // let testset = "i64";
    // let testset = "address";

    // let conf = PathBuf::from_str("res/const/const.json").unwrap();
    let conf = PathBuf::from_str(&format!("res/{testset}/{testset}.json")).unwrap();

    // let t: T = serde_json::from_str(include_str!("../res/nop/nop.json")).unwrap();
    let t: T = serde_json::from_str(&std::fs::read_to_string(&conf).unwrap()).unwrap();
    let mut m = core::ptr::null_mut();
    for c in t.commands {
        match c {
            C::Module { filename } => {
                let mm = conf.parent().unwrap().join(filename);
                println!("{:?}", mm);
                let mm = std::fs::read(&mm).unwrap();
                m = load_module(mm.as_ptr(), mm.len(), O {
                    a: false,
                    b: false,
                    c: false,
                });
                // unsafe {
                //     m.as_mut().unwrap().fp = 0;
                //     let sp = m.as_mut().unwrap().sp+1;
                //     m.as_mut().unwrap().stack[sp as usize] = SV {
                //         value_ty: 0x7f,
                //         v: SVV {
                //             u32: 0x1234
                //         }
                //     };
                //     m.as_mut().unwrap().sp = sp;
                //
                // }
            }
            C::AssertReturn { action, expected, line } => {
                match action {
                    A::Invoke { field, args } => {
                        //print!("{testset}:{field}... ");

                        unsafe {
                            m.as_mut().unwrap().fp = m.as_mut().unwrap().sp;
                        }
                        for a in args {
                            let sp = unsafe { m.as_mut().unwrap().sp }.wrapping_add(1);

                            unsafe {
                                m.as_mut().unwrap().stack[sp as usize] = a.sv();
                            }
                            unsafe {                                    m.as_mut().unwrap().sp = sp; }
                        }

                        let fs = CString::new(field.clone()).unwrap();
                        let f = get_export_fidx(m, fs.as_ptr());
                        // println!("F = {f}");
                        let r = invoke(m, f);
                        if let SafeR::Err(s) = r.safe_r() {
                            panic!("{s}");
                        }
                        // println!("{:?}", r.safe_r());

                        assert!(expected.len() < 2);
                        if let Some(exp) = expected.first() {
                            let res = unsafe {
                                let sp = m.as_mut().unwrap().sp;
                                let res = m.as_mut().unwrap().stack[sp as usize];
                                m.as_mut().unwrap().sp = sp.wrapping_sub(1);
                                res
                            };

                            if res.safe() != exp.sv().safe() {
                                println!("field {testset}:{field}::{line} failed:");
                                println!("{:?} / {:x?}", res.safe(), res.safe());
                                println!("{:?} / {:x?}", exp.sv().safe(), exp.sv().safe());
                                panic!()
                                //println!("{}", console::style("failed").red());
                           } else {
                                //println!("{}", console::style("passed").green());
                            }
                        }
                    }
                }
            }
            C::AssertInvalid { .. } => {
                // print!("{testset}:{field}... ");
            }
            C::AssertMalformed {..} => {
                // print!("{testset}:{field}... ");
            }
            C::AssertTrap { .. } => {
                // print!("{testset}:{field}... ");
            }
            C::AssertExhaustion { .. } => {}
        }
    }
}
