use cln_rpc::model::requests::NewaddrAddresstype;

pub trait AddressString {
    fn to_string(&self) -> String;
}

impl AddressString for NewaddrAddresstype {
    fn to_string(&self) -> String {
        (match self {
            NewaddrAddresstype::BECH32 => "bech32",
            NewaddrAddresstype::P2TR => "p2tr",
            NewaddrAddresstype::ALL => "all",
        })
        .to_string()
    }
}
