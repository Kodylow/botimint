use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use cln_rpc::model::requests::{
    CreateonionHops, DatastoreMode, DelinvoiceStatus, ListinvoicesIndex, ListsendpaysStatus,
    NewaddrAddresstype, SendonionFirst_hop, SendpayRoute,
};
use cln_rpc::primitives::{
    Amount, AmountOrAll, AmountOrAny, Feerate, Outpoint, PublicKey, Routehint, RoutehintList,
    Routehop, Secret, Sha256, ShortChannelId, TlvEntry, TlvStream,
};
use serde_json::Value;

// Define a trait for types that can be created from an Option<Value>
pub trait FromOptionValue: Sized {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String>;
}

impl FromOptionValue for Secret {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => {
                let bytes =
                    hex::decode(s).map_err(|_| "Failed to decode hex string".to_string())?;
                Secret::try_from(bytes).map_err(|_| "Failed to parse Secret".to_string())
            }
            _ => Err("Invalid value for Secret".to_string()),
        }
    }
}

impl FromOptionValue for Vec<Secret> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut secrets = Vec::new();
                for val in arr {
                    if let Value::String(s) = val {
                        let bytes = hex::decode(s)
                            .map_err(|_| "Failed to decode hex string".to_string())?;
                        secrets.push(
                            Secret::try_from(bytes)
                                .map_err(|_| "Failed to parse Secret".to_string())?,
                        );
                    } else {
                        return Err("Invalid value for Vec<Secret>".to_string());
                    }
                }
                Ok(secrets)
            }
            _ => Err("Invalid value for Vec<Secret>".to_string()),
        }
    }
}

impl FromOptionValue for Sha256 {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => {
                Sha256::from_str(s).map_err(|_| "Failed to parse Sha256".to_string())
            }
            _ => Err("Invalid value for Sha256".to_string()),
        }
    }
}

impl FromOptionValue for u16 {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Number(num)) => num
                .as_u64()
                .and_then(|v| v.try_into().ok())
                .ok_or_else(|| "Failed to parse u16".to_string()),
            _ => Err("Invalid value for u16".to_string()),
        }
    }
}

impl FromOptionValue for ShortChannelId {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => ShortChannelId::from_str(s)
                .map_err(|_| "Failed to parse ShortChannelId".to_string()),
            _ => Err("Invalid value for ShortChannelId".to_string()),
        }
    }
}

impl FromOptionValue for Vec<SendpayRoute> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut routes = Vec::new();
                for val in arr {
                    if let Value::Object(map) = val {
                        let amount_msat = map
                            .get("amount_msat")
                            .ok_or_else(|| "amount_msat is missing".to_string())
                            .and_then(|v| {
                                Amount::from_option_value(&Some(v.clone()))
                                    .map_err(|_| "Failed to parse amount_msat".to_string())
                            })?;

                        let id = map
                            .get("id")
                            .ok_or_else(|| "id is missing".to_string())
                            .and_then(|v| {
                                PublicKey::from_option_value(&Some(v.clone()))
                                    .map_err(|_| "Failed to parse id".to_string())
                            })?;

                        let delay = map
                            .get("delay")
                            .ok_or_else(|| "delay is missing".to_string())
                            .and_then(|v| {
                                u16::from_option_value(&Some(v.clone()))
                                    .map_err(|_| "Failed to parse delay".to_string())
                            })?;

                        let channel = map
                            .get("channel")
                            .ok_or_else(|| "channel is missing".to_string())
                            .and_then(|v| {
                                ShortChannelId::from_option_value(&Some(v.clone()))
                                    .map_err(|_| "Failed to parse channel".to_string())
                            })?;

                        routes.push(SendpayRoute {
                            amount_msat,
                            id,
                            delay,
                            channel,
                        });
                    } else {
                        return Err("Invalid value for SendpayRoute".to_string());
                    }
                }
                Ok(routes)
            }
            _ => Err("Invalid value for Vec<SendpayRoute>".to_string()),
        }
    }
}

// Implement FromOptionValue for various types
impl FromOptionValue for String {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| "Failed to parse as String".to_string())
    }
}

impl FromOptionValue for Vec<String> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut strings = Vec::new();
                for val in arr {
                    if let Value::String(s) = val {
                        strings.push(s.clone());
                    } else {
                        return Err("Invalid value for Vec<String>".to_string());
                    }
                }
                Ok(strings)
            }
            _ => Err("Invalid value for Vec<String>".to_string()),
        }
    }
}

