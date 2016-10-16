# gen_simd
This is a program to generate x86_64 SIMD instructions for Rustc

This program requires GIT be locally installed.

It will read the GCC platform specific header files for i386 to extract names and GCC builtin prefixes. It will then read LLVMINT library to grab LLVM link-code-names (term maybe incorrect). It will then preform an inner join (more or less) on these records, and translate them into an output that can be used in `librustc_platform_intrinsic` crate.

On initial usage one will want to use the `-fetch` flag to populate these repositories (a git clone of both will be done).

Sample usage to generate aarch64, arm, and x86 instructions

     gen_simd -fetch -x86 -aarch64 -arm -clean

Flags:

     -fetch: clone gcc-mirror and llvmint
     -x86: build the x86.rs file
     -aarch64: build the aarch64.rs file
     -arm: build the arm.rs file
     -clean deleted the gcc-mirror and llvmint files


The repositories in question:

GCC: https://github.com/gcc-mirror/gcc

LLVMINT: https://github.com/huonw/llvmint

