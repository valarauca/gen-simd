
use super::regex::{Regex,Captures};
use std::hash::{Hash, SipHasher, Hasher};
use super::joiner::args_builder;

fn hash_str(s: &str) -> u64 {
    let mut sip = SipHasher::new();
    s.hash(&mut sip);
    sip.finish()
}

lazy_static! {
    static ref LLVMINTx86: Regex = Regex::new(
r#"/// The `(\S+)` intrinsic; known as `(__builtin_ia32_\S+)` in GCC\.\s*#\[link_name = "llvm\.x86\.(\S+?)\.\S+\s*pub fn \S+\(([\s\S]*?)\) -> (\S+);"#
    ).unwrap();
    static ref LLVMINTaarch64: Regex = Regex::new(
r#"/// The `(\S+)` intrinsic\.\s*#\[link_name = "llvm\.aarch64\S+"\]\s*pub fn (\S+)\(([\s\S]*?)\) -> (\S+);"#
    ).unwrap();
    static ref LLVMINTarm: Regex = Regex::new(
r#"/// The `(\S+)` intrinsic\.\s*#\[link_name = "llvm\.arm\.neon\S+"\]\s*pub fn neon_(\S+)\(([\s\S]*?)\) -> (\S+);"#
    ).unwrap();
    static ref TYPECODE: Regex = Regex::new(
r#"\w: ((?:\*mut )?[a-z:\d]+)"#
    ).unwrap();
}

#[inline]
fn cc<'a>(caps: &Captures<'a>, point: usize) -> &'a str {
    match caps.at(point) {
        Option::Some(x) => x.trim(),
        Option::None => unreachable!()
    }
}

fn get_args<'a>(buffer: &'a str) -> Vec<&'a str> {
    TYPECODE.captures_iter(buffer).map(|x| cc(&x,1)).collect()
}

#[derive(Debug)]
pub struct Llvmint<'a> {
    pub llvm_code: &'a str,
    pub gnu_builtin: &'a str,
    pub gnu_hash: u64,
    pub features: &'a str,
    pub ret_val: &'a str,
    pub args: Vec<&'a str>
}
impl<'a> Llvmint<'a> {
    pub fn new_x86(caps: Captures<'a>) -> Llvmint<'a> {
        Llvmint {
            llvm_code: cc(&caps,1),
            gnu_builtin: cc(&caps,2),
            gnu_hash: hash_str(cc(&caps,2)),
            features: cc(&caps,3),
            ret_val: cc(&caps,5),
            args: get_args(cc(&caps,4))
        }
    }
    pub fn new_aarch64(caps: Captures<'a>) -> Llvmint<'a> {
        Llvmint {
            llvm_code: cc(&caps,1),
            gnu_builtin: cc(&caps,2),
            gnu_hash: 0u64,
            features: "",
            ret_val: cc(&caps,4),
            args: get_args(cc(&caps,3))
        }
    }
}

pub fn populate_llvmint_x86<'a>(buffer: &'a str) -> Vec<Llvmint<'a>> {
    LLVMINTx86.captures_iter(buffer).map(Llvmint::new_x86).collect()
}

pub fn populate_llvmint_aarch64<'a>(buffer: &'a str) -> Vec<Llvmint<'a>> {
    LLVMINTaarch64.captures_iter(buffer).map(Llvmint::new_aarch64).collect()
}

pub fn populate_llvmint_arm<'a>(buffer: &'a str) -> Vec<Llvmint<'a>> {
    LLVMINTarm.captures_iter(buffer).map(Llvmint::new_aarch64).collect()
}

