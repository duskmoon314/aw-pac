#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lcd_gctl: LCD_GCTL,
    lcd_gint0: LCD_GINT0,
    lcd_gint1: LCD_GINT1,
    _reserved3: [u8; 0x04],
    lcd_frm_ctl: LCD_FRM_CTL,
    lcd_frm_seed: [LCD_FRM_SEED; 6],
    lcd_frm_tab: [LCD_FRM_TAB; 4],
    lcd_3d_fifo: LCD_3D_FIFO,
    lcd_ctl: LCD_CTL,
    lcd_dclk: LCD_DCLK,
    lcd_basic0: LCD_BASIC0,
    lcd_basic1: LCD_BASIC1,
    lcd_basic2: LCD_BASIC2,
    lcd_basic3: LCD_BASIC3,
    lcd_hv_if: LCD_HV_IF,
    _reserved14: [u8; 0x04],
    lcd_cpu_if: LCD_CPU_IF,
    lcd_cpu_wr: LCD_CPU_WR,
    lcd_cpu_rd: [LCD_CPU_RD; 2],
    _reserved17: [u8; 0x14],
    lcd_lvds_if: LCD_LVDS_IF,
    lcd_io_pol: LCD_IO_POL,
    lcd_io_tri: LCD_IO_TRI,
    _reserved20: [u8; 0x6c],
    lcd_debug: LCD_DEBUG,
    lcd_ceu_ctl: LCD_CEU_CTL,
    _reserved22: [u8; 0x0c],
    lcd_ceu_coef_mul: [LCD_CEU_COEF_MUL; 3],
    lcd_ceu_coef_add: (),
    _reserved24: [u8; 0x24],
    lcd_ceu_coef_rang: [LCD_CEU_COEF_RANG; 3],
    _reserved25: [u8; 0x14],
    lcd_cpu_tri0: LCD_CPU_TRI0,
    lcd_cpu_tri1: LCD_CPU_TRI1,
    lcd_cpu_tri2: LCD_CPU_TRI2,
    lcd_cpu_tri3: LCD_CPU_TRI3,
    lcd_cpu_tri4: LCD_CPU_TRI4,
    lcd_cpu_tri5: LCD_CPU_TRI5,
    _reserved31: [u8; 0x08],
    lcd_cmap_ctl: LCD_CMAP_CTL,
    _reserved32: [u8; 0x0c],
    lcd_cmap_odd: [LCD_CMAP_ODD; 2],
    lcd_cmap_even: [LCD_CMAP_EVEN; 2],
    _reserved34: [u8; 0x50],
    lcd_safe_period: LCD_SAFE_PERIOD,
    _reserved35: [u8; 0x2c],
    lcd_lvds_ana: [LCD_LVDS_ANA; 2],
    fsync_gen_ctrl: FSYNC_GEN_CTRL,
    fsync_gen_dly: FSYNC_GEN_DLY,
    lcd_sync_ctl: LCD_SYNC_CTL,
    lcd_sync_pos: LCD_SYNC_POS,
    lcd_slave_stop_pos: LCD_SLAVE_STOP_POS,
    _reserved41: [u8; 0x01c4],
    lcd_gamma_table: [LCD_GAMMA_TABLE; 256],
}
impl RegisterBlock {
    #[doc = "0x00 - LCD Global Control Register"]
    #[inline(always)]
    pub const fn lcd_gctl(&self) -> &LCD_GCTL {
        &self.lcd_gctl
    }
    #[doc = "0x04 - LCD Global Interrupt Register0"]
    #[inline(always)]
    pub const fn lcd_gint0(&self) -> &LCD_GINT0 {
        &self.lcd_gint0
    }
    #[doc = "0x08 - LCD Global Interrupt Register1"]
    #[inline(always)]
    pub const fn lcd_gint1(&self) -> &LCD_GINT1 {
        &self.lcd_gint1
    }
    #[doc = "0x10 - LCD FRM Control Register"]
    #[inline(always)]
    pub const fn lcd_frm_ctl(&self) -> &LCD_FRM_CTL {
        &self.lcd_frm_ctl
    }
    #[doc = "0x14..0x2c - LCD FRM Seed Register"]
    #[inline(always)]
    pub const fn lcd_frm_seed(&self, n: usize) -> &LCD_FRM_SEED {
        &self.lcd_frm_seed[n]
    }
    #[doc = "0x2c..0x3c - LCD FRM Table Register"]
    #[inline(always)]
    pub const fn lcd_frm_tab(&self, n: usize) -> &LCD_FRM_TAB {
        &self.lcd_frm_tab[n]
    }
    #[doc = "0x3c - LCD 3D FIFO Register"]
    #[inline(always)]
    pub const fn lcd_3d_fifo(&self) -> &LCD_3D_FIFO {
        &self.lcd_3d_fifo
    }
    #[doc = "0x40 - LCD Control Register"]
    #[inline(always)]
    pub const fn lcd_ctl(&self) -> &LCD_CTL {
        &self.lcd_ctl
    }
    #[doc = "0x44 - LCD Data Clock Register"]
    #[inline(always)]
    pub const fn lcd_dclk(&self) -> &LCD_DCLK {
        &self.lcd_dclk
    }
    #[doc = "0x48 - LCD Basic Timing Register0"]
    #[inline(always)]
    pub const fn lcd_basic0(&self) -> &LCD_BASIC0 {
        &self.lcd_basic0
    }
    #[doc = "0x4c - LCD Basic Timing Register1"]
    #[inline(always)]
    pub const fn lcd_basic1(&self) -> &LCD_BASIC1 {
        &self.lcd_basic1
    }
    #[doc = "0x50 - LCD Basic Timing Register2"]
    #[inline(always)]
    pub const fn lcd_basic2(&self) -> &LCD_BASIC2 {
        &self.lcd_basic2
    }
    #[doc = "0x54 - LCD Basic Timing Register3"]
    #[inline(always)]
    pub const fn lcd_basic3(&self) -> &LCD_BASIC3 {
        &self.lcd_basic3
    }
    #[doc = "0x58 - LCD HV Panel Interface Register"]
    #[inline(always)]
    pub const fn lcd_hv_if(&self) -> &LCD_HV_IF {
        &self.lcd_hv_if
    }
    #[doc = "0x60 - LCD CPU Panel Interface Register"]
    #[inline(always)]
    pub const fn lcd_cpu_if(&self) -> &LCD_CPU_IF {
        &self.lcd_cpu_if
    }
    #[doc = "0x64 - LCD CPU Panel Write Data Register"]
    #[inline(always)]
    pub const fn lcd_cpu_wr(&self) -> &LCD_CPU_WR {
        &self.lcd_cpu_wr
    }
    #[doc = "0x68..0x70 - LCD CPU Panel Read Data Register\\[i\\]"]
    #[inline(always)]
    pub const fn lcd_cpu_rd(&self, n: usize) -> &LCD_CPU_RD {
        &self.lcd_cpu_rd[n]
    }
    #[doc = "0x84 - LCD LVDS Configure Register"]
    #[inline(always)]
    pub const fn lcd_lvds_if(&self) -> &LCD_LVDS_IF {
        &self.lcd_lvds_if
    }
    #[doc = "0x88 - LCD IO Polarity Register"]
    #[inline(always)]
    pub const fn lcd_io_pol(&self) -> &LCD_IO_POL {
        &self.lcd_io_pol
    }
    #[doc = "0x8c - LCD IO Control Register"]
    #[inline(always)]
    pub const fn lcd_io_tri(&self) -> &LCD_IO_TRI {
        &self.lcd_io_tri
    }
    #[doc = "0xfc - LCD Debug Register"]
    #[inline(always)]
    pub const fn lcd_debug(&self) -> &LCD_DEBUG {
        &self.lcd_debug
    }
    #[doc = "0x100 - LCD CEU Control Register"]
    #[inline(always)]
    pub const fn lcd_ceu_ctl(&self) -> &LCD_CEU_CTL {
        &self.lcd_ceu_ctl
    }
    #[doc = "0x110..0x11c - LCD CEU Coefficient Register0"]
    #[inline(always)]
    pub const fn lcd_ceu_coef_mul(&self, n: usize) -> &LCD_CEU_COEF_MUL {
        &self.lcd_ceu_coef_mul[n]
    }
    #[doc = "0x11c..0x128 - LCD CEU Coefficient Register1"]
    #[inline(always)]
    pub const fn lcd_ceu_coef_add(&self, n: usize) -> &LCD_CEU_COEF_ADD {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(284)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x140..0x14c - LCD CEU Coefficient Register2"]
    #[inline(always)]
    pub const fn lcd_ceu_coef_rang(&self, n: usize) -> &LCD_CEU_COEF_RANG {
        &self.lcd_ceu_coef_rang[n]
    }
    #[doc = "0x160 - LCD CPU Panel Trigger Register0"]
    #[inline(always)]
    pub const fn lcd_cpu_tri0(&self) -> &LCD_CPU_TRI0 {
        &self.lcd_cpu_tri0
    }
    #[doc = "0x164 - LCD CPU Panel Trigger Register1"]
    #[inline(always)]
    pub const fn lcd_cpu_tri1(&self) -> &LCD_CPU_TRI1 {
        &self.lcd_cpu_tri1
    }
    #[doc = "0x168 - LCD CPU Panel Trigger Register2"]
    #[inline(always)]
    pub const fn lcd_cpu_tri2(&self) -> &LCD_CPU_TRI2 {
        &self.lcd_cpu_tri2
    }
    #[doc = "0x16c - LCD CPU Panel Trigger Register3"]
    #[inline(always)]
    pub const fn lcd_cpu_tri3(&self) -> &LCD_CPU_TRI3 {
        &self.lcd_cpu_tri3
    }
    #[doc = "0x170 - LCD CPU Panel Trigger Register4"]
    #[inline(always)]
    pub const fn lcd_cpu_tri4(&self) -> &LCD_CPU_TRI4 {
        &self.lcd_cpu_tri4
    }
    #[doc = "0x174 - LCD CPU Panel Trigger Register5"]
    #[inline(always)]
    pub const fn lcd_cpu_tri5(&self) -> &LCD_CPU_TRI5 {
        &self.lcd_cpu_tri5
    }
    #[doc = "0x180 - LCD Color Map Control Register"]
    #[inline(always)]
    pub const fn lcd_cmap_ctl(&self) -> &LCD_CMAP_CTL {
        &self.lcd_cmap_ctl
    }
    #[doc = "0x190..0x198 - LCD Color Map Odd Line Register\\[i\\]"]
    #[inline(always)]
    pub const fn lcd_cmap_odd(&self, n: usize) -> &LCD_CMAP_ODD {
        &self.lcd_cmap_odd[n]
    }
    #[doc = "0x198..0x1a0 - LCD Color Map Even Line Register\\[i\\]"]
    #[inline(always)]
    pub const fn lcd_cmap_even(&self, n: usize) -> &LCD_CMAP_EVEN {
        &self.lcd_cmap_even[n]
    }
    #[doc = "0x1f0 - LCD Safe Period Register"]
    #[inline(always)]
    pub const fn lcd_safe_period(&self) -> &LCD_SAFE_PERIOD {
        &self.lcd_safe_period
    }
    #[doc = "0x220..0x228 - LCD LVDS Analog Register \\[i\\]"]
    #[inline(always)]
    pub const fn lcd_lvds_ana(&self, n: usize) -> &LCD_LVDS_ANA {
        &self.lcd_lvds_ana[n]
    }
    #[doc = "0x228 - FSYNC_GEN_CTRL"]
    #[inline(always)]
    pub const fn fsync_gen_ctrl(&self) -> &FSYNC_GEN_CTRL {
        &self.fsync_gen_ctrl
    }
    #[doc = "0x22c - FSYNC_GEN_DLY"]
    #[inline(always)]
    pub const fn fsync_gen_dly(&self) -> &FSYNC_GEN_DLY {
        &self.fsync_gen_dly
    }
    #[doc = "0x230 - LCD Sync Control Register"]
    #[inline(always)]
    pub const fn lcd_sync_ctl(&self) -> &LCD_SYNC_CTL {
        &self.lcd_sync_ctl
    }
    #[doc = "0x234 - LCD Sync Position Register"]
    #[inline(always)]
    pub const fn lcd_sync_pos(&self) -> &LCD_SYNC_POS {
        &self.lcd_sync_pos
    }
    #[doc = "0x238 - LCD Slave Stop Position Register"]
    #[inline(always)]
    pub const fn lcd_slave_stop_pos(&self) -> &LCD_SLAVE_STOP_POS {
        &self.lcd_slave_stop_pos
    }
    #[doc = "0x400..0x800 - LCD Gamma Table Register"]
    #[inline(always)]
    pub const fn lcd_gamma_table(&self, n: usize) -> &LCD_GAMMA_TABLE {
        &self.lcd_gamma_table[n]
    }
}
#[doc = "lcd_gctl (rw) register accessor: LCD Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_gctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_gctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_gctl`] module"]
pub type LCD_GCTL = crate::Reg<lcd_gctl::LCD_GCTL_SPEC>;
#[doc = "LCD Global Control Register"]
pub mod lcd_gctl;
#[doc = "lcd_gint0 (rw) register accessor: LCD Global Interrupt Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_gint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_gint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_gint0`] module"]
pub type LCD_GINT0 = crate::Reg<lcd_gint0::LCD_GINT0_SPEC>;
#[doc = "LCD Global Interrupt Register0"]
pub mod lcd_gint0;
#[doc = "lcd_gint1 (rw) register accessor: LCD Global Interrupt Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_gint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_gint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_gint1`] module"]
pub type LCD_GINT1 = crate::Reg<lcd_gint1::LCD_GINT1_SPEC>;
#[doc = "LCD Global Interrupt Register1"]
pub mod lcd_gint1;
#[doc = "lcd_frm_ctl (rw) register accessor: LCD FRM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_frm_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_frm_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_frm_ctl`] module"]
pub type LCD_FRM_CTL = crate::Reg<lcd_frm_ctl::LCD_FRM_CTL_SPEC>;
#[doc = "LCD FRM Control Register"]
pub mod lcd_frm_ctl;
#[doc = "lcd_frm_seed (rw) register accessor: LCD FRM Seed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_frm_seed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_frm_seed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_frm_seed`] module"]
pub type LCD_FRM_SEED = crate::Reg<lcd_frm_seed::LCD_FRM_SEED_SPEC>;
#[doc = "LCD FRM Seed Register"]
pub mod lcd_frm_seed;
#[doc = "lcd_frm_tab (rw) register accessor: LCD FRM Table Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_frm_tab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_frm_tab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_frm_tab`] module"]
pub type LCD_FRM_TAB = crate::Reg<lcd_frm_tab::LCD_FRM_TAB_SPEC>;
#[doc = "LCD FRM Table Register"]
pub mod lcd_frm_tab;
#[doc = "lcd_3d_fifo (rw) register accessor: LCD 3D FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_3d_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_3d_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_3d_fifo`] module"]
pub type LCD_3D_FIFO = crate::Reg<lcd_3d_fifo::LCD_3D_FIFO_SPEC>;
#[doc = "LCD 3D FIFO Register"]
pub mod lcd_3d_fifo;
#[doc = "lcd_ctl (rw) register accessor: LCD Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctl`] module"]
pub type LCD_CTL = crate::Reg<lcd_ctl::LCD_CTL_SPEC>;
#[doc = "LCD Control Register"]
pub mod lcd_ctl;
#[doc = "lcd_dclk (rw) register accessor: LCD Data Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_dclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_dclk`] module"]
pub type LCD_DCLK = crate::Reg<lcd_dclk::LCD_DCLK_SPEC>;
#[doc = "LCD Data Clock Register"]
pub mod lcd_dclk;
#[doc = "lcd_basic0 (rw) register accessor: LCD Basic Timing Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_basic0`] module"]
pub type LCD_BASIC0 = crate::Reg<lcd_basic0::LCD_BASIC0_SPEC>;
#[doc = "LCD Basic Timing Register0"]
pub mod lcd_basic0;
#[doc = "lcd_basic1 (rw) register accessor: LCD Basic Timing Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_basic1`] module"]
pub type LCD_BASIC1 = crate::Reg<lcd_basic1::LCD_BASIC1_SPEC>;
#[doc = "LCD Basic Timing Register1"]
pub mod lcd_basic1;
#[doc = "lcd_basic2 (rw) register accessor: LCD Basic Timing Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_basic2`] module"]
pub type LCD_BASIC2 = crate::Reg<lcd_basic2::LCD_BASIC2_SPEC>;
#[doc = "LCD Basic Timing Register2"]
pub mod lcd_basic2;
#[doc = "lcd_basic3 (rw) register accessor: LCD Basic Timing Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_basic3`] module"]
pub type LCD_BASIC3 = crate::Reg<lcd_basic3::LCD_BASIC3_SPEC>;
#[doc = "LCD Basic Timing Register3"]
pub mod lcd_basic3;
#[doc = "lcd_hv_if (rw) register accessor: LCD HV Panel Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_hv_if::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_hv_if::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_hv_if`] module"]
pub type LCD_HV_IF = crate::Reg<lcd_hv_if::LCD_HV_IF_SPEC>;
#[doc = "LCD HV Panel Interface Register"]
pub mod lcd_hv_if;
#[doc = "lcd_cpu_if (rw) register accessor: LCD CPU Panel Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_if::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_if::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_if`] module"]
pub type LCD_CPU_IF = crate::Reg<lcd_cpu_if::LCD_CPU_IF_SPEC>;
#[doc = "LCD CPU Panel Interface Register"]
pub mod lcd_cpu_if;
#[doc = "lcd_cpu_wr (rw) register accessor: LCD CPU Panel Write Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_wr`] module"]
pub type LCD_CPU_WR = crate::Reg<lcd_cpu_wr::LCD_CPU_WR_SPEC>;
#[doc = "LCD CPU Panel Write Data Register"]
pub mod lcd_cpu_wr;
#[doc = "lcd_cpu_rd (rw) register accessor: LCD CPU Panel Read Data Register\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_rd`] module"]
pub type LCD_CPU_RD = crate::Reg<lcd_cpu_rd::LCD_CPU_RD_SPEC>;
#[doc = "LCD CPU Panel Read Data Register\\[i\\]"]
pub mod lcd_cpu_rd;
#[doc = "lcd_lvds_if (rw) register accessor: LCD LVDS Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_lvds_if::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_lvds_if::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lvds_if`] module"]
pub type LCD_LVDS_IF = crate::Reg<lcd_lvds_if::LCD_LVDS_IF_SPEC>;
#[doc = "LCD LVDS Configure Register"]
pub mod lcd_lvds_if;
#[doc = "lcd_io_pol (rw) register accessor: LCD IO Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_io_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_io_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_io_pol`] module"]
pub type LCD_IO_POL = crate::Reg<lcd_io_pol::LCD_IO_POL_SPEC>;
#[doc = "LCD IO Polarity Register"]
pub mod lcd_io_pol;
#[doc = "lcd_io_tri (rw) register accessor: LCD IO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_io_tri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_io_tri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_io_tri`] module"]
pub type LCD_IO_TRI = crate::Reg<lcd_io_tri::LCD_IO_TRI_SPEC>;
#[doc = "LCD IO Control Register"]
pub mod lcd_io_tri;
#[doc = "lcd_debug (rw) register accessor: LCD Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_debug`] module"]
pub type LCD_DEBUG = crate::Reg<lcd_debug::LCD_DEBUG_SPEC>;
#[doc = "LCD Debug Register"]
pub mod lcd_debug;
#[doc = "lcd_ceu_ctl (rw) register accessor: LCD CEU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ceu_ctl`] module"]
pub type LCD_CEU_CTL = crate::Reg<lcd_ceu_ctl::LCD_CEU_CTL_SPEC>;
#[doc = "LCD CEU Control Register"]
pub mod lcd_ceu_ctl;
#[doc = "lcd_ceu_coef_mul (rw) register accessor: LCD CEU Coefficient Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_coef_mul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_coef_mul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ceu_coef_mul`] module"]
pub type LCD_CEU_COEF_MUL = crate::Reg<lcd_ceu_coef_mul::LCD_CEU_COEF_MUL_SPEC>;
#[doc = "LCD CEU Coefficient Register0"]
pub mod lcd_ceu_coef_mul;
#[doc = "lcd_ceu_coef_add (rw) register accessor: LCD CEU Coefficient Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_coef_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_coef_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ceu_coef_add`] module"]
pub type LCD_CEU_COEF_ADD = crate::Reg<lcd_ceu_coef_add::LCD_CEU_COEF_ADD_SPEC>;
#[doc = "LCD CEU Coefficient Register1"]
pub mod lcd_ceu_coef_add;
#[doc = "lcd_ceu_coef_rang (rw) register accessor: LCD CEU Coefficient Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_coef_rang::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_coef_rang::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ceu_coef_rang`] module"]
pub type LCD_CEU_COEF_RANG = crate::Reg<lcd_ceu_coef_rang::LCD_CEU_COEF_RANG_SPEC>;
#[doc = "LCD CEU Coefficient Register2"]
pub mod lcd_ceu_coef_rang;
#[doc = "lcd_cpu_tri0 (rw) register accessor: LCD CPU Panel Trigger Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_tri0`] module"]
pub type LCD_CPU_TRI0 = crate::Reg<lcd_cpu_tri0::LCD_CPU_TRI0_SPEC>;
#[doc = "LCD CPU Panel Trigger Register0"]
pub mod lcd_cpu_tri0;
#[doc = "lcd_cpu_tri1 (rw) register accessor: LCD CPU Panel Trigger Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_tri1`] module"]
pub type LCD_CPU_TRI1 = crate::Reg<lcd_cpu_tri1::LCD_CPU_TRI1_SPEC>;
#[doc = "LCD CPU Panel Trigger Register1"]
pub mod lcd_cpu_tri1;
#[doc = "lcd_cpu_tri2 (rw) register accessor: LCD CPU Panel Trigger Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_tri2`] module"]
pub type LCD_CPU_TRI2 = crate::Reg<lcd_cpu_tri2::LCD_CPU_TRI2_SPEC>;
#[doc = "LCD CPU Panel Trigger Register2"]
pub mod lcd_cpu_tri2;
#[doc = "lcd_cpu_tri3 (rw) register accessor: LCD CPU Panel Trigger Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_tri3`] module"]
pub type LCD_CPU_TRI3 = crate::Reg<lcd_cpu_tri3::LCD_CPU_TRI3_SPEC>;
#[doc = "LCD CPU Panel Trigger Register3"]
pub mod lcd_cpu_tri3;
#[doc = "lcd_cpu_tri4 (rw) register accessor: LCD CPU Panel Trigger Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_tri4`] module"]
pub type LCD_CPU_TRI4 = crate::Reg<lcd_cpu_tri4::LCD_CPU_TRI4_SPEC>;
#[doc = "LCD CPU Panel Trigger Register4"]
pub mod lcd_cpu_tri4;
#[doc = "lcd_cpu_tri5 (rw) register accessor: LCD CPU Panel Trigger Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cpu_tri5`] module"]
pub type LCD_CPU_TRI5 = crate::Reg<lcd_cpu_tri5::LCD_CPU_TRI5_SPEC>;
#[doc = "LCD CPU Panel Trigger Register5"]
pub mod lcd_cpu_tri5;
#[doc = "lcd_cmap_ctl (rw) register accessor: LCD Color Map Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmap_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmap_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cmap_ctl`] module"]
pub type LCD_CMAP_CTL = crate::Reg<lcd_cmap_ctl::LCD_CMAP_CTL_SPEC>;
#[doc = "LCD Color Map Control Register"]
pub mod lcd_cmap_ctl;
#[doc = "lcd_cmap_odd (rw) register accessor: LCD Color Map Odd Line Register\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmap_odd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmap_odd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cmap_odd`] module"]
pub type LCD_CMAP_ODD = crate::Reg<lcd_cmap_odd::LCD_CMAP_ODD_SPEC>;
#[doc = "LCD Color Map Odd Line Register\\[i\\]"]
pub mod lcd_cmap_odd;
#[doc = "lcd_cmap_even (rw) register accessor: LCD Color Map Even Line Register\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmap_even::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmap_even::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cmap_even`] module"]
pub type LCD_CMAP_EVEN = crate::Reg<lcd_cmap_even::LCD_CMAP_EVEN_SPEC>;
#[doc = "LCD Color Map Even Line Register\\[i\\]"]
pub mod lcd_cmap_even;
#[doc = "lcd_safe_period (rw) register accessor: LCD Safe Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_safe_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_safe_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_safe_period`] module"]
pub type LCD_SAFE_PERIOD = crate::Reg<lcd_safe_period::LCD_SAFE_PERIOD_SPEC>;
#[doc = "LCD Safe Period Register"]
pub mod lcd_safe_period;
#[doc = "lcd_lvds_ana (rw) register accessor: LCD LVDS Analog Register \\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_lvds_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_lvds_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lvds_ana`] module"]
pub type LCD_LVDS_ANA = crate::Reg<lcd_lvds_ana::LCD_LVDS_ANA_SPEC>;
#[doc = "LCD LVDS Analog Register \\[i\\]"]
pub mod lcd_lvds_ana;
#[doc = "fsync_gen_ctrl (rw) register accessor: FSYNC_GEN_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsync_gen_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsync_gen_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsync_gen_ctrl`] module"]
pub type FSYNC_GEN_CTRL = crate::Reg<fsync_gen_ctrl::FSYNC_GEN_CTRL_SPEC>;
#[doc = "FSYNC_GEN_CTRL"]
pub mod fsync_gen_ctrl;
#[doc = "fsync_gen_dly (rw) register accessor: FSYNC_GEN_DLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsync_gen_dly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsync_gen_dly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsync_gen_dly`] module"]
pub type FSYNC_GEN_DLY = crate::Reg<fsync_gen_dly::FSYNC_GEN_DLY_SPEC>;
#[doc = "FSYNC_GEN_DLY"]
pub mod fsync_gen_dly;
#[doc = "lcd_sync_ctl (rw) register accessor: LCD Sync Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_sync_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_sync_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_sync_ctl`] module"]
pub type LCD_SYNC_CTL = crate::Reg<lcd_sync_ctl::LCD_SYNC_CTL_SPEC>;
#[doc = "LCD Sync Control Register"]
pub mod lcd_sync_ctl;
#[doc = "lcd_sync_pos (rw) register accessor: LCD Sync Position Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_sync_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_sync_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_sync_pos`] module"]
pub type LCD_SYNC_POS = crate::Reg<lcd_sync_pos::LCD_SYNC_POS_SPEC>;
#[doc = "LCD Sync Position Register"]
pub mod lcd_sync_pos;
#[doc = "lcd_slave_stop_pos (rw) register accessor: LCD Slave Stop Position Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_slave_stop_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_slave_stop_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_slave_stop_pos`] module"]
pub type LCD_SLAVE_STOP_POS = crate::Reg<lcd_slave_stop_pos::LCD_SLAVE_STOP_POS_SPEC>;
#[doc = "LCD Slave Stop Position Register"]
pub mod lcd_slave_stop_pos;
#[doc = "lcd_gamma_table (rw) register accessor: LCD Gamma Table Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_gamma_table::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_gamma_table::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_gamma_table`] module"]
pub type LCD_GAMMA_TABLE = crate::Reg<lcd_gamma_table::LCD_GAMMA_TABLE_SPEC>;
#[doc = "LCD Gamma Table Register"]
pub mod lcd_gamma_table;
