#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    owa_gen_ctl: OWA_GEN_CTL,
    owa_tx_cfig: OWA_TX_CFIG,
    owa_rx_cfig: OWA_RX_CFIG,
    owa_ista: OWA_ISTA,
    owa_rxfifo: OWA_RXFIFO,
    owa_fctl: OWA_FCTL,
    owa_fsta: OWA_FSTA,
    owa_int: OWA_INT,
    owa_tx_fifo: OWA_TX_FIFO,
    owa_tx_cnt: OWA_TX_CNT,
    owa_rx_cnt: OWA_RX_CNT,
    owa_tx_chsta0: OWA_TX_CHSTA0,
    owa_tx_chsta1: OWA_TX_CHSTA1,
    owa_rxchsta0: OWA_RXCHSTA0,
    owa_rxchsta1: OWA_RXCHSTA1,
    _reserved15: [u8; 0x04],
    owa_exp_ctl: OWA_EXP_CTL,
    owa_exp_ista: OWA_EXP_ISTA,
    owa_exp_info_0: OWA_EXP_INFO_0,
    owa_exp_info_1: OWA_EXP_INFO_1,
    owa_exp_dbg_0: OWA_EXP_DBG_0,
    owa_exp_dbg_1: OWA_EXP_DBG_1,
}
impl RegisterBlock {
    #[doc = "0x00 - OWA General Control Register"]
    #[inline(always)]
    pub const fn owa_gen_ctl(&self) -> &OWA_GEN_CTL {
        &self.owa_gen_ctl
    }
    #[doc = "0x04 - OWA TX Configuration Register"]
    #[inline(always)]
    pub const fn owa_tx_cfig(&self) -> &OWA_TX_CFIG {
        &self.owa_tx_cfig
    }
    #[doc = "0x08 - OWA RX Configuration Register"]
    #[inline(always)]
    pub const fn owa_rx_cfig(&self) -> &OWA_RX_CFIG {
        &self.owa_rx_cfig
    }
    #[doc = "0x0c - OWA Interrupt Status Register"]
    #[inline(always)]
    pub const fn owa_ista(&self) -> &OWA_ISTA {
        &self.owa_ista
    }
    #[doc = "0x10 - OWA RXFIFO Register"]
    #[inline(always)]
    pub const fn owa_rxfifo(&self) -> &OWA_RXFIFO {
        &self.owa_rxfifo
    }
    #[doc = "0x14 - OWA FIFO Control Register"]
    #[inline(always)]
    pub const fn owa_fctl(&self) -> &OWA_FCTL {
        &self.owa_fctl
    }
    #[doc = "0x18 - OWA FIFO Status Register"]
    #[inline(always)]
    pub const fn owa_fsta(&self) -> &OWA_FSTA {
        &self.owa_fsta
    }
    #[doc = "0x1c - OWA Interrupt Control Register"]
    #[inline(always)]
    pub const fn owa_int(&self) -> &OWA_INT {
        &self.owa_int
    }
    #[doc = "0x20 - OWA TX FIFO Register"]
    #[inline(always)]
    pub const fn owa_tx_fifo(&self) -> &OWA_TX_FIFO {
        &self.owa_tx_fifo
    }
    #[doc = "0x24 - OWA TX Counter Register"]
    #[inline(always)]
    pub const fn owa_tx_cnt(&self) -> &OWA_TX_CNT {
        &self.owa_tx_cnt
    }
    #[doc = "0x28 - OWA RX Counter Register"]
    #[inline(always)]
    pub const fn owa_rx_cnt(&self) -> &OWA_RX_CNT {
        &self.owa_rx_cnt
    }
    #[doc = "0x2c - OWA TX Channel Status Register0"]
    #[inline(always)]
    pub const fn owa_tx_chsta0(&self) -> &OWA_TX_CHSTA0 {
        &self.owa_tx_chsta0
    }
    #[doc = "0x30 - OWA TX Channel Status Register1"]
    #[inline(always)]
    pub const fn owa_tx_chsta1(&self) -> &OWA_TX_CHSTA1 {
        &self.owa_tx_chsta1
    }
    #[doc = "0x34 - OWA RX Channel Status Register0"]
    #[inline(always)]
    pub const fn owa_rxchsta0(&self) -> &OWA_RXCHSTA0 {
        &self.owa_rxchsta0
    }
    #[doc = "0x38 - OWA RX Channel Status Register1"]
    #[inline(always)]
    pub const fn owa_rxchsta1(&self) -> &OWA_RXCHSTA1 {
        &self.owa_rxchsta1
    }
    #[doc = "0x40 - OWA Expand Control Register"]
    #[inline(always)]
    pub const fn owa_exp_ctl(&self) -> &OWA_EXP_CTL {
        &self.owa_exp_ctl
    }
    #[doc = "0x44 - OWA Expand Interrupt Status Register"]
    #[inline(always)]
    pub const fn owa_exp_ista(&self) -> &OWA_EXP_ISTA {
        &self.owa_exp_ista
    }
    #[doc = "0x48 - OWA Expand Infomation Register0"]
    #[inline(always)]
    pub const fn owa_exp_info_0(&self) -> &OWA_EXP_INFO_0 {
        &self.owa_exp_info_0
    }
    #[doc = "0x4c - OWA Expand Infomation Register1"]
    #[inline(always)]
    pub const fn owa_exp_info_1(&self) -> &OWA_EXP_INFO_1 {
        &self.owa_exp_info_1
    }
    #[doc = "0x50 - OWA Expand Debug Register0"]
    #[inline(always)]
    pub const fn owa_exp_dbg_0(&self) -> &OWA_EXP_DBG_0 {
        &self.owa_exp_dbg_0
    }
    #[doc = "0x54 - OWA Expand Debug Register1"]
    #[inline(always)]
    pub const fn owa_exp_dbg_1(&self) -> &OWA_EXP_DBG_1 {
        &self.owa_exp_dbg_1
    }
}
#[doc = "owa_gen_ctl (rw) register accessor: OWA General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_gen_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_gen_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_gen_ctl`] module"]
pub type OWA_GEN_CTL = crate::Reg<owa_gen_ctl::OWA_GEN_CTL_SPEC>;
#[doc = "OWA General Control Register"]
pub mod owa_gen_ctl;
#[doc = "owa_tx_cfig (rw) register accessor: OWA TX Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_tx_cfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_tx_cfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_tx_cfig`] module"]
pub type OWA_TX_CFIG = crate::Reg<owa_tx_cfig::OWA_TX_CFIG_SPEC>;
#[doc = "OWA TX Configuration Register"]
pub mod owa_tx_cfig;
#[doc = "owa_rx_cfig (rw) register accessor: OWA RX Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_rx_cfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_rx_cfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_rx_cfig`] module"]
pub type OWA_RX_CFIG = crate::Reg<owa_rx_cfig::OWA_RX_CFIG_SPEC>;
#[doc = "OWA RX Configuration Register"]
pub mod owa_rx_cfig;
#[doc = "owa_ista (rw) register accessor: OWA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_ista::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_ista::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_ista`] module"]
pub type OWA_ISTA = crate::Reg<owa_ista::OWA_ISTA_SPEC>;
#[doc = "OWA Interrupt Status Register"]
pub mod owa_ista;
#[doc = "owa_rxfifo (rw) register accessor: OWA RXFIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_rxfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_rxfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_rxfifo`] module"]
pub type OWA_RXFIFO = crate::Reg<owa_rxfifo::OWA_RXFIFO_SPEC>;
#[doc = "OWA RXFIFO Register"]
pub mod owa_rxfifo;
#[doc = "owa_fctl (rw) register accessor: OWA FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_fctl`] module"]
pub type OWA_FCTL = crate::Reg<owa_fctl::OWA_FCTL_SPEC>;
#[doc = "OWA FIFO Control Register"]
pub mod owa_fctl;
#[doc = "owa_fsta (rw) register accessor: OWA FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_fsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_fsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_fsta`] module"]
pub type OWA_FSTA = crate::Reg<owa_fsta::OWA_FSTA_SPEC>;
#[doc = "OWA FIFO Status Register"]
pub mod owa_fsta;
#[doc = "owa_int (rw) register accessor: OWA Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_int`] module"]
pub type OWA_INT = crate::Reg<owa_int::OWA_INT_SPEC>;
#[doc = "OWA Interrupt Control Register"]
pub mod owa_int;
#[doc = "owa_tx_fifo (rw) register accessor: OWA TX FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_tx_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_tx_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_tx_fifo`] module"]
pub type OWA_TX_FIFO = crate::Reg<owa_tx_fifo::OWA_TX_FIFO_SPEC>;
#[doc = "OWA TX FIFO Register"]
pub mod owa_tx_fifo;
#[doc = "owa_tx_cnt (rw) register accessor: OWA TX Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_tx_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_tx_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_tx_cnt`] module"]
pub type OWA_TX_CNT = crate::Reg<owa_tx_cnt::OWA_TX_CNT_SPEC>;
#[doc = "OWA TX Counter Register"]
pub mod owa_tx_cnt;
#[doc = "owa_rx_cnt (rw) register accessor: OWA RX Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_rx_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_rx_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_rx_cnt`] module"]
pub type OWA_RX_CNT = crate::Reg<owa_rx_cnt::OWA_RX_CNT_SPEC>;
#[doc = "OWA RX Counter Register"]
pub mod owa_rx_cnt;
#[doc = "owa_tx_chsta0 (rw) register accessor: OWA TX Channel Status Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_tx_chsta0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_tx_chsta0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_tx_chsta0`] module"]
pub type OWA_TX_CHSTA0 = crate::Reg<owa_tx_chsta0::OWA_TX_CHSTA0_SPEC>;
#[doc = "OWA TX Channel Status Register0"]
pub mod owa_tx_chsta0;
#[doc = "owa_tx_chsta1 (rw) register accessor: OWA TX Channel Status Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_tx_chsta1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_tx_chsta1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_tx_chsta1`] module"]
pub type OWA_TX_CHSTA1 = crate::Reg<owa_tx_chsta1::OWA_TX_CHSTA1_SPEC>;
#[doc = "OWA TX Channel Status Register1"]
pub mod owa_tx_chsta1;
#[doc = "owa_rxchsta0 (rw) register accessor: OWA RX Channel Status Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_rxchsta0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_rxchsta0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_rxchsta0`] module"]
pub type OWA_RXCHSTA0 = crate::Reg<owa_rxchsta0::OWA_RXCHSTA0_SPEC>;
#[doc = "OWA RX Channel Status Register0"]
pub mod owa_rxchsta0;
#[doc = "owa_rxchsta1 (rw) register accessor: OWA RX Channel Status Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_rxchsta1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_rxchsta1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_rxchsta1`] module"]
pub type OWA_RXCHSTA1 = crate::Reg<owa_rxchsta1::OWA_RXCHSTA1_SPEC>;
#[doc = "OWA RX Channel Status Register1"]
pub mod owa_rxchsta1;
#[doc = "owa_exp_ctl (rw) register accessor: OWA Expand Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_exp_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_exp_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_exp_ctl`] module"]
pub type OWA_EXP_CTL = crate::Reg<owa_exp_ctl::OWA_EXP_CTL_SPEC>;
#[doc = "OWA Expand Control Register"]
pub mod owa_exp_ctl;
#[doc = "owa_exp_ista (rw) register accessor: OWA Expand Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_exp_ista::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_exp_ista::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_exp_ista`] module"]
pub type OWA_EXP_ISTA = crate::Reg<owa_exp_ista::OWA_EXP_ISTA_SPEC>;
#[doc = "OWA Expand Interrupt Status Register"]
pub mod owa_exp_ista;
#[doc = "owa_exp_info_0 (rw) register accessor: OWA Expand Infomation Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_exp_info_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_exp_info_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_exp_info_0`] module"]
pub type OWA_EXP_INFO_0 = crate::Reg<owa_exp_info_0::OWA_EXP_INFO_0_SPEC>;
#[doc = "OWA Expand Infomation Register0"]
pub mod owa_exp_info_0;
#[doc = "owa_exp_info_1 (rw) register accessor: OWA Expand Infomation Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_exp_info_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_exp_info_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_exp_info_1`] module"]
pub type OWA_EXP_INFO_1 = crate::Reg<owa_exp_info_1::OWA_EXP_INFO_1_SPEC>;
#[doc = "OWA Expand Infomation Register1"]
pub mod owa_exp_info_1;
#[doc = "owa_exp_dbg_0 (rw) register accessor: OWA Expand Debug Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_exp_dbg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_exp_dbg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_exp_dbg_0`] module"]
pub type OWA_EXP_DBG_0 = crate::Reg<owa_exp_dbg_0::OWA_EXP_DBG_0_SPEC>;
#[doc = "OWA Expand Debug Register0"]
pub mod owa_exp_dbg_0;
#[doc = "owa_exp_dbg_1 (rw) register accessor: OWA Expand Debug Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_exp_dbg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_exp_dbg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_exp_dbg_1`] module"]
pub type OWA_EXP_DBG_1 = crate::Reg<owa_exp_dbg_1::OWA_EXP_DBG_1_SPEC>;
#[doc = "OWA Expand Debug Register1"]
pub mod owa_exp_dbg_1;
