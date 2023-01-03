use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Name{
    pub(crate) name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    pub(crate) Subscribe: Name
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SubscribeError{
    AlreadyRegistered, InvalidName
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SubscribeResultEnum{
    Ok,
    Err(SubscribeError)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SubscribeResult{
    pub(crate) SubscribeResult: SubscribeResultEnum
}