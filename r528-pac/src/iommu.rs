#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - IOMMU Reset Register"]
    pub iommu_reset_reg: crate::Reg<iommu_reset_reg::IOMMU_RESET_REG_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20 - IOMMU Enable Register"]
    pub iommu_enable_reg: crate::Reg<iommu_enable_reg::IOMMU_ENABLE_REG_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x30 - IOMMU Bypass Register"]
    pub iommu_bypass_reg: crate::Reg<iommu_bypass_reg::IOMMU_BYPASS_REG_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x40 - IOMMU Auto Gating Register"]
    pub iommu_auto_gating_reg: crate::Reg<iommu_auto_gating_reg::IOMMU_AUTO_GATING_REG_SPEC>,
    #[doc = "0x44 - IOMMU Write Buffer Control Register"]
    pub iommu_wbuf_ctrl_reg: crate::Reg<iommu_wbuf_ctrl_reg::IOMMU_WBUF_CTRL_REG_SPEC>,
    #[doc = "0x48 - IOMMU Out of Order Control Register"]
    pub iommu_ooo_ctrl_reg: crate::Reg<iommu_ooo_ctrl_reg::IOMMU_OOO_CTRL_REG_SPEC>,
    #[doc = "0x4c - IOMMU 4KB Boundary Protect Control Register"]
    pub iommu_4kb_bdy_prt_ctrl_reg:
        crate::Reg<iommu_4kb_bdy_prt_ctrl_reg::IOMMU_4KB_BDY_PRT_CTRL_REG_SPEC>,
    #[doc = "0x50 - IOMMU Translation Table Base Register"]
    pub iommu_ttb_reg: crate::Reg<iommu_ttb_reg::IOMMU_TTB_REG_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x60 - IOMMU TLB Enable Register"]
    pub iommu_tlb_enable_reg: crate::Reg<iommu_tlb_enable_reg::IOMMU_TLB_ENABLE_REG_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x70 - IOMMU TLB Prefetch Register"]
    pub iommu_tlb_prefetch_reg: crate::Reg<iommu_tlb_prefetch_reg::IOMMU_TLB_PREFETCH_REG_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x80 - IOMMU TLB Flush Enable Register"]
    pub iommu_tlb_flush_enable_reg:
        crate::Reg<iommu_tlb_flush_enable_reg::IOMMU_TLB_FLUSH_ENABLE_REG_SPEC>,
    #[doc = "0x84 - IOMMU TLB Invalidation Mode Select Register"]
    pub iommu_tlb_ivld_mode_sel_reg:
        crate::Reg<iommu_tlb_ivld_mode_sel_reg::IOMMU_TLB_IVLD_MODE_SEL_REG_SPEC>,
    #[doc = "0x88 - IOMMU TLB Invalidation Start Address Register"]
    pub iommu_tlb_ivld_sta_addr_reg:
        crate::Reg<iommu_tlb_ivld_sta_addr_reg::IOMMU_TLB_IVLD_STA_ADDR_REG_SPEC>,
    #[doc = "0x8c - IOMMU TLB Invalidation End Address Register"]
    pub iommu_tlb_ivld_end_addr_reg:
        crate::Reg<iommu_tlb_ivld_end_addr_reg::IOMMU_TLB_IVLD_END_ADDR_REG_SPEC>,
    #[doc = "0x90 - IOMMU TLB Invalidation Address Register"]
    pub iommu_tlb_ivld_addr_reg: crate::Reg<iommu_tlb_ivld_addr_reg::IOMMU_TLB_IVLD_ADDR_REG_SPEC>,
    #[doc = "0x94 - IOMMU TLB Invalidation Address Mask Register"]
    pub iommu_tlb_ivld_addr_mask_reg:
        crate::Reg<iommu_tlb_ivld_addr_mask_reg::IOMMU_TLB_IVLD_ADDR_MASK_REG_SPEC>,
    #[doc = "0x98 - IOMMU TLB Invalidation Enable Register"]
    pub iommu_tlb_ivld_enable_reg:
        crate::Reg<iommu_tlb_ivld_enable_reg::IOMMU_TLB_IVLD_ENABLE_REG_SPEC>,
    #[doc = "0x9c - IOMMU PC Invalidation Mode Select Register"]
    pub iommu_pc_ivld_mode_sel_reg:
        crate::Reg<iommu_pc_ivld_mode_sel_reg::IOMMU_PC_IVLD_MODE_SEL_REG_SPEC>,
    #[doc = "0xa0 - IOMMU PC Invalidation Address Register"]
    pub iommu_pc_ivld_addr_reg: crate::Reg<iommu_pc_ivld_addr_reg::IOMMU_PC_IVLD_ADDR_REG_SPEC>,
    #[doc = "0xa4 - IOMMU PC Invalidation Start Address Register"]
    pub iommu_pc_ivld_sta_addr_reg:
        crate::Reg<iommu_pc_ivld_sta_addr_reg::IOMMU_PC_IVLD_STA_ADDR_REG_SPEC>,
    #[doc = "0xa8 - IOMMU PC Invalidation Enable Register"]
    pub iommu_pc_ivld_enable_reg:
        crate::Reg<iommu_pc_ivld_enable_reg::IOMMU_PC_IVLD_ENABLE_REG_SPEC>,
    #[doc = "0xac - IOMMU PC Invalidation End Address Register"]
    pub iommu_pc_ivld_end_addr_reg:
        crate::Reg<iommu_pc_ivld_end_addr_reg::IOMMU_PC_IVLD_END_ADDR_REG_SPEC>,
    #[doc = "0xb0 - IOMMU Domain Authority Control 0 Register"]
    pub iommu_dm_aut_ctrl0_reg: crate::Reg<iommu_dm_aut_ctrl0_reg::IOMMU_DM_AUT_CTRL0_REG_SPEC>,
    #[doc = "0xb4 - IOMMU Domain Authority Control 1 Register"]
    pub iommu_dm_aut_ctrl1_reg: crate::Reg<iommu_dm_aut_ctrl1_reg::IOMMU_DM_AUT_CTRL1_REG_SPEC>,
    #[doc = "0xb8 - IOMMU Domain Authority Control 2 Register"]
    pub iommu_dm_aut_ctrl2_reg: crate::Reg<iommu_dm_aut_ctrl2_reg::IOMMU_DM_AUT_CTRL2_REG_SPEC>,
    #[doc = "0xbc - IOMMU Domain Authority Control 3 Register"]
    pub iommu_dm_aut_ctrl3_reg: crate::Reg<iommu_dm_aut_ctrl3_reg::IOMMU_DM_AUT_CTRL3_REG_SPEC>,
    #[doc = "0xc0 - IOMMU Domain Authority Control 4 Register"]
    pub iommu_dm_aut_ctrl4_reg: crate::Reg<iommu_dm_aut_ctrl4_reg::IOMMU_DM_AUT_CTRL4_REG_SPEC>,
    #[doc = "0xc4 - IOMMU Domain Authority Control 5 Register"]
    pub iommu_dm_aut_ctrl5_reg: crate::Reg<iommu_dm_aut_ctrl5_reg::IOMMU_DM_AUT_CTRL5_REG_SPEC>,
    #[doc = "0xc8 - IOMMU Domain Authority Control 6 Register"]
    pub iommu_dm_aut_ctrl6_reg: crate::Reg<iommu_dm_aut_ctrl6_reg::IOMMU_DM_AUT_CTRL6_REG_SPEC>,
    #[doc = "0xcc - IOMMU Domain Authority Control 7 Register"]
    pub iommu_dm_aut_ctrl7_reg: crate::Reg<iommu_dm_aut_ctrl7_reg::IOMMU_DM_AUT_CTRL7_REG_SPEC>,
    #[doc = "0xd0 - IOMMU Domain Authority Overwrite Register"]
    pub iommu_dm_aut_ovwt_reg: crate::Reg<iommu_dm_aut_ovwt_reg::IOMMU_DM_AUT_OVWT_REG_SPEC>,
    _reserved31: [u8; 0x2c],
    #[doc = "0x100 - IOMMU Interrupt Enable Register"]
    pub iommu_int_enable_reg: crate::Reg<iommu_int_enable_reg::IOMMU_INT_ENABLE_REG_SPEC>,
    #[doc = "0x104 - IOMMU Interrupt Clear Register"]
    pub iommu_int_clr_reg: crate::Reg<iommu_int_clr_reg::IOMMU_INT_CLR_REG_SPEC>,
    #[doc = "0x108 - IOMMU Interrupt Status Register"]
    pub iommu_int_sta_reg: crate::Reg<iommu_int_sta_reg::IOMMU_INT_STA_REG_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x110 - IOMMU Interrupt Error Address 0"]
    pub iommu_int_err_addr0_reg: crate::Reg<iommu_int_err_addr0_reg::IOMMU_INT_ERR_ADDR0_REG_SPEC>,
    #[doc = "0x114 - IOMMU Interrupt Error Address 1"]
    pub iommu_int_err_addr1_reg: crate::Reg<iommu_int_err_addr1_reg::IOMMU_INT_ERR_ADDR1_REG_SPEC>,
    #[doc = "0x118 - IOMMU Interrupt Error Address 2"]
    pub iommu_int_err_addr2_reg: crate::Reg<iommu_int_err_addr2_reg::IOMMU_INT_ERR_ADDR2_REG_SPEC>,
    #[doc = "0x11c - IOMMU Interrupt Error Address 3"]
    pub iommu_int_err_addr3_reg: crate::Reg<iommu_int_err_addr3_reg::IOMMU_INT_ERR_ADDR3_REG_SPEC>,
    #[doc = "0x120 - IOMMU Interrupt Error Address 4"]
    pub iommu_int_err_addr4_reg: crate::Reg<iommu_int_err_addr4_reg::IOMMU_INT_ERR_ADDR4_REG_SPEC>,
    #[doc = "0x124 - IOMMU Interrupt Error Address 5"]
    pub iommu_int_err_addr5_reg: crate::Reg<iommu_int_err_addr5_reg::IOMMU_INT_ERR_ADDR5_REG_SPEC>,
    #[doc = "0x128 - IOMMU Interrupt Error Address 6"]
    pub iommu_int_err_addr6_reg: crate::Reg<iommu_int_err_addr6_reg::IOMMU_INT_ERR_ADDR6_REG_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x130 - IOMMU Interrupt Error Address 7"]
    pub iommu_int_err_addr7_reg: crate::Reg<iommu_int_err_addr7_reg::IOMMU_INT_ERR_ADDR7_REG_SPEC>,
    #[doc = "0x134 - IOMMU Interrupt Error Address 8"]
    pub iommu_int_err_addr8_reg: crate::Reg<iommu_int_err_addr8_reg::IOMMU_INT_ERR_ADDR8_REG_SPEC>,
    _reserved43: [u8; 0x18],
    #[doc = "0x150 - IOMMU Interrupt Error Data 0 Register"]
    pub iommu_int_err_data0_reg: crate::Reg<iommu_int_err_data0_reg::IOMMU_INT_ERR_DATA0_REG_SPEC>,
    #[doc = "0x154 - IOMMU Interrupt Error Data 1 Register"]
    pub iommu_int_err_data1_reg: crate::Reg<iommu_int_err_data1_reg::IOMMU_INT_ERR_DATA1_REG_SPEC>,
    #[doc = "0x158 - IOMMU Interrupt Error Data 2 Register"]
    pub iommu_int_err_data2_reg: crate::Reg<iommu_int_err_data2_reg::IOMMU_INT_ERR_DATA2_REG_SPEC>,
    #[doc = "0x15c - IOMMU Interrupt Error Data 3 Register"]
    pub iommu_int_err_data3_reg: crate::Reg<iommu_int_err_data3_reg::IOMMU_INT_ERR_DATA3_REG_SPEC>,
    #[doc = "0x160 - IOMMU Interrupt Error Data 4 Register"]
    pub iommu_int_err_data4_reg: crate::Reg<iommu_int_err_data4_reg::IOMMU_INT_ERR_DATA4_REG_SPEC>,
    #[doc = "0x164 - IOMMU Interrupt Error Data 5 Register"]
    pub iommu_int_err_data5_reg: crate::Reg<iommu_int_err_data5_reg::IOMMU_INT_ERR_DATA5_REG_SPEC>,
    #[doc = "0x168 - IOMMU Interrupt Error Data 6 Register"]
    pub iommu_int_err_data6_reg: crate::Reg<iommu_int_err_data6_reg::IOMMU_INT_ERR_DATA6_REG_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x170 - IOMMU Interrupt Error Data 7 Register"]
    pub iommu_int_err_data7_reg: crate::Reg<iommu_int_err_data7_reg::IOMMU_INT_ERR_DATA7_REG_SPEC>,
    #[doc = "0x174 - IOMMU Interrupt Error Data 8 Register"]
    pub iommu_int_err_data8_reg: crate::Reg<iommu_int_err_data8_reg::IOMMU_INT_ERR_DATA8_REG_SPEC>,
    _reserved52: [u8; 0x08],
    #[doc = "0x180 - IOMMU L1 Page Table Interrupt Register"]
    pub iommu_l1pg_int_reg: crate::Reg<iommu_l1pg_int_reg::IOMMU_L1PG_INT_REG_SPEC>,
    #[doc = "0x184 - IOMMU L2 Page Table Interrupt Register"]
    pub iommu_l2pg_int_reg: crate::Reg<iommu_l2pg_int_reg::IOMMU_L2PG_INT_REG_SPEC>,
    _reserved54: [u8; 0x08],
    #[doc = "0x190 - IOMMU Virtual Address Register"]
    pub iommu_va_reg: crate::Reg<iommu_va_reg::IOMMU_VA_REG_SPEC>,
    #[doc = "0x194 - IOMMU Virtual Address Data Register"]
    pub iommu_va_data_reg: crate::Reg<iommu_va_data_reg::IOMMU_VA_DATA_REG_SPEC>,
    #[doc = "0x198 - IOMMU Virtual Address Configuration Register"]
    pub iommu_va_config_reg: crate::Reg<iommu_va_config_reg::IOMMU_VA_CONFIG_REG_SPEC>,
    _reserved57: [u8; 0x64],
    #[doc = "0x200 - IOMMU PMU Enable Register"]
    pub iommu_pmu_enable_reg: crate::Reg<iommu_pmu_enable_reg::IOMMU_PMU_ENABLE_REG_SPEC>,
    _reserved58: [u8; 0x0c],
    #[doc = "0x210 - IOMMU PMU Clear Register"]
    pub iommu_pmu_clr_reg: crate::Reg<iommu_pmu_clr_reg::IOMMU_PMU_CLR_REG_SPEC>,
    _reserved59: [u8; 0x1c],
    #[doc = "0x230 - IOMMU PMU Access Low 0 Register"]
    pub iommu_pmu_access_low0_reg:
        crate::Reg<iommu_pmu_access_low0_reg::IOMMU_PMU_ACCESS_LOW0_REG_SPEC>,
    #[doc = "0x234 - IOMMU PMU Access High 0 Register"]
    pub iommu_pmu_access_high0_reg:
        crate::Reg<iommu_pmu_access_high0_reg::IOMMU_PMU_ACCESS_HIGH0_REG_SPEC>,
    #[doc = "0x238 - IOMMU PMU Hit Low 0 Register"]
    pub iommu_pmu_hit_low0_reg: crate::Reg<iommu_pmu_hit_low0_reg::IOMMU_PMU_HIT_LOW0_REG_SPEC>,
    #[doc = "0x23c - IOMMU PMU Hit High 0 Register"]
    pub iommu_pmu_hit_high0_reg: crate::Reg<iommu_pmu_hit_high0_reg::IOMMU_PMU_HIT_HIGH0_REG_SPEC>,
    #[doc = "0x240 - IOMMU PMU Access Low 1 Register"]
    pub iommu_pmu_access_low1_reg:
        crate::Reg<iommu_pmu_access_low1_reg::IOMMU_PMU_ACCESS_LOW1_REG_SPEC>,
    #[doc = "0x244 - IOMMU PMU Access High 1 Register"]
    pub iommu_pmu_access_high1_reg:
        crate::Reg<iommu_pmu_access_high1_reg::IOMMU_PMU_ACCESS_HIGH1_REG_SPEC>,
    #[doc = "0x248 - IOMMU PMU Hit Low 1 Register"]
    pub iommu_pmu_hit_low1_reg: crate::Reg<iommu_pmu_hit_low1_reg::IOMMU_PMU_HIT_LOW1_REG_SPEC>,
    #[doc = "0x24c - IOMMU PMU Hit High 1 Register"]
    pub iommu_pmu_hit_high1_reg: crate::Reg<iommu_pmu_hit_high1_reg::IOMMU_PMU_HIT_HIGH1_REG_SPEC>,
    #[doc = "0x250 - IOMMU PMU Access Low 2 Register"]
    pub iommu_pmu_access_low2_reg:
        crate::Reg<iommu_pmu_access_low2_reg::IOMMU_PMU_ACCESS_LOW2_REG_SPEC>,
    #[doc = "0x254 - IOMMU PMU Access High 2 Register"]
    pub iommu_pmu_access_high2_reg:
        crate::Reg<iommu_pmu_access_high2_reg::IOMMU_PMU_ACCESS_HIGH2_REG_SPEC>,
    #[doc = "0x258 - IOMMU PMU Hit Low 2 Register"]
    pub iommu_pmu_hit_low2_reg: crate::Reg<iommu_pmu_hit_low2_reg::IOMMU_PMU_HIT_LOW2_REG_SPEC>,
    #[doc = "0x25c - IOMMU PMU Hit High 2 Register"]
    pub iommu_pmu_hit_high2_reg: crate::Reg<iommu_pmu_hit_high2_reg::IOMMU_PMU_HIT_HIGH2_REG_SPEC>,
    #[doc = "0x260 - IOMMU PMU Access Low 3 Register"]
    pub iommu_pmu_access_low3_reg:
        crate::Reg<iommu_pmu_access_low3_reg::IOMMU_PMU_ACCESS_LOW3_REG_SPEC>,
    #[doc = "0x264 - IOMMU PMU Access High 3 Register"]
    pub iommu_pmu_access_high3_reg:
        crate::Reg<iommu_pmu_access_high3_reg::IOMMU_PMU_ACCESS_HIGH3_REG_SPEC>,
    #[doc = "0x268 - IOMMU PMU Hit Low 3 Register"]
    pub iommu_pmu_hit_low3_reg: crate::Reg<iommu_pmu_hit_low3_reg::IOMMU_PMU_HIT_LOW3_REG_SPEC>,
    #[doc = "0x26c - IOMMU PMU Hit High 3 Register"]
    pub iommu_pmu_hit_high3_reg: crate::Reg<iommu_pmu_hit_high3_reg::IOMMU_PMU_HIT_HIGH3_REG_SPEC>,
    #[doc = "0x270 - IOMMU PMU Access Low 4 Register"]
    pub iommu_pmu_access_low4_reg:
        crate::Reg<iommu_pmu_access_low4_reg::IOMMU_PMU_ACCESS_LOW4_REG_SPEC>,
    #[doc = "0x274 - IOMMU PMU Access High 4 Register"]
    pub iommu_pmu_access_high4_reg:
        crate::Reg<iommu_pmu_access_high4_reg::IOMMU_PMU_ACCESS_HIGH4_REG_SPEC>,
    #[doc = "0x278 - IOMMU PMU Hit Low 4 Register"]
    pub iommu_pmu_hit_low4_reg: crate::Reg<iommu_pmu_hit_low4_reg::IOMMU_PMU_HIT_LOW4_REG_SPEC>,
    #[doc = "0x27c - IOMMU PMU Hit High 4 Register"]
    pub iommu_pmu_hit_high4_reg: crate::Reg<iommu_pmu_hit_high4_reg::IOMMU_PMU_HIT_HIGH4_REG_SPEC>,
    #[doc = "0x280 - IOMMU PMU Access Low 5 Register"]
    pub iommu_pmu_access_low5_reg:
        crate::Reg<iommu_pmu_access_low5_reg::IOMMU_PMU_ACCESS_LOW5_REG_SPEC>,
    #[doc = "0x284 - IOMMU PMU Access High 5 Register"]
    pub iommu_pmu_access_high5_reg:
        crate::Reg<iommu_pmu_access_high5_reg::IOMMU_PMU_ACCESS_HIGH5_REG_SPEC>,
    #[doc = "0x288 - IOMMU PMU Hit Low 5 Register"]
    pub iommu_pmu_hit_low5_reg: crate::Reg<iommu_pmu_hit_low5_reg::IOMMU_PMU_HIT_LOW5_REG_SPEC>,
    #[doc = "0x28c - IOMMU PMU Hit High 5 Register"]
    pub iommu_pmu_hit_high5_reg: crate::Reg<iommu_pmu_hit_high5_reg::IOMMU_PMU_HIT_HIGH5_REG_SPEC>,
    #[doc = "0x290 - IOMMU PMU Access Low 6 Register"]
    pub iommu_pmu_access_low6_reg:
        crate::Reg<iommu_pmu_access_low6_reg::IOMMU_PMU_ACCESS_LOW6_REG_SPEC>,
    #[doc = "0x294 - IOMMU PMU Access High 6 Register"]
    pub iommu_pmu_access_high6_reg:
        crate::Reg<iommu_pmu_access_high6_reg::IOMMU_PMU_ACCESS_HIGH6_REG_SPEC>,
    #[doc = "0x298 - IOMMU PMU Hit Low 6 Register"]
    pub iommu_pmu_hit_low6_reg: crate::Reg<iommu_pmu_hit_low6_reg::IOMMU_PMU_HIT_LOW6_REG_SPEC>,
    #[doc = "0x29c - IOMMU PMU Hit High 6 Register"]
    pub iommu_pmu_hit_high6_reg: crate::Reg<iommu_pmu_hit_high6_reg::IOMMU_PMU_HIT_HIGH6_REG_SPEC>,
    _reserved87: [u8; 0x30],
    #[doc = "0x2d0 - IOMMU PMU Access Low 7 Register"]
    pub iommu_pmu_access_low7_reg:
        crate::Reg<iommu_pmu_access_low7_reg::IOMMU_PMU_ACCESS_LOW7_REG_SPEC>,
    #[doc = "0x2d4 - IOMMU PMU Access High 7 Register"]
    pub iommu_pmu_access_high7_reg:
        crate::Reg<iommu_pmu_access_high7_reg::IOMMU_PMU_ACCESS_HIGH7_REG_SPEC>,
    #[doc = "0x2d8 - IOMMU PMU Hit Low 7 Register"]
    pub iommu_pmu_hit_low7_reg: crate::Reg<iommu_pmu_hit_low7_reg::IOMMU_PMU_HIT_LOW7_REG_SPEC>,
    #[doc = "0x2dc - IOMMU PMU Hit High 7 Register"]
    pub iommu_pmu_hit_high7_reg: crate::Reg<iommu_pmu_hit_high7_reg::IOMMU_PMU_HIT_HIGH7_REG_SPEC>,
    #[doc = "0x2e0 - IOMMU PMU Access Low 8 Register"]
    pub iommu_pmu_access_low8_reg:
        crate::Reg<iommu_pmu_access_low8_reg::IOMMU_PMU_ACCESS_LOW8_REG_SPEC>,
    #[doc = "0x2e4 - IOMMU PMU Access High 8 Register"]
    pub iommu_pmu_access_high8_reg:
        crate::Reg<iommu_pmu_access_high8_reg::IOMMU_PMU_ACCESS_HIGH8_REG_SPEC>,
    #[doc = "0x2e8 - IOMMU PMU Hit Low 8 Register"]
    pub iommu_pmu_hit_low8_reg: crate::Reg<iommu_pmu_hit_low8_reg::IOMMU_PMU_HIT_LOW8_REG_SPEC>,
    #[doc = "0x2ec - IOMMU PMU Hit High 8 Register"]
    pub iommu_pmu_hit_high8_reg: crate::Reg<iommu_pmu_hit_high8_reg::IOMMU_PMU_HIT_HIGH8_REG_SPEC>,
    _reserved95: [u8; 0x10],
    #[doc = "0x300 - IOMMU Total Latency Low 0 Register"]
    pub iommu_pmu_tl_low0_reg: crate::Reg<iommu_pmu_tl_low0_reg::IOMMU_PMU_TL_LOW0_REG_SPEC>,
    #[doc = "0x304 - IOMMU Total Latency High 0 Register"]
    pub iommu_pmu_tl_high0_reg: crate::Reg<iommu_pmu_tl_high0_reg::IOMMU_PMU_TL_HIGH0_REG_SPEC>,
    #[doc = "0x308 - IOMMU Max Latency 0 Register"]
    pub iommu_pmu_ml0_reg: crate::Reg<iommu_pmu_ml0_reg::IOMMU_PMU_ML0_REG_SPEC>,
    _reserved98: [u8; 0x04],
    #[doc = "0x310 - IOMMU Total Latency Low 1 Register"]
    pub iommu_pmu_tl_low1_reg: crate::Reg<iommu_pmu_tl_low1_reg::IOMMU_PMU_TL_LOW1_REG_SPEC>,
    #[doc = "0x314 - IOMMU Total Latency High 1 Register"]
    pub iommu_pmu_tl_high1_reg: crate::Reg<iommu_pmu_tl_high1_reg::IOMMU_PMU_TL_HIGH1_REG_SPEC>,
    #[doc = "0x318 - IOMMU Max Latency 1 Register"]
    pub iommu_pmu_ml1_reg: crate::Reg<iommu_pmu_ml1_reg::IOMMU_PMU_ML1_REG_SPEC>,
    _reserved101: [u8; 0x04],
    #[doc = "0x320 - IOMMU Total Latency Low 2 Register"]
    pub iommu_pmu_tl_low2_reg: crate::Reg<iommu_pmu_tl_low2_reg::IOMMU_PMU_TL_LOW2_REG_SPEC>,
    #[doc = "0x324 - IOMMU Total Latency High 2 Register"]
    pub iommu_pmu_tl_high2_reg: crate::Reg<iommu_pmu_tl_high2_reg::IOMMU_PMU_TL_HIGH2_REG_SPEC>,
    #[doc = "0x328 - IOMMU Max Latency 2 Register"]
    pub iommu_pmu_ml2_reg: crate::Reg<iommu_pmu_ml2_reg::IOMMU_PMU_ML2_REG_SPEC>,
    _reserved104: [u8; 0x04],
    #[doc = "0x330 - IOMMU Total Latency Low 3 Register"]
    pub iommu_pmu_tl_low3_reg: crate::Reg<iommu_pmu_tl_low3_reg::IOMMU_PMU_TL_LOW3_REG_SPEC>,
    #[doc = "0x334 - IOMMU Total Latency High 3 Register"]
    pub iommu_pmu_tl_high3_reg: crate::Reg<iommu_pmu_tl_high3_reg::IOMMU_PMU_TL_HIGH3_REG_SPEC>,
    #[doc = "0x338 - IOMMU Max Latency 3 Register"]
    pub iommu_pmu_ml3_reg: crate::Reg<iommu_pmu_ml3_reg::IOMMU_PMU_ML3_REG_SPEC>,
    _reserved107: [u8; 0x04],
    #[doc = "0x340 - IOMMU Total Latency Low 4 Register"]
    pub iommu_pmu_tl_low4_reg: crate::Reg<iommu_pmu_tl_low4_reg::IOMMU_PMU_TL_LOW4_REG_SPEC>,
    #[doc = "0x344 - IOMMU Total Latency High 4 Register"]
    pub iommu_pmu_tl_high4_reg: crate::Reg<iommu_pmu_tl_high4_reg::IOMMU_PMU_TL_HIGH4_REG_SPEC>,
    #[doc = "0x348 - IOMMU Max Latency 4 Register"]
    pub iommu_pmu_ml4_reg: crate::Reg<iommu_pmu_ml4_reg::IOMMU_PMU_ML4_REG_SPEC>,
    _reserved110: [u8; 0x04],
    #[doc = "0x350 - IOMMU Total Latency Low 5 Register"]
    pub iommu_pmu_tl_low5_reg: crate::Reg<iommu_pmu_tl_low5_reg::IOMMU_PMU_TL_LOW5_REG_SPEC>,
    #[doc = "0x354 - IOMMU Total Latency High 5 Register"]
    pub iommu_pmu_tl_high5_reg: crate::Reg<iommu_pmu_tl_high5_reg::IOMMU_PMU_TL_HIGH5_REG_SPEC>,
    #[doc = "0x358 - IOMMU Max Latency 5 Register"]
    pub iommu_pmu_ml5_reg: crate::Reg<iommu_pmu_ml5_reg::IOMMU_PMU_ML5_REG_SPEC>,
    _reserved113: [u8; 0x04],
    #[doc = "0x360 - IOMMU Total Latency Low 6 Register"]
    pub iommu_pmu_tl_low6_reg: crate::Reg<iommu_pmu_tl_low6_reg::IOMMU_PMU_TL_LOW6_REG_SPEC>,
    #[doc = "0x364 - IOMMU Total Latency High 6 Register"]
    pub iommu_pmu_tl_high6_reg: crate::Reg<iommu_pmu_tl_high6_reg::IOMMU_PMU_TL_HIGH6_REG_SPEC>,
    #[doc = "0x368 - IOMMU Max Latency 6 Register"]
    pub iommu_pmu_ml6_reg: crate::Reg<iommu_pmu_ml6_reg::IOMMU_PMU_ML6_REG_SPEC>,
}
#[doc = "IOMMU_RESET_REG register accessor: an alias for `Reg<IOMMU_RESET_REG_SPEC>`"]
pub type IOMMU_RESET_REG = crate::Reg<iommu_reset_reg::IOMMU_RESET_REG_SPEC>;
#[doc = "IOMMU Reset Register"]
pub mod iommu_reset_reg;
#[doc = "IOMMU_ENABLE_REG register accessor: an alias for `Reg<IOMMU_ENABLE_REG_SPEC>`"]
pub type IOMMU_ENABLE_REG = crate::Reg<iommu_enable_reg::IOMMU_ENABLE_REG_SPEC>;
#[doc = "IOMMU Enable Register"]
pub mod iommu_enable_reg;
#[doc = "IOMMU_BYPASS_REG register accessor: an alias for `Reg<IOMMU_BYPASS_REG_SPEC>`"]
pub type IOMMU_BYPASS_REG = crate::Reg<iommu_bypass_reg::IOMMU_BYPASS_REG_SPEC>;
#[doc = "IOMMU Bypass Register"]
pub mod iommu_bypass_reg;
#[doc = "IOMMU_AUTO_GATING_REG register accessor: an alias for `Reg<IOMMU_AUTO_GATING_REG_SPEC>`"]
pub type IOMMU_AUTO_GATING_REG = crate::Reg<iommu_auto_gating_reg::IOMMU_AUTO_GATING_REG_SPEC>;
#[doc = "IOMMU Auto Gating Register"]
pub mod iommu_auto_gating_reg;
#[doc = "IOMMU_WBUF_CTRL_REG register accessor: an alias for `Reg<IOMMU_WBUF_CTRL_REG_SPEC>`"]
pub type IOMMU_WBUF_CTRL_REG = crate::Reg<iommu_wbuf_ctrl_reg::IOMMU_WBUF_CTRL_REG_SPEC>;
#[doc = "IOMMU Write Buffer Control Register"]
pub mod iommu_wbuf_ctrl_reg;
#[doc = "IOMMU_OOO_CTRL_REG register accessor: an alias for `Reg<IOMMU_OOO_CTRL_REG_SPEC>`"]
pub type IOMMU_OOO_CTRL_REG = crate::Reg<iommu_ooo_ctrl_reg::IOMMU_OOO_CTRL_REG_SPEC>;
#[doc = "IOMMU Out of Order Control Register"]
pub mod iommu_ooo_ctrl_reg;
#[doc = "IOMMU_4KB_BDY_PRT_CTRL_REG register accessor: an alias for `Reg<IOMMU_4KB_BDY_PRT_CTRL_REG_SPEC>`"]
pub type IOMMU_4KB_BDY_PRT_CTRL_REG =
    crate::Reg<iommu_4kb_bdy_prt_ctrl_reg::IOMMU_4KB_BDY_PRT_CTRL_REG_SPEC>;
