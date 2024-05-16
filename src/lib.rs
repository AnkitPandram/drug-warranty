use soroban_sdk::{contractimpl, BytesN, Env, map, symbol, Vec};
use soroban_sdk::auth::{check_auth, SignaturePayload, verify};
use soroban_sdk::serde::Serialize;
use soroban_sdk::xdr::{AccountId, ScAddress, ScVal};

pub struct DrugAuthContract;

#[derive(Serialize, Clone)]
struct DrugData {
    origin: AccountId,
    batch_info: String,
    hash: Option<BytesN>,
    flagged: bool,
}

#[contractimpl]
impl DrugAuthContract {

    pub fn register_drug(env: Env, identifier: BytesN, batch_info: String, hash: Option<BytesN>) {
        
        check_auth(
            &env,
            SignaturePayload {
                function: symbol!("register_drug"),
                contract: ScAddress::this(&env),
                args: ScVal::Vec(vec![
                    ScVal::LedgerKey(env.invoker()),
                    ScVal::Bytes(identifier.into()),
                    ScVal::String(batch_info),
                    hash.map(ScVal::Bytes).unwrap_or(ScVal::Void),
                ]),
            },
        );

        let drug_data = DrugData {
            origin: env.invoker(),
            batch_info,
            hash,
            flagged: false,
        };

        env.storage().instance().map::<_, DrugData>(symbol!("drugs")).set(
            &identifier,
            &drug_data,
        );
    }

    pub fn verify_drug(env: Env, identifier: BytesN) -> bool {
        if let Some(drug_data) = env.storage().instance().map::<_, DrugData>(symbol!("drugs")).get(&identifier) {
            !drug_data.flagged
        } else {
            false
        }
    }

    pub fn flag_drug(env: Env, identifier: BytesN) {
        
        check_auth(
            &env,
            SignaturePayload {
                function: symbol!("flag_drug"),
                contract: ScAddress::this(&env),
                args: ScVal::Vec(vec![
                    ScVal::LedgerKey(env.invoker()),
                    ScVal::Bytes(identifier.into()),
                ]),
            },
        );

        if let Some(mut drug_data) = env.storage().instance().map::<_, DrugData>(symbol!("drugs")).get(&identifier) {
            drug_data.flagged = true;
            env.storage().instance().map::<_, DrugData>(symbol!("drugs")).set(
                &identifier,
                &drug_data,
            );
        } 
    }
}
