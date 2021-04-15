pub const CURVE_REGISTRY_ADDRESS: &str = "0x7D86446dDb609eD0F5f8684AcF30380a356b2B4c";
pub const CURVE_REGISTRY_ABI: &str = include_str!("resources/abi/curve_registry.abi.json");
pub const CURVE_COMP_POOL_ADDRESS: &str = "0xA2B47E3D5c44877cca798226B7B8118F9BFb7A56";
pub const CURVE_SAAVE_POOL_ADDRESS: &str = "0xEB16Ae0052ed37f479f7fe63849198Df1765a733";

// FIXME: We should derive all addresses through registry.get_lp_token(pool_address)
pub const CURVE_COMP_LP_TOKEN_ADDRESS: &str = "0x845838DF265Dcd2c412A1Dc9e959c7d08537f8a2";
pub const CURVE_COMP_LP_TOKEN_ABI: &str =
    include_str!("resources/abi/curve_comp_lp_token.abi.json");
pub const CURVE_SAAVE_LP_TOKEN_ADDRESS: &str = "0x02d341CcB60fAaf662bC0554d13778015d1b285C";
pub const CURVE_SAAVE_LP_TOKEN_ABI: &str =
    include_str!("resources/abi/curve_saave_lp_token.abi.json");

pub const YEARN_CRV_COMP_VAULT_ADDRESS: &str = "0x629c759D1E83eFbF63d84eb3868B564d9521C129";
pub const YEARN_VAULT_V1_ABI: &str = include_str!("resources/abi/yearn_vault.abi.json");
pub const YEARN_CRV_SAAVE_VAULT_ADDRESS: &str = "0xBacB69571323575C6a5A3b4F9EEde1DC7D31FBc1";

pub const HOUR_IN_SEC: u64 = 60 * 60;
pub const DAY_IN_SEC: u64 = HOUR_IN_SEC * 24;
pub const WEEK_IN_SEC: u64 = DAY_IN_SEC * 7;
pub const MONTH_IN_SEC: u64 = DAY_IN_SEC * 30;
pub const YEAR_IN_SEC: u64 = DAY_IN_SEC * 365;
