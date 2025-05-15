#![allow(dead_code)]

use std::collections::HashMap;

pub type Bytes = Vec<u8>;
pub type Hash256 = Vec<u8>; //TODO use [u8; 32]
pub type Keylet = Hash256; //TODO use hash or type together

pub enum DataSource {
    Tx,
    CurrentLedgerObj,
    KeyletLedgerObj(Keylet),
}

#[derive(Debug)]
pub struct MockData {
    tx: serde_json::Value,
    hosting_ledger_obj: serde_json::Value,
    header: serde_json::Value,
    ledger: HashMap<Keylet, serde_json::Value>,
    field_names: HashMap<i32, String>,
}

impl MockData {
    pub fn new(
        tx_str: &String,
        hosting_ledger_obj_str: &String,
        header_str: &String,
        ledger_str: &String,
    ) -> Self {
        let tx = serde_json::from_str(tx_str).expect("Tx JSON bad formatted");
        let hosting_ledger_obj = serde_json::from_str(hosting_ledger_obj_str)
            .expect("Hosting ledger object JSON bad formatted");
        let header = serde_json::from_str(header_str).expect("Ledger header JSON bad formatted");
        let ledger = serde_json::from_str(ledger_str).expect("Ledger JSON bad formatted");
        let field_names = polulate_field_names();

        MockData {
            tx,
            hosting_ledger_obj,
            header,
            ledger,
            field_names,
        }
    }

    pub fn obj_exist(&self, keylet: &Keylet) -> bool {
        self.ledger.get(keylet).is_some()
    }

    #[inline]
    pub fn get_field_name(&self, field_id: i32) -> Option<String> {
        self.field_names.get(&field_id).cloned()
    }

    pub fn get_field_value(
        &self,
        source: DataSource,
        idx_fields: Vec<i32>,
    ) -> Option<&serde_json::Value> {
        let mut curr = match source {
            DataSource::Tx => &self.tx,
            DataSource::CurrentLedgerObj => &self.hosting_ledger_obj,
            DataSource::KeyletLedgerObj(obj_hash) => self.ledger.get(&obj_hash)?,
        };

        for idx_field in idx_fields {
            if curr.is_array() {
                curr = curr.as_array().unwrap().get(idx_field as usize)?;
            } else {
                curr = curr.get(self.get_field_name(idx_field)?)?;
            }
        }
        Some(curr)
    }

    pub fn get_array_len(&self, source: DataSource, idx_fields: Vec<i32>) -> Option<usize> {
        let value = self.get_field_value(source, idx_fields)?;
        if value.is_array() {
            Some(value.as_array()?.len())
        } else {
            None
        }
    }

    pub fn get_ledger_sqn(&self) -> Option<&serde_json::Value> {
        self.header.get("ledger_index")
    }

    pub fn get_parent_ledger_time(&self) -> Option<&serde_json::Value> {
        self.header.get("parent_close_time")
    }

    pub fn get_parent_ledger_hash(&self) -> Option<&serde_json::Value> {
        self.header.get("parent_hash")
    }

    pub fn set_current_ledger_obj_data(&mut self, data: Vec<u8>) {
        self.hosting_ledger_obj["data"] = serde_json::Value::from(data);
    }
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
    sfield_names.insert(917534, "XChainClaimAttestationCollectionElement".to_string());
    sfield_names.insert(917535, "XChainCreateAccountAttestationCollectionElement".to_string());
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
