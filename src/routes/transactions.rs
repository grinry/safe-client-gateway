use crate::services::transactions;

#[get("/transactions/<safe_address>")]
pub fn all(safe_address: String) -> String {
    transactions::get_all_transactions(safe_address)
}

#[get("/transaction/<tx_hash>")]
pub fn details(tx_hash: String) -> String {
    transactions::get_transactions_details(tx_hash)
}

#[get("/transactions/about")]
pub fn about() -> String {
    transactions::get_about()
}