//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::str`.

use std::iter::repeat;
use std::str::{ParseBoolError, Utf8Error, from_utf8};

#[cfg(all(feature = "alloc", not(feature="std")))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use strategy::*;
use strategy::statics::static_map;
use arbitrary::*;

arbitrary!(ParseBoolError; "".parse::<bool>().unwrap_err());

type ELSeq  = W<Just<&'static [u8]>>;
type ELSeqs = TupleUnion<(ELSeq, ELSeq, ELSeq, ELSeq)>;

fn gen_el_seqs() -> ELSeqs {
    prop_oneof![
        Just(&[0xC2]), // None
        Just(&[0x80]), // Some(1)
        Just(&[0xE0, 0xA0, 0x00]), // Some(2)
        Just(&[0xF0, 0x90, 0x80, 0x00]) // Some(3)
    ]
}

arbitrary!(Utf8Error, SFnPtrMap<(StrategyFor<u16>, ELSeqs), Utf8Error>;
    static_map((any::<u16>(), gen_el_seqs()), |(vut, elseq)| {
        let v = repeat(b'_').take(vut as usize)
                    .chain(elseq.iter().cloned())
                    .collect::<Vec<u8>>();
        from_utf8(&v).unwrap_err()
    })
);

#[cfg(test)]
mod test {
    no_panic_test!(
        parse_bool_errror => ParseBoolError,
        utf8_error => Utf8Error
    );
}
