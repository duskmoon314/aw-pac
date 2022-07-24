#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD Global Control Register"]
    pub lcd_gctl: crate::Reg<lcd_gctl::LCD_GCTL_SPEC>,
    #[doc = "0x04 - LCD Global Interrupt Register0"]
    pub lcd_gint0: crate::Reg<lcd_gint0::LCD_GINT0_SPEC>,
    #[doc = "0x08 - LCD Global Interrupt Register1"]
    pub lcd_gint1: crate::Reg<lcd_gint1::LCD_GINT1_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - LCD FRM Control Register"]
    pub lcd_frm_ctl: crate::Reg<lcd_frm_ctl::LCD_FRM_CTL_SPEC>,
    #[doc = "0x14..0x2c - LCD FRM Seed Register"]
    pub lcd_frm_seed: [crate::Reg<lcd_frm_seed::LCD_FRM_SEED_SPEC>; 6],
    #[doc = "0x2c..0x3c - LCD FRM Table Register"]
    pub lcd_frm_tab: [crate::Reg<lcd_frm_tab::LCD_FRM_TAB_SPEC>; 4],
    #[doc = "0x3c - LCD 3D FIFO Register"]
    pub lcd_3d_fifo: crate::Reg<lcd_3d_fifo::LCD_3D_FIFO_SPEC>,
    #[doc = "0x40 - LCD Control Register"]
    pub lcd_ctl: crate::Reg<lcd_ctl::LCD_CTL_SPEC>,
    #[doc = "0x44 - LCD Data Clock Register"]
    pub lcd_dclk: crate::Reg<lcd_dclk::LCD_DCLK_SPEC>,
    #[doc = "0x48 - LCD Basic Timing Register0"]
    pub lcd_basic0: crate::Reg<lcd_basic0::LCD_BASIC0_SPEC>,
    #[doc = "0x4c - LCD Basic Timing Register1"]
    pub lcd_basic1: crate::Reg<lcd_basic1::LCD_BASIC1_SPEC>,
    #[doc = "0x50 - LCD Basic Timing Register2"]
    pub lcd_basic2: crate::Reg<lcd_basic2::LCD_BASIC2_SPEC>,
    #[doc = "0x54 - LCD Basic Timing Register3"]
    pub lcd_basic3: crate::Reg<lcd_basic3::LCD_BASIC3_SPEC>,
    #[doc = "0x58 - LCD HV Panel Interface Register"]
    pub lcd_hv_if: crate::Reg<lcd_hv_if::LCD_HV_IF_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x60 - LCD CPU Panel Interface Register"]
    pub lcd_cpu_if: crate::Reg<lcd_cpu_if::LCD_CPU_IF_SPEC>,
    #[doc = "0x64 - LCD CPU Panel Write Data Register"]
    pub lcd_cpu_wr: crate::Reg<lcd_cpu_wr::LCD_CPU_WR_SPEC>,
    #[doc = "0x68..0x70 - LCD CPU Panel Read Data Register\\[i\\]"]
    pub lcd_cpu_rd: [crate::Reg<lcd_cpu_rd::LCD_CPU_RD_SPEC>; 2],
    _reserved17: [u8; 0x14],
    #[doc = "0x84 - LCD LVDS Configure Register"]
    pub lcd_lvds_if: crate::Reg<lcd_lvds_if::LCD_LVDS_IF_SPEC>,
    #[doc = "0x88 - LCD IO Polarity Register"]
    pub lcd_io_pol: crate::Reg<lcd_io_pol::LCD_IO_POL_SPEC>,
    #[doc = "0x8c - LCD IO Control Register"]
    pub lcd_io_tri: crate::Reg<lcd_io_tri::LCD_IO_TRI_SPEC>,
    _reserved20: [u8; 0x6c],
    #[doc = "0xfc - LCD Debug Register"]
    pub lcd_debug: crate::Reg<lcd_debug::LCD_DEBUG_SPEC>,
    #[doc = "0x100 - LCD CEU Control Register"]
    pub lcd_ceu_ctl: crate::Reg<lcd_ceu_ctl::LCD_CEU_CTL_SPEC>,
    _reserved22: [u8; 0x0c],
    #[doc = "0x110..0x11c - LCD CEU Coefficient Register0"]
    pub lcd_ceu_coef_mul: [crate::Reg<lcd_ceu_coef_mul::LCD_CEU_COEF_MUL_SPEC>; 3],
    #[doc = "0x11c - LCD CEU Coefficient Register1"]
    pub lcd_ceu_coef_add0: crate::Reg<lcd_ceu_coef_add::LCD_CEU_COEF_ADD_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0x12c - LCD CEU Coefficient Register1"]
    pub lcd_ceu_coef_add1: crate::Reg<lcd_ceu_coef_add::LCD_CEU_COEF_ADD_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0x13c - LCD CEU Coefficient Register1"]
    pub lcd_ceu_coef_add2: crate::Reg<lcd_ceu_coef_add::LCD_CEU_COEF_ADD_SPEC>,
    #[doc = "0x140..0x14c - LCD CEU Coefficient Register2"]
    pub lcd_ceu_coef_rang: [crate::Reg<lcd_ceu_coef_rang::LCD_CEU_COEF_RANG_SPEC>; 3],
    _reserved27: [u8; 0x14],
    #[doc = "0x160 - LCD CPU Panel Trigger Register0"]
    pub lcd_cpu_tri0: crate::Reg<lcd_cpu_tri0::LCD_CPU_TRI0_SPEC>,
    #[doc = "0x164 - LCD CPU Panel Trigger Register1"]
    pub lcd_cpu_tri1: crate::Reg<lcd_cpu_tri1::LCD_CPU_TRI1_SPEC>,
    #[doc = "0x168 - LCD CPU Panel Trigger Register2"]
    pub lcd_cpu_tri2: crate::Reg<lcd_cpu_tri2::LCD_CPU_TRI2_SPEC>,
    #[doc = "0x16c - LCD CPU Panel Trigger Register3"]
    pub lcd_cpu_tri3: crate::Reg<lcd_cpu_tri3::LCD_CPU_TRI3_SPEC>,
    #[doc = "0x170 - LCD CPU Panel Trigger Register4"]
    pub lcd_cpu_tri4: crate::Reg<lcd_cpu_tri4::LCD_CPU_TRI4_SPEC>,
    #[doc = "0x174 - LCD CPU Panel Trigger Register5"]
    pub lcd_cpu_tri5: crate::Reg<lcd_cpu_tri5::LCD_CPU_TRI5_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0x180 - LCD Color Map Control Register"]
    pub lcd_cmap_ctl: crate::Reg<lcd_cmap_ctl::LCD_CMAP_CTL_SPEC>,
    _reserved34: [u8; 0x0c],
    #[doc = "0x190..0x198 - LCD Color Map Odd Line Register\\[i\\]"]
    pub lcd_cmap_odd: [crate::Reg<lcd_cmap_odd::LCD_CMAP_ODD_SPEC>; 2],
    #[doc = "0x198..0x1a0 - LCD Color Map Even Line Register\\[i\\]"]
    pub lcd_cmap_even: [crate::Reg<lcd_cmap_even::LCD_CMAP_EVEN_SPEC>; 2],
    _reserved36: [u8; 0x50],
    #[doc = "0x1f0 - LCD Safe Period Register"]
    pub lcd_safe_period: crate::Reg<lcd_safe_period::LCD_SAFE_PERIOD_SPEC>,
    _reserved37: [u8; 0x2c],
    #[doc = "0x220..0x228 - LCD LVDS Analog Register \\[i\\]"]
    pub lcd_lvds_ana: [crate::Reg<lcd_lvds_ana::LCD_LVDS_ANA_SPEC>; 2],
    #[doc = "0x228 - FSYNC_GEN_CTRL"]
    pub fsync_gen_ctrl: crate::Reg<fsync_gen_ctrl::FSYNC_GEN_CTRL_SPEC>,
    #[doc = "0x22c - FSYNC_GEN_DLY"]
    pub fsync_gen_dly: crate::Reg<fsync_gen_dly::FSYNC_GEN_DLY_SPEC>,
    #[doc = "0x230 - LCD Sync Control Register"]
    pub lcd_sync_ctl: crate::Reg<lcd_sync_ctl::LCD_SYNC_CTL_SPEC>,
    #[doc = "0x234 - LCD Sync Position Register"]
    pub lcd_sync_pos: crate::Reg<lcd_sync_pos::LCD_SYNC_POS_SPEC>,
    #[doc = "0x238 - LCD Slave Stop Position Register"]
    pub lcd_slave_stop_pos: crate::Reg<lcd_slave_stop_pos::LCD_SLAVE_STOP_POS_SPEC>,
    _reserved43: [u8; 0x01c4],
    #[doc = "0x400..0x800 - LCD Gamma Table Register"]
    pub lcd_gamma_table: [crate::Reg<lcd_gamma_table::LCD_GAMMA_TABLE_SPEC>; 256],
}
#[doc = "lcd_gctl register accessor: an alias for `Reg<LCD_GCTL_SPEC>`"]
pub type LCD_GCTL = crate::Reg<lcd_gctl::LCD_GCTL_SPEC>;
#[doc = "LCD Global Control Register"]
pub mod lcd_gctl;
#[doc = "lcd_gint0 register accessor: an alias for `Reg<LCD_GINT0_SPEC>`"]
pub type LCD_GINT0 = crate::Reg<lcd_gint0::LCD_GINT0_SPEC>;
#[doc = "LCD Global Interrupt Register0"]
pub mod lcd_gint0;
#[doc = "lcd_gint1 register accessor: an alias for `Reg<LCD_GINT1_SPEC>`"]
pub type LCD_GINT1 = crate::Reg<lcd_gint1::LCD_GINT1_SPEC>;
#[doc = "LCD Global Interrupt Register1"]
pub mod lcd_gint1;
#[doc = "lcd_frm_ctl register accessor: an alias for `Reg<LCD_FRM_CTL_SPEC>`"]
pub type LCD_FRM_CTL = crate::Reg<lcd_frm_ctl::LCD_FRM_CTL_SPEC>;
#[doc = "LCD FRM Control Register"]
pub mod lcd_frm_ctl;
#[doc = "lcd_frm_seed register accessor: an alias for `Reg<LCD_FRM_SEED_SPEC>`"]
pub type LCD_FRM_SEED = crate::Reg<lcd_frm_seed::LCD_FRM_SEED_SPEC>;
#[doc = "LCD FRM Seed Register"]
pub mod lcd_frm_seed;
#[doc = "lcd_frm_tab register accessor: an alias for `Reg<LCD_FRM_TAB_SPEC>`"]
pub type LCD_FRM_TAB = crate::Reg<lcd_frm_tab::LCD_FRM_TAB_SPEC>;
#[doc = "LCD FRM Table Register"]
pub mod lcd_frm_tab;
#[doc = "lcd_3d_fifo register accessor: an alias for `Reg<LCD_3D_FIFO_SPEC>`"]
pub type LCD_3D_FIFO = crate::Reg<lcd_3d_fifo::LCD_3D_FIFO_SPEC>;
#[doc = "LCD 3D FIFO Register"]
pub mod lcd_3d_fifo;
#[doc = "lcd_ctl register accessor: an alias for `Reg<LCD_CTL_SPEC>`"]
pub type LCD_CTL = crate::Reg<lcd_ctl::LCD_CTL_SPEC>;
#[doc = "LCD Control Register"]
pub mod lcd_ctl;
#[doc = "lcd_dclk register accessor: an alias for `Reg<LCD_DCLK_SPEC>`"]
pub type LCD_DCLK = crate::Reg<lcd_dclk::LCD_DCLK_SPEC>;
#[doc = "LCD Data Clock Register"]
pub mod lcd_dclk;
#[doc = "lcd_basic0 register accessor: an alias for `Reg<LCD_BASIC0_SPEC>`"]
pub type LCD_BASIC0 = crate::Reg<lcd_basic0::LCD_BASIC0_SPEC>;
#[doc = "LCD Basic Timing Register0"]
pub mod lcd_basic0;
#[doc = "lcd_basic1 register accessor: an alias for `Reg<LCD_BASIC1_SPEC>`"]
pub type LCD_BASIC1 = crate::Reg<lcd_basic1::LCD_BASIC1_SPEC>;
#[doc = "LCD Basic Timing Register1"]
pub mod lcd_basic1;
#[doc = "lcd_basic2 register accessor: an alias for `Reg<LCD_BASIC2_SPEC>`"]
pub type LCD_BASIC2 = crate::Reg<lcd_basic2::LCD_BASIC2_SPEC>;
#[doc = "LCD Basic Timing Register2"]
pub mod lcd_basic2;
#[doc = "lcd_basic3 register accessor: an alias for `Reg<LCD_BASIC3_SPEC>`"]
pub type LCD_BASIC3 = crate::Reg<lcd_basic3::LCD_BASIC3_SPEC>;
#[doc = "LCD Basic Timing Register3"]
pub mod lcd_basic3;
#[doc = "lcd_hv_if register accessor: an alias for `Reg<LCD_HV_IF_SPEC>`"]
pub type LCD_HV_IF = crate::Reg<lcd_hv_if::LCD_HV_IF_SPEC>;
#[doc = "LCD HV Panel Interface Register"]
pub mod lcd_hv_if;
#[doc = "lcd_cpu_if register accessor: an alias for `Reg<LCD_CPU_IF_SPEC>`"]
pub type LCD_CPU_IF = crate::Reg<lcd_cpu_if::LCD_CPU_IF_SPEC>;
#[doc = "LCD CPU Panel Interface Register"]
pub mod lcd_cpu_if;
#[doc = "lcd_cpu_wr register accessor: an alias for `Reg<LCD_CPU_WR_SPEC>`"]
pub type LCD_CPU_WR = crate::Reg<lcd_cpu_wr::LCD_CPU_WR_SPEC>;
#[doc = "LCD CPU Panel Write Data Register"]
pub mod lcd_cpu_wr;
#[doc = "lcd_cpu_rd register accessor: an alias for `Reg<LCD_CPU_RD_SPEC>`"]
pub type LCD_CPU_RD = crate::Reg<lcd_cpu_rd::LCD_CPU_RD_SPEC>;
#[doc = "LCD CPU Panel Read Data Register\\[i\\]"]
pub mod lcd_cpu_rd;
#[doc = "lcd_lvds_if register accessor: an alias for `Reg<LCD_LVDS_IF_SPEC>`"]
pub type LCD_LVDS_IF = crate::Reg<lcd_lvds_if::LCD_LVDS_IF_SPEC>;
#[doc = "LCD LVDS Configure Register"]
pub mod lcd_lvds_if;
#[doc = "lcd_io_pol register accessor: an alias for `Reg<LCD_IO_POL_SPEC>`"]
pub type LCD_IO_POL = crate::Reg<lcd_io_pol::LCD_IO_POL_SPEC>;
#[doc = "LCD IO Polarity Register"]
pub mod lcd_io_pol;
#[doc = "lcd_io_tri register accessor: an alias for `Reg<LCD_IO_TRI_SPEC>`"]
pub type LCD_IO_TRI = crate::Reg<lcd_io_tri::LCD_IO_TRI_SPEC>;
#[doc = "LCD IO Control Register"]
pub mod lcd_io_tri;
#[doc = "lcd_debug register accessor: an alias for `Reg<LCD_DEBUG_SPEC>`"]
pub type LCD_DEBUG = crate::Reg<lcd_debug::LCD_DEBUG_SPEC>;
#[doc = "LCD Debug Register"]
pub mod lcd_debug;
#[doc = "lcd_ceu_ctl register accessor: an alias for `Reg<LCD_CEU_CTL_SPEC>`"]
pub type LCD_CEU_CTL = crate::Reg<lcd_ceu_ctl::LCD_CEU_CTL_SPEC>;
#[doc = "LCD CEU Control Register"]
pub mod lcd_ceu_ctl;
#[doc = "lcd_ceu_coef_mul register accessor: an alias for `Reg<LCD_CEU_COEF_MUL_SPEC>`"]
pub type LCD_CEU_COEF_MUL = crate::Reg<lcd_ceu_coef_mul::LCD_CEU_COEF_MUL_SPEC>;
#[doc = "LCD CEU Coefficient Register0"]
pub mod lcd_ceu_coef_mul;
#[doc = "lcd_ceu_coef_add register accessor: an alias for `Reg<LCD_CEU_COEF_ADD_SPEC>`"]
pub type LCD_CEU_COEF_ADD = crate::Reg<lcd_ceu_coef_add::LCD_CEU_COEF_ADD_SPEC>;
#[doc = "LCD CEU Coefficient Register1"]
pub mod lcd_ceu_coef_add;
#[doc = "lcd_ceu_coef_rang register accessor: an alias for `Reg<LCD_CEU_COEF_RANG_SPEC>`"]
pub type LCD_CEU_COEF_RANG = crate::Reg<lcd_ceu_coef_rang::LCD_CEU_COEF_RANG_SPEC>;
#[doc = "LCD CEU Coefficient Register2"]
pub mod lcd_ceu_coef_rang;
#[doc = "lcd_cpu_tri0 register accessor: an alias for `Reg<LCD_CPU_TRI0_SPEC>`"]
pub type LCD_CPU_TRI0 = crate::Reg<lcd_cpu_tri0::LCD_CPU_TRI0_SPEC>;
#[doc = "LCD CPU Panel Trigger Register0"]
pub mod lcd_cpu_tri0;
#[doc = "lcd_cpu_tri1 register accessor: an alias for `Reg<LCD_CPU_TRI1_SPEC>`"]
pub type LCD_CPU_TRI1 = crate::Reg<lcd_cpu_tri1::LCD_CPU_TRI1_SPEC>;
#[doc = "LCD CPU Panel Trigger Register1"]
pub mod lcd_cpu_tri1;
#[doc = "lcd_cpu_tri2 register accessor: an alias for `Reg<LCD_CPU_TRI2_SPEC>`"]
pub type LCD_CPU_TRI2 = crate::Reg<lcd_cpu_tri2::LCD_CPU_TRI2_SPEC>;
#[doc = "LCD CPU Panel Trigger Register2"]
pub mod lcd_cpu_tri2;
#[doc = "lcd_cpu_tri3 register accessor: an alias for `Reg<LCD_CPU_TRI3_SPEC>`"]
pub type LCD_CPU_TRI3 = crate::Reg<lcd_cpu_tri3::LCD_CPU_TRI3_SPEC>;
#[doc = "LCD CPU Panel Trigger Register3"]
pub mod lcd_cpu_tri3;
#[doc = "lcd_cpu_tri4 register accessor: an alias for `Reg<LCD_CPU_TRI4_SPEC>`"]
pub type LCD_CPU_TRI4 = crate::Reg<lcd_cpu_tri4::LCD_CPU_TRI4_SPEC>;
#[doc = "LCD CPU Panel Trigger Register4"]
pub mod lcd_cpu_tri4;
#[doc = "lcd_cpu_tri5 register accessor: an alias for `Reg<LCD_CPU_TRI5_SPEC>`"]
pub type LCD_CPU_TRI5 = crate::Reg<lcd_cpu_tri5::LCD_CPU_TRI5_SPEC>;
#[doc = "LCD CPU Panel Trigger Register5"]
pub mod lcd_cpu_tri5;
#[doc = "lcd_cmap_ctl register accessor: an alias for `Reg<LCD_CMAP_CTL_SPEC>`"]
pub type LCD_CMAP_CTL = crate::Reg<lcd_cmap_ctl::LCD_CMAP_CTL_SPEC>;
#[doc = "LCD Color Map Control Register"]
pub mod lcd_cmap_ctl;
#[doc = "lcd_cmap_odd register accessor: an alias for `Reg<LCD_CMAP_ODD_SPEC>`"]
pub type LCD_CMAP_ODD = crate::Reg<lcd_cmap_odd::LCD_CMAP_ODD_SPEC>;
#[doc = "LCD Color Map Odd Line Register\\[i\\]"]
pub mod lcd_cmap_odd;
#[doc = "lcd_cmap_even register accessor: an alias for `Reg<LCD_CMAP_EVEN_SPEC>`"]
pub type LCD_CMAP_EVEN = crate::Reg<lcd_cmap_even::LCD_CMAP_EVEN_SPEC>;
#[doc = "LCD Color Map Even Line Register\\[i\\]"]
pub mod lcd_cmap_even;
#[doc = "lcd_safe_period register accessor: an alias for `Reg<LCD_SAFE_PERIOD_SPEC>`"]
pub type LCD_SAFE_PERIOD = crate::Reg<lcd_safe_period::LCD_SAFE_PERIOD_SPEC>;
#[doc = "LCD Safe Period Register"]
pub mod lcd_safe_period;
#[doc = "lcd_lvds_ana register accessor: an alias for `Reg<LCD_LVDS_ANA_SPEC>`"]
pub type LCD_LVDS_ANA = crate::Reg<lcd_lvds_ana::LCD_LVDS_ANA_SPEC>;
#[doc = "LCD LVDS Analog Register \\[i\\]"]
pub mod lcd_lvds_ana;
#[doc = "fsync_gen_ctrl register accessor: an alias for `Reg<FSYNC_GEN_CTRL_SPEC>`"]
pub type FSYNC_GEN_CTRL = crate::Reg<fsync_gen_ctrl::FSYNC_GEN_CTRL_SPEC>;
#[doc = "FSYNC_GEN_CTRL"]
pub mod fsync_gen_ctrl;
#[doc = "fsync_gen_dly register accessor: an alias for `Reg<FSYNC_GEN_DLY_SPEC>`"]
pub type FSYNC_GEN_DLY = crate::Reg<fsync_gen_dly::FSYNC_GEN_DLY_SPEC>;
#[doc = "FSYNC_GEN_DLY"]
pub mod fsync_gen_dly;
#[doc = "lcd_sync_ctl register accessor: an alias for `Reg<LCD_SYNC_CTL_SPEC>`"]
pub type LCD_SYNC_CTL = crate::Reg<lcd_sync_ctl::LCD_SYNC_CTL_SPEC>;
#[doc = "LCD Sync Control Register"]
pub mod lcd_sync_ctl;
#[doc = "lcd_sync_pos register accessor: an alias for `Reg<LCD_SYNC_POS_SPEC>`"]
pub type LCD_SYNC_POS = crate::Reg<lcd_sync_pos::LCD_SYNC_POS_SPEC>;
#[doc = "LCD Sync Position Register"]
pub mod lcd_sync_pos;
#[doc = "lcd_slave_stop_pos register accessor: an alias for `Reg<LCD_SLAVE_STOP_POS_SPEC>`"]
pub type LCD_SLAVE_STOP_POS = crate::Reg<lcd_slave_stop_pos::LCD_SLAVE_STOP_POS_SPEC>;
#[doc = "LCD Slave Stop Position Register"]
pub mod lcd_slave_stop_pos;
#[doc = "lcd_gamma_table register accessor: an alias for `Reg<LCD_GAMMA_TABLE_SPEC>`"]
pub type LCD_GAMMA_TABLE = crate::Reg<lcd_gamma_table::LCD_GAMMA_TABLE_SPEC>;
#[doc = "LCD Gamma Table Register"]
pub mod lcd_gamma_table;
