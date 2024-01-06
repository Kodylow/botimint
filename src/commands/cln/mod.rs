use std::sync::Arc;

use cln_rpc::ClnRpc;
use serenity::model::prelude::application_command::CommandData;
use serenity::prelude::Context;
use tokio::sync::Mutex;

use crate::utils::discord_utils::create_and_log_command;

pub mod addgossip;
pub mod autoclean;
pub mod checkmessage;
pub mod close;
pub mod connect;
pub mod createinvoice;
pub mod createonion;
pub mod datastore;
pub mod decode;
pub mod decodepay;
pub mod deldatastore;
pub mod delexpiredinvoice;
pub mod delinvoice;
pub mod disconnect;
pub mod feerates;
pub mod fundchannel;
pub mod fundpsbt;
pub mod get_connection_string;
pub mod getroute;
pub mod info;
pub mod invoice;
pub mod keysend;
pub mod listchannels;
pub mod listclosedchannels;
pub mod listdatastore;
pub mod listforwards;
pub mod listfunds;
pub mod listhtlcs;
pub mod listinvoices;
pub mod listnodes;
pub mod listpays;
pub mod listpeerchannels;
pub mod listpeers;
pub mod listsendpays;
pub mod listtransactions;
pub mod newaddr;
pub mod pay;
pub mod ping;
pub mod preapproveinvoice;
pub mod preapprovekeysend;
pub mod sendcustommsg;
pub mod sendonion;
pub mod sendpay;
pub mod sendpsbt;
pub mod setchannel;
pub mod signinvoice;
pub mod signmessage;
pub mod signpsbt;
pub mod staticbackup;
pub mod stop;
pub mod txdiscard;
pub mod txprepare;
pub mod txsend;
pub mod utxopsbt;
pub mod waitanyinvoice;
pub mod waitinvoice;
pub mod waitsendpay;
pub mod withdraw;

pub enum ClnCommand {
    ClnInfo,
    ClnListPeers,
    ClnListFunds,
    ClnConnect,
    ClnNewAddr,
    ClnCreateInvoice,
    ClnFundChannel,
    ClnSendPay,
    ClnPay,
    ClnPing,
    ClnListChannels,
    ClnAddGossip,
    ClnAutoClean,
    ClnCheckMessage,
    ClnClose,
    ClnDatastore,
    ClnCreateOnion,
    ClnDelDatastore,
    ClnDelExpiredInvoice,
    ClnDelInvoice,
    ClnInvoice,
    ClnListDatastore,
    ClnListInvoices,
    ClnSendOnion,
    ClnListSendPays,
    ClnListTransactions,
    ClnListNodes,
    ClnWaitAnyInvoice,
    ClnWaitInvoice,
    ClnWaitSendPay,
    ClnWithdraw,
    ClnKeySend,
    ClnFundPsbt,
    ClnSendPsbt,
    ClnSignPsbt,
    ClnUtxoPsbt,
    ClnTxDiscard,
    ClnTxPrepare,
    ClnTxSend,
    ClnListClosedChannels,
    ClnListPeerChannels,
    ClnDecode,
    ClnDecodePay,
    ClnDisconnect,
    ClnFeerates,
    ClnGetRoute,
    ClnListForwards,
    ClnListHtlcs,
    ClnListPays,
    ClnStop,
    ClnPreApproveInvoice,
    ClnPreApproveKeySend,
    ClnSendCustomMsg,
    ClnSetChannel,
    ClnSignInvoice,
    ClnSignMessage,
    ClnStaticBackup,
    Unknown,
}

