#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    riscv_sta_add0: RISCV_STA_ADD0,
    riscv_sta_add1: RISCV_STA_ADD1,
    _reserved2: [u8; 0x04],
    rf1p_cfg: RF1P_CFG,
    _reserved3: [u8; 0x08],
    rom_cfg: ROM_CFG,
    wakeup_en: WAKEUP_EN,
    wakeup_mask: [WAKEUP_MASK; 5],
    _reserved6: [u8; 0x08],
    ts_tmode_sel: TS_TMODE_SEL,
    sram_addr_twist: SRAM_ADDR_TWIST,
    work_mode: WORK_MODE,
    _reserved9: [u8; 0x04],
    retite_pc0: RETITE_PC0,
    retite_pc1: RETITE_PC1,
    _reserved11: [u8; 0x08],
    irq_mode: [IRQ_MODE; 5],
    _reserved12: [u8; 0x90],
    riscv_axi_pmu_ctrl: RISCV_AXI_PMU_CTRL,
    riscv_axi_pmu_prd: RISCV_AXI_PMU_PRD,
    riscv_axi_pmu_lat_rd: RISCV_AXI_PMU_LAT_RD,
    riscv_axi_pmu_lat_wr: RISCV_AXI_PMU_LAT_WR,
    riscv_axi_pmu_req_rd: RISCV_AXI_PMU_REQ_RD,
    riscv_axi_pmu_req_wr: RISCV_AXI_PMU_REQ_WR,
    riscv_axi_pmu_bw_rd: RISCV_AXI_PMU_BW_RD,
    riscv_axi_pmu_bw_wr: RISCV_AXI_PMU_BW_WR,
}
impl RegisterBlock {
    #[doc = "0x04 - RISCV Start Address0 Register"]
    #[inline(always)]
    pub const fn riscv_sta_add0(&self) -> &RISCV_STA_ADD0 {
        &self.riscv_sta_add0
    }
    #[doc = "0x08 - RISCV Start Address1 Register"]
    #[inline(always)]
    pub const fn riscv_sta_add1(&self) -> &RISCV_STA_ADD1 {
        &self.riscv_sta_add1
    }
    #[doc = "0x10 - RF1P Configuration Register"]
    #[inline(always)]
    pub const fn rf1p_cfg(&self) -> &RF1P_CFG {
        &self.rf1p_cfg
    }
    #[doc = "0x1c - ROM Configuration Register"]
    #[inline(always)]
    pub const fn rom_cfg(&self) -> &ROM_CFG {
        &self.rom_cfg
    }
    #[doc = "0x20 - Wakeup Enable Register"]
    #[inline(always)]
    pub const fn wakeup_en(&self) -> &WAKEUP_EN {
        &self.wakeup_en
    }
    #[doc = "0x24..0x38 - Wakeup Mask Register"]
    #[inline(always)]
    pub const fn wakeup_mask(&self, n: usize) -> &WAKEUP_MASK {
        &self.wakeup_mask[n]
    }
    #[doc = "0x40 - Timestamp Test Mode Select Register"]
    #[inline(always)]
    pub const fn ts_tmode_sel(&self) -> &TS_TMODE_SEL {
        &self.ts_tmode_sel
    }
    #[doc = "0x44 - SRAM Address Twist Register"]
    #[inline(always)]
    pub const fn sram_addr_twist(&self) -> &SRAM_ADDR_TWIST {
        &self.sram_addr_twist
    }
    #[doc = "0x48 - Work Mode Register"]
    #[inline(always)]
    pub const fn work_mode(&self) -> &WORK_MODE {
        &self.work_mode
    }
    #[doc = "0x50 - Retire PC0 Register"]
    #[inline(always)]
    pub const fn retite_pc0(&self) -> &RETITE_PC0 {
        &self.retite_pc0
    }
    #[doc = "0x54 - Retire PC1 Register"]
    #[inline(always)]
    pub const fn retite_pc1(&self) -> &RETITE_PC1 {
        &self.retite_pc1
    }
    #[doc = "0x60..0x74 - IRQ Mode Register"]
    #[inline(always)]
    pub const fn irq_mode(&self, n: usize) -> &IRQ_MODE {
        &self.irq_mode[n]
    }
    #[doc = "0x104 - RISCV AXI PMU Control Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_ctrl(&self) -> &RISCV_AXI_PMU_CTRL {
        &self.riscv_axi_pmu_ctrl
    }
    #[doc = "0x108 - RISCV AXI PMU Period Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_prd(&self) -> &RISCV_AXI_PMU_PRD {
        &self.riscv_axi_pmu_prd
    }
    #[doc = "0x10c - RISCV AXI PMU Read Latency Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_lat_rd(&self) -> &RISCV_AXI_PMU_LAT_RD {
        &self.riscv_axi_pmu_lat_rd
    }
    #[doc = "0x110 - RISCV AXI PMU Write Latency Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_lat_wr(&self) -> &RISCV_AXI_PMU_LAT_WR {
        &self.riscv_axi_pmu_lat_wr
    }
    #[doc = "0x114 - RISCV AXI PMU Read Request Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_req_rd(&self) -> &RISCV_AXI_PMU_REQ_RD {
        &self.riscv_axi_pmu_req_rd
    }
    #[doc = "0x118 - RISCV AXI PMU Write Request Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_req_wr(&self) -> &RISCV_AXI_PMU_REQ_WR {
        &self.riscv_axi_pmu_req_wr
    }
    #[doc = "0x11c - RISCV AXI PMU Read Bandwidth Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_bw_rd(&self) -> &RISCV_AXI_PMU_BW_RD {
        &self.riscv_axi_pmu_bw_rd
    }
    #[doc = "0x120 - RISCV AXI PMU Write Bandwidth Register"]
    #[inline(always)]
    pub const fn riscv_axi_pmu_bw_wr(&self) -> &RISCV_AXI_PMU_BW_WR {
        &self.riscv_axi_pmu_bw_wr
    }
}
#[doc = "riscv_sta_add0 (rw) register accessor: RISCV Start Address0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_sta_add0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_sta_add0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_sta_add0`] module"]
pub type RISCV_STA_ADD0 = crate::Reg<riscv_sta_add0::RISCV_STA_ADD0_SPEC>;
#[doc = "RISCV Start Address0 Register"]
pub mod riscv_sta_add0;
#[doc = "riscv_sta_add1 (rw) register accessor: RISCV Start Address1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_sta_add1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_sta_add1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_sta_add1`] module"]
pub type RISCV_STA_ADD1 = crate::Reg<riscv_sta_add1::RISCV_STA_ADD1_SPEC>;
#[doc = "RISCV Start Address1 Register"]
pub mod riscv_sta_add1;
#[doc = "rf1p_cfg (rw) register accessor: RF1P Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf1p_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf1p_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf1p_cfg`] module"]
pub type RF1P_CFG = crate::Reg<rf1p_cfg::RF1P_CFG_SPEC>;
#[doc = "RF1P Configuration Register"]
pub mod rf1p_cfg;
#[doc = "rom_cfg (rw) register accessor: ROM Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_cfg`] module"]
pub type ROM_CFG = crate::Reg<rom_cfg::ROM_CFG_SPEC>;
#[doc = "ROM Configuration Register"]
pub mod rom_cfg;
#[doc = "wakeup_en (rw) register accessor: Wakeup Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_en`] module"]
pub type WAKEUP_EN = crate::Reg<wakeup_en::WAKEUP_EN_SPEC>;
#[doc = "Wakeup Enable Register"]
pub mod wakeup_en;
#[doc = "wakeup_mask (rw) register accessor: Wakeup Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_mask`] module"]
pub type WAKEUP_MASK = crate::Reg<wakeup_mask::WAKEUP_MASK_SPEC>;
#[doc = "Wakeup Mask Register"]
pub mod wakeup_mask;
#[doc = "ts_tmode_sel (rw) register accessor: Timestamp Test Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_tmode_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_tmode_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_tmode_sel`] module"]
pub type TS_TMODE_SEL = crate::Reg<ts_tmode_sel::TS_TMODE_SEL_SPEC>;
#[doc = "Timestamp Test Mode Select Register"]
pub mod ts_tmode_sel;
#[doc = "sram_addr_twist (rw) register accessor: SRAM Address Twist Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_addr_twist::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_addr_twist::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_addr_twist`] module"]
pub type SRAM_ADDR_TWIST = crate::Reg<sram_addr_twist::SRAM_ADDR_TWIST_SPEC>;
#[doc = "SRAM Address Twist Register"]
pub mod sram_addr_twist;
#[doc = "work_mode (r) register accessor: Work Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`work_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@work_mode`] module"]
pub type WORK_MODE = crate::Reg<work_mode::WORK_MODE_SPEC>;
#[doc = "Work Mode Register"]
pub mod work_mode;
#[doc = "retite_pc0 (r) register accessor: Retire PC0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retite_pc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retite_pc0`] module"]
pub type RETITE_PC0 = crate::Reg<retite_pc0::RETITE_PC0_SPEC>;
#[doc = "Retire PC0 Register"]
pub mod retite_pc0;
#[doc = "retite_pc1 (r) register accessor: Retire PC1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retite_pc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retite_pc1`] module"]
pub type RETITE_PC1 = crate::Reg<retite_pc1::RETITE_PC1_SPEC>;
#[doc = "Retire PC1 Register"]
pub mod retite_pc1;
#[doc = "irq_mode (rw) register accessor: IRQ Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_mode`] module"]
pub type IRQ_MODE = crate::Reg<irq_mode::IRQ_MODE_SPEC>;
#[doc = "IRQ Mode Register"]
pub mod irq_mode;
#[doc = "riscv_axi_pmu_ctrl (rw) register accessor: RISCV AXI PMU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_axi_pmu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_ctrl`] module"]
pub type RISCV_AXI_PMU_CTRL = crate::Reg<riscv_axi_pmu_ctrl::RISCV_AXI_PMU_CTRL_SPEC>;
#[doc = "RISCV AXI PMU Control Register"]
pub mod riscv_axi_pmu_ctrl;
#[doc = "riscv_axi_pmu_prd (rw) register accessor: RISCV AXI PMU Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_prd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_axi_pmu_prd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_prd`] module"]
pub type RISCV_AXI_PMU_PRD = crate::Reg<riscv_axi_pmu_prd::RISCV_AXI_PMU_PRD_SPEC>;
#[doc = "RISCV AXI PMU Period Register"]
pub mod riscv_axi_pmu_prd;
#[doc = "riscv_axi_pmu_lat_rd (r) register accessor: RISCV AXI PMU Read Latency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_lat_rd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_lat_rd`] module"]
pub type RISCV_AXI_PMU_LAT_RD = crate::Reg<riscv_axi_pmu_lat_rd::RISCV_AXI_PMU_LAT_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Latency Register"]
pub mod riscv_axi_pmu_lat_rd;
#[doc = "riscv_axi_pmu_lat_wr (r) register accessor: RISCV AXI PMU Write Latency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_lat_wr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_lat_wr`] module"]
pub type RISCV_AXI_PMU_LAT_WR = crate::Reg<riscv_axi_pmu_lat_wr::RISCV_AXI_PMU_LAT_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Latency Register"]
pub mod riscv_axi_pmu_lat_wr;
#[doc = "riscv_axi_pmu_req_rd (r) register accessor: RISCV AXI PMU Read Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_req_rd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_req_rd`] module"]
pub type RISCV_AXI_PMU_REQ_RD = crate::Reg<riscv_axi_pmu_req_rd::RISCV_AXI_PMU_REQ_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Request Register"]
pub mod riscv_axi_pmu_req_rd;
#[doc = "riscv_axi_pmu_req_wr (r) register accessor: RISCV AXI PMU Write Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_req_wr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_req_wr`] module"]
pub type RISCV_AXI_PMU_REQ_WR = crate::Reg<riscv_axi_pmu_req_wr::RISCV_AXI_PMU_REQ_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Request Register"]
pub mod riscv_axi_pmu_req_wr;
#[doc = "riscv_axi_pmu_bw_rd (r) register accessor: RISCV AXI PMU Read Bandwidth Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_bw_rd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_bw_rd`] module"]
pub type RISCV_AXI_PMU_BW_RD = crate::Reg<riscv_axi_pmu_bw_rd::RISCV_AXI_PMU_BW_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Bandwidth Register"]
pub mod riscv_axi_pmu_bw_rd;
#[doc = "riscv_axi_pmu_bw_wr (r) register accessor: RISCV AXI PMU Write Bandwidth Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_bw_wr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_axi_pmu_bw_wr`] module"]
pub type RISCV_AXI_PMU_BW_WR = crate::Reg<riscv_axi_pmu_bw_wr::RISCV_AXI_PMU_BW_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Bandwidth Register"]
pub mod riscv_axi_pmu_bw_wr;
