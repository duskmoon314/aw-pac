#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - IOMMU Reset Register"]
    pub iommu_reset: IOMMU_RESET,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20 - IOMMU Enable Register"]
    pub iommu_enable: IOMMU_ENABLE,
    _reserved2: [u8; 0x0c],
    #[doc = "0x30 - IOMMU Bypass Register"]
    pub iommu_bypass: IOMMU_BYPASS,
    _reserved3: [u8; 0x0c],
    #[doc = "0x40 - IOMMU Auto Gating Register"]
    pub iommu_auto_gating: IOMMU_AUTO_GATING,
    #[doc = "0x44 - IOMMU Write Buffer Control Register"]
    pub iommu_wbuf_ctrl: IOMMU_WBUF_CTRL,
    #[doc = "0x48 - IOMMU Out of Order Control Register"]
    pub iommu_ooo_ctrl: IOMMU_OOO_CTRL,
    #[doc = "0x4c - IOMMU 4KB Boundary Protect Control Register"]
    pub iommu_4kb_bdy_prt_ctrl: IOMMU_4KB_BDY_PRT_CTRL,
    #[doc = "0x50 - IOMMU Translation Table Base Register"]
    pub iommu_ttb: IOMMU_TTB,
    _reserved8: [u8; 0x0c],
    #[doc = "0x60 - IOMMU TLB Enable Register"]
    pub iommu_tlb_enable: IOMMU_TLB_ENABLE,
    _reserved9: [u8; 0x0c],
    #[doc = "0x70 - IOMMU TLB Prefetch Register"]
    pub iommu_tlb_prefetch: IOMMU_TLB_PREFETCH,
    _reserved10: [u8; 0x0c],
    #[doc = "0x80 - IOMMU TLB Flush Enable Register"]
    pub iommu_tlb_flush_enable: IOMMU_TLB_FLUSH_ENABLE,
    #[doc = "0x84 - IOMMU TLB Invalidation Mode Select Register"]
    pub iommu_tlb_ivld_mode_sel: IOMMU_TLB_IVLD_MODE_SEL,
    #[doc = "0x88 - IOMMU TLB Invalidation Start Address Register"]
    pub iommu_tlb_ivld_sta_addr: IOMMU_TLB_IVLD_STA_ADDR,
    #[doc = "0x8c - IOMMU TLB Invalidation End Address Register"]
    pub iommu_tlb_ivld_end_addr: IOMMU_TLB_IVLD_END_ADDR,
    #[doc = "0x90 - IOMMU TLB Invalidation Address Register"]
    pub iommu_tlb_ivld_addr: IOMMU_TLB_IVLD_ADDR,
    #[doc = "0x94 - IOMMU TLB Invalidation Address Mask Register"]
    pub iommu_tlb_ivld_addr_mask: IOMMU_TLB_IVLD_ADDR_MASK,
    #[doc = "0x98 - IOMMU TLB Invalidation Enable Register"]
    pub iommu_tlb_ivld_enable: IOMMU_TLB_IVLD_ENABLE,
    #[doc = "0x9c - IOMMU PC Invalidation Mode Select Register"]
    pub iommu_pc_ivld_mode_sel: IOMMU_PC_IVLD_MODE_SEL,
    #[doc = "0xa0 - IOMMU PC Invalidation Address Register"]
    pub iommu_pc_ivld_addr: IOMMU_PC_IVLD_ADDR,
    #[doc = "0xa4 - IOMMU PC Invalidation Start Address Register"]
    pub iommu_pc_ivld_sta_addr: IOMMU_PC_IVLD_STA_ADDR,
    #[doc = "0xa8 - IOMMU PC Invalidation Enable Register"]
    pub iommu_pc_ivld_enable: IOMMU_PC_IVLD_ENABLE,
    #[doc = "0xac - IOMMU PC Invalidation End Address Register"]
    pub iommu_pc_ivld_end_addr: IOMMU_PC_IVLD_END_ADDR,
    #[doc = "0xb0..0xd0 - IOMMU Domain Authority Control \\[i\\] Register\n\nSoftware can set 15 different permission control types in IOMMU_DM_AUT_CTRL0-7. A default access control type is DOMAIN0. The read/write operation of DOMAIN1-15 is unlimited by default.\n\nSoftware needs to set the index of the permission control domain corresponding to the page table item in the bit\\[7:4\\] of the Level2 page table, the default value is 0 (use domain 0), that is, the read/write operation is not controlled.\n\nSetting REG_ARD_OVWT can mask the Domain control defined by IOMMU_DM_AUT_CTRL0-7. All Level2 page table type are covered by the type of REG_ARD_OVWT. The read/write operation is permitted by default."]
    pub iommu_dm_aut_ctrl: [IOMMU_DM_AUT_CTRL; 8],
    #[doc = "0xd0 - IOMMU Domain Authority Overwrite Register"]
    pub iommu_dm_aut_ovwt: IOMMU_DM_AUT_OVWT,
    _reserved24: [u8; 0x2c],
    #[doc = "0x100 - IOMMU Interrupt Enable Register"]
    pub iommu_int_enable: IOMMU_INT_ENABLE,
    #[doc = "0x104 - IOMMU Interrupt Clear Register"]
    pub iommu_int_clr: IOMMU_INT_CLR,
    #[doc = "0x108 - IOMMU Interrupt Status Register"]
    pub iommu_int_sta: IOMMU_INT_STA,
    _reserved27: [u8; 0x04],
    #[doc = "0x110..0x12c - IOMMU Interrupt Error Address \\[i\\]"]
    pub iommu_int_err_addr_tlb: [IOMMU_INT_ERR_ADDR_TLB; 7],
    _reserved28: [u8; 0x04],
    #[doc = "0x130..0x138 - IOMMU Interrupt Error Address L\\[i\\]"]
    pub iommu_int_err_addr_l: [IOMMU_INT_ERR_ADDR_L; 2],
    _reserved29: [u8; 0x18],
    #[doc = "0x150..0x16c - IOMMU Interrupt Error Data \\[i\\] Register"]
    pub iommu_int_err_data_tlb: [IOMMU_INT_ERR_DATA_TLB; 7],
    _reserved30: [u8; 0x04],
    #[doc = "0x170..0x178 - IOMMU Interrupt Error Data L\\[i\\] Register"]
    pub iommu_int_err_data_l: [IOMMU_INT_ERR_DATA_L; 2],
    _reserved31: [u8; 0x08],
    #[doc = "0x180..0x188 - IOMMU L\\[i\\] Page Table Interrupt Register"]
    pub iommu_lpg_int: [IOMMU_LPG_INT; 2],
    _reserved32: [u8; 0x08],
    #[doc = "0x190 - IOMMU Virtual Address Register"]
    pub iommu_va: IOMMU_VA,
    #[doc = "0x194 - IOMMU Virtual Address Data Register"]
    pub iommu_va_data: IOMMU_VA_DATA,
    #[doc = "0x198 - IOMMU Virtual Address Configuration Register"]
    pub iommu_va_config: IOMMU_VA_CONFIG,
    _reserved35: [u8; 0x64],
    #[doc = "0x200 - IOMMU PMU Enable Register"]
    pub iommu_pmu_enable: IOMMU_PMU_ENABLE,
    _reserved36: [u8; 0x0c],
    #[doc = "0x210 - IOMMU PMU Clear Register"]
    pub iommu_pmu_clr: IOMMU_PMU_CLR,
    _reserved37: [u8; 0x1c],
    #[doc = "0x230 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low0: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x234 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high0: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x238 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low0: IOMMU_PMU_HIT_LOW,
    #[doc = "0x23c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high0: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x240 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low1: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x244 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high1: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x248 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low1: IOMMU_PMU_HIT_LOW,
    #[doc = "0x24c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high1: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x250 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low2: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x254 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high2: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x258 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low2: IOMMU_PMU_HIT_LOW,
    #[doc = "0x25c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high2: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x260 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low3: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x264 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high3: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x268 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low3: IOMMU_PMU_HIT_LOW,
    #[doc = "0x26c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high3: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x270 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low4: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x274 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high4: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x278 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low4: IOMMU_PMU_HIT_LOW,
    #[doc = "0x27c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high4: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x280 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low5: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x284 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high5: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x288 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low5: IOMMU_PMU_HIT_LOW,
    #[doc = "0x28c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high5: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x290 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low6: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x294 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high6: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x298 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low6: IOMMU_PMU_HIT_LOW,
    #[doc = "0x29c - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high6: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x2a0 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low7: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x2a4 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high7: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x2a8 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low7: IOMMU_PMU_HIT_LOW,
    #[doc = "0x2ac - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high7: IOMMU_PMU_HIT_HIGH,
    #[doc = "0x2b0 - IOMMU PMU Access Low \\[i\\] Register"]
    pub iommu_pmu_access_low8: IOMMU_PMU_ACCESS_LOW,
    #[doc = "0x2b4 - IOMMU PMU Access High \\[i\\] Register"]
    pub iommu_pmu_access_high8: IOMMU_PMU_ACCESS_HIGH,
    #[doc = "0x2b8 - IOMMU PMU Hit Low \\[i\\] Register"]
    pub iommu_pmu_hit_low8: IOMMU_PMU_HIT_LOW,
    #[doc = "0x2bc - IOMMU PMU Hit High \\[i\\] Register"]
    pub iommu_pmu_hit_high8: IOMMU_PMU_HIT_HIGH,
    _reserved73: [u8; 0x40],
    #[doc = "0x300 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low0: IOMMU_PMU_TL_LOW,
    #[doc = "0x304 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high0: IOMMU_PMU_TL_HIGH,
    #[doc = "0x308 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml0: IOMMU_PMU_ML,
    _reserved76: [u8; 0x04],
    #[doc = "0x310 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low1: IOMMU_PMU_TL_LOW,
    #[doc = "0x314 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high1: IOMMU_PMU_TL_HIGH,
    #[doc = "0x318 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml1: IOMMU_PMU_ML,
    _reserved79: [u8; 0x04],
    #[doc = "0x320 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low2: IOMMU_PMU_TL_LOW,
    #[doc = "0x324 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high2: IOMMU_PMU_TL_HIGH,
    #[doc = "0x328 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml2: IOMMU_PMU_ML,
    _reserved82: [u8; 0x04],
    #[doc = "0x330 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low3: IOMMU_PMU_TL_LOW,
    #[doc = "0x334 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high3: IOMMU_PMU_TL_HIGH,
    #[doc = "0x338 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml3: IOMMU_PMU_ML,
    _reserved85: [u8; 0x04],
    #[doc = "0x340 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low4: IOMMU_PMU_TL_LOW,
    #[doc = "0x344 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high4: IOMMU_PMU_TL_HIGH,
    #[doc = "0x348 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml4: IOMMU_PMU_ML,
    _reserved88: [u8; 0x04],
    #[doc = "0x350 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low5: IOMMU_PMU_TL_LOW,
    #[doc = "0x354 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high5: IOMMU_PMU_TL_HIGH,
    #[doc = "0x358 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml5: IOMMU_PMU_ML,
    _reserved91: [u8; 0x04],
    #[doc = "0x360 - IOMMU Total Latency Low \\[i\\] Register"]
    pub iommu_pmu_tl_low6: IOMMU_PMU_TL_LOW,
    #[doc = "0x364 - IOMMU Total Latency High \\[i\\] Register"]
    pub iommu_pmu_tl_high6: IOMMU_PMU_TL_HIGH,
    #[doc = "0x368 - IOMMU Max Latency \\[i\\] Register"]
    pub iommu_pmu_ml6: IOMMU_PMU_ML,
}
#[doc = "iommu_reset (rw) register accessor: an alias for `Reg<IOMMU_RESET_SPEC>`"]
pub type IOMMU_RESET = crate::Reg<iommu_reset::IOMMU_RESET_SPEC>;
#[doc = "IOMMU Reset Register"]
pub mod iommu_reset;
#[doc = "iommu_enable (rw) register accessor: an alias for `Reg<IOMMU_ENABLE_SPEC>`"]
pub type IOMMU_ENABLE = crate::Reg<iommu_enable::IOMMU_ENABLE_SPEC>;
#[doc = "IOMMU Enable Register"]
pub mod iommu_enable;
#[doc = "iommu_bypass (rw) register accessor: an alias for `Reg<IOMMU_BYPASS_SPEC>`"]
pub type IOMMU_BYPASS = crate::Reg<iommu_bypass::IOMMU_BYPASS_SPEC>;
#[doc = "IOMMU Bypass Register"]
pub mod iommu_bypass;
#[doc = "iommu_auto_gating (rw) register accessor: an alias for `Reg<IOMMU_AUTO_GATING_SPEC>`"]
pub type IOMMU_AUTO_GATING = crate::Reg<iommu_auto_gating::IOMMU_AUTO_GATING_SPEC>;
#[doc = "IOMMU Auto Gating Register"]
pub mod iommu_auto_gating;
#[doc = "iommu_wbuf_ctrl (rw) register accessor: an alias for `Reg<IOMMU_WBUF_CTRL_SPEC>`"]
pub type IOMMU_WBUF_CTRL = crate::Reg<iommu_wbuf_ctrl::IOMMU_WBUF_CTRL_SPEC>;
#[doc = "IOMMU Write Buffer Control Register"]
pub mod iommu_wbuf_ctrl;
#[doc = "iommu_ooo_ctrl (rw) register accessor: an alias for `Reg<IOMMU_OOO_CTRL_SPEC>`"]
pub type IOMMU_OOO_CTRL = crate::Reg<iommu_ooo_ctrl::IOMMU_OOO_CTRL_SPEC>;
#[doc = "IOMMU Out of Order Control Register"]
pub mod iommu_ooo_ctrl;
#[doc = "iommu_4kb_bdy_prt_ctrl (rw) register accessor: an alias for `Reg<IOMMU_4KB_BDY_PRT_CTRL_SPEC>`"]
pub type IOMMU_4KB_BDY_PRT_CTRL = crate::Reg<iommu_4kb_bdy_prt_ctrl::IOMMU_4KB_BDY_PRT_CTRL_SPEC>;
#[doc = "IOMMU 4KB Boundary Protect Control Register"]
pub mod iommu_4kb_bdy_prt_ctrl;
#[doc = "iommu_ttb (rw) register accessor: an alias for `Reg<IOMMU_TTB_SPEC>`"]
pub type IOMMU_TTB = crate::Reg<iommu_ttb::IOMMU_TTB_SPEC>;
#[doc = "IOMMU Translation Table Base Register"]
pub mod iommu_ttb;
#[doc = "iommu_tlb_enable (rw) register accessor: an alias for `Reg<IOMMU_TLB_ENABLE_SPEC>`"]
pub type IOMMU_TLB_ENABLE = crate::Reg<iommu_tlb_enable::IOMMU_TLB_ENABLE_SPEC>;
#[doc = "IOMMU TLB Enable Register"]
pub mod iommu_tlb_enable;
#[doc = "iommu_tlb_prefetch (rw) register accessor: an alias for `Reg<IOMMU_TLB_PREFETCH_SPEC>`"]
pub type IOMMU_TLB_PREFETCH = crate::Reg<iommu_tlb_prefetch::IOMMU_TLB_PREFETCH_SPEC>;
#[doc = "IOMMU TLB Prefetch Register"]
pub mod iommu_tlb_prefetch;
#[doc = "iommu_tlb_flush_enable (rw) register accessor: an alias for `Reg<IOMMU_TLB_FLUSH_ENABLE_SPEC>`"]
pub type IOMMU_TLB_FLUSH_ENABLE = crate::Reg<iommu_tlb_flush_enable::IOMMU_TLB_FLUSH_ENABLE_SPEC>;
#[doc = "IOMMU TLB Flush Enable Register"]
pub mod iommu_tlb_flush_enable;
#[doc = "iommu_tlb_ivld_mode_sel (rw) register accessor: an alias for `Reg<IOMMU_TLB_IVLD_MODE_SEL_SPEC>`"]
pub type IOMMU_TLB_IVLD_MODE_SEL =
    crate::Reg<iommu_tlb_ivld_mode_sel::IOMMU_TLB_IVLD_MODE_SEL_SPEC>;
