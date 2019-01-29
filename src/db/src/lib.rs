#[macro_use]
extern crate log;
extern crate lmdb;
extern crate lmdb_sys;

pub mod blockchain_db;
pub mod db_lmdb;

#[cfg(test)]
mod tests;