impl FromOptionValue for bool {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_bool())
            .ok_or_else(|| "Failed to parse as Bool".to_string())
    }
}

impl FromOptionValue for u32 {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .ok_or_else(|| "Failed to parse as u32".to_string())
    }
}

impl FromOptionValue for Vec<u32> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut ints = Vec::new();
                for val in arr {
                    if let Value::Number(num) = val {
                        ints.push(
                            num.as_u64()
                                .and_then(|v| v.try_into().ok())
                                .ok_or_else(|| "Failed to parse u32".to_string())?,
                        );
                    } else {
                        return Err("Invalid value for Vec<u32>".to_string());
                    }
                }
                Ok(ints)
            }
            _ => Err("Invalid value for Vec<u32>".to_string()),
        }
    }
}

impl FromOptionValue for u64 {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_u64())
            .ok_or_else(|| "Failed to parse as u64".to_string())
    }
}

impl FromOptionValue for f64 {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_f64())
            .ok_or_else(|| "Failed to parse as f64".to_string())
    }
}

impl FromOptionValue for PublicKey {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_str())
            .map(|v| PublicKey::from_str(v).unwrap())
            .ok_or_else(|| "Failed to parse as PublicKey".to_string())
    }
}

impl FromOptionValue for Amount {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        parse_amount(value, |amount| amount)
    }
}

impl FromOptionValue for AmountOrAll {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value.as_ref().and_then(|v| v.as_str()) {
            Some("all") => Ok(AmountOrAll::All),
            Some(_) => parse_amount(value, AmountOrAll::Amount),
            None => Err("No value provided".to_string()),
        }
    }
}

impl FromOptionValue for AmountOrAny {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value.as_ref().and_then(|v| v.as_str()) {
            Some("any") => Ok(AmountOrAny::Any),
            Some(_) => parse_amount(value, AmountOrAny::Amount),
            None => Err("No value provided".to_string()),
        }
    }
}

impl FromOptionValue for NewaddrAddresstype {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value.as_ref().and_then(|v| v.as_str()) {
            Some("bech32") => Ok(NewaddrAddresstype::BECH32),
            Some("p2tr") => Ok(NewaddrAddresstype::P2TR),
            Some("all") => Ok(NewaddrAddresstype::ALL),
            Some(_) => Err("Invalid value for address type".to_string()),
            None => Err("No value provided".to_string()),
        }
    }
}

fn parse_amount<F, T>(value: &Option<Value>, constructor: F) -> Result<T, String>
where
    F: Fn(Amount) -> T,
{
    match value.as_ref().and_then(|v| v.as_str()) {
        Some(s) => {
            let parse_with_suffix = |suffix: &str, constructor_fn: fn(u64) -> Amount| {
                s.trim_end_matches(suffix)
                    .parse::<u64>()
                    .map_err(|_| "Failed to parse as u64".to_string())
                    .map(constructor_fn)
            };

            let amount = if s.ends_with("btc") {
                parse_with_suffix("btc", Amount::from_btc)
            } else if s.ends_with("sat") {
                parse_with_suffix("sat", Amount::from_sat)
            } else if s.ends_with("msat") {
                parse_with_suffix("msat", Amount::from_msat)
            } else {
                // Default to sat if no specific suffix is found
                s.parse::<u64>()
                    .map_err(|_| "Failed to parse as u64".to_string())
                    .map(Amount::from_msat)
            };
            amount.map(constructor)
        }
        None => Err("No value provided".to_string()),
    }
}

impl FromOptionValue for Outpoint {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() != 2 {
                    return Err("Invalid outpoint format".to_string());
                }

                let txid = Sha256::from_str(parts[0])
                    .map_err(|_| format!("Failed to parse txid {} as Sha256", parts[0]))?;
                let vout = parts[1]
                    .parse::<u32>()
                    .map_err(|_| "Failed to parse vout as u32".to_string())?;

                Ok(Outpoint { txid, outnum: vout })
            }
            _ => Err("Invalid value for Outpoint".to_string()),
        }
    }
}

