#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EMAC Basic Control Register0"]
    pub emac_basic_ctl0: EMAC_BASIC_CTL0,
    #[doc = "0x04 - EMAC Basic Control Register1"]
    pub emac_basic_ctl1: EMAC_BASIC_CTL1,
    #[doc = "0x08 - EMAC Interrupt Status Register"]
    pub emac_int_sta: EMAC_INT_STA,
    #[doc = "0x0c - EMAC Interrupt Enable Register"]
    pub emac_int_en: EMAC_INT_EN,
    #[doc = "0x10 - EMAC Transmit Control Register0"]
    pub emac_tx_ctl0: EMAC_TX_CTL0,
    #[doc = "0x14 - EMAC Transmit Control Register1"]
    pub emac_tx_ctl1: EMAC_TX_CTL1,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - EMAC Transmit Flow Control Register"]
    pub emac_tx_flow_ctl: EMAC_TX_FLOW_CTL,
    #[doc = "0x20 - EMAC Transmit Descriptor List Address Register"]
    pub emac_tx_dma_desc_list: EMAC_TX_DMA_DESC_LIST,
    #[doc = "0x24 - EMAC Receive Control Register0"]
    pub emac_rx_ctl0: EMAC_RX_CTL0,
    #[doc = "0x28 - EMAC Receive Control Register1"]
    pub emac_rx_ctl1: EMAC_RX_CTL1,
    _reserved10: [u8; 0x08],
    #[doc = "0x34 - EMAC Receive Descriptor List Address Register"]
    pub emac_rx_dma_desc_list: EMAC_RX_DMA_DESC_LIST,
    #[doc = "0x38 - EMAC Receive Frame Filter Register"]
    pub emac_rx_frm_flt: EMAC_RX_FRM_FLT,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - EMAC Hash Table Register0"]
    pub emac_rx_hash0: EMAC_RX_HASH0,
    #[doc = "0x44 - EMAC Hash Table Register1"]
    pub emac_rx_hash1: EMAC_RX_HASH1,
    #[doc = "0x48 - EMAC Management Interface Command Register"]
    pub emac_mii_cmd: EMAC_MII_CMD,
    #[doc = "0x4c - EMAC Management Interface Data Register"]
    pub emac_mii_data: EMAC_MII_DATA,
    #[doc = "0x50 - EMAC MAC Address High Register"]
    pub emac_addr_high0: EMAC_ADDR_HIGH0,
    #[doc = "0x54 - EMAC MAC Address Low Register"]
    pub emac_addr_low0: EMAC_ADDR_LOW,
    #[doc = "0x58 - EMAC MAC Address High Register"]
    pub emac_addr_high1: EMAC_ADDR_HIGH,
    #[doc = "0x5c - EMAC MAC Address Low Register"]
    pub emac_addr_low1: EMAC_ADDR_LOW,
    #[doc = "0x60 - EMAC MAC Address High Register"]
    pub emac_addr_high2: EMAC_ADDR_HIGH,
    #[doc = "0x64 - EMAC MAC Address Low Register"]
    pub emac_addr_low2: EMAC_ADDR_LOW,
    #[doc = "0x68 - EMAC MAC Address High Register"]
    pub emac_addr_high3: EMAC_ADDR_HIGH,
    #[doc = "0x6c - EMAC MAC Address Low Register"]
    pub emac_addr_low3: EMAC_ADDR_LOW,
    #[doc = "0x70 - EMAC MAC Address High Register"]
    pub emac_addr_high4: EMAC_ADDR_HIGH,
    #[doc = "0x74 - EMAC MAC Address Low Register"]
    pub emac_addr_low4: EMAC_ADDR_LOW,
    #[doc = "0x78 - EMAC MAC Address High Register"]
    pub emac_addr_high5: EMAC_ADDR_HIGH,
    #[doc = "0x7c - EMAC MAC Address Low Register"]
    pub emac_addr_low5: EMAC_ADDR_LOW,
    #[doc = "0x80 - EMAC MAC Address High Register"]
    pub emac_addr_high6: EMAC_ADDR_HIGH,
    #[doc = "0x84 - EMAC MAC Address Low Register"]
    pub emac_addr_low6: EMAC_ADDR_LOW,
    #[doc = "0x88 - EMAC MAC Address High Register"]
    pub emac_addr_high7: EMAC_ADDR_HIGH,
    #[doc = "0x8c - EMAC MAC Address Low Register"]
    pub emac_addr_low7: EMAC_ADDR_LOW,
    _reserved32: [u8; 0x20],
    #[doc = "0xb0 - EMAC Transmit DMA Status Register"]
    pub emac_tx_dma_sta: EMAC_TX_DMA_STA,
    #[doc = "0xb4 - EMAC Current Transmit Descriptor Register"]
    pub emac_tx_cur_desc: EMAC_TX_CUR_DESC,
    #[doc = "0xb8 - EMAC Current Transmit Buffer Address Register"]
    pub emac_tx_cur_buf: EMAC_TX_CUR_BUF,
    _reserved35: [u8; 0x04],
    #[doc = "0xc0 - EMAC Receive DMA Status Register"]
    pub emac_rx_dma_sta: EMAC_RX_DMA_STA,
    #[doc = "0xc4 - EMAC Current Receive Descriptor Register"]
    pub emac_rx_cur_desc: EMAC_RX_CUR_DESC,
    #[doc = "0xc8 - EMAC Current Receive Buffer Address Register"]
    pub emac_rx_cur_buf: EMAC_RX_CUR_BUF,
    _reserved38: [u8; 0x04],
    #[doc = "0xd0 - EMAC RGMII Status Register"]
    pub emac_rgmii_sta: EMAC_RGMII_STA,
}
#[doc = "emac_basic_ctl0 (rw) register accessor: an alias for `Reg<EMAC_BASIC_CTL0_SPEC>`"]
pub type EMAC_BASIC_CTL0 = crate::Reg<emac_basic_ctl0::EMAC_BASIC_CTL0_SPEC>;
#[doc = "EMAC Basic Control Register0"]
pub mod emac_basic_ctl0;
#[doc = "emac_basic_ctl1 (rw) register accessor: an alias for `Reg<EMAC_BASIC_CTL1_SPEC>`"]
pub type EMAC_BASIC_CTL1 = crate::Reg<emac_basic_ctl1::EMAC_BASIC_CTL1_SPEC>;
#[doc = "EMAC Basic Control Register1"]
pub mod emac_basic_ctl1;
#[doc = "emac_int_sta (rw) register accessor: an alias for `Reg<EMAC_INT_STA_SPEC>`"]
pub type EMAC_INT_STA = crate::Reg<emac_int_sta::EMAC_INT_STA_SPEC>;
#[doc = "EMAC Interrupt Status Register"]
pub mod emac_int_sta;
#[doc = "emac_int_en (rw) register accessor: an alias for `Reg<EMAC_INT_EN_SPEC>`"]
pub type EMAC_INT_EN = crate::Reg<emac_int_en::EMAC_INT_EN_SPEC>;
#[doc = "EMAC Interrupt Enable Register"]
pub mod emac_int_en;
#[doc = "emac_tx_ctl0 (rw) register accessor: an alias for `Reg<EMAC_TX_CTL0_SPEC>`"]
pub type EMAC_TX_CTL0 = crate::Reg<emac_tx_ctl0::EMAC_TX_CTL0_SPEC>;
#[doc = "EMAC Transmit Control Register0"]
pub mod emac_tx_ctl0;
#[doc = "emac_tx_ctl1 (rw) register accessor: an alias for `Reg<EMAC_TX_CTL1_SPEC>`"]
pub type EMAC_TX_CTL1 = crate::Reg<emac_tx_ctl1::EMAC_TX_CTL1_SPEC>;
#[doc = "EMAC Transmit Control Register1"]
pub mod emac_tx_ctl1;
#[doc = "emac_tx_flow_ctl (rw) register accessor: an alias for `Reg<EMAC_TX_FLOW_CTL_SPEC>`"]
pub type EMAC_TX_FLOW_CTL = crate::Reg<emac_tx_flow_ctl::EMAC_TX_FLOW_CTL_SPEC>;
#[doc = "EMAC Transmit Flow Control Register"]
pub mod emac_tx_flow_ctl;
#[doc = "emac_tx_dma_desc_list (rw) register accessor: an alias for `Reg<EMAC_TX_DMA_DESC_LIST_SPEC>`"]
pub type EMAC_TX_DMA_DESC_LIST = crate::Reg<emac_tx_dma_desc_list::EMAC_TX_DMA_DESC_LIST_SPEC>;
#[doc = "EMAC Transmit Descriptor List Address Register"]
pub mod emac_tx_dma_desc_list;
#[doc = "emac_rx_ctl0 (rw) register accessor: an alias for `Reg<EMAC_RX_CTL0_SPEC>`"]
pub type EMAC_RX_CTL0 = crate::Reg<emac_rx_ctl0::EMAC_RX_CTL0_SPEC>;
#[doc = "EMAC Receive Control Register0"]
pub mod emac_rx_ctl0;
#[doc = "emac_rx_ctl1 (rw) register accessor: an alias for `Reg<EMAC_RX_CTL1_SPEC>`"]
pub type EMAC_RX_CTL1 = crate::Reg<emac_rx_ctl1::EMAC_RX_CTL1_SPEC>;
#[doc = "EMAC Receive Control Register1"]
pub mod emac_rx_ctl1;
#[doc = "emac_rx_dma_desc_list (rw) register accessor: an alias for `Reg<EMAC_RX_DMA_DESC_LIST_SPEC>`"]
pub type EMAC_RX_DMA_DESC_LIST = crate::Reg<emac_rx_dma_desc_list::EMAC_RX_DMA_DESC_LIST_SPEC>;
#[doc = "EMAC Receive Descriptor List Address Register"]
pub mod emac_rx_dma_desc_list;
#[doc = "emac_rx_frm_flt (rw) register accessor: an alias for `Reg<EMAC_RX_FRM_FLT_SPEC>`"]
pub type EMAC_RX_FRM_FLT = crate::Reg<emac_rx_frm_flt::EMAC_RX_FRM_FLT_SPEC>;
#[doc = "EMAC Receive Frame Filter Register"]
pub mod emac_rx_frm_flt;
#[doc = "emac_rx_hash0 (rw) register accessor: an alias for `Reg<EMAC_RX_HASH0_SPEC>`"]
pub type EMAC_RX_HASH0 = crate::Reg<emac_rx_hash0::EMAC_RX_HASH0_SPEC>;
#[doc = "EMAC Hash Table Register0"]
pub mod emac_rx_hash0;
#[doc = "emac_rx_hash1 (rw) register accessor: an alias for `Reg<EMAC_RX_HASH1_SPEC>`"]
pub type EMAC_RX_HASH1 = crate::Reg<emac_rx_hash1::EMAC_RX_HASH1_SPEC>;
#[doc = "EMAC Hash Table Register1"]
pub mod emac_rx_hash1;
#[doc = "emac_mii_cmd (rw) register accessor: an alias for `Reg<EMAC_MII_CMD_SPEC>`"]
pub type EMAC_MII_CMD = crate::Reg<emac_mii_cmd::EMAC_MII_CMD_SPEC>;
#[doc = "EMAC Management Interface Command Register"]
pub mod emac_mii_cmd;
#[doc = "emac_mii_data (rw) register accessor: an alias for `Reg<EMAC_MII_DATA_SPEC>`"]
pub type EMAC_MII_DATA = crate::Reg<emac_mii_data::EMAC_MII_DATA_SPEC>;
#[doc = "EMAC Management Interface Data Register"]
pub mod emac_mii_data;
#[doc = "emac_addr_high0 (rw) register accessor: an alias for `Reg<EMAC_ADDR_HIGH0_SPEC>`"]
pub type EMAC_ADDR_HIGH0 = crate::Reg<emac_addr_high0::EMAC_ADDR_HIGH0_SPEC>;
#[doc = "EMAC MAC Address High Register"]
pub mod emac_addr_high0;
#[doc = "emac_addr_high (rw) register accessor: an alias for `Reg<EMAC_ADDR_HIGH_SPEC>`"]
pub type EMAC_ADDR_HIGH = crate::Reg<emac_addr_high::EMAC_ADDR_HIGH_SPEC>;
#[doc = "EMAC MAC Address High Register"]
pub mod emac_addr_high;
#[doc = "emac_addr_low (rw) register accessor: an alias for `Reg<EMAC_ADDR_LOW_SPEC>`"]
pub type EMAC_ADDR_LOW = crate::Reg<emac_addr_low::EMAC_ADDR_LOW_SPEC>;
#[doc = "EMAC MAC Address Low Register"]
pub mod emac_addr_low;
#[doc = "emac_tx_dma_sta (r) register accessor: an alias for `Reg<EMAC_TX_DMA_STA_SPEC>`"]
pub type EMAC_TX_DMA_STA = crate::Reg<emac_tx_dma_sta::EMAC_TX_DMA_STA_SPEC>;
#[doc = "EMAC Transmit DMA Status Register"]
pub mod emac_tx_dma_sta;
#[doc = "emac_tx_cur_desc (r) register accessor: an alias for `Reg<EMAC_TX_CUR_DESC_SPEC>`"]
pub type EMAC_TX_CUR_DESC = crate::Reg<emac_tx_cur_desc::EMAC_TX_CUR_DESC_SPEC>;
#[doc = "EMAC Current Transmit Descriptor Register"]
pub mod emac_tx_cur_desc;
#[doc = "emac_tx_cur_buf (r) register accessor: an alias for `Reg<EMAC_TX_CUR_BUF_SPEC>`"]
pub type EMAC_TX_CUR_BUF = crate::Reg<emac_tx_cur_buf::EMAC_TX_CUR_BUF_SPEC>;
#[doc = "EMAC Current Transmit Buffer Address Register"]
pub mod emac_tx_cur_buf;
#[doc = "emac_rx_dma_sta (r) register accessor: an alias for `Reg<EMAC_RX_DMA_STA_SPEC>`"]
pub type EMAC_RX_DMA_STA = crate::Reg<emac_rx_dma_sta::EMAC_RX_DMA_STA_SPEC>;
#[doc = "EMAC Receive DMA Status Register"]
pub mod emac_rx_dma_sta;
#[doc = "emac_rx_cur_desc (r) register accessor: an alias for `Reg<EMAC_RX_CUR_DESC_SPEC>`"]
pub type EMAC_RX_CUR_DESC = crate::Reg<emac_rx_cur_desc::EMAC_RX_CUR_DESC_SPEC>;
#[doc = "EMAC Current Receive Descriptor Register"]
pub mod emac_rx_cur_desc;
#[doc = "emac_rx_cur_buf (r) register accessor: an alias for `Reg<EMAC_RX_CUR_BUF_SPEC>`"]
pub type EMAC_RX_CUR_BUF = crate::Reg<emac_rx_cur_buf::EMAC_RX_CUR_BUF_SPEC>;
#[doc = "EMAC Current Receive Buffer Address Register"]
pub mod emac_rx_cur_buf;
#[doc = "emac_rgmii_sta (rw) register accessor: an alias for `Reg<EMAC_RGMII_STA_SPEC>`"]
pub type EMAC_RGMII_STA = crate::Reg<emac_rgmii_sta::EMAC_RGMII_STA_SPEC>;
#[doc = "EMAC RGMII Status Register"]
pub mod emac_rgmii_sta;
