pub use mailbox::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mailbox {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_localDomain"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_MESSAGE_BODY_BYTES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_MESSAGE_BODY_BYTES",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("count"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultIsm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultIsm"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainSecurityModule",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delivered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delivered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dispatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dispatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipientAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_messageBody"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_defaultIsm"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPaused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestCheckpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestCheckpoint"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("localDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("localDomain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("process"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("process"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_metadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recipientIsm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recipientIsm"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainSecurityModule",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("root"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("root"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDefaultIsm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDefaultIsm"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tree"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tree"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DefaultIsmSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DefaultIsmSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Dispatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Dispatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DispatchId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DispatchId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Process"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Process"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProcessId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProcessId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MAILBOX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qb\0\")8\x03\x80b\0\")\x839\x81\x01`@\x81\x90Ra\x001\x91a\0?V[c\xFF\xFF\xFF\xFF\x16`\x80Ra\0lV[`\0` \x82\x84\x03\x12\x15a\0QW`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0eW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa!\x93b\0\0\x96`\09`\0\x81\x81a\x020\x01R\x81\x81a\x060\x01Ra\r\x05\x01Ra!\x93`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01lW`\x005`\xE0\x1C\x80c\x90|\x0F\x92\x11a\0\xCDW\x80c\xF2\xFD\xE3\x8B\x11a\0\x81W\x80c\xFA1\xDE\x01\x11a\0fW\x80c\xFA1\xDE\x01\x14a\x03\x0EW\x80c\xFDT\xB2(\x14a\x03!W\x80c\xFF\xA1\xADt\x14a\x03+W`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\x02\xE8W\x80c\xF7\x94hz\x14a\x02\xFBW`\0\x80\xFD[\x80c\xE4\x95\xF1\xD4\x11a\0\xB2W\x80c\xE4\x95\xF1\xD4\x14a\x02\xAAW\x80c\xE7\x0FH\xAC\x14a\x02\xCDW\x80c\xEB\xF0\xC7\x17\x14a\x02\xE0W`\0\x80\xFD[\x80c\x90|\x0F\x92\x14a\x02pW\x80c\xB1\x87\xBD&\x14a\x02\x92W`\0\x80\xFD[\x80cqP\x18\xA6\x11a\x01$W\x80c\x84V\xCBY\x11a\x01\tW\x80c\x84V\xCBY\x14a\x02#W\x80c\x8D68\xF4\x14a\x02+W\x80c\x8D\xA5\xCB[\x14a\x02RW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x08W\x80c|9\xD10\x14a\x02\x10W`\0\x80\xFD[\x80cH\\\xC9U\x11a\x01UW\x80cH\\\xC9U\x14a\x01\x99W\x80cR*\xE0\x02\x14a\x01\xACW\x80cn_Qn\x14a\x01\xC3W`\0\x80\xFD[\x80c\x06f\x1A\xBD\x14a\x01qW\x80c?K\xA8:\x14a\x01\x8FW[`\0\x80\xFD[`\xB8T[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x97a\x03EV[\0[a\x01\x97a\x01\xA76`\x04a\x1A\x93V[a\x03\x80V[a\x01\xB5a\x08\0\x81V[`@Q\x90\x81R` \x01a\x01\x86V[`\x97Ta\x01\xE3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x86V[a\x01\x97a\x053V[a\x01\x97a\x02\x1E6`\x04a\x1B\x0EV[a\x05GV[a\x01\x97a\t\xFAV[a\x01u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xE3V[a\x02xa\n5V[`@\x80Q\x92\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\x86V[a\x02\x9Aa\n]V[`@Q\x90\x15\x15\x81R` \x01a\x01\x86V[a\x02\x9Aa\x02\xB86`\x04a\x1BzV[`\xB9` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xE3a\x02\xDB6`\x04a\x1B\x93V[a\npV[a\x01\xB5a\x0B>V[a\x01\x97a\x02\xF66`\x04a\x1B\x93V[a\x0BJV[a\x01\x97a\x03\t6`\x04a\x1B\x93V[a\x0C\x01V[a\x01\xB5a\x03\x1C6`\x04a\x1B\xB0V[a\x0C\x12V[`\xB8Ta\x01\xB5\x90\x81V[a\x033`\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x86V[a\x03Ma\r\xCEV[a\x03Ua\x0EOV[`@Q\x7F\xA4_G\xFD\xEA\x8A\x1E\xFD\xD9\x02\x9AV\x91\xC7\xF7Y\xC3+|i\x862\xB5cW>\x15V%\xD1i3\x90`\0\x90\xA1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x03\xA0WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x03\xBAWP0;\x15\x80\x15a\x03\xBAWP`\0T`\xFF\x16`\x01\x14[a\x04KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x04\xA9W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x04\xB1a\x0E\xC2V[a\x04\xB9a\x0FYV[a\x04\xC2\x83a\x0BJV[a\x04\xCB\x82a\x0F\xF8V[\x80\x15a\x05.W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[a\x05;a\r\xCEV[a\x05E`\0a\x10\xE5V[V[`\x01`eT\x14a\x05\xB3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Freentrant call (or paused)\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0`e\x81\x90Ua\x05\xC4\x83\x83a\x11\\V[`\xFF\x16\x14a\x06.W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x08`$\x82\x01R\x7F!version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16a\x06_\x83\x83a\x11\x80V[c\xFF\xFF\xFF\xFF\x16\x14a\x06\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7F!destination\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0a\x07\r\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x11\xA3\x92PPPV[`\0\x81\x81R`\xB9` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x07\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7Fdelivered\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0\x81\x81R`\xB9` R`@\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x07\xCDa\x02\xDB\x85\x85a\x11\xAEV[`@Q\x7F\xF7\xE8:\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c\xF7\xE8:\xEE\x90a\x08(\x90\x89\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x1CRV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08k\x91\x90a\x1C\x84V[a\x08\xD1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7F!module\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0a\x08\xDD\x85\x85a\x11\xC7V[\x90P`\0a\x08\xEB\x86\x86a\x11\xD7V[\x90P`\0a\x08\xF9\x87\x87a\x11\xAEV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cV\xD5\xD4u\x84\x84a\t#\x8B\x8Ba\x11\xF0V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tB\x94\x93\x92\x91\x90a\x1C\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\tpW=`\0\x80>=`\0\xFD[PPPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x84c\xFF\xFF\xFF\xFF\x16\x7F\r8\x1C*WJ\xE8\xF0N!=\xB7\xCF\xB4\xDF\x8D\xF7\x12\xCD\xBDB}\x98h\xFF\xEF8\x06`\xCAet`@Q`@Q\x80\x91\x03\x90\xA4`@Q\x85\x90\x7F\x1C\xAE8\xCD\xD3\xD3\x91\x94\x89''%\xA5\xAEb\xA4\xF4\x8B)\x89\xB0\xDA\xE8C\xD3\xC2y\xFE\xE1\x80s\xA9\x90`\0\x90\xA2PP`\x01`eUPPPPPPPV[a\n\x02a\r\xCEV[a\n\na\x12\x0CV[`@Q\x7F\x9E\x87\xFA\xC8\x8F\xF6a\xF0-D\xF9S\x83\xC8\x17\xFE\xCEK\xCE`\n=\xABzT@hx\xB9e\xE7R\x90`\0\x90\xA1V[`\0\x80a\n@a\x0B>V[`\x01a\nK`\xB8T\x90V[a\nU\x91\x90a\x1D\x05V[\x91P\x91P\x90\x91V[`\0a\nk`eT`\x02\x14\x90V[\x90P\x90V[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDER<\xF3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\n\xF7WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\n\xF4\x91\x81\x01\x90a\x1D)V[`\x01[\x15a\x0B Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\x0B\x1EW\x92\x91PPV[P[PP`\x97Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\0a\nk`\x98a\x12\x7FV[a\x0BRa\r\xCEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0B\xF5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[a\x0B\xFE\x81a\x10\xE5V[PV[a\x0C\ta\r\xCEV[a\x0B\xFE\x81a\x0F\xF8V[`\0a\x0C `eT`\x02\x14\x90V[\x15a\x0C\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x06`$\x82\x01R\x7Fpaused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[a\x08\0\x82\x11\x15a\x0C\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7Fmsg too long\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0a\r.`\0a\r\x03`\xB8T\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x8A\x8A\x8A\x8Aa\x12\x92V[\x80Q` \x82\x01 \x90\x91Pa\rC`\x98\x82a\x12\xD0V[\x85\x87c\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fv\x9Fq\x1D \xC6y\x15=8\"T\xF5\x98\x92a;X\xA9|\xC8v\xB2I\x13J\xC2\\\x80\xF9\xC8\x14\x85`@Qa\r\x91\x91\x90a\x1DFV[`@Q\x80\x91\x03\x90\xA4`@Q\x81\x90\x7Fx\x8D\xBC\x1BqRs!x!\x0E\x7FM\x9D\x01\x0E\xF0\x16\xF9\xEA\xFB\xE6g\x86\xBDqi\xF5n\x0C5:\x90`\0\x90\xA2\x96\x95PPPPPPV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04BV[`eT`\x02\x14a\x0E\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7F!paused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x01`eUV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0E\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[a\x05Ea\x14\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x10vW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7F!contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x97\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xA7j\xD0\xAD\xBFE1\x8F\x863\xAA\x02\x10\xF7\x11'=P\xFB\xB6\xFE\xF7n\xD9[\xBA\xE9p\x82\xC7]\xAA\x90`\0\x90\xA2PV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0a\x11k`\x01\x82\x84\x86a\x1D\xB2V[a\x11t\x91a\x1D\xDCV[`\xF8\x1C\x90P[\x92\x91PPV[`\0a\x11\x90`-`)\x84\x86a\x1D\xB2V[a\x11\x99\x91a\x1E\"V[`\xE0\x1C\x93\x92PPPV[\x80Q` \x90\x91\x01 \x90V[`\0a\x11\xC0a\x11\xBD\x84\x84a\x14\xAFV[\x90V[\x93\x92PPPV[`\0a\x11\x90`\t`\x05\x84\x86a\x1D\xB2V[`\0a\x11\xE7`)`\t\x84\x86a\x1D\xB2V[a\x11\xC0\x91a\x1EhV[6`\0a\x12\0\x83`M\x81\x87a\x1D\xB2V[\x91P\x91P[\x92P\x92\x90PV[`eT`\x02\x03a\x12xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x06`$\x82\x01R\x7Fpaused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x02`eUV[`\0a\x11z\x82a\x12\x8Da\x14\xBFV[a\x19\x80V[``\x88\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a\x12\xB3\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1E\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x98\x97PPPPPPPPV[`\x01a\x12\xDE` `\x02a ZV[a\x12\xE8\x91\x90a fV[\x82` \x01T\x10a\x13TW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fmerkle tree full\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x01\x82` \x01`\0\x82\x82Ta\x13i\x91\x90a yV[\x90\x91UPP` \x82\x01T`\0[` \x81\x10\x15a\x14\x06W\x81`\x01\x16`\x01\x03a\x13\xA5W\x82\x84\x82` \x81\x10a\x13\x9DWa\x13\x9Da \x8CV[\x01UPPPPV[\x83\x81` \x81\x10a\x13\xB7Wa\x13\xB7a \x8CV[\x01T`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x84\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P`\x02\x82a\x13\xF2\x91\x90a \xBBV[\x91P\x80a\x13\xFE\x81a \xF6V[\x91PPa\x13vV[Pa\x05.a!.V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x14\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[a\x05E3a\x10\xE5V[`\0a\x11\xE7`M`-\x84\x86a\x1D\xB2V[a\x14\xC7a\x1ARV[`\0\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` \x82\x01R\x7F\xB4\xC1\x19Q\x95|o\x8Fd,J\xF6\x1C\xD6\xB2F@\xFE\xC6\xDC\x7F\xC6\x07\xEE\x82\x06\xA9\x9E\x92A\r0`@\x82\x01R\x7F!\xDD\xB9\xA3V\x81\\?\xAC\x10&\xB6\xDE\xC5\xDF1$\xAF\xBA\xDBH\\\x9B\xA5\xA3\xE39\x8A\x04\xB7\xBA\x85``\x82\x01R\x7F\xE5\x87i\xB3*\x1B\xEA\xF1\xEA'7ZD\tZ\r\x1F\xB6d\xCE-\xD3X\xE7\xFC\xBF\xB7\x8C&\xA1\x93D`\x80\x82\x01R\x7F\x0E\xB0\x1E\xBF\xC9\xED'P\x0C\xD4\xDF\xC9y'-\x1F\t\x13\xCC\x9FfT\r~\x80\x05\x81\x11\t\xE1\xCF-`\xA0\x82\x01R\x7F\x88|\"\xBD\x87P\xD3@\x16\xAC<f\xB5\xFF\x10-\xAC\xDDs\xF6\xB0\x14\xE7\x10\xB5\x1E\x80\"\xAF\x9A\x19h`\xC0\x82\x01R\x7F\xFF\xD7\x01W\xE4\x80c\xFC3\xC9z\x05\x0F\x7Fd\x023\xBFdl\xC9\x8D\x95$\xC6\xB9+\xCF:\xB5o\x83`\xE0\x82\x01R\x7F\x98g\xCC_\x7F\x19k\x93\xBA\xE1\xE2~c t$E\xD2\x90\xF2&8'I\x8BT\xFE\xC59\xF7V\xAFa\x01\0\x82\x01R\x7F\xCE\xFA\xD4\xE5\x08\xC0\x98\xB9\xA7\xE1\xD8\xFE\xB1\x99U\xFB\x02\xBA\x96uXPxq\ti\xD3D\x0FPT\xE0a\x01 \x82\x01R\x7F\xF9\xDC>\x7F\xE0\x16\xE0P\xEF\xF2`3O\x18\xA5\xD4\xFE9\x1D\x82\t#\x19\xF5\x96O..\xB7\xC1\xC3\xA5a\x01@\x82\x01R\x7F\xF8\xB1:I\xE2\x82\xF6\t\xC3\x17\xA83\xFB\x8D\x97m\x11Q|W\x1D\x12!\xA2e\xD2Z\xF7x\xEC\xF8\x92a\x01`\x82\x01R\x7F4\x90\xC6\xCE\xEBE\n\xEC\xDC\x82\xE2\x82\x93\x03\x1D\x10\xC7\xD7;\xF8^W\xBF\x04\x1A\x976\n\xA2\xC5\xD9\x9Ca\x01\x80\x82\x01R\x7F\xC1\xDF\x82\xD9\xC4\xB8t\x13\xEA\xE2\xEF\x04\x8F\x94\xB4\xD3UL\xEAs\xD9+\x0Fz\xF9n\x02q\xC6\x91\xE2\xBBa\x01\xA0\x82\x01R\x7F\\g\xAD\xD7\xC6\xCA\xF3\x02%j\xDE\xDFz\xB1\x14\xDA\n\xCF\xE8p\xD4I\xA3\xA4\x89\xF7\x81\xD6Y\xE8\xBE\xCCa\x01\xC0\x82\x01R\x7F\xDA{\xCE\x9FN\x86\x18\xB6\xBD/A2\xCEy\x8C\xDCz`\xE7\xE1F\nr\x99\xE3\xC64*W\x96&\xD2a\x01\xE0\x82\x01R\x7F'3\xE5\x0FRn\xC2\xFA\x19\xA2+1\xE8\xEDP\xF2<\xD1\xFD\xF9L\x91T\xED:v\t\xA2\xF1\xFF\x98\x1Fa\x02\0\x82\x01R\x7F\xE1\xD3\xB5\xC8\x07\xB2\x81\xE4h<\xC6\xD61\\\xF9[\x9A\xDE\x86A\xDE\xFC\xB3#r\xF1\xC1&\xE3\x98\xEFza\x02 \x82\x01R\x7FZ-\xCE\n\x8A\x7Fh\xBBtV\x0F\x8Fq\x83|,.\xBB\xCB\xF7\xFF\xFBB\xAE\x18\x96\xF1?|ty\xA0a\x02@\x82\x01R\x7F\xB4j(\xB6\xF5U@\xF8\x94D\xF6=\xE07\x8E=\x12\x1B\xE0\x9E\x06\xCC\x9D\xED\x1C \xE6Xv\xD3j\xA0a\x02`\x82\x01R\x7F\xC6^\x96EdG\x86\xB6 \xE2\xDD*\xD6H\xDD\xFC\xBFJ~[\x1A:N\xCF\xE7\xF6Fg\xA3\xF0\xB7\xE2a\x02\x80\x82\x01R\x7F\xF4A\x85\x88\xED5\xA2E\x8C\xFF\xEB9\xB9=&\xF1\x8D*\xB1;\xDC\xE6\xAE\xE5\x8E{\x995\x9E\xC2\xDF\xD9a\x02\xA0\x82\x01R\x7FZ\x9C\x16\xDC\0\xD6\xEF\x18\xB7\x93:o\x8D\xC6\\\xCBUfq8wo}\xEA\x10\x10p\xDC\x87\x96\xE3wa\x02\xC0\x82\x01R\x7FM\xF8O@\xAE\x0C\x82)\xD0\xD6\x06\x9E\\\x8F9\xA7\xC2\x99gz\t\xD3g\xFC{\x05\xE3\xBC8\x0E\xE6Ra\x02\xE0\x82\x01R\x7F\xCD\xC7%\x95\xF7L{\x10C\xD0\xE1\xFF\xBA\xB74d\x8C\x83\x8D\xFB\x05'\xD9q\xB6\x02\xBC!l\x96\x19\xEFa\x03\0\x82\x01R\x7F\n\xBFZ\xC9t\xA1\xEDW\xF4\x05\n\xA5\x10\xDD\x9Ct\xF5\x08'{9\xD7\x97;\xB2\xDF\xCC\xC5\xEE\xB0a\x8Da\x03 \x82\x01R\x7F\xB8\xCDt\x04o\xF37\xF0\xA7\xBF,\x8E\x03\xE1\x0Fd,\x18\x86y\x8Dq\x80j\xB1\xE8\x88\xD9\xE5\xEE\x87\xD0a\x03@\x82\x01R\x7F\x83\x8CVU\xCB!\xC6\xCB\x831;Zc\x11u\xDF\xF4\x967r\xCC\xE9\x10\x81\x88\xB3J\xC8|\x81\xC4\x1Ea\x03`\x82\x01R\x7Ff.\xE4\xDD-\xD7\xB2\xBCpya\xB1\xE6F\xC4\x04vi\xDC\xB6XO\r\x8Dw\r\xAF]~}\xEB.a\x03\x80\x82\x01R\x7F8\x8A\xB2\x0E%s\xD1q\xA8\x81\x08\xE7\x9D\x82\x0E\x98\xF2l\x0B\x84\xAA\x8B/J\xA4\x96\x8D\xBB\x81\x8E\xA3\"a\x03\xA0\x82\x01R\x7F\x93#|P\xBAu\xEEH_L\"\xAD\xF2\xF7A@\x0B\xDF\x8Dj\x9C\xC7\xDF~\xCA\xE5v\"\x16e\xD75a\x03\xC0\x82\x01R\x7F\x84H\x81\x8B\xB4\xAEEb\x84\x9E\x94\x9E\x17\xAC\x16\xE0\xBE\x16h\x8E\x15k\\\xF1^\t\x8Cb|\0V\xA9a\x03\xE0\x82\x01R\x90V[` \x82\x01T`\0\x90\x81[` \x81\x10\x15a\x1AJW`\x01\x82\x82\x1C\x16`\0\x86\x83` \x81\x10a\x19\xADWa\x19\xADa \x8CV[\x01T\x90P\x81`\x01\x03a\x19\xEAW`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x86\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94Pa\x1A5V[\x84\x86\x84` \x81\x10a\x19\xFDWa\x19\xFDa \x8CV[` \x02\x01Q`@Q` \x01a\x1A\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94P[PP\x80\x80a\x1AB\x90a \xF6V[\x91PPa\x19\x8AV[PP\x92\x91PPV[`@Q\x80a\x04\0\x01`@R\x80` \x90` \x82\x02\x806\x837P\x91\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\xFEW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xA6W`\0\x80\xFD[\x825a\x1A\xB1\x81a\x1AqV[\x91P` \x83\x015a\x1A\xC1\x81a\x1AqV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1A\xDEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xF6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x12\x05W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1B$W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B<W`\0\x80\xFD[a\x1BH\x88\x83\x89\x01a\x1A\xCCV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1BaW`\0\x80\xFD[Pa\x1Bn\x87\x82\x88\x01a\x1A\xCCV[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\x1B\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1B\xA5W`\0\x80\xFD[\x815a\x11\xC0\x81a\x1AqV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1B\xC6W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1B\xDAW`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xFDW`\0\x80\xFD[a\x1Bn\x87\x82\x88\x01a\x1A\xCCV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`@\x81R`\0a\x1Cf`@\x83\x01\x86\x88a\x1C\tV[\x82\x81\x03` \x84\x01Ra\x1Cy\x81\x85\x87a\x1C\tV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1C\x96W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\xC0W`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x1C\xCC``\x83\x01\x84\x86a\x1C\tV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x1D\"Wa\x1D\"a\x1C\xD6V[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D;W`\0\x80\xFD[\x81Qa\x11\xC0\x81a\x1AqV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x1DsW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1DWV[P`\0`@\x82\x86\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80\x85\x85\x11\x15a\x1D\xC2W`\0\x80\xFD[\x83\x86\x11\x15a\x1D\xCFW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x815\x81\x81\x16\x91`\x01\x85\x10\x15a\x1AJW`\x01\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x815\x81\x81\x16\x91`\x04\x85\x10\x15a\x1AJW`\x04\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV[\x805` \x83\x10\x15a\x11zW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89`\xF8\x1B\x16\x81R`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x8A`\xE0\x1B\x16`\x01\x84\x01R\x80\x89`\xE0\x1B\x16`\x05\x84\x01R\x87`\t\x84\x01R\x80\x87`\xE0\x1B\x16`)\x84\x01RP\x84`-\x83\x01R\x82\x84`M\x84\x017P`\0\x91\x01`M\x01\x90\x81R\x97\x96PPPPPPPV[`\x01\x81\x81[\x80\x85\x11\x15a\x1F\x93W\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x1FyWa\x1Fya\x1C\xD6V[\x80\x85\x16\x15a\x1F\x86W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1F?V[P\x92P\x92\x90PV[`\0\x82a\x1F\xAAWP`\x01a\x11zV[\x81a\x1F\xB7WP`\0a\x11zV[\x81`\x01\x81\x14a\x1F\xCDW`\x02\x81\x14a\x1F\xD7Wa\x1F\xF3V[`\x01\x91PPa\x11zV[`\xFF\x84\x11\x15a\x1F\xE8Wa\x1F\xE8a\x1C\xD6V[PP`\x01\x82\x1Ba\x11zV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a \x16WP\x81\x81\na\x11zV[a  \x83\x83a\x1F:V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a RWa Ra\x1C\xD6V[\x02\x93\x92PPPV[`\0a\x11\xC0\x83\x83a\x1F\x9BV[\x81\x81\x03\x81\x81\x11\x15a\x11zWa\x11za\x1C\xD6V[\x80\x82\x01\x80\x82\x11\x15a\x11zWa\x11za\x1C\xD6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82a \xF1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a!'Wa!'a\x1C\xD6V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA7\xCAy\xCAr\x03U\xC58\x19\t\x7F\x91'\0J\xE2\xAB]\x8C\x13\x1EC\xC6\x03\xA28e\x84\x9FP\xCAdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static MAILBOX_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01lW`\x005`\xE0\x1C\x80c\x90|\x0F\x92\x11a\0\xCDW\x80c\xF2\xFD\xE3\x8B\x11a\0\x81W\x80c\xFA1\xDE\x01\x11a\0fW\x80c\xFA1\xDE\x01\x14a\x03\x0EW\x80c\xFDT\xB2(\x14a\x03!W\x80c\xFF\xA1\xADt\x14a\x03+W`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\x02\xE8W\x80c\xF7\x94hz\x14a\x02\xFBW`\0\x80\xFD[\x80c\xE4\x95\xF1\xD4\x11a\0\xB2W\x80c\xE4\x95\xF1\xD4\x14a\x02\xAAW\x80c\xE7\x0FH\xAC\x14a\x02\xCDW\x80c\xEB\xF0\xC7\x17\x14a\x02\xE0W`\0\x80\xFD[\x80c\x90|\x0F\x92\x14a\x02pW\x80c\xB1\x87\xBD&\x14a\x02\x92W`\0\x80\xFD[\x80cqP\x18\xA6\x11a\x01$W\x80c\x84V\xCBY\x11a\x01\tW\x80c\x84V\xCBY\x14a\x02#W\x80c\x8D68\xF4\x14a\x02+W\x80c\x8D\xA5\xCB[\x14a\x02RW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x08W\x80c|9\xD10\x14a\x02\x10W`\0\x80\xFD[\x80cH\\\xC9U\x11a\x01UW\x80cH\\\xC9U\x14a\x01\x99W\x80cR*\xE0\x02\x14a\x01\xACW\x80cn_Qn\x14a\x01\xC3W`\0\x80\xFD[\x80c\x06f\x1A\xBD\x14a\x01qW\x80c?K\xA8:\x14a\x01\x8FW[`\0\x80\xFD[`\xB8T[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x97a\x03EV[\0[a\x01\x97a\x01\xA76`\x04a\x1A\x93V[a\x03\x80V[a\x01\xB5a\x08\0\x81V[`@Q\x90\x81R` \x01a\x01\x86V[`\x97Ta\x01\xE3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x86V[a\x01\x97a\x053V[a\x01\x97a\x02\x1E6`\x04a\x1B\x0EV[a\x05GV[a\x01\x97a\t\xFAV[a\x01u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xE3V[a\x02xa\n5V[`@\x80Q\x92\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\x86V[a\x02\x9Aa\n]V[`@Q\x90\x15\x15\x81R` \x01a\x01\x86V[a\x02\x9Aa\x02\xB86`\x04a\x1BzV[`\xB9` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xE3a\x02\xDB6`\x04a\x1B\x93V[a\npV[a\x01\xB5a\x0B>V[a\x01\x97a\x02\xF66`\x04a\x1B\x93V[a\x0BJV[a\x01\x97a\x03\t6`\x04a\x1B\x93V[a\x0C\x01V[a\x01\xB5a\x03\x1C6`\x04a\x1B\xB0V[a\x0C\x12V[`\xB8Ta\x01\xB5\x90\x81V[a\x033`\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x86V[a\x03Ma\r\xCEV[a\x03Ua\x0EOV[`@Q\x7F\xA4_G\xFD\xEA\x8A\x1E\xFD\xD9\x02\x9AV\x91\xC7\xF7Y\xC3+|i\x862\xB5cW>\x15V%\xD1i3\x90`\0\x90\xA1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x03\xA0WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x03\xBAWP0;\x15\x80\x15a\x03\xBAWP`\0T`\xFF\x16`\x01\x14[a\x04KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x04\xA9W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x04\xB1a\x0E\xC2V[a\x04\xB9a\x0FYV[a\x04\xC2\x83a\x0BJV[a\x04\xCB\x82a\x0F\xF8V[\x80\x15a\x05.W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[a\x05;a\r\xCEV[a\x05E`\0a\x10\xE5V[V[`\x01`eT\x14a\x05\xB3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Freentrant call (or paused)\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0`e\x81\x90Ua\x05\xC4\x83\x83a\x11\\V[`\xFF\x16\x14a\x06.W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x08`$\x82\x01R\x7F!version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16a\x06_\x83\x83a\x11\x80V[c\xFF\xFF\xFF\xFF\x16\x14a\x06\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7F!destination\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0a\x07\r\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x11\xA3\x92PPPV[`\0\x81\x81R`\xB9` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x07\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7Fdelivered\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0\x81\x81R`\xB9` R`@\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x07\xCDa\x02\xDB\x85\x85a\x11\xAEV[`@Q\x7F\xF7\xE8:\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c\xF7\xE8:\xEE\x90a\x08(\x90\x89\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x1CRV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08k\x91\x90a\x1C\x84V[a\x08\xD1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7F!module\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0a\x08\xDD\x85\x85a\x11\xC7V[\x90P`\0a\x08\xEB\x86\x86a\x11\xD7V[\x90P`\0a\x08\xF9\x87\x87a\x11\xAEV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cV\xD5\xD4u\x84\x84a\t#\x8B\x8Ba\x11\xF0V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tB\x94\x93\x92\x91\x90a\x1C\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\tpW=`\0\x80>=`\0\xFD[PPPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x84c\xFF\xFF\xFF\xFF\x16\x7F\r8\x1C*WJ\xE8\xF0N!=\xB7\xCF\xB4\xDF\x8D\xF7\x12\xCD\xBDB}\x98h\xFF\xEF8\x06`\xCAet`@Q`@Q\x80\x91\x03\x90\xA4`@Q\x85\x90\x7F\x1C\xAE8\xCD\xD3\xD3\x91\x94\x89''%\xA5\xAEb\xA4\xF4\x8B)\x89\xB0\xDA\xE8C\xD3\xC2y\xFE\xE1\x80s\xA9\x90`\0\x90\xA2PP`\x01`eUPPPPPPPV[a\n\x02a\r\xCEV[a\n\na\x12\x0CV[`@Q\x7F\x9E\x87\xFA\xC8\x8F\xF6a\xF0-D\xF9S\x83\xC8\x17\xFE\xCEK\xCE`\n=\xABzT@hx\xB9e\xE7R\x90`\0\x90\xA1V[`\0\x80a\n@a\x0B>V[`\x01a\nK`\xB8T\x90V[a\nU\x91\x90a\x1D\x05V[\x91P\x91P\x90\x91V[`\0a\nk`eT`\x02\x14\x90V[\x90P\x90V[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDER<\xF3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\n\xF7WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\n\xF4\x91\x81\x01\x90a\x1D)V[`\x01[\x15a\x0B Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\x0B\x1EW\x92\x91PPV[P[PP`\x97Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\0a\nk`\x98a\x12\x7FV[a\x0BRa\r\xCEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0B\xF5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[a\x0B\xFE\x81a\x10\xE5V[PV[a\x0C\ta\r\xCEV[a\x0B\xFE\x81a\x0F\xF8V[`\0a\x0C `eT`\x02\x14\x90V[\x15a\x0C\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x06`$\x82\x01R\x7Fpaused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[a\x08\0\x82\x11\x15a\x0C\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7Fmsg too long\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\0a\r.`\0a\r\x03`\xB8T\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x8A\x8A\x8A\x8Aa\x12\x92V[\x80Q` \x82\x01 \x90\x91Pa\rC`\x98\x82a\x12\xD0V[\x85\x87c\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fv\x9Fq\x1D \xC6y\x15=8\"T\xF5\x98\x92a;X\xA9|\xC8v\xB2I\x13J\xC2\\\x80\xF9\xC8\x14\x85`@Qa\r\x91\x91\x90a\x1DFV[`@Q\x80\x91\x03\x90\xA4`@Q\x81\x90\x7Fx\x8D\xBC\x1BqRs!x!\x0E\x7FM\x9D\x01\x0E\xF0\x16\xF9\xEA\xFB\xE6g\x86\xBDqi\xF5n\x0C5:\x90`\0\x90\xA2\x96\x95PPPPPPV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04BV[`eT`\x02\x14a\x0E\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7F!paused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x01`eUV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0E\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[a\x05Ea\x14\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;a\x10vW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7F!contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x97\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xA7j\xD0\xAD\xBFE1\x8F\x863\xAA\x02\x10\xF7\x11'=P\xFB\xB6\xFE\xF7n\xD9[\xBA\xE9p\x82\xC7]\xAA\x90`\0\x90\xA2PV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0a\x11k`\x01\x82\x84\x86a\x1D\xB2V[a\x11t\x91a\x1D\xDCV[`\xF8\x1C\x90P[\x92\x91PPV[`\0a\x11\x90`-`)\x84\x86a\x1D\xB2V[a\x11\x99\x91a\x1E\"V[`\xE0\x1C\x93\x92PPPV[\x80Q` \x90\x91\x01 \x90V[`\0a\x11\xC0a\x11\xBD\x84\x84a\x14\xAFV[\x90V[\x93\x92PPPV[`\0a\x11\x90`\t`\x05\x84\x86a\x1D\xB2V[`\0a\x11\xE7`)`\t\x84\x86a\x1D\xB2V[a\x11\xC0\x91a\x1EhV[6`\0a\x12\0\x83`M\x81\x87a\x1D\xB2V[\x91P\x91P[\x92P\x92\x90PV[`eT`\x02\x03a\x12xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x06`$\x82\x01R\x7Fpaused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x02`eUV[`\0a\x11z\x82a\x12\x8Da\x14\xBFV[a\x19\x80V[``\x88\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a\x12\xB3\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1E\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x98\x97PPPPPPPPV[`\x01a\x12\xDE` `\x02a ZV[a\x12\xE8\x91\x90a fV[\x82` \x01T\x10a\x13TW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fmerkle tree full\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04BV[`\x01\x82` \x01`\0\x82\x82Ta\x13i\x91\x90a yV[\x90\x91UPP` \x82\x01T`\0[` \x81\x10\x15a\x14\x06W\x81`\x01\x16`\x01\x03a\x13\xA5W\x82\x84\x82` \x81\x10a\x13\x9DWa\x13\x9Da \x8CV[\x01UPPPPV[\x83\x81` \x81\x10a\x13\xB7Wa\x13\xB7a \x8CV[\x01T`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x84\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P`\x02\x82a\x13\xF2\x91\x90a \xBBV[\x91P\x80a\x13\xFE\x81a \xF6V[\x91PPa\x13vV[Pa\x05.a!.V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x14\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04BV[a\x05E3a\x10\xE5V[`\0a\x11\xE7`M`-\x84\x86a\x1D\xB2V[a\x14\xC7a\x1ARV[`\0\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` \x82\x01R\x7F\xB4\xC1\x19Q\x95|o\x8Fd,J\xF6\x1C\xD6\xB2F@\xFE\xC6\xDC\x7F\xC6\x07\xEE\x82\x06\xA9\x9E\x92A\r0`@\x82\x01R\x7F!\xDD\xB9\xA3V\x81\\?\xAC\x10&\xB6\xDE\xC5\xDF1$\xAF\xBA\xDBH\\\x9B\xA5\xA3\xE39\x8A\x04\xB7\xBA\x85``\x82\x01R\x7F\xE5\x87i\xB3*\x1B\xEA\xF1\xEA'7ZD\tZ\r\x1F\xB6d\xCE-\xD3X\xE7\xFC\xBF\xB7\x8C&\xA1\x93D`\x80\x82\x01R\x7F\x0E\xB0\x1E\xBF\xC9\xED'P\x0C\xD4\xDF\xC9y'-\x1F\t\x13\xCC\x9FfT\r~\x80\x05\x81\x11\t\xE1\xCF-`\xA0\x82\x01R\x7F\x88|\"\xBD\x87P\xD3@\x16\xAC<f\xB5\xFF\x10-\xAC\xDDs\xF6\xB0\x14\xE7\x10\xB5\x1E\x80\"\xAF\x9A\x19h`\xC0\x82\x01R\x7F\xFF\xD7\x01W\xE4\x80c\xFC3\xC9z\x05\x0F\x7Fd\x023\xBFdl\xC9\x8D\x95$\xC6\xB9+\xCF:\xB5o\x83`\xE0\x82\x01R\x7F\x98g\xCC_\x7F\x19k\x93\xBA\xE1\xE2~c t$E\xD2\x90\xF2&8'I\x8BT\xFE\xC59\xF7V\xAFa\x01\0\x82\x01R\x7F\xCE\xFA\xD4\xE5\x08\xC0\x98\xB9\xA7\xE1\xD8\xFE\xB1\x99U\xFB\x02\xBA\x96uXPxq\ti\xD3D\x0FPT\xE0a\x01 \x82\x01R\x7F\xF9\xDC>\x7F\xE0\x16\xE0P\xEF\xF2`3O\x18\xA5\xD4\xFE9\x1D\x82\t#\x19\xF5\x96O..\xB7\xC1\xC3\xA5a\x01@\x82\x01R\x7F\xF8\xB1:I\xE2\x82\xF6\t\xC3\x17\xA83\xFB\x8D\x97m\x11Q|W\x1D\x12!\xA2e\xD2Z\xF7x\xEC\xF8\x92a\x01`\x82\x01R\x7F4\x90\xC6\xCE\xEBE\n\xEC\xDC\x82\xE2\x82\x93\x03\x1D\x10\xC7\xD7;\xF8^W\xBF\x04\x1A\x976\n\xA2\xC5\xD9\x9Ca\x01\x80\x82\x01R\x7F\xC1\xDF\x82\xD9\xC4\xB8t\x13\xEA\xE2\xEF\x04\x8F\x94\xB4\xD3UL\xEAs\xD9+\x0Fz\xF9n\x02q\xC6\x91\xE2\xBBa\x01\xA0\x82\x01R\x7F\\g\xAD\xD7\xC6\xCA\xF3\x02%j\xDE\xDFz\xB1\x14\xDA\n\xCF\xE8p\xD4I\xA3\xA4\x89\xF7\x81\xD6Y\xE8\xBE\xCCa\x01\xC0\x82\x01R\x7F\xDA{\xCE\x9FN\x86\x18\xB6\xBD/A2\xCEy\x8C\xDCz`\xE7\xE1F\nr\x99\xE3\xC64*W\x96&\xD2a\x01\xE0\x82\x01R\x7F'3\xE5\x0FRn\xC2\xFA\x19\xA2+1\xE8\xEDP\xF2<\xD1\xFD\xF9L\x91T\xED:v\t\xA2\xF1\xFF\x98\x1Fa\x02\0\x82\x01R\x7F\xE1\xD3\xB5\xC8\x07\xB2\x81\xE4h<\xC6\xD61\\\xF9[\x9A\xDE\x86A\xDE\xFC\xB3#r\xF1\xC1&\xE3\x98\xEFza\x02 \x82\x01R\x7FZ-\xCE\n\x8A\x7Fh\xBBtV\x0F\x8Fq\x83|,.\xBB\xCB\xF7\xFF\xFBB\xAE\x18\x96\xF1?|ty\xA0a\x02@\x82\x01R\x7F\xB4j(\xB6\xF5U@\xF8\x94D\xF6=\xE07\x8E=\x12\x1B\xE0\x9E\x06\xCC\x9D\xED\x1C \xE6Xv\xD3j\xA0a\x02`\x82\x01R\x7F\xC6^\x96EdG\x86\xB6 \xE2\xDD*\xD6H\xDD\xFC\xBFJ~[\x1A:N\xCF\xE7\xF6Fg\xA3\xF0\xB7\xE2a\x02\x80\x82\x01R\x7F\xF4A\x85\x88\xED5\xA2E\x8C\xFF\xEB9\xB9=&\xF1\x8D*\xB1;\xDC\xE6\xAE\xE5\x8E{\x995\x9E\xC2\xDF\xD9a\x02\xA0\x82\x01R\x7FZ\x9C\x16\xDC\0\xD6\xEF\x18\xB7\x93:o\x8D\xC6\\\xCBUfq8wo}\xEA\x10\x10p\xDC\x87\x96\xE3wa\x02\xC0\x82\x01R\x7FM\xF8O@\xAE\x0C\x82)\xD0\xD6\x06\x9E\\\x8F9\xA7\xC2\x99gz\t\xD3g\xFC{\x05\xE3\xBC8\x0E\xE6Ra\x02\xE0\x82\x01R\x7F\xCD\xC7%\x95\xF7L{\x10C\xD0\xE1\xFF\xBA\xB74d\x8C\x83\x8D\xFB\x05'\xD9q\xB6\x02\xBC!l\x96\x19\xEFa\x03\0\x82\x01R\x7F\n\xBFZ\xC9t\xA1\xEDW\xF4\x05\n\xA5\x10\xDD\x9Ct\xF5\x08'{9\xD7\x97;\xB2\xDF\xCC\xC5\xEE\xB0a\x8Da\x03 \x82\x01R\x7F\xB8\xCDt\x04o\xF37\xF0\xA7\xBF,\x8E\x03\xE1\x0Fd,\x18\x86y\x8Dq\x80j\xB1\xE8\x88\xD9\xE5\xEE\x87\xD0a\x03@\x82\x01R\x7F\x83\x8CVU\xCB!\xC6\xCB\x831;Zc\x11u\xDF\xF4\x967r\xCC\xE9\x10\x81\x88\xB3J\xC8|\x81\xC4\x1Ea\x03`\x82\x01R\x7Ff.\xE4\xDD-\xD7\xB2\xBCpya\xB1\xE6F\xC4\x04vi\xDC\xB6XO\r\x8Dw\r\xAF]~}\xEB.a\x03\x80\x82\x01R\x7F8\x8A\xB2\x0E%s\xD1q\xA8\x81\x08\xE7\x9D\x82\x0E\x98\xF2l\x0B\x84\xAA\x8B/J\xA4\x96\x8D\xBB\x81\x8E\xA3\"a\x03\xA0\x82\x01R\x7F\x93#|P\xBAu\xEEH_L\"\xAD\xF2\xF7A@\x0B\xDF\x8Dj\x9C\xC7\xDF~\xCA\xE5v\"\x16e\xD75a\x03\xC0\x82\x01R\x7F\x84H\x81\x8B\xB4\xAEEb\x84\x9E\x94\x9E\x17\xAC\x16\xE0\xBE\x16h\x8E\x15k\\\xF1^\t\x8Cb|\0V\xA9a\x03\xE0\x82\x01R\x90V[` \x82\x01T`\0\x90\x81[` \x81\x10\x15a\x1AJW`\x01\x82\x82\x1C\x16`\0\x86\x83` \x81\x10a\x19\xADWa\x19\xADa \x8CV[\x01T\x90P\x81`\x01\x03a\x19\xEAW`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x86\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94Pa\x1A5V[\x84\x86\x84` \x81\x10a\x19\xFDWa\x19\xFDa \x8CV[` \x02\x01Q`@Q` \x01a\x1A\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94P[PP\x80\x80a\x1AB\x90a \xF6V[\x91PPa\x19\x8AV[PP\x92\x91PPV[`@Q\x80a\x04\0\x01`@R\x80` \x90` \x82\x02\x806\x837P\x91\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\xFEW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xA6W`\0\x80\xFD[\x825a\x1A\xB1\x81a\x1AqV[\x91P` \x83\x015a\x1A\xC1\x81a\x1AqV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1A\xDEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xF6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x12\x05W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1B$W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B<W`\0\x80\xFD[a\x1BH\x88\x83\x89\x01a\x1A\xCCV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1BaW`\0\x80\xFD[Pa\x1Bn\x87\x82\x88\x01a\x1A\xCCV[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\x1B\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1B\xA5W`\0\x80\xFD[\x815a\x11\xC0\x81a\x1AqV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1B\xC6W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1B\xDAW`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xFDW`\0\x80\xFD[a\x1Bn\x87\x82\x88\x01a\x1A\xCCV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`@\x81R`\0a\x1Cf`@\x83\x01\x86\x88a\x1C\tV[\x82\x81\x03` \x84\x01Ra\x1Cy\x81\x85\x87a\x1C\tV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1C\x96W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\xC0W`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x1C\xCC``\x83\x01\x84\x86a\x1C\tV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x1D\"Wa\x1D\"a\x1C\xD6V[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D;W`\0\x80\xFD[\x81Qa\x11\xC0\x81a\x1AqV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x1DsW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1DWV[P`\0`@\x82\x86\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80\x85\x85\x11\x15a\x1D\xC2W`\0\x80\xFD[\x83\x86\x11\x15a\x1D\xCFW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x815\x81\x81\x16\x91`\x01\x85\x10\x15a\x1AJW`\x01\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x815\x81\x81\x16\x91`\x04\x85\x10\x15a\x1AJW`\x04\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV[\x805` \x83\x10\x15a\x11zW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89`\xF8\x1B\x16\x81R`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x8A`\xE0\x1B\x16`\x01\x84\x01R\x80\x89`\xE0\x1B\x16`\x05\x84\x01R\x87`\t\x84\x01R\x80\x87`\xE0\x1B\x16`)\x84\x01RP\x84`-\x83\x01R\x82\x84`M\x84\x017P`\0\x91\x01`M\x01\x90\x81R\x97\x96PPPPPPPV[`\x01\x81\x81[\x80\x85\x11\x15a\x1F\x93W\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x1FyWa\x1Fya\x1C\xD6V[\x80\x85\x16\x15a\x1F\x86W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1F?V[P\x92P\x92\x90PV[`\0\x82a\x1F\xAAWP`\x01a\x11zV[\x81a\x1F\xB7WP`\0a\x11zV[\x81`\x01\x81\x14a\x1F\xCDW`\x02\x81\x14a\x1F\xD7Wa\x1F\xF3V[`\x01\x91PPa\x11zV[`\xFF\x84\x11\x15a\x1F\xE8Wa\x1F\xE8a\x1C\xD6V[PP`\x01\x82\x1Ba\x11zV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a \x16WP\x81\x81\na\x11zV[a  \x83\x83a\x1F:V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a RWa Ra\x1C\xD6V[\x02\x93\x92PPPV[`\0a\x11\xC0\x83\x83a\x1F\x9BV[\x81\x81\x03\x81\x81\x11\x15a\x11zWa\x11za\x1C\xD6V[\x80\x82\x01\x80\x82\x11\x15a\x11zWa\x11za\x1C\xD6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82a \xF1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a!'Wa!'a\x1C\xD6V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA7\xCAy\xCAr\x03U\xC58\x19\t\x7F\x91'\0J\xE2\xAB]\x8C\x13\x1EC\xC6\x03\xA28e\x84\x9FP\xCAdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static MAILBOX_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Mailbox<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Mailbox<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Mailbox<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Mailbox<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Mailbox<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Mailbox)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Mailbox<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MAILBOX_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MAILBOX_ABI.clone(),
                MAILBOX_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_MESSAGE_BODY_BYTES` (0x522ae002) function
        pub fn max_message_body_bytes(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 42, 224, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `count` (0x06661abd) function
        pub fn count(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([6, 102, 26, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultIsm` (0x6e5f516e) function
        pub fn default_ism(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([110, 95, 81, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delivered` (0xe495f1d4) function
        pub fn delivered(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 149, 241, 212], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dispatch` (0xfa31de01) function
        pub fn dispatch(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [250, 49, 222, 1],
                    (destination_domain, recipient_address, message_body),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
            default_ism: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (owner, default_ism))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPaused` (0xb187bd26) function
        pub fn is_paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCheckpoint` (0x907c0f92) function
        pub fn latest_checkpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], u32)> {
            self.0
                .method_hash([144, 124, 15, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `localDomain` (0x8d3638f4) function
        pub fn local_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([141, 54, 56, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process` (0x7c39d130) function
        pub fn process(
            &self,
            metadata: ::ethers::core::types::Bytes,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 57, 209, 48], (metadata, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recipientIsm` (0xe70f48ac) function
        pub fn recipient_ism(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 15, 72, 172], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `root` (0xebf0c717) function
        pub fn root(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 240, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDefaultIsm` (0xf794687a) function
        pub fn set_default_ism(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 148, 104, 122], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tree` (0xfd54b228) function
        pub fn tree(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 84, 178, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DefaultIsmSet` event
        pub fn default_ism_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultIsmSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Dispatch` event
        pub fn dispatch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DispatchFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DispatchId` event
        pub fn dispatch_id_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DispatchIdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Process` event
        pub fn process_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProcessFilter> {
            self.0.event()
        }
        ///Gets the contract's `ProcessId` event
        pub fn process_id_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessIdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MailboxEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Mailbox<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DefaultIsmSet", abi = "DefaultIsmSet(address)")]
    pub struct DefaultIsmSetFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Dispatch", abi = "Dispatch(address,uint32,bytes32,bytes)")]
    pub struct DispatchFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: u32,
        #[ethevent(indexed)]
        pub recipient: [u8; 32],
        pub message: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DispatchId", abi = "DispatchId(bytes32)")]
    pub struct DispatchIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Paused", abi = "Paused()")]
    pub struct PausedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Process", abi = "Process(uint32,bytes32,address)")]
    pub struct ProcessFilter {
        #[ethevent(indexed)]
        pub origin: u32,
        #[ethevent(indexed)]
        pub sender: [u8; 32],
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ProcessId", abi = "ProcessId(bytes32)")]
    pub struct ProcessIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused()")]
    pub struct UnpausedFilter;
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MailboxEvents {
        DefaultIsmSetFilter(DefaultIsmSetFilter),
        DispatchFilter(DispatchFilter),
        DispatchIdFilter(DispatchIdFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        ProcessFilter(ProcessFilter),
        ProcessIdFilter(ProcessIdFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MailboxEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DefaultIsmSetFilter::decode_log(log) {
                return Ok(MailboxEvents::DefaultIsmSetFilter(decoded));
            }
            if let Ok(decoded) = DispatchFilter::decode_log(log) {
                return Ok(MailboxEvents::DispatchFilter(decoded));
            }
            if let Ok(decoded) = DispatchIdFilter::decode_log(log) {
                return Ok(MailboxEvents::DispatchIdFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MailboxEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MailboxEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(MailboxEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = ProcessFilter::decode_log(log) {
                return Ok(MailboxEvents::ProcessFilter(decoded));
            }
            if let Ok(decoded) = ProcessIdFilter::decode_log(log) {
                return Ok(MailboxEvents::ProcessIdFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(MailboxEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MailboxEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultIsmSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DispatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DispatchIdFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessIdFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultIsmSetFilter> for MailboxEvents {
        fn from(value: DefaultIsmSetFilter) -> Self {
            Self::DefaultIsmSetFilter(value)
        }
    }
    impl ::core::convert::From<DispatchFilter> for MailboxEvents {
        fn from(value: DispatchFilter) -> Self {
            Self::DispatchFilter(value)
        }
    }
    impl ::core::convert::From<DispatchIdFilter> for MailboxEvents {
        fn from(value: DispatchIdFilter) -> Self {
            Self::DispatchIdFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for MailboxEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MailboxEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for MailboxEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<ProcessFilter> for MailboxEvents {
        fn from(value: ProcessFilter) -> Self {
            Self::ProcessFilter(value)
        }
    }
    impl ::core::convert::From<ProcessIdFilter> for MailboxEvents {
        fn from(value: ProcessIdFilter) -> Self {
            Self::ProcessIdFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for MailboxEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_MESSAGE_BODY_BYTES` function with signature `MAX_MESSAGE_BODY_BYTES()` and selector `0x522ae002`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MAX_MESSAGE_BODY_BYTES", abi = "MAX_MESSAGE_BODY_BYTES()")]
    pub struct MaxMessageBodyBytesCall;
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `count` function with signature `count()` and selector `0x06661abd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "count", abi = "count()")]
    pub struct CountCall;
    ///Container type for all input parameters for the `defaultIsm` function with signature `defaultIsm()` and selector `0x6e5f516e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "defaultIsm", abi = "defaultIsm()")]
    pub struct DefaultIsmCall;
    ///Container type for all input parameters for the `delivered` function with signature `delivered(bytes32)` and selector `0xe495f1d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "delivered", abi = "delivered(bytes32)")]
    pub struct DeliveredCall(pub [u8; 32]);
    ///Container type for all input parameters for the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `0xfa31de01`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "dispatch", abi = "dispatch(uint32,bytes32,bytes)")]
    pub struct DispatchCall {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub owner: ::ethers::core::types::Address,
        pub default_ism: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isPaused", abi = "isPaused()")]
    pub struct IsPausedCall;
    ///Container type for all input parameters for the `latestCheckpoint` function with signature `latestCheckpoint()` and selector `0x907c0f92`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "latestCheckpoint", abi = "latestCheckpoint()")]
    pub struct LatestCheckpointCall;
    ///Container type for all input parameters for the `localDomain` function with signature `localDomain()` and selector `0x8d3638f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "localDomain", abi = "localDomain()")]
    pub struct LocalDomainCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `process` function with signature `process(bytes,bytes)` and selector `0x7c39d130`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "process", abi = "process(bytes,bytes)")]
    pub struct ProcessCall {
        pub metadata: ::ethers::core::types::Bytes,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `recipientIsm` function with signature `recipientIsm(address)` and selector `0xe70f48ac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recipientIsm", abi = "recipientIsm(address)")]
    pub struct RecipientIsmCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `root` function with signature `root()` and selector `0xebf0c717`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "root", abi = "root()")]
    pub struct RootCall;
    ///Container type for all input parameters for the `setDefaultIsm` function with signature `setDefaultIsm(address)` and selector `0xf794687a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setDefaultIsm", abi = "setDefaultIsm(address)")]
    pub struct SetDefaultIsmCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `tree` function with signature `tree()` and selector `0xfd54b228`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tree", abi = "tree()")]
    pub struct TreeCall;
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MailboxCalls {
        MaxMessageBodyBytes(MaxMessageBodyBytesCall),
        Version(VersionCall),
        Count(CountCall),
        DefaultIsm(DefaultIsmCall),
        Delivered(DeliveredCall),
        Dispatch(DispatchCall),
        Initialize(InitializeCall),
        IsPaused(IsPausedCall),
        LatestCheckpoint(LatestCheckpointCall),
        LocalDomain(LocalDomainCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Process(ProcessCall),
        RecipientIsm(RecipientIsmCall),
        RenounceOwnership(RenounceOwnershipCall),
        Root(RootCall),
        SetDefaultIsm(SetDefaultIsmCall),
        TransferOwnership(TransferOwnershipCall),
        Tree(TreeCall),
        Unpause(UnpauseCall),
    }
    impl ::ethers::core::abi::AbiDecode for MailboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <MaxMessageBodyBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxMessageBodyBytes(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <CountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Count(decoded));
            }
            if let Ok(decoded)
                = <DefaultIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultIsm(decoded));
            }
            if let Ok(decoded)
                = <DeliveredCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delivered(decoded));
            }
            if let Ok(decoded)
                = <DispatchCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dispatch(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <IsPausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPaused(decoded));
            }
            if let Ok(decoded)
                = <LatestCheckpointCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LatestCheckpoint(decoded));
            }
            if let Ok(decoded)
                = <LocalDomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LocalDomain(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded)
                = <ProcessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Process(decoded));
            }
            if let Ok(decoded)
                = <RecipientIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecipientIsm(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <RootCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Root(decoded));
            }
            if let Ok(decoded)
                = <SetDefaultIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDefaultIsm(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <TreeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Tree(decoded));
            }
            if let Ok(decoded)
                = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MailboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxMessageBodyBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Count(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delivered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dispatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCheckpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LocalDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Process(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecipientIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Root(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDefaultIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Tree(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MailboxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxMessageBodyBytes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Count(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultIsm(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delivered(element) => ::core::fmt::Display::fmt(element, f),
                Self::Dispatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCheckpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::LocalDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Process(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecipientIsm(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Root(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultIsm(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tree(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxMessageBodyBytesCall> for MailboxCalls {
        fn from(value: MaxMessageBodyBytesCall) -> Self {
            Self::MaxMessageBodyBytes(value)
        }
    }
    impl ::core::convert::From<VersionCall> for MailboxCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<CountCall> for MailboxCalls {
        fn from(value: CountCall) -> Self {
            Self::Count(value)
        }
    }
    impl ::core::convert::From<DefaultIsmCall> for MailboxCalls {
        fn from(value: DefaultIsmCall) -> Self {
            Self::DefaultIsm(value)
        }
    }
    impl ::core::convert::From<DeliveredCall> for MailboxCalls {
        fn from(value: DeliveredCall) -> Self {
            Self::Delivered(value)
        }
    }
    impl ::core::convert::From<DispatchCall> for MailboxCalls {
        fn from(value: DispatchCall) -> Self {
            Self::Dispatch(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MailboxCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsPausedCall> for MailboxCalls {
        fn from(value: IsPausedCall) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<LatestCheckpointCall> for MailboxCalls {
        fn from(value: LatestCheckpointCall) -> Self {
            Self::LatestCheckpoint(value)
        }
    }
    impl ::core::convert::From<LocalDomainCall> for MailboxCalls {
        fn from(value: LocalDomainCall) -> Self {
            Self::LocalDomain(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MailboxCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for MailboxCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<ProcessCall> for MailboxCalls {
        fn from(value: ProcessCall) -> Self {
            Self::Process(value)
        }
    }
    impl ::core::convert::From<RecipientIsmCall> for MailboxCalls {
        fn from(value: RecipientIsmCall) -> Self {
            Self::RecipientIsm(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MailboxCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RootCall> for MailboxCalls {
        fn from(value: RootCall) -> Self {
            Self::Root(value)
        }
    }
    impl ::core::convert::From<SetDefaultIsmCall> for MailboxCalls {
        fn from(value: SetDefaultIsmCall) -> Self {
            Self::SetDefaultIsm(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MailboxCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TreeCall> for MailboxCalls {
        fn from(value: TreeCall) -> Self {
            Self::Tree(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for MailboxCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    ///Container type for all return fields from the `MAX_MESSAGE_BODY_BYTES` function with signature `MAX_MESSAGE_BODY_BYTES()` and selector `0x522ae002`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxMessageBodyBytesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VersionReturn(pub u8);
    ///Container type for all return fields from the `count` function with signature `count()` and selector `0x06661abd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CountReturn(pub u32);
    ///Container type for all return fields from the `defaultIsm` function with signature `defaultIsm()` and selector `0x6e5f516e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultIsmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delivered` function with signature `delivered(bytes32)` and selector `0xe495f1d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeliveredReturn(pub bool);
    ///Container type for all return fields from the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `0xfa31de01`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DispatchReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsPausedReturn(pub bool);
    ///Container type for all return fields from the `latestCheckpoint` function with signature `latestCheckpoint()` and selector `0x907c0f92`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LatestCheckpointReturn(pub [u8; 32], pub u32);
    ///Container type for all return fields from the `localDomain` function with signature `localDomain()` and selector `0x8d3638f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LocalDomainReturn(pub u32);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `recipientIsm` function with signature `recipientIsm(address)` and selector `0xe70f48ac`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RecipientIsmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `root` function with signature `root()` and selector `0xebf0c717`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `tree` function with signature `tree()` and selector `0xfd54b228`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TreeReturn {
        pub count: ::ethers::core::types::U256,
    }
}
