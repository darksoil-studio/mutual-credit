use hdk::prelude::*;
use serde::de::DeserializeOwned;

pub fn call_transactions<I, R>(fn_name: String, payload: I) -> ExternResult<R>
where
    I: serde::Serialize + std::fmt::Debug,
    R: serde::Serialize + std::fmt::Debug + DeserializeOwned,
{
    let response = call(
        CallTargetCell::Local,
        "transactions".into(),
        fn_name.into(),
        None,
        payload,
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(WasmError::Guest(format!(
            "Error creating the transaction: {:?}",
            response
        ))),
    }?;

    let transaction_hash: R = result.decode()?;

    Ok(transaction_hash)
}