#[doc = "IOMMU 4KB Boundary Protect Control Register"]
pub mod iommu_4kb_bdy_prt_ctrl_reg;
#[doc = "IOMMU_TTB_REG register accessor: an alias for `Reg<IOMMU_TTB_REG_SPEC>`"]
pub type IOMMU_TTB_REG = crate::Reg<iommu_ttb_reg::IOMMU_TTB_REG_SPEC>;
#[doc = "IOMMU Translation Table Base Register"]
pub mod iommu_ttb_reg;
#[doc = "IOMMU_TLB_ENABLE_REG register accessor: an alias for `Reg<IOMMU_TLB_ENABLE_REG_SPEC>`"]
pub type IOMMU_TLB_ENABLE_REG = crate::Reg<iommu_tlb_enable_reg::IOMMU_TLB_ENABLE_REG_SPEC>;
#[doc = "IOMMU TLB Enable Register"]
pub mod iommu_tlb_enable_reg;
#[doc = "IOMMU_TLB_PREFETCH_REG register accessor: an alias for `Reg<IOMMU_TLB_PREFETCH_REG_SPEC>`"]
pub type IOMMU_TLB_PREFETCH_REG = crate::Reg<iommu_tlb_prefetch_reg::IOMMU_TLB_PREFETCH_REG_SPEC>;
#[doc = "IOMMU TLB Prefetch Register"]
pub mod iommu_tlb_prefetch_reg;
#[doc = "IOMMU_TLB_FLUSH_ENABLE_REG register accessor: an alias for `Reg<IOMMU_TLB_FLUSH_ENABLE_REG_SPEC>`"]
pub type IOMMU_TLB_FLUSH_ENABLE_REG =
    crate::Reg<iommu_tlb_flush_enable_reg::IOMMU_TLB_FLUSH_ENABLE_REG_SPEC>;
