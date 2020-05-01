// External imports
use serde_derive::{Deserialize, Serialize};
// Workspace imports
// Local imports
use crate::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "storage_state_update"]
pub struct NewStorageState {
    pub storage_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "storage_state_update"]
pub struct StoredStorageState {
    pub id: i32,
    pub storage_state: String,
}

#[derive(Debug, Insertable)]
#[table_name = "events_state"]
pub struct NewBlockEvent {
    pub block_type: String, // 'Committed', 'Verified'
    pub transaction_hash: Vec<u8>,
    pub block_num: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "events_state"]
pub struct StoredBlockEvent {
    pub id: i32,
    pub block_type: String, // 'Committed', 'Verified'
    pub transaction_hash: Vec<u8>,
    pub block_num: i64,
}