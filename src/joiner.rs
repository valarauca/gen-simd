use super::crappy_parser::Intrinsic;
use super::llvmint_parser::Llvmint;

//conert an input type to an output type
fn to_type(s: &str) -> Option<String> {
    match s {
        "()" => Some("&::VOID".to_string()),
        "i8" => Some("&::I8".to_string()),
        "u8" => Some("&::I8".to_string()),
        "::simdty::i8x8" => Some("&::I8x8".to_string()),
        "::simdty::u8x8" => Some("&::U8x8".to_string()),
        "::simdty::i8x16" => Some("&::I8x16".to_string()),
        "::simdty::u8x16" => Some("&::U8x16".to_string()),
        "::simdty::i8x32" => Some("&::I8x32".to_string()),
        "::simdty::u8x32" => Some("&::U8x32".to_string()),
        "::simdty::i8x64" => Some("&::I8x64".to_string()),
        "::simdty::u8x64" => Some("&::U8x64".to_string()),
        "i16" => Some("&::I16".to_string()),
        "u16" => Some("&::U16".to_string()),
        "::simdty::i16x4" => Some("&::I16x4".to_string()),
        "::simdty::u16x4" => Some("&::U16x4".to_string()),
        "::simdty::i16x8" => Some("&::I16x8".to_string()),
        "::simdty::u16x8" => Some("&::U16x8".to_string()),
        "::simdty::i16x16" => Some("&::I16x16".to_string()),
        "::simdty::u16x16" => Some("&::U16x16".to_string()),
        "::simdty::i16x32" => Some("&::I16x32".to_string()),
        "::simdty::u16x32" => Some("&::U16x32".to_string()),
        "i32" => Some("&::I32".to_string()),
        "u32" => Some("&::U32".to_string()),
        "::simdty::i32x2" => Some("&::I32x2".to_string()),
        "::simdty::u32x2" => Some("&::U32x2".to_string()),
        "::simdty::i32x4" => Some("&::I32x4".to_string()),
        "::simdty::u32x4" => Some("&::U32x4".to_string()),
        "::simdty::i32x8" => Some("&::I32x8".to_string()),
        "::simdty::u32x8" => Some("&::U32x8".to_string()),
        "::simdty::i32x16" => Some("&::I32x16".to_string()),
        "::simdty::u32x16" => Some("&::U32x16".to_string()),
        "i64" => Some("&::I64".to_string()),
        "u64" => Some("&::U64".to_string()),
        "::simdty::i64x1" => Some("&::I64x1".to_string()),
        "::simdty::u64x1" => Some("&::U64x1".to_string()),
        "::simdty::i64x2" => Some("&::I64x2".to_string()),
        "::simdty::u64x2" => Some("&::U64x2".to_string()),
        "::simdty::i64x4" => Some("&::I64x4".to_string()),
        "::simdty::u64x4" => Some("&::U64x4".to_string()),
        "::simdty::i64x8" => Some("&::I64x8".to_string()),
        "::simdty::u64x8" => Some("&::U64x8".to_string()),
        "f32" => Some("&::F32".to_string()),
        "::simdty::f32x2" => Some("&::F32x2".to_string()),
        "::simdty::f32x4" => Some("&::F32x4".to_string()),
        "::simdty::f32x8" => Some("&::F32x8".to_string()),
        "::simdty::f32x16" => Some("&::F32x16".to_string()),
        "f64" => Some("&::F64".to_string()),
        "::simdty::f64x1" => Some("&::F64x1".to_string()),
        "::simdty::f64x2" => Some("&::F64x2".to_string()),
        "::simdty::f64x4" => Some("&::F64x4".to_string()),
        "::simdty::f64x8" => Some("&::F64x8".to_string()),
        "*mut i8" => Some("{ static PTR: Type = Type::Pointer(&::I8, Some(&::I8), true); &PTR }".to_string()),
        "*mut u8" => Some("{ static PTR: Type = Type::Pointer(&::I8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i8x8" => Some("{ static PTR: Type = Type::Pointer(&::I8x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u8x8" => Some("{ static PTR: Type = Type::Pointer(&::U8x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i8x16" => Some("{ static PTR: Type = Type::Pointer(&::I8x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u8x16" => Some("{ static PTR: Type = Type::Pointer(&::U8x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i8x32" => Some("{ static PTR: Type = Type::Pointer(&::I8x32, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u8x32" => Some("{ static PTR: Type = Type::Pointer(&::U8x32, Some(&::I8), true); &PTR }".to_string()),
        "*mut i16" => Some("{ static PTR: Type = Type::Pointer(&::I16, Some(&::I8), true); &PTR }".to_string()),
        "*mut u16" => Some("{ static PTR: Type = Type::Pointer(&::U16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i16x4" => Some("{ static PTR: Type = Type::Pointer(&::I16x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u16x4" => Some("{ static PTR: Type = Type::Pointer(&::U16x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i16x8" => Some("{ static PTR: Type = Type::Pointer(&::I16x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u16x8" => Some("{ static PTR: Type = Type::Pointer(&::U16x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i16x16" => Some("{ static PTR: Type = Type::Pointer(&::I16x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u16x16" => Some("{ static PTR: Type = Type::Pointer(&::U16x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut i32" => Some("{ static PTR: Type = Type::Pointer(&::I32, Some(&::I8), true); &PTR }".to_string()),
        "*mut u32" => Some("{ static PTR: Type = Type::Pointer(&::U32, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i32x2" => Some("{ static PTR: Type = Type::Pointer(&::I32x2, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u32x2" => Some("{ static PTR: Type = Type::Pointer(&::U32x2, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i32x4" => Some("{ static PTR: Type = Type::Pointer(&::I32x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u32x4" => Some("{ static PTR: Type = Type::Pointer(&::U32x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i32x8" => Some("{ static PTR: Type = Type::Pointer(&::I32x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u32x8" => Some("{ static PTR: Type = Type::Pointer(&::U32x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut i64" => Some("{ static PTR: Type = Type::Pointer(&::I64, Some(&::I8), true); &PTR }".to_string()),
        "*mut u64" => Some("{ static PTR: Type = Type::Pointer(&::U64, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i64x1" => Some("{ static PTR: Type = Type::Pointer(&::I64x1, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u64x1" => Some("{ static PTR: Type = Type::Pointer(&::U64x1, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i64x2" => Some("{ static PTR: Type = Type::Pointer(&::I64x2, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u64x2" => Some("{ static PTR: Type = Type::Pointer(&::U64x2, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i64x4" => Some("{ static PTR: Type = Type::Pointer(&::I64x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u64x4" => Some("{ static PTR: Type = Type::Pointer(&::U64x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut f32" => Some("{ static PTR: Type = Type::Pointer(&::F32, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f32x2" => Some("{ static PTR: Type = Type::Pointer(&::F32x2, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f32x4" => Some("{ static PTR: Type = Type::Pointer(&::F32x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f32x8" => Some("{ static PTR: Type = Type::Pointer(&::F32x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut f64" => Some("{ static PTR: Type = Type::Pointer(&::F64, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f64x1" => Some("{ static PTR: Type = Type::Pointer(&::F64x1, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f64x2" => Some("{ static PTR: Type = Type::Pointer(&::F64x2, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f64x4" => Some("{ static PTR: Type = Type::Pointer(&::F64x4, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f64x8" => Some("{ static PTR: Type = Type::Pointer(&::F64x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::f32x16" => Some("{ static PTR: Type = Type::Pointer(&::F32x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i64x8" => Some("{ static PTR: Type = Type::Pointer(&::I64x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u64x8" => Some("{ static PTR: Type = Type::Pointer(&::U64x8, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i32x16" => Some("{ static PTR: Type = Type::Pointer(&::I32x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u32x16" => Some("{ static PTR: Type = Type::Pointer(&::U32x16, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i16x32" => Some("{ static PTR: Type = Type::Pointer(&::I16x32, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u16x32" => Some("{ static PTR: Type = Type::Pointer(&::U16x32, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::i8x64" => Some("{ static PTR: Type = Type::Pointer(&::I8x64, Some(&::I8), true); &PTR }".to_string()),
        "*mut ::simdty::u8x64" => Some("{ static PTR: Type = Type::Pointer(&::U8x64, Some(&::I8), true); &PTR }".to_string()),
        _ => None
    }
}

//build a prototype intrinisic match arm
fn match_arm(name: &str,inputs: usize, args: &str, output: &str, def: &str, header: &str) -> String {
    format!("
       \"{7}{0}\" => Intrinsic {5}
            inputs: {5} static INPUTS: [&'static Type; {1}] = {2}; &INPUTS {6},
            output: {3},
            definition: Named(\"{4}\")
        {6}",name,inputs,args,output,def,"{","}", header)
}

pub fn args_builder<'a>(llvm: &Llvmint<'a>) -> String {
    let mut buffer = String::with_capacity(4000);
    buffer.push('[');
    for arg in llvm.args.iter() {
        match to_type(arg) {
            Option::None => panic!("type not found in {:?}", llvm),
            Option::Some(ref x) => {
                buffer.push_str(x);
            }
        };
        buffer.push(',');
    }
    if buffer.len() > 5 {
        buffer.pop();
    }
    buffer.push(']');
    buffer
}


pub fn joiner_x86<'a>(llvm: &Llvmint<'a>, gnu: &Intrinsic) -> String {
    match_arm(
        &gnu.name,
        llvm.args.len(),
        &args_builder(llvm),
        &to_type(llvm.ret_val).expect("Could not unwrap retvalue"),
        llvm.llvm_code,
        "x86")
}

pub fn joiner_generic<'a>(llvm: &Llvmint<'a>, header: &str) -> String {
    match_arm(
        &llvm.gnu_builtin,
        llvm.args.len(),
        &args_builder(llvm),
        &to_type(llvm.ret_val).expect("Could not unwrap retvalue"),
        llvm.llvm_code,
        header)
}
