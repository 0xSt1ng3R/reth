use reth_primitives::{Address, BlockNumber, StorageEntry, B256};
use reth_storage_errors::provider::ProviderResult;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};
use std::collections::{HashSet, HashMap};

use auto_impl::auto_impl;
use reth_interfaces::provider::ProviderResult;
use reth_primitives::{Address, BlockNumber, StorageEntry, B256};

/// Storage reader
#[auto_impl::auto_impl(&, Arc, Box)]
pub trait StorageReader: Send + Sync {
    /// Get plainstate storages for addresses and storage keys.
    fn plain_state_storages(
        &self,
        addresses_with_keys: impl IntoIterator<Item = (Address, impl IntoIterator<Item = B256>)>,
    ) -> ProviderResult<Vec<(Address, Vec<StorageEntry>)>>;

    /// Iterate over storage changesets and return all storage slots that were changed.
    fn changed_storages_with_range(
        &self,
        range: RangeInclusive<BlockNumber>,
    ) -> ProviderResult<BTreeMap<Address, BTreeSet<B256>>>;

    fn changed_storages_with_range_with_content_1(
        &self,
        range: RangeInclusive<BlockNumber>,
    ) -> ProviderResult<HashMap<Address, HashMap<B256, StorageEntry>>>;

    fn changed_storages_with_range_with_content_2(
        &self,
        range: RangeInclusive<BlockNumber>,
    ) -> ProviderResult<HashMap<Address, HashMap<B256, StorageEntry>>>;

    /// Iterate over storage changesets by block number range and address and return all storage slots that were changed.
    fn changed_storages_with_range_by_address(
        &self,
        range: RangeInclusive<BlockNumber>,
        address: Address,
    ) -> ProviderResult<BTreeMap<Address, BTreeSet<B256>>>;

    /// Iterate over storage changesets by block number range and addresses and return all storage slots that were changed.
    fn changed_storages_with_range_by_addresses(
        &self,
        range: RangeInclusive<BlockNumber>,
        addresses: Vec<Address>,
    ) -> ProviderResult<BTreeMap<Address, BTreeSet<B256>>>;

    fn changed_storages_with_range_by_addresses_with_content_1(
        &self,
        range: RangeInclusive<BlockNumber>,
        addresses: HashSet<Address>,
    ) -> ProviderResult<HashMap<Address, HashMap<B256, StorageEntry>>>;

    fn changed_storages_with_range_by_addresses_with_content_2(
        &self,
        range: RangeInclusive<BlockNumber>,
        addresses: HashSet<Address>,
    ) -> ProviderResult<HashMap<Address, ((u64, u64), HashMap<B256, StorageEntry>)>>;

    /// Iterate over storage changesets and return all storage slots that were changed alongside
    /// each specific set of blocks.
    ///
    /// NOTE: Get inclusive range of blocks.
    fn changed_storages_and_blocks_with_range(
        &self,
        range: RangeInclusive<BlockNumber>,
    ) -> ProviderResult<BTreeMap<(Address, B256), Vec<u64>>>;
}