#[doc = "IOMMU TLB Flush Enable Register"]
pub mod iommu_tlb_flush_enable_reg;
#[doc = "IOMMU_TLB_IVLD_MODE_SEL_REG register accessor: an alias for `Reg<IOMMU_TLB_IVLD_MODE_SEL_REG_SPEC>`"]
pub type IOMMU_TLB_IVLD_MODE_SEL_REG =
    crate::Reg<iommu_tlb_ivld_mode_sel_reg::IOMMU_TLB_IVLD_MODE_SEL_REG_SPEC>;
#[doc = "IOMMU TLB Invalidation Mode Select Register"]
pub mod iommu_tlb_ivld_mode_sel_reg;
#[doc = "IOMMU_TLB_IVLD_STA_ADDR_REG register accessor: an alias for `Reg<IOMMU_TLB_IVLD_STA_ADDR_REG_SPEC>`"]
pub type IOMMU_TLB_IVLD_STA_ADDR_REG =
    crate::Reg<iommu_tlb_ivld_sta_addr_reg::IOMMU_TLB_IVLD_STA_ADDR_REG_SPEC>;
#[doc = "IOMMU TLB Invalidation Start Address Register"]
pub mod iommu_tlb_ivld_sta_addr_reg;
#[doc = "IOMMU_TLB_IVLD_END_ADDR_REG register accessor: an alias for `Reg<IOMMU_TLB_IVLD_END_ADDR_REG_SPEC>`"]
pub type IOMMU_TLB_IVLD_END_ADDR_REG =
    crate::Reg<iommu_tlb_ivld_end_addr_reg::IOMMU_TLB_IVLD_END_ADDR_REG_SPEC>;
