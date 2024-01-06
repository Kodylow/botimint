use std::time::UNIX_EPOCH;

use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;
use time::format_description::well_known::iso8601;
use time::OffsetDateTime;

use crate::commands::CommandOptionInfo;
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Deserialize)]
pub struct ListOperationsRequest {
    pub limit: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct OperationOutput {
    pub id: OperationId,
    pub creation_time: String,
    pub operation_kind: String,
    pub operation_meta: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<serde_json::Value>,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = crate::commands::discord_command_options_to_map(options);
    let limit: usize = get_option_as(&options_map, "limit").unwrap();
    const ISO8601_CONFIG: iso8601::EncodedConfig = iso8601::Config::DEFAULT
        .set_formatted_components(iso8601::FormattedComponents::DateTime)
        .encode();
    let operations = fm_client
        .operation_log()
        .list_operations(limit, None)
        .await
        .into_iter()
        .map(|(k, v)| {
            let creation_time = OffsetDateTime::from_unix_timestamp(
                k.creation_time
                    .duration_since(UNIX_EPOCH)
                    .expect("Couldn't convert time from SystemTime to timestamp")
                    .as_secs() as i64,
            )
            .expect("Couldn't convert time from SystemTime to OffsetDateTime")
            .format(&iso8601::Iso8601::<ISO8601_CONFIG>)
            .expect("Couldn't format OffsetDateTime as ISO8601");

            OperationOutput {
                id: k.operation_id,
                creation_time,
                operation_kind: v.operation_module_kind().to_owned(),
                operation_meta: v.meta(),
                outcome: v.outcome(),
            }
        })
        .collect::<Vec<_>>();
    to_codeblock(serde_json::to_string_pretty(&operations).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "limit",
        description: "The maximum number of operations to return",
        kind: CommandOptionType::Integer,
        required: true,
    }];

    command
        .name("fm_list_operations")
        .description("Get the list of current operations");

    for opt_info in options {
        command.create_option(|opt| {
            opt.name(opt_info.name)
                .description(opt_info.description)
                .kind(opt_info.kind)
                .required(opt_info.required)
        });
    }

    command
}