impl From<&str> for ClnCommand {
    fn from(s: &str) -> Self {
        match s {
            "cln_info" => Self::ClnInfo,
            "cln_listpeers" => Self::ClnListPeers,
            "cln_listfunds" => Self::ClnListFunds,
            "cln_connect" => Self::ClnConnect,
            "cln_newaddr" => Self::ClnNewAddr,
            "cln_createinvoice" => Self::ClnCreateInvoice,
            "cln_fundchannel" => Self::ClnFundChannel,
            "cln_sendpay" => Self::ClnSendPay,
            "cln_pay" => Self::ClnPay,
            "cln_ping" => Self::ClnPing,
            "cln_listchannels" => Self::ClnListChannels,
            "cln_addgossip" => Self::ClnAddGossip,
            "cln_autoclean" => Self::ClnAutoClean,
            "cln_checkmessage" => Self::ClnCheckMessage,
            "cln_close" => Self::ClnClose,
            "cln_datastore" => Self::ClnDatastore,
            "cln_createonion" => Self::ClnCreateOnion,
            "cln_deldatastore" => Self::ClnDelDatastore,
            "cln_delexpiredinvoice" => Self::ClnDelExpiredInvoice,
            "cln_delinvoice" => Self::ClnDelInvoice,
            "cln_invoice" => Self::ClnInvoice,
            "cln_listdatastore" => Self::ClnListDatastore,
            "cln_listinvoices" => Self::ClnListInvoices,
            "cln_sendonion" => Self::ClnSendOnion,
            "cln_listsendpays" => Self::ClnListSendPays,
            "cln_listtransactions" => Self::ClnListTransactions,
            "cln_listnodes" => Self::ClnListNodes,
            "cln_waitanyinvoice" => Self::ClnWaitAnyInvoice,
            "cln_waitinvoice" => Self::ClnWaitInvoice,
            "cln_waitsendpay" => Self::ClnWaitSendPay,
            "cln_withdraw" => Self::ClnWithdraw,
            "cln_keysend" => Self::ClnKeySend,
            "cln_fundpsbt" => Self::ClnFundPsbt,
            "cln_sendpsbt" => Self::ClnSendPsbt,
            "cln_signpsbt" => Self::ClnSignPsbt,
            "cln_utxopsbt" => Self::ClnUtxoPsbt,
            "cln_txdiscard" => Self::ClnTxDiscard,
            "cln_txprepare" => Self::ClnTxPrepare,
            "cln_txsend" => Self::ClnTxSend,
            "cln_listclosedchannels" => Self::ClnListClosedChannels,
            "cln_listpeerchannels" => Self::ClnListPeerChannels,
            "cln_decode" => Self::ClnDecode,
            "cln_decodepay" => Self::ClnDecodePay,
            "cln_disconnect" => Self::ClnDisconnect,
            "cln_feerates" => Self::ClnFeerates,
            "cln_getroute" => Self::ClnGetRoute,
            "cln_listforwards" => Self::ClnListForwards,
            "cln_listhtlcs" => Self::ClnListHtlcs,
            "cln_listpays" => Self::ClnListPays,
            "cln_stop" => Self::ClnStop,
            "cln_preapproveinvoice" => Self::ClnPreApproveInvoice,
            "cln_preapprovekeysend" => Self::ClnPreApproveKeySend,
            "cln_sendcustommsg" => Self::ClnSendCustomMsg,
            "cln_setchannel" => Self::ClnSetChannel,
            "cln_signinvoice" => Self::ClnSignInvoice,
            "cln_signmessage" => Self::ClnSignMessage,
            "cln_staticbackup" => Self::ClnStaticBackup,
            _ => Self::Unknown,
        }
    }
}

pub async fn ready(ctx: &Context) {
    let commands = vec![
        info::register,
        listpeers::register,
        listfunds::register,
        connect::register,
        newaddr::register,
        createinvoice::register,
        fundchannel::register,
        sendpay::register,
        pay::register,
        ping::register,
        listchannels::register,
        addgossip::register,
        autoclean::register,
        checkmessage::register,
        close::register,
        datastore::register,
        createonion::register,
        deldatastore::register,
        delexpiredinvoice::register,
        delinvoice::register,
        invoice::register,
        listdatastore::register,
        listinvoices::register,
        sendonion::register,
        listsendpays::register,
        listtransactions::register,
        listnodes::register,
        waitanyinvoice::register,
        waitinvoice::register,
        waitsendpay::register,
        withdraw::register,
        keysend::register,
        fundpsbt::register,
        sendpsbt::register,
        signpsbt::register,
        utxopsbt::register,
        txdiscard::register,
        txprepare::register,
        txsend::register,
        listclosedchannels::register,
        listpeerchannels::register,
        decode::register,
        decodepay::register,
        disconnect::register,
        feerates::register,
        getroute::register,
        listforwards::register,
        listhtlcs::register,
        listpays::register,
        stop::register,
        preapproveinvoice::register,
        preapprovekeysend::register,
        sendcustommsg::register,
        setchannel::register,
        signinvoice::register,
        signmessage::register,
        staticbackup::register,
    ];

    for command in commands {
        create_and_log_command(&ctx.http, command).await;
    }
}

