#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD Global Control Register"]
    pub lcd_gctl_reg: crate::Reg<lcd_gctl_reg::LCD_GCTL_REG_SPEC>,
    #[doc = "0x04 - LCD Global Interrupt Register0"]
    pub lcd_gint0_reg: crate::Reg<lcd_gint0_reg::LCD_GINT0_REG_SPEC>,
    #[doc = "0x08 - LCD Global Interrupt Register1"]
    pub lcd_gint1_reg: crate::Reg<lcd_gint1_reg::LCD_GINT1_REG_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - LCD FRM Control Register"]
    pub lcd_frm_ctl_reg: crate::Reg<lcd_frm_ctl_reg::LCD_FRM_CTL_REG_SPEC>,
    #[doc = "0x14..0x2c - LCD FRM Seed Register"]
    pub lcd_frm_seed_reg: [crate::Reg<lcd_frm_seed_reg::LCD_FRM_SEED_REG_SPEC>; 6],
    #[doc = "0x2c..0x3c - LCD FRM Table Register"]
    pub lcd_frm_tab_reg: [crate::Reg<lcd_frm_tab_reg::LCD_FRM_TAB_REG_SPEC>; 4],
    #[doc = "0x3c - LCD 3D FIFO Register"]
    pub lcd_3d_fifo_reg: crate::Reg<lcd_3d_fifo_reg::LCD_3D_FIFO_REG_SPEC>,
    #[doc = "0x40 - LCD Control Register"]
    pub lcd_ctl_reg: crate::Reg<lcd_ctl_reg::LCD_CTL_REG_SPEC>,
    #[doc = "0x44 - LCD Data Clock Register"]
    pub lcd_dclk_reg: crate::Reg<lcd_dclk_reg::LCD_DCLK_REG_SPEC>,
    #[doc = "0x48 - LCD Basic Timing Register0"]
    pub lcd_basic0_reg: crate::Reg<lcd_basic0_reg::LCD_BASIC0_REG_SPEC>,
    #[doc = "0x4c - LCD Basic Timing Register1"]
    pub lcd_basic1_reg: crate::Reg<lcd_basic1_reg::LCD_BASIC1_REG_SPEC>,
    #[doc = "0x50 - LCD Basic Timing Register2"]
    pub lcd_basic2_reg: crate::Reg<lcd_basic2_reg::LCD_BASIC2_REG_SPEC>,
    #[doc = "0x54 - LCD Basic Timing Register3"]
    pub lcd_basic3_reg: crate::Reg<lcd_basic3_reg::LCD_BASIC3_REG_SPEC>,
    #[doc = "0x58 - LCD HV Panel Interface Register"]
    pub lcd_hv_if_reg: crate::Reg<lcd_hv_if_reg::LCD_HV_IF_REG_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x60 - LCD CPU Panel Interface Register"]
    pub lcd_cpu_if_reg: crate::Reg<lcd_cpu_if_reg::LCD_CPU_IF_REG_SPEC>,
    #[doc = "0x64 - LCD CPU Panel Write Data Register"]
    pub lcd_cpu_wr_reg: crate::Reg<lcd_cpu_wr_reg::LCD_CPU_WR_REG_SPEC>,
    #[doc = "0x68 - LCD CPU Panel Read Data Register0"]
    pub lcd_cpu_rd0_reg: crate::Reg<lcd_cpu_rd0_reg::LCD_CPU_RD0_REG_SPEC>,
    #[doc = "0x6c - LCD CPU Panel Read Data Register1"]
    pub lcd_cpu_rd1_reg: crate::Reg<lcd_cpu_rd1_reg::LCD_CPU_RD1_REG_SPEC>,
    _reserved18: [u8; 0x14],
    #[doc = "0x84 - LCD LVDS Configure Register"]
    pub lcd_lvds_if_reg: crate::Reg<lcd_lvds_if_reg::LCD_LVDS_IF_REG_SPEC>,
    #[doc = "0x88 - LCD IO Polarity Register"]
    pub lcd_io_pol_reg: crate::Reg<lcd_io_pol_reg::LCD_IO_POL_REG_SPEC>,
    #[doc = "0x8c - LCD IO Control Register"]
    pub lcd_io_tri_reg: crate::Reg<lcd_io_tri_reg::LCD_IO_TRI_REG_SPEC>,
    _reserved21: [u8; 0x6c],
    #[doc = "0xfc - LCD Debug Register"]
    pub lcd_debug_reg: crate::Reg<lcd_debug_reg::LCD_DEBUG_REG_SPEC>,
    #[doc = "0x100 - LCD CEU Control Register"]
    pub lcd_ceu_ctl_reg: crate::Reg<lcd_ceu_ctl_reg::LCD_CEU_CTL_REG_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x110..0x11c - LCD CEU Coefficient Register0"]
    pub lcd_ceu_coef_mul_reg: [crate::Reg<lcd_ceu_coef_mul_reg::LCD_CEU_COEF_MUL_REG_SPEC>; 3],
    #[doc = "0x11c - LCD CEU Coefficient Register1"]
    pub lcd_ceu_coef_add_reg0: crate::Reg<lcd_ceu_coef_add_reg::LCD_CEU_COEF_ADD_REG_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0x12c - LCD CEU Coefficient Register1"]
    pub lcd_ceu_coef_add_reg1: crate::Reg<lcd_ceu_coef_add_reg::LCD_CEU_COEF_ADD_REG_SPEC>,
    _reserved26: [u8; 0x0c],
    #[doc = "0x13c - LCD CEU Coefficient Register1"]
    pub lcd_ceu_coef_add_reg2: crate::Reg<lcd_ceu_coef_add_reg::LCD_CEU_COEF_ADD_REG_SPEC>,
    #[doc = "0x140..0x14c - LCD CEU Coefficient Register2"]
    pub lcd_ceu_coef_rang_reg: [crate::Reg<lcd_ceu_coef_rang_reg::LCD_CEU_COEF_RANG_REG_SPEC>; 3],
    _reserved28: [u8; 0x14],
    #[doc = "0x160 - LCD CPU Panel Trigger Register0"]
    pub lcd_cpu_tri0_reg: crate::Reg<lcd_cpu_tri0_reg::LCD_CPU_TRI0_REG_SPEC>,
    #[doc = "0x164 - LCD CPU Panel Trigger Register1"]
    pub lcd_cpu_tri1_reg: crate::Reg<lcd_cpu_tri1_reg::LCD_CPU_TRI1_REG_SPEC>,
    #[doc = "0x168 - LCD CPU Panel Trigger Register2"]
    pub lcd_cpu_tri2_reg: crate::Reg<lcd_cpu_tri2_reg::LCD_CPU_TRI2_REG_SPEC>,
    #[doc = "0x16c - LCD CPU Panel Trigger Register3"]
    pub lcd_cpu_tri3_reg: crate::Reg<lcd_cpu_tri3_reg::LCD_CPU_TRI3_REG_SPEC>,
    #[doc = "0x170 - LCD CPU Panel Trigger Register4"]
    pub lcd_cpu_tri4_reg: crate::Reg<lcd_cpu_tri4_reg::LCD_CPU_TRI4_REG_SPEC>,
    #[doc = "0x174 - LCD CPU Panel Trigger Register5"]
    pub lcd_cpu_tri5_reg: crate::Reg<lcd_cpu_tri5_reg::LCD_CPU_TRI5_REG_SPEC>,
    _reserved34: [u8; 0x08],
    #[doc = "0x180 - LCD Color Map Control Register"]
    pub lcd_cmap_ctl_reg: crate::Reg<lcd_cmap_ctl_reg::LCD_CMAP_CTL_REG_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0x190 - LCD Color Map Odd Line Register0"]
    pub lcd_cmap_odd0_reg: crate::Reg<lcd_cmap_odd0_reg::LCD_CMAP_ODD0_REG_SPEC>,
    #[doc = "0x194 - LCD Color Map Odd Line Register1"]
    pub lcd_cmap_odd1_reg: crate::Reg<lcd_cmap_odd1_reg::LCD_CMAP_ODD1_REG_SPEC>,
    #[doc = "0x198 - LCD Color Map Even Line Register0"]
    pub lcd_cmap_even0_reg: crate::Reg<lcd_cmap_even0_reg::LCD_CMAP_EVEN0_REG_SPEC>,
    #[doc = "0x19c - LCD Color Map Even Line Register1"]
    pub lcd_cmap_even1_reg: crate::Reg<lcd_cmap_even1_reg::LCD_CMAP_EVEN1_REG_SPEC>,
    _reserved39: [u8; 0x50],
    #[doc = "0x1f0 - LCD Safe Period Register"]
    pub lcd_safe_period_reg: crate::Reg<lcd_safe_period_reg::LCD_SAFE_PERIOD_REG_SPEC>,
    _reserved40: [u8; 0x2c],
    #[doc = "0x220 - LCD LVDS Analog Register 0"]
    pub lcd_lvds0_ana_reg: crate::Reg<lcd_lvds0_ana_reg::LCD_LVDS0_ANA_REG_SPEC>,
    #[doc = "0x224 - LCD LVDS Analog Register 1"]
    pub lcd_lvds1_ana_reg: crate::Reg<lcd_lvds1_ana_reg::LCD_LVDS1_ANA_REG_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x230 - LCD Sync Control Register"]
    pub lcd_sync_ctl_reg: crate::Reg<lcd_sync_ctl_reg::LCD_SYNC_CTL_REG_SPEC>,
    #[doc = "0x234 - LCD Sync Position Register"]
    pub lcd_sync_pos_reg: crate::Reg<lcd_sync_pos_reg::LCD_SYNC_POS_REG_SPEC>,
    #[doc = "0x238 - LCD Slave Stop Position Register"]
    pub lcd_slave_stop_pos_reg: crate::Reg<lcd_slave_stop_pos_reg::LCD_SLAVE_STOP_POS_REG_SPEC>,
    _reserved45: [u8; 0x08],
    #[doc = "0x244 - LCD LVDS1 IF Register"]
    pub lcd_lvds1_if_reg: crate::Reg<lcd_lvds1_if_reg::LCD_LVDS1_IF_REG_SPEC>,
    _reserved46: [u8; 0x01b8],
    #[doc = "0x400..0x800 - LCD Gamma Table Register"]
    pub lcd_gamma_table_reg: [crate::Reg<lcd_gamma_table_reg::LCD_GAMMA_TABLE_REG_SPEC>; 256],
}
#[doc = "lcd_gctl_reg register accessor: an alias for `Reg<LCD_GCTL_REG_SPEC>`"]
pub type LCD_GCTL_REG = crate::Reg<lcd_gctl_reg::LCD_GCTL_REG_SPEC>;
#[doc = "LCD Global Control Register"]
pub mod lcd_gctl_reg;
#[doc = "lcd_gint0_reg register accessor: an alias for `Reg<LCD_GINT0_REG_SPEC>`"]
pub type LCD_GINT0_REG = crate::Reg<lcd_gint0_reg::LCD_GINT0_REG_SPEC>;
#[doc = "LCD Global Interrupt Register0"]
pub mod lcd_gint0_reg;
#[doc = "lcd_gint1_reg register accessor: an alias for `Reg<LCD_GINT1_REG_SPEC>`"]
pub type LCD_GINT1_REG = crate::Reg<lcd_gint1_reg::LCD_GINT1_REG_SPEC>;
#[doc = "LCD Global Interrupt Register1"]
pub mod lcd_gint1_reg;
#[doc = "lcd_frm_ctl_reg register accessor: an alias for `Reg<LCD_FRM_CTL_REG_SPEC>`"]
pub type LCD_FRM_CTL_REG = crate::Reg<lcd_frm_ctl_reg::LCD_FRM_CTL_REG_SPEC>;
#[doc = "LCD FRM Control Register"]
pub mod lcd_frm_ctl_reg;
#[doc = "lcd_frm_seed_reg register accessor: an alias for `Reg<LCD_FRM_SEED_REG_SPEC>`"]
pub type LCD_FRM_SEED_REG = crate::Reg<lcd_frm_seed_reg::LCD_FRM_SEED_REG_SPEC>;
#[doc = "LCD FRM Seed Register"]
pub mod lcd_frm_seed_reg;
#[doc = "lcd_frm_tab_reg register accessor: an alias for `Reg<LCD_FRM_TAB_REG_SPEC>`"]
pub type LCD_FRM_TAB_REG = crate::Reg<lcd_frm_tab_reg::LCD_FRM_TAB_REG_SPEC>;
#[doc = "LCD FRM Table Register"]
pub mod lcd_frm_tab_reg;
#[doc = "lcd_3d_fifo_reg register accessor: an alias for `Reg<LCD_3D_FIFO_REG_SPEC>`"]
pub type LCD_3D_FIFO_REG = crate::Reg<lcd_3d_fifo_reg::LCD_3D_FIFO_REG_SPEC>;
#[doc = "LCD 3D FIFO Register"]
pub mod lcd_3d_fifo_reg;
#[doc = "lcd_ctl_reg register accessor: an alias for `Reg<LCD_CTL_REG_SPEC>`"]
pub type LCD_CTL_REG = crate::Reg<lcd_ctl_reg::LCD_CTL_REG_SPEC>;
#[doc = "LCD Control Register"]
pub mod lcd_ctl_reg;
#[doc = "lcd_dclk_reg register accessor: an alias for `Reg<LCD_DCLK_REG_SPEC>`"]
pub type LCD_DCLK_REG = crate::Reg<lcd_dclk_reg::LCD_DCLK_REG_SPEC>;
#[doc = "LCD Data Clock Register"]
pub mod lcd_dclk_reg;
#[doc = "lcd_basic0_reg register accessor: an alias for `Reg<LCD_BASIC0_REG_SPEC>`"]
pub type LCD_BASIC0_REG = crate::Reg<lcd_basic0_reg::LCD_BASIC0_REG_SPEC>;
#[doc = "LCD Basic Timing Register0"]
pub mod lcd_basic0_reg;
#[doc = "lcd_basic1_reg register accessor: an alias for `Reg<LCD_BASIC1_REG_SPEC>`"]
pub type LCD_BASIC1_REG = crate::Reg<lcd_basic1_reg::LCD_BASIC1_REG_SPEC>;
#[doc = "LCD Basic Timing Register1"]
pub mod lcd_basic1_reg;
#[doc = "lcd_basic2_reg register accessor: an alias for `Reg<LCD_BASIC2_REG_SPEC>`"]
pub type LCD_BASIC2_REG = crate::Reg<lcd_basic2_reg::LCD_BASIC2_REG_SPEC>;
#[doc = "LCD Basic Timing Register2"]
pub mod lcd_basic2_reg;
#[doc = "lcd_basic3_reg register accessor: an alias for `Reg<LCD_BASIC3_REG_SPEC>`"]
pub type LCD_BASIC3_REG = crate::Reg<lcd_basic3_reg::LCD_BASIC3_REG_SPEC>;
#[doc = "LCD Basic Timing Register3"]
pub mod lcd_basic3_reg;
#[doc = "lcd_hv_if_reg register accessor: an alias for `Reg<LCD_HV_IF_REG_SPEC>`"]
pub type LCD_HV_IF_REG = crate::Reg<lcd_hv_if_reg::LCD_HV_IF_REG_SPEC>;
#[doc = "LCD HV Panel Interface Register"]
pub mod lcd_hv_if_reg;
#[doc = "lcd_cpu_if_reg register accessor: an alias for `Reg<LCD_CPU_IF_REG_SPEC>`"]
pub type LCD_CPU_IF_REG = crate::Reg<lcd_cpu_if_reg::LCD_CPU_IF_REG_SPEC>;
#[doc = "LCD CPU Panel Interface Register"]
pub mod lcd_cpu_if_reg;
#[doc = "lcd_cpu_wr_reg register accessor: an alias for `Reg<LCD_CPU_WR_REG_SPEC>`"]
pub type LCD_CPU_WR_REG = crate::Reg<lcd_cpu_wr_reg::LCD_CPU_WR_REG_SPEC>;
#[doc = "LCD CPU Panel Write Data Register"]
pub mod lcd_cpu_wr_reg;
#[doc = "lcd_cpu_rd0_reg register accessor: an alias for `Reg<LCD_CPU_RD0_REG_SPEC>`"]
pub type LCD_CPU_RD0_REG = crate::Reg<lcd_cpu_rd0_reg::LCD_CPU_RD0_REG_SPEC>;
#[doc = "LCD CPU Panel Read Data Register0"]
pub mod lcd_cpu_rd0_reg;
#[doc = "lcd_cpu_rd1_reg register accessor: an alias for `Reg<LCD_CPU_RD1_REG_SPEC>`"]
pub type LCD_CPU_RD1_REG = crate::Reg<lcd_cpu_rd1_reg::LCD_CPU_RD1_REG_SPEC>;
#[doc = "LCD CPU Panel Read Data Register1"]
pub mod lcd_cpu_rd1_reg;
#[doc = "lcd_lvds_if_reg register accessor: an alias for `Reg<LCD_LVDS_IF_REG_SPEC>`"]
pub type LCD_LVDS_IF_REG = crate::Reg<lcd_lvds_if_reg::LCD_LVDS_IF_REG_SPEC>;
#[doc = "LCD LVDS Configure Register"]
pub mod lcd_lvds_if_reg;
#[doc = "lcd_io_pol_reg register accessor: an alias for `Reg<LCD_IO_POL_REG_SPEC>`"]
pub type LCD_IO_POL_REG = crate::Reg<lcd_io_pol_reg::LCD_IO_POL_REG_SPEC>;
#[doc = "LCD IO Polarity Register"]
pub mod lcd_io_pol_reg;
#[doc = "lcd_io_tri_reg register accessor: an alias for `Reg<LCD_IO_TRI_REG_SPEC>`"]
pub type LCD_IO_TRI_REG = crate::Reg<lcd_io_tri_reg::LCD_IO_TRI_REG_SPEC>;
#[doc = "LCD IO Control Register"]
pub mod lcd_io_tri_reg;
#[doc = "lcd_debug_reg register accessor: an alias for `Reg<LCD_DEBUG_REG_SPEC>`"]
pub type LCD_DEBUG_REG = crate::Reg<lcd_debug_reg::LCD_DEBUG_REG_SPEC>;
#[doc = "LCD Debug Register"]
pub mod lcd_debug_reg;
#[doc = "lcd_ceu_ctl_reg register accessor: an alias for `Reg<LCD_CEU_CTL_REG_SPEC>`"]
pub type LCD_CEU_CTL_REG = crate::Reg<lcd_ceu_ctl_reg::LCD_CEU_CTL_REG_SPEC>;
#[doc = "LCD CEU Control Register"]
pub mod lcd_ceu_ctl_reg;
#[doc = "lcd_ceu_coef_mul_reg register accessor: an alias for `Reg<LCD_CEU_COEF_MUL_REG_SPEC>`"]
pub type LCD_CEU_COEF_MUL_REG = crate::Reg<lcd_ceu_coef_mul_reg::LCD_CEU_COEF_MUL_REG_SPEC>;
#[doc = "LCD CEU Coefficient Register0"]
pub mod lcd_ceu_coef_mul_reg;
#[doc = "lcd_ceu_coef_add_reg register accessor: an alias for `Reg<LCD_CEU_COEF_ADD_REG_SPEC>`"]
pub type LCD_CEU_COEF_ADD_REG = crate::Reg<lcd_ceu_coef_add_reg::LCD_CEU_COEF_ADD_REG_SPEC>;
#[doc = "LCD CEU Coefficient Register1"]
pub mod lcd_ceu_coef_add_reg;
#[doc = "lcd_ceu_coef_rang_reg register accessor: an alias for `Reg<LCD_CEU_COEF_RANG_REG_SPEC>`"]
pub type LCD_CEU_COEF_RANG_REG = crate::Reg<lcd_ceu_coef_rang_reg::LCD_CEU_COEF_RANG_REG_SPEC>;
#[doc = "LCD CEU Coefficient Register2"]
pub mod lcd_ceu_coef_rang_reg;
#[doc = "lcd_cpu_tri0_reg register accessor: an alias for `Reg<LCD_CPU_TRI0_REG_SPEC>`"]
pub type LCD_CPU_TRI0_REG = crate::Reg<lcd_cpu_tri0_reg::LCD_CPU_TRI0_REG_SPEC>;
#[doc = "LCD CPU Panel Trigger Register0"]
pub mod lcd_cpu_tri0_reg;
#[doc = "lcd_cpu_tri1_reg register accessor: an alias for `Reg<LCD_CPU_TRI1_REG_SPEC>`"]
pub type LCD_CPU_TRI1_REG = crate::Reg<lcd_cpu_tri1_reg::LCD_CPU_TRI1_REG_SPEC>;
#[doc = "LCD CPU Panel Trigger Register1"]
pub mod lcd_cpu_tri1_reg;
#[doc = "lcd_cpu_tri2_reg register accessor: an alias for `Reg<LCD_CPU_TRI2_REG_SPEC>`"]
pub type LCD_CPU_TRI2_REG = crate::Reg<lcd_cpu_tri2_reg::LCD_CPU_TRI2_REG_SPEC>;
#[doc = "LCD CPU Panel Trigger Register2"]
pub mod lcd_cpu_tri2_reg;
#[doc = "lcd_cpu_tri3_reg register accessor: an alias for `Reg<LCD_CPU_TRI3_REG_SPEC>`"]
pub type LCD_CPU_TRI3_REG = crate::Reg<lcd_cpu_tri3_reg::LCD_CPU_TRI3_REG_SPEC>;
#[doc = "LCD CPU Panel Trigger Register3"]
pub mod lcd_cpu_tri3_reg;
#[doc = "lcd_cpu_tri4_reg register accessor: an alias for `Reg<LCD_CPU_TRI4_REG_SPEC>`"]
pub type LCD_CPU_TRI4_REG = crate::Reg<lcd_cpu_tri4_reg::LCD_CPU_TRI4_REG_SPEC>;
#[doc = "LCD CPU Panel Trigger Register4"]
pub mod lcd_cpu_tri4_reg;
#[doc = "lcd_cpu_tri5_reg register accessor: an alias for `Reg<LCD_CPU_TRI5_REG_SPEC>`"]
pub type LCD_CPU_TRI5_REG = crate::Reg<lcd_cpu_tri5_reg::LCD_CPU_TRI5_REG_SPEC>;
#[doc = "LCD CPU Panel Trigger Register5"]
pub mod lcd_cpu_tri5_reg;
#[doc = "lcd_cmap_ctl_reg register accessor: an alias for `Reg<LCD_CMAP_CTL_REG_SPEC>`"]
pub type LCD_CMAP_CTL_REG = crate::Reg<lcd_cmap_ctl_reg::LCD_CMAP_CTL_REG_SPEC>;
#[doc = "LCD Color Map Control Register"]
pub mod lcd_cmap_ctl_reg;
#[doc = "lcd_cmap_odd0_reg register accessor: an alias for `Reg<LCD_CMAP_ODD0_REG_SPEC>`"]
pub type LCD_CMAP_ODD0_REG = crate::Reg<lcd_cmap_odd0_reg::LCD_CMAP_ODD0_REG_SPEC>;
#[doc = "LCD Color Map Odd Line Register0"]
pub mod lcd_cmap_odd0_reg;
#[doc = "lcd_cmap_odd1_reg register accessor: an alias for `Reg<LCD_CMAP_ODD1_REG_SPEC>`"]
pub type LCD_CMAP_ODD1_REG = crate::Reg<lcd_cmap_odd1_reg::LCD_CMAP_ODD1_REG_SPEC>;
#[doc = "LCD Color Map Odd Line Register1"]
pub mod lcd_cmap_odd1_reg;
#[doc = "lcd_cmap_even0_reg register accessor: an alias for `Reg<LCD_CMAP_EVEN0_REG_SPEC>`"]
pub type LCD_CMAP_EVEN0_REG = crate::Reg<lcd_cmap_even0_reg::LCD_CMAP_EVEN0_REG_SPEC>;
#[doc = "LCD Color Map Even Line Register0"]
pub mod lcd_cmap_even0_reg;
#[doc = "lcd_cmap_even1_reg register accessor: an alias for `Reg<LCD_CMAP_EVEN1_REG_SPEC>`"]
pub type LCD_CMAP_EVEN1_REG = crate::Reg<lcd_cmap_even1_reg::LCD_CMAP_EVEN1_REG_SPEC>;
#[doc = "LCD Color Map Even Line Register1"]
pub mod lcd_cmap_even1_reg;
#[doc = "lcd_safe_period_reg register accessor: an alias for `Reg<LCD_SAFE_PERIOD_REG_SPEC>`"]
pub type LCD_SAFE_PERIOD_REG = crate::Reg<lcd_safe_period_reg::LCD_SAFE_PERIOD_REG_SPEC>;
#[doc = "LCD Safe Period Register"]
pub mod lcd_safe_period_reg;
#[doc = "lcd_lvds0_ana_reg register accessor: an alias for `Reg<LCD_LVDS0_ANA_REG_SPEC>`"]
pub type LCD_LVDS0_ANA_REG = crate::Reg<lcd_lvds0_ana_reg::LCD_LVDS0_ANA_REG_SPEC>;
#[doc = "LCD LVDS Analog Register 0"]
pub mod lcd_lvds0_ana_reg;
#[doc = "lcd_lvds1_ana_reg register accessor: an alias for `Reg<LCD_LVDS1_ANA_REG_SPEC>`"]
pub type LCD_LVDS1_ANA_REG = crate::Reg<lcd_lvds1_ana_reg::LCD_LVDS1_ANA_REG_SPEC>;
#[doc = "LCD LVDS Analog Register 1"]
pub mod lcd_lvds1_ana_reg;
#[doc = "lcd_sync_ctl_reg register accessor: an alias for `Reg<LCD_SYNC_CTL_REG_SPEC>`"]
pub type LCD_SYNC_CTL_REG = crate::Reg<lcd_sync_ctl_reg::LCD_SYNC_CTL_REG_SPEC>;
#[doc = "LCD Sync Control Register"]
pub mod lcd_sync_ctl_reg;
#[doc = "lcd_sync_pos_reg register accessor: an alias for `Reg<LCD_SYNC_POS_REG_SPEC>`"]
pub type LCD_SYNC_POS_REG = crate::Reg<lcd_sync_pos_reg::LCD_SYNC_POS_REG_SPEC>;
#[doc = "LCD Sync Position Register"]
pub mod lcd_sync_pos_reg;
#[doc = "lcd_slave_stop_pos_reg register accessor: an alias for `Reg<LCD_SLAVE_STOP_POS_REG_SPEC>`"]
pub type LCD_SLAVE_STOP_POS_REG = crate::Reg<lcd_slave_stop_pos_reg::LCD_SLAVE_STOP_POS_REG_SPEC>;
#[doc = "LCD Slave Stop Position Register"]
pub mod lcd_slave_stop_pos_reg;
#[doc = "lcd_lvds1_if_reg register accessor: an alias for `Reg<LCD_LVDS1_IF_REG_SPEC>`"]
pub type LCD_LVDS1_IF_REG = crate::Reg<lcd_lvds1_if_reg::LCD_LVDS1_IF_REG_SPEC>;
#[doc = "LCD LVDS1 IF Register"]
pub mod lcd_lvds1_if_reg;
#[doc = "lcd_gamma_table_reg register accessor: an alias for `Reg<LCD_GAMMA_TABLE_REG_SPEC>`"]
pub type LCD_GAMMA_TABLE_REG = crate::Reg<lcd_gamma_table_reg::LCD_GAMMA_TABLE_REG_SPEC>;
#[doc = "LCD Gamma Table Register"]
pub mod lcd_gamma_table_reg;
