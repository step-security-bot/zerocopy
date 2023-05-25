// Copyright 2022 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

include!("../../zerocopy-derive/tests/util.rs");

extern crate zerocopy;

use zerocopy::{transmute, transmute_ref};

fn main() {}

// It is unsupported to inspect the usize value of a pointer during const eval.
const POINTER_VALUE: usize = transmute!(&0usize as *const usize);

const SRC_NOT_AS_BYTES: AU16 = transmute!(NotZerocopy(AU16(0)));

const DST_NOT_FROM_BYTES: NotZerocopy = transmute!(AU16(0));

const INCREASE_SIZE: AU16 = transmute!(0u8);

const DECREASE_SIZE: u8 = transmute!(AU16(0));

const REF_SRC_NOT_AS_BYTES: &AU16 = transmute_ref!(&NotZerocopy(AU16(0)));

const REF_DST_NOT_FROM_BYTES: &NotZerocopy = transmute_ref!(&AU16(0));

const REF_INCREASE_SIZE: &[u8; 2] = transmute_ref!(&0u8);

const REF_DECREASE_SIZE: &u8 = transmute_ref!(&[0u8; 2]);

const REF_INCREASE_ALIGNMENT: &AU16 = transmute_ref!(&[0u8; 2]);

const REF_SRC_UNSIZED: &[u8; 1] = transmute_ref!(&[0u8][..]);

const REF_DST_UNSIZED: &[u8] = transmute_ref!(&[0u8; 1]);

const REF_SRC_DST_UNSIZED: &[u8] = transmute_ref!(&[0u8][..]);

const REF_SRC_NOT_A_REFERENCE: &u8 = transmute_ref!(0usize);

const REF_DST_NOT_A_REFERENCE: usize = transmute_ref!(&0u8);

const REF_SRC_DST_NOT_REFERENCES: usize = transmute_ref!(0usize);

fn ref_dst_mutable() {
    let _: &mut u8 = transmute_ref!(&0u8);
}