pub async fn handle_run(
    command_name: &str,
    command_data: &CommandData,
    cln_client: &Arc<Mutex<ClnRpc>>,
) -> String {
    match ClnCommand::from(command_name) {
        ClnCommand::ClnInfo => info::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListPeers => listpeers::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListFunds => listfunds::run(&command_data.options, cln_client).await,
        ClnCommand::ClnConnect => connect::run(&command_data.options, cln_client).await,
        ClnCommand::ClnNewAddr => newaddr::run(&command_data.options, cln_client).await,
        ClnCommand::ClnCreateInvoice => createinvoice::run(&command_data.options, cln_client).await,
        ClnCommand::ClnFundChannel => fundchannel::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSendPay => sendpay::run(&command_data.options, cln_client).await,
        ClnCommand::ClnPay => pay::run(&command_data.options, cln_client).await,
        ClnCommand::ClnPing => ping::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListChannels => listchannels::run(&command_data.options, cln_client).await,
        ClnCommand::ClnAddGossip => addgossip::run(&command_data.options, cln_client).await,
        ClnCommand::ClnAutoClean => autoclean::run(&command_data.options, cln_client).await,
        ClnCommand::ClnCheckMessage => checkmessage::run(&command_data.options, cln_client).await,
        ClnCommand::ClnClose => close::run(&command_data.options, cln_client).await,
        ClnCommand::ClnDatastore => datastore::run(&command_data.options, cln_client).await,
        ClnCommand::ClnCreateOnion => createonion::run(&command_data.options, cln_client).await,
        ClnCommand::ClnDelDatastore => deldatastore::run(&command_data.options, cln_client).await,
        ClnCommand::ClnDelExpiredInvoice => {
            delexpiredinvoice::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnDelInvoice => delinvoice::run(&command_data.options, cln_client).await,
        ClnCommand::ClnInvoice => invoice::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListDatastore => listdatastore::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListInvoices => listinvoices::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSendOnion => sendonion::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListSendPays => listsendpays::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListTransactions => {
            listtransactions::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnListNodes => listnodes::run(&command_data.options, cln_client).await,
        ClnCommand::ClnWaitAnyInvoice => {
            waitanyinvoice::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnWaitInvoice => waitinvoice::run(&command_data.options, cln_client).await,
        ClnCommand::ClnWaitSendPay => waitsendpay::run(&command_data.options, cln_client).await,
        ClnCommand::ClnWithdraw => withdraw::run(&command_data.options, cln_client).await,
        ClnCommand::ClnKeySend => keysend::run(&command_data.options, cln_client).await,
        ClnCommand::ClnFundPsbt => fundpsbt::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSendPsbt => sendpsbt::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSignPsbt => signpsbt::run(&command_data.options, cln_client).await,
        ClnCommand::ClnUtxoPsbt => utxopsbt::run(&command_data.options, cln_client).await,
        ClnCommand::ClnTxDiscard => txdiscard::run(&command_data.options, cln_client).await,
        ClnCommand::ClnTxPrepare => txprepare::run(&command_data.options, cln_client).await,
        ClnCommand::ClnTxSend => txsend::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListClosedChannels => {
            listclosedchannels::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnListPeerChannels => {
            listpeerchannels::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnDecode => decode::run(&command_data.options, cln_client).await,
        ClnCommand::ClnDecodePay => decodepay::run(&command_data.options, cln_client).await,
        ClnCommand::ClnDisconnect => disconnect::run(&command_data.options, cln_client).await,
        ClnCommand::ClnFeerates => feerates::run(&command_data.options, cln_client).await,
        ClnCommand::ClnGetRoute => getroute::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListForwards => listforwards::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListHtlcs => listhtlcs::run(&command_data.options, cln_client).await,
        ClnCommand::ClnListPays => listpays::run(&command_data.options, cln_client).await,
        ClnCommand::ClnStop => stop::run(&command_data.options, cln_client).await,
        ClnCommand::ClnPreApproveInvoice => {
            preapproveinvoice::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnPreApproveKeySend => {
            preapprovekeysend::run(&command_data.options, cln_client).await
        }
        ClnCommand::ClnSendCustomMsg => sendcustommsg::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSetChannel => setchannel::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSignInvoice => signinvoice::run(&command_data.options, cln_client).await,
        ClnCommand::ClnSignMessage => signmessage::run(&command_data.options, cln_client).await,
        ClnCommand::ClnStaticBackup => staticbackup::run(&command_data.options, cln_client).await,
        ClnCommand::Unknown => format!("Unknown command: {}", command_name),
    }
}
