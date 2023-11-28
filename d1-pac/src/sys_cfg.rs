#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    dsp_boot_rammap: DSP_BOOT_RAMMAP,
    _reserved1: [u8; 0x18],
    ver: VER,
    _reserved2: [u8; 0x08],
    emac_ephy_clk0: EMAC_EPHY_CLK0,
    _reserved3: [u8; 0x011c],
    sys_ldo_ctrl: SYS_LDO_CTRL,
    _reserved4: [u8; 0x0c],
    rescal_ctrl: RESCAL_CTRL,
    _reserved5: [u8; 0x04],
    res240_ctrl: RES240_CTRL,
    rescal_status: RESCAL_STATUS,
}
impl RegisterBlock {
    #[doc = "0x08 - DSP Boot SRAM Remap Control Register"]
    #[inline(always)]
    pub const fn dsp_boot_rammap(&self) -> &DSP_BOOT_RAMMAP {
        &self.dsp_boot_rammap
    }
    #[doc = "0x24 - Version Register"]
    #[inline(always)]
    pub const fn ver(&self) -> &VER {
        &self.ver
    }
    #[doc = "0x30 - EMAC-EPHY Clock Register 0"]
    #[inline(always)]
    pub const fn emac_ephy_clk0(&self) -> &EMAC_EPHY_CLK0 {
        &self.emac_ephy_clk0
    }
    #[doc = "0x150 - System LDO Control Register"]
    #[inline(always)]
    pub const fn sys_ldo_ctrl(&self) -> &SYS_LDO_CTRL {
        &self.sys_ldo_ctrl
    }
    #[doc = "0x160 - Resistor Calibration Control Register"]
    #[inline(always)]
    pub const fn rescal_ctrl(&self) -> &RESCAL_CTRL {
        &self.rescal_ctrl
    }
    #[doc = "0x168 - 240ohms Resistor Manual Control Register"]
    #[inline(always)]
    pub const fn res240_ctrl(&self) -> &RES240_CTRL {
        &self.res240_ctrl
    }
    #[doc = "0x16c - Resistor Calibration Status Register"]
    #[inline(always)]
    pub const fn rescal_status(&self) -> &RESCAL_STATUS {
        &self.rescal_status
    }
}
#[doc = "dsp_boot_rammap (rw) register accessor: DSP Boot SRAM Remap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_boot_rammap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_boot_rammap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_boot_rammap`] module"]
pub type DSP_BOOT_RAMMAP = crate::Reg<dsp_boot_rammap::DSP_BOOT_RAMMAP_SPEC>;
#[doc = "DSP Boot SRAM Remap Control Register"]
pub mod dsp_boot_rammap;
#[doc = "ver (r) register accessor: Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver`] module"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Version Register"]
pub mod ver;
#[doc = "emac_ephy_clk0 (rw) register accessor: EMAC-EPHY Clock Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_ephy_clk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_ephy_clk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_ephy_clk0`] module"]
pub type EMAC_EPHY_CLK0 = crate::Reg<emac_ephy_clk0::EMAC_EPHY_CLK0_SPEC>;
#[doc = "EMAC-EPHY Clock Register 0"]
pub mod emac_ephy_clk0;
#[doc = "sys_ldo_ctrl (rw) register accessor: System LDO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ldo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ldo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ldo_ctrl`] module"]
pub type SYS_LDO_CTRL = crate::Reg<sys_ldo_ctrl::SYS_LDO_CTRL_SPEC>;
#[doc = "System LDO Control Register"]
pub mod sys_ldo_ctrl;
#[doc = "rescal_ctrl (rw) register accessor: Resistor Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rescal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rescal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rescal_ctrl`] module"]
pub type RESCAL_CTRL = crate::Reg<rescal_ctrl::RESCAL_CTRL_SPEC>;
#[doc = "Resistor Calibration Control Register"]
pub mod rescal_ctrl;
#[doc = "res240_ctrl (rw) register accessor: 240ohms Resistor Manual Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res240_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res240_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res240_ctrl`] module"]
pub type RES240_CTRL = crate::Reg<res240_ctrl::RES240_CTRL_SPEC>;
#[doc = "240ohms Resistor Manual Control Register"]
pub mod res240_ctrl;
#[doc = "rescal_status (r) register accessor: Resistor Calibration Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rescal_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rescal_status`] module"]
pub type RESCAL_STATUS = crate::Reg<rescal_status::RESCAL_STATUS_SPEC>;
#[doc = "Resistor Calibration Status Register"]
pub mod rescal_status;
