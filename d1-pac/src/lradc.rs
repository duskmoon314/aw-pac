#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lradc_ctrl: LRADC_CTRL,
    lradc_intc: LRADC_INTC,
    lradc_ints: LRADC_INTS,
    lradc_data: LRADC_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - LRADC Control Register"]
    #[inline(always)]
    pub const fn lradc_ctrl(&self) -> &LRADC_CTRL {
        &self.lradc_ctrl
    }
    #[doc = "0x04 - LRADC Interrupt Control Register"]
    #[inline(always)]
    pub const fn lradc_intc(&self) -> &LRADC_INTC {
        &self.lradc_intc
    }
    #[doc = "0x08 - LRADC Interrupt Status Register"]
    #[inline(always)]
    pub const fn lradc_ints(&self) -> &LRADC_INTS {
        &self.lradc_ints
    }
    #[doc = "0x0c - LRADC Data Register"]
    #[inline(always)]
    pub const fn lradc_data(&self) -> &LRADC_DATA {
        &self.lradc_data
    }
}
#[doc = "lradc_ctrl (rw) register accessor: LRADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lradc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lradc_ctrl`] module"]
pub type LRADC_CTRL = crate::Reg<lradc_ctrl::LRADC_CTRL_SPEC>;
#[doc = "LRADC Control Register"]
pub mod lradc_ctrl;
#[doc = "lradc_intc (rw) register accessor: LRADC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lradc_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lradc_intc`] module"]
pub type LRADC_INTC = crate::Reg<lradc_intc::LRADC_INTC_SPEC>;
#[doc = "LRADC Interrupt Control Register"]
pub mod lradc_intc;
#[doc = "lradc_ints (rw) register accessor: LRADC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lradc_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lradc_ints`] module"]
pub type LRADC_INTS = crate::Reg<lradc_ints::LRADC_INTS_SPEC>;
#[doc = "LRADC Interrupt Status Register"]
pub mod lradc_ints;
#[doc = "lradc_data (r) register accessor: LRADC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lradc_data`] module"]
pub type LRADC_DATA = crate::Reg<lradc_data::LRADC_DATA_SPEC>;
#[doc = "LRADC Data Register"]
pub mod lradc_data;
