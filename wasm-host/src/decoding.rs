use crate::hashing::HASH256_LEN;
use bigdecimal::{BigDecimal, Signed, ToPrimitive, Zero};
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use xrpl::core::addresscodec::utils::decode_base58;
use xrpl::core::binarycodec::definitions::{get_ledger_entry_type_code, get_transaction_type_code};
use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
use xrpl::core::binarycodec::types::{Amount, Currency, Issue};
use xrpl::core::exceptions::{XRPLCoreException, XRPLCoreResult};
use xrpl::utils::exceptions::XRPRangeException;
use xrpl::utils::{MAX_IOU_EXPONENT, MAX_IOU_PRECISION, MIN_IOU_EXPONENT, verify_valid_ic_value};

pub const ACCOUNT_ID_LEN: usize = 20;
pub type AccountId = Vec<u8>;

/* from rippled:

    STYPE(STI_UNKNOWN, -2)                        \
    STYPE(STI_NOTPRESENT, 0)                      \
    STYPE(STI_UINT16, 1)                          \
                                                  \
    /* types (common) */                          \
    STYPE(STI_UINT32, 2)                          \
    STYPE(STI_UINT64, 3)                          \
    STYPE(STI_UINT128, 4)                         \
    STYPE(STI_UINT256, 5)                         \
    STYPE(STI_AMOUNT, 6)                          \
    STYPE(STI_VL, 7)                              \
    STYPE(STI_ACCOUNT, 8)                         \
    STYPE(STI_NUMBER, 9)                          \
                                                  \
    /* 10-13 are reserved */                      \
    STYPE(STI_OBJECT, 14)                         \
    STYPE(STI_ARRAY, 15)                          \
                                                  \
    /* types (uncommon) */                        \
    STYPE(STI_UINT8, 16)                          \
    STYPE(STI_UINT160, 17)                        \
    STYPE(STI_PATHSET, 18)                        \
    STYPE(STI_VECTOR256, 19)                      \
    STYPE(STI_UINT96, 20)                         \
    STYPE(STI_UINT192, 21)                        \
    STYPE(STI_UINT384, 22)                        \
    STYPE(STI_UINT512, 23)                        \
    STYPE(STI_ISSUE, 24)                          \
    STYPE(STI_XCHAIN_BRIDGE, 25)                  \
    STYPE(STI_CURRENCY, 26)                       \
                                                  \
    /* high-level types */                        \
    /* cannot be serialized inside other types */ \
    STYPE(STI_TRANSACTION, 10001)                 \
    STYPE(STI_LEDGERENTRY, 10002)                 \
    STYPE(STI_VALIDATION, 10003)                  \
    STYPE(STI_METADATA, 10004)

*/
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Decodable {
    UINT16,
    Uint16_TX_TYPE,
    UINT16_LEDGER_OBJECT_TYPE,
    UINT32,
    UINT64,
    UINT128,
    UINT256,
    AMOUNT,
    VL_HEX,
    VL_OTHER,
    ACCOUNT,
    NUMBER,
    OBJECT,
    ARRAY,
    UINT8,
    UINT160,
    PATHSET,
    VECTOR256,
    UINT96,
    UINT192,
    UINT384,
    UINT512,
    ISSUE,
    XCHAIN_BRIDGE,
    CURRENCY,
    AS_IS,
    NOT,
}

impl Decodable {
    pub fn from_sfield(field: i32) -> Self {
        assert!(field >= 0);
        if let Some(name) = SField_To_Name.get(&field) {
            if name == "TransactionType" {
                return Decodable::Uint16_TX_TYPE;
            } else if name == "LedgerEntryType" {
                return Decodable::UINT16_LEDGER_OBJECT_TYPE;
            } else if name == "PublicKey"
                || name == "MessageKey"
                || name == "SigningPubKey"
                || name == "TxnSignature"
            {
                return Decodable::VL_HEX;
            }
        }

        let field_type = field >> 16;
        match field_type {
            1 => Decodable::UINT16,
            2 => Decodable::UINT32,
            3 => Decodable::UINT64,
            4 => Decodable::UINT128,
            5 => Decodable::UINT256,
            6 => Decodable::AMOUNT,
            7 => Decodable::VL_OTHER,
            8 => Decodable::ACCOUNT,
            9 => Decodable::NUMBER,
            // 10-13 are reserved as stated in rippled
            14 => Decodable::OBJECT,
            15 => Decodable::ARRAY,
            16 => Decodable::UINT8,
            17 => Decodable::UINT160,
            18 => Decodable::PATHSET,
            19 => Decodable::VECTOR256,
            20 => Decodable::UINT96,
            21 => Decodable::UINT192,
            22 => Decodable::UINT384,
            23 => Decodable::UINT512,
            24 => Decodable::ISSUE,
            25 => Decodable::XCHAIN_BRIDGE,
            26 => Decodable::CURRENCY,

            _ => Decodable::NOT,
        }
    }
}

