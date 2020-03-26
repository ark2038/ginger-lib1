use crate::{
    biginteger::BigInteger768 as BigInteger,
    fields::{Fp768, Fp768Parameters, FpParameters},
};

pub type Fq = Fp768<FqParameters>;

pub struct FqParameters;

impl Fp768Parameters for FqParameters {}
impl FpParameters for FqParameters {
    type BigInt = BigInteger;

    const MODULUS: BigInteger = BigInteger([
        0x5E9063DE245E8001,
        0xE39D54522CDD119F,
        0x638810719AC425F0,
        0x685ACCE9767254A4,
        0xB80F0DA5CB537E38,
        0xB117E776F218059D,
        0x99D124D9A15AF79D,
        0x07FDB925E8A0ED8D,
        0x5EB7E8F96C97D873,
        0xB7F997505B8FAFED,
        0x10229022EEE2CDAD,
        0x01C4C62D92C411,
    ]);

    const MODULUS_BITS: u32 = 753;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 15;

    const R: BigInteger = BigInteger([
        0x98A8ECABD9DC6F42,
        0x91CD31C65A034686,
        0x97C3E4A0CD14572E,
        0x79589819C788B601,
        0xED269C942108976F,
        0x1E0F4D8ACF031D68,
        0x320C3BB713338559,
        0x598B4302D2F00A62,
        0x4074C9CBFD8CA621,
        0x0FA47EDB3865E88C,
        0x95455FB31FF9A195,
        0x7B479EC8E242,
    ]);

    const R2: BigInteger = BigInteger([
        0x84717088cfd190c8,
        0xc7d9ff8e7df03c0a,
        0xa24bea56242b3507,
        0xa896a656a0714c7d,
        0x80a46659ff6f3ddf,
        0x2f47839ef88d7ce8,
        0xa8c86d4604a3b597,
        0xe03c79cac4f7ef07,
        0x2505daf1f4a81245,
        0x8e4605754c381723,
        0xb081f15bcbfdacaf,
        0x2a33e89cb485,
    ]);

    const INV: u64 = 0xF2044CFBE45E7FFF;

// 17
    const GENERATOR: BigInteger = BigInteger([
        0xA8F627F0E629635E,
        0x202AFCE346C36872,
        0x85E1ECE733493254,
        0x6D76E610664AC389,
        0xDF542F3F04441585,
        0x3AA4885BF6D4DD80,
        0xEB8B63C1C0FFFC74,
        0xD2488E985F6CFA4E,
        0xCCE1C2A623F7A66A,
        0x2A060F4D5085B19A,
        0xA9111A596408842F,
        0x11CA8D50BF627,
    ]);

    const TWO_ADICITY: u32 = 15;

    const ROOT_OF_UNITY: BigInteger = BigInteger([
        0x03B079C7556AC378,
        0x2C8C74D04A3F00D4,
        0xD3B001061B90D4CF,
        0x946E77514891B0E6,
        0x79CAEC8AD6DC9EA1,
        0xBEFD780EDC81435D,
        0xE093D4DCA630B154,
        0x43A0F673199F1C12,
        0x92276C78436253FF,
        0xE249D1CF014FCD24,
        0x96F36471FB7C3EC5,
        0x1080B8906B7C4,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xAF4831EF122F4000,
        0x71CEAA29166E88CF,
        0x31C40838CD6212F8,
        0x342D6674BB392A52,
        0xDC0786D2E5A9BF1C,
        0xD88BF3BB790C02CE,
        0xCCE8926CD0AD7BCE,
        0x83FEDC92F45076C6,
        0xAF5BF47CB64BEC39,
        0xDBFCCBA82DC7D7F6,
        0x88114811777166D6,
        0xE26316C96208,
    ]);

    const T: BigInteger = BigInteger([
        0x233EBD20C7BC48BD,
        0x4BE1C73AA8A459BA,
        0xA948C71020E33588,
        0xFC70D0B599D2ECE4,
        0x0B3B701E1B4B96A6,
        0xEF3B622FCEEDE430,
        0xDB1B33A249B342B5,
        0xB0E60FFB724BD141,
        0x5FDABD6FD1F2D92F,
        0x9B5B6FF32EA0B71F,
        0x882220452045DDC5,
        0x3898C5B25,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x119F5E9063DE245E,
        0x25F0E39D54522CDD,
        0x54A4638810719AC4,
        0x7E38685ACCE97672,
        0x059DB80F0DA5CB53,
        0xF79DB117E776F218,
        0xED8D99D124D9A15A,
        0xD87307FDB925E8A0,
        0xAFED5EB7E8F96C97,
        0xCDADB7F997505B8F,
        0xC41110229022EEE2,
        0x1C4C62D92,
    ]);
}