#[doc = "IOMMU TLB Invalidation Mode Select Register"]
pub mod iommu_tlb_ivld_mode_sel;
#[doc = "iommu_tlb_ivld_sta_addr (rw) register accessor: an alias for `Reg<IOMMU_TLB_IVLD_STA_ADDR_SPEC>`"]
pub type IOMMU_TLB_IVLD_STA_ADDR =
    crate::Reg<iommu_tlb_ivld_sta_addr::IOMMU_TLB_IVLD_STA_ADDR_SPEC>;
#[doc = "IOMMU TLB Invalidation Start Address Register"]
pub mod iommu_tlb_ivld_sta_addr;
#[doc = "iommu_tlb_ivld_end_addr (rw) register accessor: an alias for `Reg<IOMMU_TLB_IVLD_END_ADDR_SPEC>`"]
pub type IOMMU_TLB_IVLD_END_ADDR =
    crate::Reg<iommu_tlb_ivld_end_addr::IOMMU_TLB_IVLD_END_ADDR_SPEC>;
#[doc = "IOMMU TLB Invalidation End Address Register"]
pub mod iommu_tlb_ivld_end_addr;
#[doc = "iommu_tlb_ivld_addr (rw) register accessor: an alias for `Reg<IOMMU_TLB_IVLD_ADDR_SPEC>`"]
pub type IOMMU_TLB_IVLD_ADDR = crate::Reg<iommu_tlb_ivld_addr::IOMMU_TLB_IVLD_ADDR_SPEC>;
#[doc = "IOMMU TLB Invalidation Address Register"]
pub mod iommu_tlb_ivld_addr;
#[doc = "iommu_tlb_ivld_addr_mask (rw) register accessor: an alias for `Reg<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>`"]
pub type IOMMU_TLB_IVLD_ADDR_MASK =
    crate::Reg<iommu_tlb_ivld_addr_mask::IOMMU_TLB_IVLD_ADDR_MASK_SPEC>;
