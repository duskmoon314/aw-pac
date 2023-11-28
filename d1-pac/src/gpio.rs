#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    pb_cfg0: PB_CFG0,
    pb_cfg1: PB_CFG1,
    _reserved2: [u8; 0x08],
    pb_dat: PB_DAT,
    pb_drv0: PB_DRV0,
    pb_drv1: PB_DRV1,
    _reserved5: [u8; 0x08],
    pb_pull0: PB_PULL0,
    _reserved6: [u8; 0x08],
    pc_cfg0: PC_CFG0,
    _reserved7: [u8; 0x0c],
    pc_dat: PC_DAT,
    pc_drv0: PC_DRV0,
    _reserved9: [u8; 0x0c],
    pc_pull0: PC_PULL0,
    _reserved10: [u8; 0x08],
    pd_cfg0: PD_CFG0,
    pd_cfg1: PD_CFG1,
    pd_cfg2: PD_CFG2,
    _reserved13: [u8; 0x04],
    pd_dat: PD_DAT,
    pd_drv0: PD_DRV0,
    pd_drv1: PD_DRV1,
    pd_drv2: PD_DRV2,
    _reserved17: [u8; 0x04],
    pd_pull0: PD_PULL0,
    pd_pull1: PD_PULL1,
    _reserved19: [u8; 0x04],
    pe_cfg0: PE_CFG0,
    pe_cfg1: PE_CFG1,
    pe_cfg2: PE_CFG2,
    _reserved22: [u8; 0x04],
    pe_dat: PE_DAT,
    pe_drv0: PE_DRV0,
    pe_drv1: PE_DRV1,
    pe_drv2: PE_DRV2,
    _reserved26: [u8; 0x04],
    pe_pull0: PE_PULL0,
    pe_pull1: PE_PULL1,
    _reserved28: [u8; 0x04],
    pf_cfg0: PF_CFG0,
    _reserved29: [u8; 0x0c],
    pf_dat: PF_DAT,
    pf_drv0: PF_DRV0,
    _reserved31: [u8; 0x0c],
    pf_pull0: PF_PULL0,
    _reserved32: [u8; 0x08],
    pg_cfg0: PG_CFG0,
    pg_cfg1: PG_CFG1,
    pg_cfg2: PG_CFG2,
    _reserved35: [u8; 0x04],
    pg_dat: PG_DAT,
    pg_drv0: PG_DRV0,
    pg_drv1: PG_DRV1,
    pg_drv2: PG_DRV2,
    _reserved39: [u8; 0x04],
    pg_pull0: PG_PULL0,
    pg_pull1: PG_PULL1,
    _reserved41: [u8; 0xd4],
    pb_eint_cfg0: PB_EINT_CFG0,
    pb_eint_cfg1: PB_EINT_CFG1,
    _reserved43: [u8; 0x08],
    pb_eint_ctl: PB_EINT_CTL,
    pb_eint_status: PB_EINT_STATUS,
    pb_eint_deb: PB_EINT_DEB,
    _reserved46: [u8; 0x04],
    pc_eint_cfg0: PC_EINT_CFG0,
    _reserved47: [u8; 0x0c],
    pc_eint_ctl: PC_EINT_CTL,
    pc_eint_status: PC_EINT_STATUS,
    pc_eint_deb: PC_EINT_DEB,
    _reserved50: [u8; 0x04],
    pd_eint_cfg0: PD_EINT_CFG0,
    pd_eint_cfg1: PD_EINT_CFG1,
    pd_eint_cfg2: PD_EINT_CFG2,
    _reserved53: [u8; 0x04],
    pd_eint_ctl: PD_EINT_CTL,
    pd_eint_status: PD_EINT_STATUS,
    pd_eint_deb: PD_EINT_DEB,
    _reserved56: [u8; 0x04],
    pe_eint_cfg0: PE_EINT_CFG0,
    pe_eint_cfg1: PE_EINT_CFG1,
    pe_eint_cfg2: PE_EINT_CFG2,
    _reserved59: [u8; 0x04],
    pe_eint_ctl: PE_EINT_CTL,
    pe_eint_status: PE_EINT_STATUS,
    pe_eint_deb: PE_EINT_DEB,
    _reserved62: [u8; 0x04],
    pf_eint_cfg0: PF_EINT_CFG0,
    _reserved63: [u8; 0x0c],
    pf_eint_ctl: PF_EINT_CTL,
    pf_eint_status: PF_EINT_STATUS,
    pf_eint_deb: PF_EINT_DEB,
    _reserved66: [u8; 0x04],
    pg_eint_cfg0: PG_EINT_CFG0,
    pg_eint_cfg1: PG_EINT_CFG1,
    pg_eint_cfg2: PG_EINT_CFG2,
    _reserved69: [u8; 0x04],
    pg_eint_ctl: PG_EINT_CTL,
    pg_eint_status: PG_EINT_STATUS,
    pg_eint_deb: PG_EINT_DEB,
    _reserved72: [u8; 0x64],
    pio_pow_mod_sel: PIO_POW_MOD_SEL,
    pio_pow_ms_ctl: PIO_POW_MS_CTL,
    pio_pow_val: PIO_POW_VAL,
    _reserved75: [u8; 0x04],
    pio_pow_vol_sel_ctl: PIO_POW_VOL_SEL_CTL,
}
impl RegisterBlock {
    #[doc = "0x30 - PB Configure Register 0"]
    #[inline(always)]
    pub const fn pb_cfg0(&self) -> &PB_CFG0 {
        &self.pb_cfg0
    }
    #[doc = "0x34 - PB Configure Register 1"]
    #[inline(always)]
    pub const fn pb_cfg1(&self) -> &PB_CFG1 {
        &self.pb_cfg1
    }
    #[doc = "0x40 - PB Data Register"]
    #[inline(always)]
    pub const fn pb_dat(&self) -> &PB_DAT {
        &self.pb_dat
    }
    #[doc = "0x44 - PB Multi_Driving Register 0"]
    #[inline(always)]
    pub const fn pb_drv0(&self) -> &PB_DRV0 {
        &self.pb_drv0
    }
    #[doc = "0x48 - PB Multi_Driving Register 1"]
    #[inline(always)]
    pub const fn pb_drv1(&self) -> &PB_DRV1 {
        &self.pb_drv1
    }
    #[doc = "0x54 - PB Pull Register 0"]
    #[inline(always)]
    pub const fn pb_pull0(&self) -> &PB_PULL0 {
        &self.pb_pull0
    }
    #[doc = "0x60 - PC Configure Register 0"]
    #[inline(always)]
    pub const fn pc_cfg0(&self) -> &PC_CFG0 {
        &self.pc_cfg0
    }
    #[doc = "0x70 - PC Data Register"]
    #[inline(always)]
    pub const fn pc_dat(&self) -> &PC_DAT {
        &self.pc_dat
    }
    #[doc = "0x74 - PC Multi_Driving Register 0"]
    #[inline(always)]
    pub const fn pc_drv0(&self) -> &PC_DRV0 {
        &self.pc_drv0
    }
    #[doc = "0x84 - PC Pull Register 0"]
    #[inline(always)]
    pub const fn pc_pull0(&self) -> &PC_PULL0 {
        &self.pc_pull0
    }
    #[doc = "0x90 - PD Configure Register 0"]
    #[inline(always)]
    pub const fn pd_cfg0(&self) -> &PD_CFG0 {
        &self.pd_cfg0
    }
    #[doc = "0x94 - PD Configure Register 1"]
    #[inline(always)]
    pub const fn pd_cfg1(&self) -> &PD_CFG1 {
        &self.pd_cfg1
    }
    #[doc = "0x98 - PD Configure Register 2"]
    #[inline(always)]
    pub const fn pd_cfg2(&self) -> &PD_CFG2 {
        &self.pd_cfg2
    }
    #[doc = "0xa0 - PD Data Register"]
    #[inline(always)]
    pub const fn pd_dat(&self) -> &PD_DAT {
        &self.pd_dat
    }
    #[doc = "0xa4 - PD Multi_Driving Register 0"]
    #[inline(always)]
    pub const fn pd_drv0(&self) -> &PD_DRV0 {
        &self.pd_drv0
    }
    #[doc = "0xa8 - PD Multi_Driving Register 1"]
    #[inline(always)]
    pub const fn pd_drv1(&self) -> &PD_DRV1 {
        &self.pd_drv1
    }
    #[doc = "0xac - PD Multi_Driving Register 2"]
    #[inline(always)]
    pub const fn pd_drv2(&self) -> &PD_DRV2 {
        &self.pd_drv2
    }
    #[doc = "0xb4 - PD Pull Register 0"]
    #[inline(always)]
    pub const fn pd_pull0(&self) -> &PD_PULL0 {
        &self.pd_pull0
    }
    #[doc = "0xb8 - PD Pull Register 1"]
    #[inline(always)]
    pub const fn pd_pull1(&self) -> &PD_PULL1 {
        &self.pd_pull1
    }
    #[doc = "0xc0 - PE Configure Register 0"]
    #[inline(always)]
    pub const fn pe_cfg0(&self) -> &PE_CFG0 {
        &self.pe_cfg0
    }
    #[doc = "0xc4 - PE Configure Register 1"]
    #[inline(always)]
    pub const fn pe_cfg1(&self) -> &PE_CFG1 {
        &self.pe_cfg1
    }
    #[doc = "0xc8 - PE Configure Register 2"]
    #[inline(always)]
    pub const fn pe_cfg2(&self) -> &PE_CFG2 {
        &self.pe_cfg2
    }
    #[doc = "0xd0 - PE Data Register"]
    #[inline(always)]
    pub const fn pe_dat(&self) -> &PE_DAT {
        &self.pe_dat
    }
    #[doc = "0xd4 - PE Multi_Driving Register 0"]
    #[inline(always)]
    pub const fn pe_drv0(&self) -> &PE_DRV0 {
        &self.pe_drv0
    }
    #[doc = "0xd8 - PE Multi_Driving Register 1"]
    #[inline(always)]
    pub const fn pe_drv1(&self) -> &PE_DRV1 {
        &self.pe_drv1
    }
    #[doc = "0xdc - PE Multi_Driving Register 2"]
    #[inline(always)]
    pub const fn pe_drv2(&self) -> &PE_DRV2 {
        &self.pe_drv2
    }
    #[doc = "0xe4 - PE Pull Register 0"]
    #[inline(always)]
    pub const fn pe_pull0(&self) -> &PE_PULL0 {
        &self.pe_pull0
    }
    #[doc = "0xe8 - PE Pull Register 1"]
    #[inline(always)]
    pub const fn pe_pull1(&self) -> &PE_PULL1 {
        &self.pe_pull1
    }
    #[doc = "0xf0 - PF Configure Register 0"]
    #[inline(always)]
    pub const fn pf_cfg0(&self) -> &PF_CFG0 {
        &self.pf_cfg0
    }
    #[doc = "0x100 - PF Data Register"]
    #[inline(always)]
    pub const fn pf_dat(&self) -> &PF_DAT {
        &self.pf_dat
    }
    #[doc = "0x104 - PF Multi_Driving Register 0"]
    #[inline(always)]
    pub const fn pf_drv0(&self) -> &PF_DRV0 {
        &self.pf_drv0
    }
    #[doc = "0x114 - PF Pull Register 0"]
    #[inline(always)]
    pub const fn pf_pull0(&self) -> &PF_PULL0 {
        &self.pf_pull0
    }
    #[doc = "0x120 - PG Configure Register 0"]
    #[inline(always)]
    pub const fn pg_cfg0(&self) -> &PG_CFG0 {
        &self.pg_cfg0
    }
    #[doc = "0x124 - PG Configure Register 1"]
    #[inline(always)]
    pub const fn pg_cfg1(&self) -> &PG_CFG1 {
        &self.pg_cfg1
    }
    #[doc = "0x128 - PG Configure Register 2"]
    #[inline(always)]
    pub const fn pg_cfg2(&self) -> &PG_CFG2 {
        &self.pg_cfg2
    }
    #[doc = "0x130 - PG Data Register"]
    #[inline(always)]
    pub const fn pg_dat(&self) -> &PG_DAT {
        &self.pg_dat
    }
    #[doc = "0x134 - PG Multi_Driving Register 0"]
    #[inline(always)]
    pub const fn pg_drv0(&self) -> &PG_DRV0 {
        &self.pg_drv0
    }
    #[doc = "0x138 - PG Multi_Driving Register 1"]
    #[inline(always)]
    pub const fn pg_drv1(&self) -> &PG_DRV1 {
        &self.pg_drv1
    }
    #[doc = "0x13c - PG Multi_Driving Register 2"]
    #[inline(always)]
    pub const fn pg_drv2(&self) -> &PG_DRV2 {
        &self.pg_drv2
    }
    #[doc = "0x144 - PG Pull Register 0"]
    #[inline(always)]
    pub const fn pg_pull0(&self) -> &PG_PULL0 {
        &self.pg_pull0
    }
    #[doc = "0x148 - PG Pull Register 1"]
    #[inline(always)]
    pub const fn pg_pull1(&self) -> &PG_PULL1 {
        &self.pg_pull1
    }
    #[doc = "0x220 - PB External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pb_eint_cfg0(&self) -> &PB_EINT_CFG0 {
        &self.pb_eint_cfg0
    }
    #[doc = "0x224 - PB External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pb_eint_cfg1(&self) -> &PB_EINT_CFG1 {
        &self.pb_eint_cfg1
    }
    #[doc = "0x230 - PB External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pb_eint_ctl(&self) -> &PB_EINT_CTL {
        &self.pb_eint_ctl
    }
    #[doc = "0x234 - PB External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pb_eint_status(&self) -> &PB_EINT_STATUS {
        &self.pb_eint_status
    }
    #[doc = "0x238 - PB External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pb_eint_deb(&self) -> &PB_EINT_DEB {
        &self.pb_eint_deb
    }
    #[doc = "0x240 - PC External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pc_eint_cfg0(&self) -> &PC_EINT_CFG0 {
        &self.pc_eint_cfg0
    }
    #[doc = "0x250 - PC External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pc_eint_ctl(&self) -> &PC_EINT_CTL {
        &self.pc_eint_ctl
    }
    #[doc = "0x254 - PC External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pc_eint_status(&self) -> &PC_EINT_STATUS {
        &self.pc_eint_status
    }
    #[doc = "0x258 - PC External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pc_eint_deb(&self) -> &PC_EINT_DEB {
        &self.pc_eint_deb
    }
    #[doc = "0x260 - PD External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pd_eint_cfg0(&self) -> &PD_EINT_CFG0 {
        &self.pd_eint_cfg0
    }
    #[doc = "0x264 - PD External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pd_eint_cfg1(&self) -> &PD_EINT_CFG1 {
        &self.pd_eint_cfg1
    }
    #[doc = "0x268 - PD External Interrupt Configure Register 2"]
    #[inline(always)]
    pub const fn pd_eint_cfg2(&self) -> &PD_EINT_CFG2 {
        &self.pd_eint_cfg2
    }
    #[doc = "0x270 - PD External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pd_eint_ctl(&self) -> &PD_EINT_CTL {
        &self.pd_eint_ctl
    }
    #[doc = "0x274 - PD External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pd_eint_status(&self) -> &PD_EINT_STATUS {
        &self.pd_eint_status
    }
    #[doc = "0x278 - PD External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pd_eint_deb(&self) -> &PD_EINT_DEB {
        &self.pd_eint_deb
    }
    #[doc = "0x280 - PE External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pe_eint_cfg0(&self) -> &PE_EINT_CFG0 {
        &self.pe_eint_cfg0
    }
    #[doc = "0x284 - PE External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pe_eint_cfg1(&self) -> &PE_EINT_CFG1 {
        &self.pe_eint_cfg1
    }
    #[doc = "0x288 - PE External Interrupt Configure Register 2"]
    #[inline(always)]
    pub const fn pe_eint_cfg2(&self) -> &PE_EINT_CFG2 {
        &self.pe_eint_cfg2
    }
    #[doc = "0x290 - PE External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pe_eint_ctl(&self) -> &PE_EINT_CTL {
        &self.pe_eint_ctl
    }
    #[doc = "0x294 - PE External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pe_eint_status(&self) -> &PE_EINT_STATUS {
        &self.pe_eint_status
    }
    #[doc = "0x298 - PE External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pe_eint_deb(&self) -> &PE_EINT_DEB {
        &self.pe_eint_deb
    }
    #[doc = "0x2a0 - PF External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pf_eint_cfg0(&self) -> &PF_EINT_CFG0 {
        &self.pf_eint_cfg0
    }
    #[doc = "0x2b0 - PF External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pf_eint_ctl(&self) -> &PF_EINT_CTL {
        &self.pf_eint_ctl
    }
    #[doc = "0x2b4 - PF External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pf_eint_status(&self) -> &PF_EINT_STATUS {
        &self.pf_eint_status
    }
    #[doc = "0x2b8 - PF External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pf_eint_deb(&self) -> &PF_EINT_DEB {
        &self.pf_eint_deb
    }
    #[doc = "0x2c0 - PG External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pg_eint_cfg0(&self) -> &PG_EINT_CFG0 {
        &self.pg_eint_cfg0
    }
    #[doc = "0x2c4 - PG External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pg_eint_cfg1(&self) -> &PG_EINT_CFG1 {
        &self.pg_eint_cfg1
    }
    #[doc = "0x2c8 - PG External Interrupt Configure Register 2"]
    #[inline(always)]
    pub const fn pg_eint_cfg2(&self) -> &PG_EINT_CFG2 {
        &self.pg_eint_cfg2
    }
    #[doc = "0x2d0 - PG External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pg_eint_ctl(&self) -> &PG_EINT_CTL {
        &self.pg_eint_ctl
    }
    #[doc = "0x2d4 - PG External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pg_eint_status(&self) -> &PG_EINT_STATUS {
        &self.pg_eint_status
    }
    #[doc = "0x2d8 - PG External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pg_eint_deb(&self) -> &PG_EINT_DEB {
        &self.pg_eint_deb
    }
    #[doc = "0x340 - PIO Group Withstand Voltage Mode Select Register"]
    #[inline(always)]
    pub const fn pio_pow_mod_sel(&self) -> &PIO_POW_MOD_SEL {
        &self.pio_pow_mod_sel
    }
    #[doc = "0x344 - PIO Group Withstand Voltage Mode Select Control Register"]
    #[inline(always)]
    pub const fn pio_pow_ms_ctl(&self) -> &PIO_POW_MS_CTL {
        &self.pio_pow_ms_ctl
    }
    #[doc = "0x348 - PIO Group Power Value Register"]
    #[inline(always)]
    pub const fn pio_pow_val(&self) -> &PIO_POW_VAL {
        &self.pio_pow_val
    }
    #[doc = "0x350 - PIO Group Power Voltage Select Control Register"]
    #[inline(always)]
    pub const fn pio_pow_vol_sel_ctl(&self) -> &PIO_POW_VOL_SEL_CTL {
        &self.pio_pow_vol_sel_ctl
    }
}
#[doc = "pb_cfg0 (rw) register accessor: PB Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_cfg0`] module"]
pub type PB_CFG0 = crate::Reg<pb_cfg0::PB_CFG0_SPEC>;
#[doc = "PB Configure Register 0"]
pub mod pb_cfg0;
#[doc = "pb_cfg1 (rw) register accessor: PB Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_cfg1`] module"]
pub type PB_CFG1 = crate::Reg<pb_cfg1::PB_CFG1_SPEC>;
#[doc = "PB Configure Register 1"]
pub mod pb_cfg1;
#[doc = "pb_dat (rw) register accessor: PB Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_dat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_dat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_dat`] module"]
pub type PB_DAT = crate::Reg<pb_dat::PB_DAT_SPEC>;
#[doc = "PB Data Register"]
pub mod pb_dat;
#[doc = "pb_drv0 (rw) register accessor: PB Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_drv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_drv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_drv0`] module"]
pub type PB_DRV0 = crate::Reg<pb_drv0::PB_DRV0_SPEC>;
#[doc = "PB Multi_Driving Register 0"]
pub mod pb_drv0;
#[doc = "pb_drv1 (rw) register accessor: PB Multi_Driving Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_drv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_drv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_drv1`] module"]
pub type PB_DRV1 = crate::Reg<pb_drv1::PB_DRV1_SPEC>;
#[doc = "PB Multi_Driving Register 1"]
pub mod pb_drv1;
#[doc = "pb_pull0 (rw) register accessor: PB Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_pull0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_pull0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pull0`] module"]
pub type PB_PULL0 = crate::Reg<pb_pull0::PB_PULL0_SPEC>;
#[doc = "PB Pull Register 0"]
pub mod pb_pull0;
#[doc = "pc_cfg0 (rw) register accessor: PC Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_cfg0`] module"]
pub type PC_CFG0 = crate::Reg<pc_cfg0::PC_CFG0_SPEC>;
#[doc = "PC Configure Register 0"]
pub mod pc_cfg0;
#[doc = "pc_dat (rw) register accessor: PC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_dat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_dat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_dat`] module"]
pub type PC_DAT = crate::Reg<pc_dat::PC_DAT_SPEC>;
#[doc = "PC Data Register"]
pub mod pc_dat;
#[doc = "pc_drv0 (rw) register accessor: PC Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_drv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_drv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_drv0`] module"]
pub type PC_DRV0 = crate::Reg<pc_drv0::PC_DRV0_SPEC>;
#[doc = "PC Multi_Driving Register 0"]
pub mod pc_drv0;
#[doc = "pc_pull0 (rw) register accessor: PC Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_pull0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_pull0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pull0`] module"]
pub type PC_PULL0 = crate::Reg<pc_pull0::PC_PULL0_SPEC>;
#[doc = "PC Pull Register 0"]
pub mod pc_pull0;
#[doc = "pd_cfg0 (rw) register accessor: PD Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg0`] module"]
pub type PD_CFG0 = crate::Reg<pd_cfg0::PD_CFG0_SPEC>;
#[doc = "PD Configure Register 0"]
pub mod pd_cfg0;
#[doc = "pd_cfg1 (rw) register accessor: PD Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg1`] module"]
pub type PD_CFG1 = crate::Reg<pd_cfg1::PD_CFG1_SPEC>;
#[doc = "PD Configure Register 1"]
pub mod pd_cfg1;
#[doc = "pd_cfg2 (rw) register accessor: PD Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg2`] module"]
pub type PD_CFG2 = crate::Reg<pd_cfg2::PD_CFG2_SPEC>;
#[doc = "PD Configure Register 2"]
pub mod pd_cfg2;
#[doc = "pd_dat (rw) register accessor: PD Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_dat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_dat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_dat`] module"]
pub type PD_DAT = crate::Reg<pd_dat::PD_DAT_SPEC>;
#[doc = "PD Data Register"]
pub mod pd_dat;
#[doc = "pd_drv0 (rw) register accessor: PD Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_drv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_drv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_drv0`] module"]
pub type PD_DRV0 = crate::Reg<pd_drv0::PD_DRV0_SPEC>;
#[doc = "PD Multi_Driving Register 0"]
pub mod pd_drv0;
#[doc = "pd_drv1 (rw) register accessor: PD Multi_Driving Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_drv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_drv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_drv1`] module"]
pub type PD_DRV1 = crate::Reg<pd_drv1::PD_DRV1_SPEC>;
#[doc = "PD Multi_Driving Register 1"]
pub mod pd_drv1;
#[doc = "pd_drv2 (rw) register accessor: PD Multi_Driving Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_drv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_drv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_drv2`] module"]
pub type PD_DRV2 = crate::Reg<pd_drv2::PD_DRV2_SPEC>;
#[doc = "PD Multi_Driving Register 2"]
pub mod pd_drv2;
#[doc = "pd_pull0 (rw) register accessor: PD Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_pull0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_pull0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pull0`] module"]
pub type PD_PULL0 = crate::Reg<pd_pull0::PD_PULL0_SPEC>;
#[doc = "PD Pull Register 0"]
pub mod pd_pull0;
#[doc = "pd_pull1 (rw) register accessor: PD Pull Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_pull1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_pull1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pull1`] module"]
pub type PD_PULL1 = crate::Reg<pd_pull1::PD_PULL1_SPEC>;
#[doc = "PD Pull Register 1"]
pub mod pd_pull1;
#[doc = "pe_cfg0 (rw) register accessor: PE Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg0`] module"]
pub type PE_CFG0 = crate::Reg<pe_cfg0::PE_CFG0_SPEC>;
#[doc = "PE Configure Register 0"]
pub mod pe_cfg0;
#[doc = "pe_cfg1 (rw) register accessor: PE Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg1`] module"]
pub type PE_CFG1 = crate::Reg<pe_cfg1::PE_CFG1_SPEC>;
#[doc = "PE Configure Register 1"]
pub mod pe_cfg1;
#[doc = "pe_cfg2 (rw) register accessor: PE Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg2`] module"]
pub type PE_CFG2 = crate::Reg<pe_cfg2::PE_CFG2_SPEC>;
#[doc = "PE Configure Register 2"]
pub mod pe_cfg2;
#[doc = "pe_dat (rw) register accessor: PE Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_dat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_dat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_dat`] module"]
pub type PE_DAT = crate::Reg<pe_dat::PE_DAT_SPEC>;
#[doc = "PE Data Register"]
pub mod pe_dat;
#[doc = "pe_drv0 (rw) register accessor: PE Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_drv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_drv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_drv0`] module"]
pub type PE_DRV0 = crate::Reg<pe_drv0::PE_DRV0_SPEC>;
#[doc = "PE Multi_Driving Register 0"]
pub mod pe_drv0;
#[doc = "pe_drv1 (rw) register accessor: PE Multi_Driving Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_drv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_drv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_drv1`] module"]
pub type PE_DRV1 = crate::Reg<pe_drv1::PE_DRV1_SPEC>;
#[doc = "PE Multi_Driving Register 1"]
pub mod pe_drv1;
#[doc = "pe_drv2 (rw) register accessor: PE Multi_Driving Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_drv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_drv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_drv2`] module"]
pub type PE_DRV2 = crate::Reg<pe_drv2::PE_DRV2_SPEC>;
#[doc = "PE Multi_Driving Register 2"]
pub mod pe_drv2;
#[doc = "pe_pull0 (rw) register accessor: PE Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_pull0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_pull0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pull0`] module"]
pub type PE_PULL0 = crate::Reg<pe_pull0::PE_PULL0_SPEC>;
#[doc = "PE Pull Register 0"]
pub mod pe_pull0;
#[doc = "pe_pull1 (rw) register accessor: PE Pull Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_pull1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_pull1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pull1`] module"]
pub type PE_PULL1 = crate::Reg<pe_pull1::PE_PULL1_SPEC>;
#[doc = "PE Pull Register 1"]
pub mod pe_pull1;
#[doc = "pf_cfg0 (rw) register accessor: PF Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_cfg0`] module"]
pub type PF_CFG0 = crate::Reg<pf_cfg0::PF_CFG0_SPEC>;
#[doc = "PF Configure Register 0"]
pub mod pf_cfg0;
#[doc = "pf_dat (rw) register accessor: PF Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_dat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_dat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_dat`] module"]
pub type PF_DAT = crate::Reg<pf_dat::PF_DAT_SPEC>;
#[doc = "PF Data Register"]
pub mod pf_dat;
#[doc = "pf_drv0 (rw) register accessor: PF Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_drv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_drv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_drv0`] module"]
pub type PF_DRV0 = crate::Reg<pf_drv0::PF_DRV0_SPEC>;
#[doc = "PF Multi_Driving Register 0"]
pub mod pf_drv0;
#[doc = "pf_pull0 (rw) register accessor: PF Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_pull0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_pull0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pull0`] module"]
pub type PF_PULL0 = crate::Reg<pf_pull0::PF_PULL0_SPEC>;
#[doc = "PF Pull Register 0"]
pub mod pf_pull0;
#[doc = "pg_cfg0 (rw) register accessor: PG Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_cfg0`] module"]
pub type PG_CFG0 = crate::Reg<pg_cfg0::PG_CFG0_SPEC>;
#[doc = "PG Configure Register 0"]
pub mod pg_cfg0;
#[doc = "pg_cfg1 (rw) register accessor: PG Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_cfg1`] module"]
pub type PG_CFG1 = crate::Reg<pg_cfg1::PG_CFG1_SPEC>;
#[doc = "PG Configure Register 1"]
pub mod pg_cfg1;
#[doc = "pg_cfg2 (rw) register accessor: PG Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_cfg2`] module"]
pub type PG_CFG2 = crate::Reg<pg_cfg2::PG_CFG2_SPEC>;
#[doc = "PG Configure Register 2"]
pub mod pg_cfg2;
#[doc = "pg_dat (rw) register accessor: PG Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_dat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_dat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_dat`] module"]
pub type PG_DAT = crate::Reg<pg_dat::PG_DAT_SPEC>;
#[doc = "PG Data Register"]
pub mod pg_dat;
#[doc = "pg_drv0 (rw) register accessor: PG Multi_Driving Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_drv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_drv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_drv0`] module"]
pub type PG_DRV0 = crate::Reg<pg_drv0::PG_DRV0_SPEC>;
#[doc = "PG Multi_Driving Register 0"]
pub mod pg_drv0;
#[doc = "pg_drv1 (rw) register accessor: PG Multi_Driving Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_drv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_drv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_drv1`] module"]
pub type PG_DRV1 = crate::Reg<pg_drv1::PG_DRV1_SPEC>;
#[doc = "PG Multi_Driving Register 1"]
pub mod pg_drv1;
#[doc = "pg_drv2 (rw) register accessor: PG Multi_Driving Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_drv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_drv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_drv2`] module"]
pub type PG_DRV2 = crate::Reg<pg_drv2::PG_DRV2_SPEC>;
#[doc = "PG Multi_Driving Register 2"]
pub mod pg_drv2;
#[doc = "pg_pull0 (rw) register accessor: PG Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_pull0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_pull0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_pull0`] module"]
pub type PG_PULL0 = crate::Reg<pg_pull0::PG_PULL0_SPEC>;
#[doc = "PG Pull Register 0"]
pub mod pg_pull0;
#[doc = "pg_pull1 (rw) register accessor: PG Pull Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_pull1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_pull1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_pull1`] module"]
pub type PG_PULL1 = crate::Reg<pg_pull1::PG_PULL1_SPEC>;
#[doc = "PG Pull Register 1"]
pub mod pg_pull1;
#[doc = "pb_eint_cfg0 (rw) register accessor: PB External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_eint_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_eint_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_eint_cfg0`] module"]
pub type PB_EINT_CFG0 = crate::Reg<pb_eint_cfg0::PB_EINT_CFG0_SPEC>;
#[doc = "PB External Interrupt Configure Register 0"]
pub mod pb_eint_cfg0;
#[doc = "pb_eint_cfg1 (rw) register accessor: PB External Interrupt Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_eint_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_eint_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_eint_cfg1`] module"]
pub type PB_EINT_CFG1 = crate::Reg<pb_eint_cfg1::PB_EINT_CFG1_SPEC>;
#[doc = "PB External Interrupt Configure Register 1"]
pub mod pb_eint_cfg1;
#[doc = "pb_eint_ctl (rw) register accessor: PB External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_eint_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_eint_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_eint_ctl`] module"]
pub type PB_EINT_CTL = crate::Reg<pb_eint_ctl::PB_EINT_CTL_SPEC>;
#[doc = "PB External Interrupt Control Register"]
pub mod pb_eint_ctl;
#[doc = "pb_eint_status (rw) register accessor: PB External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_eint_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_eint_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_eint_status`] module"]
pub type PB_EINT_STATUS = crate::Reg<pb_eint_status::PB_EINT_STATUS_SPEC>;
#[doc = "PB External Interrupt Status Register"]
pub mod pb_eint_status;
#[doc = "pb_eint_deb (rw) register accessor: PB External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_eint_deb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_eint_deb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_eint_deb`] module"]
pub type PB_EINT_DEB = crate::Reg<pb_eint_deb::PB_EINT_DEB_SPEC>;
#[doc = "PB External Interrupt Debounce Register"]
pub mod pb_eint_deb;
#[doc = "pc_eint_cfg0 (rw) register accessor: PC External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_eint_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_eint_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_eint_cfg0`] module"]
pub type PC_EINT_CFG0 = crate::Reg<pc_eint_cfg0::PC_EINT_CFG0_SPEC>;
#[doc = "PC External Interrupt Configure Register 0"]
pub mod pc_eint_cfg0;
#[doc = "pc_eint_ctl (rw) register accessor: PC External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_eint_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_eint_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_eint_ctl`] module"]
pub type PC_EINT_CTL = crate::Reg<pc_eint_ctl::PC_EINT_CTL_SPEC>;
#[doc = "PC External Interrupt Control Register"]
pub mod pc_eint_ctl;
#[doc = "pc_eint_status (rw) register accessor: PC External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_eint_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_eint_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_eint_status`] module"]
pub type PC_EINT_STATUS = crate::Reg<pc_eint_status::PC_EINT_STATUS_SPEC>;
#[doc = "PC External Interrupt Status Register"]
pub mod pc_eint_status;
#[doc = "pc_eint_deb (rw) register accessor: PC External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_eint_deb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_eint_deb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_eint_deb`] module"]
pub type PC_EINT_DEB = crate::Reg<pc_eint_deb::PC_EINT_DEB_SPEC>;
#[doc = "PC External Interrupt Debounce Register"]
pub mod pc_eint_deb;
#[doc = "pd_eint_cfg0 (rw) register accessor: PD External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg0`] module"]
pub type PD_EINT_CFG0 = crate::Reg<pd_eint_cfg0::PD_EINT_CFG0_SPEC>;
#[doc = "PD External Interrupt Configure Register 0"]
pub mod pd_eint_cfg0;
#[doc = "pd_eint_cfg1 (rw) register accessor: PD External Interrupt Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg1`] module"]
pub type PD_EINT_CFG1 = crate::Reg<pd_eint_cfg1::PD_EINT_CFG1_SPEC>;
#[doc = "PD External Interrupt Configure Register 1"]
pub mod pd_eint_cfg1;
#[doc = "pd_eint_cfg2 (rw) register accessor: PD External Interrupt Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg2`] module"]
pub type PD_EINT_CFG2 = crate::Reg<pd_eint_cfg2::PD_EINT_CFG2_SPEC>;
#[doc = "PD External Interrupt Configure Register 2"]
pub mod pd_eint_cfg2;
#[doc = "pd_eint_ctl (rw) register accessor: PD External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_ctl`] module"]
pub type PD_EINT_CTL = crate::Reg<pd_eint_ctl::PD_EINT_CTL_SPEC>;
#[doc = "PD External Interrupt Control Register"]
pub mod pd_eint_ctl;
#[doc = "pd_eint_status (rw) register accessor: PD External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_status`] module"]
pub type PD_EINT_STATUS = crate::Reg<pd_eint_status::PD_EINT_STATUS_SPEC>;
#[doc = "PD External Interrupt Status Register"]
pub mod pd_eint_status;
#[doc = "pd_eint_deb (rw) register accessor: PD External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_deb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_deb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_deb`] module"]
pub type PD_EINT_DEB = crate::Reg<pd_eint_deb::PD_EINT_DEB_SPEC>;
#[doc = "PD External Interrupt Debounce Register"]
pub mod pd_eint_deb;
#[doc = "pe_eint_cfg0 (rw) register accessor: PE External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg0`] module"]
pub type PE_EINT_CFG0 = crate::Reg<pe_eint_cfg0::PE_EINT_CFG0_SPEC>;
#[doc = "PE External Interrupt Configure Register 0"]
pub mod pe_eint_cfg0;
#[doc = "pe_eint_cfg1 (rw) register accessor: PE External Interrupt Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg1`] module"]
pub type PE_EINT_CFG1 = crate::Reg<pe_eint_cfg1::PE_EINT_CFG1_SPEC>;
#[doc = "PE External Interrupt Configure Register 1"]
pub mod pe_eint_cfg1;
#[doc = "pe_eint_cfg2 (rw) register accessor: PE External Interrupt Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg2`] module"]
pub type PE_EINT_CFG2 = crate::Reg<pe_eint_cfg2::PE_EINT_CFG2_SPEC>;
#[doc = "PE External Interrupt Configure Register 2"]
pub mod pe_eint_cfg2;
#[doc = "pe_eint_ctl (rw) register accessor: PE External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_ctl`] module"]
pub type PE_EINT_CTL = crate::Reg<pe_eint_ctl::PE_EINT_CTL_SPEC>;
#[doc = "PE External Interrupt Control Register"]
pub mod pe_eint_ctl;
#[doc = "pe_eint_status (rw) register accessor: PE External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_status`] module"]
pub type PE_EINT_STATUS = crate::Reg<pe_eint_status::PE_EINT_STATUS_SPEC>;
#[doc = "PE External Interrupt Status Register"]
pub mod pe_eint_status;
#[doc = "pe_eint_deb (rw) register accessor: PE External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_deb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_deb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_deb`] module"]
pub type PE_EINT_DEB = crate::Reg<pe_eint_deb::PE_EINT_DEB_SPEC>;
#[doc = "PE External Interrupt Debounce Register"]
pub mod pe_eint_deb;
#[doc = "pf_eint_cfg0 (rw) register accessor: PF External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_eint_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_eint_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_cfg0`] module"]
pub type PF_EINT_CFG0 = crate::Reg<pf_eint_cfg0::PF_EINT_CFG0_SPEC>;
#[doc = "PF External Interrupt Configure Register 0"]
pub mod pf_eint_cfg0;
#[doc = "pf_eint_ctl (rw) register accessor: PF External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_eint_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_eint_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_ctl`] module"]
pub type PF_EINT_CTL = crate::Reg<pf_eint_ctl::PF_EINT_CTL_SPEC>;
#[doc = "PF External Interrupt Control Register"]
pub mod pf_eint_ctl;
#[doc = "pf_eint_status (rw) register accessor: PF External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_eint_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_eint_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_status`] module"]
pub type PF_EINT_STATUS = crate::Reg<pf_eint_status::PF_EINT_STATUS_SPEC>;
#[doc = "PF External Interrupt Status Register"]
pub mod pf_eint_status;
#[doc = "pf_eint_deb (rw) register accessor: PF External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_eint_deb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_eint_deb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_deb`] module"]
pub type PF_EINT_DEB = crate::Reg<pf_eint_deb::PF_EINT_DEB_SPEC>;
#[doc = "PF External Interrupt Debounce Register"]
pub mod pf_eint_deb;
#[doc = "pg_eint_cfg0 (rw) register accessor: PG External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_eint_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_eint_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_eint_cfg0`] module"]
pub type PG_EINT_CFG0 = crate::Reg<pg_eint_cfg0::PG_EINT_CFG0_SPEC>;
#[doc = "PG External Interrupt Configure Register 0"]
pub mod pg_eint_cfg0;
#[doc = "pg_eint_cfg1 (rw) register accessor: PG External Interrupt Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_eint_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_eint_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_eint_cfg1`] module"]
pub type PG_EINT_CFG1 = crate::Reg<pg_eint_cfg1::PG_EINT_CFG1_SPEC>;
#[doc = "PG External Interrupt Configure Register 1"]
pub mod pg_eint_cfg1;
#[doc = "pg_eint_cfg2 (rw) register accessor: PG External Interrupt Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_eint_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_eint_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_eint_cfg2`] module"]
pub type PG_EINT_CFG2 = crate::Reg<pg_eint_cfg2::PG_EINT_CFG2_SPEC>;
#[doc = "PG External Interrupt Configure Register 2"]
pub mod pg_eint_cfg2;
#[doc = "pg_eint_ctl (rw) register accessor: PG External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_eint_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_eint_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_eint_ctl`] module"]
pub type PG_EINT_CTL = crate::Reg<pg_eint_ctl::PG_EINT_CTL_SPEC>;
#[doc = "PG External Interrupt Control Register"]
pub mod pg_eint_ctl;
#[doc = "pg_eint_status (rw) register accessor: PG External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_eint_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_eint_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_eint_status`] module"]
pub type PG_EINT_STATUS = crate::Reg<pg_eint_status::PG_EINT_STATUS_SPEC>;
#[doc = "PG External Interrupt Status Register"]
pub mod pg_eint_status;
#[doc = "pg_eint_deb (rw) register accessor: PG External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_eint_deb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_eint_deb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_eint_deb`] module"]
pub type PG_EINT_DEB = crate::Reg<pg_eint_deb::PG_EINT_DEB_SPEC>;
#[doc = "PG External Interrupt Debounce Register"]
pub mod pg_eint_deb;
#[doc = "pio_pow_mod_sel (rw) register accessor: PIO Group Withstand Voltage Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_mod_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_pow_mod_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio_pow_mod_sel`] module"]
pub type PIO_POW_MOD_SEL = crate::Reg<pio_pow_mod_sel::PIO_POW_MOD_SEL_SPEC>;
#[doc = "PIO Group Withstand Voltage Mode Select Register"]
pub mod pio_pow_mod_sel;
#[doc = "pio_pow_ms_ctl (rw) register accessor: PIO Group Withstand Voltage Mode Select Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_ms_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_pow_ms_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio_pow_ms_ctl`] module"]
pub type PIO_POW_MS_CTL = crate::Reg<pio_pow_ms_ctl::PIO_POW_MS_CTL_SPEC>;
#[doc = "PIO Group Withstand Voltage Mode Select Control Register"]
pub mod pio_pow_ms_ctl;
#[doc = "pio_pow_val (r) register accessor: PIO Group Power Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio_pow_val`] module"]
pub type PIO_POW_VAL = crate::Reg<pio_pow_val::PIO_POW_VAL_SPEC>;
#[doc = "PIO Group Power Value Register"]
pub mod pio_pow_val;
#[doc = "pio_pow_vol_sel_ctl (rw) register accessor: PIO Group Power Voltage Select Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_vol_sel_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_pow_vol_sel_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio_pow_vol_sel_ctl`] module"]
pub type PIO_POW_VOL_SEL_CTL = crate::Reg<pio_pow_vol_sel_ctl::PIO_POW_VOL_SEL_CTL_SPEC>;
#[doc = "PIO Group Power Voltage Select Control Register"]
pub mod pio_pow_vol_sel_ctl;
