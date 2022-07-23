#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - DSP Boot SRAM Remap Control Register"]
    pub dsp_boot_rammap: crate::Reg<dsp_boot_rammap::DSP_BOOT_RAMMAP_SPEC>,
    _reserved1: [u8; 0x18],
    #[doc = "0x24 - Version Register"]
    pub ver: crate::Reg<ver::VER_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x30 - EMAC-EPHY Clock Register 0"]
    pub emac_ephy_clk0: crate::Reg<emac_ephy_clk0::EMAC_EPHY_CLK0_SPEC>,
    _reserved3: [u8; 0x011c],
    #[doc = "0x150 - System LDO Control Register"]
    pub sys_ldo_ctrl: crate::Reg<sys_ldo_ctrl::SYS_LDO_CTRL_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x160 - Resistor Calibration Control Register"]
    pub rescal_ctrl: crate::Reg<rescal_ctrl::RESCAL_CTRL_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x168 - 240ohms Resistor Manual Control Register"]
    pub res240_ctrl: crate::Reg<res240_ctrl::RES240_CTRL_SPEC>,
    #[doc = "0x16c - Resistor Calibration Status Register"]
    pub rescal_status: crate::Reg<rescal_status::RESCAL_STATUS_SPEC>,
}
#[doc = "dsp_boot_rammap register accessor: an alias for `Reg<DSP_BOOT_RAMMAP_SPEC>`"]
pub type DSP_BOOT_RAMMAP = crate::Reg<dsp_boot_rammap::DSP_BOOT_RAMMAP_SPEC>;
#[doc = "DSP Boot SRAM Remap Control Register"]
pub mod dsp_boot_rammap;
#[doc = "ver register accessor: an alias for `Reg<VER_SPEC>`"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Version Register"]
pub mod ver;
#[doc = "emac_ephy_clk0 register accessor: an alias for `Reg<EMAC_EPHY_CLK0_SPEC>`"]
pub type EMAC_EPHY_CLK0 = crate::Reg<emac_ephy_clk0::EMAC_EPHY_CLK0_SPEC>;
#[doc = "EMAC-EPHY Clock Register 0"]
pub mod emac_ephy_clk0;
#[doc = "sys_ldo_ctrl register accessor: an alias for `Reg<SYS_LDO_CTRL_SPEC>`"]
pub type SYS_LDO_CTRL = crate::Reg<sys_ldo_ctrl::SYS_LDO_CTRL_SPEC>;
#[doc = "System LDO Control Register"]
pub mod sys_ldo_ctrl;
#[doc = "rescal_ctrl register accessor: an alias for `Reg<RESCAL_CTRL_SPEC>`"]
pub type RESCAL_CTRL = crate::Reg<rescal_ctrl::RESCAL_CTRL_SPEC>;
#[doc = "Resistor Calibration Control Register"]
pub mod rescal_ctrl;
#[doc = "res240_ctrl register accessor: an alias for `Reg<RES240_CTRL_SPEC>`"]
pub type RES240_CTRL = crate::Reg<res240_ctrl::RES240_CTRL_SPEC>;
#[doc = "240ohms Resistor Manual Control Register"]
pub mod res240_ctrl;
#[doc = "rescal_status register accessor: an alias for `Reg<RESCAL_STATUS_SPEC>`"]
pub type RESCAL_STATUS = crate::Reg<rescal_status::RESCAL_STATUS_SPEC>;
#[doc = "Resistor Calibration Status Register"]
pub mod rescal_status;
