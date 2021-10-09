// Copyright (C) 2021 Subspace Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Data structures related to objects (useful data) stored on Subspace Network.
//!
//! Mappings provided are of 3 kinds:
//! * for objects within a block
//! * for objects within a piece
//! * for global objects in the global history of the blockchain

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

/// Object stored inside of the block
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    TypeInfo,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "std", derive(Debug))]
#[serde(rename_all = "camelCase")]
pub enum BlockObject {
    /// V0 of object mapping data structure
    #[codec(index = 0)]
    V0 {
        /// 24-bit little-endian offset of the object
        offset: [u8; 3],
        /// 24-bit little-endian size of the object
        size: [u8; 3],
    },
}

impl BlockObject {
    /// Offset of the object (limited to 24-bit size internally)
    pub fn offset(&self) -> u32 {
        match self {
            BlockObject::V0 { offset, .. } => {
                u32::from_le_bytes([offset[0], offset[1], offset[2], 0])
            }
        }
    }

    /// Offset of the object (limited to 24-bit size internally)
    pub fn size(&self) -> u32 {
        match self {
            BlockObject::V0 { size, .. } => u32::from_le_bytes([size[0], size[1], size[2], 0]),
        }
    }
}

/// Mapping of objects stored inside of the block
#[derive(
    Default,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    TypeInfo,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct BlockObjectMapping {
    /// Objects stored inside of the block
    pub objects: Vec<BlockObject>,
}

/// Object stored inside of the block
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    TypeInfo,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "std", derive(Debug))]
#[serde(rename_all = "camelCase")]
pub enum PieceObject {
    /// V0 of object mapping data structure
    #[codec(index = 0)]
    V0 {
        /// Offset of the object
        offset: u16,
        /// 24-bit little-endian size of the object
        size: [u8; 3],
    },
}

impl PieceObject {
    /// Offset of the object
    pub fn offset(&self) -> u16 {
        match self {
            PieceObject::V0 { offset, .. } => *offset,
        }
    }

    /// Offset of the object (limited to 24-bit size internally)
    pub fn size(&self) -> u32 {
        match self {
            PieceObject::V0 { size, .. } => u32::from_le_bytes([size[0], size[1], size[2], 0]),
        }
    }
}

/// Mapping of objects stored inside of the piece
#[derive(
    Default,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    TypeInfo,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PieceObjectMapping {
    /// Objects stored inside of the block
    pub objects: Vec<PieceObject>,
}

/// Object stored inside in the history of the blockchain
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Encode,
    Decode,
    TypeInfo,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "std", derive(Debug))]
#[serde(rename_all = "camelCase")]
pub enum GlobalObject {
    /// V0 of object mapping data structure
    #[codec(index = 0)]
    V0 {
        /// Piece index where object is contained (at least its beginning, might not fit fully)
        piece_index: u64,
        /// Offset of the object
        offset: u16,
        /// 24-bit little-endian size of the object
        size: [u8; 3],
    },
}

impl GlobalObject {
    /// Piece index where object is contained (at least its beginning, might not fit fully)
    pub fn piece_index(&self) -> u64 {
        match self {
            GlobalObject::V0 { piece_index, .. } => *piece_index,
        }
    }

    /// Offset of the object
    pub fn offset(&self) -> u16 {
        match self {
            GlobalObject::V0 { offset, .. } => *offset,
        }
    }

    /// Offset of the object (limited to 24-bit size internally)
    pub fn size(&self) -> u32 {
        match self {
            GlobalObject::V0 { size, .. } => u32::from_le_bytes([size[0], size[1], size[2], 0]),
        }
    }
}