#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    #[doc = "0x30 - PB Configure Register 0"]
    pub pb_cfg0: crate::Reg<pb_cfg0::PB_CFG0_SPEC>,
    #[doc = "0x34 - PB Configure Register 1"]
    pub pb_cfg1: crate::Reg<pb_cfg1::PB_CFG1_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x40 - PB Data Register"]
    pub pb_dat: crate::Reg<pb_dat::PB_DAT_SPEC>,
    #[doc = "0x44 - PB Multi_Driving Register 0"]
    pub pb_drv0: crate::Reg<pb_drv0::PB_DRV0_SPEC>,
    #[doc = "0x48 - PB Multi_Driving Register 1"]
    pub pb_drv1: crate::Reg<pb_drv1::PB_DRV1_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x54 - PB Pull Register 0"]
    pub pb_pull0: crate::Reg<pb_pull0::PB_PULL0_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x60 - PC Configure Register 0"]
    pub pc_cfg0: crate::Reg<pc_cfg0::PC_CFG0_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x70 - PC Data Register"]
    pub pc_dat: crate::Reg<pc_dat::PC_DAT_SPEC>,
    #[doc = "0x74 - PC Multi_Driving Register 0"]
    pub pc_drv0: crate::Reg<pc_drv0::PC_DRV0_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x84 - PC Pull Register 0"]
    pub pc_pull0: crate::Reg<pc_pull0::PC_PULL0_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x90 - PD Configure Register 0"]
    pub pd_cfg0: crate::Reg<pd_cfg0::PD_CFG0_SPEC>,
    #[doc = "0x94 - PD Configure Register 1"]
    pub pd_cfg1: crate::Reg<pd_cfg1::PD_CFG1_SPEC>,
    #[doc = "0x98 - PD Configure Register 2"]
    pub pd_cfg2: crate::Reg<pd_cfg2::PD_CFG2_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0xa0 - PD Data Register"]
    pub pd_dat: crate::Reg<pd_dat::PD_DAT_SPEC>,
    #[doc = "0xa4 - PD Multi_Driving Register 0"]
    pub pd_drv0: crate::Reg<pd_drv0::PD_DRV0_SPEC>,
    #[doc = "0xa8 - PD Multi_Driving Register 1"]
    pub pd_drv1: crate::Reg<pd_drv1::PD_DRV1_SPEC>,
    #[doc = "0xac - PD Multi_Driving Register 2"]
    pub pd_drv2: crate::Reg<pd_drv2::PD_DRV2_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0xb4 - PD Pull Register 0"]
    pub pd_pull0: crate::Reg<pd_pull0::PD_PULL0_SPEC>,
    #[doc = "0xb8 - PD Pull Register 1"]
    pub pd_pull1: crate::Reg<pd_pull1::PD_PULL1_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0xc0 - PE Configure Register 0"]
    pub pe_cfg0: crate::Reg<pe_cfg0::PE_CFG0_SPEC>,
    #[doc = "0xc4 - PE Configure Register 1"]
    pub pe_cfg1: crate::Reg<pe_cfg1::PE_CFG1_SPEC>,
    #[doc = "0xc8 - PE Configure Register 2"]
    pub pe_cfg2: crate::Reg<pe_cfg2::PE_CFG2_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0xd0 - PE Data Register"]
    pub pe_dat: crate::Reg<pe_dat::PE_DAT_SPEC>,
    #[doc = "0xd4 - PE Multi_Driving Register 0"]
    pub pe_drv0: crate::Reg<pe_drv0::PE_DRV0_SPEC>,
    #[doc = "0xd8 - PE Multi_Driving Register 1"]
    pub pe_drv1: crate::Reg<pe_drv1::PE_DRV1_SPEC>,
    #[doc = "0xdc - PE Multi_Driving Register 2"]
    pub pe_drv2: crate::Reg<pe_drv2::PE_DRV2_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xe4 - PE Pull Register 0"]
    pub pe_pull0: crate::Reg<pe_pull0::PE_PULL0_SPEC>,
    #[doc = "0xe8 - PE Pull Register 1"]
    pub pe_pull1: crate::Reg<pe_pull1::PE_PULL1_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0xf0 - PF Configure Register 0"]
    pub pf_cfg0: crate::Reg<pf_cfg0::PF_CFG0_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x100 - PF Data Register"]
    pub pf_dat: crate::Reg<pf_dat::PF_DAT_SPEC>,
    #[doc = "0x104 - PF Multi_Driving Register 0"]
    pub pf_drv0: crate::Reg<pf_drv0::PF_DRV0_SPEC>,
    _reserved31: [u8; 0x0c],
    #[doc = "0x114 - PF Pull Register 0"]
    pub pf_pull0: crate::Reg<pf_pull0::PF_PULL0_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0x120 - PG Configure Register 0"]
    pub pg_cfg0: crate::Reg<pg_cfg0::PG_CFG0_SPEC>,
    #[doc = "0x124 - PG Configure Register 1"]
    pub pg_cfg1: crate::Reg<pg_cfg1::PG_CFG1_SPEC>,
    #[doc = "0x128 - PG Configure Register 2"]
    pub pg_cfg2: crate::Reg<pg_cfg2::PG_CFG2_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x130 - PG Data Register"]
    pub pg_dat: crate::Reg<pg_dat::PG_DAT_SPEC>,
    #[doc = "0x134 - PG Multi_Driving Register 0"]
    pub pg_drv0: crate::Reg<pg_drv0::PG_DRV0_SPEC>,
    #[doc = "0x138 - PG Multi_Driving Register 1"]
    pub pg_drv1: crate::Reg<pg_drv1::PG_DRV1_SPEC>,
    #[doc = "0x13c - PG Multi_Driving Register 2"]
    pub pg_drv2: crate::Reg<pg_drv2::PG_DRV2_SPEC>,
    _reserved39: [u8; 0x04],
    #[doc = "0x144 - PG Pull Register 0"]
    pub pg_pull0: crate::Reg<pg_pull0::PG_PULL0_SPEC>,
    #[doc = "0x148 - PG Pull Register 1"]
    pub pg_pull1: crate::Reg<pg_pull1::PG_PULL1_SPEC>,
    _reserved41: [u8; 0xd4],
    #[doc = "0x220 - PB External Interrupt Configure Register 0"]
    pub pb_eint_cfg0: crate::Reg<pb_eint_cfg0::PB_EINT_CFG0_SPEC>,
    #[doc = "0x224 - PB External Interrupt Configure Register 1"]
    pub pb_eint_cfg1: crate::Reg<pb_eint_cfg1::PB_EINT_CFG1_SPEC>,
    _reserved43: [u8; 0x08],
    #[doc = "0x230 - PB External Interrupt Control Register"]
    pub pb_eint_ctl: crate::Reg<pb_eint_ctl::PB_EINT_CTL_SPEC>,
    #[doc = "0x234 - PB External Interrupt Status Register"]
    pub pb_eint_status: crate::Reg<pb_eint_status::PB_EINT_STATUS_SPEC>,
    #[doc = "0x238 - PB External Interrupt Debounce Register"]
    pub pb_eint_deb: crate::Reg<pb_eint_deb::PB_EINT_DEB_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x240 - PC External Interrupt Configure Register 0"]
    pub pc_eint_cfg0: crate::Reg<pc_eint_cfg0::PC_EINT_CFG0_SPEC>,
    _reserved47: [u8; 0x0c],
    #[doc = "0x250 - PC External Interrupt Control Register"]
    pub pc_eint_ctl: crate::Reg<pc_eint_ctl::PC_EINT_CTL_SPEC>,
    #[doc = "0x254 - PC External Interrupt Status Register"]
    pub pc_eint_status: crate::Reg<pc_eint_status::PC_EINT_STATUS_SPEC>,
    #[doc = "0x258 - PC External Interrupt Debounce Register"]
    pub pc_eint_deb: crate::Reg<pc_eint_deb::PC_EINT_DEB_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x260 - PD External Interrupt Configure Register 0"]
    pub pd_eint_cfg0: crate::Reg<pd_eint_cfg0::PD_EINT_CFG0_SPEC>,
    #[doc = "0x264 - PD External Interrupt Configure Register 1"]
    pub pd_eint_cfg1: crate::Reg<pd_eint_cfg1::PD_EINT_CFG1_SPEC>,
    #[doc = "0x268 - PD External Interrupt Configure Register 2"]
    pub pd_eint_cfg2: crate::Reg<pd_eint_cfg2::PD_EINT_CFG2_SPEC>,
    _reserved53: [u8; 0x04],
    #[doc = "0x270 - PD External Interrupt Control Register"]
    pub pd_eint_ctl: crate::Reg<pd_eint_ctl::PD_EINT_CTL_SPEC>,
    #[doc = "0x274 - PD External Interrupt Status Register"]
    pub pd_eint_status: crate::Reg<pd_eint_status::PD_EINT_STATUS_SPEC>,
    #[doc = "0x278 - PD External Interrupt Debounce Register"]
    pub pd_eint_deb: crate::Reg<pd_eint_deb::PD_EINT_DEB_SPEC>,
    _reserved56: [u8; 0x04],
    #[doc = "0x280 - PE External Interrupt Configure Register 0"]
    pub pe_eint_cfg0: crate::Reg<pe_eint_cfg0::PE_EINT_CFG0_SPEC>,
    #[doc = "0x284 - PE External Interrupt Configure Register 1"]
    pub pe_eint_cfg1: crate::Reg<pe_eint_cfg1::PE_EINT_CFG1_SPEC>,
    #[doc = "0x288 - PE External Interrupt Configure Register 2"]
    pub pe_eint_cfg2: crate::Reg<pe_eint_cfg2::PE_EINT_CFG2_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0x290 - PE External Interrupt Control Register"]
    pub pe_eint_ctl: crate::Reg<pe_eint_ctl::PE_EINT_CTL_SPEC>,
    #[doc = "0x294 - PE External Interrupt Status Register"]
    pub pe_eint_status: crate::Reg<pe_eint_status::PE_EINT_STATUS_SPEC>,
    #[doc = "0x298 - PE External Interrupt Debounce Register"]
    pub pe_eint_deb: crate::Reg<pe_eint_deb::PE_EINT_DEB_SPEC>,
    _reserved62: [u8; 0x04],
    #[doc = "0x2a0 - PF External Interrupt Configure Register 0"]
    pub pf_eint_cfg0: crate::Reg<pf_eint_cfg0::PF_EINT_CFG0_SPEC>,
    _reserved63: [u8; 0x0c],
    #[doc = "0x2b0 - PF External Interrupt Control Register"]
    pub pf_eint_ctl: crate::Reg<pf_eint_ctl::PF_EINT_CTL_SPEC>,
    #[doc = "0x2b4 - PF External Interrupt Status Register"]
    pub pf_eint_status: crate::Reg<pf_eint_status::PF_EINT_STATUS_SPEC>,
    #[doc = "0x2b8 - PF External Interrupt Debounce Register"]
    pub pf_eint_deb: crate::Reg<pf_eint_deb::PF_EINT_DEB_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x2c0 - PG External Interrupt Configure Register 0"]
    pub pg_eint_cfg0: crate::Reg<pg_eint_cfg0::PG_EINT_CFG0_SPEC>,
    #[doc = "0x2c4 - PG External Interrupt Configure Register 1"]
    pub pg_eint_cfg1: crate::Reg<pg_eint_cfg1::PG_EINT_CFG1_SPEC>,
    #[doc = "0x2c8 - PG External Interrupt Configure Register 2"]
    pub pg_eint_cfg2: crate::Reg<pg_eint_cfg2::PG_EINT_CFG2_SPEC>,
    _reserved69: [u8; 0x04],
    #[doc = "0x2d0 - PG External Interrupt Control Register"]
    pub pg_eint_ctl: crate::Reg<pg_eint_ctl::PG_EINT_CTL_SPEC>,
    #[doc = "0x2d4 - PG External Interrupt Status Register"]
    pub pg_eint_status: crate::Reg<pg_eint_status::PG_EINT_STATUS_SPEC>,
    #[doc = "0x2d8 - PG External Interrupt Debounce Register"]
    pub pg_eint_deb: crate::Reg<pg_eint_deb::PG_EINT_DEB_SPEC>,
    _reserved72: [u8; 0x64],
    #[doc = "0x340 - PIO Group Withstand Voltage Mode Select Register"]
    pub pio_pow_mod_sel: crate::Reg<pio_pow_mod_sel::PIO_POW_MOD_SEL_SPEC>,
    #[doc = "0x344 - PIO Group Withstand Voltage Mode Select Control Register"]
    pub pio_pow_ms_ctl: crate::Reg<pio_pow_ms_ctl::PIO_POW_MS_CTL_SPEC>,
    #[doc = "0x348 - PIO Group Power Value Register"]
    pub pio_pow_val: crate::Reg<pio_pow_val::PIO_POW_VAL_SPEC>,
    _reserved75: [u8; 0x04],
    #[doc = "0x350 - PIO Group Power Voltage Select Control Register"]
    pub pio_pow_vol_sel_ctl: crate::Reg<pio_pow_vol_sel_ctl::PIO_POW_VOL_SEL_CTL_SPEC>,
}
#[doc = "pb_cfg0 register accessor: an alias for `Reg<PB_CFG0_SPEC>`"]
pub type PB_CFG0 = crate::Reg<pb_cfg0::PB_CFG0_SPEC>;
#[doc = "PB Configure Register 0"]
pub mod pb_cfg0;
#[doc = "pb_cfg1 register accessor: an alias for `Reg<PB_CFG1_SPEC>`"]
pub type PB_CFG1 = crate::Reg<pb_cfg1::PB_CFG1_SPEC>;
#[doc = "PB Configure Register 1"]
pub mod pb_cfg1;
#[doc = "pb_dat register accessor: an alias for `Reg<PB_DAT_SPEC>`"]
pub type PB_DAT = crate::Reg<pb_dat::PB_DAT_SPEC>;
#[doc = "PB Data Register"]
pub mod pb_dat;
#[doc = "pb_drv0 register accessor: an alias for `Reg<PB_DRV0_SPEC>`"]
pub type PB_DRV0 = crate::Reg<pb_drv0::PB_DRV0_SPEC>;
#[doc = "PB Multi_Driving Register 0"]
pub mod pb_drv0;
#[doc = "pb_drv1 register accessor: an alias for `Reg<PB_DRV1_SPEC>`"]
pub type PB_DRV1 = crate::Reg<pb_drv1::PB_DRV1_SPEC>;
#[doc = "PB Multi_Driving Register 1"]
pub mod pb_drv1;
#[doc = "pb_pull0 register accessor: an alias for `Reg<PB_PULL0_SPEC>`"]
pub type PB_PULL0 = crate::Reg<pb_pull0::PB_PULL0_SPEC>;
#[doc = "PB Pull Register 0"]
pub mod pb_pull0;
#[doc = "pc_cfg0 register accessor: an alias for `Reg<PC_CFG0_SPEC>`"]
pub type PC_CFG0 = crate::Reg<pc_cfg0::PC_CFG0_SPEC>;
#[doc = "PC Configure Register 0"]
pub mod pc_cfg0;
#[doc = "pc_dat register accessor: an alias for `Reg<PC_DAT_SPEC>`"]
pub type PC_DAT = crate::Reg<pc_dat::PC_DAT_SPEC>;
#[doc = "PC Data Register"]
pub mod pc_dat;
#[doc = "pc_drv0 register accessor: an alias for `Reg<PC_DRV0_SPEC>`"]
pub type PC_DRV0 = crate::Reg<pc_drv0::PC_DRV0_SPEC>;
#[doc = "PC Multi_Driving Register 0"]
pub mod pc_drv0;
#[doc = "pc_pull0 register accessor: an alias for `Reg<PC_PULL0_SPEC>`"]
pub type PC_PULL0 = crate::Reg<pc_pull0::PC_PULL0_SPEC>;
#[doc = "PC Pull Register 0"]
pub mod pc_pull0;
#[doc = "pd_cfg0 register accessor: an alias for `Reg<PD_CFG0_SPEC>`"]
pub type PD_CFG0 = crate::Reg<pd_cfg0::PD_CFG0_SPEC>;
#[doc = "PD Configure Register 0"]
pub mod pd_cfg0;
#[doc = "pd_cfg1 register accessor: an alias for `Reg<PD_CFG1_SPEC>`"]
pub type PD_CFG1 = crate::Reg<pd_cfg1::PD_CFG1_SPEC>;
#[doc = "PD Configure Register 1"]
pub mod pd_cfg1;
#[doc = "pd_cfg2 register accessor: an alias for `Reg<PD_CFG2_SPEC>`"]
pub type PD_CFG2 = crate::Reg<pd_cfg2::PD_CFG2_SPEC>;
#[doc = "PD Configure Register 2"]
pub mod pd_cfg2;
#[doc = "pd_dat register accessor: an alias for `Reg<PD_DAT_SPEC>`"]
pub type PD_DAT = crate::Reg<pd_dat::PD_DAT_SPEC>;
#[doc = "PD Data Register"]
pub mod pd_dat;
#[doc = "pd_drv0 register accessor: an alias for `Reg<PD_DRV0_SPEC>`"]
pub type PD_DRV0 = crate::Reg<pd_drv0::PD_DRV0_SPEC>;
#[doc = "PD Multi_Driving Register 0"]
pub mod pd_drv0;
#[doc = "pd_drv1 register accessor: an alias for `Reg<PD_DRV1_SPEC>`"]
pub type PD_DRV1 = crate::Reg<pd_drv1::PD_DRV1_SPEC>;
#[doc = "PD Multi_Driving Register 1"]
pub mod pd_drv1;
#[doc = "pd_drv2 register accessor: an alias for `Reg<PD_DRV2_SPEC>`"]
pub type PD_DRV2 = crate::Reg<pd_drv2::PD_DRV2_SPEC>;
#[doc = "PD Multi_Driving Register 2"]
pub mod pd_drv2;
#[doc = "pd_pull0 register accessor: an alias for `Reg<PD_PULL0_SPEC>`"]
pub type PD_PULL0 = crate::Reg<pd_pull0::PD_PULL0_SPEC>;
#[doc = "PD Pull Register 0"]
pub mod pd_pull0;
#[doc = "pd_pull1 register accessor: an alias for `Reg<PD_PULL1_SPEC>`"]
pub type PD_PULL1 = crate::Reg<pd_pull1::PD_PULL1_SPEC>;
#[doc = "PD Pull Register 1"]
pub mod pd_pull1;
#[doc = "pe_cfg0 register accessor: an alias for `Reg<PE_CFG0_SPEC>`"]
pub type PE_CFG0 = crate::Reg<pe_cfg0::PE_CFG0_SPEC>;
#[doc = "PE Configure Register 0"]
pub mod pe_cfg0;
#[doc = "pe_cfg1 register accessor: an alias for `Reg<PE_CFG1_SPEC>`"]
pub type PE_CFG1 = crate::Reg<pe_cfg1::PE_CFG1_SPEC>;
#[doc = "PE Configure Register 1"]
pub mod pe_cfg1;
#[doc = "pe_cfg2 register accessor: an alias for `Reg<PE_CFG2_SPEC>`"]
pub type PE_CFG2 = crate::Reg<pe_cfg2::PE_CFG2_SPEC>;
#[doc = "PE Configure Register 2"]
pub mod pe_cfg2;
#[doc = "pe_dat register accessor: an alias for `Reg<PE_DAT_SPEC>`"]
pub type PE_DAT = crate::Reg<pe_dat::PE_DAT_SPEC>;
#[doc = "PE Data Register"]
pub mod pe_dat;
#[doc = "pe_drv0 register accessor: an alias for `Reg<PE_DRV0_SPEC>`"]
pub type PE_DRV0 = crate::Reg<pe_drv0::PE_DRV0_SPEC>;
#[doc = "PE Multi_Driving Register 0"]
pub mod pe_drv0;
#[doc = "pe_drv1 register accessor: an alias for `Reg<PE_DRV1_SPEC>`"]
pub type PE_DRV1 = crate::Reg<pe_drv1::PE_DRV1_SPEC>;
#[doc = "PE Multi_Driving Register 1"]
pub mod pe_drv1;
#[doc = "pe_drv2 register accessor: an alias for `Reg<PE_DRV2_SPEC>`"]
pub type PE_DRV2 = crate::Reg<pe_drv2::PE_DRV2_SPEC>;
#[doc = "PE Multi_Driving Register 2"]
pub mod pe_drv2;
#[doc = "pe_pull0 register accessor: an alias for `Reg<PE_PULL0_SPEC>`"]
pub type PE_PULL0 = crate::Reg<pe_pull0::PE_PULL0_SPEC>;
#[doc = "PE Pull Register 0"]
pub mod pe_pull0;
#[doc = "pe_pull1 register accessor: an alias for `Reg<PE_PULL1_SPEC>`"]
pub type PE_PULL1 = crate::Reg<pe_pull1::PE_PULL1_SPEC>;
#[doc = "PE Pull Register 1"]
pub mod pe_pull1;
#[doc = "pf_cfg0 register accessor: an alias for `Reg<PF_CFG0_SPEC>`"]
pub type PF_CFG0 = crate::Reg<pf_cfg0::PF_CFG0_SPEC>;
#[doc = "PF Configure Register 0"]
pub mod pf_cfg0;
#[doc = "pf_dat register accessor: an alias for `Reg<PF_DAT_SPEC>`"]
pub type PF_DAT = crate::Reg<pf_dat::PF_DAT_SPEC>;
#[doc = "PF Data Register"]
pub mod pf_dat;
#[doc = "pf_drv0 register accessor: an alias for `Reg<PF_DRV0_SPEC>`"]
pub type PF_DRV0 = crate::Reg<pf_drv0::PF_DRV0_SPEC>;
#[doc = "PF Multi_Driving Register 0"]
pub mod pf_drv0;
#[doc = "pf_pull0 register accessor: an alias for `Reg<PF_PULL0_SPEC>`"]
pub type PF_PULL0 = crate::Reg<pf_pull0::PF_PULL0_SPEC>;
#[doc = "PF Pull Register 0"]
pub mod pf_pull0;
#[doc = "pg_cfg0 register accessor: an alias for `Reg<PG_CFG0_SPEC>`"]
pub type PG_CFG0 = crate::Reg<pg_cfg0::PG_CFG0_SPEC>;
#[doc = "PG Configure Register 0"]
pub mod pg_cfg0;
#[doc = "pg_cfg1 register accessor: an alias for `Reg<PG_CFG1_SPEC>`"]
pub type PG_CFG1 = crate::Reg<pg_cfg1::PG_CFG1_SPEC>;
#[doc = "PG Configure Register 1"]
pub mod pg_cfg1;
#[doc = "pg_cfg2 register accessor: an alias for `Reg<PG_CFG2_SPEC>`"]
pub type PG_CFG2 = crate::Reg<pg_cfg2::PG_CFG2_SPEC>;
#[doc = "PG Configure Register 2"]
pub mod pg_cfg2;
#[doc = "pg_dat register accessor: an alias for `Reg<PG_DAT_SPEC>`"]
pub type PG_DAT = crate::Reg<pg_dat::PG_DAT_SPEC>;
#[doc = "PG Data Register"]
pub mod pg_dat;
#[doc = "pg_drv0 register accessor: an alias for `Reg<PG_DRV0_SPEC>`"]
pub type PG_DRV0 = crate::Reg<pg_drv0::PG_DRV0_SPEC>;
#[doc = "PG Multi_Driving Register 0"]
pub mod pg_drv0;
#[doc = "pg_drv1 register accessor: an alias for `Reg<PG_DRV1_SPEC>`"]
pub type PG_DRV1 = crate::Reg<pg_drv1::PG_DRV1_SPEC>;
#[doc = "PG Multi_Driving Register 1"]
pub mod pg_drv1;
#[doc = "pg_drv2 register accessor: an alias for `Reg<PG_DRV2_SPEC>`"]
pub type PG_DRV2 = crate::Reg<pg_drv2::PG_DRV2_SPEC>;
#[doc = "PG Multi_Driving Register 2"]
pub mod pg_drv2;
#[doc = "pg_pull0 register accessor: an alias for `Reg<PG_PULL0_SPEC>`"]
pub type PG_PULL0 = crate::Reg<pg_pull0::PG_PULL0_SPEC>;
#[doc = "PG Pull Register 0"]
pub mod pg_pull0;
#[doc = "pg_pull1 register accessor: an alias for `Reg<PG_PULL1_SPEC>`"]
pub type PG_PULL1 = crate::Reg<pg_pull1::PG_PULL1_SPEC>;
#[doc = "PG Pull Register 1"]
pub mod pg_pull1;
#[doc = "pb_eint_cfg0 register accessor: an alias for `Reg<PB_EINT_CFG0_SPEC>`"]
pub type PB_EINT_CFG0 = crate::Reg<pb_eint_cfg0::PB_EINT_CFG0_SPEC>;
#[doc = "PB External Interrupt Configure Register 0"]
pub mod pb_eint_cfg0;
#[doc = "pb_eint_cfg1 register accessor: an alias for `Reg<PB_EINT_CFG1_SPEC>`"]
pub type PB_EINT_CFG1 = crate::Reg<pb_eint_cfg1::PB_EINT_CFG1_SPEC>;
#[doc = "PB External Interrupt Configure Register 1"]
pub mod pb_eint_cfg1;
#[doc = "pb_eint_ctl register accessor: an alias for `Reg<PB_EINT_CTL_SPEC>`"]
pub type PB_EINT_CTL = crate::Reg<pb_eint_ctl::PB_EINT_CTL_SPEC>;
#[doc = "PB External Interrupt Control Register"]
pub mod pb_eint_ctl;
#[doc = "pb_eint_status register accessor: an alias for `Reg<PB_EINT_STATUS_SPEC>`"]
pub type PB_EINT_STATUS = crate::Reg<pb_eint_status::PB_EINT_STATUS_SPEC>;
#[doc = "PB External Interrupt Status Register"]
pub mod pb_eint_status;
#[doc = "pb_eint_deb register accessor: an alias for `Reg<PB_EINT_DEB_SPEC>`"]
pub type PB_EINT_DEB = crate::Reg<pb_eint_deb::PB_EINT_DEB_SPEC>;
#[doc = "PB External Interrupt Debounce Register"]
pub mod pb_eint_deb;
#[doc = "pc_eint_cfg0 register accessor: an alias for `Reg<PC_EINT_CFG0_SPEC>`"]
pub type PC_EINT_CFG0 = crate::Reg<pc_eint_cfg0::PC_EINT_CFG0_SPEC>;
#[doc = "PC External Interrupt Configure Register 0"]
pub mod pc_eint_cfg0;
#[doc = "pc_eint_ctl register accessor: an alias for `Reg<PC_EINT_CTL_SPEC>`"]
pub type PC_EINT_CTL = crate::Reg<pc_eint_ctl::PC_EINT_CTL_SPEC>;
#[doc = "PC External Interrupt Control Register"]
pub mod pc_eint_ctl;
#[doc = "pc_eint_status register accessor: an alias for `Reg<PC_EINT_STATUS_SPEC>`"]
pub type PC_EINT_STATUS = crate::Reg<pc_eint_status::PC_EINT_STATUS_SPEC>;
#[doc = "PC External Interrupt Status Register"]
pub mod pc_eint_status;
#[doc = "pc_eint_deb register accessor: an alias for `Reg<PC_EINT_DEB_SPEC>`"]
pub type PC_EINT_DEB = crate::Reg<pc_eint_deb::PC_EINT_DEB_SPEC>;
#[doc = "PC External Interrupt Debounce Register"]
pub mod pc_eint_deb;
#[doc = "pd_eint_cfg0 register accessor: an alias for `Reg<PD_EINT_CFG0_SPEC>`"]
pub type PD_EINT_CFG0 = crate::Reg<pd_eint_cfg0::PD_EINT_CFG0_SPEC>;
#[doc = "PD External Interrupt Configure Register 0"]
pub mod pd_eint_cfg0;
#[doc = "pd_eint_cfg1 register accessor: an alias for `Reg<PD_EINT_CFG1_SPEC>`"]
pub type PD_EINT_CFG1 = crate::Reg<pd_eint_cfg1::PD_EINT_CFG1_SPEC>;
#[doc = "PD External Interrupt Configure Register 1"]
pub mod pd_eint_cfg1;
#[doc = "pd_eint_cfg2 register accessor: an alias for `Reg<PD_EINT_CFG2_SPEC>`"]
pub type PD_EINT_CFG2 = crate::Reg<pd_eint_cfg2::PD_EINT_CFG2_SPEC>;
#[doc = "PD External Interrupt Configure Register 2"]
pub mod pd_eint_cfg2;
#[doc = "pd_eint_ctl register accessor: an alias for `Reg<PD_EINT_CTL_SPEC>`"]
pub type PD_EINT_CTL = crate::Reg<pd_eint_ctl::PD_EINT_CTL_SPEC>;
#[doc = "PD External Interrupt Control Register"]
pub mod pd_eint_ctl;
#[doc = "pd_eint_status register accessor: an alias for `Reg<PD_EINT_STATUS_SPEC>`"]
pub type PD_EINT_STATUS = crate::Reg<pd_eint_status::PD_EINT_STATUS_SPEC>;
#[doc = "PD External Interrupt Status Register"]
pub mod pd_eint_status;
#[doc = "pd_eint_deb register accessor: an alias for `Reg<PD_EINT_DEB_SPEC>`"]
pub type PD_EINT_DEB = crate::Reg<pd_eint_deb::PD_EINT_DEB_SPEC>;
#[doc = "PD External Interrupt Debounce Register"]
pub mod pd_eint_deb;
#[doc = "pe_eint_cfg0 register accessor: an alias for `Reg<PE_EINT_CFG0_SPEC>`"]
pub type PE_EINT_CFG0 = crate::Reg<pe_eint_cfg0::PE_EINT_CFG0_SPEC>;
#[doc = "PE External Interrupt Configure Register 0"]
pub mod pe_eint_cfg0;
#[doc = "pe_eint_cfg1 register accessor: an alias for `Reg<PE_EINT_CFG1_SPEC>`"]
pub type PE_EINT_CFG1 = crate::Reg<pe_eint_cfg1::PE_EINT_CFG1_SPEC>;
#[doc = "PE External Interrupt Configure Register 1"]
pub mod pe_eint_cfg1;
#[doc = "pe_eint_cfg2 register accessor: an alias for `Reg<PE_EINT_CFG2_SPEC>`"]
pub type PE_EINT_CFG2 = crate::Reg<pe_eint_cfg2::PE_EINT_CFG2_SPEC>;
#[doc = "PE External Interrupt Configure Register 2"]
pub mod pe_eint_cfg2;
#[doc = "pe_eint_ctl register accessor: an alias for `Reg<PE_EINT_CTL_SPEC>`"]
pub type PE_EINT_CTL = crate::Reg<pe_eint_ctl::PE_EINT_CTL_SPEC>;
#[doc = "PE External Interrupt Control Register"]
pub mod pe_eint_ctl;
#[doc = "pe_eint_status register accessor: an alias for `Reg<PE_EINT_STATUS_SPEC>`"]
pub type PE_EINT_STATUS = crate::Reg<pe_eint_status::PE_EINT_STATUS_SPEC>;
#[doc = "PE External Interrupt Status Register"]
pub mod pe_eint_status;
#[doc = "pe_eint_deb register accessor: an alias for `Reg<PE_EINT_DEB_SPEC>`"]
pub type PE_EINT_DEB = crate::Reg<pe_eint_deb::PE_EINT_DEB_SPEC>;
#[doc = "PE External Interrupt Debounce Register"]
pub mod pe_eint_deb;
#[doc = "pf_eint_cfg0 register accessor: an alias for `Reg<PF_EINT_CFG0_SPEC>`"]
pub type PF_EINT_CFG0 = crate::Reg<pf_eint_cfg0::PF_EINT_CFG0_SPEC>;
#[doc = "PF External Interrupt Configure Register 0"]
pub mod pf_eint_cfg0;
#[doc = "pf_eint_ctl register accessor: an alias for `Reg<PF_EINT_CTL_SPEC>`"]
pub type PF_EINT_CTL = crate::Reg<pf_eint_ctl::PF_EINT_CTL_SPEC>;
#[doc = "PF External Interrupt Control Register"]
pub mod pf_eint_ctl;
#[doc = "pf_eint_status register accessor: an alias for `Reg<PF_EINT_STATUS_SPEC>`"]
pub type PF_EINT_STATUS = crate::Reg<pf_eint_status::PF_EINT_STATUS_SPEC>;
#[doc = "PF External Interrupt Status Register"]
pub mod pf_eint_status;
#[doc = "pf_eint_deb register accessor: an alias for `Reg<PF_EINT_DEB_SPEC>`"]
pub type PF_EINT_DEB = crate::Reg<pf_eint_deb::PF_EINT_DEB_SPEC>;
#[doc = "PF External Interrupt Debounce Register"]
pub mod pf_eint_deb;
#[doc = "pg_eint_cfg0 register accessor: an alias for `Reg<PG_EINT_CFG0_SPEC>`"]
pub type PG_EINT_CFG0 = crate::Reg<pg_eint_cfg0::PG_EINT_CFG0_SPEC>;
#[doc = "PG External Interrupt Configure Register 0"]
pub mod pg_eint_cfg0;
#[doc = "pg_eint_cfg1 register accessor: an alias for `Reg<PG_EINT_CFG1_SPEC>`"]
pub type PG_EINT_CFG1 = crate::Reg<pg_eint_cfg1::PG_EINT_CFG1_SPEC>;
#[doc = "PG External Interrupt Configure Register 1"]
pub mod pg_eint_cfg1;
#[doc = "pg_eint_cfg2 register accessor: an alias for `Reg<PG_EINT_CFG2_SPEC>`"]
pub type PG_EINT_CFG2 = crate::Reg<pg_eint_cfg2::PG_EINT_CFG2_SPEC>;
#[doc = "PG External Interrupt Configure Register 2"]
pub mod pg_eint_cfg2;
#[doc = "pg_eint_ctl register accessor: an alias for `Reg<PG_EINT_CTL_SPEC>`"]
pub type PG_EINT_CTL = crate::Reg<pg_eint_ctl::PG_EINT_CTL_SPEC>;
#[doc = "PG External Interrupt Control Register"]
pub mod pg_eint_ctl;
#[doc = "pg_eint_status register accessor: an alias for `Reg<PG_EINT_STATUS_SPEC>`"]
pub type PG_EINT_STATUS = crate::Reg<pg_eint_status::PG_EINT_STATUS_SPEC>;
#[doc = "PG External Interrupt Status Register"]
pub mod pg_eint_status;
#[doc = "pg_eint_deb register accessor: an alias for `Reg<PG_EINT_DEB_SPEC>`"]
pub type PG_EINT_DEB = crate::Reg<pg_eint_deb::PG_EINT_DEB_SPEC>;
#[doc = "PG External Interrupt Debounce Register"]
pub mod pg_eint_deb;
#[doc = "pio_pow_mod_sel register accessor: an alias for `Reg<PIO_POW_MOD_SEL_SPEC>`"]
pub type PIO_POW_MOD_SEL = crate::Reg<pio_pow_mod_sel::PIO_POW_MOD_SEL_SPEC>;
#[doc = "PIO Group Withstand Voltage Mode Select Register"]
pub mod pio_pow_mod_sel;
#[doc = "pio_pow_ms_ctl register accessor: an alias for `Reg<PIO_POW_MS_CTL_SPEC>`"]
pub type PIO_POW_MS_CTL = crate::Reg<pio_pow_ms_ctl::PIO_POW_MS_CTL_SPEC>;
#[doc = "PIO Group Withstand Voltage Mode Select Control Register"]
pub mod pio_pow_ms_ctl;
#[doc = "pio_pow_val register accessor: an alias for `Reg<PIO_POW_VAL_SPEC>`"]
pub type PIO_POW_VAL = crate::Reg<pio_pow_val::PIO_POW_VAL_SPEC>;
#[doc = "PIO Group Power Value Register"]
pub mod pio_pow_val;
#[doc = "pio_pow_vol_sel_ctl register accessor: an alias for `Reg<PIO_POW_VOL_SEL_CTL_SPEC>`"]
pub type PIO_POW_VOL_SEL_CTL = crate::Reg<pio_pow_vol_sel_ctl::PIO_POW_VOL_SEL_CTL_SPEC>;
#[doc = "PIO Group Power Voltage Select Control Register"]
pub mod pio_pow_vol_sel_ctl;
