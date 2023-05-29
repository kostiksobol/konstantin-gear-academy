#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<InitHello, ()>;
    type Handle = InOut<HelloAction, HelloEvent>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = HelloState;
}

#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct HelloState{
    pub greeting: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitHello {
    pub greeting: String,
}

#[derive(Clone, Encode, Decode, TypeInfo)]
pub enum HelloAction{
    SendHelloTo(ActorId),
    SendHelloReply,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum HelloEvent{
    HelloToSended,
    HelloReplySended,
}

