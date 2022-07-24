#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - RISCV Start Address0 Register"]
    pub riscv_sta_add0: crate::Reg<riscv_sta_add0::RISCV_STA_ADD0_SPEC>,
    #[doc = "0x08 - RISCV Start Address1 Register"]
    pub riscv_sta_add1: crate::Reg<riscv_sta_add1::RISCV_STA_ADD1_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - RF1P Configuration Register"]
    pub rf1p_cfg: crate::Reg<rf1p_cfg::RF1P_CFG_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x1c - ROM Configuration Register"]
    pub rom_cfg: crate::Reg<rom_cfg::ROM_CFG_SPEC>,
    #[doc = "0x20 - Wakeup Enable Register"]
    pub wakeup_en: crate::Reg<wakeup_en::WAKEUP_EN_SPEC>,
    #[doc = "0x24..0x38 - Wakeup Mask Register"]
    pub wakeup_mask: [crate::Reg<wakeup_mask::WAKEUP_MASK_SPEC>; 5],
    _reserved6: [u8; 0x08],
    #[doc = "0x40 - Timestamp Test Mode Select Register"]
    pub ts_tmode_sel: crate::Reg<ts_tmode_sel::TS_TMODE_SEL_SPEC>,
    #[doc = "0x44 - SRAM Address Twist Register"]
    pub sram_addr_twist: crate::Reg<sram_addr_twist::SRAM_ADDR_TWIST_SPEC>,
    #[doc = "0x48 - Work Mode Register"]
    pub work_mode: crate::Reg<work_mode::WORK_MODE_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x50 - Retire PC0 Register"]
    pub retite_pc0: crate::Reg<retite_pc0::RETITE_PC0_SPEC>,
    #[doc = "0x54 - Retire PC1 Register"]
    pub retite_pc1: crate::Reg<retite_pc1::RETITE_PC1_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x60..0x74 - IRQ Mode Register"]
    pub irq_mode: [crate::Reg<irq_mode::IRQ_MODE_SPEC>; 5],
    _reserved12: [u8; 0x90],
    #[doc = "0x104 - RISCV AXI PMU Control Register"]
    pub riscv_axi_pmu_ctrl: crate::Reg<riscv_axi_pmu_ctrl::RISCV_AXI_PMU_CTRL_SPEC>,
    #[doc = "0x108 - RISCV AXI PMU Period Register"]
    pub riscv_axi_pmu_prd: crate::Reg<riscv_axi_pmu_prd::RISCV_AXI_PMU_PRD_SPEC>,
    #[doc = "0x10c - RISCV AXI PMU Read Latency Register"]
    pub riscv_axi_pmu_lat_rd: crate::Reg<riscv_axi_pmu_lat_rd::RISCV_AXI_PMU_LAT_RD_SPEC>,
    #[doc = "0x110 - RISCV AXI PMU Write Latency Register"]
    pub riscv_axi_pmu_lat_wr: crate::Reg<riscv_axi_pmu_lat_wr::RISCV_AXI_PMU_LAT_WR_SPEC>,
    #[doc = "0x114 - RISCV AXI PMU Read Request Register"]
    pub riscv_axi_pmu_req_rd: crate::Reg<riscv_axi_pmu_req_rd::RISCV_AXI_PMU_REQ_RD_SPEC>,
    #[doc = "0x118 - RISCV AXI PMU Write Request Register"]
    pub riscv_axi_pmu_req_wr: crate::Reg<riscv_axi_pmu_req_wr::RISCV_AXI_PMU_REQ_WR_SPEC>,
    #[doc = "0x11c - RISCV AXI PMU Read Bandwidth Register"]
    pub riscv_axi_pmu_bw_rd: crate::Reg<riscv_axi_pmu_bw_rd::RISCV_AXI_PMU_BW_RD_SPEC>,
    #[doc = "0x120 - RISCV AXI PMU Write Bandwidth Register"]
    pub riscv_axi_pmu_bw_wr: crate::Reg<riscv_axi_pmu_bw_wr::RISCV_AXI_PMU_BW_WR_SPEC>,
}
#[doc = "riscv_sta_add0 register accessor: an alias for `Reg<RISCV_STA_ADD0_SPEC>`"]
pub type RISCV_STA_ADD0 = crate::Reg<riscv_sta_add0::RISCV_STA_ADD0_SPEC>;
#[doc = "RISCV Start Address0 Register"]
pub mod riscv_sta_add0;
#[doc = "riscv_sta_add1 register accessor: an alias for `Reg<RISCV_STA_ADD1_SPEC>`"]
pub type RISCV_STA_ADD1 = crate::Reg<riscv_sta_add1::RISCV_STA_ADD1_SPEC>;
#[doc = "RISCV Start Address1 Register"]
pub mod riscv_sta_add1;
#[doc = "rf1p_cfg register accessor: an alias for `Reg<RF1P_CFG_SPEC>`"]
pub type RF1P_CFG = crate::Reg<rf1p_cfg::RF1P_CFG_SPEC>;
#[doc = "RF1P Configuration Register"]
pub mod rf1p_cfg;
#[doc = "rom_cfg register accessor: an alias for `Reg<ROM_CFG_SPEC>`"]
pub type ROM_CFG = crate::Reg<rom_cfg::ROM_CFG_SPEC>;
#[doc = "ROM Configuration Register"]
pub mod rom_cfg;
#[doc = "wakeup_en register accessor: an alias for `Reg<WAKEUP_EN_SPEC>`"]
pub type WAKEUP_EN = crate::Reg<wakeup_en::WAKEUP_EN_SPEC>;
#[doc = "Wakeup Enable Register"]
pub mod wakeup_en;
#[doc = "wakeup_mask register accessor: an alias for `Reg<WAKEUP_MASK_SPEC>`"]
pub type WAKEUP_MASK = crate::Reg<wakeup_mask::WAKEUP_MASK_SPEC>;
#[doc = "Wakeup Mask Register"]
pub mod wakeup_mask;
#[doc = "ts_tmode_sel register accessor: an alias for `Reg<TS_TMODE_SEL_SPEC>`"]
pub type TS_TMODE_SEL = crate::Reg<ts_tmode_sel::TS_TMODE_SEL_SPEC>;
#[doc = "Timestamp Test Mode Select Register"]
pub mod ts_tmode_sel;
#[doc = "sram_addr_twist register accessor: an alias for `Reg<SRAM_ADDR_TWIST_SPEC>`"]
pub type SRAM_ADDR_TWIST = crate::Reg<sram_addr_twist::SRAM_ADDR_TWIST_SPEC>;
#[doc = "SRAM Address Twist Register"]
pub mod sram_addr_twist;
#[doc = "work_mode register accessor: an alias for `Reg<WORK_MODE_SPEC>`"]
pub type WORK_MODE = crate::Reg<work_mode::WORK_MODE_SPEC>;
#[doc = "Work Mode Register"]
pub mod work_mode;
#[doc = "retite_pc0 register accessor: an alias for `Reg<RETITE_PC0_SPEC>`"]
pub type RETITE_PC0 = crate::Reg<retite_pc0::RETITE_PC0_SPEC>;
#[doc = "Retire PC0 Register"]
pub mod retite_pc0;
#[doc = "retite_pc1 register accessor: an alias for `Reg<RETITE_PC1_SPEC>`"]
pub type RETITE_PC1 = crate::Reg<retite_pc1::RETITE_PC1_SPEC>;
#[doc = "Retire PC1 Register"]
pub mod retite_pc1;
#[doc = "irq_mode register accessor: an alias for `Reg<IRQ_MODE_SPEC>`"]
pub type IRQ_MODE = crate::Reg<irq_mode::IRQ_MODE_SPEC>;
#[doc = "IRQ Mode Register"]
pub mod irq_mode;
#[doc = "riscv_axi_pmu_ctrl register accessor: an alias for `Reg<RISCV_AXI_PMU_CTRL_SPEC>`"]
pub type RISCV_AXI_PMU_CTRL = crate::Reg<riscv_axi_pmu_ctrl::RISCV_AXI_PMU_CTRL_SPEC>;
#[doc = "RISCV AXI PMU Control Register"]
pub mod riscv_axi_pmu_ctrl;
#[doc = "riscv_axi_pmu_prd register accessor: an alias for `Reg<RISCV_AXI_PMU_PRD_SPEC>`"]
pub type RISCV_AXI_PMU_PRD = crate::Reg<riscv_axi_pmu_prd::RISCV_AXI_PMU_PRD_SPEC>;
#[doc = "RISCV AXI PMU Period Register"]
pub mod riscv_axi_pmu_prd;
#[doc = "riscv_axi_pmu_lat_rd register accessor: an alias for `Reg<RISCV_AXI_PMU_LAT_RD_SPEC>`"]
pub type RISCV_AXI_PMU_LAT_RD = crate::Reg<riscv_axi_pmu_lat_rd::RISCV_AXI_PMU_LAT_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Latency Register"]
pub mod riscv_axi_pmu_lat_rd;
#[doc = "riscv_axi_pmu_lat_wr register accessor: an alias for `Reg<RISCV_AXI_PMU_LAT_WR_SPEC>`"]
pub type RISCV_AXI_PMU_LAT_WR = crate::Reg<riscv_axi_pmu_lat_wr::RISCV_AXI_PMU_LAT_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Latency Register"]
pub mod riscv_axi_pmu_lat_wr;
#[doc = "riscv_axi_pmu_req_rd register accessor: an alias for `Reg<RISCV_AXI_PMU_REQ_RD_SPEC>`"]
pub type RISCV_AXI_PMU_REQ_RD = crate::Reg<riscv_axi_pmu_req_rd::RISCV_AXI_PMU_REQ_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Request Register"]
pub mod riscv_axi_pmu_req_rd;
#[doc = "riscv_axi_pmu_req_wr register accessor: an alias for `Reg<RISCV_AXI_PMU_REQ_WR_SPEC>`"]
pub type RISCV_AXI_PMU_REQ_WR = crate::Reg<riscv_axi_pmu_req_wr::RISCV_AXI_PMU_REQ_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Request Register"]
pub mod riscv_axi_pmu_req_wr;
#[doc = "riscv_axi_pmu_bw_rd register accessor: an alias for `Reg<RISCV_AXI_PMU_BW_RD_SPEC>`"]
pub type RISCV_AXI_PMU_BW_RD = crate::Reg<riscv_axi_pmu_bw_rd::RISCV_AXI_PMU_BW_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Bandwidth Register"]
pub mod riscv_axi_pmu_bw_rd;
#[doc = "riscv_axi_pmu_bw_wr register accessor: an alias for `Reg<RISCV_AXI_PMU_BW_WR_SPEC>`"]
pub type RISCV_AXI_PMU_BW_WR = crate::Reg<riscv_axi_pmu_bw_wr::RISCV_AXI_PMU_BW_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Bandwidth Register"]
pub mod riscv_axi_pmu_bw_wr;