#[doc = "IOMMU TLB Invalidation End Address Register"]
pub mod iommu_tlb_ivld_end_addr_reg;
#[doc = "IOMMU_TLB_IVLD_ADDR_REG register accessor: an alias for `Reg<IOMMU_TLB_IVLD_ADDR_REG_SPEC>`"]
pub type IOMMU_TLB_IVLD_ADDR_REG =
    crate::Reg<iommu_tlb_ivld_addr_reg::IOMMU_TLB_IVLD_ADDR_REG_SPEC>;
#[doc = "IOMMU TLB Invalidation Address Register"]
pub mod iommu_tlb_ivld_addr_reg;
#[doc = "IOMMU_TLB_IVLD_ADDR_MASK_REG register accessor: an alias for `Reg<IOMMU_TLB_IVLD_ADDR_MASK_REG_SPEC>`"]
pub type IOMMU_TLB_IVLD_ADDR_MASK_REG =
    crate::Reg<iommu_tlb_ivld_addr_mask_reg::IOMMU_TLB_IVLD_ADDR_MASK_REG_SPEC>;
#[doc = "IOMMU TLB Invalidation Address Mask Register"]
pub mod iommu_tlb_ivld_addr_mask_reg;
#[doc = "IOMMU_TLB_IVLD_ENABLE_REG register accessor: an alias for `Reg<IOMMU_TLB_IVLD_ENABLE_REG_SPEC>`"]
pub type IOMMU_TLB_IVLD_ENABLE_REG =
    crate::Reg<iommu_tlb_ivld_enable_reg::IOMMU_TLB_IVLD_ENABLE_REG_SPEC>;
