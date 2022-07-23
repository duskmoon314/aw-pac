#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - TV Encoder DAC MAP Register"]
    pub tve_dac_map: crate::Reg<tve_dac_map::TVE_DAC_MAP_SPEC>,
    #[doc = "0x24 - TV Encoder DAC STAUTS Register"]
    pub tve_dac_status: crate::Reg<tve_dac_status::TVE_DAC_STATUS_SPEC>,
    #[doc = "0x28 - TV Encoder DAC CFG0 Register"]
    pub tve_dac_cfg0: crate::Reg<tve_dac_cfg0::TVE_DAC_CFG0_SPEC>,
    #[doc = "0x2c - TV Encoder DAC CFG1 Register"]
    pub tve_dac_cfg1: crate::Reg<tve_dac_cfg1::TVE_DAC_CFG1_SPEC>,
    #[doc = "0x30 - TV Encoder DAC CFG2 Register"]
    pub tve_dac_cfg2: crate::Reg<tve_dac_cfg2::TVE_DAC_CFG2_SPEC>,
    #[doc = "0x34 - TV Encoder DAC CFG2 Register"]
    pub tve_dac_cfg3: crate::Reg<tve_dac_cfg3::TVE_DAC_CFG3_SPEC>,
    _reserved6: [u8; 0xb8],
    #[doc = "0xf0 - TV Encoder DAC TEST Register"]
    pub tve_dac_test: crate::Reg<tve_dac_test::TVE_DAC_TEST_SPEC>,
}
#[doc = "tve_dac_map register accessor: an alias for `Reg<TVE_DAC_MAP_SPEC>`"]
pub type TVE_DAC_MAP = crate::Reg<tve_dac_map::TVE_DAC_MAP_SPEC>;
#[doc = "TV Encoder DAC MAP Register"]
pub mod tve_dac_map;
#[doc = "tve_dac_status register accessor: an alias for `Reg<TVE_DAC_STATUS_SPEC>`"]
pub type TVE_DAC_STATUS = crate::Reg<tve_dac_status::TVE_DAC_STATUS_SPEC>;
#[doc = "TV Encoder DAC STAUTS Register"]
pub mod tve_dac_status;
#[doc = "tve_dac_cfg0 register accessor: an alias for `Reg<TVE_DAC_CFG0_SPEC>`"]
pub type TVE_DAC_CFG0 = crate::Reg<tve_dac_cfg0::TVE_DAC_CFG0_SPEC>;
#[doc = "TV Encoder DAC CFG0 Register"]
pub mod tve_dac_cfg0;
#[doc = "tve_dac_cfg1 register accessor: an alias for `Reg<TVE_DAC_CFG1_SPEC>`"]
pub type TVE_DAC_CFG1 = crate::Reg<tve_dac_cfg1::TVE_DAC_CFG1_SPEC>;
#[doc = "TV Encoder DAC CFG1 Register"]
pub mod tve_dac_cfg1;
#[doc = "tve_dac_cfg2 register accessor: an alias for `Reg<TVE_DAC_CFG2_SPEC>`"]
pub type TVE_DAC_CFG2 = crate::Reg<tve_dac_cfg2::TVE_DAC_CFG2_SPEC>;
#[doc = "TV Encoder DAC CFG2 Register"]
pub mod tve_dac_cfg2;
#[doc = "tve_dac_cfg3 register accessor: an alias for `Reg<TVE_DAC_CFG3_SPEC>`"]
pub type TVE_DAC_CFG3 = crate::Reg<tve_dac_cfg3::TVE_DAC_CFG3_SPEC>;
#[doc = "TV Encoder DAC CFG2 Register"]
pub mod tve_dac_cfg3;
#[doc = "tve_dac_test register accessor: an alias for `Reg<TVE_DAC_TEST_SPEC>`"]
pub type TVE_DAC_TEST = crate::Reg<tve_dac_test::TVE_DAC_TEST_SPEC>;
#[doc = "TV Encoder DAC TEST Register"]
pub mod tve_dac_test;
