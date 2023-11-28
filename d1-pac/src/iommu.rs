#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    iommu_reset: IOMMU_RESET,
    _reserved1: [u8; 0x0c],
    iommu_enable: IOMMU_ENABLE,
    _reserved2: [u8; 0x0c],
    iommu_bypass: IOMMU_BYPASS,
    _reserved3: [u8; 0x0c],
    iommu_auto_gating: IOMMU_AUTO_GATING,
    iommu_wbuf_ctrl: IOMMU_WBUF_CTRL,
    iommu_ooo_ctrl: IOMMU_OOO_CTRL,
    iommu_4kb_bdy_prt_ctrl: IOMMU_4KB_BDY_PRT_CTRL,
    iommu_ttb: IOMMU_TTB,
    _reserved8: [u8; 0x0c],
    iommu_tlb_enable: IOMMU_TLB_ENABLE,
    _reserved9: [u8; 0x0c],
    iommu_tlb_prefetch: IOMMU_TLB_PREFETCH,
    _reserved10: [u8; 0x0c],
    iommu_tlb_flush_enable: IOMMU_TLB_FLUSH_ENABLE,
    iommu_tlb_ivld_mode_sel: IOMMU_TLB_IVLD_MODE_SEL,
    iommu_tlb_ivld_sta_addr: IOMMU_TLB_IVLD_STA_ADDR,
    iommu_tlb_ivld_end_addr: IOMMU_TLB_IVLD_END_ADDR,
    iommu_tlb_ivld_addr: IOMMU_TLB_IVLD_ADDR,
    iommu_tlb_ivld_addr_mask: IOMMU_TLB_IVLD_ADDR_MASK,
    iommu_tlb_ivld_enable: IOMMU_TLB_IVLD_ENABLE,
    iommu_pc_ivld_mode_sel: IOMMU_PC_IVLD_MODE_SEL,
    iommu_pc_ivld_addr: IOMMU_PC_IVLD_ADDR,
    iommu_pc_ivld_sta_addr: IOMMU_PC_IVLD_STA_ADDR,
    iommu_pc_ivld_enable: IOMMU_PC_IVLD_ENABLE,
    iommu_pc_ivld_end_addr: IOMMU_PC_IVLD_END_ADDR,
    iommu_dm_aut_ctrl: [IOMMU_DM_AUT_CTRL; 8],
    iommu_dm_aut_ovwt: IOMMU_DM_AUT_OVWT,
    _reserved24: [u8; 0x2c],
    iommu_int_enable: IOMMU_INT_ENABLE,
    iommu_int_clr: IOMMU_INT_CLR,
    iommu_int_sta: IOMMU_INT_STA,
    _reserved27: [u8; 0x04],
    iommu_int_err_addr_tlb: [IOMMU_INT_ERR_ADDR_TLB; 7],
    _reserved28: [u8; 0x04],
    iommu_int_err_addr_l: [IOMMU_INT_ERR_ADDR_L; 2],
    _reserved29: [u8; 0x18],
    iommu_int_err_data_tlb: [IOMMU_INT_ERR_DATA_TLB; 7],
    _reserved30: [u8; 0x04],
    iommu_int_err_data_l: [IOMMU_INT_ERR_DATA_L; 2],
    _reserved31: [u8; 0x08],
    iommu_lpg_int: [IOMMU_LPG_INT; 2],
    _reserved32: [u8; 0x08],
    iommu_va: IOMMU_VA,
    iommu_va_data: IOMMU_VA_DATA,
    iommu_va_config: IOMMU_VA_CONFIG,
    _reserved35: [u8; 0x64],
    iommu_pmu_enable: IOMMU_PMU_ENABLE,
    _reserved36: [u8; 0x0c],
    iommu_pmu_clr: IOMMU_PMU_CLR,
    _reserved37: [u8; 0x1c],
    iommu_pmu_access_low: (),
    _reserved38: [u8; 0x04],
    iommu_pmu_access_high: (),
    _reserved39: [u8; 0x04],
    iommu_pmu_hit_low: (),
    _reserved40: [u8; 0x04],
    iommu_pmu_hit_high: (),
    _reserved41: [u8; 0xc4],
    iommu_pmu_tl_low: (),
    _reserved42: [u8; 0x04],
    iommu_pmu_tl_high: (),
    _reserved43: [u8; 0x04],
    iommu_pmu_ml: (),
}
impl RegisterBlock {
    #[doc = "0x10 - IOMMU Reset Register"]
    #[inline(always)]
    pub const fn iommu_reset(&self) -> &IOMMU_RESET {
        &self.iommu_reset
    }
    #[doc = "0x20 - IOMMU Enable Register"]
    #[inline(always)]
    pub const fn iommu_enable(&self) -> &IOMMU_ENABLE {
        &self.iommu_enable
    }
    #[doc = "0x30 - IOMMU Bypass Register"]
    #[inline(always)]
    pub const fn iommu_bypass(&self) -> &IOMMU_BYPASS {
        &self.iommu_bypass
    }
    #[doc = "0x40 - IOMMU Auto Gating Register"]
    #[inline(always)]
    pub const fn iommu_auto_gating(&self) -> &IOMMU_AUTO_GATING {
        &self.iommu_auto_gating
    }
    #[doc = "0x44 - IOMMU Write Buffer Control Register"]
    #[inline(always)]
    pub const fn iommu_wbuf_ctrl(&self) -> &IOMMU_WBUF_CTRL {
        &self.iommu_wbuf_ctrl
    }
    #[doc = "0x48 - IOMMU Out of Order Control Register"]
    #[inline(always)]
    pub const fn iommu_ooo_ctrl(&self) -> &IOMMU_OOO_CTRL {
        &self.iommu_ooo_ctrl
    }
    #[doc = "0x4c - IOMMU 4KB Boundary Protect Control Register"]
    #[inline(always)]
    pub const fn iommu_4kb_bdy_prt_ctrl(&self) -> &IOMMU_4KB_BDY_PRT_CTRL {
        &self.iommu_4kb_bdy_prt_ctrl
    }
    #[doc = "0x50 - IOMMU Translation Table Base Register"]
    #[inline(always)]
    pub const fn iommu_ttb(&self) -> &IOMMU_TTB {
        &self.iommu_ttb
    }
    #[doc = "0x60 - IOMMU TLB Enable Register"]
    #[inline(always)]
    pub const fn iommu_tlb_enable(&self) -> &IOMMU_TLB_ENABLE {
        &self.iommu_tlb_enable
    }
    #[doc = "0x70 - IOMMU TLB Prefetch Register"]
    #[inline(always)]
    pub const fn iommu_tlb_prefetch(&self) -> &IOMMU_TLB_PREFETCH {
        &self.iommu_tlb_prefetch
    }
    #[doc = "0x80 - IOMMU TLB Flush Enable Register"]
    #[inline(always)]
    pub const fn iommu_tlb_flush_enable(&self) -> &IOMMU_TLB_FLUSH_ENABLE {
        &self.iommu_tlb_flush_enable
    }
    #[doc = "0x84 - IOMMU TLB Invalidation Mode Select Register"]
    #[inline(always)]
    pub const fn iommu_tlb_ivld_mode_sel(&self) -> &IOMMU_TLB_IVLD_MODE_SEL {
        &self.iommu_tlb_ivld_mode_sel
    }
    #[doc = "0x88 - IOMMU TLB Invalidation Start Address Register"]
    #[inline(always)]
    pub const fn iommu_tlb_ivld_sta_addr(&self) -> &IOMMU_TLB_IVLD_STA_ADDR {
        &self.iommu_tlb_ivld_sta_addr
    }
    #[doc = "0x8c - IOMMU TLB Invalidation End Address Register"]
    #[inline(always)]
    pub const fn iommu_tlb_ivld_end_addr(&self) -> &IOMMU_TLB_IVLD_END_ADDR {
        &self.iommu_tlb_ivld_end_addr
    }
    #[doc = "0x90 - IOMMU TLB Invalidation Address Register"]
    #[inline(always)]
    pub const fn iommu_tlb_ivld_addr(&self) -> &IOMMU_TLB_IVLD_ADDR {
        &self.iommu_tlb_ivld_addr
    }
    #[doc = "0x94 - IOMMU TLB Invalidation Address Mask Register"]
    #[inline(always)]
    pub const fn iommu_tlb_ivld_addr_mask(&self) -> &IOMMU_TLB_IVLD_ADDR_MASK {
        &self.iommu_tlb_ivld_addr_mask
    }
    #[doc = "0x98 - IOMMU TLB Invalidation Enable Register"]
    #[inline(always)]
    pub const fn iommu_tlb_ivld_enable(&self) -> &IOMMU_TLB_IVLD_ENABLE {
        &self.iommu_tlb_ivld_enable
    }
    #[doc = "0x9c - IOMMU PC Invalidation Mode Select Register"]
    #[inline(always)]
    pub const fn iommu_pc_ivld_mode_sel(&self) -> &IOMMU_PC_IVLD_MODE_SEL {
        &self.iommu_pc_ivld_mode_sel
    }
    #[doc = "0xa0 - IOMMU PC Invalidation Address Register"]
    #[inline(always)]
    pub const fn iommu_pc_ivld_addr(&self) -> &IOMMU_PC_IVLD_ADDR {
        &self.iommu_pc_ivld_addr
    }
    #[doc = "0xa4 - IOMMU PC Invalidation Start Address Register"]
    #[inline(always)]
    pub const fn iommu_pc_ivld_sta_addr(&self) -> &IOMMU_PC_IVLD_STA_ADDR {
        &self.iommu_pc_ivld_sta_addr
    }
    #[doc = "0xa8 - IOMMU PC Invalidation Enable Register"]
    #[inline(always)]
    pub const fn iommu_pc_ivld_enable(&self) -> &IOMMU_PC_IVLD_ENABLE {
        &self.iommu_pc_ivld_enable
    }
    #[doc = "0xac - IOMMU PC Invalidation End Address Register"]
    #[inline(always)]
    pub const fn iommu_pc_ivld_end_addr(&self) -> &IOMMU_PC_IVLD_END_ADDR {
        &self.iommu_pc_ivld_end_addr
    }
    #[doc = "0xb0..0xd0 - IOMMU Domain Authority Control \\[i\\] Register\n\nSoftware can set 15 different permission control types in IOMMU_DM_AUT_CTRL0-7. A default access control type is DOMAIN0. The read/write operation of DOMAIN1-15 is unlimited by default.\n\nSoftware needs to set the index of the permission control domain corresponding to the page table item in the bit\\[7:4\\] of the Level2 page table, the default value is 0 (use domain 0), that is, the read/write operation is not controlled.\n\nSetting REG_ARD_OVWT can mask the Domain control defined by IOMMU_DM_AUT_CTRL0-7. All Level2 page table type are covered by the type of REG_ARD_OVWT. The read/write operation is permitted by default."]
    #[inline(always)]
    pub const fn iommu_dm_aut_ctrl(&self, n: usize) -> &IOMMU_DM_AUT_CTRL {
        &self.iommu_dm_aut_ctrl[n]
    }
    #[doc = "0xd0 - IOMMU Domain Authority Overwrite Register"]
    #[inline(always)]
    pub const fn iommu_dm_aut_ovwt(&self) -> &IOMMU_DM_AUT_OVWT {
        &self.iommu_dm_aut_ovwt
    }
    #[doc = "0x100 - IOMMU Interrupt Enable Register"]
    #[inline(always)]
    pub const fn iommu_int_enable(&self) -> &IOMMU_INT_ENABLE {
        &self.iommu_int_enable
    }
    #[doc = "0x104 - IOMMU Interrupt Clear Register"]
    #[inline(always)]
    pub const fn iommu_int_clr(&self) -> &IOMMU_INT_CLR {
        &self.iommu_int_clr
    }
    #[doc = "0x108 - IOMMU Interrupt Status Register"]
    #[inline(always)]
    pub const fn iommu_int_sta(&self) -> &IOMMU_INT_STA {
        &self.iommu_int_sta
    }
    #[doc = "0x110..0x12c - IOMMU Interrupt Error Address \\[i\\]"]
    #[inline(always)]
    pub const fn iommu_int_err_addr_tlb(&self, n: usize) -> &IOMMU_INT_ERR_ADDR_TLB {
        &self.iommu_int_err_addr_tlb[n]
    }
    #[doc = "0x130..0x138 - IOMMU Interrupt Error Address L\\[i\\]"]
    #[inline(always)]
    pub const fn iommu_int_err_addr_l(&self, n: usize) -> &IOMMU_INT_ERR_ADDR_L {
        &self.iommu_int_err_addr_l[n]
    }
    #[doc = "0x150..0x16c - IOMMU Interrupt Error Data \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_int_err_data_tlb(&self, n: usize) -> &IOMMU_INT_ERR_DATA_TLB {
        &self.iommu_int_err_data_tlb[n]
    }
    #[doc = "0x170..0x178 - IOMMU Interrupt Error Data L\\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_int_err_data_l(&self, n: usize) -> &IOMMU_INT_ERR_DATA_L {
        &self.iommu_int_err_data_l[n]
    }
    #[doc = "0x180..0x188 - IOMMU L\\[i\\] Page Table Interrupt Register"]
    #[inline(always)]
    pub const fn iommu_lpg_int(&self, n: usize) -> &IOMMU_LPG_INT {
        &self.iommu_lpg_int[n]
    }
    #[doc = "0x180 - IOMMU L\\[i\\] Page Table Interrupt Register"]
    #[inline(always)]
    pub const fn iommu_l0pg_int(&self) -> &IOMMU_LPG_INT {
        self.iommu_lpg_int(0)
    }
    #[doc = "0x184 - IOMMU L\\[i\\] Page Table Interrupt Register"]
    #[inline(always)]
    pub const fn iommu_l1pg_int(&self) -> &IOMMU_LPG_INT {
        self.iommu_lpg_int(1)
    }
    #[doc = "0x190 - IOMMU Virtual Address Register"]
    #[inline(always)]
    pub const fn iommu_va(&self) -> &IOMMU_VA {
        &self.iommu_va
    }
    #[doc = "0x194 - IOMMU Virtual Address Data Register"]
    #[inline(always)]
    pub const fn iommu_va_data(&self) -> &IOMMU_VA_DATA {
        &self.iommu_va_data
    }
    #[doc = "0x198 - IOMMU Virtual Address Configuration Register"]
    #[inline(always)]
    pub const fn iommu_va_config(&self) -> &IOMMU_VA_CONFIG {
        &self.iommu_va_config
    }
    #[doc = "0x200 - IOMMU PMU Enable Register"]
    #[inline(always)]
    pub const fn iommu_pmu_enable(&self) -> &IOMMU_PMU_ENABLE {
        &self.iommu_pmu_enable
    }
    #[doc = "0x210 - IOMMU PMU Clear Register"]
    #[inline(always)]
    pub const fn iommu_pmu_clr(&self) -> &IOMMU_PMU_CLR {
        &self.iommu_pmu_clr
    }
    #[doc = "0x230..0x254 - IOMMU PMU Access Low \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_access_low(&self, n: usize) -> &IOMMU_PMU_ACCESS_LOW {
        #[allow(clippy::no_effect)]
        [(); 9][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(560)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x234..0x258 - IOMMU PMU Access High \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_access_high(&self, n: usize) -> &IOMMU_PMU_ACCESS_HIGH {
        #[allow(clippy::no_effect)]
        [(); 9][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(564)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x238..0x25c - IOMMU PMU Hit Low \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_hit_low(&self, n: usize) -> &IOMMU_PMU_HIT_LOW {
        #[allow(clippy::no_effect)]
        [(); 9][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(568)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x23c..0x260 - IOMMU PMU Hit High \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_hit_high(&self, n: usize) -> &IOMMU_PMU_HIT_HIGH {
        #[allow(clippy::no_effect)]
        [(); 9][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(572)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x300..0x31c - IOMMU Total Latency Low \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_tl_low(&self, n: usize) -> &IOMMU_PMU_TL_LOW {
        #[allow(clippy::no_effect)]
        [(); 7][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(768)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x304..0x320 - IOMMU Total Latency High \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_tl_high(&self, n: usize) -> &IOMMU_PMU_TL_HIGH {
        #[allow(clippy::no_effect)]
        [(); 7][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(772)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x308..0x324 - IOMMU Max Latency \\[i\\] Register"]
    #[inline(always)]
    pub const fn iommu_pmu_ml(&self, n: usize) -> &IOMMU_PMU_ML {
        #[allow(clippy::no_effect)]
        [(); 7][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(776)
                .add(16 * n)
                .cast()
        }
    }
}
#[doc = "iommu_reset (rw) register accessor: IOMMU Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_reset`] module"]
pub type IOMMU_RESET = crate::Reg<iommu_reset::IOMMU_RESET_SPEC>;
#[doc = "IOMMU Reset Register"]
pub mod iommu_reset;
#[doc = "iommu_enable (rw) register accessor: IOMMU Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_enable`] module"]
pub type IOMMU_ENABLE = crate::Reg<iommu_enable::IOMMU_ENABLE_SPEC>;
#[doc = "IOMMU Enable Register"]
pub mod iommu_enable;
#[doc = "iommu_bypass (rw) register accessor: IOMMU Bypass Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_bypass`] module"]
pub type IOMMU_BYPASS = crate::Reg<iommu_bypass::IOMMU_BYPASS_SPEC>;
#[doc = "IOMMU Bypass Register"]
pub mod iommu_bypass;
#[doc = "iommu_auto_gating (rw) register accessor: IOMMU Auto Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_auto_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_auto_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_auto_gating`] module"]
pub type IOMMU_AUTO_GATING = crate::Reg<iommu_auto_gating::IOMMU_AUTO_GATING_SPEC>;
#[doc = "IOMMU Auto Gating Register"]
pub mod iommu_auto_gating;
#[doc = "iommu_wbuf_ctrl (rw) register accessor: IOMMU Write Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_wbuf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_wbuf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_wbuf_ctrl`] module"]
pub type IOMMU_WBUF_CTRL = crate::Reg<iommu_wbuf_ctrl::IOMMU_WBUF_CTRL_SPEC>;
#[doc = "IOMMU Write Buffer Control Register"]
pub mod iommu_wbuf_ctrl;
#[doc = "iommu_ooo_ctrl (rw) register accessor: IOMMU Out of Order Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_ooo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_ooo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_ooo_ctrl`] module"]
pub type IOMMU_OOO_CTRL = crate::Reg<iommu_ooo_ctrl::IOMMU_OOO_CTRL_SPEC>;
#[doc = "IOMMU Out of Order Control Register"]
pub mod iommu_ooo_ctrl;
#[doc = "iommu_4kb_bdy_prt_ctrl (rw) register accessor: IOMMU 4KB Boundary Protect Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_4kb_bdy_prt_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_4kb_bdy_prt_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_4kb_bdy_prt_ctrl`] module"]
pub type IOMMU_4KB_BDY_PRT_CTRL = crate::Reg<iommu_4kb_bdy_prt_ctrl::IOMMU_4KB_BDY_PRT_CTRL_SPEC>;
#[doc = "IOMMU 4KB Boundary Protect Control Register"]
pub mod iommu_4kb_bdy_prt_ctrl;
#[doc = "iommu_ttb (rw) register accessor: IOMMU Translation Table Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_ttb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_ttb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_ttb`] module"]
pub type IOMMU_TTB = crate::Reg<iommu_ttb::IOMMU_TTB_SPEC>;
#[doc = "IOMMU Translation Table Base Register"]
pub mod iommu_ttb;
#[doc = "iommu_tlb_enable (rw) register accessor: IOMMU TLB Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_enable`] module"]
pub type IOMMU_TLB_ENABLE = crate::Reg<iommu_tlb_enable::IOMMU_TLB_ENABLE_SPEC>;
#[doc = "IOMMU TLB Enable Register"]
pub mod iommu_tlb_enable;
#[doc = "iommu_tlb_prefetch (rw) register accessor: IOMMU TLB Prefetch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_prefetch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_prefetch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_prefetch`] module"]
pub type IOMMU_TLB_PREFETCH = crate::Reg<iommu_tlb_prefetch::IOMMU_TLB_PREFETCH_SPEC>;
#[doc = "IOMMU TLB Prefetch Register"]
pub mod iommu_tlb_prefetch;
#[doc = "iommu_tlb_flush_enable (rw) register accessor: IOMMU TLB Flush Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_flush_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_flush_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_flush_enable`] module"]
pub type IOMMU_TLB_FLUSH_ENABLE = crate::Reg<iommu_tlb_flush_enable::IOMMU_TLB_FLUSH_ENABLE_SPEC>;
#[doc = "IOMMU TLB Flush Enable Register"]
pub mod iommu_tlb_flush_enable;
#[doc = "iommu_tlb_ivld_mode_sel (rw) register accessor: IOMMU TLB Invalidation Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_mode_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_mode_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_ivld_mode_sel`] module"]
pub type IOMMU_TLB_IVLD_MODE_SEL =
    crate::Reg<iommu_tlb_ivld_mode_sel::IOMMU_TLB_IVLD_MODE_SEL_SPEC>;
#[doc = "IOMMU TLB Invalidation Mode Select Register"]
pub mod iommu_tlb_ivld_mode_sel;
#[doc = "iommu_tlb_ivld_sta_addr (rw) register accessor: IOMMU TLB Invalidation Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_sta_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_sta_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_ivld_sta_addr`] module"]
pub type IOMMU_TLB_IVLD_STA_ADDR =
    crate::Reg<iommu_tlb_ivld_sta_addr::IOMMU_TLB_IVLD_STA_ADDR_SPEC>;
#[doc = "IOMMU TLB Invalidation Start Address Register"]
pub mod iommu_tlb_ivld_sta_addr;
#[doc = "iommu_tlb_ivld_end_addr (rw) register accessor: IOMMU TLB Invalidation End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_ivld_end_addr`] module"]
pub type IOMMU_TLB_IVLD_END_ADDR =
    crate::Reg<iommu_tlb_ivld_end_addr::IOMMU_TLB_IVLD_END_ADDR_SPEC>;
#[doc = "IOMMU TLB Invalidation End Address Register"]
pub mod iommu_tlb_ivld_end_addr;
#[doc = "iommu_tlb_ivld_addr (rw) register accessor: IOMMU TLB Invalidation Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_ivld_addr`] module"]
pub type IOMMU_TLB_IVLD_ADDR = crate::Reg<iommu_tlb_ivld_addr::IOMMU_TLB_IVLD_ADDR_SPEC>;
#[doc = "IOMMU TLB Invalidation Address Register"]
pub mod iommu_tlb_ivld_addr;
#[doc = "iommu_tlb_ivld_addr_mask (rw) register accessor: IOMMU TLB Invalidation Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_addr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_addr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_ivld_addr_mask`] module"]
pub type IOMMU_TLB_IVLD_ADDR_MASK =
    crate::Reg<iommu_tlb_ivld_addr_mask::IOMMU_TLB_IVLD_ADDR_MASK_SPEC>;
#[doc = "IOMMU TLB Invalidation Address Mask Register"]
pub mod iommu_tlb_ivld_addr_mask;
#[doc = "iommu_tlb_ivld_enable (rw) register accessor: IOMMU TLB Invalidation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_tlb_ivld_enable`] module"]
pub type IOMMU_TLB_IVLD_ENABLE = crate::Reg<iommu_tlb_ivld_enable::IOMMU_TLB_IVLD_ENABLE_SPEC>;
#[doc = "IOMMU TLB Invalidation Enable Register"]
pub mod iommu_tlb_ivld_enable;
#[doc = "iommu_pc_ivld_mode_sel (rw) register accessor: IOMMU PC Invalidation Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_mode_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_mode_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pc_ivld_mode_sel`] module"]
pub type IOMMU_PC_IVLD_MODE_SEL = crate::Reg<iommu_pc_ivld_mode_sel::IOMMU_PC_IVLD_MODE_SEL_SPEC>;
#[doc = "IOMMU PC Invalidation Mode Select Register"]
pub mod iommu_pc_ivld_mode_sel;
#[doc = "iommu_pc_ivld_addr (rw) register accessor: IOMMU PC Invalidation Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pc_ivld_addr`] module"]
pub type IOMMU_PC_IVLD_ADDR = crate::Reg<iommu_pc_ivld_addr::IOMMU_PC_IVLD_ADDR_SPEC>;
#[doc = "IOMMU PC Invalidation Address Register"]
pub mod iommu_pc_ivld_addr;
#[doc = "iommu_pc_ivld_sta_addr (rw) register accessor: IOMMU PC Invalidation Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_sta_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_sta_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pc_ivld_sta_addr`] module"]
pub type IOMMU_PC_IVLD_STA_ADDR = crate::Reg<iommu_pc_ivld_sta_addr::IOMMU_PC_IVLD_STA_ADDR_SPEC>;
#[doc = "IOMMU PC Invalidation Start Address Register"]
pub mod iommu_pc_ivld_sta_addr;
#[doc = "iommu_pc_ivld_enable (rw) register accessor: IOMMU PC Invalidation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pc_ivld_enable`] module"]
pub type IOMMU_PC_IVLD_ENABLE = crate::Reg<iommu_pc_ivld_enable::IOMMU_PC_IVLD_ENABLE_SPEC>;
#[doc = "IOMMU PC Invalidation Enable Register"]
pub mod iommu_pc_ivld_enable;
#[doc = "iommu_pc_ivld_end_addr (rw) register accessor: IOMMU PC Invalidation End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pc_ivld_end_addr`] module"]
pub type IOMMU_PC_IVLD_END_ADDR = crate::Reg<iommu_pc_ivld_end_addr::IOMMU_PC_IVLD_END_ADDR_SPEC>;
#[doc = "IOMMU PC Invalidation End Address Register"]
pub mod iommu_pc_ivld_end_addr;
#[doc = "iommu_dm_aut_ctrl (rw) register accessor: IOMMU Domain Authority Control \\[i\\] Register\n\nSoftware can set 15 different permission control types in IOMMU_DM_AUT_CTRL0-7. A default access control type is DOMAIN0. The read/write operation of DOMAIN1-15 is unlimited by default.\n\nSoftware needs to set the index of the permission control domain corresponding to the page table item in the bit\\[7:4\\] of the Level2 page table, the default value is 0 (use domain 0), that is, the read/write operation is not controlled.\n\nSetting REG_ARD_OVWT can mask the Domain control defined by IOMMU_DM_AUT_CTRL0-7. All Level2 page table type are covered by the type of REG_ARD_OVWT. The read/write operation is permitted by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_dm_aut_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_dm_aut_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_dm_aut_ctrl`] module"]
pub type IOMMU_DM_AUT_CTRL = crate::Reg<iommu_dm_aut_ctrl::IOMMU_DM_AUT_CTRL_SPEC>;
#[doc = "IOMMU Domain Authority Control \\[i\\] Register\n\nSoftware can set 15 different permission control types in IOMMU_DM_AUT_CTRL0-7. A default access control type is DOMAIN0. The read/write operation of DOMAIN1-15 is unlimited by default.\n\nSoftware needs to set the index of the permission control domain corresponding to the page table item in the bit\\[7:4\\] of the Level2 page table, the default value is 0 (use domain 0), that is, the read/write operation is not controlled.\n\nSetting REG_ARD_OVWT can mask the Domain control defined by IOMMU_DM_AUT_CTRL0-7. All Level2 page table type are covered by the type of REG_ARD_OVWT. The read/write operation is permitted by default."]
pub mod iommu_dm_aut_ctrl;
#[doc = "iommu_dm_aut_ovwt (rw) register accessor: IOMMU Domain Authority Overwrite Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_dm_aut_ovwt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_dm_aut_ovwt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_dm_aut_ovwt`] module"]
pub type IOMMU_DM_AUT_OVWT = crate::Reg<iommu_dm_aut_ovwt::IOMMU_DM_AUT_OVWT_SPEC>;
#[doc = "IOMMU Domain Authority Overwrite Register"]
pub mod iommu_dm_aut_ovwt;
#[doc = "iommu_int_enable (rw) register accessor: IOMMU Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_int_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_enable`] module"]
pub type IOMMU_INT_ENABLE = crate::Reg<iommu_int_enable::IOMMU_INT_ENABLE_SPEC>;
#[doc = "IOMMU Interrupt Enable Register"]
pub mod iommu_int_enable;
#[doc = "iommu_int_clr (rw) register accessor: IOMMU Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_clr`] module"]
pub type IOMMU_INT_CLR = crate::Reg<iommu_int_clr::IOMMU_INT_CLR_SPEC>;
#[doc = "IOMMU Interrupt Clear Register"]
pub mod iommu_int_clr;
#[doc = "iommu_int_sta (rw) register accessor: IOMMU Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_int_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_sta`] module"]
pub type IOMMU_INT_STA = crate::Reg<iommu_int_sta::IOMMU_INT_STA_SPEC>;
#[doc = "IOMMU Interrupt Status Register"]
pub mod iommu_int_sta;
#[doc = "iommu_int_err_addr_tlb (r) register accessor: IOMMU Interrupt Error Address \\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_err_addr_tlb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_err_addr_tlb`] module"]
pub type IOMMU_INT_ERR_ADDR_TLB = crate::Reg<iommu_int_err_addr_tlb::IOMMU_INT_ERR_ADDR_TLB_SPEC>;
#[doc = "IOMMU Interrupt Error Address \\[i\\]"]
pub mod iommu_int_err_addr_tlb;
#[doc = "iommu_int_err_addr_l (r) register accessor: IOMMU Interrupt Error Address L\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_err_addr_l::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_err_addr_l`] module"]
pub type IOMMU_INT_ERR_ADDR_L = crate::Reg<iommu_int_err_addr_l::IOMMU_INT_ERR_ADDR_L_SPEC>;
#[doc = "IOMMU Interrupt Error Address L\\[i\\]"]
pub mod iommu_int_err_addr_l;
#[doc = "iommu_int_err_data_tlb (r) register accessor: IOMMU Interrupt Error Data \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_err_data_tlb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_err_data_tlb`] module"]
pub type IOMMU_INT_ERR_DATA_TLB = crate::Reg<iommu_int_err_data_tlb::IOMMU_INT_ERR_DATA_TLB_SPEC>;
#[doc = "IOMMU Interrupt Error Data \\[i\\] Register"]
pub mod iommu_int_err_data_tlb;
#[doc = "iommu_int_err_data_l (r) register accessor: IOMMU Interrupt Error Data L\\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_err_data_l::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_int_err_data_l`] module"]
pub type IOMMU_INT_ERR_DATA_L = crate::Reg<iommu_int_err_data_l::IOMMU_INT_ERR_DATA_L_SPEC>;
#[doc = "IOMMU Interrupt Error Data L\\[i\\] Register"]
pub mod iommu_int_err_data_l;
#[doc = "iommu_lpg_int (r) register accessor: IOMMU L\\[i\\] Page Table Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_lpg_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_lpg_int`] module"]
pub type IOMMU_LPG_INT = crate::Reg<iommu_lpg_int::IOMMU_LPG_INT_SPEC>;
#[doc = "IOMMU L\\[i\\] Page Table Interrupt Register"]
pub mod iommu_lpg_int;
#[doc = "iommu_va (rw) register accessor: IOMMU Virtual Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_va::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_va::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_va`] module"]
pub type IOMMU_VA = crate::Reg<iommu_va::IOMMU_VA_SPEC>;
#[doc = "IOMMU Virtual Address Register"]
pub mod iommu_va;
#[doc = "iommu_va_data (rw) register accessor: IOMMU Virtual Address Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_va_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_va_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_va_data`] module"]
pub type IOMMU_VA_DATA = crate::Reg<iommu_va_data::IOMMU_VA_DATA_SPEC>;
#[doc = "IOMMU Virtual Address Data Register"]
pub mod iommu_va_data;
#[doc = "iommu_va_config (rw) register accessor: IOMMU Virtual Address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_va_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_va_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_va_config`] module"]
pub type IOMMU_VA_CONFIG = crate::Reg<iommu_va_config::IOMMU_VA_CONFIG_SPEC>;
#[doc = "IOMMU Virtual Address Configuration Register"]
pub mod iommu_va_config;
#[doc = "iommu_pmu_enable (rw) register accessor: IOMMU PMU Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_enable`] module"]
pub type IOMMU_PMU_ENABLE = crate::Reg<iommu_pmu_enable::IOMMU_PMU_ENABLE_SPEC>;
#[doc = "IOMMU PMU Enable Register"]
pub mod iommu_pmu_enable;
#[doc = "iommu_pmu_clr (rw) register accessor: IOMMU PMU Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_clr`] module"]
pub type IOMMU_PMU_CLR = crate::Reg<iommu_pmu_clr::IOMMU_PMU_CLR_SPEC>;
#[doc = "IOMMU PMU Clear Register"]
pub mod iommu_pmu_clr;
#[doc = "iommu_pmu_access_low (rw) register accessor: IOMMU PMU Access Low \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_access_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_access_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_access_low`] module"]
pub type IOMMU_PMU_ACCESS_LOW = crate::Reg<iommu_pmu_access_low::IOMMU_PMU_ACCESS_LOW_SPEC>;
#[doc = "IOMMU PMU Access Low \\[i\\] Register"]
pub mod iommu_pmu_access_low;
#[doc = "iommu_pmu_access_high (rw) register accessor: IOMMU PMU Access High \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_access_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_access_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_access_high`] module"]
pub type IOMMU_PMU_ACCESS_HIGH = crate::Reg<iommu_pmu_access_high::IOMMU_PMU_ACCESS_HIGH_SPEC>;
#[doc = "IOMMU PMU Access High \\[i\\] Register"]
pub mod iommu_pmu_access_high;
#[doc = "iommu_pmu_hit_low (rw) register accessor: IOMMU PMU Hit Low \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_hit_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_hit_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_hit_low`] module"]
pub type IOMMU_PMU_HIT_LOW = crate::Reg<iommu_pmu_hit_low::IOMMU_PMU_HIT_LOW_SPEC>;
#[doc = "IOMMU PMU Hit Low \\[i\\] Register"]
pub mod iommu_pmu_hit_low;
#[doc = "iommu_pmu_hit_high (rw) register accessor: IOMMU PMU Hit High \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_hit_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_hit_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_hit_high`] module"]
pub type IOMMU_PMU_HIT_HIGH = crate::Reg<iommu_pmu_hit_high::IOMMU_PMU_HIT_HIGH_SPEC>;
#[doc = "IOMMU PMU Hit High \\[i\\] Register"]
pub mod iommu_pmu_hit_high;
#[doc = "iommu_pmu_tl_low (rw) register accessor: IOMMU Total Latency Low \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_tl_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_tl_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_tl_low`] module"]
pub type IOMMU_PMU_TL_LOW = crate::Reg<iommu_pmu_tl_low::IOMMU_PMU_TL_LOW_SPEC>;
#[doc = "IOMMU Total Latency Low \\[i\\] Register"]
pub mod iommu_pmu_tl_low;
#[doc = "iommu_pmu_tl_high (rw) register accessor: IOMMU Total Latency High \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_tl_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_tl_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_tl_high`] module"]
pub type IOMMU_PMU_TL_HIGH = crate::Reg<iommu_pmu_tl_high::IOMMU_PMU_TL_HIGH_SPEC>;
#[doc = "IOMMU Total Latency High \\[i\\] Register"]
pub mod iommu_pmu_tl_high;
#[doc = "iommu_pmu_ml (rw) register accessor: IOMMU Max Latency \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_ml::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_ml::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_pmu_ml`] module"]
pub type IOMMU_PMU_ML = crate::Reg<iommu_pmu_ml::IOMMU_PMU_ML_SPEC>;
#[doc = "IOMMU Max Latency \\[i\\] Register"]
pub mod iommu_pmu_ml;