#[doc = "IOMMU TLB Invalidation Enable Register"]
pub mod iommu_tlb_ivld_enable_reg;
#[doc = "IOMMU_PC_IVLD_MODE_SEL_REG register accessor: an alias for `Reg<IOMMU_PC_IVLD_MODE_SEL_REG_SPEC>`"]
pub type IOMMU_PC_IVLD_MODE_SEL_REG =
    crate::Reg<iommu_pc_ivld_mode_sel_reg::IOMMU_PC_IVLD_MODE_SEL_REG_SPEC>;
#[doc = "IOMMU PC Invalidation Mode Select Register"]
pub mod iommu_pc_ivld_mode_sel_reg;
#[doc = "IOMMU_PC_IVLD_ADDR_REG register accessor: an alias for `Reg<IOMMU_PC_IVLD_ADDR_REG_SPEC>`"]
pub type IOMMU_PC_IVLD_ADDR_REG = crate::Reg<iommu_pc_ivld_addr_reg::IOMMU_PC_IVLD_ADDR_REG_SPEC>;
#[doc = "IOMMU PC Invalidation Address Register"]
pub mod iommu_pc_ivld_addr_reg;
#[doc = "IOMMU_PC_IVLD_STA_ADDR_REG register accessor: an alias for `Reg<IOMMU_PC_IVLD_STA_ADDR_REG_SPEC>`"]
pub type IOMMU_PC_IVLD_STA_ADDR_REG =
    crate::Reg<iommu_pc_ivld_sta_addr_reg::IOMMU_PC_IVLD_STA_ADDR_REG_SPEC>;
#[doc = "IOMMU PC Invalidation Start Address Register"]
pub mod iommu_pc_ivld_sta_addr_reg;
#[doc = "IOMMU_PC_IVLD_ENABLE_REG register accessor: an alias for `Reg<IOMMU_PC_IVLD_ENABLE_REG_SPEC>`"]
pub type IOMMU_PC_IVLD_ENABLE_REG =
    crate::Reg<iommu_pc_ivld_enable_reg::IOMMU_PC_IVLD_ENABLE_REG_SPEC>;
#[doc = "IOMMU PC Invalidation Enable Register"]
pub mod iommu_pc_ivld_enable_reg;
#[doc = "IOMMU_PC_IVLD_END_ADDR_REG register accessor: an alias for `Reg<IOMMU_PC_IVLD_END_ADDR_REG_SPEC>`"]
pub type IOMMU_PC_IVLD_END_ADDR_REG =
    crate::Reg<iommu_pc_ivld_end_addr_reg::IOMMU_PC_IVLD_END_ADDR_REG_SPEC>;
#[doc = "IOMMU PC Invalidation End Address Register"]
pub mod iommu_pc_ivld_end_addr_reg;
#[doc = "IOMMU_DM_AUT_CTRL0_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL0_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL0_REG = crate::Reg<iommu_dm_aut_ctrl0_reg::IOMMU_DM_AUT_CTRL0_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 0 Register"]
pub mod iommu_dm_aut_ctrl0_reg;
#[doc = "IOMMU_DM_AUT_CTRL1_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL1_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL1_REG = crate::Reg<iommu_dm_aut_ctrl1_reg::IOMMU_DM_AUT_CTRL1_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 1 Register"]
pub mod iommu_dm_aut_ctrl1_reg;
#[doc = "IOMMU_DM_AUT_CTRL2_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL2_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL2_REG = crate::Reg<iommu_dm_aut_ctrl2_reg::IOMMU_DM_AUT_CTRL2_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 2 Register"]
pub mod iommu_dm_aut_ctrl2_reg;
#[doc = "IOMMU_DM_AUT_CTRL3_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL3_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL3_REG = crate::Reg<iommu_dm_aut_ctrl3_reg::IOMMU_DM_AUT_CTRL3_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 3 Register"]
pub mod iommu_dm_aut_ctrl3_reg;
#[doc = "IOMMU_DM_AUT_CTRL4_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL4_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL4_REG = crate::Reg<iommu_dm_aut_ctrl4_reg::IOMMU_DM_AUT_CTRL4_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 4 Register"]
pub mod iommu_dm_aut_ctrl4_reg;
#[doc = "IOMMU_DM_AUT_CTRL5_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL5_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL5_REG = crate::Reg<iommu_dm_aut_ctrl5_reg::IOMMU_DM_AUT_CTRL5_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 5 Register"]
pub mod iommu_dm_aut_ctrl5_reg;
#[doc = "IOMMU_DM_AUT_CTRL6_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL6_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL6_REG = crate::Reg<iommu_dm_aut_ctrl6_reg::IOMMU_DM_AUT_CTRL6_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 6 Register"]
pub mod iommu_dm_aut_ctrl6_reg;
#[doc = "IOMMU_DM_AUT_CTRL7_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_CTRL7_REG_SPEC>`"]
pub type IOMMU_DM_AUT_CTRL7_REG = crate::Reg<iommu_dm_aut_ctrl7_reg::IOMMU_DM_AUT_CTRL7_REG_SPEC>;
#[doc = "IOMMU Domain Authority Control 7 Register"]
pub mod iommu_dm_aut_ctrl7_reg;
#[doc = "IOMMU_DM_AUT_OVWT_REG register accessor: an alias for `Reg<IOMMU_DM_AUT_OVWT_REG_SPEC>`"]
pub type IOMMU_DM_AUT_OVWT_REG = crate::Reg<iommu_dm_aut_ovwt_reg::IOMMU_DM_AUT_OVWT_REG_SPEC>;
#[doc = "IOMMU Domain Authority Overwrite Register"]
pub mod iommu_dm_aut_ovwt_reg;
#[doc = "IOMMU_INT_ENABLE_REG register accessor: an alias for `Reg<IOMMU_INT_ENABLE_REG_SPEC>`"]
pub type IOMMU_INT_ENABLE_REG = crate::Reg<iommu_int_enable_reg::IOMMU_INT_ENABLE_REG_SPEC>;
#[doc = "IOMMU Interrupt Enable Register"]
pub mod iommu_int_enable_reg;
#[doc = "IOMMU_INT_CLR_REG register accessor: an alias for `Reg<IOMMU_INT_CLR_REG_SPEC>`"]
pub type IOMMU_INT_CLR_REG = crate::Reg<iommu_int_clr_reg::IOMMU_INT_CLR_REG_SPEC>;
#[doc = "IOMMU Interrupt Clear Register"]
pub mod iommu_int_clr_reg;
#[doc = "IOMMU_INT_STA_REG register accessor: an alias for `Reg<IOMMU_INT_STA_REG_SPEC>`"]
pub type IOMMU_INT_STA_REG = crate::Reg<iommu_int_sta_reg::IOMMU_INT_STA_REG_SPEC>;
#[doc = "IOMMU Interrupt Status Register"]
pub mod iommu_int_sta_reg;
#[doc = "IOMMU_INT_ERR_ADDR0_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR0_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR0_REG =
    crate::Reg<iommu_int_err_addr0_reg::IOMMU_INT_ERR_ADDR0_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 0"]
