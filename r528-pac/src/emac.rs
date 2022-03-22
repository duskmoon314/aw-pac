#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EMAC Basic Control Register0"]
    pub emac_basic_ctl0: crate::Reg<emac_basic_ctl0::EMAC_BASIC_CTL0_SPEC>,
    #[doc = "0x04 - EMAC Basic Control Register1"]
    pub emac_basic_ctl1: crate::Reg<emac_basic_ctl1::EMAC_BASIC_CTL1_SPEC>,
    #[doc = "0x08 - EMAC Interrupt Status Register"]
    pub emac_int_sta: crate::Reg<emac_int_sta::EMAC_INT_STA_SPEC>,
    #[doc = "0x0c - EMAC Interrupt Enable Register"]
    pub emac_int_en: crate::Reg<emac_int_en::EMAC_INT_EN_SPEC>,
    #[doc = "0x10 - EMAC Transmit Control Register0"]
    pub emac_tx_ctl0: crate::Reg<emac_tx_ctl0::EMAC_TX_CTL0_SPEC>,
    #[doc = "0x14 - EMAC Transmit Control Register1"]
    pub emac_tx_ctl1: crate::Reg<emac_tx_ctl1::EMAC_TX_CTL1_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - EMAC Transmit Flow Control Register"]
    pub emac_tx_flow_ctl: crate::Reg<emac_tx_flow_ctl::EMAC_TX_FLOW_CTL_SPEC>,
    #[doc = "0x20 - EMAC Transmit Descriptor List Address Register"]
    pub emac_tx_dma_desc_list: crate::Reg<emac_tx_dma_desc_list::EMAC_TX_DMA_DESC_LIST_SPEC>,
    #[doc = "0x24 - EMAC Receive Control Register0"]
    pub emac_rx_ctl0: crate::Reg<emac_rx_ctl0::EMAC_RX_CTL0_SPEC>,
    #[doc = "0x28 - EMAC Receive Control Register1"]
    pub emac_rx_ctl1: crate::Reg<emac_rx_ctl1::EMAC_RX_CTL1_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x34 - EMAC Receive Descriptor List Address Register"]
    pub emac_rx_dma_desc_list: crate::Reg<emac_rx_dma_desc_list::EMAC_RX_DMA_DESC_LIST_SPEC>,
    #[doc = "0x38 - EMAC Receive Frame Filter Register"]
    pub emac_rx_frm_flt: crate::Reg<emac_rx_frm_flt::EMAC_RX_FRM_FLT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - EMAC Hash Table Register0"]
    pub emac_rx_hash0: crate::Reg<emac_rx_hash0::EMAC_RX_HASH0_SPEC>,
    #[doc = "0x44 - EMAC Hash Table Register1"]
    pub emac_rx_hash1: crate::Reg<emac_rx_hash1::EMAC_RX_HASH1_SPEC>,
    #[doc = "0x48 - EMAC Management Interface Command Register"]
    pub emac_mii_cmd: crate::Reg<emac_mii_cmd::EMAC_MII_CMD_SPEC>,
    #[doc = "0x4c - EMAC Management Interface Data Register"]
    pub emac_mii_data: crate::Reg<emac_mii_data::EMAC_MII_DATA_SPEC>,
    #[doc = "0x50 - EMAC MAC Address High Register"]
    pub emac_addr_high0: crate::Reg<emac_addr_high0::EMAC_ADDR_HIGH0_SPEC>,
    #[doc = "0x54 - EMAC MAC Address Low Register"]
    pub emac_addr_low0: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x58 - EMAC MAC Address High Register"]
    pub emac_addr_high1: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x5c - EMAC MAC Address Low Register"]
    pub emac_addr_low1: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x60 - EMAC MAC Address High Register"]
    pub emac_addr_high2: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x64 - EMAC MAC Address Low Register"]
    pub emac_addr_low2: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x68 - EMAC MAC Address High Register"]
    pub emac_addr_high3: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x6c - EMAC MAC Address Low Register"]
    pub emac_addr_low3: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x70 - EMAC MAC Address High Register"]
    pub emac_addr_high4: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x74 - EMAC MAC Address Low Register"]
    pub emac_addr_low4: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x78 - EMAC MAC Address High Register"]
    pub emac_addr_high5: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x7c - EMAC MAC Address Low Register"]
    pub emac_addr_low5: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x80 - EMAC MAC Address High Register"]
    pub emac_addr_high6: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x84 - EMAC MAC Address Low Register"]
    pub emac_addr_low6: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    #[doc = "0x88 - EMAC MAC Address High Register"]
    pub emac_addr_high7: crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>,
    #[doc = "0x8c - EMAC MAC Address Low Register"]
    pub emac_addr_low7: crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>,
    _reserved32: [u8; 0x20],
    #[doc = "0xb0 - EMAC Transmit DMA Status Register"]
    pub emac_tx_dma_sta: crate::Reg<emac_tx_dma_sta::EMAC_TX_DMA_STA_SPEC>,
    #[doc = "0xb4 - EMAC Current Transmit Descriptor Register"]
    pub emac_tx_cur_desc: crate::Reg<emac_tx_cur_desc::EMAC_TX_CUR_DESC_SPEC>,
    #[doc = "0xb8 - EMAC Current Transmit Buffer Address Register"]
    pub emac_tx_cur_buf: crate::Reg<emac_tx_cur_buf::EMAC_TX_CUR_BUF_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0xc0 - EMAC Receive DMA Status Register"]
    pub emac_rx_dma_sta: crate::Reg<emac_rx_dma_sta::EMAC_RX_DMA_STA_SPEC>,
    #[doc = "0xc4 - EMAC Current Receive Descriptor Register"]
    pub emac_rx_cur_desc: crate::Reg<emac_rx_cur_desc::EMAC_RX_CUR_DESC_SPEC>,
    #[doc = "0xc8 - EMAC Current Receive Buffer Address Register"]
    pub emac_rx_cur_buf: crate::Reg<emac_rx_cur_buf::EMAC_RX_CUR_BUF_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0xd0 - EMAC RGMII Status Register"]
    pub emac_rgmii_sta: crate::Reg<emac_rgmii_sta::EMAC_RGMII_STA_SPEC>,
}
#[doc = "EMAC_BASIC_CTL0 register accessor: an alias for `Reg<EMAC_BASIC_CTL0_SPEC>`"]
pub type EMAC_BASIC_CTL0 = crate::Reg<emac_basic_ctl0::EMAC_BASIC_CTL0_SPEC>;
#[doc = "EMAC Basic Control Register0"]
pub mod emac_basic_ctl0;
#[doc = "EMAC_BASIC_CTL1 register accessor: an alias for `Reg<EMAC_BASIC_CTL1_SPEC>`"]
pub type EMAC_BASIC_CTL1 = crate::Reg<emac_basic_ctl1::EMAC_BASIC_CTL1_SPEC>;
#[doc = "EMAC Basic Control Register1"]
pub mod emac_basic_ctl1;
#[doc = "EMAC_INT_STA register accessor: an alias for `Reg<EMAC_INT_STA_SPEC>`"]
pub type EMAC_INT_STA = crate::Reg<emac_int_sta::EMAC_INT_STA_SPEC>;
#[doc = "EMAC Interrupt Status Register"]
pub mod emac_int_sta;
#[doc = "EMAC_INT_EN register accessor: an alias for `Reg<EMAC_INT_EN_SPEC>`"]
pub type EMAC_INT_EN = crate::Reg<emac_int_en::EMAC_INT_EN_SPEC>;
#[doc = "EMAC Interrupt Enable Register"]
pub mod emac_int_en;
#[doc = "EMAC_TX_CTL0 register accessor: an alias for `Reg<EMAC_TX_CTL0_SPEC>`"]
pub type EMAC_TX_CTL0 = crate::Reg<emac_tx_ctl0::EMAC_TX_CTL0_SPEC>;
#[doc = "EMAC Transmit Control Register0"]
pub mod emac_tx_ctl0;
#[doc = "EMAC_TX_CTL1 register accessor: an alias for `Reg<EMAC_TX_CTL1_SPEC>`"]
pub type EMAC_TX_CTL1 = crate::Reg<emac_tx_ctl1::EMAC_TX_CTL1_SPEC>;
#[doc = "EMAC Transmit Control Register1"]
pub mod emac_tx_ctl1;
#[doc = "EMAC_TX_FLOW_CTL register accessor: an alias for `Reg<EMAC_TX_FLOW_CTL_SPEC>`"]
pub type EMAC_TX_FLOW_CTL = crate::Reg<emac_tx_flow_ctl::EMAC_TX_FLOW_CTL_SPEC>;
#[doc = "EMAC Transmit Flow Control Register"]
pub mod emac_tx_flow_ctl;
#[doc = "EMAC_TX_DMA_DESC_LIST register accessor: an alias for `Reg<EMAC_TX_DMA_DESC_LIST_SPEC>`"]
pub type EMAC_TX_DMA_DESC_LIST = crate::Reg<emac_tx_dma_desc_list::EMAC_TX_DMA_DESC_LIST_SPEC>;
#[doc = "EMAC Transmit Descriptor List Address Register"]
pub mod emac_tx_dma_desc_list;
#[doc = "EMAC_RX_CTL0 register accessor: an alias for `Reg<EMAC_RX_CTL0_SPEC>`"]
pub type EMAC_RX_CTL0 = crate::Reg<emac_rx_ctl0::EMAC_RX_CTL0_SPEC>;
#[doc = "EMAC Receive Control Register0"]
pub mod emac_rx_ctl0;
#[doc = "EMAC_RX_CTL1 register accessor: an alias for `Reg<EMAC_RX_CTL1_SPEC>`"]
pub type EMAC_RX_CTL1 = crate::Reg<emac_rx_ctl1::EMAC_RX_CTL1_SPEC>;
#[doc = "EMAC Receive Control Register1"]
pub mod emac_rx_ctl1;
#[doc = "EMAC_RX_DMA_DESC_LIST register accessor: an alias for `Reg<EMAC_RX_DMA_DESC_LIST_SPEC>`"]
pub type EMAC_RX_DMA_DESC_LIST = crate::Reg<emac_rx_dma_desc_list::EMAC_RX_DMA_DESC_LIST_SPEC>;
#[doc = "EMAC Receive Descriptor List Address Register"]
pub mod emac_rx_dma_desc_list;
#[doc = "EMAC_RX_FRM_FLT register accessor: an alias for `Reg<EMAC_RX_FRM_FLT_SPEC>`"]
pub type EMAC_RX_FRM_FLT = crate::Reg<emac_rx_frm_flt::EMAC_RX_FRM_FLT_SPEC>;
#[doc = "EMAC Receive Frame Filter Register"]
pub mod emac_rx_frm_flt;
#[doc = "EMAC_RX_HASH0 register accessor: an alias for `Reg<EMAC_RX_HASH0_SPEC>`"]
pub type EMAC_RX_HASH0 = crate::Reg<emac_rx_hash0::EMAC_RX_HASH0_SPEC>;
#[doc = "EMAC Hash Table Register0"]
pub mod emac_rx_hash0;
#[doc = "EMAC_RX_HASH1 register accessor: an alias for `Reg<EMAC_RX_HASH1_SPEC>`"]
pub type EMAC_RX_HASH1 = crate::Reg<emac_rx_hash1::EMAC_RX_HASH1_SPEC>;
#[doc = "EMAC Hash Table Register1"]
pub mod emac_rx_hash1;
#[doc = "EMAC_MII_CMD register accessor: an alias for `Reg<EMAC_MII_CMD_SPEC>`"]
pub type EMAC_MII_CMD = crate::Reg<emac_mii_cmd::EMAC_MII_CMD_SPEC>;
#[doc = "EMAC Management Interface Command Register"]
pub mod emac_mii_cmd;
#[doc = "EMAC_MII_DATA register accessor: an alias for `Reg<EMAC_MII_DATA_SPEC>`"]
pub type EMAC_MII_DATA = crate::Reg<emac_mii_data::EMAC_MII_DATA_SPEC>;
#[doc = "EMAC Management Interface Data Register"]
pub mod emac_mii_data;
#[doc = "EMAC_ADDR_HIGH0 register accessor: an alias for `Reg<EMAC_ADDR_HIGH0_SPEC>`"]
pub type EMAC_ADDR_HIGH0 = crate::Reg<emac_addr_high0::EMAC_ADDR_HIGH0_SPEC>;
#[doc = "EMAC MAC Address High Register"]
pub mod emac_addr_high0;
#[doc = "EMAC_ADDR_HIGH register accessor: an alias for `Reg<EMAC_ADDR_HIGH_SPEC>`"]
pub type EMAC_ADDR_HIGH = crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>;
#[doc = "EMAC MAC Address High Register"]
pub mod emac_addr_high;
#[doc = "EMAC_ADDR_LOW register accessor: an alias for `Reg<EMAC_ADDR_LOW_SPEC>`"]
pub type EMAC_ADDR_LOW = crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>;
#[doc = "EMAC MAC Address Low Register"]
pub mod emac_addr_low;
#[doc = "EMAC_TX_DMA_STA register accessor: an alias for `Reg<EMAC_TX_DMA_STA_SPEC>`"]
pub type EMAC_TX_DMA_STA = crate::Reg<emac_tx_dma_sta::EMAC_TX_DMA_STA_SPEC>;
#[doc = "EMAC Transmit DMA Status Register"]
pub mod emac_tx_dma_sta;
#[doc = "EMAC_TX_CUR_DESC register accessor: an alias for `Reg<EMAC_TX_CUR_DESC_SPEC>`"]
pub type EMAC_TX_CUR_DESC = crate::Reg<emac_tx_cur_desc::EMAC_TX_CUR_DESC_SPEC>;
#[doc = "EMAC Current Transmit Descriptor Register"]
pub mod emac_tx_cur_desc;
#[doc = "EMAC_TX_CUR_BUF register accessor: an alias for `Reg<EMAC_TX_CUR_BUF_SPEC>`"]
pub type EMAC_TX_CUR_BUF = crate::Reg<emac_tx_cur_buf::EMAC_TX_CUR_BUF_SPEC>;
#[doc = "EMAC Current Transmit Buffer Address Register"]
pub mod emac_tx_cur_buf;
#[doc = "EMAC_RX_DMA_STA register accessor: an alias for `Reg<EMAC_RX_DMA_STA_SPEC>`"]
pub type EMAC_RX_DMA_STA = crate::Reg<emac_rx_dma_sta::EMAC_RX_DMA_STA_SPEC>;
#[doc = "EMAC Receive DMA Status Register"]
pub mod emac_rx_dma_sta;
#[doc = "EMAC_RX_CUR_DESC register accessor: an alias for `Reg<EMAC_RX_CUR_DESC_SPEC>`"]
pub type EMAC_RX_CUR_DESC = crate::Reg<emac_rx_cur_desc::EMAC_RX_CUR_DESC_SPEC>;
#[doc = "EMAC Current Receive Descriptor Register"]
pub mod emac_rx_cur_desc;
#[doc = "EMAC_RX_CUR_BUF register accessor: an alias for `Reg<EMAC_RX_CUR_BUF_SPEC>`"]
pub type EMAC_RX_CUR_BUF = crate::Reg<emac_rx_cur_buf::EMAC_RX_CUR_BUF_SPEC>;
#[doc = "EMAC Current Receive Buffer Address Register"]
pub mod emac_rx_cur_buf;
#[doc = "EMAC_RGMII_STA register accessor: an alias for `Reg<EMAC_RGMII_STA_SPEC>`"]
pub type EMAC_RGMII_STA = crate::Reg<emac_rgmii_sta::EMAC_RGMII_STA_SPEC>;
#[doc = "EMAC RGMII Status Register"]
pub mod emac_rgmii_sta;
