#![feature(
    bool_to_option,
    destructuring_assignment,
    generic_associated_types,
    trait_alias,
    min_type_alias_impl_trait
)]
#![allow(
    incomplete_features,
    clippy::mutable_key_type,
    clippy::unused_io_amount
)]

pub mod accessors;
pub mod adapter;
mod changeset;
mod common;
mod dbutils;
mod kv;
mod models;
pub mod stagedsync;
mod state;
pub mod txdb;

pub use changeset::ChangeSet;
pub use dbutils::{tables, DupSort, SyncStage, Table};
pub use kv::{
    mdbx::*,
    remote::{kv_client::KvClient as RemoteKvClient, RemoteCursor, RemoteTransaction},
    traits::{
        txutil, ComparatorFunc, Cursor, CursorDupSort, MutableCursor, MutableCursorDupSort,
        MutableTransaction, Transaction,
    },
};
pub use state::*;