pub mod iommu_int_err_addr0_reg;
#[doc = "IOMMU_INT_ERR_ADDR1_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR1_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR1_REG =
    crate::Reg<iommu_int_err_addr1_reg::IOMMU_INT_ERR_ADDR1_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 1"]
pub mod iommu_int_err_addr1_reg;
#[doc = "IOMMU_INT_ERR_ADDR2_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR2_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR2_REG =
    crate::Reg<iommu_int_err_addr2_reg::IOMMU_INT_ERR_ADDR2_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 2"]
pub mod iommu_int_err_addr2_reg;
#[doc = "IOMMU_INT_ERR_ADDR3_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR3_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR3_REG =
    crate::Reg<iommu_int_err_addr3_reg::IOMMU_INT_ERR_ADDR3_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 3"]
pub mod iommu_int_err_addr3_reg;
#[doc = "IOMMU_INT_ERR_ADDR4_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR4_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR4_REG =
    crate::Reg<iommu_int_err_addr4_reg::IOMMU_INT_ERR_ADDR4_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 4"]
pub mod iommu_int_err_addr4_reg;
#[doc = "IOMMU_INT_ERR_ADDR5_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR5_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR5_REG =
    crate::Reg<iommu_int_err_addr5_reg::IOMMU_INT_ERR_ADDR5_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 5"]
pub mod iommu_int_err_addr5_reg;
#[doc = "IOMMU_INT_ERR_ADDR6_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR6_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR6_REG =
    crate::Reg<iommu_int_err_addr6_reg::IOMMU_INT_ERR_ADDR6_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 6"]
pub mod iommu_int_err_addr6_reg;
#[doc = "IOMMU_INT_ERR_ADDR7_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR7_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR7_REG =
    crate::Reg<iommu_int_err_addr7_reg::IOMMU_INT_ERR_ADDR7_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 7"]
pub mod iommu_int_err_addr7_reg;
#[doc = "IOMMU_INT_ERR_ADDR8_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_ADDR8_REG_SPEC>`"]
pub type IOMMU_INT_ERR_ADDR8_REG =
    crate::Reg<iommu_int_err_addr8_reg::IOMMU_INT_ERR_ADDR8_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Address 8"]
pub mod iommu_int_err_addr8_reg;
#[doc = "IOMMU_INT_ERR_DATA0_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA0_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA0_REG =
    crate::Reg<iommu_int_err_data0_reg::IOMMU_INT_ERR_DATA0_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 0 Register"]
pub mod iommu_int_err_data0_reg;
#[doc = "IOMMU_INT_ERR_DATA1_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA1_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA1_REG =
    crate::Reg<iommu_int_err_data1_reg::IOMMU_INT_ERR_DATA1_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 1 Register"]
pub mod iommu_int_err_data1_reg;
#[doc = "IOMMU_INT_ERR_DATA2_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA2_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA2_REG =
    crate::Reg<iommu_int_err_data2_reg::IOMMU_INT_ERR_DATA2_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 2 Register"]
pub mod iommu_int_err_data2_reg;
#[doc = "IOMMU_INT_ERR_DATA3_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA3_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA3_REG =
    crate::Reg<iommu_int_err_data3_reg::IOMMU_INT_ERR_DATA3_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 3 Register"]
pub mod iommu_int_err_data3_reg;
#[doc = "IOMMU_INT_ERR_DATA4_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA4_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA4_REG =
    crate::Reg<iommu_int_err_data4_reg::IOMMU_INT_ERR_DATA4_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 4 Register"]
pub mod iommu_int_err_data4_reg;
#[doc = "IOMMU_INT_ERR_DATA5_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA5_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA5_REG =
    crate::Reg<iommu_int_err_data5_reg::IOMMU_INT_ERR_DATA5_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 5 Register"]
pub mod iommu_int_err_data5_reg;
#[doc = "IOMMU_INT_ERR_DATA6_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA6_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA6_REG =
    crate::Reg<iommu_int_err_data6_reg::IOMMU_INT_ERR_DATA6_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 6 Register"]
pub mod iommu_int_err_data6_reg;
#[doc = "IOMMU_INT_ERR_DATA7_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA7_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA7_REG =
    crate::Reg<iommu_int_err_data7_reg::IOMMU_INT_ERR_DATA7_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 7 Register"]
pub mod iommu_int_err_data7_reg;
#[doc = "IOMMU_INT_ERR_DATA8_REG register accessor: an alias for `Reg<IOMMU_INT_ERR_DATA8_REG_SPEC>`"]
pub type IOMMU_INT_ERR_DATA8_REG =
    crate::Reg<iommu_int_err_data8_reg::IOMMU_INT_ERR_DATA8_REG_SPEC>;
#[doc = "IOMMU Interrupt Error Data 8 Register"]
pub mod iommu_int_err_data8_reg;
#[doc = "IOMMU_L1PG_INT_REG register accessor: an alias for `Reg<IOMMU_L1PG_INT_REG_SPEC>`"]
pub type IOMMU_L1PG_INT_REG = crate::Reg<iommu_l1pg_int_reg::IOMMU_L1PG_INT_REG_SPEC>;
#[doc = "IOMMU L1 Page Table Interrupt Register"]
pub mod iommu_l1pg_int_reg;
#[doc = "IOMMU_L2PG_INT_REG register accessor: an alias for `Reg<IOMMU_L2PG_INT_REG_SPEC>`"]
pub type IOMMU_L2PG_INT_REG = crate::Reg<iommu_l2pg_int_reg::IOMMU_L2PG_INT_REG_SPEC>;
#[doc = "IOMMU L2 Page Table Interrupt Register"]
pub mod iommu_l2pg_int_reg;
#[doc = "IOMMU_VA_REG register accessor: an alias for `Reg<IOMMU_VA_REG_SPEC>`"]
pub type IOMMU_VA_REG = crate::Reg<iommu_va_reg::IOMMU_VA_REG_SPEC>;
#[doc = "IOMMU Virtual Address Register"]
pub mod iommu_va_reg;
#[doc = "IOMMU_VA_DATA_REG register accessor: an alias for `Reg<IOMMU_VA_DATA_REG_SPEC>`"]
pub type IOMMU_VA_DATA_REG = crate::Reg<iommu_va_data_reg::IOMMU_VA_DATA_REG_SPEC>;
#[doc = "IOMMU Virtual Address Data Register"]
pub mod iommu_va_data_reg;
#[doc = "IOMMU_VA_CONFIG_REG register accessor: an alias for `Reg<IOMMU_VA_CONFIG_REG_SPEC>`"]
pub type IOMMU_VA_CONFIG_REG = crate::Reg<iommu_va_config_reg::IOMMU_VA_CONFIG_REG_SPEC>;
#[doc = "IOMMU Virtual Address Configuration Register"]
pub mod iommu_va_config_reg;
#[doc = "IOMMU_PMU_ENABLE_REG register accessor: an alias for `Reg<IOMMU_PMU_ENABLE_REG_SPEC>`"]
pub type IOMMU_PMU_ENABLE_REG = crate::Reg<iommu_pmu_enable_reg::IOMMU_PMU_ENABLE_REG_SPEC>;
#[doc = "IOMMU PMU Enable Register"]
pub mod iommu_pmu_enable_reg;
#[doc = "IOMMU_PMU_CLR_REG register accessor: an alias for `Reg<IOMMU_PMU_CLR_REG_SPEC>`"]
pub type IOMMU_PMU_CLR_REG = crate::Reg<iommu_pmu_clr_reg::IOMMU_PMU_CLR_REG_SPEC>;
#[doc = "IOMMU PMU Clear Register"]
pub mod iommu_pmu_clr_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW0_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW0_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW0_REG =
    crate::Reg<iommu_pmu_access_low0_reg::IOMMU_PMU_ACCESS_LOW0_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 0 Register"]
