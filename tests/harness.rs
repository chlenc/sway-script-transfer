use std::str::FromStr;

use fuels::{prelude::*, tx::ContractId};

abigen!(Script(
    name = "Script",
    abi = "out/debug/script_transfer-abi.json"
));
#[tokio::test]
async fn main() {
    let mint_amout = 1_000_000_000;

    let wallets_config = WalletsConfig::new(Some(2), Some(1), Some(mint_amout));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None).await;

    let alice = wallets[0].clone();
    let alice_balance = alice.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    let bob = wallets[1].clone();
    let bob_balance = alice.get_asset_balance(&BASE_ASSET_ID).await.unwrap();

    assert_eq!(alice_balance, mint_amout);
    assert_eq!(bob_balance, mint_amout);

    let amount0 = mint_amout / 2;
    let contract_id = ContractId::from_str(BASE_ASSET_ID.to_string().as_str()).unwrap();
    let instance = Script::new(alice.clone(), "out/debug/script_transfer.bin");
    let receipts = instance
        .main(amount0, contract_id, Address::from(bob.address()))
        .tx_params(TxParameters::default().set_gas_price(1))
        .call()
        .await;

    dbg!(&receipts);

    receipts.unwrap();

    assert_eq!(
        alice.get_asset_balance(&BASE_ASSET_ID).await.unwrap(),
        bob_balance + amount0
    );
}
