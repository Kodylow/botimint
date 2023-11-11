use std::collections::HashMap;
use std::str::FromStr;
use cln_rpc::model::requests::NewaddrAddresstype;

use anyhow::Result;
use cln_rpc::primitives::{ Amount, AmountOrAll, Feerate, Outpoint, PublicKey, Sha256 };
use serde_json::Value;

// Define a trait for types that can be created from an Option<Value>
pub trait FromOptionValue: Sized {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String>;
}

pub trait AddressString {
    fn to_string(&self) -> String;
}

impl AddressString for NewaddrAddresstype {
    fn to_string(&self) -> String {
        (
            match self {
                NewaddrAddresstype::BECH32 => "bech32",
                NewaddrAddresstype::P2TR => "p2tr",
                NewaddrAddresstype::ALL => "all",
                _ => "bech32",
            }
        ).to_string()
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

impl FromOptionValue for u64 {
    fn from_option_value(value: &Option<Value>) -> Result<Self, String> {
        value
            .as_ref()
            .and_then(|v| v.as_u64())
            .ok_or_else(|| "Failed to parse as u64".to_string())
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
    where F: Fn(Amount) -> T
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
                    .map(Amount::from_sat)
            };
            amount.map(constructor)
        }
        None => Err("No value provided".to_string()),
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

                        let txid = Sha256::from_str(parts[0]).map_err(|_|
                            format!("Failed to parse txid {} as Sha256", parts[0])
                        )?;
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

#[allow(private_bounds)]
// Generalized get_option_as function
pub fn get_option_as<T: FromOptionValue>(
    options_map: &HashMap<String, Option<Value>>,
    key: &str
) -> Option<T> {
    options_map.get(key).and_then(|v| T::from_option_value(v).ok())
}
