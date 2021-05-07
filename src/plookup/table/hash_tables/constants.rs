// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

// For the Bls and BN254 curves, the large prime p is different.
// This leads to diffences in the subsequent difference in constants
// we have defined below.
// These are the required constants for the
// Currently making the s_i usize.

use crate::plookup::table::hash_tables::{HashTableTwo, HashTableThree};
use crate::prelude::BlsScalar;

/// This is the smallest prime that exceeds the BLS construction
pub const V: usize = 661;

/// This is the order of the bits in the s box breakdown
pub const N: u64 = 27;

/// This is all s values used in the bar function
//[s_n, s_{n-1} ..., s_1]
pub const S: [u64; 27] = [
    693, 696, 694, 668, 679, 695, 691, 693, 700, 688, 700, 694, 701, 694, 699, 701, 701, 701, 695,
    698, 697, 703, 702, 691, 688, 703, 679,
];

/// Arity of hash table
pub const T_S: u32 = 4;

/// Hash table containing fixed binary
/// possibilities for Hash
pub const HASH_TABLE_TWO: HashTableTwo = HashTableTwo([
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::zero(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
]);

/// Hash table containing fixed binary
/// possibilities for Hash
pub const HASH_TABLE_THREE: HashTableThree = HashTableThree([
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
    ],
    [
        BlsScalar::zero(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
    ],
    [
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar::one(),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar::one(),
    ],
    [
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
        BlsScalar([
            17179869180,
            12756850513266774020,
            3681868479150465002,
            3479420709561305823,
        ]),
    ],
]);
