use serenity::model::prelude::command::Command;

pub async fn create_and_log_command<F>(
    http: &serenity::http::Http,
    command_register: F,
) -> serenity::Result<()>
where
    F: FnOnce(
            &mut serenity::builder::CreateApplicationCommand,
        ) -> &mut serenity::builder::CreateApplicationCommand
        + Send
        + Sync,
{
    let command = Command::create_global_application_command(http, command_register).await?;
    println!(
        "I created the following global slash command: {:#?}",
        command
    );
    Ok(())
}
