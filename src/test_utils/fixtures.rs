use rstest::*;
use tracing_subscriber::{filter, FmtSubscriber};
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
use {super::katana::Katana, crate::test_utils::evm_contract::KakarotEvmContract, ethers::abi::Token, rand::Rng};

#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
#[awt]
pub async fn counter(#[future] katana: Katana) -> (Katana, KakarotEvmContract) {
    let eoa = katana.eoa();
    let contract = eoa.deploy_evm_contract("Counter", ()).await.expect("Failed to deploy Counter contract");
    (katana, contract)
}

#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
#[awt]
pub async fn erc20(#[future] katana: Katana) -> (Katana, KakarotEvmContract) {
    let eoa = katana.eoa();
    let contract = eoa
        .deploy_evm_contract(
            "ERC20",
            (
                Token::String("Test".into()),               // name
                Token::String("TT".into()),                 // symbol
                Token::Uint(ethers::types::U256::from(18)), // decimals
            ),
        )
        .await
        .expect("Failed to deploy ERC20 contract");
    (katana, contract)
}

/// This fixture creates a new test environment on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
pub async fn katana() -> Katana {
    let mut bytes = [0u8; 100024];
    rand::thread_rng().fill(bytes.as_mut_slice());

    // Create a new test environment on Katana
    Katana::new(&mut arbitrary::Unstructured::new(&bytes)).await
}

/// This fixture configures the tests. The following setup
/// is used:
/// - The log level is set to `info`
#[fixture]
pub fn setup() {
    let filter = filter::EnvFilter::new("info");
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}