#[doc = "IOMMU TLB Invalidation Address Mask Register"]
pub mod iommu_tlb_ivld_addr_mask;
#[doc = "iommu_tlb_ivld_enable (rw) register accessor: an alias for `Reg<IOMMU_TLB_IVLD_ENABLE_SPEC>`"]
pub type IOMMU_TLB_IVLD_ENABLE = crate::Reg<iommu_tlb_ivld_enable::IOMMU_TLB_IVLD_ENABLE_SPEC>;
#[doc = "IOMMU TLB Invalidation Enable Register"]
pub mod iommu_tlb_ivld_enable;
#[doc = "iommu_pc_ivld_mode_sel (rw) register accessor: an alias for `Reg<IOMMU_PC_IVLD_MODE_SEL_SPEC>`"]
pub type IOMMU_PC_IVLD_MODE_SEL = crate::Reg<iommu_pc_ivld_mode_sel::IOMMU_PC_IVLD_MODE_SEL_SPEC>;
#[doc = "IOMMU PC Invalidation Mode Select Register"]
pub mod iommu_pc_ivld_mode_sel;
#[doc = "iommu_pc_ivld_addr (rw) register accessor: an alias for `Reg<IOMMU_PC_IVLD_ADDR_SPEC>`"]
pub type IOMMU_PC_IVLD_ADDR = crate::Reg<iommu_pc_ivld_addr::IOMMU_PC_IVLD_ADDR_SPEC>;
#[doc = "IOMMU PC Invalidation Address Register"]
pub mod iommu_pc_ivld_addr;
#[doc = "iommu_pc_ivld_sta_addr (rw) register accessor: an alias for `Reg<IOMMU_PC_IVLD_STA_ADDR_SPEC>`"]
pub type IOMMU_PC_IVLD_STA_ADDR = crate::Reg<iommu_pc_ivld_sta_addr::IOMMU_PC_IVLD_STA_ADDR_SPEC>;
#[doc = "IOMMU PC Invalidation Start Address Register"]
pub mod iommu_pc_ivld_sta_addr;
#[doc = "iommu_pc_ivld_enable (rw) register accessor: an alias for `Reg<IOMMU_PC_IVLD_ENABLE_SPEC>`"]
pub type IOMMU_PC_IVLD_ENABLE = crate::Reg<iommu_pc_ivld_enable::IOMMU_PC_IVLD_ENABLE_SPEC>;
#[doc = "IOMMU PC Invalidation Enable Register"]
pub mod iommu_pc_ivld_enable;
#[doc = "iommu_pc_ivld_end_addr (rw) register accessor: an alias for `Reg<IOMMU_PC_IVLD_END_ADDR_SPEC>`"]
pub type IOMMU_PC_IVLD_END_ADDR = crate::Reg<iommu_pc_ivld_end_addr::IOMMU_PC_IVLD_END_ADDR_SPEC>;
#[doc = "IOMMU PC Invalidation End Address Register"]
pub mod iommu_pc_ivld_end_addr;
#[doc = "iommu_dm_aut_ctrl (rw) register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL = crate::Reg<iommu_dm_aut_ctrl::IOMMU_DM_AUT_CTRL_SPEC>;
#[doc = "IOMMU Domain Authority Control \\[i\\] Register\n\nSoftware can set 15 different permission control types in IOMMU_DM_AUT_CTRL0-7. A default access control type is DOMAIN0. The read/write operation of DOMAIN1-15 is unlimited by default.\n\nSoftware needs to set the index of the permission control domain corresponding to the page table item in the bit\\[7:4\\] of the Level2 page table, the default value is 0 (use domain 0), that is, the read/write operation is not controlled.\n\nSetting REG_ARD_OVWT can mask the Domain control defined by IOMMU_DM_AUT_CTRL0-7. All Level2 page table type are covered by the type of REG_ARD_OVWT. The read/write operation is permitted by default."]
pub mod iommu_dm_aut_ctrl;
#[doc = "iommu_dm_aut_ovwt (rw) register accessor: an alias for `Reg<IOMMU_DM_AUT_OVWT_SPEC>`"]
pub type IOMMU_DM_AUT_OVWT = crate::Reg<iommu_dm_aut_ovwt::IOMMU_DM_AUT_OVWT_SPEC>;
#[doc = "IOMMU Domain Authority Overwrite Register"]
pub mod iommu_dm_aut_ovwt;
#[doc = "iommu_int_enable (rw) register accessor: an alias for `Reg<IOMMU_INT_ENABLE_SPEC>`"]
pub type IOMMU_INT_ENABLE = crate::Reg<iommu_int_enable::IOMMU_INT_ENABLE_SPEC>;
#[doc = "IOMMU Interrupt Enable Register"]
pub mod iommu_int_enable;
#[doc = "iommu_int_clr (rw) register accessor: an alias for `Reg<IOMMU_INT_CLR_SPEC>`"]
pub type IOMMU_INT_CLR = crate::Reg<iommu_int_clr::IOMMU_INT_CLR_SPEC>;
#[doc = "IOMMU Interrupt Clear Register"]
pub mod iommu_int_clr;
#[doc = "iommu_int_sta (rw) register accessor: an alias for `Reg<IOMMU_INT_STA_SPEC>`"]
pub type IOMMU_INT_STA = crate::Reg<iommu_int_sta::IOMMU_INT_STA_SPEC>;
#[doc = "IOMMU Interrupt Status Register"]
pub mod iommu_int_sta;
#[doc = "iommu_int_err_addr_tlb (r) register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR_TLB_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR_TLB = crate::Reg<iommu_int_err_addr_tlb::IOMMU_INT_ERR_ADDR_TLB_SPEC>;
#[doc = "IOMMU Interrupt Error Address \\[i\\]"]
pub mod iommu_int_err_addr_tlb;
#[doc = "iommu_int_err_addr_l (r) register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR_L_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR_L = crate::Reg<iommu_int_err_addr_l::IOMMU_INT_ERR_ADDR_L_SPEC>;
#[doc = "IOMMU Interrupt Error Address L\\[i\\]"]
pub mod iommu_int_err_addr_l;
#[doc = "iommu_int_err_data_tlb (r) register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA_TLB_SPEC>`"]
pub type IOMMU_INT_ERR_DATA_TLB = crate::Reg<iommu_int_err_data_tlb::IOMMU_INT_ERR_DATA_TLB_SPEC>;
#[doc = "IOMMU Interrupt Error Data \\[i\\] Register"]
pub mod iommu_int_err_data_tlb;
#[doc = "iommu_int_err_data_l (r) register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA_L_SPEC>`"]
pub type IOMMU_INT_ERR_DATA_L = crate::Reg<iommu_int_err_data_l::IOMMU_INT_ERR_DATA_L_SPEC>;
#[doc = "IOMMU Interrupt Error Data L\\[i\\] Register"]
pub mod iommu_int_err_data_l;
#[doc = "iommu_lpg_int (r) register accessor: an alias for `Reg<IOMMU_LPG_INT_SPEC>`"]
pub type IOMMU_LPG_INT = crate::Reg<iommu_lpg_int::IOMMU_LPG_INT_SPEC>;
#[doc = "IOMMU L\\[i\\] Page Table Interrupt Register"]
pub mod iommu_lpg_int;
#[doc = "iommu_va (rw) register accessor: an alias for `Reg<IOMMU_VA_SPEC>`"]
pub type IOMMU_VA = crate::Reg<iommu_va::IOMMU_VA_SPEC>;
#[doc = "IOMMU Virtual Address Register"]
pub mod iommu_va;
#[doc = "iommu_va_data (rw) register accessor: an alias for `Reg<IOMMU_VA_DATA_SPEC>`"]
pub type IOMMU_VA_DATA = crate::Reg<iommu_va_data::IOMMU_VA_DATA_SPEC>;
#[doc = "IOMMU Virtual Address Data Register"]
pub mod iommu_va_data;
#[doc = "iommu_va_config (rw) register accessor: an alias for `Reg<IOMMU_VA_CONFIG_SPEC>`"]
pub type IOMMU_VA_CONFIG = crate::Reg<iommu_va_config::IOMMU_VA_CONFIG_SPEC>;
#[doc = "IOMMU Virtual Address Configuration Register"]
pub mod iommu_va_config;
#[doc = "iommu_pmu_enable (rw) register accessor: an alias for `Reg<IOMMU_PMU_ENABLE_SPEC>`"]
pub type IOMMU_PMU_ENABLE = crate::Reg<iommu_pmu_enable::IOMMU_PMU_ENABLE_SPEC>;
#[doc = "IOMMU PMU Enable Register"]
pub mod iommu_pmu_enable;
#[doc = "iommu_pmu_clr (rw) register accessor: an alias for `Reg<IOMMU_PMU_CLR_SPEC>`"]
pub type IOMMU_PMU_CLR = crate::Reg<iommu_pmu_clr::IOMMU_PMU_CLR_SPEC>;
#[doc = "IOMMU PMU Clear Register"]
pub mod iommu_pmu_clr;
#[doc = "iommu_pmu_access_low (rw) register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW = crate::Reg<iommu_pmu_access_low::IOMMU_PMU_ACCESS_LOW_SPEC>;
#[doc = "IOMMU PMU Access Low \\[i\\] Register"]
pub mod iommu_pmu_access_low;
#[doc = "iommu_pmu_access_high (rw) register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH = crate::Reg<iommu_pmu_access_high::IOMMU_PMU_ACCESS_HIGH_SPEC>;
#[doc = "IOMMU PMU Access High \\[i\\] Register"]
pub mod iommu_pmu_access_high;
#[doc = "iommu_pmu_hit_low (rw) register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW = crate::Reg<iommu_pmu_hit_low::IOMMU_PMU_HIT_LOW_SPEC>;
#[doc = "IOMMU PMU Hit Low \\[i\\] Register"]
pub mod iommu_pmu_hit_low;
#[doc = "iommu_pmu_hit_high (rw) register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH = crate::Reg<iommu_pmu_hit_high::IOMMU_PMU_HIT_HIGH_SPEC>;
#[doc = "IOMMU PMU Hit High \\[i\\] Register"]
pub mod iommu_pmu_hit_high;
#[doc = "iommu_pmu_tl_low (rw) register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW_SPEC>`"]
pub type IOMMU_PMU_TL_LOW = crate::Reg<iommu_pmu_tl_low::IOMMU_PMU_TL_LOW_SPEC>;
#[doc = "IOMMU Total Latency Low \\[i\\] Register"]
pub mod iommu_pmu_tl_low;
#[doc = "iommu_pmu_tl_high (rw) register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH = crate::Reg<iommu_pmu_tl_high::IOMMU_PMU_TL_HIGH_SPEC>;
#[doc = "IOMMU Total Latency High \\[i\\] Register"]
pub mod iommu_pmu_tl_high;
#[doc = "iommu_pmu_ml (rw) register accessor: an alias for `Reg<IOMMU_PMU_ML_SPEC>`"]
pub type IOMMU_PMU_ML = crate::Reg<iommu_pmu_ml::IOMMU_PMU_ML_SPEC>;
#[doc = "IOMMU Max Latency \\[i\\] Register"]
pub mod iommu_pmu_ml;
