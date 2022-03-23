#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x44],
    #[doc = "0x44 - SIP FLASH Test Mapping Register"]
    pub sip_flash_test_map: crate::Reg<sip_flash_test_map::SIP_FLASH_TEST_MAP_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x50 - Psensor control register"]
    pub ps_ctl: crate::Reg<ps_ctl::PS_CTL_SPEC>,
    #[doc = "0x54 - Psensor counter register"]
    pub ps_cnt: crate::Reg<ps_cnt::PS_CNT_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x60 - FLASH/PSRAM PIN MAPPING Control Register"]
    pub flash_psarm_pin_map: crate::Reg<flash_psarm_pin_map::FLASH_PSARM_PIN_MAP_SPEC>,
    _reserved4: [u8; 0x4c],
    #[doc = "0xb0..0xb8 - General Debug Register"]
    pub general_dbg: [crate::Reg<general_dbg::GENERAL_DBG_SPEC>; 2],
}
#[doc = "SIP_FLASH_TEST_MAP register accessor: an alias for `Reg<SIP_FLASH_TEST_MAP_SPEC>`"]
pub type SIP_FLASH_TEST_MAP = crate::Reg<sip_flash_test_map::SIP_FLASH_TEST_MAP_SPEC>;
#[doc = "SIP FLASH Test Mapping Register"]
pub mod sip_flash_test_map;
#[doc = "PS_CTL register accessor: an alias for `Reg<PS_CTL_SPEC>`"]
pub type PS_CTL = crate::Reg<ps_ctl::PS_CTL_SPEC>;
#[doc = "Psensor control register"]
pub mod ps_ctl;
#[doc = "PS_CNT register accessor: an alias for `Reg<PS_CNT_SPEC>`"]
pub type PS_CNT = crate::Reg<ps_cnt::PS_CNT_SPEC>;
#[doc = "Psensor counter register"]
pub mod ps_cnt;
#[doc = "FLASH_PSARM_PIN_MAP register accessor: an alias for `Reg<FLASH_PSARM_PIN_MAP_SPEC>`"]
pub type FLASH_PSARM_PIN_MAP = crate::Reg<flash_psarm_pin_map::FLASH_PSARM_PIN_MAP_SPEC>;
#[doc = "FLASH/PSRAM PIN MAPPING Control Register"]
pub mod flash_psarm_pin_map;
#[doc = "GENERAL_DBG register accessor: an alias for `Reg<GENERAL_DBG_SPEC>`"]
pub type GENERAL_DBG = crate::Reg<general_dbg::GENERAL_DBG_SPEC>;
#[doc = "General Debug Register"]
pub mod general_dbg;
