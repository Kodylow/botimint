```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "lowercase")]
pub enum Request {
    // Getinfo(requests::GetinfoRequest),
    // ListPeers(requests::ListpeersRequest),
    // ListFunds(requests::ListfundsRequest),
    // SendPay(requests::SendpayRequest),
    // ListChannels(requests::ListchannelsRequest),
    // AddGossip(requests::AddgossipRequest),
    // AutoCleanInvoice(requests::AutocleaninvoiceRequest),
    // CheckMessage(requests::CheckmessageRequest),
    // Close(requests::CloseRequest),
    // Connect(requests::ConnectRequest),
    // CreateInvoice(requests::CreateinvoiceRequest),
    // Datastore(requests::DatastoreRequest),
    // CreateOnion(requests::CreateonionRequest),
    // DelDatastore(requests::DeldatastoreRequest),
    // DelExpiredInvoice(requests::DelexpiredinvoiceRequest),
    // DelInvoice(requests::DelinvoiceRequest),
    // Invoice(requests::InvoiceRequest),
    // ListDatastore(requests::ListdatastoreRequest),
    // ListInvoices(requests::ListinvoicesRequest),
    // SendOnion(requests::SendonionRequest),
    // ListSendPays(requests::ListsendpaysRequest),
    // ListTransactions(requests::ListtransactionsRequest),
    // Pay(requests::PayRequest),
    // ListNodes(requests::ListnodesRequest),
    // WaitAnyInvoice(requests::WaitanyinvoiceRequest),
    // WaitInvoice(requests::WaitinvoiceRequest),
    // WaitSendPay(requests::WaitsendpayRequest),
    // NewAddr(requests::NewaddrRequest),
    // Withdraw(requests::WithdrawRequest),
    // KeySend(requests::KeysendRequest),
    // FundPsbt(requests::FundpsbtRequest),
    // SendPsbt(requests::SendpsbtRequest),
    // SignPsbt(requests::SignpsbtRequest),
    // UtxoPsbt(requests::UtxopsbtRequest),
    // TxDiscard(requests::TxdiscardRequest),
    // TxPrepare(requests::TxprepareRequest),
    // TxSend(requests::TxsendRequest),
    // ListPeerChannels(requests::ListpeerchannelsRequest),
    // ListClosedChannels(requests::ListclosedchannelsRequest),
    // DecodePay(requests::DecodepayRequest),
    // Decode(requests::DecodeRequest),
    Disconnect(requests::DisconnectRequest),
    Feerates(requests::FeeratesRequest),
    // FundChannel(requests::FundchannelRequest),
    GetRoute(requests::GetrouteRequest),
    ListForwards(requests::ListforwardsRequest),
    ListPays(requests::ListpaysRequest),
    ListHtlcs(requests::ListhtlcsRequest),
    // Ping(requests::PingRequest),
    SendCustomMsg(requests::SendcustommsgRequest),
    SetChannel(requests::SetchannelRequest),
    SignInvoice(requests::SigninvoiceRequest),
    SignMessage(requests::SignmessageRequest),
    Stop(requests::StopRequest),
    PreApproveKeysend(requests::PreapprovekeysendRequest),
    PreApproveInvoice(requests::PreapproveinvoiceRequest),
    StaticBackup(requests::StaticbackupRequest),
}
```
