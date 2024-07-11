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
                    crate::run_test($path)
            }
        )*
    };
}

//find . -name \*.wast | xargs -I{} mkdir {}_
//find . -name \*.wast | xargs -I{} wast2json {} -o {}_/{}.json
mod core_test {
    test! {
        [address, "address"],
        [align, "align"],
        // [binary, "binary"],
        [binary_leb128, "address"],
        // [block, "block"],
        // [br, "br"],
        // [br_if, "br_if"],
        // [br_table, "br_table"],
        // [bulk, "bulk"],
        // [call, "call"],
        // [call_indirect, "call_indirect"],
        [comments, "comments"],
        [const_, "const"],
        // [convertions, "convertions"],
        // [custom, "custom"],
        // [data, "data"],
        // [elem, "elem"],
        [endianness, "endianness"],
        // [exports, "exports"],
        [f32, "f32"],
        [f32_bitwise, "f32_bitwise"],
        [f32_cmp, "f32_cmp"],
        [f64, "f64"],
        [f64_bitwise, "f64_bitwise"],
        [f64_cmp, "f64_cmp"],
        // [fac, "fac"],
        // [float_exprs, "float_exprs"],
        [float_literals, "float_literals"],
        // [float_memory, "float_memory"],
        [float_misc, "float_misc"],
        [forward, "forward"],
        // [func, "func"],
        // [func_ptrs, "func_ptrs"],
        // [global, "global"],
        [i32_, "i32"],
        [i64_, "i64"],
        // [if_, "if"],
        // [imports, "imports"],
        [inline_module, "inline-module"],
        [int_exprs, "int_exprs"],
        [int_literals, "int_literals"],
        [labels, "labels"],
        // [left_to_right, "left-to-right"],
        // [linking, "linking"],
        // [load, "load"],
        [local_get, "local_get"],



   // [test_nop, "nop"],
}
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
    let conf = PathBuf::from_str(&format!("res/{testset}.wast_/{testset}.wast.json")).unwrap();

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

                        if expected.len() < 2 {
                            println!(">2 vals what");
                            continue;
                        }

                        if let Some(exp) = expected.first() {
                            let res = unsafe {
                                let sp = m.as_mut().unwrap().sp;
                                let res = m.as_mut().unwrap().stack[sp as usize];
                                m.as_mut().unwrap().sp = sp.wrapping_sub(1);
                                res
                            };

                            // if let SafeSV::F32(_) = res.safe() {
                            //     println!("f32 detected");
                            //     continue;
                            // }

                            // if let SafeSV::F64(_) = res.safe() {
                            //     println!("f64 detected");
                            //     continue;
                            // }

                            if res.safe() != exp.sv().safe() {
                                println!("field {testset}:{field}::{line} failed:");
                                println!("res: {:?} / {:x?}", res.safe(), res.safe());
                                println!("exp: {:?} / {:x?}", exp.sv().safe(), exp.sv().safe());
                                unsafe { println!("res/exp: {:x?} / {:x?}", exp.sv().v.u64, res.v.u64); }
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
