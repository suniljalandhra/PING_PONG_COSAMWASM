
use cw_storage_plus::Item;
use cosmwasm_std::Uint64;



pub const PING_COUNT: Item<Uint64> = Item::new("ping_count");