pub mod iommu_pmu_access_low0_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH0_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH0_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH0_REG =
    crate::Reg<iommu_pmu_access_high0_reg::IOMMU_PMU_ACCESS_HIGH0_REG_SPEC>;
#[doc = "IOMMU PMU Access High 0 Register"]
pub mod iommu_pmu_access_high0_reg;
#[doc = "IOMMU_PMU_HIT_LOW0_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW0_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW0_REG = crate::Reg<iommu_pmu_hit_low0_reg::IOMMU_PMU_HIT_LOW0_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 0 Register"]
pub mod iommu_pmu_hit_low0_reg;
#[doc = "IOMMU_PMU_HIT_HIGH0_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH0_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH0_REG =
    crate::Reg<iommu_pmu_hit_high0_reg::IOMMU_PMU_HIT_HIGH0_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 0 Register"]
pub mod iommu_pmu_hit_high0_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW1_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW1_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW1_REG =
    crate::Reg<iommu_pmu_access_low1_reg::IOMMU_PMU_ACCESS_LOW1_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 1 Register"]
pub mod iommu_pmu_access_low1_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH1_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH1_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH1_REG =
    crate::Reg<iommu_pmu_access_high1_reg::IOMMU_PMU_ACCESS_HIGH1_REG_SPEC>;
#[doc = "IOMMU PMU Access High 1 Register"]
pub mod iommu_pmu_access_high1_reg;
#[doc = "IOMMU_PMU_HIT_LOW1_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW1_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW1_REG = crate::Reg<iommu_pmu_hit_low1_reg::IOMMU_PMU_HIT_LOW1_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 1 Register"]
pub mod iommu_pmu_hit_low1_reg;
#[doc = "IOMMU_PMU_HIT_HIGH1_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH1_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH1_REG =
    crate::Reg<iommu_pmu_hit_high1_reg::IOMMU_PMU_HIT_HIGH1_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 1 Register"]
pub mod iommu_pmu_hit_high1_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW2_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW2_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW2_REG =
    crate::Reg<iommu_pmu_access_low2_reg::IOMMU_PMU_ACCESS_LOW2_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 2 Register"]
pub mod iommu_pmu_access_low2_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH2_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH2_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH2_REG =
    crate::Reg<iommu_pmu_access_high2_reg::IOMMU_PMU_ACCESS_HIGH2_REG_SPEC>;
#[doc = "IOMMU PMU Access High 2 Register"]
pub mod iommu_pmu_access_high2_reg;
#[doc = "IOMMU_PMU_HIT_LOW2_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW2_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW2_REG = crate::Reg<iommu_pmu_hit_low2_reg::IOMMU_PMU_HIT_LOW2_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 2 Register"]
pub mod iommu_pmu_hit_low2_reg;
#[doc = "IOMMU_PMU_HIT_HIGH2_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH2_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH2_REG =
    crate::Reg<iommu_pmu_hit_high2_reg::IOMMU_PMU_HIT_HIGH2_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 2 Register"]
pub mod iommu_pmu_hit_high2_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW3_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW3_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW3_REG =
    crate::Reg<iommu_pmu_access_low3_reg::IOMMU_PMU_ACCESS_LOW3_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 3 Register"]
pub mod iommu_pmu_access_low3_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH3_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH3_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH3_REG =
    crate::Reg<iommu_pmu_access_high3_reg::IOMMU_PMU_ACCESS_HIGH3_REG_SPEC>;
#[doc = "IOMMU PMU Access High 3 Register"]
pub mod iommu_pmu_access_high3_reg;
#[doc = "IOMMU_PMU_HIT_LOW3_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW3_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW3_REG = crate::Reg<iommu_pmu_hit_low3_reg::IOMMU_PMU_HIT_LOW3_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 3 Register"]
pub mod iommu_pmu_hit_low3_reg;
#[doc = "IOMMU_PMU_HIT_HIGH3_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH3_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH3_REG =
    crate::Reg<iommu_pmu_hit_high3_reg::IOMMU_PMU_HIT_HIGH3_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 3 Register"]
pub mod iommu_pmu_hit_high3_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW4_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW4_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW4_REG =
    crate::Reg<iommu_pmu_access_low4_reg::IOMMU_PMU_ACCESS_LOW4_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 4 Register"]
pub mod iommu_pmu_access_low4_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH4_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH4_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH4_REG =
    crate::Reg<iommu_pmu_access_high4_reg::IOMMU_PMU_ACCESS_HIGH4_REG_SPEC>;
#[doc = "IOMMU PMU Access High 4 Register"]
pub mod iommu_pmu_access_high4_reg;
#[doc = "IOMMU_PMU_HIT_LOW4_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW4_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW4_REG = crate::Reg<iommu_pmu_hit_low4_reg::IOMMU_PMU_HIT_LOW4_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 4 Register"]
pub mod iommu_pmu_hit_low4_reg;
#[doc = "IOMMU_PMU_HIT_HIGH4_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH4_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH4_REG =
    crate::Reg<iommu_pmu_hit_high4_reg::IOMMU_PMU_HIT_HIGH4_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 4 Register"]
pub mod iommu_pmu_hit_high4_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW5_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW5_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW5_REG =
    crate::Reg<iommu_pmu_access_low5_reg::IOMMU_PMU_ACCESS_LOW5_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 5 Register"]
pub mod iommu_pmu_access_low5_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH5_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH5_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH5_REG =
    crate::Reg<iommu_pmu_access_high5_reg::IOMMU_PMU_ACCESS_HIGH5_REG_SPEC>;
#[doc = "IOMMU PMU Access High 5 Register"]
pub mod iommu_pmu_access_high5_reg;
#[doc = "IOMMU_PMU_HIT_LOW5_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW5_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW5_REG = crate::Reg<iommu_pmu_hit_low5_reg::IOMMU_PMU_HIT_LOW5_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 5 Register"]
pub mod iommu_pmu_hit_low5_reg;
#[doc = "IOMMU_PMU_HIT_HIGH5_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH5_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH5_REG =
    crate::Reg<iommu_pmu_hit_high5_reg::IOMMU_PMU_HIT_HIGH5_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 5 Register"]
pub mod iommu_pmu_hit_high5_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW6_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW6_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW6_REG =
    crate::Reg<iommu_pmu_access_low6_reg::IOMMU_PMU_ACCESS_LOW6_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 6 Register"]
pub mod iommu_pmu_access_low6_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH6_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH6_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH6_REG =
    crate::Reg<iommu_pmu_access_high6_reg::IOMMU_PMU_ACCESS_HIGH6_REG_SPEC>;
#[doc = "IOMMU PMU Access High 6 Register"]
pub mod iommu_pmu_access_high6_reg;
#[doc = "IOMMU_PMU_HIT_LOW6_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW6_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW6_REG = crate::Reg<iommu_pmu_hit_low6_reg::IOMMU_PMU_HIT_LOW6_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 6 Register"]
pub mod iommu_pmu_hit_low6_reg;
#[doc = "IOMMU_PMU_HIT_HIGH6_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH6_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH6_REG =
    crate::Reg<iommu_pmu_hit_high6_reg::IOMMU_PMU_HIT_HIGH6_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 6 Register"]
pub mod iommu_pmu_hit_high6_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW7_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW7_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW7_REG =
    crate::Reg<iommu_pmu_access_low7_reg::IOMMU_PMU_ACCESS_LOW7_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 7 Register"]
pub mod iommu_pmu_access_low7_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH7_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH7_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH7_REG =
    crate::Reg<iommu_pmu_access_high7_reg::IOMMU_PMU_ACCESS_HIGH7_REG_SPEC>;
#[doc = "IOMMU PMU Access High 7 Register"]
pub mod iommu_pmu_access_high7_reg;
#[doc = "IOMMU_PMU_HIT_LOW7_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW7_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW7_REG = crate::Reg<iommu_pmu_hit_low7_reg::IOMMU_PMU_HIT_LOW7_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 7 Register"]
pub mod iommu_pmu_hit_low7_reg;
#[doc = "IOMMU_PMU_HIT_HIGH7_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH7_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH7_REG =
    crate::Reg<iommu_pmu_hit_high7_reg::IOMMU_PMU_HIT_HIGH7_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 7 Register"]
pub mod iommu_pmu_hit_high7_reg;
#[doc = "IOMMU_PMU_ACCESS_LOW8_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_LOW8_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_LOW8_REG =
    crate::Reg<iommu_pmu_access_low8_reg::IOMMU_PMU_ACCESS_LOW8_REG_SPEC>;
