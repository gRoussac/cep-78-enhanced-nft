pub const ARG_COLLECTION_NAME: &str = "collection_name";
pub const ARG_COLLECTION_SYMBOL: &str = "collection_symbol";
pub const ARG_TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
pub const ARG_TOKEN_ID: &str = "token_id";
pub const ARG_TOKEN_HASH: &str = "token_hash";
pub const ARG_TOKEN_OWNER: &str = "token_owner";
pub const ARG_TARGET_KEY: &str = "target_key";
pub const ARG_SOURCE_KEY: &str = "source_key";
pub const ARG_ALLOW_MINTING: &str = "allow_minting";
pub const ARG_MINTING_MODE: &str = "minting_mode";
pub const ARG_TOKEN_META_DATA: &str = "token_meta_data";
pub const ARG_APPROVE_ALL: &str = "approve_all";
pub const ARG_OPERATOR: &str = "operator";
pub const ARG_OWNERSHIP_MODE: &str = "ownership_mode";
pub const ARG_HOLDER_MODE: &str = "holder_mode";
pub const ARG_WHITELIST_MODE: &str = "whitelist_mode";
pub const ARG_NFT_KIND: &str = "nft_kind";
pub const ARG_JSON_SCHEMA: &str = "json_schema";
pub const ARG_RECEIPT_NAME: &str = "receipt_name";
pub const ARG_CONTRACT_WHITELIST: &str = "contract_whitelist";
pub const ARG_ACCOUNTS_WHITELIST: &str = "accounts_whitelist";
pub const ARG_NFT_METADATA_KIND: &str = "nft_metadata_kind";
pub const ARG_IDENTIFIER_MODE: &str = "identifier_mode";
pub const ARG_METADATA_MUTABILITY: &str = "metadata_mutability";
pub const ARG_BURN_MODE: &str = "burn_mode";
pub const ARG_OWNER_LOOKUP_MODE: &str = "owner_reverse_lookup_mode";
pub const ARG_NFT_PACKAGE_HASH: &str = "cep78_package_key";
pub const ARG_NAMED_KEY_CONVENTION: &str = "named_key_convention";
pub const ARG_ACCESS_KEY_NAME_1_0_0: &str = "access_key_name";
pub const ARG_HASH_KEY_NAME_1_0_0: &str = "hash_key_name";
pub const ARG_EVENTS_MODE: &str = "events_mode";

pub const OPERATOR: &str = "operator";
pub const NUMBER_OF_MINTED_TOKENS: &str = "number_of_minted_tokens";
pub const INSTALLER: &str = "installer";
pub const JSON_SCHEMA: &str = "json_schema";
pub const HASH_KEY_NAME_1_0_0: &str = "nft_contract_package";
pub const ACCESS_KEY_NAME_1_0_0: &str = "nft_contract_package_access";
pub const HASH_KEY_NAME_PREFIX: &str = "cep78_contract_package";
pub const ACCESS_KEY_NAME_PREFIX: &str = "cep78_contract_package_access_";
pub const CONTRACT_NAME_PREFIX: &str = "cep78_contract_hash_";
pub const CONTRACT_VERSION_PREFIX: &str = "cep78_contract_version_";
pub const COLLECTION_NAME: &str = "collection_name";
pub const COLLECTION_SYMBOL: &str = "collection_symbol";
pub const TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
pub const OWNERSHIP_MODE: &str = "ownership_mode";
pub const NFT_KIND: &str = "nft_kind";
pub const ALLOW_MINTING: &str = "allow_minting";
pub const MINTING_MODE: &str = "minting_mode";
pub const HOLDER_MODE: &str = "holder_mode";
pub const WHITELIST_MODE: &str = "whitelist_mode";
pub const TOKEN_OWNERS: &str = "token_owners";
pub const TOKEN_ISSUERS: &str = "token_issuers";
pub const OWNED_TOKENS: &str = "owned_tokens";
pub const BURNT_TOKENS: &str = "burnt_tokens";
pub const TOKEN_COUNTS: &str = "balances";
pub const CONTRACT_WHITELIST: &str = "contract_whitelist";
pub const ACCOUNTS_WHITELIST: &str = "accounts_whitelist";
pub const RECEIPT_NAME: &str = "receipt_name";
pub const NFT_METADATA_KIND: &str = "nft_metadata_kind";
pub const IDENTIFIER_MODE: &str = "identifier_mode";
pub const BURN_MODE: &str = "burn_mode";
pub const METADATA_MUTABILITY: &str = "metadata_mutability";
pub const METADATA_CUSTOM_VALIDATED: &str = "metadata_custom_validated";
pub const METADATA_CEP78: &str = "metadata_cep78";
pub const METADATA_NFT721: &str = "metadata_nft721";
pub const METADATA_RAW: &str = "metadata_raw";
pub const PAGE_TABLE: &str = "page_table";
pub const HASH_BY_INDEX: &str = "hash_by_index";
pub const INDEX_BY_HASH: &str = "index_by_hash";
pub const CEP78_PREFIX: &str = "cep78_";
pub const PAGE_DICTIONARY_PREFIX: &str = "page_";
pub const PAGE_LIMIT: &str = "page_limit";
pub const UNMATCHED_HASH_COUNT: &str = "unmatched_hash_count";
pub const MIGRATION_FLAG: &str = "migration_flag";
pub const REPORTING_MODE: &str = "reporting_mode";
pub const EVENTS: &str = "events";
pub const EVENT_ID_TRACKER: &str = "id_tracker";
pub const EVENTS_MODE: &str = "events_mode";

pub const ENTRY_POINT_INIT: &str = "init";
pub const ENTRY_POINT_SET_VARIABLES: &str = "set_variables";
pub const ENTRY_POINT_MINT: &str = "mint";
pub const ENTRY_POINT_BURN: &str = "burn";
pub const ENTRY_POINT_TRANSFER: &str = "transfer";
pub const ENTRY_POINT_APPROVE: &str = "approve";
pub const ENTRY_POINT_BALANCE_OF: &str = "balance_of";
pub const ENTRY_POINT_OWNER_OF: &str = "owner_of";
pub const ENTRY_POINT_GET_APPROVED: &str = "get_approved";
pub const ENTRY_POINT_METADATA: &str = "metadata";
pub const ENTRY_POINT_SET_APPROVE_FOR_ALL: &str = "set_approval_for_all";
pub const ENTRY_POINT_SET_TOKEN_METADATA: &str = "set_token_metadata";
pub const ENTRY_POINT_MIGRATE: &str = "migrate";
pub const ENTRY_POINT_UPDATED_RECEIPTS: &str = "updated_receipts";
pub const ENTRY_POINT_REGISTER_OWNER: &str = "register_owner";

// The cap on the amount of tokens within a given CEP-78 collection.
pub const MAX_TOTAL_TOKEN_SUPPLY: u64 = 1_000_000u64;
