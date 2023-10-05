pub use test_multisig_ism::*;
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
pub mod test_multisig_ism {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accept"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accept"),
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
                    ::std::borrow::ToOwned::to_owned("moduleType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("moduleType"),
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
                    ::std::borrow::ToOwned::to_owned("setAccept"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAccept"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("validatorsAndThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validatorsAndThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TESTMULTISIGISM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\xFF\x19\x16`\x01\x17\x90Ua\x03w\x80a\0-`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80cO\xC3\xAA|\x11a\0PW\x80cO\xC3\xAA|\x14a\0\xAFW\x80cde\xE6\x9F\x14a\0\xF0W\x80c\xF7\xE8:\xEE\x14a\x01\nW`\0\x80\xFD[\x80c(R\xB7\x1C\x14a\0lW\x80c.\x0E\xD24\x14a\0\x8EW[`\0\x80\xFD[`\0Ta\0y\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA1a\0\x9C6`\x04a\x01\xD7V[a\x01&V[`@Qa\0\x85\x92\x91\x90a\x02\x19V[a\0\xEEa\0\xBD6`\x04a\x02}V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[a\0\xF8`\x04\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\x85V[a\0ya\x01\x186`\x04a\x02\xA6V[`\0T`\xFF\x16\x94\x93PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91`\0\x91\x82\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x01bWa\x01ba\x03\x12V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x91P`\x01\x90P[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x01\xA7W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xBFW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x01\x8EW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x01\xEAW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x01W`\0\x80\xFD[a\x02\r\x85\x82\x86\x01a\x01\x95V[\x90\x96\x90\x95P\x93PPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x90``\x84\x01\x90\x82\x87\x01\x84[\x82\x81\x10\x15a\x02hW\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x026V[PPP`\xFF\x94\x90\x94\x16\x92\x01\x91\x90\x91RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\x8FW`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x02\x9FW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x02\xBCW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xD4W`\0\x80\xFD[a\x02\xE0\x88\x83\x89\x01a\x01\x95V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x02\xF9W`\0\x80\xFD[Pa\x03\x06\x87\x82\x88\x01a\x01\x95V[\x95\x98\x94\x97P\x95PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD2X\xF8?\xDCS\xFC\x8D:`\xE5\x0E7\xE6\xD4r\n\xE9\xF3s\xC9w\xE9\x8F\xC4(\x99\x1Bc\xCE4\xC2dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static TESTMULTISIGISM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80cO\xC3\xAA|\x11a\0PW\x80cO\xC3\xAA|\x14a\0\xAFW\x80cde\xE6\x9F\x14a\0\xF0W\x80c\xF7\xE8:\xEE\x14a\x01\nW`\0\x80\xFD[\x80c(R\xB7\x1C\x14a\0lW\x80c.\x0E\xD24\x14a\0\x8EW[`\0\x80\xFD[`\0Ta\0y\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA1a\0\x9C6`\x04a\x01\xD7V[a\x01&V[`@Qa\0\x85\x92\x91\x90a\x02\x19V[a\0\xEEa\0\xBD6`\x04a\x02}V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[a\0\xF8`\x04\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\x85V[a\0ya\x01\x186`\x04a\x02\xA6V[`\0T`\xFF\x16\x94\x93PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91`\0\x91\x82\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x01bWa\x01ba\x03\x12V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x91P`\x01\x90P[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x01\xA7W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xBFW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x01\x8EW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x01\xEAW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x01W`\0\x80\xFD[a\x02\r\x85\x82\x86\x01a\x01\x95V[\x90\x96\x90\x95P\x93PPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x90``\x84\x01\x90\x82\x87\x01\x84[\x82\x81\x10\x15a\x02hW\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x026V[PPP`\xFF\x94\x90\x94\x16\x92\x01\x91\x90\x91RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\x8FW`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x02\x9FW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x02\xBCW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xD4W`\0\x80\xFD[a\x02\xE0\x88\x83\x89\x01a\x01\x95V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x02\xF9W`\0\x80\xFD[Pa\x03\x06\x87\x82\x88\x01a\x01\x95V[\x95\x98\x94\x97P\x95PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD2X\xF8?\xDCS\xFC\x8D:`\xE5\x0E7\xE6\xD4r\n\xE9\xF3s\xC9w\xE9\x8F\xC4(\x99\x1Bc\xCE4\xC2dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static TESTMULTISIGISM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TestMultisigIsm<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TestMultisigIsm<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TestMultisigIsm<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TestMultisigIsm<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TestMultisigIsm<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TestMultisigIsm))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TestMultisigIsm<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TESTMULTISIGISM_ABI.clone(),
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
                TESTMULTISIGISM_ABI.clone(),
                TESTMULTISIGISM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `accept` (0x2852b71c) function
        pub fn accept(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([40, 82, 183, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `moduleType` (0x6465e69f) function
        pub fn module_type(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([100, 101, 230, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAccept` (0x4fc3aa7c) function
        pub fn set_accept(
            &self,
            val: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 195, 170, 124], val)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorsAndThreshold` (0x2e0ed234) function
        pub fn validators_and_threshold(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<::ethers::core::types::Address>, u8),
        > {
            self.0
                .method_hash([46, 14, 210, 52], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0xf7e83aee) function
        pub fn verify(
            &self,
            p0: ::ethers::core::types::Bytes,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([247, 232, 58, 238], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TestMultisigIsm<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `accept` function with signature `accept()` and selector `0x2852b71c`
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
    #[ethcall(name = "accept", abi = "accept()")]
    pub struct AcceptCall;
    ///Container type for all input parameters for the `moduleType` function with signature `moduleType()` and selector `0x6465e69f`
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
    #[ethcall(name = "moduleType", abi = "moduleType()")]
    pub struct ModuleTypeCall;
    ///Container type for all input parameters for the `setAccept` function with signature `setAccept(bool)` and selector `0x4fc3aa7c`
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
    #[ethcall(name = "setAccept", abi = "setAccept(bool)")]
    pub struct SetAcceptCall {
        pub val: bool,
    }
    ///Container type for all input parameters for the `validatorsAndThreshold` function with signature `validatorsAndThreshold(bytes)` and selector `0x2e0ed234`
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
    #[ethcall(name = "validatorsAndThreshold", abi = "validatorsAndThreshold(bytes)")]
    pub struct ValidatorsAndThresholdCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,bytes)` and selector `0xf7e83aee`
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
    #[ethcall(name = "verify", abi = "verify(bytes,bytes)")]
    pub struct VerifyCall(
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TestMultisigIsmCalls {
        Accept(AcceptCall),
        ModuleType(ModuleTypeCall),
        SetAccept(SetAcceptCall),
        ValidatorsAndThreshold(ValidatorsAndThresholdCall),
        Verify(VerifyCall),
    }
    impl ::ethers::core::abi::AbiDecode for TestMultisigIsmCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AcceptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Accept(decoded));
            }
            if let Ok(decoded)
                = <ModuleTypeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ModuleType(decoded));
            }
            if let Ok(decoded)
                = <SetAcceptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAccept(decoded));
            }
            if let Ok(decoded)
                = <ValidatorsAndThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidatorsAndThreshold(decoded));
            }
            if let Ok(decoded)
                = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TestMultisigIsmCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Accept(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ModuleType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAccept(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorsAndThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TestMultisigIsmCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Accept(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModuleType(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAccept(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatorsAndThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptCall> for TestMultisigIsmCalls {
        fn from(value: AcceptCall) -> Self {
            Self::Accept(value)
        }
    }
    impl ::core::convert::From<ModuleTypeCall> for TestMultisigIsmCalls {
        fn from(value: ModuleTypeCall) -> Self {
            Self::ModuleType(value)
        }
    }
    impl ::core::convert::From<SetAcceptCall> for TestMultisigIsmCalls {
        fn from(value: SetAcceptCall) -> Self {
            Self::SetAccept(value)
        }
    }
    impl ::core::convert::From<ValidatorsAndThresholdCall> for TestMultisigIsmCalls {
        fn from(value: ValidatorsAndThresholdCall) -> Self {
            Self::ValidatorsAndThreshold(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for TestMultisigIsmCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    ///Container type for all return fields from the `accept` function with signature `accept()` and selector `0x2852b71c`
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
    pub struct AcceptReturn(pub bool);
    ///Container type for all return fields from the `moduleType` function with signature `moduleType()` and selector `0x6465e69f`
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
    pub struct ModuleTypeReturn(pub u8);
    ///Container type for all return fields from the `validatorsAndThreshold` function with signature `validatorsAndThreshold(bytes)` and selector `0x2e0ed234`
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
    pub struct ValidatorsAndThresholdReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub u8,
    );
    ///Container type for all return fields from the `verify` function with signature `verify(bytes,bytes)` and selector `0xf7e83aee`
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
    pub struct VerifyReturn(pub bool);
}
