use serde_json::json;

pub mod addgossip;
pub mod autoclean;
pub mod checkmessage;
pub mod close;
pub mod connect;
pub mod createinvoice;
pub mod createonion;
pub mod datastore;
pub mod deldatastore;
pub mod delexpiredinvoice;
pub mod delinvoice;
pub mod fundchannel;
pub mod get_connection_string;
pub mod info;
pub mod invoice;
pub mod listchannels;
pub mod listdatastore;
pub mod listfunds;
pub mod listinvoices;
pub mod listpeers;
pub mod listsendpays;
pub mod listtransactions;
pub mod newaddr;
pub mod pay;
pub mod ping;
pub mod sendonion;
pub mod sendpay;
pub mod withdraw;

fn format_json(res: cln_rpc::Response) -> String {
    let data = serde_json::to_string_pretty(&json!(res)).unwrap();
    format!("```json\n{}\n```", data)
}
