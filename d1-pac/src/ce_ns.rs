#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ce_tda: CE_TDA,
    _reserved1: [u8; 0x04],
    ce_icr: CE_ICR,
    ce_isr: CE_ISR,
    ce_tlr: CE_TLR,
    ce_tsr: CE_TSR,
    ce_esr: CE_ESR,
    _reserved6: [u8; 0x08],
    ce_csa: CE_CSA,
    ce_cda: CE_CDA,
    ce_tpr: CE_TPR,
}
impl RegisterBlock {
    #[doc = "0x00 - Task Descriptor Address"]
    #[inline(always)]
    pub const fn ce_tda(&self) -> &CE_TDA {
        &self.ce_tda
    }
    #[doc = "0x08 - Interrupt Control Register"]
    #[inline(always)]
    pub const fn ce_icr(&self) -> &CE_ICR {
        &self.ce_icr
    }
    #[doc = "0x0c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn ce_isr(&self) -> &CE_ISR {
        &self.ce_isr
    }
    #[doc = "0x10 - Task Load Register"]
    #[inline(always)]
    pub const fn ce_tlr(&self) -> &CE_TLR {
        &self.ce_tlr
    }
    #[doc = "0x14 - Task Status Register"]
    #[inline(always)]
    pub const fn ce_tsr(&self) -> &CE_TSR {
        &self.ce_tsr
    }
    #[doc = "0x18 - Error Status Register"]
    #[inline(always)]
    pub const fn ce_esr(&self) -> &CE_ESR {
        &self.ce_esr
    }
    #[doc = "0x24 - Current Source Address Register"]
    #[inline(always)]
    pub const fn ce_csa(&self) -> &CE_CSA {
        &self.ce_csa
    }
    #[doc = "0x28 - Current Destination Address Register"]
    #[inline(always)]
    pub const fn ce_cda(&self) -> &CE_CDA {
        &self.ce_cda
    }
    #[doc = "0x2c - Throughput Register"]
    #[inline(always)]
    pub const fn ce_tpr(&self) -> &CE_TPR {
        &self.ce_tpr
    }
}
#[doc = "ce_tda (rw) register accessor: Task Descriptor Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_tda`] module"]
pub type CE_TDA = crate::Reg<ce_tda::CE_TDA_SPEC>;
#[doc = "Task Descriptor Address"]
pub mod ce_tda;
#[doc = "ce_icr (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_icr`] module"]
pub type CE_ICR = crate::Reg<ce_icr::CE_ICR_SPEC>;
#[doc = "Interrupt Control Register"]
pub mod ce_icr;
#[doc = "ce_isr (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_isr`] module"]
pub type CE_ISR = crate::Reg<ce_isr::CE_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod ce_isr;
#[doc = "ce_tlr (rw) register accessor: Task Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_tlr`] module"]
pub type CE_TLR = crate::Reg<ce_tlr::CE_TLR_SPEC>;
#[doc = "Task Load Register"]
pub mod ce_tlr;
#[doc = "ce_tsr (rw) register accessor: Task Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_tsr`] module"]
pub type CE_TSR = crate::Reg<ce_tsr::CE_TSR_SPEC>;
#[doc = "Task Status Register"]
pub mod ce_tsr;
#[doc = "ce_esr (rw) register accessor: Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_esr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_esr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_esr`] module"]
pub type CE_ESR = crate::Reg<ce_esr::CE_ESR_SPEC>;
#[doc = "Error Status Register"]
pub mod ce_esr;
#[doc = "ce_csa (rw) register accessor: Current Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_csa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_csa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_csa`] module"]
pub type CE_CSA = crate::Reg<ce_csa::CE_CSA_SPEC>;
#[doc = "Current Source Address Register"]
pub mod ce_csa;
#[doc = "ce_cda (rw) register accessor: Current Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_cda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_cda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_cda`] module"]
pub type CE_CDA = crate::Reg<ce_cda::CE_CDA_SPEC>;
#[doc = "Current Destination Address Register"]
pub mod ce_cda;
#[doc = "ce_tpr (rw) register accessor: Throughput Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_tpr`] module"]
pub type CE_TPR = crate::Reg<ce_tpr::CE_TPR_SPEC>;
#[doc = "Throughput Register"]
pub mod ce_tpr;
