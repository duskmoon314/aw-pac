#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - RISCV Start Address0 Register"]
    pub riscv_sta_add0_reg: crate::Reg<riscv_sta_add0_reg::RISCV_STA_ADD0_REG_SPEC>,
    #[doc = "0x08 - RISCV Start Address1 Register"]
    pub riscv_sta_add1_reg: crate::Reg<riscv_sta_add1_reg::RISCV_STA_ADD1_REG_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - RF1P Configuration Register"]
    pub rf1p_cfg_reg: crate::Reg<rf1p_cfg_reg::RF1P_CFG_REG_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x1c - ROM Configuration Register"]
    pub rom_cfg_reg: crate::Reg<rom_cfg_reg::ROM_CFG_REG_SPEC>,
    #[doc = "0x20 - Wakeup Enable Register"]
    pub wakeup_en_reg: crate::Reg<wakeup_en_reg::WAKEUP_EN_REG_SPEC>,
    #[doc = "0x24 - Wakeup Mask0 Register"]
    pub wakeup_mask0_reg: crate::Reg<wakeup_mask0_reg::WAKEUP_MASK0_REG_SPEC>,
    #[doc = "0x28 - Wakeup Mask1 Register"]
    pub wakeup_mask1_reg: crate::Reg<wakeup_mask1_reg::WAKEUP_MASK1_REG_SPEC>,
    #[doc = "0x2c - Wakeup Mask2 Register"]
    pub wakeup_mask2_reg: crate::Reg<wakeup_mask2_reg::WAKEUP_MASK2_REG_SPEC>,
    #[doc = "0x30 - Wakeup Mask3 Register"]
    pub wakeup_mask3_reg: crate::Reg<wakeup_mask3_reg::WAKEUP_MASK3_REG_SPEC>,
    #[doc = "0x34 - Wakeup Mask4 Register"]
    pub wakeup_mask4_reg: crate::Reg<wakeup_mask4_reg::WAKEUP_MASK4_REG_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - Timestamp Test Mode Select Register"]
    pub ts_tmode_sel_reg: crate::Reg<ts_tmode_sel_reg::TS_TMODE_SEL_REG_SPEC>,
    #[doc = "0x44 - SRAM Address Twist Register"]
    pub sram_addr_twist_reg: crate::Reg<sram_addr_twist_reg::SRAM_ADDR_TWIST_REG_SPEC>,
    #[doc = "0x48 - Work Mode Register"]
    pub work_mode_reg: crate::Reg<work_mode_reg::WORK_MODE_REG_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x50 - Retire PC0 Register"]
    pub retite_pc0_reg: crate::Reg<retite_pc0_reg::RETITE_PC0_REG_SPEC>,
    #[doc = "0x54 - Retire PC1 Register"]
    pub retite_pc1_reg: crate::Reg<retite_pc1_reg::RETITE_PC1_REG_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x60 - IRQ Mode0 Register"]
    pub irq_mode0_reg: crate::Reg<irq_mode0_reg::IRQ_MODE0_REG_SPEC>,
    #[doc = "0x64 - IRQ Mode1 Register"]
    pub irq_mode1_reg: crate::Reg<irq_mode1_reg::IRQ_MODE1_REG_SPEC>,
    #[doc = "0x68 - IRQ Mode2 Register"]
    pub irq_mode2_reg: crate::Reg<irq_mode2_reg::IRQ_MODE2_REG_SPEC>,
    #[doc = "0x6c - IRQ Mode3 Register"]
    pub irq_mode3_reg: crate::Reg<irq_mode3_reg::IRQ_MODE3_REG_SPEC>,
    #[doc = "0x70 - IRQ Mode4 Register"]
    pub irq_mode4_reg: crate::Reg<irq_mode4_reg::IRQ_MODE4_REG_SPEC>,
    _reserved20: [u8; 0x90],
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
#[doc = "RISCV_STA_ADD0_REG register accessor: an alias for `Reg<RISCV_STA_ADD0_REG_SPEC>`"]
pub type RISCV_STA_ADD0_REG = crate::Reg<riscv_sta_add0_reg::RISCV_STA_ADD0_REG_SPEC>;
#[doc = "RISCV Start Address0 Register"]
pub mod riscv_sta_add0_reg;
#[doc = "RISCV_STA_ADD1_REG register accessor: an alias for `Reg<RISCV_STA_ADD1_REG_SPEC>`"]
pub type RISCV_STA_ADD1_REG = crate::Reg<riscv_sta_add1_reg::RISCV_STA_ADD1_REG_SPEC>;
#[doc = "RISCV Start Address1 Register"]
pub mod riscv_sta_add1_reg;
#[doc = "RF1P_CFG_REG register accessor: an alias for `Reg<RF1P_CFG_REG_SPEC>`"]
pub type RF1P_CFG_REG = crate::Reg<rf1p_cfg_reg::RF1P_CFG_REG_SPEC>;
#[doc = "RF1P Configuration Register"]
pub mod rf1p_cfg_reg;
#[doc = "ROM_CFG_REG register accessor: an alias for `Reg<ROM_CFG_REG_SPEC>`"]
pub type ROM_CFG_REG = crate::Reg<rom_cfg_reg::ROM_CFG_REG_SPEC>;
#[doc = "ROM Configuration Register"]
pub mod rom_cfg_reg;
#[doc = "WAKEUP_EN_REG register accessor: an alias for `Reg<WAKEUP_EN_REG_SPEC>`"]
pub type WAKEUP_EN_REG = crate::Reg<wakeup_en_reg::WAKEUP_EN_REG_SPEC>;
#[doc = "Wakeup Enable Register"]
pub mod wakeup_en_reg;
#[doc = "WAKEUP_MASK0_REG register accessor: an alias for `Reg<WAKEUP_MASK0_REG_SPEC>`"]
pub type WAKEUP_MASK0_REG = crate::Reg<wakeup_mask0_reg::WAKEUP_MASK0_REG_SPEC>;
#[doc = "Wakeup Mask0 Register"]
pub mod wakeup_mask0_reg;
#[doc = "WAKEUP_MASK1_REG register accessor: an alias for `Reg<WAKEUP_MASK1_REG_SPEC>`"]
pub type WAKEUP_MASK1_REG = crate::Reg<wakeup_mask1_reg::WAKEUP_MASK1_REG_SPEC>;
#[doc = "Wakeup Mask1 Register"]
pub mod wakeup_mask1_reg;
#[doc = "WAKEUP_MASK2_REG register accessor: an alias for `Reg<WAKEUP_MASK2_REG_SPEC>`"]
pub type WAKEUP_MASK2_REG = crate::Reg<wakeup_mask2_reg::WAKEUP_MASK2_REG_SPEC>;
#[doc = "Wakeup Mask2 Register"]
pub mod wakeup_mask2_reg;
#[doc = "WAKEUP_MASK3_REG register accessor: an alias for `Reg<WAKEUP_MASK3_REG_SPEC>`"]
pub type WAKEUP_MASK3_REG = crate::Reg<wakeup_mask3_reg::WAKEUP_MASK3_REG_SPEC>;
#[doc = "Wakeup Mask3 Register"]
pub mod wakeup_mask3_reg;
#[doc = "WAKEUP_MASK4_REG register accessor: an alias for `Reg<WAKEUP_MASK4_REG_SPEC>`"]
pub type WAKEUP_MASK4_REG = crate::Reg<wakeup_mask4_reg::WAKEUP_MASK4_REG_SPEC>;
#[doc = "Wakeup Mask4 Register"]
pub mod wakeup_mask4_reg;
#[doc = "TS_TMODE_SEL_REG register accessor: an alias for `Reg<TS_TMODE_SEL_REG_SPEC>`"]
pub type TS_TMODE_SEL_REG = crate::Reg<ts_tmode_sel_reg::TS_TMODE_SEL_REG_SPEC>;
#[doc = "Timestamp Test Mode Select Register"]
pub mod ts_tmode_sel_reg;
#[doc = "SRAM_ADDR_TWIST_REG register accessor: an alias for `Reg<SRAM_ADDR_TWIST_REG_SPEC>`"]
pub type SRAM_ADDR_TWIST_REG = crate::Reg<sram_addr_twist_reg::SRAM_ADDR_TWIST_REG_SPEC>;
#[doc = "SRAM Address Twist Register"]
pub mod sram_addr_twist_reg;
#[doc = "WORK_MODE_REG register accessor: an alias for `Reg<WORK_MODE_REG_SPEC>`"]
pub type WORK_MODE_REG = crate::Reg<work_mode_reg::WORK_MODE_REG_SPEC>;
#[doc = "Work Mode Register"]
pub mod work_mode_reg;
#[doc = "RETITE_PC0_REG register accessor: an alias for `Reg<RETITE_PC0_REG_SPEC>`"]
pub type RETITE_PC0_REG = crate::Reg<retite_pc0_reg::RETITE_PC0_REG_SPEC>;
#[doc = "Retire PC0 Register"]
pub mod retite_pc0_reg;
#[doc = "RETITE_PC1_REG register accessor: an alias for `Reg<RETITE_PC1_REG_SPEC>`"]
pub type RETITE_PC1_REG = crate::Reg<retite_pc1_reg::RETITE_PC1_REG_SPEC>;
#[doc = "Retire PC1 Register"]
pub mod retite_pc1_reg;
#[doc = "IRQ_MODE0_REG register accessor: an alias for `Reg<IRQ_MODE0_REG_SPEC>`"]
pub type IRQ_MODE0_REG = crate::Reg<irq_mode0_reg::IRQ_MODE0_REG_SPEC>;
#[doc = "IRQ Mode0 Register"]
pub mod irq_mode0_reg;
#[doc = "IRQ_MODE1_REG register accessor: an alias for `Reg<IRQ_MODE1_REG_SPEC>`"]
pub type IRQ_MODE1_REG = crate::Reg<irq_mode1_reg::IRQ_MODE1_REG_SPEC>;
#[doc = "IRQ Mode1 Register"]
pub mod irq_mode1_reg;
#[doc = "IRQ_MODE2_REG register accessor: an alias for `Reg<IRQ_MODE2_REG_SPEC>`"]
pub type IRQ_MODE2_REG = crate::Reg<irq_mode2_reg::IRQ_MODE2_REG_SPEC>;
#[doc = "IRQ Mode2 Register"]
pub mod irq_mode2_reg;
#[doc = "IRQ_MODE3_REG register accessor: an alias for `Reg<IRQ_MODE3_REG_SPEC>`"]
pub type IRQ_MODE3_REG = crate::Reg<irq_mode3_reg::IRQ_MODE3_REG_SPEC>;
#[doc = "IRQ Mode3 Register"]
pub mod irq_mode3_reg;
#[doc = "IRQ_MODE4_REG register accessor: an alias for `Reg<IRQ_MODE4_REG_SPEC>`"]
pub type IRQ_MODE4_REG = crate::Reg<irq_mode4_reg::IRQ_MODE4_REG_SPEC>;
#[doc = "IRQ Mode4 Register"]
pub mod irq_mode4_reg;
#[doc = "RISCV_AXI_PMU_CTRL register accessor: an alias for `Reg<RISCV_AXI_PMU_CTRL_SPEC>`"]
pub type RISCV_AXI_PMU_CTRL = crate::Reg<riscv_axi_pmu_ctrl::RISCV_AXI_PMU_CTRL_SPEC>;
#[doc = "RISCV AXI PMU Control Register"]
pub mod riscv_axi_pmu_ctrl;
#[doc = "RISCV_AXI_PMU_PRD register accessor: an alias for `Reg<RISCV_AXI_PMU_PRD_SPEC>`"]
pub type RISCV_AXI_PMU_PRD = crate::Reg<riscv_axi_pmu_prd::RISCV_AXI_PMU_PRD_SPEC>;
#[doc = "RISCV AXI PMU Period Register"]
pub mod riscv_axi_pmu_prd;
#[doc = "RISCV_AXI_PMU_LAT_RD register accessor: an alias for `Reg<RISCV_AXI_PMU_LAT_RD_SPEC>`"]
pub type RISCV_AXI_PMU_LAT_RD = crate::Reg<riscv_axi_pmu_lat_rd::RISCV_AXI_PMU_LAT_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Latency Register"]
pub mod riscv_axi_pmu_lat_rd;
#[doc = "RISCV_AXI_PMU_LAT_WR register accessor: an alias for `Reg<RISCV_AXI_PMU_LAT_WR_SPEC>`"]
pub type RISCV_AXI_PMU_LAT_WR = crate::Reg<riscv_axi_pmu_lat_wr::RISCV_AXI_PMU_LAT_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Latency Register"]
pub mod riscv_axi_pmu_lat_wr;
#[doc = "RISCV_AXI_PMU_REQ_RD register accessor: an alias for `Reg<RISCV_AXI_PMU_REQ_RD_SPEC>`"]
pub type RISCV_AXI_PMU_REQ_RD = crate::Reg<riscv_axi_pmu_req_rd::RISCV_AXI_PMU_REQ_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Request Register"]
pub mod riscv_axi_pmu_req_rd;
#[doc = "RISCV_AXI_PMU_REQ_WR register accessor: an alias for `Reg<RISCV_AXI_PMU_REQ_WR_SPEC>`"]
pub type RISCV_AXI_PMU_REQ_WR = crate::Reg<riscv_axi_pmu_req_wr::RISCV_AXI_PMU_REQ_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Request Register"]
pub mod riscv_axi_pmu_req_wr;
#[doc = "RISCV_AXI_PMU_BW_RD register accessor: an alias for `Reg<RISCV_AXI_PMU_BW_RD_SPEC>`"]
pub type RISCV_AXI_PMU_BW_RD = crate::Reg<riscv_axi_pmu_bw_rd::RISCV_AXI_PMU_BW_RD_SPEC>;
#[doc = "RISCV AXI PMU Read Bandwidth Register"]
pub mod riscv_axi_pmu_bw_rd;
#[doc = "RISCV_AXI_PMU_BW_WR register accessor: an alias for `Reg<RISCV_AXI_PMU_BW_WR_SPEC>`"]
pub type RISCV_AXI_PMU_BW_WR = crate::Reg<riscv_axi_pmu_bw_wr::RISCV_AXI_PMU_BW_WR_SPEC>;
#[doc = "RISCV AXI PMU Write Bandwidth Register"]
pub mod riscv_axi_pmu_bw_wr;
