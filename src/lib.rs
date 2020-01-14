extern crate ewasm_api;


#[no_mangle]
pub extern "C" fn main() {

    let contract_address = ewasm_api::current_address();
    let mut storage_key = ewasm_api::types::StorageKey::default();
    storage_key.bytes.copy_from_slice(&contract_address.bytes);
    let mut storage_value = ewasm_api::types::StorageValue::default();
    storage_value.bytes.copy_from_slice(&contract_address.bytes);
    ewasm_api::storage_store(&storage_key, &storage_value);
    return;
}
