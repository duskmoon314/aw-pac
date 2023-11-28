#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    emac_basic_ctl0: EMAC_BASIC_CTL0,
    emac_basic_ctl1: EMAC_BASIC_CTL1,
    emac_int_sta: EMAC_INT_STA,
    emac_int_en: EMAC_INT_EN,
    emac_tx_ctl0: EMAC_TX_CTL0,
    emac_tx_ctl1: EMAC_TX_CTL1,
    _reserved6: [u8; 0x04],
    emac_tx_flow_ctl: EMAC_TX_FLOW_CTL,
    emac_tx_dma_desc_list: EMAC_TX_DMA_DESC_LIST,
    emac_rx_ctl0: EMAC_RX_CTL0,
    emac_rx_ctl1: EMAC_RX_CTL1,
    _reserved10: [u8; 0x08],
    emac_rx_dma_desc_list: EMAC_RX_DMA_DESC_LIST,
    emac_rx_frm_flt: EMAC_RX_FRM_FLT,
    _reserved12: [u8; 0x04],
    emac_rx_hash0: EMAC_RX_HASH0,
    emac_rx_hash1: EMAC_RX_HASH1,
    emac_mii_cmd: EMAC_MII_CMD,
    emac_mii_data: EMAC_MII_DATA,
    emac_addr_high0: EMAC_ADDR_HIGH0,
    emac_addr_low: (),
    _reserved18: [u8; 0x04],
    emac_addr_high: (),
    _reserved19: [u8; 0x58],
    emac_tx_dma_sta: EMAC_TX_DMA_STA,
    emac_tx_cur_desc: EMAC_TX_CUR_DESC,
    emac_tx_cur_buf: EMAC_TX_CUR_BUF,
    _reserved22: [u8; 0x04],
    emac_rx_dma_sta: EMAC_RX_DMA_STA,
    emac_rx_cur_desc: EMAC_RX_CUR_DESC,
    emac_rx_cur_buf: EMAC_RX_CUR_BUF,
    _reserved25: [u8; 0x04],
    emac_rgmii_sta: EMAC_RGMII_STA,
}
impl RegisterBlock {
    #[doc = "0x00 - EMAC Basic Control Register0"]
    #[inline(always)]
    pub const fn emac_basic_ctl0(&self) -> &EMAC_BASIC_CTL0 {
        &self.emac_basic_ctl0
    }
    #[doc = "0x04 - EMAC Basic Control Register1"]
    #[inline(always)]
    pub const fn emac_basic_ctl1(&self) -> &EMAC_BASIC_CTL1 {
        &self.emac_basic_ctl1
    }
    #[doc = "0x08 - EMAC Interrupt Status Register"]
    #[inline(always)]
    pub const fn emac_int_sta(&self) -> &EMAC_INT_STA {
        &self.emac_int_sta
    }
    #[doc = "0x0c - EMAC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn emac_int_en(&self) -> &EMAC_INT_EN {
        &self.emac_int_en
    }
    #[doc = "0x10 - EMAC Transmit Control Register0"]
    #[inline(always)]
    pub const fn emac_tx_ctl0(&self) -> &EMAC_TX_CTL0 {
        &self.emac_tx_ctl0
    }
    #[doc = "0x14 - EMAC Transmit Control Register1"]
    #[inline(always)]
    pub const fn emac_tx_ctl1(&self) -> &EMAC_TX_CTL1 {
        &self.emac_tx_ctl1
    }
    #[doc = "0x1c - EMAC Transmit Flow Control Register"]
    #[inline(always)]
    pub const fn emac_tx_flow_ctl(&self) -> &EMAC_TX_FLOW_CTL {
        &self.emac_tx_flow_ctl
    }
    #[doc = "0x20 - EMAC Transmit Descriptor List Address Register"]
    #[inline(always)]
    pub const fn emac_tx_dma_desc_list(&self) -> &EMAC_TX_DMA_DESC_LIST {
        &self.emac_tx_dma_desc_list
    }
    #[doc = "0x24 - EMAC Receive Control Register0"]
    #[inline(always)]
    pub const fn emac_rx_ctl0(&self) -> &EMAC_RX_CTL0 {
        &self.emac_rx_ctl0
    }
    #[doc = "0x28 - EMAC Receive Control Register1"]
    #[inline(always)]
    pub const fn emac_rx_ctl1(&self) -> &EMAC_RX_CTL1 {
        &self.emac_rx_ctl1
    }
    #[doc = "0x34 - EMAC Receive Descriptor List Address Register"]
    #[inline(always)]
    pub const fn emac_rx_dma_desc_list(&self) -> &EMAC_RX_DMA_DESC_LIST {
        &self.emac_rx_dma_desc_list
    }
    #[doc = "0x38 - EMAC Receive Frame Filter Register"]
    #[inline(always)]
    pub const fn emac_rx_frm_flt(&self) -> &EMAC_RX_FRM_FLT {
        &self.emac_rx_frm_flt
    }
    #[doc = "0x40 - EMAC Hash Table Register0"]
    #[inline(always)]
    pub const fn emac_rx_hash0(&self) -> &EMAC_RX_HASH0 {
        &self.emac_rx_hash0
    }
    #[doc = "0x44 - EMAC Hash Table Register1"]
    #[inline(always)]
    pub const fn emac_rx_hash1(&self) -> &EMAC_RX_HASH1 {
        &self.emac_rx_hash1
    }
    #[doc = "0x48 - EMAC Management Interface Command Register"]
    #[inline(always)]
    pub const fn emac_mii_cmd(&self) -> &EMAC_MII_CMD {
        &self.emac_mii_cmd
    }
    #[doc = "0x4c - EMAC Management Interface Data Register"]
    #[inline(always)]
    pub const fn emac_mii_data(&self) -> &EMAC_MII_DATA {
        &self.emac_mii_data
    }
    #[doc = "0x50 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high0(&self) -> &EMAC_ADDR_HIGH0 {
        &self.emac_addr_high0
    }
    #[doc = "0x54..0x74 - EMAC MAC Address Low Register"]
    #[inline(always)]
    pub const fn emac_addr_low(&self, n: usize) -> &EMAC_ADDR_LOW {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(84).add(8 * n).cast() }
    }
    #[doc = "0x58..0x74 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high(&self, n: usize) -> &EMAC_ADDR_HIGH {
        #[allow(clippy::no_effect)]
        [(); 7][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(88).add(8 * n).cast() }
    }
    #[doc = "0x58 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high1(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(0)
    }
    #[doc = "0x60 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high2(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(1)
    }
    #[doc = "0x68 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high3(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(2)
    }
    #[doc = "0x70 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high4(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(3)
    }
    #[doc = "0x78 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high5(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(4)
    }
    #[doc = "0x80 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high6(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(5)
    }
    #[doc = "0x88 - EMAC MAC Address High Register"]
    #[inline(always)]
    pub const fn emac_addr_high7(&self) -> &EMAC_ADDR_HIGH {
        self.emac_addr_high(6)
    }
    #[doc = "0xb0 - EMAC Transmit DMA Status Register"]
    #[inline(always)]
    pub const fn emac_tx_dma_sta(&self) -> &EMAC_TX_DMA_STA {
        &self.emac_tx_dma_sta
    }
    #[doc = "0xb4 - EMAC Current Transmit Descriptor Register"]
    #[inline(always)]
    pub const fn emac_tx_cur_desc(&self) -> &EMAC_TX_CUR_DESC {
        &self.emac_tx_cur_desc
    }
    #[doc = "0xb8 - EMAC Current Transmit Buffer Address Register"]
    #[inline(always)]
    pub const fn emac_tx_cur_buf(&self) -> &EMAC_TX_CUR_BUF {
        &self.emac_tx_cur_buf
    }
    #[doc = "0xc0 - EMAC Receive DMA Status Register"]
    #[inline(always)]
    pub const fn emac_rx_dma_sta(&self) -> &EMAC_RX_DMA_STA {
        &self.emac_rx_dma_sta
    }
    #[doc = "0xc4 - EMAC Current Receive Descriptor Register"]
    #[inline(always)]
    pub const fn emac_rx_cur_desc(&self) -> &EMAC_RX_CUR_DESC {
        &self.emac_rx_cur_desc
    }
    #[doc = "0xc8 - EMAC Current Receive Buffer Address Register"]
    #[inline(always)]
    pub const fn emac_rx_cur_buf(&self) -> &EMAC_RX_CUR_BUF {
        &self.emac_rx_cur_buf
    }
    #[doc = "0xd0 - EMAC RGMII Status Register"]
    #[inline(always)]
    pub const fn emac_rgmii_sta(&self) -> &EMAC_RGMII_STA {
        &self.emac_rgmii_sta
    }
}
#[doc = "emac_basic_ctl0 (rw) register accessor: EMAC Basic Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_basic_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_basic_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_basic_ctl0`] module"]
pub type EMAC_BASIC_CTL0 = crate::Reg<emac_basic_ctl0::EMAC_BASIC_CTL0_SPEC>;
#[doc = "EMAC Basic Control Register0"]
pub mod emac_basic_ctl0;
#[doc = "emac_basic_ctl1 (rw) register accessor: EMAC Basic Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_basic_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_basic_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_basic_ctl1`] module"]
pub type EMAC_BASIC_CTL1 = crate::Reg<emac_basic_ctl1::EMAC_BASIC_CTL1_SPEC>;
#[doc = "EMAC Basic Control Register1"]
pub mod emac_basic_ctl1;
#[doc = "emac_int_sta (rw) register accessor: EMAC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_int_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_int_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_int_sta`] module"]
pub type EMAC_INT_STA = crate::Reg<emac_int_sta::EMAC_INT_STA_SPEC>;
#[doc = "EMAC Interrupt Status Register"]
pub mod emac_int_sta;
#[doc = "emac_int_en (rw) register accessor: EMAC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_int_en`] module"]
pub type EMAC_INT_EN = crate::Reg<emac_int_en::EMAC_INT_EN_SPEC>;
#[doc = "EMAC Interrupt Enable Register"]
pub mod emac_int_en;
#[doc = "emac_tx_ctl0 (rw) register accessor: EMAC Transmit Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_tx_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_ctl0`] module"]
pub type EMAC_TX_CTL0 = crate::Reg<emac_tx_ctl0::EMAC_TX_CTL0_SPEC>;
#[doc = "EMAC Transmit Control Register0"]
pub mod emac_tx_ctl0;
#[doc = "emac_tx_ctl1 (rw) register accessor: EMAC Transmit Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_tx_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_ctl1`] module"]
pub type EMAC_TX_CTL1 = crate::Reg<emac_tx_ctl1::EMAC_TX_CTL1_SPEC>;
#[doc = "EMAC Transmit Control Register1"]
pub mod emac_tx_ctl1;
#[doc = "emac_tx_flow_ctl (rw) register accessor: EMAC Transmit Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_flow_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_tx_flow_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_flow_ctl`] module"]
pub type EMAC_TX_FLOW_CTL = crate::Reg<emac_tx_flow_ctl::EMAC_TX_FLOW_CTL_SPEC>;
#[doc = "EMAC Transmit Flow Control Register"]
pub mod emac_tx_flow_ctl;
#[doc = "emac_tx_dma_desc_list (rw) register accessor: EMAC Transmit Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_dma_desc_list::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_tx_dma_desc_list::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_dma_desc_list`] module"]
pub type EMAC_TX_DMA_DESC_LIST = crate::Reg<emac_tx_dma_desc_list::EMAC_TX_DMA_DESC_LIST_SPEC>;
#[doc = "EMAC Transmit Descriptor List Address Register"]
pub mod emac_tx_dma_desc_list;
#[doc = "emac_rx_ctl0 (rw) register accessor: EMAC Receive Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_ctl0`] module"]
pub type EMAC_RX_CTL0 = crate::Reg<emac_rx_ctl0::EMAC_RX_CTL0_SPEC>;
#[doc = "EMAC Receive Control Register0"]
pub mod emac_rx_ctl0;
#[doc = "emac_rx_ctl1 (rw) register accessor: EMAC Receive Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_ctl1`] module"]
pub type EMAC_RX_CTL1 = crate::Reg<emac_rx_ctl1::EMAC_RX_CTL1_SPEC>;
#[doc = "EMAC Receive Control Register1"]
pub mod emac_rx_ctl1;
#[doc = "emac_rx_dma_desc_list (rw) register accessor: EMAC Receive Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_dma_desc_list::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_dma_desc_list::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_dma_desc_list`] module"]
pub type EMAC_RX_DMA_DESC_LIST = crate::Reg<emac_rx_dma_desc_list::EMAC_RX_DMA_DESC_LIST_SPEC>;
#[doc = "EMAC Receive Descriptor List Address Register"]
pub mod emac_rx_dma_desc_list;
#[doc = "emac_rx_frm_flt (rw) register accessor: EMAC Receive Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_frm_flt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_frm_flt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_frm_flt`] module"]
pub type EMAC_RX_FRM_FLT = crate::Reg<emac_rx_frm_flt::EMAC_RX_FRM_FLT_SPEC>;
#[doc = "EMAC Receive Frame Filter Register"]
pub mod emac_rx_frm_flt;
#[doc = "emac_rx_hash0 (rw) register accessor: EMAC Hash Table Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_hash0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_hash0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_hash0`] module"]
pub type EMAC_RX_HASH0 = crate::Reg<emac_rx_hash0::EMAC_RX_HASH0_SPEC>;
#[doc = "EMAC Hash Table Register0"]
pub mod emac_rx_hash0;
#[doc = "emac_rx_hash1 (rw) register accessor: EMAC Hash Table Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_hash1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_hash1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_hash1`] module"]
pub type EMAC_RX_HASH1 = crate::Reg<emac_rx_hash1::EMAC_RX_HASH1_SPEC>;
#[doc = "EMAC Hash Table Register1"]
pub mod emac_rx_hash1;
#[doc = "emac_mii_cmd (rw) register accessor: EMAC Management Interface Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_mii_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_mii_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_mii_cmd`] module"]
pub type EMAC_MII_CMD = crate::Reg<emac_mii_cmd::EMAC_MII_CMD_SPEC>;
#[doc = "EMAC Management Interface Command Register"]
pub mod emac_mii_cmd;
#[doc = "emac_mii_data (rw) register accessor: EMAC Management Interface Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_mii_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_mii_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_mii_data`] module"]
pub type EMAC_MII_DATA = crate::Reg<emac_mii_data::EMAC_MII_DATA_SPEC>;
#[doc = "EMAC Management Interface Data Register"]
pub mod emac_mii_data;
#[doc = "emac_addr_high0 (rw) register accessor: EMAC MAC Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_addr_high0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_addr_high0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_addr_high0`] module"]
pub type EMAC_ADDR_HIGH0 = crate::Reg<emac_addr_high0::EMAC_ADDR_HIGH0_SPEC>;
#[doc = "EMAC MAC Address High Register"]
pub mod emac_addr_high0;
#[doc = "emac_addr_high (rw) register accessor: EMAC MAC Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_addr_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_addr_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_addr_high`] module"]
pub type EMAC_ADDR_HIGH = crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>;
#[doc = "EMAC MAC Address High Register"]
pub mod emac_addr_high;
#[doc = "emac_addr_low (rw) register accessor: EMAC MAC Address Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_addr_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_addr_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_addr_low`] module"]
pub type EMAC_ADDR_LOW = crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>;
#[doc = "EMAC MAC Address Low Register"]
pub mod emac_addr_low;
#[doc = "emac_tx_dma_sta (r) register accessor: EMAC Transmit DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_dma_sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_dma_sta`] module"]
pub type EMAC_TX_DMA_STA = crate::Reg<emac_tx_dma_sta::EMAC_TX_DMA_STA_SPEC>;
#[doc = "EMAC Transmit DMA Status Register"]
pub mod emac_tx_dma_sta;
#[doc = "emac_tx_cur_desc (r) register accessor: EMAC Current Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_cur_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_cur_desc`] module"]
pub type EMAC_TX_CUR_DESC = crate::Reg<emac_tx_cur_desc::EMAC_TX_CUR_DESC_SPEC>;
#[doc = "EMAC Current Transmit Descriptor Register"]
pub mod emac_tx_cur_desc;
#[doc = "emac_tx_cur_buf (r) register accessor: EMAC Current Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_cur_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_tx_cur_buf`] module"]
pub type EMAC_TX_CUR_BUF = crate::Reg<emac_tx_cur_buf::EMAC_TX_CUR_BUF_SPEC>;
#[doc = "EMAC Current Transmit Buffer Address Register"]
pub mod emac_tx_cur_buf;
#[doc = "emac_rx_dma_sta (r) register accessor: EMAC Receive DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_dma_sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_dma_sta`] module"]
pub type EMAC_RX_DMA_STA = crate::Reg<emac_rx_dma_sta::EMAC_RX_DMA_STA_SPEC>;
#[doc = "EMAC Receive DMA Status Register"]
pub mod emac_rx_dma_sta;
#[doc = "emac_rx_cur_desc (r) register accessor: EMAC Current Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_cur_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_cur_desc`] module"]
pub type EMAC_RX_CUR_DESC = crate::Reg<emac_rx_cur_desc::EMAC_RX_CUR_DESC_SPEC>;
#[doc = "EMAC Current Receive Descriptor Register"]
pub mod emac_rx_cur_desc;
#[doc = "emac_rx_cur_buf (r) register accessor: EMAC Current Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_cur_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rx_cur_buf`] module"]
pub type EMAC_RX_CUR_BUF = crate::Reg<emac_rx_cur_buf::EMAC_RX_CUR_BUF_SPEC>;
#[doc = "EMAC Current Receive Buffer Address Register"]
pub mod emac_rx_cur_buf;
#[doc = "emac_rgmii_sta (rw) register accessor: EMAC RGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rgmii_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rgmii_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_rgmii_sta`] module"]
pub type EMAC_RGMII_STA = crate::Reg<emac_rgmii_sta::EMAC_RGMII_STA_SPEC>;
#[doc = "EMAC RGMII Status Register"]
pub mod emac_rgmii_sta;
