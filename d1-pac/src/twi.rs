#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    twi_addr: TWI_ADDR,
    twi_xaddr: TWI_XADDR,
    twi_data: TWI_DATA,
    twi_cntr: TWI_CNTR,
    twi_stat: TWI_STAT,
    twi_ccr: TWI_CCR,
    twi_srst: TWI_SRST,
    twi_efr: TWI_EFR,
    twi_lcr: TWI_LCR,
    _reserved9: [u8; 0x01dc],
    twi_drv_ctrl: TWI_DRV_CTRL,
    twi_drv_cfg: TWI_DRV_CFG,
    twi_drv_slv: TWI_DRV_SLV,
    twi_drv_fmt: TWI_DRV_FMT,
    twi_drv_bus_ctrl: TWI_DRV_BUS_CTRL,
    twi_drv_int_ctrl: TWI_DRV_INT_CTRL,
    twi_drv_dma_cfg: TWI_DRV_DMA_CFG,
    twi_drv_fifo_con: TWI_DRV_FIFO_CON,
    _reserved17: [u8; 0xe0],
    twi_drv_send_fifo_acc: TWI_DRV_SEND_FIFO_ACC,
    twi_drv_recv_fifo_acc: TWI_DRV_RECV_FIFO_ACC,
}
impl RegisterBlock {
    #[doc = "0x00 - TWI Slave Address Register"]
    #[inline(always)]
    pub const fn twi_addr(&self) -> &TWI_ADDR {
        &self.twi_addr
    }
    #[doc = "0x04 - TWI Extended Slave Address Register"]
    #[inline(always)]
    pub const fn twi_xaddr(&self) -> &TWI_XADDR {
        &self.twi_xaddr
    }
    #[doc = "0x08 - TWI Data Byte Register"]
    #[inline(always)]
    pub const fn twi_data(&self) -> &TWI_DATA {
        &self.twi_data
    }
    #[doc = "0x0c - TWI Control Register"]
    #[inline(always)]
    pub const fn twi_cntr(&self) -> &TWI_CNTR {
        &self.twi_cntr
    }
    #[doc = "0x10 - TWI Status Register"]
    #[inline(always)]
    pub const fn twi_stat(&self) -> &TWI_STAT {
        &self.twi_stat
    }
    #[doc = "0x14 - TWI Clock Control Register"]
    #[inline(always)]
    pub const fn twi_ccr(&self) -> &TWI_CCR {
        &self.twi_ccr
    }
    #[doc = "0x18 - TWI Software Reset Register"]
    #[inline(always)]
    pub const fn twi_srst(&self) -> &TWI_SRST {
        &self.twi_srst
    }
    #[doc = "0x1c - TWI Enhance Feature Register"]
    #[inline(always)]
    pub const fn twi_efr(&self) -> &TWI_EFR {
        &self.twi_efr
    }
    #[doc = "0x20 - TWI Line Control Register"]
    #[inline(always)]
    pub const fn twi_lcr(&self) -> &TWI_LCR {
        &self.twi_lcr
    }
    #[doc = "0x200 - TWI_DRV Control Register"]
    #[inline(always)]
    pub const fn twi_drv_ctrl(&self) -> &TWI_DRV_CTRL {
        &self.twi_drv_ctrl
    }
    #[doc = "0x204 - TWI_DRV Transmission Configuration Register"]
    #[inline(always)]
    pub const fn twi_drv_cfg(&self) -> &TWI_DRV_CFG {
        &self.twi_drv_cfg
    }
    #[doc = "0x208 - TWI_DRV Slave ID Register"]
    #[inline(always)]
    pub const fn twi_drv_slv(&self) -> &TWI_DRV_SLV {
        &self.twi_drv_slv
    }
    #[doc = "0x20c - TWI_DRV Packet Format Register"]
    #[inline(always)]
    pub const fn twi_drv_fmt(&self) -> &TWI_DRV_FMT {
        &self.twi_drv_fmt
    }
    #[doc = "0x210 - TWI_DRV Bus Control Register"]
    #[inline(always)]
    pub const fn twi_drv_bus_ctrl(&self) -> &TWI_DRV_BUS_CTRL {
        &self.twi_drv_bus_ctrl
    }
    #[doc = "0x214 - TWI_DRV Interrupt Control Register"]
    #[inline(always)]
    pub const fn twi_drv_int_ctrl(&self) -> &TWI_DRV_INT_CTRL {
        &self.twi_drv_int_ctrl
    }
    #[doc = "0x218 - TWI_DRV DMA Configure Register"]
    #[inline(always)]
    pub const fn twi_drv_dma_cfg(&self) -> &TWI_DRV_DMA_CFG {
        &self.twi_drv_dma_cfg
    }
    #[doc = "0x21c - TWI_DRV FIFO Content Register"]
    #[inline(always)]
    pub const fn twi_drv_fifo_con(&self) -> &TWI_DRV_FIFO_CON {
        &self.twi_drv_fifo_con
    }
    #[doc = "0x300 - TWI_DRV Send Data FIFO Access Register"]
    #[inline(always)]
    pub const fn twi_drv_send_fifo_acc(&self) -> &TWI_DRV_SEND_FIFO_ACC {
        &self.twi_drv_send_fifo_acc
    }
    #[doc = "0x304 - TWI_DRV Receive Data FIFO Access Register"]
    #[inline(always)]
    pub const fn twi_drv_recv_fifo_acc(&self) -> &TWI_DRV_RECV_FIFO_ACC {
        &self.twi_drv_recv_fifo_acc
    }
}
#[doc = "twi_addr (rw) register accessor: TWI Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_addr`] module"]
pub type TWI_ADDR = crate::Reg<twi_addr::TWI_ADDR_SPEC>;
#[doc = "TWI Slave Address Register"]
pub mod twi_addr;
#[doc = "twi_xaddr (rw) register accessor: TWI Extended Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_xaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_xaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_xaddr`] module"]
pub type TWI_XADDR = crate::Reg<twi_xaddr::TWI_XADDR_SPEC>;
#[doc = "TWI Extended Slave Address Register"]
pub mod twi_xaddr;
#[doc = "twi_data (rw) register accessor: TWI Data Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_data`] module"]
pub type TWI_DATA = crate::Reg<twi_data::TWI_DATA_SPEC>;
#[doc = "TWI Data Byte Register"]
pub mod twi_data;
#[doc = "twi_cntr (rw) register accessor: TWI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_cntr`] module"]
pub type TWI_CNTR = crate::Reg<twi_cntr::TWI_CNTR_SPEC>;
#[doc = "TWI Control Register"]
pub mod twi_cntr;
#[doc = "twi_stat (r) register accessor: TWI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_stat`] module"]
pub type TWI_STAT = crate::Reg<twi_stat::TWI_STAT_SPEC>;
#[doc = "TWI Status Register"]
pub mod twi_stat;
#[doc = "twi_ccr (rw) register accessor: TWI Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_ccr`] module"]
pub type TWI_CCR = crate::Reg<twi_ccr::TWI_CCR_SPEC>;
#[doc = "TWI Clock Control Register"]
pub mod twi_ccr;
#[doc = "twi_srst (rw) register accessor: TWI Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_srst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_srst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_srst`] module"]
pub type TWI_SRST = crate::Reg<twi_srst::TWI_SRST_SPEC>;
#[doc = "TWI Software Reset Register"]
pub mod twi_srst;
#[doc = "twi_efr (rw) register accessor: TWI Enhance Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_efr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_efr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_efr`] module"]
pub type TWI_EFR = crate::Reg<twi_efr::TWI_EFR_SPEC>;
#[doc = "TWI Enhance Feature Register"]
pub mod twi_efr;
#[doc = "twi_lcr (rw) register accessor: TWI Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_lcr`] module"]
pub type TWI_LCR = crate::Reg<twi_lcr::TWI_LCR_SPEC>;
#[doc = "TWI Line Control Register"]
pub mod twi_lcr;
#[doc = "twi_drv_ctrl (rw) register accessor: TWI_DRV Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_ctrl`] module"]
pub type TWI_DRV_CTRL = crate::Reg<twi_drv_ctrl::TWI_DRV_CTRL_SPEC>;
#[doc = "TWI_DRV Control Register"]
pub mod twi_drv_ctrl;
#[doc = "twi_drv_cfg (rw) register accessor: TWI_DRV Transmission Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_cfg`] module"]
pub type TWI_DRV_CFG = crate::Reg<twi_drv_cfg::TWI_DRV_CFG_SPEC>;
#[doc = "TWI_DRV Transmission Configuration Register"]
pub mod twi_drv_cfg;
#[doc = "twi_drv_slv (rw) register accessor: TWI_DRV Slave ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_slv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_slv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_slv`] module"]
pub type TWI_DRV_SLV = crate::Reg<twi_drv_slv::TWI_DRV_SLV_SPEC>;
#[doc = "TWI_DRV Slave ID Register"]
pub mod twi_drv_slv;
#[doc = "twi_drv_fmt (rw) register accessor: TWI_DRV Packet Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_fmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_fmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_fmt`] module"]
pub type TWI_DRV_FMT = crate::Reg<twi_drv_fmt::TWI_DRV_FMT_SPEC>;
#[doc = "TWI_DRV Packet Format Register"]
pub mod twi_drv_fmt;
#[doc = "twi_drv_bus_ctrl (rw) register accessor: TWI_DRV Bus Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_bus_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_bus_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_bus_ctrl`] module"]
pub type TWI_DRV_BUS_CTRL = crate::Reg<twi_drv_bus_ctrl::TWI_DRV_BUS_CTRL_SPEC>;
#[doc = "TWI_DRV Bus Control Register"]
pub mod twi_drv_bus_ctrl;
#[doc = "twi_drv_int_ctrl (rw) register accessor: TWI_DRV Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_int_ctrl`] module"]
pub type TWI_DRV_INT_CTRL = crate::Reg<twi_drv_int_ctrl::TWI_DRV_INT_CTRL_SPEC>;
#[doc = "TWI_DRV Interrupt Control Register"]
pub mod twi_drv_int_ctrl;
#[doc = "twi_drv_dma_cfg (rw) register accessor: TWI_DRV DMA Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_dma_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_dma_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_dma_cfg`] module"]
pub type TWI_DRV_DMA_CFG = crate::Reg<twi_drv_dma_cfg::TWI_DRV_DMA_CFG_SPEC>;
#[doc = "TWI_DRV DMA Configure Register"]
pub mod twi_drv_dma_cfg;
#[doc = "twi_drv_fifo_con (rw) register accessor: TWI_DRV FIFO Content Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_fifo_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_fifo_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_fifo_con`] module"]
pub type TWI_DRV_FIFO_CON = crate::Reg<twi_drv_fifo_con::TWI_DRV_FIFO_CON_SPEC>;
#[doc = "TWI_DRV FIFO Content Register"]
pub mod twi_drv_fifo_con;
#[doc = "twi_drv_send_fifo_acc (w) register accessor: TWI_DRV Send Data FIFO Access Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_send_fifo_acc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_send_fifo_acc`] module"]
pub type TWI_DRV_SEND_FIFO_ACC = crate::Reg<twi_drv_send_fifo_acc::TWI_DRV_SEND_FIFO_ACC_SPEC>;
#[doc = "TWI_DRV Send Data FIFO Access Register"]
pub mod twi_drv_send_fifo_acc;
#[doc = "twi_drv_recv_fifo_acc (r) register accessor: TWI_DRV Receive Data FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_recv_fifo_acc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_drv_recv_fifo_acc`] module"]
pub type TWI_DRV_RECV_FIFO_ACC = crate::Reg<twi_drv_recv_fifo_acc::TWI_DRV_RECV_FIFO_ACC_SPEC>;
#[doc = "TWI_DRV Receive Data FIFO Access Register"]
pub mod twi_drv_recv_fifo_acc;
