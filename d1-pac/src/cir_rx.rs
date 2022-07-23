#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CIR Control Register"]
    pub cir_ctl: crate::Reg<cir_ctl::CIR_CTL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - CIR Receiver Pulse Configure Register"]
    pub cir_rxpcfg: crate::Reg<cir_rxpcfg::CIR_RXPCFG_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - CIR Receiver FIFO Register"]
    pub cir_rxfifo: crate::Reg<cir_rxfifo::CIR_RXFIFO_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x2c - CIR Receiver Interrupt Control Register"]
    pub cir_rxint: crate::Reg<cir_rxint::CIR_RXINT_SPEC>,
    #[doc = "0x30 - CIR Receiver Status Register"]
    pub cir_rxsta: crate::Reg<cir_rxsta::CIR_RXSTA_SPEC>,
    #[doc = "0x34 - CIR Receiver Configure Register"]
    pub cir_rxcfg: crate::Reg<cir_rxcfg::CIR_RXCFG_SPEC>,
}
#[doc = "cir_ctl register accessor: an alias for `Reg<CIR_CTL_SPEC>`"]
pub type CIR_CTL = crate::Reg<cir_ctl::CIR_CTL_SPEC>;
#[doc = "CIR Control Register"]
pub mod cir_ctl;
#[doc = "cir_rxpcfg register accessor: an alias for `Reg<CIR_RXPCFG_SPEC>`"]
pub type CIR_RXPCFG = crate::Reg<cir_rxpcfg::CIR_RXPCFG_SPEC>;
#[doc = "CIR Receiver Pulse Configure Register"]
pub mod cir_rxpcfg;
#[doc = "cir_rxfifo register accessor: an alias for `Reg<CIR_RXFIFO_SPEC>`"]
pub type CIR_RXFIFO = crate::Reg<cir_rxfifo::CIR_RXFIFO_SPEC>;
#[doc = "CIR Receiver FIFO Register"]
pub mod cir_rxfifo;
#[doc = "cir_rxint register accessor: an alias for `Reg<CIR_RXINT_SPEC>`"]
pub type CIR_RXINT = crate::Reg<cir_rxint::CIR_RXINT_SPEC>;
#[doc = "CIR Receiver Interrupt Control Register"]
pub mod cir_rxint;
#[doc = "cir_rxsta register accessor: an alias for `Reg<CIR_RXSTA_SPEC>`"]
pub type CIR_RXSTA = crate::Reg<cir_rxsta::CIR_RXSTA_SPEC>;
#[doc = "CIR Receiver Status Register"]
pub mod cir_rxsta;
#[doc = "cir_rxcfg register accessor: an alias for `Reg<CIR_RXCFG_SPEC>`"]
pub type CIR_RXCFG = crate::Reg<cir_rxcfg::CIR_RXCFG_SPEC>;
#[doc = "CIR Receiver Configure Register"]
pub mod cir_rxcfg;
