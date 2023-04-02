script;
use std::token::transfer_to_address;
use std::constants::BASE_ASSET_ID;
fn main(amount: u64, asset: ContractId, bob_address: Address) {
    transfer_to_address(amount, asset, bob_address);
}
