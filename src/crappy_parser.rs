use super::regex::{Regex, Captures};
use std::hash::{Hash, SipHasher, Hasher};


lazy_static! {
    static ref GETSYMBOL: Regex = Regex::new(
    r"extern\s*__inline\s*\S+\s*__attribute__\s*\(\(__gnu_inline__, __always_inline__, __artificial__\)\)\s*(\S+) \([\s\S]*?\)\s*\{[\S\s]*?(__builtin_ia32_\S+) [\s\S]+?\}"
    ).unwrap();
}


fn hash_str(s: &str) -> u64 {
    let mut sip = SipHasher::new();
    s.hash(&mut sip);
    sip.finish()
}

#[derive(Debug)]
pub struct Intrinsic {
    pub name: String,
    pub gnu_builtin: String,
    pub gnu_hash: u64
}
impl Intrinsic {
    pub fn new(caps: Captures) -> Intrinsic {
        Intrinsic {
            name: caps.at(1).unwrap().to_string(),
            gnu_builtin: caps.at(2).unwrap().to_string(),
            gnu_hash: hash_str(caps.at(2).unwrap())
        }
    }
}

pub fn populate_intrinsic(buffer: &str) -> Vec<Intrinsic> {
    GETSYMBOL.captures_iter(buffer).map(Intrinsic::new).collect()
}

#[test]
fn test_populate() {
    let buffer = r#"
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_multishift_epi64_epi8 (__mmask32 __M, __m256i __X, __m256i __Y)
{
   return (__m256i) __builtin_ia32_vpmultishiftqb256_mask ((__v32qi) __X,
    							  (__v32qi) __Y,
    							  (__v32qi)
    							  _mm256_setzero_si256 (),
    							  (__mmask32) __M);
}

/* Sets the low SPFP value of A from the low value of B.  */
extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_move_ss (__m128 __A, __m128 __B)
{
   return (__m128) __builtin_ia32_movss ((__v4sf)__A, (__v4sf)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_abs_epi16 (__m256i __A)
{
   return (__m256i)__builtin_ia32_pabsw256 ((__v16hi)__A);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_stream_ss (float * __P, __m128 __Y)
{
  __builtin_ia32_movntss (__P, (__v4sf) __Y);
}
"#;

    //parse selected text
    let intrinsic = populate_intrinsic(buffer);

    //check length

    //break down intrinsic0
    assert_eq!(intrinsic[0].name, "_mm256_maskz_multishift_epi64_epi8");
    assert_eq!(intrinsic[0].gnu_builtin, "__builtin_ia32_vpmultishiftqb256_mask");

    //break down intrinsic1
    assert_eq!(intrinsic[1].name, "_mm_move_ss");
    assert_eq!(intrinsic[1].gnu_builtin, "__builtin_ia32_movss");

    //break down intrinsic2
    assert_eq!(intrinsic[2].name, "_mm256_abs_epi16");
    assert_eq!(intrinsic[2].gnu_builtin, "__builtin_ia32_pabsw256");

    assert_eq!(intrinsic[3].name, "_mm_stream_ss");
    assert_eq!(intrinsic[3].gnu_builtin, "__builtin_ia32_movntss");
}
