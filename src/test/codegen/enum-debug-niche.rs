// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This test depends on a patch that was committed to upstream LLVM
// before 7.0, then backported to the Rust LLVM fork.  It tests that
// optimized enum debug info accurately reflects the enum layout.

// ignore-tidy-linelength
// ignore-windows
// min-system-llvm-version 7.0

// compile-flags: -g -C no-prepopulate-passes

// CHECK-LABEL: @main
// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_variant_part,{{.*}}discriminator:{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "A",{{.*}}extraData:{{.*}}
// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_structure_type,{{.*}}name: "A",{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "B",{{.*}}extraData:{{.*}}
// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_structure_type,{{.*}}name: "B",{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "C",{{.*}}extraData:{{.*}}
// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_structure_type,{{.*}}name: "C",{{.*}}
// CHECK-NOT: {{.*}}DIDerivedType{{.*}}name: "D",{{.*}}extraData:{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "D",{{.*}}
// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_structure_type,{{.*}}name: "D",{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}flags: DIFlagArtificial{{.*}}

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

enum E { A, B, C, D(bool) }

pub fn main() {
    let e = E::D(true);
}
