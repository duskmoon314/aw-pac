#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TWI Slave Address Register"]
    pub twi_addr: crate::Reg<twi_addr::TWI_ADDR_SPEC>,
    #[doc = "0x04 - TWI Extended Slave Address Register"]
    pub twi_xaddr: crate::Reg<twi_xaddr::TWI_XADDR_SPEC>,
    #[doc = "0x08 - TWI Data Byte Register"]
    pub twi_data: crate::Reg<twi_data::TWI_DATA_SPEC>,
    #[doc = "0x0c - TWI Control Register"]
    pub twi_cntr: crate::Reg<twi_cntr::TWI_CNTR_SPEC>,
    #[doc = "0x10 - TWI Status Register"]
    pub twi_stat: crate::Reg<twi_stat::TWI_STAT_SPEC>,
    #[doc = "0x14 - TWI Clock Control Register"]
    pub twi_ccr: crate::Reg<twi_ccr::TWI_CCR_SPEC>,
    #[doc = "0x18 - TWI Software Reset Register"]
    pub twi_srst: crate::Reg<twi_srst::TWI_SRST_SPEC>,
    #[doc = "0x1c - TWI Enhance Feature Register"]
    pub twi_efr: crate::Reg<twi_efr::TWI_EFR_SPEC>,
    #[doc = "0x20 - TWI Line Control Register"]
    pub twi_lcr: crate::Reg<twi_lcr::TWI_LCR_SPEC>,
    _reserved9: [u8; 0x01dc],
    #[doc = "0x200 - TWI_DRV Control Register"]
    pub twi_drv_ctrl: crate::Reg<twi_drv_ctrl::TWI_DRV_CTRL_SPEC>,
    #[doc = "0x204 - TWI_DRV Transmission Configuration Register"]
    pub twi_drv_cfg: crate::Reg<twi_drv_cfg::TWI_DRV_CFG_SPEC>,
    #[doc = "0x208 - TWI_DRV Slave ID Register"]
    pub twi_drv_slv: crate::Reg<twi_drv_slv::TWI_DRV_SLV_SPEC>,
    #[doc = "0x20c - TWI_DRV Packet Format Register"]
    pub twi_drv_fmt: crate::Reg<twi_drv_fmt::TWI_DRV_FMT_SPEC>,
    #[doc = "0x210 - TWI_DRV Bus Control Register"]
    pub twi_drv_bus_ctrl: crate::Reg<twi_drv_bus_ctrl::TWI_DRV_BUS_CTRL_SPEC>,
    #[doc = "0x214 - TWI_DRV Interrupt Control Register"]
    pub twi_drv_int_ctrl: crate::Reg<twi_drv_int_ctrl::TWI_DRV_INT_CTRL_SPEC>,
    #[doc = "0x218 - TWI_DRV DMA Configure Register"]
    pub twi_drv_dma_cfg: crate::Reg<twi_drv_dma_cfg::TWI_DRV_DMA_CFG_SPEC>,
    #[doc = "0x21c - TWI_DRV FIFO Content Register"]
    pub twi_drv_fifo_con: crate::Reg<twi_drv_fifo_con::TWI_DRV_FIFO_CON_SPEC>,
    _reserved17: [u8; 0xe0],
    #[doc = "0x300 - TWI_DRV Send Data FIFO Access Register"]
    pub twi_drv_send_fifo_acc: crate::Reg<twi_drv_send_fifo_acc::TWI_DRV_SEND_FIFO_ACC_SPEC>,
    #[doc = "0x304 - TWI_DRV Receive Data FIFO Access Register"]
    pub twi_drv_recv_fifo_acc: crate::Reg<twi_drv_recv_fifo_acc::TWI_DRV_RECV_FIFO_ACC_SPEC>,
}
#[doc = "twi_addr register accessor: an alias for `Reg<TWI_ADDR_SPEC>`"]
pub type TWI_ADDR = crate::Reg<twi_addr::TWI_ADDR_SPEC>;
#[doc = "TWI Slave Address Register"]
pub mod twi_addr;
#[doc = "twi_xaddr register accessor: an alias for `Reg<TWI_XADDR_SPEC>`"]
pub type TWI_XADDR = crate::Reg<twi_xaddr::TWI_XADDR_SPEC>;
#[doc = "TWI Extended Slave Address Register"]
pub mod twi_xaddr;
#[doc = "twi_data register accessor: an alias for `Reg<TWI_DATA_SPEC>`"]
pub type TWI_DATA = crate::Reg<twi_data::TWI_DATA_SPEC>;
#[doc = "TWI Data Byte Register"]
pub mod twi_data;
#[doc = "twi_cntr register accessor: an alias for `Reg<TWI_CNTR_SPEC>`"]
pub type TWI_CNTR = crate::Reg<twi_cntr::TWI_CNTR_SPEC>;
#[doc = "TWI Control Register"]
pub mod twi_cntr;
#[doc = "twi_stat register accessor: an alias for `Reg<TWI_STAT_SPEC>`"]
pub type TWI_STAT = crate::Reg<twi_stat::TWI_STAT_SPEC>;
#[doc = "TWI Status Register"]
pub mod twi_stat;
#[doc = "twi_ccr register accessor: an alias for `Reg<TWI_CCR_SPEC>`"]
pub type TWI_CCR = crate::Reg<twi_ccr::TWI_CCR_SPEC>;
#[doc = "TWI Clock Control Register"]
pub mod twi_ccr;
#[doc = "twi_srst register accessor: an alias for `Reg<TWI_SRST_SPEC>`"]
pub type TWI_SRST = crate::Reg<twi_srst::TWI_SRST_SPEC>;
#[doc = "TWI Software Reset Register"]
pub mod twi_srst;
#[doc = "twi_efr register accessor: an alias for `Reg<TWI_EFR_SPEC>`"]
pub type TWI_EFR = crate::Reg<twi_efr::TWI_EFR_SPEC>;
#[doc = "TWI Enhance Feature Register"]
pub mod twi_efr;
#[doc = "twi_lcr register accessor: an alias for `Reg<TWI_LCR_SPEC>`"]
pub type TWI_LCR = crate::Reg<twi_lcr::TWI_LCR_SPEC>;
#[doc = "TWI Line Control Register"]
pub mod twi_lcr;
#[doc = "twi_drv_ctrl register accessor: an alias for `Reg<TWI_DRV_CTRL_SPEC>`"]
pub type TWI_DRV_CTRL = crate::Reg<twi_drv_ctrl::TWI_DRV_CTRL_SPEC>;
#[doc = "TWI_DRV Control Register"]
pub mod twi_drv_ctrl;
#[doc = "twi_drv_cfg register accessor: an alias for `Reg<TWI_DRV_CFG_SPEC>`"]
pub type TWI_DRV_CFG = crate::Reg<twi_drv_cfg::TWI_DRV_CFG_SPEC>;
#[doc = "TWI_DRV Transmission Configuration Register"]
pub mod twi_drv_cfg;
#[doc = "twi_drv_slv register accessor: an alias for `Reg<TWI_DRV_SLV_SPEC>`"]
pub type TWI_DRV_SLV = crate::Reg<twi_drv_slv::TWI_DRV_SLV_SPEC>;
#[doc = "TWI_DRV Slave ID Register"]
pub mod twi_drv_slv;
#[doc = "twi_drv_fmt register accessor: an alias for `Reg<TWI_DRV_FMT_SPEC>`"]
pub type TWI_DRV_FMT = crate::Reg<twi_drv_fmt::TWI_DRV_FMT_SPEC>;
#[doc = "TWI_DRV Packet Format Register"]
pub mod twi_drv_fmt;
#[doc = "twi_drv_bus_ctrl register accessor: an alias for `Reg<TWI_DRV_BUS_CTRL_SPEC>`"]
pub type TWI_DRV_BUS_CTRL = crate::Reg<twi_drv_bus_ctrl::TWI_DRV_BUS_CTRL_SPEC>;
#[doc = "TWI_DRV Bus Control Register"]
pub mod twi_drv_bus_ctrl;
#[doc = "twi_drv_int_ctrl register accessor: an alias for `Reg<TWI_DRV_INT_CTRL_SPEC>`"]
pub type TWI_DRV_INT_CTRL = crate::Reg<twi_drv_int_ctrl::TWI_DRV_INT_CTRL_SPEC>;
#[doc = "TWI_DRV Interrupt Control Register"]
pub mod twi_drv_int_ctrl;
#[doc = "twi_drv_dma_cfg register accessor: an alias for `Reg<TWI_DRV_DMA_CFG_SPEC>`"]
pub type TWI_DRV_DMA_CFG = crate::Reg<twi_drv_dma_cfg::TWI_DRV_DMA_CFG_SPEC>;
#[doc = "TWI_DRV DMA Configure Register"]
pub mod twi_drv_dma_cfg;
#[doc = "twi_drv_fifo_con register accessor: an alias for `Reg<TWI_DRV_FIFO_CON_SPEC>`"]
pub type TWI_DRV_FIFO_CON = crate::Reg<twi_drv_fifo_con::TWI_DRV_FIFO_CON_SPEC>;
#[doc = "TWI_DRV FIFO Content Register"]
pub mod twi_drv_fifo_con;
#[doc = "twi_drv_send_fifo_acc register accessor: an alias for `Reg<TWI_DRV_SEND_FIFO_ACC_SPEC>`"]
pub type TWI_DRV_SEND_FIFO_ACC = crate::Reg<twi_drv_send_fifo_acc::TWI_DRV_SEND_FIFO_ACC_SPEC>;
#[doc = "TWI_DRV Send Data FIFO Access Register"]
pub mod twi_drv_send_fifo_acc;
#[doc = "twi_drv_recv_fifo_acc register accessor: an alias for `Reg<TWI_DRV_RECV_FIFO_ACC_SPEC>`"]
pub type TWI_DRV_RECV_FIFO_ACC = crate::Reg<twi_drv_recv_fifo_acc::TWI_DRV_RECV_FIFO_ACC_SPEC>;
#[doc = "TWI_DRV Receive Data FIFO Access Register"]
pub mod twi_drv_recv_fifo_acc;
