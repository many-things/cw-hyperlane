pub use test_recipient::*;
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
pub mod test_recipient {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("fooBar"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fooBar"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("handle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("handle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
                    ::std::borrow::ToOwned::to_owned("interchainSecurityModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "interchainSecurityModule",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("lastCallMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastCallMessage"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastCaller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastCaller"),
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
                    ::std::borrow::ToOwned::to_owned("lastData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastSender"),
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
                    ::std::borrow::ToOwned::to_owned("setInterchainSecurityModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setInterchainSecurityModule",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ism"),
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("ReceivedCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReceivedCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReceivedMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReceivedMessage"),
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
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
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
    pub static TESTRECIPIENT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0oV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\t\xD3\x80a\0~`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xC8W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x81W\x80c\xDER<\xF3\x11a\0[W\x80c\xDER<\xF3\x14a\x01\x9DW\x80c\xF0|\x1FG\x14a\x01\xBDW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD0W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01oW\x80c\x8D\xA5\xCB[\x14a\x01wW\x80c\xA4\x98/\xDE\x14a\x01\x95W`\0\x80\xFD[\x80c!\x13R*\x11a\0\xB2W\x80c!\x13R*\x14a\x01\0W\x80c%o\xEC\x88\x14a\x01EW\x80cV\xD5\xD4u\x14a\x01\\W`\0\x80\xFD[\x80bnu\xEC\x14a\0\xCDW\x80c\x0Er\xCC\x06\x14a\0\xEBW[`\0\x80\xFD[a\0\xD5a\x01\xE3V[`@Qa\0\xE2\x91\x90a\x05\xE0V[`@Q\x80\x91\x03\x90\xF3[a\0\xFEa\0\xF96`\x04a\x05\xFAV[a\x02qV[\0[`\x04Ta\x01 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE2V[a\x01N`\x02T\x81V[`@Q\x90\x81R` \x01a\0\xE2V[a\0\xFEa\x01j6`\x04a\x06yV[a\x02\xC0V[a\0\xFEa\x03\x1AV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 V[a\0\xD5a\x03.V[`\x01Ta\x01 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\0\xFEa\x01\xCB6`\x04a\x06\xDEV[a\x03;V[a\0\xFEa\x01\xDE6`\x04a\x05\xFAV[a\x03\xCAV[`\x03\x80Ta\x01\xF0\x90a\x07*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x1C\x90a\x07*V[\x80\x15a\x02iW\x80`\x1F\x10a\x02>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02iV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02LW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\x02ya\x04\x86V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82\x84c\xFF\xFF\xFF\xFF\x16\x7F\xBAgtL\x89\x91\x13\xA8Oa]Q\xAF]\x82\xF5\xFE\xDC\xF2l\x9AGM\x93c\xC3\xAD\x9B\x0B\xD5\x01\xAC\x84\x84`@Qa\x02\xF9\x92\x91\x90a\x07\xC6V[`@Q\x80\x91\x03\x90\xA3`\x02\x83\x90U`\x03a\x03\x13\x82\x84\x83a\x08`V[PPPPPV[a\x03\"a\x04\x86V[a\x03,`\0a\x05\x07V[V[`\x05\x80Ta\x01\xF0\x90a\x07*V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97\xD86z\x1F9\xEB\x9E\x97\xF2b\xFA\xFB\xB0Y%\xC0\xBC\xFE\x12\n\xAA\xD7\xB9s|\xAE4\xF7I\xC2\x06\x84\x84\x84`@Qa\x03\x85\x93\x92\x91\x90a\tzV[`@Q\x80\x91\x03\x90\xA2`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90U`\x05a\x03\xC4\x82\x84\x83a\x08`V[PPPPV[a\x03\xD2a\x04\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x04zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x04\x83\x81a\x05\x07V[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04qV[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x05\xA2W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05\x86V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x05\xF3` \x83\x01\x84a\x05|V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x0CW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xF3W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x06BW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06ZW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x06rW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06\x8FW`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xA3W`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC6W`\0\x80\xFD[a\x06\xD2\x87\x82\x88\x01a\x060V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x06\xF3W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x11W`\0\x80\xFD[a\x07\x1D\x86\x82\x87\x01a\x060V[\x94\x97\x90\x96P\x93\x94PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x07>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07wW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R`\0a\x07\xDA` \x83\x01\x84\x86a\x07}V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x08[W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x088WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08WW\x82\x81U`\x01\x01a\x08DV[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x08xWa\x08xa\x07\xE2V[a\x08\x8C\x83a\x08\x86\x83Ta\x07*V[\x83a\x08\x11V[`\0`\x1F\x84\x11`\x01\x81\x14a\x08\xDEW`\0\x85\x15a\x08\xA8WP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x03\x13V[`\0\x83\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x90\x83[\x82\x81\x10\x15a\t-W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\t\rV[P\x86\x82\x10\x15a\thW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x83\x81R`@` \x82\x01R`\0a\t\x94`@\x83\x01\x84\x86a\x07}V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x97\x16\x99)+\xB9\x04\xF6\x08[\x8D\xAC\xB8\xDC\xD7/-w|lF`\xBF\xD6\\c\xCC\xFC\xBAt\xB5\xF7dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static TESTRECIPIENT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xC8W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x81W\x80c\xDER<\xF3\x11a\0[W\x80c\xDER<\xF3\x14a\x01\x9DW\x80c\xF0|\x1FG\x14a\x01\xBDW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD0W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01oW\x80c\x8D\xA5\xCB[\x14a\x01wW\x80c\xA4\x98/\xDE\x14a\x01\x95W`\0\x80\xFD[\x80c!\x13R*\x11a\0\xB2W\x80c!\x13R*\x14a\x01\0W\x80c%o\xEC\x88\x14a\x01EW\x80cV\xD5\xD4u\x14a\x01\\W`\0\x80\xFD[\x80bnu\xEC\x14a\0\xCDW\x80c\x0Er\xCC\x06\x14a\0\xEBW[`\0\x80\xFD[a\0\xD5a\x01\xE3V[`@Qa\0\xE2\x91\x90a\x05\xE0V[`@Q\x80\x91\x03\x90\xF3[a\0\xFEa\0\xF96`\x04a\x05\xFAV[a\x02qV[\0[`\x04Ta\x01 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE2V[a\x01N`\x02T\x81V[`@Q\x90\x81R` \x01a\0\xE2V[a\0\xFEa\x01j6`\x04a\x06yV[a\x02\xC0V[a\0\xFEa\x03\x1AV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 V[a\0\xD5a\x03.V[`\x01Ta\x01 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\0\xFEa\x01\xCB6`\x04a\x06\xDEV[a\x03;V[a\0\xFEa\x01\xDE6`\x04a\x05\xFAV[a\x03\xCAV[`\x03\x80Ta\x01\xF0\x90a\x07*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x1C\x90a\x07*V[\x80\x15a\x02iW\x80`\x1F\x10a\x02>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02iV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02LW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\x02ya\x04\x86V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82\x84c\xFF\xFF\xFF\xFF\x16\x7F\xBAgtL\x89\x91\x13\xA8Oa]Q\xAF]\x82\xF5\xFE\xDC\xF2l\x9AGM\x93c\xC3\xAD\x9B\x0B\xD5\x01\xAC\x84\x84`@Qa\x02\xF9\x92\x91\x90a\x07\xC6V[`@Q\x80\x91\x03\x90\xA3`\x02\x83\x90U`\x03a\x03\x13\x82\x84\x83a\x08`V[PPPPPV[a\x03\"a\x04\x86V[a\x03,`\0a\x05\x07V[V[`\x05\x80Ta\x01\xF0\x90a\x07*V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97\xD86z\x1F9\xEB\x9E\x97\xF2b\xFA\xFB\xB0Y%\xC0\xBC\xFE\x12\n\xAA\xD7\xB9s|\xAE4\xF7I\xC2\x06\x84\x84\x84`@Qa\x03\x85\x93\x92\x91\x90a\tzV[`@Q\x80\x91\x03\x90\xA2`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90U`\x05a\x03\xC4\x82\x84\x83a\x08`V[PPPPV[a\x03\xD2a\x04\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x04zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x04\x83\x81a\x05\x07V[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04qV[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x05\xA2W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05\x86V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x05\xF3` \x83\x01\x84a\x05|V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x0CW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xF3W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x06BW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06ZW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x06rW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06\x8FW`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xA3W`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC6W`\0\x80\xFD[a\x06\xD2\x87\x82\x88\x01a\x060V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x06\xF3W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x11W`\0\x80\xFD[a\x07\x1D\x86\x82\x87\x01a\x060V[\x94\x97\x90\x96P\x93\x94PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x07>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07wW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R`\0a\x07\xDA` \x83\x01\x84\x86a\x07}V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x08[W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x088WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08WW\x82\x81U`\x01\x01a\x08DV[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x08xWa\x08xa\x07\xE2V[a\x08\x8C\x83a\x08\x86\x83Ta\x07*V[\x83a\x08\x11V[`\0`\x1F\x84\x11`\x01\x81\x14a\x08\xDEW`\0\x85\x15a\x08\xA8WP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x03\x13V[`\0\x83\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x90\x83[\x82\x81\x10\x15a\t-W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\t\rV[P\x86\x82\x10\x15a\thW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x83\x81R`@` \x82\x01R`\0a\t\x94`@\x83\x01\x84\x86a\x07}V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x97\x16\x99)+\xB9\x04\xF6\x08[\x8D\xAC\xB8\xDC\xD7/-w|lF`\xBF\xD6\\c\xCC\xFC\xBAt\xB5\xF7dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static TESTRECIPIENT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TestRecipient<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TestRecipient<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TestRecipient<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TestRecipient<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TestRecipient<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TestRecipient))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TestRecipient<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TESTRECIPIENT_ABI.clone(),
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
                TESTRECIPIENT_ABI.clone(),
                TESTRECIPIENT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `fooBar` (0xf07c1f47) function
        pub fn foo_bar(
            &self,
            amount: ::ethers::core::types::U256,
            message: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 124, 31, 71], (amount, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handle` (0x56d5d475) function
        pub fn handle(
            &self,
            origin: u32,
            sender: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 213, 212, 117], (origin, sender, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `interchainSecurityModule` (0xde523cf3) function
        pub fn interchain_security_module(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([222, 82, 60, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCallMessage` (0xa4982fde) function
        pub fn last_call_message(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([164, 152, 47, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCaller` (0x2113522a) function
        pub fn last_caller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([33, 19, 82, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastData` (0x006e75ec) function
        pub fn last_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([0, 110, 117, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastSender` (0x256fec88) function
        pub fn last_sender(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([37, 111, 236, 136], ())
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInterchainSecurityModule` (0x0e72cc06) function
        pub fn set_interchain_security_module(
            &self,
            ism: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 114, 204, 6], ism)
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
        ///Gets the contract's `ReceivedCall` event
        pub fn received_call_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReceivedCallFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReceivedMessage` event
        pub fn received_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReceivedMessageFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TestRecipientEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TestRecipient<M> {
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
    #[ethevent(name = "ReceivedCall", abi = "ReceivedCall(address,uint256,string)")]
    pub struct ReceivedCallFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub message: ::std::string::String,
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
    #[ethevent(name = "ReceivedMessage", abi = "ReceivedMessage(uint32,bytes32,string)")]
    pub struct ReceivedMessageFilter {
        #[ethevent(indexed)]
        pub origin: u32,
        #[ethevent(indexed)]
        pub sender: [u8; 32],
        pub message: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TestRecipientEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ReceivedCallFilter(ReceivedCallFilter),
        ReceivedMessageFilter(ReceivedMessageFilter),
    }
    impl ::ethers::contract::EthLogDecode for TestRecipientEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(TestRecipientEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ReceivedCallFilter::decode_log(log) {
                return Ok(TestRecipientEvents::ReceivedCallFilter(decoded));
            }
            if let Ok(decoded) = ReceivedMessageFilter::decode_log(log) {
                return Ok(TestRecipientEvents::ReceivedMessageFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TestRecipientEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReceivedCallFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReceivedMessageFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for TestRecipientEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ReceivedCallFilter> for TestRecipientEvents {
        fn from(value: ReceivedCallFilter) -> Self {
            Self::ReceivedCallFilter(value)
        }
    }
    impl ::core::convert::From<ReceivedMessageFilter> for TestRecipientEvents {
        fn from(value: ReceivedMessageFilter) -> Self {
            Self::ReceivedMessageFilter(value)
        }
    }
    ///Container type for all input parameters for the `fooBar` function with signature `fooBar(uint256,string)` and selector `0xf07c1f47`
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
    #[ethcall(name = "fooBar", abi = "fooBar(uint256,string)")]
    pub struct FooBarCall {
        pub amount: ::ethers::core::types::U256,
        pub message: ::std::string::String,
    }
    ///Container type for all input parameters for the `handle` function with signature `handle(uint32,bytes32,bytes)` and selector `0x56d5d475`
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
    #[ethcall(name = "handle", abi = "handle(uint32,bytes32,bytes)")]
    pub struct HandleCall {
        pub origin: u32,
        pub sender: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `interchainSecurityModule` function with signature `interchainSecurityModule()` and selector `0xde523cf3`
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
    #[ethcall(name = "interchainSecurityModule", abi = "interchainSecurityModule()")]
    pub struct InterchainSecurityModuleCall;
    ///Container type for all input parameters for the `lastCallMessage` function with signature `lastCallMessage()` and selector `0xa4982fde`
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
    #[ethcall(name = "lastCallMessage", abi = "lastCallMessage()")]
    pub struct LastCallMessageCall;
    ///Container type for all input parameters for the `lastCaller` function with signature `lastCaller()` and selector `0x2113522a`
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
    #[ethcall(name = "lastCaller", abi = "lastCaller()")]
    pub struct LastCallerCall;
    ///Container type for all input parameters for the `lastData` function with signature `lastData()` and selector `0x006e75ec`
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
    #[ethcall(name = "lastData", abi = "lastData()")]
    pub struct LastDataCall;
    ///Container type for all input parameters for the `lastSender` function with signature `lastSender()` and selector `0x256fec88`
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
    #[ethcall(name = "lastSender", abi = "lastSender()")]
    pub struct LastSenderCall;
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
    ///Container type for all input parameters for the `setInterchainSecurityModule` function with signature `setInterchainSecurityModule(address)` and selector `0x0e72cc06`
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
    #[ethcall(
        name = "setInterchainSecurityModule",
        abi = "setInterchainSecurityModule(address)"
    )]
    pub struct SetInterchainSecurityModuleCall {
        pub ism: ::ethers::core::types::Address,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TestRecipientCalls {
        FooBar(FooBarCall),
        Handle(HandleCall),
        InterchainSecurityModule(InterchainSecurityModuleCall),
        LastCallMessage(LastCallMessageCall),
        LastCaller(LastCallerCall),
        LastData(LastDataCall),
        LastSender(LastSenderCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetInterchainSecurityModule(SetInterchainSecurityModuleCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for TestRecipientCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <FooBarCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FooBar(decoded));
            }
            if let Ok(decoded)
                = <HandleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Handle(decoded));
            }
            if let Ok(decoded)
                = <InterchainSecurityModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InterchainSecurityModule(decoded));
            }
            if let Ok(decoded)
                = <LastCallMessageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastCallMessage(decoded));
            }
            if let Ok(decoded)
                = <LastCallerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastCaller(decoded));
            }
            if let Ok(decoded)
                = <LastDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastData(decoded));
            }
            if let Ok(decoded)
                = <LastSenderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastSender(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SetInterchainSecurityModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetInterchainSecurityModule(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TestRecipientCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FooBar(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Handle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InterchainSecurityModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCallMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetInterchainSecurityModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TestRecipientCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FooBar(element) => ::core::fmt::Display::fmt(element, f),
                Self::Handle(element) => ::core::fmt::Display::fmt(element, f),
                Self::InterchainSecurityModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCallMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInterchainSecurityModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FooBarCall> for TestRecipientCalls {
        fn from(value: FooBarCall) -> Self {
            Self::FooBar(value)
        }
    }
    impl ::core::convert::From<HandleCall> for TestRecipientCalls {
        fn from(value: HandleCall) -> Self {
            Self::Handle(value)
        }
    }
    impl ::core::convert::From<InterchainSecurityModuleCall> for TestRecipientCalls {
        fn from(value: InterchainSecurityModuleCall) -> Self {
            Self::InterchainSecurityModule(value)
        }
    }
    impl ::core::convert::From<LastCallMessageCall> for TestRecipientCalls {
        fn from(value: LastCallMessageCall) -> Self {
            Self::LastCallMessage(value)
        }
    }
    impl ::core::convert::From<LastCallerCall> for TestRecipientCalls {
        fn from(value: LastCallerCall) -> Self {
            Self::LastCaller(value)
        }
    }
    impl ::core::convert::From<LastDataCall> for TestRecipientCalls {
        fn from(value: LastDataCall) -> Self {
            Self::LastData(value)
        }
    }
    impl ::core::convert::From<LastSenderCall> for TestRecipientCalls {
        fn from(value: LastSenderCall) -> Self {
            Self::LastSender(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for TestRecipientCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for TestRecipientCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetInterchainSecurityModuleCall> for TestRecipientCalls {
        fn from(value: SetInterchainSecurityModuleCall) -> Self {
            Self::SetInterchainSecurityModule(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for TestRecipientCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `interchainSecurityModule` function with signature `interchainSecurityModule()` and selector `0xde523cf3`
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
    pub struct InterchainSecurityModuleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `lastCallMessage` function with signature `lastCallMessage()` and selector `0xa4982fde`
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
    pub struct LastCallMessageReturn(pub ::std::string::String);
    ///Container type for all return fields from the `lastCaller` function with signature `lastCaller()` and selector `0x2113522a`
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
    pub struct LastCallerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `lastData` function with signature `lastData()` and selector `0x006e75ec`
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
    pub struct LastDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `lastSender` function with signature `lastSender()` and selector `0x256fec88`
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
    pub struct LastSenderReturn(pub [u8; 32]);
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
}