#[doc = "IOMMU PMU Access Low 8 Register"]
pub mod iommu_pmu_access_low8_reg;
#[doc = "IOMMU_PMU_ACCESS_HIGH8_REG register accessor: an alias for `Reg<IOMMU_PMU_ACCESS_HIGH8_REG_SPEC>`"]
pub type IOMMU_PMU_ACCESS_HIGH8_REG =
    crate::Reg<iommu_pmu_access_high8_reg::IOMMU_PMU_ACCESS_HIGH8_REG_SPEC>;
#[doc = "IOMMU PMU Access High 8 Register"]
pub mod iommu_pmu_access_high8_reg;
#[doc = "IOMMU_PMU_HIT_LOW8_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_LOW8_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_LOW8_REG = crate::Reg<iommu_pmu_hit_low8_reg::IOMMU_PMU_HIT_LOW8_REG_SPEC>;
#[doc = "IOMMU PMU Hit Low 8 Register"]
pub mod iommu_pmu_hit_low8_reg;
#[doc = "IOMMU_PMU_HIT_HIGH8_REG register accessor: an alias for `Reg<IOMMU_PMU_HIT_HIGH8_REG_SPEC>`"]
pub type IOMMU_PMU_HIT_HIGH8_REG =
    crate::Reg<iommu_pmu_hit_high8_reg::IOMMU_PMU_HIT_HIGH8_REG_SPEC>;
#[doc = "IOMMU PMU Hit High 8 Register"]
pub mod iommu_pmu_hit_high8_reg;
#[doc = "IOMMU_PMU_TL_LOW0_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW0_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW0_REG = crate::Reg<iommu_pmu_tl_low0_reg::IOMMU_PMU_TL_LOW0_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 0 Register"]
pub mod iommu_pmu_tl_low0_reg;
#[doc = "IOMMU_PMU_TL_HIGH0_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH0_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH0_REG = crate::Reg<iommu_pmu_tl_high0_reg::IOMMU_PMU_TL_HIGH0_REG_SPEC>;
#[doc = "IOMMU Total Latency High 0 Register"]
pub mod iommu_pmu_tl_high0_reg;
#[doc = "IOMMU_PMU_ML0_REG register accessor: an alias for `Reg<IOMMU_PMU_ML0_REG_SPEC>`"]
pub type IOMMU_PMU_ML0_REG = crate::Reg<iommu_pmu_ml0_reg::IOMMU_PMU_ML0_REG_SPEC>;
#[doc = "IOMMU Max Latency 0 Register"]
pub mod iommu_pmu_ml0_reg;
#[doc = "IOMMU_PMU_TL_LOW1_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW1_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW1_REG = crate::Reg<iommu_pmu_tl_low1_reg::IOMMU_PMU_TL_LOW1_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 1 Register"]
pub mod iommu_pmu_tl_low1_reg;
#[doc = "IOMMU_PMU_TL_HIGH1_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH1_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH1_REG = crate::Reg<iommu_pmu_tl_high1_reg::IOMMU_PMU_TL_HIGH1_REG_SPEC>;
#[doc = "IOMMU Total Latency High 1 Register"]
pub mod iommu_pmu_tl_high1_reg;
#[doc = "IOMMU_PMU_ML1_REG register accessor: an alias for `Reg<IOMMU_PMU_ML1_REG_SPEC>`"]
pub type IOMMU_PMU_ML1_REG = crate::Reg<iommu_pmu_ml1_reg::IOMMU_PMU_ML1_REG_SPEC>;
#[doc = "IOMMU Max Latency 1 Register"]
pub mod iommu_pmu_ml1_reg;
#[doc = "IOMMU_PMU_TL_LOW2_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW2_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW2_REG = crate::Reg<iommu_pmu_tl_low2_reg::IOMMU_PMU_TL_LOW2_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 2 Register"]
pub mod iommu_pmu_tl_low2_reg;
#[doc = "IOMMU_PMU_TL_HIGH2_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH2_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH2_REG = crate::Reg<iommu_pmu_tl_high2_reg::IOMMU_PMU_TL_HIGH2_REG_SPEC>;
#[doc = "IOMMU Total Latency High 2 Register"]
pub mod iommu_pmu_tl_high2_reg;
#[doc = "IOMMU_PMU_ML2_REG register accessor: an alias for `Reg<IOMMU_PMU_ML2_REG_SPEC>`"]
pub type IOMMU_PMU_ML2_REG = crate::Reg<iommu_pmu_ml2_reg::IOMMU_PMU_ML2_REG_SPEC>;
#[doc = "IOMMU Max Latency 2 Register"]
pub mod iommu_pmu_ml2_reg;
#[doc = "IOMMU_PMU_TL_LOW3_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW3_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW3_REG = crate::Reg<iommu_pmu_tl_low3_reg::IOMMU_PMU_TL_LOW3_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 3 Register"]
pub mod iommu_pmu_tl_low3_reg;
#[doc = "IOMMU_PMU_TL_HIGH3_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH3_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH3_REG = crate::Reg<iommu_pmu_tl_high3_reg::IOMMU_PMU_TL_HIGH3_REG_SPEC>;
#[doc = "IOMMU Total Latency High 3 Register"]
pub mod iommu_pmu_tl_high3_reg;
#[doc = "IOMMU_PMU_ML3_REG register accessor: an alias for `Reg<IOMMU_PMU_ML3_REG_SPEC>`"]
pub type IOMMU_PMU_ML3_REG = crate::Reg<iommu_pmu_ml3_reg::IOMMU_PMU_ML3_REG_SPEC>;
#[doc = "IOMMU Max Latency 3 Register"]
pub mod iommu_pmu_ml3_reg;
#[doc = "IOMMU_PMU_TL_LOW4_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW4_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW4_REG = crate::Reg<iommu_pmu_tl_low4_reg::IOMMU_PMU_TL_LOW4_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 4 Register"]
pub mod iommu_pmu_tl_low4_reg;
#[doc = "IOMMU_PMU_TL_HIGH4_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH4_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH4_REG = crate::Reg<iommu_pmu_tl_high4_reg::IOMMU_PMU_TL_HIGH4_REG_SPEC>;
#[doc = "IOMMU Total Latency High 4 Register"]
pub mod iommu_pmu_tl_high4_reg;
#[doc = "IOMMU_PMU_ML4_REG register accessor: an alias for `Reg<IOMMU_PMU_ML4_REG_SPEC>`"]
pub type IOMMU_PMU_ML4_REG = crate::Reg<iommu_pmu_ml4_reg::IOMMU_PMU_ML4_REG_SPEC>;
#[doc = "IOMMU Max Latency 4 Register"]
pub mod iommu_pmu_ml4_reg;
#[doc = "IOMMU_PMU_TL_LOW5_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW5_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW5_REG = crate::Reg<iommu_pmu_tl_low5_reg::IOMMU_PMU_TL_LOW5_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 5 Register"]
pub mod iommu_pmu_tl_low5_reg;
#[doc = "IOMMU_PMU_TL_HIGH5_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH5_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH5_REG = crate::Reg<iommu_pmu_tl_high5_reg::IOMMU_PMU_TL_HIGH5_REG_SPEC>;
#[doc = "IOMMU Total Latency High 5 Register"]
pub mod iommu_pmu_tl_high5_reg;
#[doc = "IOMMU_PMU_ML5_REG register accessor: an alias for `Reg<IOMMU_PMU_ML5_REG_SPEC>`"]
pub type IOMMU_PMU_ML5_REG = crate::Reg<iommu_pmu_ml5_reg::IOMMU_PMU_ML5_REG_SPEC>;
#[doc = "IOMMU Max Latency 5 Register"]
pub mod iommu_pmu_ml5_reg;
#[doc = "IOMMU_PMU_TL_LOW6_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_LOW6_REG_SPEC>`"]
pub type IOMMU_PMU_TL_LOW6_REG = crate::Reg<iommu_pmu_tl_low6_reg::IOMMU_PMU_TL_LOW6_REG_SPEC>;
#[doc = "IOMMU Total Latency Low 6 Register"]
pub mod iommu_pmu_tl_low6_reg;
#[doc = "IOMMU_PMU_TL_HIGH6_REG register accessor: an alias for `Reg<IOMMU_PMU_TL_HIGH6_REG_SPEC>`"]
pub type IOMMU_PMU_TL_HIGH6_REG = crate::Reg<iommu_pmu_tl_high6_reg::IOMMU_PMU_TL_HIGH6_REG_SPEC>;
#[doc = "IOMMU Total Latency High 6 Register"]
pub mod iommu_pmu_tl_high6_reg;
#[doc = "IOMMU_PMU_ML6_REG register accessor: an alias for `Reg<IOMMU_PMU_ML6_REG_SPEC>`"]
pub type IOMMU_PMU_ML6_REG = crate::Reg<iommu_pmu_ml6_reg::IOMMU_PMU_ML6_REG_SPEC>;
#[doc = "IOMMU Max Latency 6 Register"]
pub mod iommu_pmu_ml6_reg;
