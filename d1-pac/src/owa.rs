#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OWA General Control Register"]
    pub owa_gen_ctl: OWA_GEN_CTL,
    #[doc = "0x04 - OWA TX Configuration Register"]
    pub owa_tx_cfig: OWA_TX_CFIG,
    #[doc = "0x08 - OWA RX Configuration Register"]
    pub owa_rx_cfig: OWA_RX_CFIG,
    #[doc = "0x0c - OWA Interrupt Status Register"]
    pub owa_ista: OWA_ISTA,
    #[doc = "0x10 - OWA RXFIFO Register"]
    pub owa_rxfifo: OWA_RXFIFO,
    #[doc = "0x14 - OWA FIFO Control Register"]
    pub owa_fctl: OWA_FCTL,
    #[doc = "0x18 - OWA FIFO Status Register"]
    pub owa_fsta: OWA_FSTA,
    #[doc = "0x1c - OWA Interrupt Control Register"]
    pub owa_int: OWA_INT,
    #[doc = "0x20 - OWA TX FIFO Register"]
    pub owa_tx_fifo: OWA_TX_FIFO,
    #[doc = "0x24 - OWA TX Counter Register"]
    pub owa_tx_cnt: OWA_TX_CNT,
    #[doc = "0x28 - OWA RX Counter Register"]
    pub owa_rx_cnt: OWA_RX_CNT,
    #[doc = "0x2c - OWA TX Channel Status Register0"]
    pub owa_tx_chsta0: OWA_TX_CHSTA0,
    #[doc = "0x30 - OWA TX Channel Status Register1"]
    pub owa_tx_chsta1: OWA_TX_CHSTA1,
    #[doc = "0x34 - OWA RX Channel Status Register0"]
    pub owa_rxchsta0: OWA_RXCHSTA0,
    #[doc = "0x38 - OWA RX Channel Status Register1"]
    pub owa_rxchsta1: OWA_RXCHSTA1,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - OWA Expand Control Register"]
    pub owa_exp_ctl: OWA_EXP_CTL,
    #[doc = "0x44 - OWA Expand Interrupt Status Register"]
    pub owa_exp_ista: OWA_EXP_ISTA,
    #[doc = "0x48 - OWA Expand Infomation Register0"]
    pub owa_exp_info_0: OWA_EXP_INFO_0,
    #[doc = "0x4c - OWA Expand Infomation Register1"]
    pub owa_exp_info_1: OWA_EXP_INFO_1,
    #[doc = "0x50 - OWA Expand Debug Register0"]
    pub owa_exp_dbg_0: OWA_EXP_DBG_0,
    #[doc = "0x54 - OWA Expand Debug Register1"]
    pub owa_exp_dbg_1: OWA_EXP_DBG_1,
}
#[doc = "owa_gen_ctl (rw) register accessor: an alias for `Reg<OWA_GEN_CTL_SPEC>`"]
pub type OWA_GEN_CTL = crate::Reg<owa_gen_ctl::OWA_GEN_CTL_SPEC>;
#[doc = "OWA General Control Register"]
pub mod owa_gen_ctl;
#[doc = "owa_tx_cfig (rw) register accessor: an alias for `Reg<OWA_TX_CFIG_SPEC>`"]
pub type OWA_TX_CFIG = crate::Reg<owa_tx_cfig::OWA_TX_CFIG_SPEC>;
#[doc = "OWA TX Configuration Register"]
pub mod owa_tx_cfig;
#[doc = "owa_rx_cfig (rw) register accessor: an alias for `Reg<OWA_RX_CFIG_SPEC>`"]
pub type OWA_RX_CFIG = crate::Reg<owa_rx_cfig::OWA_RX_CFIG_SPEC>;
#[doc = "OWA RX Configuration Register"]
pub mod owa_rx_cfig;
#[doc = "owa_ista (rw) register accessor: an alias for `Reg<OWA_ISTA_SPEC>`"]
pub type OWA_ISTA = crate::Reg<owa_ista::OWA_ISTA_SPEC>;
#[doc = "OWA Interrupt Status Register"]
pub mod owa_ista;
#[doc = "owa_rxfifo (rw) register accessor: an alias for `Reg<OWA_RXFIFO_SPEC>`"]
pub type OWA_RXFIFO = crate::Reg<owa_rxfifo::OWA_RXFIFO_SPEC>;
#[doc = "OWA RXFIFO Register"]
pub mod owa_rxfifo;
#[doc = "owa_fctl (rw) register accessor: an alias for `Reg<OWA_FCTL_SPEC>`"]
pub type OWA_FCTL = crate::Reg<owa_fctl::OWA_FCTL_SPEC>;
#[doc = "OWA FIFO Control Register"]
pub mod owa_fctl;
#[doc = "owa_fsta (rw) register accessor: an alias for `Reg<OWA_FSTA_SPEC>`"]
pub type OWA_FSTA = crate::Reg<owa_fsta::OWA_FSTA_SPEC>;
#[doc = "OWA FIFO Status Register"]
pub mod owa_fsta;
#[doc = "owa_int (rw) register accessor: an alias for `Reg<OWA_INT_SPEC>`"]
pub type OWA_INT = crate::Reg<owa_int::OWA_INT_SPEC>;
#[doc = "OWA Interrupt Control Register"]
pub mod owa_int;
#[doc = "owa_tx_fifo (rw) register accessor: an alias for `Reg<OWA_TX_FIFO_SPEC>`"]
pub type OWA_TX_FIFO = crate::Reg<owa_tx_fifo::OWA_TX_FIFO_SPEC>;
#[doc = "OWA TX FIFO Register"]
pub mod owa_tx_fifo;
#[doc = "owa_tx_cnt (rw) register accessor: an alias for `Reg<OWA_TX_CNT_SPEC>`"]
pub type OWA_TX_CNT = crate::Reg<owa_tx_cnt::OWA_TX_CNT_SPEC>;
#[doc = "OWA TX Counter Register"]
pub mod owa_tx_cnt;
#[doc = "owa_rx_cnt (rw) register accessor: an alias for `Reg<OWA_RX_CNT_SPEC>`"]
pub type OWA_RX_CNT = crate::Reg<owa_rx_cnt::OWA_RX_CNT_SPEC>;
#[doc = "OWA RX Counter Register"]
pub mod owa_rx_cnt;
#[doc = "owa_tx_chsta0 (rw) register accessor: an alias for `Reg<OWA_TX_CHSTA0_SPEC>`"]
pub type OWA_TX_CHSTA0 = crate::Reg<owa_tx_chsta0::OWA_TX_CHSTA0_SPEC>;
#[doc = "OWA TX Channel Status Register0"]
pub mod owa_tx_chsta0;
#[doc = "owa_tx_chsta1 (rw) register accessor: an alias for `Reg<OWA_TX_CHSTA1_SPEC>`"]
pub type OWA_TX_CHSTA1 = crate::Reg<owa_tx_chsta1::OWA_TX_CHSTA1_SPEC>;
#[doc = "OWA TX Channel Status Register1"]
pub mod owa_tx_chsta1;
#[doc = "owa_rxchsta0 (rw) register accessor: an alias for `Reg<OWA_RXCHSTA0_SPEC>`"]
pub type OWA_RXCHSTA0 = crate::Reg<owa_rxchsta0::OWA_RXCHSTA0_SPEC>;
#[doc = "OWA RX Channel Status Register0"]
pub mod owa_rxchsta0;
#[doc = "owa_rxchsta1 (rw) register accessor: an alias for `Reg<OWA_RXCHSTA1_SPEC>`"]
pub type OWA_RXCHSTA1 = crate::Reg<owa_rxchsta1::OWA_RXCHSTA1_SPEC>;
#[doc = "OWA RX Channel Status Register1"]
pub mod owa_rxchsta1;
#[doc = "owa_exp_ctl (rw) register accessor: an alias for `Reg<OWA_EXP_CTL_SPEC>`"]
pub type OWA_EXP_CTL = crate::Reg<owa_exp_ctl::OWA_EXP_CTL_SPEC>;
#[doc = "OWA Expand Control Register"]
pub mod owa_exp_ctl;
#[doc = "owa_exp_ista (rw) register accessor: an alias for `Reg<OWA_EXP_ISTA_SPEC>`"]
pub type OWA_EXP_ISTA = crate::Reg<owa_exp_ista::OWA_EXP_ISTA_SPEC>;
#[doc = "OWA Expand Interrupt Status Register"]
pub mod owa_exp_ista;
#[doc = "owa_exp_info_0 (rw) register accessor: an alias for `Reg<OWA_EXP_INFO_0_SPEC>`"]
pub type OWA_EXP_INFO_0 = crate::Reg<owa_exp_info_0::OWA_EXP_INFO_0_SPEC>;
#[doc = "OWA Expand Infomation Register0"]
pub mod owa_exp_info_0;
#[doc = "owa_exp_info_1 (rw) register accessor: an alias for `Reg<OWA_EXP_INFO_1_SPEC>`"]
pub type OWA_EXP_INFO_1 = crate::Reg<owa_exp_info_1::OWA_EXP_INFO_1_SPEC>;
#[doc = "OWA Expand Infomation Register1"]
pub mod owa_exp_info_1;
#[doc = "owa_exp_dbg_0 (rw) register accessor: an alias for `Reg<OWA_EXP_DBG_0_SPEC>`"]
pub type OWA_EXP_DBG_0 = crate::Reg<owa_exp_dbg_0::OWA_EXP_DBG_0_SPEC>;
#[doc = "OWA Expand Debug Register0"]
pub mod owa_exp_dbg_0;
#[doc = "owa_exp_dbg_1 (rw) register accessor: an alias for `Reg<OWA_EXP_DBG_1_SPEC>`"]
pub type OWA_EXP_DBG_1 = crate::Reg<owa_exp_dbg_1::OWA_EXP_DBG_1_SPEC>;
#[doc = "OWA Expand Debug Register1"]
pub mod owa_exp_dbg_1;
