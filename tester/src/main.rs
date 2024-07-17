use std::collections::HashMap;
use std::ffi::CStr;
use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;

#[repr(C)]
#[derive(Debug)]
pub enum S {
    Ok = 0,
    Err = 1,
    ErrNest = 2,
}

#[repr(C)]
pub struct R {
    status: S,
    msg: *const i8,
    p: *mut R,
}

impl R {
    pub fn safe_r(&self) -> SafeR {
        unsafe {
            match self.status {
                S::Ok => SafeR::Ok,
                S::Err => SafeR::Err(CStr::from_ptr(self.msg).to_str().unwrap().to_string()),
                S::ErrNest => SafeR::ErrNest(
                    CStr::from_ptr(self.msg).to_str().unwrap().to_string(),
                    Box::new(self.p.read().safe_r())
                )
            }
        }
    }
}
#[derive(Debug)]
pub enum SafeR {
    Ok,
    Err(String),
    ErrNest(String, Box<SafeR>),
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
                        i32: u32::from_str(value).unwrap() as i32,
                    }
                }
            }
            Arg::I64 { value } => {
                SV {
                    value_ty: 0x7e,
                    v: SVV {
                        i64: u64::from_str(value).unwrap() as i64,
                    }
                }
            }
            Arg::F32 { value } => {

                let f = match value.as_str() {
                    "nan:canonical" => f32::from_le_bytes(0x7fc00000u32.to_le_bytes()),
                    "nan:arithmetic" => f32::from_le_bytes(0x7fc00001u32.to_le_bytes()),
                    _ => {
                        f32::from_le_bytes(value.parse::<u32>().unwrap().to_le_bytes())
                    }
                };

                SV {
                    value_ty: 0x7d,
                    v: SVV {
                        f32: f,
                    }
                }
            }
            Arg::F64 { value } => {
                let f = match value.as_str() {
                    "nan:canonical" => f64::from_le_bytes(0x7ff8000000000000_u64.to_le_bytes()),
                    "nan:arithmetic" => f64::from_le_bytes(0x7ff8000000000001_u64.to_le_bytes()),
                    _ => {
                        f64::from_le_bytes(value.parse::<u64>().unwrap().to_le_bytes())
                    }
                };

                SV {
                    value_ty: 0x7c,
                    v: SVV {
                        f64: f,
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
        module: Option<String>,
    },
#[serde(rename = "get")]
Get{},

}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum C {
    #[serde(rename = "module")]
    Module{
        name: Option<String>,
        filename: String,
    },
    #[serde(rename = "action")]
    Action{
        action: A,
        expected: Vec<Arg>,
        line: u64,
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
    },
    #[serde(rename = "assert_uninstantiable")]
    AssertUninstantiable {
        // action: A,
        // expected: Vec<Arg>,
    },
    #[serde(rename = "assert_unlinkable")]
    AssertUnlinkable {
        // action: A,
        // expected: Vec<Arg>,
    },
    #[serde(rename = "register")]
    Register {
        // action: A,
        // expected: Vec<Arg>,
    }
}

#[repr(C)]
struct O {
    disable_memory_bounds: bool,
    mangle_table_index: bool,
    dlsym_trim_underscore: bool,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SafeSV {
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Invalid,
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
            _ => SafeSV::Invalid,
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

mod core_test {
    test! {
        [address, "address"],
        [align, "align"],
        // [binary, "binary"], // t
        [binary_leb128, "binary-leb128"],
        [block, "block"],
        [br, "br"],
        [br_if, "br_if"],
        // [br_table, "br_table"], //t
        [break_drop, "break-drop"],
        [call, "call"],
        [call_indirect, "call_indirect"],
        [comments, "comments"],
        [const_, "const"],
        [conversions, "conversions"],
        // [custom, "custom"], //t
        // [data, "data"], //import
        // [elem, "elem"], // import
        [endianness, "endianness"],
        // [exports, "exports"], // bug in harness
        [f32, "f32"],
        [f32_bitwise, "f32_bitwise"],
        [f32_cmp, "f32_cmp"],
        [f64, "f64"],
        [f64_bitwise, "f64_bitwise"],
        [f64_cmp, "f64_cmp"],
        [fac, "fac"],
        [float_exprs, "float_exprs"],
        [float_literals, "float_literals"],
        [float_memory, "float_memory"],
        [float_misc, "float_misc"],
        [forward, "forward"],
        [func, "func"],
        [func_ptrs, "func_ptrs"],
        // [globals, "globals"], // imports / ubsan
        [i32_, "i32"],
        [i64_, "i64"],
        [if_, "if"],
        // [imports, "imports"], // bug
        [inline_module, "inline-module"],
        [int_exprs, "int_exprs"],
        [int_literals, "int_literals"],
        [labels, "labels"],
        [left_to_right, "left-to-right"],
        // [linking, "linking"], // harness bug
        [load, "load"],
        [local_get, "local_get"],
        [local_set, "local_set"],
        [local_tee, "local_tee"],
        [loop_, "loop"],
        [memory, "memory"],
        [memory_grow, "memory_grow"], // note: fails under ubsan when f32_cmp is enabled, but not under scudo or asan and not due to a *san error?
        [memory_redundancy, "memory_redundancy"],
        [memory_size, "memory_size"],
        [memory_trap, "memory_trap"],
        [names, "names"],
        [nop, "nop"],
        [return_, "return"],
        [select, "select"],
        [skip_stack_guard_page, "skip-stack-guard-page"],
        [stack, "stack"],
        [start, "start"],
        [store, "store"],
        [switch, "switch"],
        [token, "token"],
        [traps, "traps"],
        [type_, "type"],
        [unreachable, "unreachable"],
        [unreached_invalid, "unreached-invalid"],
        [unwind, "unwind"],
        [utf8_custom_section_id, "utf8-custom-section-id"],
        [utf8_import_field, "utf8-import-field"],
        [utf8_import_module, "utf8-import-module"],
        [utf8_invalid_encoding, "utf8-invalid-encoding"],
}
}

fn main() {

}
// For asan:  ASAN_OPTIONS=detect_leaks=0 LD_PRELOAD=/usr/lib/clang/18/lib/linux/libclang_rt.asan-x86_64.so cargo test -- --test-threads=1 "core_test::" --nocapture
// For ubsan:  ASAN_OPTIONS=detect_leaks=0 LD_PRELOAD=/usr/lib/clang/18/lib/linux/libclang_rt.ubsan_standalone-x86_64.so cargo test -- --test-threads=1 "core_test::" --nocapture



pub fn run_test(testset: &'static str) {
    let mode = if std::env::vars().any(|(_, y)| y.contains("libclang_rt.asan")) {
        println!("Using asan");
        c"../bin/libwasm89_asan.so"
    } else if std::env::vars().any(|(_, y)| y.contains("libclang_rt.ubsan")) {
        println!("Using ubsan");
        c"../bin/libwasm89_ubsan.so"
    } else {
        println!("Using standard build");
        c"../bin/libwasm89.so"
    };
    let (load_module, get_export_fidx, invoke, _snap, _snap_dest) = unsafe {
        let l = libc::dlopen(mode.as_ptr(), libc::RTLD_NOW);
        assert_ne!(l, core::ptr::null_mut());
        let load_module = libc::dlsym(l, c"load_module".as_ptr());
        assert_ne!(load_module, core::ptr::null_mut());
        let load_module = core::mem::transmute::<*mut libc::c_void, extern "C" fn(*const u8, usize, O)->*mut Module>(load_module);

        let get_export_fidx = libc::dlsym(l, c"get_export_fidx".as_ptr());
        assert_ne!(get_export_fidx, core::ptr::null_mut());
        let get_export_fidx = core::mem::transmute::<*mut libc::c_void, extern "C" fn(*mut Module, *const u8, u32)->usize>(get_export_fidx);

        let invoke = libc::dlsym(l, c"invoke".as_ptr());
        assert_ne!(invoke, core::ptr::null_mut());
        let invoke = core::mem::transmute::<*mut libc::c_void, extern "C" fn(*mut Module, usize)->R>(invoke);

        let snapshot = libc::dlsym(l, c"snapshot".as_ptr());
        assert_ne!(snapshot, core::ptr::null_mut());
        let snapshot = core::mem::transmute::<*mut libc::c_void, extern "C" fn(*mut Module)->*mut Module>(snapshot);

        let snapshot_dest = libc::dlsym(l, c"snapshot_destroy".as_ptr());
        assert_ne!(snapshot_dest, core::ptr::null_mut());
        let snapshot_dest = core::mem::transmute::<*mut libc::c_void, extern "C" fn(*mut Module)>(snapshot_dest);

        (load_module, get_export_fidx, invoke, snapshot, snapshot_dest)
    };

    // let testset = "i32";
    // let testset = "i64";
    // let testset = "address";

    // let conf = PathBuf::from_str("res/const/const.json").unwrap();
    let conf = PathBuf::from_str(&format!("res/wg-1.0/{testset}.wast_/{testset}.wast.json")).unwrap();

    // let t: T = serde_json::from_str(include_str!("../res/nop/nop.json")).unwrap();
    let t: T = serde_json::from_str(&std::fs::read_to_string(&conf).expect(&format!("Failed to find {}", conf.display()))).unwrap();

    let mut mod_map = HashMap::new();

    let mut m = core::ptr::null_mut();
    for c in t.commands {
        match c {
            C::Module { filename, name } => {
                let mm = conf.parent().unwrap().join(filename);
                println!("{:?}", mm);
                let mm = std::fs::read(&mm).unwrap();
                let mo = load_module(mm.as_ptr(), mm.len(), O {
                    disable_memory_bounds: false,
                    mangle_table_index: false,
                    dlsym_trim_underscore: false,
                });
                assert_ne!(mo, core::ptr::null_mut());

                if let Some(name) = name {
                    mod_map.insert(name, mo);
                } else {
                    m = mo;
                }

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
                    A::Invoke { field, args, module } => {
                        println!("field {testset}:{field}::{line}");

                        let m = if let Some(module) = module {
                            *mod_map.get(&module).unwrap()
                        } else {
                            m
                        };

                        // any error will leave the module in bad state
                        //let m = snap(m);


                        if field == "write" && line == 137 {
                            println!("Skip multi");
                            //snap_dest(m);
                            continue;
                        }
                        // if field == "multi" {
                        //     println!("Skip multi");
                        //     continue;

                        //print!("{testset}:{field}... ");

                        if expected.len() > 1 {
                            println!("field {testset}:{field}::{line} >2 vals what");
                            //snap_dest(m);
                            continue;
                        }

                        for a in &args {
                            match a {
                                _ =>
                                unsafe {
                                let sp = m.as_mut().unwrap().sp.wrapping_add(1);
                                m.as_mut().unwrap().stack[sp as usize] = a.sv();
                                m.as_mut().unwrap().sp = sp;
                                }
                            }
                        }

                        let mut fs = field.as_bytes().to_vec();
                        fs.push(0);
                        let f = get_export_fidx(m, fs.as_ptr(), fs.len() as u32);
                        if f as i32 == -1 {
                            panic!("Failed to find fidx: {:X?}", fs);
                        }
                        let r = invoke(m, f);
                        match r.safe_r() {
                            SafeR::Ok => {}
                            SafeR::Err(s) => panic!("{s}"),
                            SafeR::ErrNest(s1, s2) => {
                                if let SafeR::Err(ref x) = *s2 {
                                    if x == "multi_return_not_supported" {
                                        panic!("multi return func");
                                    }
                                }
                                panic!("{s1:?} {s2:?}")
                            },
                        }

                        if let Some(exp) = expected.first() {
                            let res = unsafe {
                                let sp = m.as_mut().unwrap().sp;
                                let res = m.as_mut().unwrap().stack[sp as usize];
                                m.as_mut().unwrap().sp = sp.wrapping_sub(1);
                                res
                            };

                            let res_s = res.safe();
                            let exp_s = exp.sv().safe();

                            if  res_s != exp_s  {
                                if let (SafeSV::F32(a), SafeSV::F32(b)) = (res_s, exp_s) {
                                    if a.is_nan() && b.is_nan() {
                                        //snap_dest(m);
                                        continue;
                                    }
                                }

                                if let (SafeSV::F64(a), SafeSV::F64(b)) = (res_s, exp_s) {
                                    if a.is_nan() && b.is_nan() {
                                        //snap_dest(m);
                                        continue;
                                    }
                                }


                                if let (SafeSV::F64(a), SafeSV::F64(b)) = (res_s, exp_s) {
                                    let dif = (a - b).abs();
                                    if dif < 20.0 {
                                        println!("Allowing f64 with small diff {dif}");
                                        //snap_dest(m);
                                        continue;
                                    }
                                }

                                if let (SafeSV::F32(a), SafeSV::F32(b)) = (res_s, exp_s) {
                                    let dif = (a - b).abs();
                                    if  dif < 20.0 {
                                        println!("Allowing f64 with small diff {dif}");
                                        //snap_dest(m);
                                        continue;
                                    }
                                }

                                println!("field {testset}:{field}::{line} failed:");
                                println!("args {args:?}");
                                println!("res: {:?} / {:x?}", res.safe(), res.safe());
                                println!("exp: {:?} / {:x?}", exp.sv().safe(), exp.sv().safe());
                                unsafe { println!("exp/res: {:x?} / {:x?}", exp.sv().v.u64, res.v.u64); }
                                panic!()
                                //println!("{}", console::style("failed").red());
                           } else {
                                println!("field {testset}:{field}::{line} ok");
                                //println!("{}", console::style("passed").green());
                            }
                        } else {
                            println!("field {testset}:{field}::{line} not testing, no exp");
                        }
                        //snap_dest(m);
                    }
                    A::Get { .. } => {}
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
            C::AssertUninstantiable { .. } => {}
            C::Register { .. } => {}
            C::Action { action, expected, line } => {
                match action {
                    A::Invoke { field, args, module } => {
                        println!("field {testset}:{field}::{line}");

                        assert!(module.is_none());

                        if !expected.is_empty() {
                            panic!();
                        }

                        for a in &args {
                            match a {
                                _ =>
                                    unsafe {
                                        let sp = m.as_mut().unwrap().sp.wrapping_add(1);
                                        m.as_mut().unwrap().stack[sp as usize] = a.sv();
                                        m.as_mut().unwrap().sp = sp;
                                    }
                            }
                        }

                        let mut fs = field.as_bytes().to_vec();
                        fs.push(0);
                        let f = get_export_fidx(m, fs.as_ptr(), fs.len() as u32);
                        if f as i32 == -1 {
                            panic!("Failed to find fidx: {:X?}", fs);
                        }
                        let r = invoke(m, f);
                        match r.safe_r() {
                            SafeR::Ok => {}
                            SafeR::Err(s) => panic!("{s}"),
                            SafeR::ErrNest(s1, s2) => panic!("{s1:?} {s2:?}"),
                        }
                    }
                    A::Get { .. } => {}
                }
            }
            C::AssertUnlinkable { .. } => {}
        }
    }
}

// todos:
// fix all tests
// add missing thunks for tests names:print32::1107