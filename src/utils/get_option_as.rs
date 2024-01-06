use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use cln_rpc::model::requests::{
    CreateonionHops, DatastoreMode, DelinvoiceStatus, FeeratesStyle, ListforwardsStatus,
    ListinvoicesIndex, ListpaysStatus, ListsendpaysStatus, NewaddrAddresstype, SendonionFirst_hop,
    SendpayRoute,
};
use cln_rpc::primitives::{
    Amount, AmountOrAll, AmountOrAny, Feerate, Outpoint, OutputDesc, PublicKey, Routehint,
    RoutehintList, Routehop, Secret, Sha256, ShortChannelId, TlvEntry, TlvStream,
};
use serde_json::Value;

// Define a trait for types that can be created from an Option<Value>
pub trait FromOptionValue: Sized {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String>;
}

// Macro for implementing FromOptionValue for different types
macro_rules! impl_from_option_value {
    ($type:ty, $body:expr) => {
        impl FromOptionValue for $type {
            fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
                $body(value.clone())
            }
        }
    };
}

// Common error message function
fn err_msg<T: ToString + std::fmt::Display>(detail: T) -> String {
    format!("Failed to parse {}", detail)
}

// Helper function for parsing strings
fn parse_string(value: Option<Value>) -> Result<String, String> {
    match value {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(err_msg("String")),
    }
}

// Helper function for parsing Vec<T> where T implements FromOptionValue
fn parse_vec<T: FromOptionValue>(value: &Option<Value>) -> Result<Vec<T>, String> {
    match value {
        Some(Value::Array(arr)) => arr
            .iter()
            .map(|v| T::from_option_value(&Some(v.clone())))
            .collect(),
        _ => Err(err_msg(format!("Vec<{}>", std::any::type_name::<T>()))),
    }
}

// Implement FromOptionValue for various types using the macro
impl_from_option_value!(bool, |value| {
    bool::from_option_value(&value).map_err(|_| err_msg("bool"))
});
impl_from_option_value!(u8, |value| {
    u8::from_option_value(&value).map_err(|_| err_msg("u8"))
});
impl_from_option_value!(u16, |value| {
    u16::from_option_value(&value).map_err(|_| err_msg("u16"))
});
impl_from_option_value!(u32, |value| {
    u32::from_option_value(&value).map_err(|_| err_msg("u32"))
});
impl_from_option_value!(u64, |value| {
    u64::from_option_value(&value).map_err(|_| err_msg("u64"))
});
impl_from_option_value!(usize, |value| {
    usize::from_option_value(&value).map_err(|_| err_msg("f32"))
});
impl_from_option_value!(f32, |value| {
    f32::from_option_value(&value).map_err(|_| err_msg("f32"))
});
impl_from_option_value!(f64, |value| {
    f64::from_option_value(&value).map_err(|_| err_msg("f64"))
});
impl_from_option_value!(String, parse_string);
impl_from_option_value!(PublicKey, |value| {
    parse_string(value).and_then(|s| PublicKey::from_str(&s).map_err(|_| err_msg("PublicKey")))
});
impl_from_option_value!(Secret, |value| {
    parse_string(value).and_then(|s| {
        let bytes = hex::decode(s).map_err(|_| err_msg("Secret"))?;
        Secret::try_from(bytes).map_err(|_| err_msg("Secret"))
    })
});
impl_from_option_value!(Sha256, |value| {
    parse_string(value).and_then(|s| Sha256::from_str(&s).map_err(|_| err_msg("Sha256")))
});
impl_from_option_value!(ShortChannelId, |value| {
    parse_string(value)
        .and_then(|s| ShortChannelId::from_str(&s).map_err(|_| err_msg("ShortChannelId")))
});
impl_from_option_value!(Amount, |value| {
    parse_string(value).and_then(|s| {
        let parse_with_suffix = |suffix: &str, constructor_fn: fn(u64) -> Amount| {
            s.trim_end_matches(suffix)
                .parse::<u64>()
                .map_err(|_| err_msg("Amount"))
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
                .map_err(|_| err_msg("Amount"))
                .map(Amount::from_msat)
        };
        amount
    })
});
impl_from_option_value!(OutputDesc, |value| {
    parse_string(value).and_then(|s| {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(err_msg("OutputDesc"));
        }

        let address = String::from_str(parts[0]).map_err(|_| err_msg("OutputDesc"))?;
        let amount = Amount::from_option_value(&Some(Value::String(String::from(parts[1]))))
            .map_err(|_| err_msg("OutputDesc"))?;

        Ok(OutputDesc { address, amount })
    })
});

