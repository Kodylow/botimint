use serde_json::json;

pub mod address_utils;
pub mod discord_utils;
pub mod get_option_as;

pub fn format_json(res: cln_rpc::Response) -> String {
    let data = serde_json::to_string_pretty(&json!(res)).unwrap();
    format!("```json\n{}\n```", data)
}

pub fn to_codeblock(val: String) -> String {
    format!("```json\n{}\n```", val)
}
