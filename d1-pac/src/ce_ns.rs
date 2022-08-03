#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Task Descriptor Address"]
    pub ce_tda: CE_TDA,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Interrupt Control Register"]
    pub ce_icr: CE_ICR,
    #[doc = "0x0c - Interrupt Status Register"]
    pub ce_isr: CE_ISR,
    #[doc = "0x10 - Task Load Register"]
    pub ce_tlr: CE_TLR,
    #[doc = "0x14 - Task Status Register"]
    pub ce_tsr: CE_TSR,
    #[doc = "0x18 - Error Status Register"]
    pub ce_esr: CE_ESR,
    _reserved6: [u8; 0x08],
    #[doc = "0x24 - Current Source Address Register"]
    pub ce_csa: CE_CSA,
    #[doc = "0x28 - Current Destination Address Register"]
    pub ce_cda: CE_CDA,
    #[doc = "0x2c - Throughput Register"]
    pub ce_tpr: CE_TPR,
}
#[doc = "ce_tda (rw) register accessor: an alias for `Reg<CE_TDA_SPEC>`"]
pub type CE_TDA = crate::Reg<ce_tda::CE_TDA_SPEC>;
#[doc = "Task Descriptor Address"]
pub mod ce_tda;
#[doc = "ce_icr (rw) register accessor: an alias for `Reg<CE_ICR_SPEC>`"]
pub type CE_ICR = crate::Reg<ce_icr::CE_ICR_SPEC>;
#[doc = "Interrupt Control Register"]
pub mod ce_icr;
#[doc = "ce_isr (rw) register accessor: an alias for `Reg<CE_ISR_SPEC>`"]
pub type CE_ISR = crate::Reg<ce_isr::CE_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod ce_isr;
#[doc = "ce_tlr (rw) register accessor: an alias for `Reg<CE_TLR_SPEC>`"]
pub type CE_TLR = crate::Reg<ce_tlr::CE_TLR_SPEC>;
#[doc = "Task Load Register"]
pub mod ce_tlr;
#[doc = "ce_tsr (rw) register accessor: an alias for `Reg<CE_TSR_SPEC>`"]
pub type CE_TSR = crate::Reg<ce_tsr::CE_TSR_SPEC>;
#[doc = "Task Status Register"]
pub mod ce_tsr;
#[doc = "ce_esr (rw) register accessor: an alias for `Reg<CE_ESR_SPEC>`"]
pub type CE_ESR = crate::Reg<ce_esr::CE_ESR_SPEC>;
#[doc = "Error Status Register"]
pub mod ce_esr;
#[doc = "ce_csa (rw) register accessor: an alias for `Reg<CE_CSA_SPEC>`"]
pub type CE_CSA = crate::Reg<ce_csa::CE_CSA_SPEC>;
#[doc = "Current Source Address Register"]
pub mod ce_csa;
#[doc = "ce_cda (rw) register accessor: an alias for `Reg<CE_CDA_SPEC>`"]
pub type CE_CDA = crate::Reg<ce_cda::CE_CDA_SPEC>;
#[doc = "Current Destination Address Register"]
pub mod ce_cda;
#[doc = "ce_tpr (rw) register accessor: an alias for `Reg<CE_TPR_SPEC>`"]
pub type CE_TPR = crate::Reg<ce_tpr::CE_TPR_SPEC>;
#[doc = "Throughput Register"]
pub mod ce_tpr;