impl FromOptionValue for Vec<Outpoint> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut outpoints = Vec::new();

                for val in arr {
                    if let Value::String(s) = val {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() != 2 {
                            return Err("Invalid outpoint format".to_string());
                        }

                        let txid = Sha256::from_str(parts[0])
                            .map_err(|_| format!("Failed to parse txid {} as Sha256", parts[0]))?;
                        let vout = parts[1]
                            .parse::<u32>()
                            .map_err(|_| "Failed to parse vout as u32".to_string())?;

                        outpoints.push(Outpoint { txid, outnum: vout });
                    } else {
                        return Err("Outpoint value must be a string".to_string());
                    }
                }

                Ok(outpoints)
            }
            _ => Err("Invalid value for outpoints".to_string()),
        }
    }
}

impl FromOptionValue for Feerate {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_str())
            .map(|v| Feerate::try_from(v).unwrap())
            .ok_or_else(|| "Failed to parse as FeeRate".to_string())
    }
}

impl FromOptionValue for Vec<Feerate> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut feerates = Vec::new();
                for val in arr {
                    if let Value::String(s) = val {
                        let feerate = Feerate::try_from(s.as_str())
                            .map_err(|_| "Failed to parse Feerate".to_string())?;
                        feerates.push(feerate);
                    } else {
                        return Err("Invalid value for Vec<Feerate>".to_string());
                    }
                }
                Ok(feerates)
            }
            _ => Err("Invalid value for Vec<Feerate>".to_string()),
        }
    }
}

impl FromOptionValue for DatastoreMode {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => match s.as_str() {
                "must-create" => Ok(DatastoreMode::MUST_CREATE),
                "must-replace" => Ok(DatastoreMode::MUST_REPLACE),
                "create-or-replace" => Ok(DatastoreMode::CREATE_OR_REPLACE),
                "must-append" => Ok(DatastoreMode::MUST_APPEND),
                "create-or-append" => Ok(DatastoreMode::CREATE_OR_APPEND),
                _ => Err(format!("Invalid value for DatastoreMode: {}", s)),
            },
            _ => Err("Invalid value for DatastoreMode".to_string()),
        }
    }
}

impl FromOptionValue for Vec<CreateonionHops> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut hops = Vec::new();
                for val in arr {
                    if let Value::Object(map) = val {
                        let pubkey = map
                            .get("pubkey")
                            .and_then(|v| v.as_str())
                            .and_then(|s| PublicKey::from_str(s).ok())
                            .ok_or_else(|| "Failed to parse pubkey".to_string())?;
                        let payload = map
                            .get("payload")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .ok_or_else(|| "Failed to parse payload".to_string())?;
                        hops.push(CreateonionHops { pubkey, payload });
                    } else {
                        return Err("Invalid value for Vec<CreateonionHops>".to_string());
                    }
                }
                Ok(hops)
            }
            _ => Err("Invalid value for Vec<CreateonionHops>".to_string()),
        }
    }
}

impl FromOptionValue for DelinvoiceStatus {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => match s.as_str() {
                "paid" => Ok(DelinvoiceStatus::PAID),
                "expired" => Ok(DelinvoiceStatus::EXPIRED),
                "unpaid" => Ok(DelinvoiceStatus::UNPAID),
                _ => Err(format!("Invalid value for DelinvoiceStatus: {}", s)),
            },
            _ => Err("Invalid value for DelinvoiceStatus".to_string()),
        }
    }
}

impl FromOptionValue for ListinvoicesIndex {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => match s.as_str() {
                "created" => Ok(ListinvoicesIndex::CREATED),
                "updated" => Ok(ListinvoicesIndex::UPDATED),
                _ => Err(format!("Invalid value for ListinvoicesIndex: {}", s)),
            },
            _ => Err("Invalid value for ListinvoicesIndex".to_string()),
        }
    }
}

impl FromOptionValue for ListsendpaysStatus {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => match s.as_str() {
                "pending" => Ok(ListsendpaysStatus::PENDING),
                "complete" => Ok(ListsendpaysStatus::COMPLETE),
                "failed" => Ok(ListsendpaysStatus::FAILED),
                _ => Err(format!("Invalid value for ListsendpaysStatus: {}", s)),
            },
            _ => Err("Invalid value for ListsendpaysStatus".to_string()),
        }
    }
}

impl FromOptionValue for SendonionFirst_hop {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Object(map)) => {
                let id = map
                    .get("id")
                    .ok_or_else(|| "id is missing".to_string())
                    .and_then(|v| {
                        PublicKey::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse id".to_string())
                    })?;

                let amount_msat = map
                    .get("amount_msat")
                    .ok_or_else(|| "amount_msat is missing".to_string())
                    .and_then(|v| {
                        Amount::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse amount_msat".to_string())
                    })?;