pub fn decode(s: &str, decodable: Decodable) -> Option<Vec<u8>> {
    match decodable {
        Decodable::UINT16 => decode_u16(s),
        Decodable::Uint16_TX_TYPE => decode_tx_type(s),
        Decodable::UINT16_LEDGER_OBJECT_TYPE => decode_ledger_obj_type(s),
        Decodable::UINT32 => decode_u32(s),
        Decodable::UINT64 => decode_u64(s),
        Decodable::UINT128 => decode_u128(s),
        Decodable::UINT256 => decode_hash(s),
        Decodable::AMOUNT => decode_amount(s),
        Decodable::VL_HEX => decode_hex(s),
        Decodable::VL_OTHER => decode_vl_other(s),
        Decodable::ACCOUNT => decode_account_id(s),
        Decodable::NUMBER => decode_number(s),
        Decodable::OBJECT => not_leaf(s),
        Decodable::ARRAY => not_leaf(s),
        Decodable::UINT8 => decode_u8(s),
        Decodable::UINT160 => decode_hex(s),
        Decodable::PATHSET => not_leaf(s),
        Decodable::VECTOR256 => decode_hex(s),
        Decodable::UINT96 => decode_hex(s),
        Decodable::UINT192 => decode_hex(s),
        Decodable::UINT384 => decode_hex(s),
        Decodable::UINT512 => decode_hex(s),
        Decodable::ISSUE => decode_issue(s),
        Decodable::XCHAIN_BRIDGE => not_leaf(s),
        Decodable::CURRENCY => decode_currency(s),
        Decodable::AS_IS => raw_string_to_bytes(s),
        Decodable::NOT => decode_not(s),
    }
}

pub fn decode_tx_type(tx_type: &str) -> Option<Vec<u8>> {
    get_transaction_type_code(tx_type).map(|num| num.to_le_bytes().to_vec())
}

pub fn decode_ledger_obj_type(lo_type: &str) -> Option<Vec<u8>> {
    get_ledger_entry_type_code(lo_type).map(|num| num.to_le_bytes().to_vec())
}

