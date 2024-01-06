use fedimint_client::ClientArc;
use serenity::model::prelude::application_command::CommandData;
use serenity::prelude::Context;

use crate::utils::discord_utils::create_and_log_command;

pub mod backup;
pub mod config;
pub mod discover_version;
pub mod id;
pub mod info;
pub mod list_operations;
pub mod ln;
pub mod mint;
pub mod wallet;

pub enum FmCommand {
    Backup,
    Config,
    DiscoverVersion,
    Id,
    Info,
    ListOperations,
    LnAwaitInvoice,
    LnAwaitPay,
    LnInvoice,
    LnPay,
    LnListGateways,
    LnSwitchGateway,
    MintReissue,
    MintSpend,
    MintSplit,
    MintValidate,
    WalletAwaitDeposit,
    WalletDepositAddress,
    WalletWithdraw,
    Unknown,
}

impl From<&str> for FmCommand {
    fn from(s: &str) -> Self {
        match s {
            "fm_backup" => Self::Backup,
            "fm_config" => Self::Config,
            "fm_discover_version" => Self::DiscoverVersion,
            "fm_id" => Self::Id,
            "fm_info" => Self::Info,
            "fm_list_operations" => Self::ListOperations,
            "fm_ln_await_invoice" => Self::LnAwaitInvoice,
            "fm_ln_await_pay" => Self::LnAwaitPay,
            "fm_ln_invoice" => Self::LnInvoice,
            "fm_ln_pay" => Self::LnPay,
            "fm_ln_list_gateways" => Self::LnListGateways,
            "fm_ln_switch_gateway" => Self::LnSwitchGateway,
            "fm_mint_reissue" => Self::MintReissue,
            "fm_mint_spend" => Self::MintSpend,
            "fm_mint_split" => Self::MintSplit,
            "fm_mint_validate" => Self::MintValidate,
            "fm_wallet_await_deposit" => Self::WalletAwaitDeposit,
            "fm_wallet_deposit_address" => Self::WalletDepositAddress,
            "fm_wallet_withdraw" => Self::WalletWithdraw,
            _ => Self::Unknown,
        }
    }
}

pub async fn ready(ctx: &Context) {
    let commands = vec![
        backup::register,
        config::register,
        discover_version::register,
        id::register,
        info::register,
        list_operations::register,
        ln::await_invoice::register,
        ln::await_pay::register,
        ln::invoice::register,
        ln::pay::register,
        ln::list_gateways::register,
        ln::switch_gateway::register,
        mint::reissue::register,
        mint::spend::register,
        mint::split::register,
        mint::validate::register,
        wallet::await_deposit::register,
        wallet::deposit_address::register,
        wallet::withdraw::register,
    ];

    for command in commands {
        create_and_log_command(&ctx.http, command).await;
    }
}

pub async fn handle_run(
    command_name: &str,
    command_data: &CommandData,
    fm_client: &ClientArc,
) -> String {
    match FmCommand::from(command_name) {
        FmCommand::Backup => backup::run(&command_data.options, fm_client).await,
        FmCommand::Config => config::run(&command_data.options, fm_client).await,
        FmCommand::DiscoverVersion => discover_version::run(&command_data.options, fm_client).await,
        FmCommand::Id => id::run(&command_data.options, fm_client).await,
        FmCommand::Info => info::run(&command_data.options, fm_client).await,
        FmCommand::ListOperations => list_operations::run(&command_data.options, fm_client).await,
        FmCommand::LnAwaitInvoice => ln::await_invoice::run(&command_data.options, fm_client).await,
        FmCommand::LnAwaitPay => ln::await_pay::run(&command_data.options, fm_client).await,
        FmCommand::LnInvoice => ln::invoice::run(&command_data.options, fm_client).await,
        FmCommand::LnPay => ln::pay::run(&command_data.options, fm_client).await,
        FmCommand::LnListGateways => ln::list_gateways::run(&command_data.options, fm_client).await,
        FmCommand::LnSwitchGateway => {
            ln::switch_gateway::run(&command_data.options, fm_client).await
        }
        FmCommand::MintReissue => mint::reissue::run(&command_data.options, fm_client).await,
        FmCommand::MintSpend => mint::spend::run(&command_data.options, fm_client).await,
        FmCommand::MintSplit => mint::split::run(&command_data.options, fm_client).await,
        FmCommand::MintValidate => mint::validate::run(&command_data.options, fm_client).await,
        FmCommand::WalletAwaitDeposit => {
            wallet::await_deposit::run(&command_data.options, fm_client).await
        }
        FmCommand::WalletDepositAddress => {
            wallet::deposit_address::run(&command_data.options, fm_client).await
        }
        FmCommand::WalletWithdraw => wallet::withdraw::run(&command_data.options, fm_client).await,
        FmCommand::Unknown => format!("Unknown command: {}", command_name),
    }
}
