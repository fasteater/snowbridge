pub use outbound_queue_mock::*;
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
pub mod outbound_queue_mock {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"ParaID\",\"name\":\"dest\",\"type\":\"uint32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Message\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"ParaID\",\"name\":\"dest\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"submit\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static OUTBOUNDQUEUEMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        2,
        109,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        97,
        0,
        41,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        160,
        57,
        122,
        134,
        20,
        97,
        0,
        46,
        87,
        128,
        99,
        175,
        254,
        208,
        224,
        20,
        97,
        0,
        67,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        65,
        97,
        0,
        60,
        54,
        96,
        4,
        97,
        1,
        14,
        86,
        91,
        97,
        0,
        129,
        86,
        91,
        0,
        91,
        52,
        128,
        21,
        97,
        0,
        79,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        0,
        84,
        97,
        0,
        100,
        144,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        128,
        84,
        129,
        144,
        97,
        0,
        154,
        144,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        97,
        1,
        156,
        86,
        91,
        145,
        144,
        97,
        1,
        0,
        10,
        129,
        84,
        129,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        2,
        25,
        22,
        144,
        131,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        2,
        23,
        144,
        85,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        131,
        99,
        255,
        255,
        255,
        255,
        22,
        127,
        27,
        17,
        220,
        241,
        51,
        204,
        36,
        15,
        104,
        45,
        171,
        45,
        58,
        142,
        76,
        211,
        92,
        93,
        168,
        201,
        207,
        153,
        173,
        172,
        67,
        54,
        248,
        81,
        37,
        132,
        197,
        173,
        132,
        132,
        96,
        64,
        81,
        97,
        1,
        1,
        146,
        145,
        144,
        97,
        1,
        234,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        163,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        96,
        64,
        132,
        134,
        3,
        18,
        21,
        97,
        1,
        35,
        87,
        96,
        0,
        128,
        253,
        91,
        131,
        53,
        99,
        255,
        255,
        255,
        255,
        129,
        22,
        129,
        20,
        97,
        1,
        55,
        87,
        96,
        0,
        128,
        253,
        91,
        146,
        80,
        96,
        32,
        132,
        1,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        84,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        134,
        1,
        145,
        80,
        134,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        104,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        1,
        119,
        87,
        96,
        0,
        128,
        253,
        91,
        135,
        96,
        32,
        130,
        133,
        1,
        1,
        17,
        21,
        97,
        1,
        137,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        131,
        1,
        148,
        80,
        128,
        147,
        80,
        80,
        80,
        80,
        146,
        80,
        146,
        80,
        146,
        86,
        91,
        96,
        0,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        131,
        22,
        129,
        129,
        3,
        97,
        1,
        224,
        87,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        82,
        96,
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        1,
        1,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        32,
        129,
        82,
        129,
        96,
        32,
        130,
        1,
        82,
        129,
        131,
        96,
        64,
        131,
        1,
        55,
        96,
        0,
        129,
        131,
        1,
        96,
        64,
        144,
        129,
        1,
        145,
        144,
        145,
        82,
        96,
        31,
        144,
        146,
        1,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        22,
        1,
        1,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        103,
        140,
        53,
        247,
        66,
        221,
        47,
        19,
        9,
        3,
        145,
        67,
        113,
        194,
        49,
        89,
        134,
        203,
        219,
        103,
        236,
        190,
        120,
        242,
        190,
        203,
        84,
        224,
        100,
        154,
        125,
        28,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static OUTBOUNDQUEUEMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        97,
        0,
        41,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        160,
        57,
        122,
        134,
        20,
        97,
        0,
        46,
        87,
        128,
        99,
        175,
        254,
        208,
        224,
        20,
        97,
        0,
        67,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        65,
        97,
        0,
        60,
        54,
        96,
        4,
        97,
        1,
        14,
        86,
        91,
        97,
        0,
        129,
        86,
        91,
        0,
        91,
        52,
        128,
        21,
        97,
        0,
        79,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        0,
        84,
        97,
        0,
        100,
        144,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        128,
        84,
        129,
        144,
        97,
        0,
        154,
        144,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        97,
        1,
        156,
        86,
        91,
        145,
        144,
        97,
        1,
        0,
        10,
        129,
        84,
        129,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        2,
        25,
        22,
        144,
        131,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        2,
        23,
        144,
        85,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        131,
        99,
        255,
        255,
        255,
        255,
        22,
        127,
        27,
        17,
        220,
        241,
        51,
        204,
        36,
        15,
        104,
        45,
        171,
        45,
        58,
        142,
        76,
        211,
        92,
        93,
        168,
        201,
        207,
        153,
        173,
        172,
        67,
        54,
        248,
        81,
        37,
        132,
        197,
        173,
        132,
        132,
        96,
        64,
        81,
        97,
        1,
        1,
        146,
        145,
        144,
        97,
        1,
        234,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        163,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        96,
        64,
        132,
        134,
        3,
        18,
        21,
        97,
        1,
        35,
        87,
        96,
        0,
        128,
        253,
        91,
        131,
        53,
        99,
        255,
        255,
        255,
        255,
        129,
        22,
        129,
        20,
        97,
        1,
        55,
        87,
        96,
        0,
        128,
        253,
        91,
        146,
        80,
        96,
        32,
        132,
        1,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        84,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        134,
        1,
        145,
        80,
        134,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        104,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        1,
        119,
        87,
        96,
        0,
        128,
        253,
        91,
        135,
        96,
        32,
        130,
        133,
        1,
        1,
        17,
        21,
        97,
        1,
        137,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        131,
        1,
        148,
        80,
        128,
        147,
        80,
        80,
        80,
        80,
        146,
        80,
        146,
        80,
        146,
        86,
        91,
        96,
        0,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        131,
        22,
        129,
        129,
        3,
        97,
        1,
        224,
        87,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        82,
        96,
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        1,
        1,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        32,
        129,
        82,
        129,
        96,
        32,
        130,
        1,
        82,
        129,
        131,
        96,
        64,
        131,
        1,
        55,
        96,
        0,
        129,
        131,
        1,
        96,
        64,
        144,
        129,
        1,
        145,
        144,
        145,
        82,
        96,
        31,
        144,
        146,
        1,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        22,
        1,
        1,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        103,
        140,
        53,
        247,
        66,
        221,
        47,
        19,
        9,
        3,
        145,
        67,
        113,
        194,
        49,
        89,
        134,
        203,
        219,
        103,
        236,
        190,
        120,
        242,
        190,
        203,
        84,
        224,
        100,
        154,
        125,
        28,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static OUTBOUNDQUEUEMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OutboundQueueMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OutboundQueueMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OutboundQueueMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OutboundQueueMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OutboundQueueMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(OutboundQueueMock)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OutboundQueueMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OUTBOUNDQUEUEMOCK_ABI.clone(),
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
                OUTBOUNDQUEUEMOCK_ABI.clone(),
                OUTBOUNDQUEUEMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submit` (0xa0397a86) function
        pub fn submit(
            &self,
            dest: u32,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 57, 122, 134], (dest, payload))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Message` event
        pub fn message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MessageFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MessageFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OutboundQueueMock<M> {
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
    #[ethevent(name = "Message", abi = "Message(uint32,uint64,bytes)")]
    pub struct MessageFilter {
        #[ethevent(indexed)]
        pub dest: u32,
        #[ethevent(indexed)]
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `submit` function with signature `submit(uint32,bytes)` and selector `0xa0397a86`
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
    #[ethcall(name = "submit", abi = "submit(uint32,bytes)")]
    pub struct SubmitCall {
        pub dest: u32,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OutboundQueueMockCalls {
        Nonce(NonceCall),
        Submit(SubmitCall),
    }
    impl ::ethers::core::abi::AbiDecode for OutboundQueueMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded)
                = <SubmitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Submit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OutboundQueueMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Submit(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OutboundQueueMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Submit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NonceCall> for OutboundQueueMockCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<SubmitCall> for OutboundQueueMockCalls {
        fn from(value: SubmitCall) -> Self {
            Self::Submit(value)
        }
    }
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub u64);
}
