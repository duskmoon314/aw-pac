#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    tve_dac_map: TVE_DAC_MAP,
    tve_dac_status: TVE_DAC_STATUS,
    tve_dac_cfg0: TVE_DAC_CFG0,
    tve_dac_cfg1: TVE_DAC_CFG1,
    tve_dac_cfg2: TVE_DAC_CFG2,
    tve_dac_cfg3: TVE_DAC_CFG3,
    _reserved6: [u8; 0xb8],
    tve_dac_test: TVE_DAC_TEST,
}
impl RegisterBlock {
    #[doc = "0x20 - TV Encoder DAC MAP Register"]
    #[inline(always)]
    pub const fn tve_dac_map(&self) -> &TVE_DAC_MAP {
        &self.tve_dac_map
    }
    #[doc = "0x24 - TV Encoder DAC STAUTS Register"]
    #[inline(always)]
    pub const fn tve_dac_status(&self) -> &TVE_DAC_STATUS {
        &self.tve_dac_status
    }
    #[doc = "0x28 - TV Encoder DAC CFG0 Register"]
    #[inline(always)]
    pub const fn tve_dac_cfg0(&self) -> &TVE_DAC_CFG0 {
        &self.tve_dac_cfg0
    }
    #[doc = "0x2c - TV Encoder DAC CFG1 Register"]
    #[inline(always)]
    pub const fn tve_dac_cfg1(&self) -> &TVE_DAC_CFG1 {
        &self.tve_dac_cfg1
    }
    #[doc = "0x30 - TV Encoder DAC CFG2 Register"]
    #[inline(always)]
    pub const fn tve_dac_cfg2(&self) -> &TVE_DAC_CFG2 {
        &self.tve_dac_cfg2
    }
    #[doc = "0x34 - TV Encoder DAC CFG2 Register"]
    #[inline(always)]
    pub const fn tve_dac_cfg3(&self) -> &TVE_DAC_CFG3 {
        &self.tve_dac_cfg3
    }
    #[doc = "0xf0 - TV Encoder DAC TEST Register"]
    #[inline(always)]
    pub const fn tve_dac_test(&self) -> &TVE_DAC_TEST {
        &self.tve_dac_test
    }
}
#[doc = "tve_dac_map (rw) register accessor: TV Encoder DAC MAP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_map`] module"]
pub type TVE_DAC_MAP = crate::Reg<tve_dac_map::TVE_DAC_MAP_SPEC>;
#[doc = "TV Encoder DAC MAP Register"]
pub mod tve_dac_map;
#[doc = "tve_dac_status (rw) register accessor: TV Encoder DAC STAUTS Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_status`] module"]
pub type TVE_DAC_STATUS = crate::Reg<tve_dac_status::TVE_DAC_STATUS_SPEC>;
#[doc = "TV Encoder DAC STAUTS Register"]
pub mod tve_dac_status;
#[doc = "tve_dac_cfg0 (rw) register accessor: TV Encoder DAC CFG0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_cfg0`] module"]
pub type TVE_DAC_CFG0 = crate::Reg<tve_dac_cfg0::TVE_DAC_CFG0_SPEC>;
#[doc = "TV Encoder DAC CFG0 Register"]
pub mod tve_dac_cfg0;
#[doc = "tve_dac_cfg1 (rw) register accessor: TV Encoder DAC CFG1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_cfg1`] module"]
pub type TVE_DAC_CFG1 = crate::Reg<tve_dac_cfg1::TVE_DAC_CFG1_SPEC>;
#[doc = "TV Encoder DAC CFG1 Register"]
pub mod tve_dac_cfg1;
#[doc = "tve_dac_cfg2 (rw) register accessor: TV Encoder DAC CFG2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_cfg2`] module"]
pub type TVE_DAC_CFG2 = crate::Reg<tve_dac_cfg2::TVE_DAC_CFG2_SPEC>;
#[doc = "TV Encoder DAC CFG2 Register"]
pub mod tve_dac_cfg2;
#[doc = "tve_dac_cfg3 (rw) register accessor: TV Encoder DAC CFG2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_cfg3`] module"]
pub type TVE_DAC_CFG3 = crate::Reg<tve_dac_cfg3::TVE_DAC_CFG3_SPEC>;
#[doc = "TV Encoder DAC CFG2 Register"]
pub mod tve_dac_cfg3;
#[doc = "tve_dac_test (rw) register accessor: TV Encoder DAC TEST Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac_test`] module"]
pub type TVE_DAC_TEST = crate::Reg<tve_dac_test::TVE_DAC_TEST_SPEC>;
#[doc = "TV Encoder DAC TEST Register"]
pub mod tve_dac_test;
