use serde_json::json;

pub mod addgossip;
pub mod connect;
pub mod createinvoice;
pub mod fundchannel;
pub mod get_connection_string;
pub mod info;
pub mod invoice;
pub mod listchannels;
pub mod listfunds;
pub mod listpeers;
pub mod newaddr;
pub mod pay;
pub mod ping;
pub mod sendpay;
pub mod withdraw;

fn format_json(res: cln_rpc::Response) -> String {
    let data = serde_json::to_string_pretty(&json!(res)).unwrap();
    format!("```json\n{}\n```", data)
}