pub fn decode_account_id(base58_account_id: &str) -> Option<Vec<u8>> {
    match decode_base58(base58_account_id, &[0x0]) {
        Ok(aid) => {
            if aid.len() == ACCOUNT_ID_LEN {
                Some(aid)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn decode_hash(hex_hash: &str) -> Option<Vec<u8>> {
    match hex::decode(hex_hash) {
        Ok(bytes) => {
            if bytes.len() == HASH256_LEN {
                Some(bytes)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
pub fn decode_hex(s: &str) -> Option<Vec<u8>> {
    hex::decode(s).ok()
}

pub fn decode_u8(s: &str) -> Option<Vec<u8>> {
    match s.parse::<u8>() {
        Ok(num) => Some(num.to_le_bytes().to_vec()),
        Err(_) => None,
    }
}

pub fn decode_u16(s: &str) -> Option<Vec<u8>> {
    match s.parse::<u16>() {
        Ok(num) => Some(num.to_le_bytes().to_vec()),
        Err(_) => None,
    }
}

pub fn decode_u32(s: &str) -> Option<Vec<u8>> {
    match s.parse::<u32>() {
        Ok(num) => Some(num.to_le_bytes().to_vec()),
        Err(_) => None,
    }
}

pub fn decode_u64(s: &str) -> Option<Vec<u8>> {
    match s.parse::<u64>() {
        Ok(num) => Some(num.to_le_bytes().to_vec()),
        Err(_) => None,
    }
}

pub fn decode_u128(hex_hash: &str) -> Option<Vec<u8>> {
    match hex::decode(hex_hash) {
        Ok(bytes) => {
            if bytes.len() == 16 {
                Some(bytes)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn decode_vl_other(s: &str) -> Option<Vec<u8>> {
    decode_hex(s)
}

// the following consts, _serialize_issued_currency_value and
// _deserialize_issued_currency_amount functions are copied
// from https://github.com/sephynox/xrpl-rust
const _MIN_MANTISSA: u128 = u128::pow(10, 15);
const _MAX_MANTISSA: u128 = u128::pow(10, 16) - 1;
const _POS_SIGN_BIT_MASK: i64 = 0x4000000000000000;
const _ZERO_CURRENCY_AMOUNT_HEX: u64 = 0x8000000000000000;
/// Serializes the value field of an issued currency amount
/// to its bytes representation.
pub fn _serialize_issued_currency_value(decimal: BigDecimal) -> XRPLCoreResult<[u8; 8]> {
    let decimal = decimal.with_prec(MAX_IOU_PRECISION as u64);
    verify_valid_ic_value(&decimal.to_scientific_notation())
        .map_err(|e| XRPLCoreException::XRPLUtilsError(e.to_string()))?;

    if decimal.is_zero() {
        return Ok((_ZERO_CURRENCY_AMOUNT_HEX).to_be_bytes());
    };

    let is_positive: bool = decimal.is_positive();
    let (mantissa_str, scale) = decimal.normalized().as_bigint_and_exponent();
    let mut exp: i32 = -(scale as i32);
    let mut mantissa: u128 = mantissa_str.abs().to_u128().unwrap();

    while mantissa < _MIN_MANTISSA && exp > MIN_IOU_EXPONENT {
        mantissa *= 10;
        exp -= 1;
    }

    while mantissa > _MAX_MANTISSA {
        if exp >= MAX_IOU_EXPONENT {
            return Err(XRPLBinaryCodecException::from(
                XRPRangeException::UnexpectedICAmountOverflow {
                    max: MAX_IOU_EXPONENT as usize,
                    found: exp as usize,
                },
            )
            .into());
        } else {
            mantissa /= 10;
            exp += 1;
        }
    }

    if exp < MIN_IOU_EXPONENT || mantissa < _MIN_MANTISSA {
        // Round to zero
        Ok((_ZERO_CURRENCY_AMOUNT_HEX).to_be_bytes())
    } else if exp > MAX_IOU_EXPONENT || mantissa > _MAX_MANTISSA {
        Err(
            XRPLBinaryCodecException::from(XRPRangeException::UnexpectedICAmountOverflow {
                max: MAX_IOU_EXPONENT as usize,
                found: exp as usize,
            })
            .into(),
        )
    } else {
        // "Not XRP" bit set
        let mut serial: i128 = _ZERO_CURRENCY_AMOUNT_HEX as i128;

        // "Is positive" bit set
        if is_positive {
            serial |= _POS_SIGN_BIT_MASK as i128;
        };

        // next 8 bits are exponents
        serial |= ((exp as i64 + 97) << 54) as i128;
        // last 54 bits are mantissa
        serial |= mantissa as i128;

        Ok((serial as u64).to_be_bytes())
    }
}

//TODO we will use rippled Number class for computation
pub fn _deserialize_issued_currency_amount(bytes: [u8; 8]) -> XRPLCoreResult<BigDecimal> {
    let mut value: BigDecimal;

    // Some wizardry by Amie Corso
    let exp = ((bytes[0] as i32 & 0x3F) << 2) + ((bytes[1] as i32 & 0xFF) >> 6) - 97;

    if exp < MIN_IOU_EXPONENT {
        value = BigDecimal::from(0);
    } else {
        let hex_mantissa = hex::encode([&[bytes[1] & 0x3F], &bytes[2..]].concat());
        let int_mantissa = i128::from_str_radix(&hex_mantissa, 16)
            .map_err(XRPLBinaryCodecException::ParseIntError)?;

        // Adjust scale using the exponent
        let scale = exp.unsigned_abs();
        value = BigDecimal::new(int_mantissa.into(), scale as i64);

        // Handle the sign
        if bytes[0] & 0x40 > 0 {
            // Set the value to positive (BigDecimal assumes positive by default)
            value = value.abs();
        } else {
            // Set the value to negative
            value = -value.abs();
        }
    }
    verify_valid_ic_value(&value.to_string())
        .map_err(|e| XRPLCoreException::XRPLUtilsError(e.to_string()))?;

    Ok(value)
}

pub fn decode_number(s: &str) -> Option<Vec<u8>> {
    let value = BigDecimal::from_str(s).ok()?;
    _serialize_issued_currency_value(value)
        .ok()
        .map(|bytes| bytes.to_vec())
}

pub fn decode_currency(s: &str) -> Option<Vec<u8>> {
    match Currency::try_from(s) {
        Ok(currency) => Some(currency.as_ref().to_vec()),
        Err(_) => None,
    }
}

const POSITIVE_MPT: u8 = 0b_0110_0000;
const NEGATIVE_MPT: u8 = 0b_0010_0000;
pub fn decode_amount_json(value: Value) -> Option<Vec<u8>> {
    // try to decode an MPT
    if let Some(mpt_issuance_id) = value.get("mpt_issuance_id") {
        if let Some(amount_value) = value.get("value") {
            // in rippled:
            // TYPED_SFIELD(sfMPTAmount,                UINT64,    26)
            //
            // So amount value should always be positive, but since there is a sign bit
            // in MPT amount encoding, we try to handle negative values just in case.
            let amount = amount_value.as_str()?.parse::<i64>().ok()?;
            let (negative, amount_abs) = if amount < 0 {
                // amount.wrapping_neg() handles i64::MIN correctly,
                // where -i64::MIN would overflow in regular arithmetic.
                (true, amount.wrapping_neg() as u64)
            } else {
                (false, amount as u64)
            };

            let mut bytes = Vec::new();
            bytes.push(if negative { NEGATIVE_MPT } else { POSITIVE_MPT });
            // Big Endian matches what rippled returns
            bytes.append(&mut amount_abs.to_be_bytes().to_vec());
            let mpt_issuance_id = mpt_issuance_id.as_str()?.to_string();
            let mut mpt_issuance_id_bytes = hex::decode(mpt_issuance_id.as_str()).ok()?;
            bytes.append(&mut mpt_issuance_id_bytes);
            return Some(bytes);
        }
        return None;
    }
    // if not an MPT, try to decode an XRP or IOU amount using the library
    match Amount::try_from(value) {
        Ok(amount) => Some(amount.as_ref().to_vec()),
        Err(_) => None,
    }
}

pub fn decode_amount(s: &str) -> Option<Vec<u8>> {
    let v: Value = serde_json::from_str(s).expect("Invalid json string");
    decode_amount_json(v)
}

pub fn decode_issue_json(value: Value) -> Option<Vec<u8>> {
    if let Some(mpt_issuance_id) = value.get("mpt_issuance_id") {
        return decode_hex(mpt_issuance_id.as_str()?);
    }
    match Issue::try_from(value) {
        Ok(issue) => Some(issue.as_ref().to_vec()),
        Err(_) => None,
    }
}

pub fn decode_issue(s: &str) -> Option<Vec<u8>> {
    let v: Value = serde_json::from_str(s).expect("Invalid json string");
    decode_issue_json(v)
}

pub fn not_leaf(_: &str) -> Option<Vec<u8>> {
    None
}

pub fn decode_not(_: &str) -> Option<Vec<u8>> {
    None
}

pub fn raw_string_to_bytes(s: &str) -> Option<Vec<u8>> {
    Some(s.as_bytes().to_vec())
}

lazy_static! {
    pub static ref SField_To_Name: HashMap<i32, String> = polulate_field_names();
}

fn polulate_field_names() -> HashMap<i32, String> {
    let mut sfield_names: HashMap<i32, String> = HashMap::new();
    sfield_names.insert(-1, "Invalid".to_string());
    sfield_names.insert(0, "Generic".to_string());
    sfield_names.insert(65537, "LedgerEntryType".to_string());
    sfield_names.insert(65538, "TransactionType".to_string());
    sfield_names.insert(65539, "SignerWeight".to_string());
    sfield_names.insert(65540, "TransferFee".to_string());
    sfield_names.insert(65541, "TradingFee".to_string());
    sfield_names.insert(65542, "DiscountedFee".to_string());
    sfield_names.insert(65552, "Version".to_string());
    sfield_names.insert(65553, "HookStateChangeCount".to_string());
    sfield_names.insert(65554, "HookEmitCount".to_string());
    sfield_names.insert(65555, "HookExecutionIndex".to_string());
    sfield_names.insert(65556, "HookApiVersion".to_string());
    sfield_names.insert(65557, "LedgerFixType".to_string());
    sfield_names.insert(131073, "NetworkID".to_string());
    sfield_names.insert(131074, "Flags".to_string());
    sfield_names.insert(131075, "SourceTag".to_string());
    sfield_names.insert(131076, "Sequence".to_string());
    sfield_names.insert(131077, "PreviousTxnLgrSeq".to_string());
    sfield_names.insert(131078, "LedgerSequence".to_string());
    sfield_names.insert(131079, "CloseTime".to_string());
    sfield_names.insert(131080, "ParentCloseTime".to_string());
    sfield_names.insert(131081, "SigningTime".to_string());
    sfield_names.insert(131082, "Expiration".to_string());
    sfield_names.insert(131083, "TransferRate".to_string());
    sfield_names.insert(131084, "WalletSize".to_string());
    sfield_names.insert(131085, "OwnerCount".to_string());
    sfield_names.insert(131086, "DestinationTag".to_string());
    sfield_names.insert(131087, "LastUpdateTime".to_string());
    sfield_names.insert(131088, "HighQualityIn".to_string());
    sfield_names.insert(131089, "HighQualityOut".to_string());
    sfield_names.insert(131090, "LowQualityIn".to_string());
    sfield_names.insert(131091, "LowQualityOut".to_string());
    sfield_names.insert(131092, "QualityIn".to_string());
    sfield_names.insert(131093, "QualityOut".to_string());
    sfield_names.insert(131094, "StampEscrow".to_string());
    sfield_names.insert(131095, "BondAmount".to_string());
    sfield_names.insert(131096, "LoadFee".to_string());
    sfield_names.insert(131097, "OfferSequence".to_string());
    sfield_names.insert(131098, "FirstLedgerSequence".to_string());
    sfield_names.insert(131099, "LastLedgerSequence".to_string());
    sfield_names.insert(131100, "TransactionIndex".to_string());
    sfield_names.insert(131101, "OperationLimit".to_string());
    sfield_names.insert(131102, "ReferenceFeeUnits".to_string());
    sfield_names.insert(131103, "ReserveBase".to_string());
    sfield_names.insert(131104, "ReserveIncrement".to_string());
    sfield_names.insert(131105, "SetFlag".to_string());
    sfield_names.insert(131106, "ClearFlag".to_string());
    sfield_names.insert(131107, "SignerQuorum".to_string());
    sfield_names.insert(131108, "CancelAfter".to_string());
    sfield_names.insert(131109, "FinishAfter".to_string());
    sfield_names.insert(131110, "SignerListID".to_string());
    sfield_names.insert(131111, "SettleDelay".to_string());
    sfield_names.insert(131112, "TicketCount".to_string());
    sfield_names.insert(131113, "TicketSequence".to_string());
    sfield_names.insert(131114, "NFTokenTaxon".to_string());
    sfield_names.insert(131115, "MintedNFTokens".to_string());
    sfield_names.insert(131116, "BurnedNFTokens".to_string());
    sfield_names.insert(131117, "HookStateCount".to_string());
    sfield_names.insert(131118, "EmitGeneration".to_string());
    sfield_names.insert(131120, "VoteWeight".to_string());
    sfield_names.insert(131122, "FirstNFTokenSequence".to_string());
    sfield_names.insert(131123, "OracleDocumentID".to_string());
    sfield_names.insert(131124, "ExtensionComputeLimit".to_string());
    sfield_names.insert(131125, "ExtensionSizeLimit".to_string());
    sfield_names.insert(131126, "GasPrice".to_string());
    sfield_names.insert(131127, "ComputationAllowance".to_string());
    sfield_names.insert(196609, "IndexNext".to_string());
    sfield_names.insert(196610, "IndexPrevious".to_string());
    sfield_names.insert(196611, "BookNode".to_string());
    sfield_names.insert(196612, "OwnerNode".to_string());
    sfield_names.insert(196613, "BaseFee".to_string());
    sfield_names.insert(196614, "ExchangeRate".to_string());
    sfield_names.insert(196615, "LowNode".to_string());
    sfield_names.insert(196616, "HighNode".to_string());
    sfield_names.insert(196617, "DestinationNode".to_string());
    sfield_names.insert(196618, "Cookie".to_string());
    sfield_names.insert(196619, "ServerVersion".to_string());
    sfield_names.insert(196620, "NFTokenOfferNode".to_string());
    sfield_names.insert(196621, "EmitBurden".to_string());
    sfield_names.insert(196624, "HookOn".to_string());
    sfield_names.insert(196625, "HookInstructionCount".to_string());
    sfield_names.insert(196626, "HookReturnCode".to_string());
    sfield_names.insert(196627, "ReferenceCount".to_string());
    sfield_names.insert(196628, "XChainClaimID".to_string());
    sfield_names.insert(196629, "XChainAccountCreateCount".to_string());
    sfield_names.insert(196630, "XChainAccountClaimCount".to_string());
    sfield_names.insert(196631, "AssetPrice".to_string());
    sfield_names.insert(196632, "MaximumAmount".to_string());
    sfield_names.insert(196633, "OutstandingAmount".to_string());
    sfield_names.insert(196634, "MPTAmount".to_string());
    sfield_names.insert(196635, "IssuerNode".to_string());
    sfield_names.insert(196636, "SubjectNode".to_string());
    sfield_names.insert(262145, "EmailHash".to_string());
    sfield_names.insert(327681, "LedgerHash".to_string());
    sfield_names.insert(327682, "ParentHash".to_string());
    sfield_names.insert(327683, "TransactionHash".to_string());
    sfield_names.insert(327684, "AccountHash".to_string());
    sfield_names.insert(327685, "PreviousTxnID".to_string());
    sfield_names.insert(327686, "LedgerIndex".to_string());
    sfield_names.insert(327687, "WalletLocator".to_string());
    sfield_names.insert(327688, "RootIndex".to_string());
    sfield_names.insert(327689, "AccountTxnID".to_string());
    sfield_names.insert(327690, "NFTokenID".to_string());
    sfield_names.insert(327691, "EmitParentTxnID".to_string());
    sfield_names.insert(327692, "EmitNonce".to_string());
    sfield_names.insert(327693, "EmitHookHash".to_string());
    sfield_names.insert(327694, "AMMID".to_string());
    sfield_names.insert(327696, "BookDirectory".to_string());
    sfield_names.insert(327697, "InvoiceID".to_string());
    sfield_names.insert(327698, "Nickname".to_string());
    sfield_names.insert(327699, "Amendment".to_string());
    sfield_names.insert(327701, "Digest".to_string());
    sfield_names.insert(327702, "Channel".to_string());
    sfield_names.insert(327703, "ConsensusHash".to_string());
    sfield_names.insert(327704, "CheckID".to_string());
    sfield_names.insert(327705, "ValidatedHash".to_string());
    sfield_names.insert(327706, "PreviousPageMin".to_string());
    sfield_names.insert(327707, "NextPageMin".to_string());
    sfield_names.insert(327708, "NFTokenBuyOffer".to_string());
    sfield_names.insert(327709, "NFTokenSellOffer".to_string());
    sfield_names.insert(327710, "HookStateKey".to_string());
    sfield_names.insert(327711, "HookHash".to_string());
    sfield_names.insert(327712, "HookNamespace".to_string());
    sfield_names.insert(327713, "HookSetTxnID".to_string());
    sfield_names.insert(327714, "DomainID".to_string());
    sfield_names.insert(327937, "hash".to_string());
    sfield_names.insert(327938, "index".to_string());
    sfield_names.insert(393217, "Amount".to_string());
    sfield_names.insert(393218, "Balance".to_string());
    sfield_names.insert(393219, "LimitAmount".to_string());
    sfield_names.insert(393220, "TakerPays".to_string());
    sfield_names.insert(393221, "TakerGets".to_string());
    sfield_names.insert(393222, "LowLimit".to_string());
    sfield_names.insert(393223, "HighLimit".to_string());
    sfield_names.insert(393224, "Fee".to_string());
    sfield_names.insert(393225, "SendMax".to_string());
    sfield_names.insert(393226, "DeliverMin".to_string());
    sfield_names.insert(393227, "Amount2".to_string());
    sfield_names.insert(393228, "BidMin".to_string());
    sfield_names.insert(393229, "BidMax".to_string());
    sfield_names.insert(393232, "MinimumOffer".to_string());
    sfield_names.insert(393233, "RippleEscrow".to_string());
    sfield_names.insert(393234, "DeliveredAmount".to_string());
    sfield_names.insert(393235, "NFTokenBrokerFee".to_string());
    sfield_names.insert(393238, "BaseFeeDrops".to_string());
    sfield_names.insert(393239, "ReserveBaseDrops".to_string());
    sfield_names.insert(393240, "ReserveIncrementDrops".to_string());
    sfield_names.insert(393241, "LPTokenOut".to_string());
    sfield_names.insert(393242, "LPTokenIn".to_string());
    sfield_names.insert(393243, "EPrice".to_string());
    sfield_names.insert(393244, "Price".to_string());
    sfield_names.insert(393245, "SignatureReward".to_string());
    sfield_names.insert(393246, "MinAccountCreateAmount".to_string());
    sfield_names.insert(393247, "LPTokenBalance".to_string());
    sfield_names.insert(458753, "PublicKey".to_string());
    sfield_names.insert(458754, "MessageKey".to_string());
    sfield_names.insert(458755, "SigningPubKey".to_string());
    sfield_names.insert(458756, "TxnSignature".to_string());
    sfield_names.insert(458757, "URI".to_string());
    sfield_names.insert(458758, "Signature".to_string());
    sfield_names.insert(458759, "Domain".to_string());
    sfield_names.insert(458760, "FundCode".to_string());
    sfield_names.insert(458761, "RemoveCode".to_string());
    sfield_names.insert(458762, "ExpireCode".to_string());
    sfield_names.insert(458763, "CreateCode".to_string());
    sfield_names.insert(458764, "MemoType".to_string());
    sfield_names.insert(458765, "MemoData".to_string());
    sfield_names.insert(458766, "MemoFormat".to_string());
    sfield_names.insert(458768, "Fulfillment".to_string());
    sfield_names.insert(458769, "Condition".to_string());
    sfield_names.insert(458770, "MasterSignature".to_string());
    sfield_names.insert(458771, "UNLModifyValidator".to_string());
    sfield_names.insert(458772, "ValidatorToDisable".to_string());
    sfield_names.insert(458773, "ValidatorToReEnable".to_string());
    sfield_names.insert(458774, "HookStateData".to_string());
    sfield_names.insert(458775, "HookReturnString".to_string());
    sfield_names.insert(458776, "HookParameterName".to_string());
    sfield_names.insert(458777, "HookParameterValue".to_string());
    sfield_names.insert(458778, "DIDDocument".to_string());
    sfield_names.insert(458779, "Data".to_string());
    sfield_names.insert(458780, "AssetClass".to_string());
    sfield_names.insert(458781, "Provider".to_string());
    sfield_names.insert(458782, "MPTokenMetadata".to_string());
    sfield_names.insert(458783, "CredentialType".to_string());
    sfield_names.insert(458784, "FinishFunction".to_string());
    sfield_names.insert(524289, "Account".to_string());
    sfield_names.insert(524290, "Owner".to_string());
    sfield_names.insert(524291, "Destination".to_string());
    sfield_names.insert(524292, "Issuer".to_string());
    sfield_names.insert(524293, "Authorize".to_string());
    sfield_names.insert(524294, "Unauthorize".to_string());
    sfield_names.insert(524296, "RegularKey".to_string());
    sfield_names.insert(524297, "NFTokenMinter".to_string());
    sfield_names.insert(524298, "EmitCallback".to_string());
    sfield_names.insert(524299, "Holder".to_string());
    sfield_names.insert(524304, "HookAccount".to_string());
    sfield_names.insert(524306, "OtherChainSource".to_string());
    sfield_names.insert(524307, "OtherChainDestination".to_string());
    sfield_names.insert(524308, "AttestationSignerAccount".to_string());
    sfield_names.insert(524309, "AttestationRewardAccount".to_string());
    sfield_names.insert(524310, "LockingChainDoor".to_string());
    sfield_names.insert(524311, "IssuingChainDoor".to_string());
    sfield_names.insert(524312, "Subject".to_string());
    sfield_names.insert(589825, "Number".to_string());
    sfield_names.insert(917506, "TransactionMetaData".to_string());
    sfield_names.insert(917507, "CreatedNode".to_string());
    sfield_names.insert(917508, "DeletedNode".to_string());
    sfield_names.insert(917509, "ModifiedNode".to_string());
    sfield_names.insert(917510, "PreviousFields".to_string());
    sfield_names.insert(917511, "FinalFields".to_string());
    sfield_names.insert(917512, "NewFields".to_string());
    sfield_names.insert(917513, "TemplateEntry".to_string());
    sfield_names.insert(917514, "Memo".to_string());
    sfield_names.insert(917515, "SignerEntry".to_string());
    sfield_names.insert(917516, "NFToken".to_string());
    sfield_names.insert(917517, "EmitDetails".to_string());
    sfield_names.insert(917518, "Hook".to_string());
    sfield_names.insert(917520, "Signer".to_string());
    sfield_names.insert(917522, "Majority".to_string());
    sfield_names.insert(917523, "DisabledValidator".to_string());
    sfield_names.insert(917524, "EmittedTxn".to_string());
    sfield_names.insert(917525, "HookExecution".to_string());
    sfield_names.insert(917526, "HookDefinition".to_string());
    sfield_names.insert(917527, "HookParameter".to_string());
    sfield_names.insert(917528, "HookGrant".to_string());
    sfield_names.insert(917529, "VoteEntry".to_string());
    sfield_names.insert(917530, "AuctionSlot".to_string());
    sfield_names.insert(917531, "AuthAccount".to_string());
    sfield_names.insert(917532, "XChainClaimProofSig".to_string());
    sfield_names.insert(917533, "XChainCreateAccountProofSig".to_string());
    sfield_names.insert(
        917534,
        "XChainClaimAttestationCollectionElement".to_string(),
    );
    sfield_names.insert(
        917535,
        "XChainCreateAccountAttestationCollectionElement".to_string(),
    );
    sfield_names.insert(917536, "PriceData".to_string());
    sfield_names.insert(917537, "Credential".to_string());
    sfield_names.insert(983043, "Signers".to_string());
    sfield_names.insert(983044, "SignerEntries".to_string());
    sfield_names.insert(983045, "Template".to_string());
    sfield_names.insert(983046, "Necessary".to_string());
    sfield_names.insert(983047, "Sufficient".to_string());
    sfield_names.insert(983048, "AffectedNodes".to_string());
    sfield_names.insert(983049, "Memos".to_string());
    sfield_names.insert(983050, "NFTokens".to_string());
    sfield_names.insert(983051, "Hooks".to_string());
    sfield_names.insert(983052, "VoteSlots".to_string());
    sfield_names.insert(983056, "Majorities".to_string());
    sfield_names.insert(983057, "DisabledValidators".to_string());
    sfield_names.insert(983058, "HookExecutions".to_string());
    sfield_names.insert(983059, "HookParameters".to_string());
    sfield_names.insert(983060, "HookGrants".to_string());
    sfield_names.insert(983061, "XChainClaimAttestations".to_string());
    sfield_names.insert(983062, "XChainCreateAccountAttestations".to_string());
    sfield_names.insert(983064, "PriceDataSeries".to_string());
    sfield_names.insert(983065, "AuthAccounts".to_string());
    sfield_names.insert(983066, "AuthorizeCredentials".to_string());
    sfield_names.insert(983067, "UnauthorizeCredentials".to_string());
    sfield_names.insert(983068, "AcceptedCredentials".to_string());
    sfield_names.insert(1048577, "CloseResolution".to_string());
    sfield_names.insert(1048578, "Method".to_string());
    sfield_names.insert(1048579, "TransactionResult".to_string());
    sfield_names.insert(1048580, "Scale".to_string());
    sfield_names.insert(1048581, "AssetScale".to_string());
    sfield_names.insert(1048592, "TickSize".to_string());
    sfield_names.insert(1048593, "UNLModifyDisabling".to_string());
    sfield_names.insert(1048594, "HookResult".to_string());
    sfield_names.insert(1048595, "WasLockingChainSend".to_string());
    sfield_names.insert(1114113, "TakerPaysCurrency".to_string());
    sfield_names.insert(1114114, "TakerPaysIssuer".to_string());
    sfield_names.insert(1114115, "TakerGetsCurrency".to_string());
    sfield_names.insert(1114116, "TakerGetsIssuer".to_string());
    sfield_names.insert(1179649, "Paths".to_string());
    sfield_names.insert(1245185, "Indexes".to_string());
    sfield_names.insert(1245186, "Hashes".to_string());
    sfield_names.insert(1245187, "Amendments".to_string());
    sfield_names.insert(1245188, "NFTokenOffers".to_string());
    sfield_names.insert(1245189, "CredentialIDs".to_string());
    sfield_names.insert(1376257, "MPTokenIssuanceID".to_string());
    sfield_names.insert(1572865, "LockingChainIssue".to_string());
    sfield_names.insert(1572866, "IssuingChainIssue".to_string());
    sfield_names.insert(1572867, "Asset".to_string());
    sfield_names.insert(1572868, "Asset2".to_string());
    sfield_names.insert(1638401, "XChainBridge".to_string());
    sfield_names.insert(1703937, "BaseAsset".to_string());
    sfield_names.insert(1703938, "QuoteAsset".to_string());
    sfield_names.insert(655425793, "Transaction".to_string());
    sfield_names.insert(655491329, "LedgerEntry".to_string());
    sfield_names.insert(655556865, "Validation".to_string());
    sfield_names.insert(655622401, "Metadata".to_string());
    sfield_names
}
