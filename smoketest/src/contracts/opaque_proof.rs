pub use opaque_proof::*;
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
pub mod opaque_proof {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct ParachainClient.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"headPrefix\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"headSuffix\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct ParachainClient.HeadProof\",\"name\":\"headProof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"pos\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"width\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"struct ParachainClient.MMRLeafPartial\",\"name\":\"leafPartial\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"parentNumber\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"parentHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nextAuthoritySetID\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nextAuthoritySetLen\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"nextAuthoritySetRoot\",\"type\":\"bytes32\",\"components\":[]}]},{\"internalType\":\"bytes32[]\",\"name\":\"leafProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"leafProofOrder\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"dummy\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static OPAQUEPROOF_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
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
        4,
        68,
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
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        130,
        29,
        155,
        5,
        20,
        97,
        0,
        48,
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
        62,
        54,
        96,
        4,
        97,
        3,
        21,
        86,
        91,
        80,
        86,
        91,
        0,
        91,
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
        65,
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
        64,
        81,
        96,
        192,
        129,
        1,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        130,
        130,
        16,
        23,
        21,
        97,
        0,
        149,
        87,
        97,
        0,
        149,
        97,
        0,
        67,
        86,
        91,
        96,
        64,
        82,
        144,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
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
        129,
        1,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        130,
        130,
        16,
        23,
        21,
        97,
        0,
        226,
        87,
        97,
        0,
        226,
        97,
        0,
        67,
        86,
        91,
        96,
        64,
        82,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        130,
        96,
        31,
        131,
        1,
        18,
        97,
        0,
        251,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
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
        129,
        17,
        21,
        97,
        1,
        21,
        87,
        97,
        1,
        21,
        97,
        0,
        67,
        86,
        91,
        97,
        1,
        70,
        96,
        32,
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
        96,
        31,
        132,
        1,
        22,
        1,
        97,
        0,
        155,
        86,
        91,
        129,
        129,
        82,
        132,
        96,
        32,
        131,
        134,
        1,
        1,
        17,
        21,
        97,
        1,
        91,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        96,
        32,
        133,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        145,
        129,
        1,
        96,
        32,
        1,
        145,
        144,
        145,
        82,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        130,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        137,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        96,
        32,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        17,
        21,
        97,
        1,
        165,
        87,
        97,
        1,
        165,
        97,
        0,
        67,
        86,
        91,
        129,
        96,
        5,
        27,
        97,
        1,
        180,
        130,
        130,
        1,
        97,
        0,
        155,
        86,
        91,
        146,
        131,
        82,
        132,
        129,
        1,
        130,
        1,
        146,
        130,
        129,
        1,
        144,
        135,
        133,
        17,
        21,
        97,
        1,
        206,
        87,
        96,
        0,
        128,
        253,
        91,
        131,
        135,
        1,
        146,
        80,
        91,
        132,
        131,
        16,
        21,
        97,
        1,
        237,
        87,
        130,
        53,
        130,
        82,
        145,
        131,
        1,
        145,
        144,
        131,
        1,
        144,
        97,
        1,
        212,
        86,
        91,
        151,
        150,
        80,
        80,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        96,
        130,
        132,
        3,
        18,
        21,
        97,
        2,
        10,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        64,
        81,
        96,
        96,
        129,
        1,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        130,
        16,
        129,
        131,
        17,
        23,
        21,
        97,
        2,
        46,
        87,
        97,
        2,
        46,
        97,
        0,
        67,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        147,
        80,
        132,
        53,
        131,
        82,
        96,
        32,
        133,
        1,
        53,
        96,
        32,
        132,
        1,
        82,
        96,
        64,
        133,
        1,
        53,
        145,
        80,
        128,
        130,
        17,
        21,
        97,
        2,
        87,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        2,
        100,
        133,
        130,
        134,
        1,
        97,
        1,
        120,
        86,
        91,
        96,
        64,
        131,
        1,
        82,
        80,
        80,
        146,
        145,
        80,
        80,
        86,
        91,
        128,
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
        2,
        133,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        96,
        192,
        130,
        132,
        3,
        18,
        21,
        97,
        2,
        156,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        2,
        164,
        97,
        0,
        114,
        86,
        91,
        144,
        80,
        129,
        53,
        96,
        255,
        129,
        22,
        129,
        20,
        97,
        2,
        183,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        82,
        97,
        2,
        197,
        96,
        32,
        131,
        1,
        97,
        2,
        113,
        86,
        91,
        96,
        32,
        130,
        1,
        82,
        96,
        64,
        130,
        1,
        53,
        96,
        64,
        130,
        1,
        82,
        96,
        96,
        130,
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
        129,
        22,
        129,
        20,
        97,
        2,
        239,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        96,
        130,
        1,
        82,
        97,
        3,
        0,
        96,
        128,
        131,
        1,
        97,
        2,
        113,
        86,
        91,
        96,
        128,
        130,
        1,
        82,
        96,
        160,
        130,
        1,
        53,
        96,
        160,
        130,
        1,
        82,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        3,
        39,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
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
        3,
        63,
        87,
        96,
        0,
        128,
        253,
        91,
        144,
        131,
        1,
        144,
        97,
        1,
        96,
        130,
        134,
        3,
        18,
        21,
        97,
        3,
        84,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        92,
        97,
        0,
        114,
        86,
        91,
        130,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        107,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        119,
        135,
        130,
        134,
        1,
        97,
        0,
        234,
        86,
        91,
        130,
        82,
        80,
        96,
        32,
        131,
        1,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        140,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        152,
        135,
        130,
        134,
        1,
        97,
        0,
        234,
        86,
        91,
        96,
        32,
        131,
        1,
        82,
        80,
        96,
        64,
        131,
        1,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        176,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        188,
        135,
        130,
        134,
        1,
        97,
        1,
        248,
        86,
        91,
        96,
        64,
        131,
        1,
        82,
        80,
        97,
        3,
        207,
        134,
        96,
        96,
        133,
        1,
        97,
        2,
        138,
        86,
        91,
        96,
        96,
        130,
        1,
        82,
        97,
        1,
        32,
        131,
        1,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        231,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        243,
        135,
        130,
        134,
        1,
        97,
        1,
        120,
        86,
        91,
        96,
        128,
        131,
        1,
        82,
        80,
        97,
        1,
        64,
        146,
        144,
        146,
        1,
        53,
        96,
        160,
        131,
        1,
        82,
        80,
        147,
        146,
        80,
        80,
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
        41,
        64,
        129,
        51,
        136,
        191,
        14,
        237,
        140,
        21,
        69,
        143,
        175,
        120,
        235,
        57,
        208,
        146,
        249,
        147,
        148,
        68,
        208,
        83,
        129,
        110,
        76,
        193,
        164,
        60,
        238,
        64,
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
    pub static OPAQUEPROOF_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
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
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        130,
        29,
        155,
        5,
        20,
        97,
        0,
        48,
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
        62,
        54,
        96,
        4,
        97,
        3,
        21,
        86,
        91,
        80,
        86,
        91,
        0,
        91,
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
        65,
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
        64,
        81,
        96,
        192,
        129,
        1,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        130,
        130,
        16,
        23,
        21,
        97,
        0,
        149,
        87,
        97,
        0,
        149,
        97,
        0,
        67,
        86,
        91,
        96,
        64,
        82,
        144,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
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
        129,
        1,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        130,
        130,
        16,
        23,
        21,
        97,
        0,
        226,
        87,
        97,
        0,
        226,
        97,
        0,
        67,
        86,
        91,
        96,
        64,
        82,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        130,
        96,
        31,
        131,
        1,
        18,
        97,
        0,
        251,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
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
        129,
        17,
        21,
        97,
        1,
        21,
        87,
        97,
        1,
        21,
        97,
        0,
        67,
        86,
        91,
        97,
        1,
        70,
        96,
        32,
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
        96,
        31,
        132,
        1,
        22,
        1,
        97,
        0,
        155,
        86,
        91,
        129,
        129,
        82,
        132,
        96,
        32,
        131,
        134,
        1,
        1,
        17,
        21,
        97,
        1,
        91,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        96,
        32,
        133,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        145,
        129,
        1,
        96,
        32,
        1,
        145,
        144,
        145,
        82,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        130,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        137,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        96,
        32,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        17,
        21,
        97,
        1,
        165,
        87,
        97,
        1,
        165,
        97,
        0,
        67,
        86,
        91,
        129,
        96,
        5,
        27,
        97,
        1,
        180,
        130,
        130,
        1,
        97,
        0,
        155,
        86,
        91,
        146,
        131,
        82,
        132,
        129,
        1,
        130,
        1,
        146,
        130,
        129,
        1,
        144,
        135,
        133,
        17,
        21,
        97,
        1,
        206,
        87,
        96,
        0,
        128,
        253,
        91,
        131,
        135,
        1,
        146,
        80,
        91,
        132,
        131,
        16,
        21,
        97,
        1,
        237,
        87,
        130,
        53,
        130,
        82,
        145,
        131,
        1,
        145,
        144,
        131,
        1,
        144,
        97,
        1,
        212,
        86,
        91,
        151,
        150,
        80,
        80,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        96,
        130,
        132,
        3,
        18,
        21,
        97,
        2,
        10,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        64,
        81,
        96,
        96,
        129,
        1,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        130,
        16,
        129,
        131,
        17,
        23,
        21,
        97,
        2,
        46,
        87,
        97,
        2,
        46,
        97,
        0,
        67,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        147,
        80,
        132,
        53,
        131,
        82,
        96,
        32,
        133,
        1,
        53,
        96,
        32,
        132,
        1,
        82,
        96,
        64,
        133,
        1,
        53,
        145,
        80,
        128,
        130,
        17,
        21,
        97,
        2,
        87,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        2,
        100,
        133,
        130,
        134,
        1,
        97,
        1,
        120,
        86,
        91,
        96,
        64,
        131,
        1,
        82,
        80,
        80,
        146,
        145,
        80,
        80,
        86,
        91,
        128,
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
        2,
        133,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        96,
        192,
        130,
        132,
        3,
        18,
        21,
        97,
        2,
        156,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        2,
        164,
        97,
        0,
        114,
        86,
        91,
        144,
        80,
        129,
        53,
        96,
        255,
        129,
        22,
        129,
        20,
        97,
        2,
        183,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        82,
        97,
        2,
        197,
        96,
        32,
        131,
        1,
        97,
        2,
        113,
        86,
        91,
        96,
        32,
        130,
        1,
        82,
        96,
        64,
        130,
        1,
        53,
        96,
        64,
        130,
        1,
        82,
        96,
        96,
        130,
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
        129,
        22,
        129,
        20,
        97,
        2,
        239,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        96,
        130,
        1,
        82,
        97,
        3,
        0,
        96,
        128,
        131,
        1,
        97,
        2,
        113,
        86,
        91,
        96,
        128,
        130,
        1,
        82,
        96,
        160,
        130,
        1,
        53,
        96,
        160,
        130,
        1,
        82,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        3,
        39,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
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
        3,
        63,
        87,
        96,
        0,
        128,
        253,
        91,
        144,
        131,
        1,
        144,
        97,
        1,
        96,
        130,
        134,
        3,
        18,
        21,
        97,
        3,
        84,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        92,
        97,
        0,
        114,
        86,
        91,
        130,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        107,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        119,
        135,
        130,
        134,
        1,
        97,
        0,
        234,
        86,
        91,
        130,
        82,
        80,
        96,
        32,
        131,
        1,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        140,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        152,
        135,
        130,
        134,
        1,
        97,
        0,
        234,
        86,
        91,
        96,
        32,
        131,
        1,
        82,
        80,
        96,
        64,
        131,
        1,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        176,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        188,
        135,
        130,
        134,
        1,
        97,
        1,
        248,
        86,
        91,
        96,
        64,
        131,
        1,
        82,
        80,
        97,
        3,
        207,
        134,
        96,
        96,
        133,
        1,
        97,
        2,
        138,
        86,
        91,
        96,
        96,
        130,
        1,
        82,
        97,
        1,
        32,
        131,
        1,
        53,
        130,
        129,
        17,
        21,
        97,
        3,
        231,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        243,
        135,
        130,
        134,
        1,
        97,
        1,
        120,
        86,
        91,
        96,
        128,
        131,
        1,
        82,
        80,
        97,
        1,
        64,
        146,
        144,
        146,
        1,
        53,
        96,
        160,
        131,
        1,
        82,
        80,
        147,
        146,
        80,
        80,
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
        41,
        64,
        129,
        51,
        136,
        191,
        14,
        237,
        140,
        21,
        69,
        143,
        175,
        120,
        235,
        57,
        208,
        146,
        249,
        147,
        148,
        68,
        208,
        83,
        129,
        110,
        76,
        193,
        164,
        60,
        238,
        64,
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
    pub static OPAQUEPROOF_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OpaqueProof<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OpaqueProof<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OpaqueProof<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OpaqueProof<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OpaqueProof<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(OpaqueProof)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OpaqueProof<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPAQUEPROOF_ABI.clone(),
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
                OPAQUEPROOF_ABI.clone(),
                OPAQUEPROOF_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dummy` (0x821d9b05) function
        pub fn dummy(
            &self,
            proof: Proof,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 29, 155, 5], (proof,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OpaqueProof<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `dummy` function with signature `dummy((bytes,bytes,(uint256,uint256,bytes32[]),(uint8,uint32,bytes32,uint64,uint32,bytes32),bytes32[],uint256))` and selector `0x821d9b05`
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
        name = "dummy",
        abi = "dummy((bytes,bytes,(uint256,uint256,bytes32[]),(uint8,uint32,bytes32,uint64,uint32,bytes32),bytes32[],uint256))"
    )]
    pub struct DummyCall {
        pub proof: Proof,
    }
    ///`HeadProof(uint256,uint256,bytes32[])`
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
    pub struct HeadProof {
        pub pos: ::ethers::core::types::U256,
        pub width: ::ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///`MmrleafPartial(uint8,uint32,bytes32,uint64,uint32,bytes32)`
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
    pub struct MmrleafPartial {
        pub version: u8,
        pub parent_number: u32,
        pub parent_hash: [u8; 32],
        pub next_authority_set_id: u64,
        pub next_authority_set_len: u32,
        pub next_authority_set_root: [u8; 32],
    }
    ///`Proof(bytes,bytes,(uint256,uint256,bytes32[]),(uint8,uint32,bytes32,uint64,uint32,bytes32),bytes32[],uint256)`
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
    pub struct Proof {
        pub head_prefix: ::ethers::core::types::Bytes,
        pub head_suffix: ::ethers::core::types::Bytes,
        pub head_proof: HeadProof,
        pub leaf_partial: MmrleafPartial,
        pub leaf_proof: ::std::vec::Vec<[u8; 32]>,
        pub leaf_proof_order: ::ethers::core::types::U256,
    }
}