impl_from_option_value!(Outpoint, |value| {
    parse_string(value).and_then(|s| {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(err_msg("Outpoint"));
        }

        let txid = Sha256::from_str(parts[0]).map_err(|_| err_msg("Outpoint"))?;
        let vout = parts[1].parse::<u32>().map_err(|_| err_msg("Outpoint"))?;

        Ok(Outpoint { txid, outnum: vout })
    })
});

impl_from_option_value!(SendpayRoute, |value| {
    match value {
        Some(Value::Object(map)) => {
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

            Ok(SendpayRoute {
                amount_msat,
                id,
                delay,
                channel,
            })
        }
        _ => Err("Invalid value for SendpayRoute".to_string()),
    }
});

impl_from_option_value!(AmountOrAll, |value| {
    match value {
        Some(Value::String(s)) if s == "all" => Ok(AmountOrAll::All),
        Some(_) => parse_amount(&value, AmountOrAll::Amount),
        None => Err("No value provided".to_string()),
    }
});

impl_from_option_value!(AmountOrAny, |value| {
    match value {
        Some(Value::String(s)) if s == "any" => Ok(AmountOrAny::Any),
        Some(_) => parse_amount(&value, AmountOrAny::Amount),
        None => Err("No value provided".to_string()),
    }
});

impl_from_option_value!(NewaddrAddresstype, |value| {
    match value {
        Some(Value::String(s)) if s == "bech32" => Ok(NewaddrAddresstype::BECH32),
        Some(Value::String(s)) if s == "p2tr" => Ok(NewaddrAddresstype::P2TR),
        Some(Value::String(s)) if s == "all" => Ok(NewaddrAddresstype::ALL),
        Some(_) => Err("Invalid value for address type".to_string()),
        None => Err("No value provided".to_string()),
    }
});

impl_from_option_value!(Feerate, |value| {
    match value {
        Some(Value::String(s)) => {
            Feerate::try_from(s.as_str()).map_err(|_| "Failed to parse as FeeRate".to_string())
        }
        _ => Err("No value provided".to_string()),
    }
});

impl_from_option_value!(DatastoreMode, |value| {
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
});

impl_from_option_value!(Vec<CreateonionHops>, |value| {
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
});
impl_from_option_value!(DelinvoiceStatus, |value| {
    match value {
        Some(Value::String(s)) => match s.as_str() {
            "paid" => Ok(DelinvoiceStatus::PAID),
            "expired" => Ok(DelinvoiceStatus::EXPIRED),
            "unpaid" => Ok(DelinvoiceStatus::UNPAID),
            _ => Err(format!("Invalid value for DelinvoiceStatus: {}", s)),
        },
        _ => Err("Invalid value for DelinvoiceStatus".to_string()),
    }
});

impl_from_option_value!(ListinvoicesIndex, |value| {
    match value {
        Some(Value::String(s)) => match s.as_str() {
            "created" => Ok(ListinvoicesIndex::CREATED),
            "updated" => Ok(ListinvoicesIndex::UPDATED),
            _ => Err(format!("Invalid value for ListinvoicesIndex: {}", s)),
        },
        _ => Err("Invalid value for ListinvoicesIndex".to_string()),
    }
});

impl_from_option_value!(ListsendpaysStatus, |value| {
    match value {
        Some(Value::String(s)) => match s.as_str() {
            "pending" => Ok(ListsendpaysStatus::PENDING),
            "complete" => Ok(ListsendpaysStatus::COMPLETE),
            "failed" => Ok(ListsendpaysStatus::FAILED),
            _ => Err(format!("Invalid value for ListsendpaysStatus: {}", s)),
        },
        _ => Err("Invalid value for ListsendpaysStatus".to_string()),
    }
});