#[test]
fn test_llvmint_x86() {
    let s =r#"

        /// The `llvm.x86.sse2.psrl.dq` intrinsic; known as `__builtin_ia32_psrldqi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrl.dq"]
        pub fn sse2_psrl_dq(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psll.dq.bs` intrinsic; known as `__builtin_ia32_pslldqi128_byteshift` in GCC.
        #[link_name = "llvm.x86.sse2.psll.dq.bs"]
        pub fn sse2_psll_dq_bs(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.mfence` intrinsic; known as `__builtin_ia32_mfence` in GCC.
        #[link_name = "llvm.x86.sse2.mfence"]
        pub fn sse2_mfence() -> ();"#;

    let llvm = populate_llvmint_x86(s);
    assert_eq!( llvm.len(), 3);

    //validate each command
    assert_eq!( llvm[0].llvm_code, "llvm.x86.sse2.psrl.dq");
    assert_eq!( llvm[0].gnu_builtin, "__builtin_ia32_psrldqi128");
    assert_eq!( llvm[0].features, "sse2");
    assert_eq!( llvm[0].ret_val, "::simdty::i64x2");
    assert_eq!( llvm[0].args.len(), 2);
    assert_eq!( llvm[0].args[0], "::simdty::i64x2");
    assert_eq!( llvm[0].args[1], "i32");
    assert_eq!( args_builder(&llvm[0]), "[&::I64x2,&::I32]");

    assert_eq!( llvm[1].llvm_code, "llvm.x86.sse2.psll.dq.bs");
    assert_eq!( llvm[1].gnu_builtin, "__builtin_ia32_pslldqi128_byteshift");
    assert_eq!( llvm[1].features, "sse2");
    assert_eq!( llvm[1].ret_val, "::simdty::i64x2");
    assert_eq!( llvm[1].args.len(), 2);
    assert_eq!( llvm[1].args[0], "::simdty::i64x2");
    assert_eq!( llvm[1].args[1], "i32");
    assert_eq!( args_builder(&llvm[1]), "[&::I64x2,&::I32]");

    assert_eq!( llvm[2].llvm_code, "llvm.x86.sse2.mfence");
    assert_eq!( llvm[2].gnu_builtin, "__builtin_ia32_mfence");
    assert_eq!( llvm[2].features, "sse2");
    assert_eq!( llvm[2].ret_val, "()");
    assert_eq!( llvm[2].args.len(), 0);
    assert_eq!( args_builder(&llvm[2]), "[]");
}

#[test]
fn test_llvmint_aarch() {
    let s = r#"
           /// The `llvm.aarch64.clrex` intrinsic.
           #[link_name = "llvm.aarch64.clrex"]
           pub fn clrex() -> ();
           /// The `llvm.aarch64.sdiv.v16i8` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.v16i8"]
           pub fn sdiv_v16i8(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
           /// The `llvm.aarch64.sdiv.i8` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.i8"]
           pub fn sdiv_i8(a: i8, b: i8) -> i8;
           /// The `llvm.aarch64.sdiv.v8i16` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.v8i16"]
           pub fn sdiv_v8i16(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
           /// The `llvm.aarch64.sdiv.i16` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.i16"]
           pub fn sdiv_i16(a: i16, b: i16) -> i16;
           /// The `llvm.aarch64.sdiv.v4i32` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.v4i32"]
           pub fn sdiv_v4i32(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
           /// The `llvm.aarch64.sdiv.i32` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.i32"]
           pub fn sdiv_i32(a: i32, b: i32) -> i32;
           /// The `llvm.aarch64.sdiv.v2i64` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.v2i64"]
           pub fn sdiv_v2i64(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
           /// The `llvm.aarch64.sdiv.i64` intrinsic.
           #[link_name = "llvm.aarch64.sdiv.i64"]
           pub fn sdiv_i64(a: i64, b: i64) -> i64;
           /// The `llvm.aarch64.udiv.v16i8` intrinsic.
           #[link_name = "llvm.aarch64.udiv.v16i8"]
           pub fn udiv_v16i8(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
           /// The `llvm.aarch64.hint` intrinsic.
           #[link_name = "llvm.aarch64.hint"]
           pub fn hint(a: i32) -> ();"#;

    let llvm = populate_llvmint_aarch64(s);
    assert_eq!( llvm.len(), 11);

    //validate each command
    assert_eq!( llvm[0].llvm_code, "llvm.aarch64.clrex");
    assert_eq!( llvm[0].gnu_builtin, "clrex");
    assert_eq!( llvm[0].ret_val, "()");
    assert_eq!( llvm[0].args.len(), 0);
    assert_eq!( args_builder(&llvm[0]), "[]");
    assert_eq!( llvm[1].llvm_code, "llvm.aarch64.sdiv.v16i8");
    assert_eq!( llvm[1].gnu_builtin, "sdiv_v16i8");
    assert_eq!( llvm[1].ret_val, "::simdty::i8x16");
    assert_eq!( llvm[1].args.len(), 2);
    assert_eq!( llvm[1].args[0], "::simdty::i8x16");
    assert_eq!( llvm[1].args[1], "::simdty::i8x16");
    assert_eq!( args_builder(&llvm[1]), "[&::I8x16,&::I8x16]");
}
