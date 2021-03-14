pub const CURVE_REGISTRY_ADDRESS: &str = "0x7D86446dDb609eD0F5f8684AcF30380a356b2B4c";
pub const CURVE_REGISTRY_ABI: &str = include_str!("resources/abi/curve_registry.abi.json");
pub const CURVE_COMP_POOL_ADDRESS: &str = "0xA2B47E3D5c44877cca798226B7B8118F9BFb7A56";

// FIXME: We should derive all addresses through the registry
pub const CURVE_LP_TOKEN_ADDRESS: &str = "0x845838DF265Dcd2c412A1Dc9e959c7d08537f8a2";
pub const CURVE_COMP_LP_TOKEN_ABI: &str =
    include_str!("resources/abi/curve_comp_lp_token.abi.json");

pub const YEARN_CRV_COMP_VAULT_ADDRESS: &str = "0x629c759D1E83eFbF63d84eb3868B564d9521C129";
pub const YEARN_VAULT_V1_ABI: &str = include_str!("resources/abi/yearn_vault.abi.json");