                let delay = map
                    .get("delay")
                    .ok_or_else(|| "delay is missing".to_string())
                    .and_then(|v| {
                        u16::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse delay".to_string())
                    })?;

                Ok(SendonionFirst_hop {
                    id,
                    amount_msat,
                    delay,
                })
            }
            _ => Err("Invalid value for SendonionFirst_hop".to_string()),
        }
    }
}

impl FromOptionValue for RoutehintList {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut hints = Vec::new();
                for val in arr {
                    let hint = Routehint::from_option_value(&Some(val.clone()))
                        .map_err(|_| "Failed to parse Routehint".to_string())?;
                    hints.push(hint);
                }
                Ok(RoutehintList { hints })
            }
            _ => Err("Invalid value for RoutehintList".to_string()),
        }
    }
}

impl FromOptionValue for TlvEntry {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Object(map)) => {
                let typ = map
                    .get("typ")
                    .ok_or_else(|| "typ is missing".to_string())
                    .and_then(|v| {
                        u64::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse typ".to_string())
                    })?;

                let value = map
                    .get("value")
                    .ok_or_else(|| "value is missing".to_string())
                    .and_then(|v| {
                        Vec::<u8>::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse value".to_string())
                    })?;

                Ok(TlvEntry { typ, value })
            }
            _ => Err("Invalid value for TlvEntry".to_string()),
        }
    }
}

impl FromOptionValue for TlvStream {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut entries = Vec::new();
                for val in arr {
                    let entry = TlvEntry::from_option_value(&Some(val.clone()))
                        .map_err(|_| "Failed to parse TlvEntry".to_string())?;
                    entries.push(entry);
                }
                Ok(TlvStream { entries })
            }
            _ => Err("Invalid value for TlvStream".to_string()),
        }
    }
}

impl FromOptionValue for Vec<u8> {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::String(s)) => {
                let bytes =
                    hex::decode(s).map_err(|_| "Failed to decode hex string".to_string())?;
                Ok(bytes)
            }
            _ => Err("Invalid value for Vec<u8>".to_string()),
        }
    }
}

impl FromOptionValue for Routehop {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Object(map)) => {
                let id = map
                    .get("pubkey")
                    .ok_or_else(|| "pubkey is missing".to_string())
                    .and_then(|v| {
                        PublicKey::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse pubkey".to_string())
                    })?;

                let scid = map
                    .get("short_channel_id")
                    .ok_or_else(|| "short_channel_id is missing".to_string())
                    .and_then(|v| {
                        ShortChannelId::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse short_channel_id".to_string())
                    })?;

                let feebase = map
                    .get("fee_base_msat")
                    .ok_or_else(|| "fee_base_msat is missing".to_string())
                    .and_then(|v| {
                        Amount::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse fee_base_msat".to_string())
                    })?;

                let feeprop = map
                    .get("fee_proportional_millionths")
                    .ok_or_else(|| "fee_proportional_millionths is missing".to_string())
                    .and_then(|v| {
                        u32::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse fee_proportional_millionths".to_string())
                    })?;

                let expirydelta = map
                    .get("cltv_expiry_delta")
                    .ok_or_else(|| "cltv_expiry_delta is missing".to_string())
                    .and_then(|v| {
                        u16::from_option_value(&Some(v.clone()))
                            .map_err(|_| "Failed to parse cltv_expiry_delta".to_string())
                    })?;

                Ok(Routehop {
                    id,
                    scid,
                    feebase,
                    feeprop,
                    expirydelta,
                })
            }
            _ => Err("Invalid value for Routehop".to_string()),
        }
    }
}

impl FromOptionValue for Routehint {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        match value {
            Some(Value::Array(arr)) => {
                let mut hops = Vec::new();
                for val in arr {
                    let hop = Routehop::from_option_value(&Some(val.clone()))
                        .map_err(|_| "Failed to parse Routehop".to_string())?;
                    hops.push(hop);
                }
                Ok(Routehint { hops })
            }
            _ => Err("Invalid value for Routehint".to_string()),
        }
    }
}

#[allow(private_bounds)]
// Generalized get_option_as function
pub fn get_option_as<T: FromOptionValue>(
    options_map: &HashMap<String, Option<Value>>,
    key: &str,
) -> Option<T> {
    options_map
        .get(key)
        .and_then(|v| T::from_option_value(v).ok())
}