impl_from_option_value!(SendonionFirst_hop, |value| {
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
});
impl_from_option_value!(RoutehintList, |value| {
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
});

impl_from_option_value!(TlvEntry, |value| {
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
});

impl_from_option_value!(TlvStream, |value| {
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
});

impl_from_option_value!(Routehop, |value| {
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
});

impl_from_option_value!(Routehint, |value| {
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
});

impl_from_option_value!(FeeratesStyle, |value| {
    match value {
        Some(Value::String(s)) => match s.as_str() {
            "perkb" => Ok(FeeratesStyle::PERKB),
            "perkw" => Ok(FeeratesStyle::PERKW),
            _ => Err(format!("Invalid value for FeeratesStyle: {}", s)),
        },
        _ => Err("Invalid value for FeeratesStyle".to_string()),
    }
});

impl_from_option_value!(ListpaysStatus, |value| {
    match value {
        Some(Value::String(s)) => match s.as_str() {
            "pending" => Ok(ListpaysStatus::PENDING),
            "complete" => Ok(ListpaysStatus::COMPLETE),
            "failed" => Ok(ListpaysStatus::FAILED),
            _ => Err(format!("Invalid value for ListpaysStatus: {}", s)),
        },
        _ => Err("Invalid value for ListpaysStatus".to_string()),
    }
});
impl_from_option_value!(ListforwardsStatus, |value| {
    match value {
        Some(Value::String(s)) => match s.as_str() {
            "offered" => Ok(ListforwardsStatus::OFFERED),
            "settled" => Ok(ListforwardsStatus::SETTLED),
            "local_failed" => Ok(ListforwardsStatus::LOCAL_FAILED),
            "failed" => Ok(ListforwardsStatus::FAILED),
            _ => Err(format!("Invalid value for ListforwardsStatus: {}", s)),
        },
        _ => Err("Invalid value for ListforwardsStatus".to_string()),
    }
});

impl_from_option_value!(Vec<bool>, |value| parse_vec(&value));
impl_from_option_value!(Vec<u8>, |value| parse_vec(&value));
impl_from_option_value!(Vec<u16>, |value| parse_vec(&value));
impl_from_option_value!(Vec<u32>, |value| parse_vec(&value));
impl_from_option_value!(Vec<u64>, |value| parse_vec(&value));
impl_from_option_value!(Vec<f32>, |value| parse_vec(&value));
impl_from_option_value!(Vec<f64>, |value| parse_vec(&value));
impl_from_option_value!(Vec<String>, |value| parse_vec(&value));
impl_from_option_value!(Vec<PublicKey>, |value| parse_vec(&value));
impl_from_option_value!(Vec<Secret>, |value| parse_vec(&value));
impl_from_option_value!(Vec<Sha256>, |value| parse_vec(&value));
impl_from_option_value!(Vec<ShortChannelId>, |value| parse_vec(&value));
impl_from_option_value!(Vec<Amount>, |value| parse_vec(&value));
impl_from_option_value!(Vec<OutputDesc>, |value| parse_vec(&value));
impl_from_option_value!(Vec<Outpoint>, |value| parse_vec(&value));
impl_from_option_value!(Vec<SendpayRoute>, |value| parse_vec(&value));
impl_from_option_value!(Vec<AmountOrAll>, |value| parse_vec(&value));
impl_from_option_value!(Vec<AmountOrAny>, |value| parse_vec(&value));
impl_from_option_value!(Vec<NewaddrAddresstype>, |value| parse_vec(&value));
impl_from_option_value!(Vec<Feerate>, |value| parse_vec(&value));

// Generalized get_option_as function remains the same
pub fn get_option_as<T: FromOptionValue>(
    options_map: &HashMap<String, Option<Value>>,
    key: &str,
) -> Option<T> {
    options_map
        .get(key)
        .and_then(|v| T::from_option_value(v).ok())
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
