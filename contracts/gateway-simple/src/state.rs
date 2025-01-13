use cosmwasm_std::Addr;
use secret_toolkit::storage::{Keymap, KeymapBuilder, WithoutIter};
use secret_toolkit::{serialization::Bincode2, storage::Item};

pub const ADMIN: Item<Addr> = Item::new(b"admin");

// a mapping of a account user addresses to their secrets
pub const SECRETS: Keymap<String, String, Bincode2, WithoutIter> =
    KeymapBuilder::new(b"secrets").without_iter().build();

pub const EXECUTE_INSTANTIATE_REPLY_ID: u64 = 1;

pub const BLOCK_SIZE: usize = 256;
