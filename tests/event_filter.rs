mod test_utils;

use kakarot_rpc::models::event_filter::EthEventFilter;
use reth_rpc_types::Filter;
use rstest::*;
use serde_json::Value;
use starknet::core::types::{EventFilter, FieldElement};
use test_utils::fixtures::katana;
use test_utils::sequencer::Katana;

lazy_static::lazy_static! {
    pub static ref KAKAROT_ADDRESS: FieldElement = {
        let deployments = include_str!("../lib/kakarot/deployments/katana/deployments.json");

    // Step 2: Parse the JSON
    let object: Value = serde_json::from_str(deployments).unwrap();

    // Step 3: Access the desired data
    object.get("kakarot").unwrap().get("address").unwrap().as_str().unwrap().parse().unwrap()
    };
}

fn assert_eq_event_filter(lhs: EventFilter, rhs: EventFilter) {
    assert_eq!(lhs.from_block, rhs.from_block);
    assert_eq!(lhs.to_block, rhs.to_block);
    assert_eq!(lhs.address, Some(*KAKAROT_ADDRESS));
    assert_eq!(lhs.keys, rhs.keys);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_to_starknet_event_filter_with_block_hash(#[future] katana: Katana) {
    // Given
    let eth_event_filter: Filter =
        serde_json::from_str(include_str!("test_data/conversion/eth/event_filter_block_hash.json")).unwrap();
    let eth_event_filter: EthEventFilter = eth_event_filter.into();

    let client = katana.client();

    // When
    let starknet_event_filter = eth_event_filter.to_starknet_event_filter(client).unwrap();

    // Then
    let expected: EventFilter =
        serde_json::from_str(include_str!("test_data/conversion/starknet/event_filter_block_hash.json")).unwrap();
    assert_eq_event_filter(expected, starknet_event_filter);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_to_starknet_event_filter_with_from_to(#[future] katana: Katana) {
    // Given
    let eth_event_filter: Filter =
        serde_json::from_str(include_str!("test_data/conversion/eth/event_filter_from_to.json")).unwrap();
    let eth_event_filter: EthEventFilter = eth_event_filter.into();

    let client = katana.client();

    // When
    let starknet_event_filter = eth_event_filter.to_starknet_event_filter(client).unwrap();

    // Then
    let expected: EventFilter =
        serde_json::from_str(include_str!("test_data/conversion/starknet/event_filter_from_to.json")).unwrap();
    assert_eq_event_filter(expected, starknet_event_filter);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_to_starknet_event_filter_without_topics(#[future] katana: Katana) {
    // Given
    let eth_event_filter: Filter =
        serde_json::from_str(include_str!("test_data/conversion/eth/event_filter_without_topics.json")).unwrap();
    let eth_event_filter: EthEventFilter = eth_event_filter.into();

    let client = katana.client();

    // When
    let starknet_event_filter = eth_event_filter.to_starknet_event_filter(client).unwrap();

    // Then
    let expected: EventFilter =
        serde_json::from_str(include_str!("test_data/conversion/starknet/event_filter_without_topics.json")).unwrap();
    assert_eq_event_filter(expected, starknet_event_filter);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_to_starknet_event_filter_without_address(#[future] katana: Katana) {
    // Given
    let eth_event_filter: Filter =
        serde_json::from_str(include_str!("test_data/conversion/eth/event_filter_without_address.json")).unwrap();
    let eth_event_filter: EthEventFilter = eth_event_filter.into();

    let client = katana.client();

    // When
    let starknet_event_filter = eth_event_filter.to_starknet_event_filter(client).unwrap();

    // Then
    let expected: EventFilter =
        serde_json::from_str(include_str!("test_data/conversion/starknet/event_filter_without_address.json")).unwrap();
    assert_eq_event_filter(expected, starknet_event_filter);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_to_starknet_event_filter_without_topics_or_address(#[future] katana: Katana) {
    // Given
    let eth_event_filter: Filter =
        serde_json::from_str(include_str!("test_data/conversion/eth/event_filter_without_topics_or_address.json"))
            .unwrap();
    let eth_event_filter: EthEventFilter = eth_event_filter.into();

    let client = katana.client();

    // When
    let starknet_event_filter = eth_event_filter.to_starknet_event_filter(client).unwrap();

    // Then
    let expected: EventFilter =
        serde_json::from_str(include_str!("test_data/conversion/starknet/event_filter_without_topics_or_address.json"))
            .unwrap();
    assert_eq_event_filter(expected, starknet_event_filter);
}
