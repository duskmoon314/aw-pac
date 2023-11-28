#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    smhc_ctrl: SMHC_CTRL,
    smhc_clkdiv: SMHC_CLKDIV,
    smhc_tmout: SMHC_TMOUT,
    smhc_ctype: SMHC_CTYPE,
    smhc_blksiz: SMHC_BLKSIZ,
    smhc_bytcnt: SMHC_BYTCNT,
    smhc_cmd: SMHC_CMD,
    smhc_cmdarg: SMHC_CMDARG,
    smhc_resp0: SMHC_RESP0,
    smhc_resp1: SMHC_RESP1,
    smhc_resp2: SMHC_RESP2,
    smhc_resp3: SMHC_RESP3,
    smhc_intmask: SMHC_INTMASK,
    smhc_mintsts: SMHC_MINTSTS,
    smhc_rintsts: SMHC_RINTSTS,
    smhc_status: SMHC_STATUS,
    smhc_fifoth: SMHC_FIFOTH,
    smhc_funs: SMHC_FUNS,
    smhc_tbc0: SMHC_TBC0,
    smhc_tbc1: SMHC_TBC1,
    smhc_dbgc: SMHC_DBGC,
    smhc_csdc: SMHC_CSDC,
    smhc_a12a: SMHC_A12A,
    smhc_ntsr: SMHC_NTSR,
    _reserved24: [u8; 0x18],
    smhc_hwrst: SMHC_HWRST,
    _reserved25: [u8; 0x04],
    smhc_idmac: SMHC_IDMAC,
    smhc_dlba: SMHC_DLBA,
    smhc_idst: SMHC_IDST,
    smhc_idie: SMHC_IDIE,
    _reserved29: [u8; 0x70],
    smhc_thld: SMHC_THLD,
    smhc_sfc: SMHC_SFC,
    smhc_a23a: SMHC_A23A,
    emmc_ddr_sbit_det: EMMC_DDR_SBIT_DET,
    _reserved33: [u8; 0x28],
    smhc_ext_cmd: SMHC_EXT_CMD,
    smhc_ext_resp: SMHC_EXT_RESP,
    smhc_drv_dl: SMHC_DRV_DL,
    smhc_smap_dl: SMHC_SMAP_DL,
    smhc_ds_dl: SMHC_DS_DL,
    smhc_hs400_dl: SMHC_HS400_DL,
    _reserved39: [u8; 0xb0],
    smhc_fifo: SMHC_FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn smhc_ctrl(&self) -> &SMHC_CTRL {
        &self.smhc_ctrl
    }
    #[doc = "0x04 - Clock Control Register"]
    #[inline(always)]
    pub const fn smhc_clkdiv(&self) -> &SMHC_CLKDIV {
        &self.smhc_clkdiv
    }
    #[doc = "0x08 - Time Out Register"]
    #[inline(always)]
    pub const fn smhc_tmout(&self) -> &SMHC_TMOUT {
        &self.smhc_tmout
    }
    #[doc = "0x0c - Bus Width Register"]
    #[inline(always)]
    pub const fn smhc_ctype(&self) -> &SMHC_CTYPE {
        &self.smhc_ctype
    }
    #[doc = "0x10 - Block Size Register"]
    #[inline(always)]
    pub const fn smhc_blksiz(&self) -> &SMHC_BLKSIZ {
        &self.smhc_blksiz
    }
    #[doc = "0x14 - Byte Count Register"]
    #[inline(always)]
    pub const fn smhc_bytcnt(&self) -> &SMHC_BYTCNT {
        &self.smhc_bytcnt
    }
    #[doc = "0x18 - Command Register"]
    #[inline(always)]
    pub const fn smhc_cmd(&self) -> &SMHC_CMD {
        &self.smhc_cmd
    }
    #[doc = "0x1c - Command Argument Register"]
    #[inline(always)]
    pub const fn smhc_cmdarg(&self) -> &SMHC_CMDARG {
        &self.smhc_cmdarg
    }
    #[doc = "0x20 - Response 0 Register"]
    #[inline(always)]
    pub const fn smhc_resp0(&self) -> &SMHC_RESP0 {
        &self.smhc_resp0
    }
    #[doc = "0x24 - Response 1 Register"]
    #[inline(always)]
    pub const fn smhc_resp1(&self) -> &SMHC_RESP1 {
        &self.smhc_resp1
    }
    #[doc = "0x28 - Response 2 Register"]
    #[inline(always)]
    pub const fn smhc_resp2(&self) -> &SMHC_RESP2 {
        &self.smhc_resp2
    }
    #[doc = "0x2c - Response 3 Register"]
    #[inline(always)]
    pub const fn smhc_resp3(&self) -> &SMHC_RESP3 {
        &self.smhc_resp3
    }
    #[doc = "0x30 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn smhc_intmask(&self) -> &SMHC_INTMASK {
        &self.smhc_intmask
    }
    #[doc = "0x34 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn smhc_mintsts(&self) -> &SMHC_MINTSTS {
        &self.smhc_mintsts
    }
    #[doc = "0x38 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn smhc_rintsts(&self) -> &SMHC_RINTSTS {
        &self.smhc_rintsts
    }
    #[doc = "0x3c - Status Register"]
    #[inline(always)]
    pub const fn smhc_status(&self) -> &SMHC_STATUS {
        &self.smhc_status
    }
    #[doc = "0x40 - FIFO Water Level Register"]
    #[inline(always)]
    pub const fn smhc_fifoth(&self) -> &SMHC_FIFOTH {
        &self.smhc_fifoth
    }
    #[doc = "0x44 - FIFO Function Select Register"]
    #[inline(always)]
    pub const fn smhc_funs(&self) -> &SMHC_FUNS {
        &self.smhc_funs
    }
    #[doc = "0x48 - Transferred Byte Count between Controller and Card"]
    #[inline(always)]
    pub const fn smhc_tbc0(&self) -> &SMHC_TBC0 {
        &self.smhc_tbc0
    }
    #[doc = "0x4c - Transferred Byte Count between Host Memory and Internal FIFO"]
    #[inline(always)]
    pub const fn smhc_tbc1(&self) -> &SMHC_TBC1 {
        &self.smhc_tbc1
    }
    #[doc = "0x50 - Current Debug Control Register"]
    #[inline(always)]
    pub const fn smhc_dbgc(&self) -> &SMHC_DBGC {
        &self.smhc_dbgc
    }
    #[doc = "0x54 - CRC Status Detect Control Registers"]
    #[inline(always)]
    pub const fn smhc_csdc(&self) -> &SMHC_CSDC {
        &self.smhc_csdc
    }
    #[doc = "0x58 - Auto Command 12 Argument Register"]
    #[inline(always)]
    pub const fn smhc_a12a(&self) -> &SMHC_A12A {
        &self.smhc_a12a
    }
    #[doc = "0x5c - SD New Timing Set Register"]
    #[inline(always)]
    pub const fn smhc_ntsr(&self) -> &SMHC_NTSR {
        &self.smhc_ntsr
    }
    #[doc = "0x78 - Hardware Reset Register"]
    #[inline(always)]
    pub const fn smhc_hwrst(&self) -> &SMHC_HWRST {
        &self.smhc_hwrst
    }
    #[doc = "0x80 - IDMAC Control Register"]
    #[inline(always)]
    pub const fn smhc_idmac(&self) -> &SMHC_IDMAC {
        &self.smhc_idmac
    }
    #[doc = "0x84 - Descriptor List Base Address Register"]
    #[inline(always)]
    pub const fn smhc_dlba(&self) -> &SMHC_DLBA {
        &self.smhc_dlba
    }
    #[doc = "0x88 - IDMAC Status Register"]
    #[inline(always)]
    pub const fn smhc_idst(&self) -> &SMHC_IDST {
        &self.smhc_idst
    }
    #[doc = "0x8c - IDMAC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn smhc_idie(&self) -> &SMHC_IDIE {
        &self.smhc_idie
    }
    #[doc = "0x100 - Card Threshold Control Register"]
    #[inline(always)]
    pub const fn smhc_thld(&self) -> &SMHC_THLD {
        &self.smhc_thld
    }
    #[doc = "0x104 - Sample FIFO Control Register"]
    #[inline(always)]
    pub const fn smhc_sfc(&self) -> &SMHC_SFC {
        &self.smhc_sfc
    }
    #[doc = "0x108 - Auto Command 23 Argument Register"]
    #[inline(always)]
    pub const fn smhc_a23a(&self) -> &SMHC_A23A {
        &self.smhc_a23a
    }
    #[doc = "0x10c - eMMC4.5 DDR Start Bit Detection Control Register"]
    #[inline(always)]
    pub const fn emmc_ddr_sbit_det(&self) -> &EMMC_DDR_SBIT_DET {
        &self.emmc_ddr_sbit_det
    }
    #[doc = "0x138 - Extended Command Register"]
    #[inline(always)]
    pub const fn smhc_ext_cmd(&self) -> &SMHC_EXT_CMD {
        &self.smhc_ext_cmd
    }
    #[doc = "0x13c - Extended Response Register"]
    #[inline(always)]
    pub const fn smhc_ext_resp(&self) -> &SMHC_EXT_RESP {
        &self.smhc_ext_resp
    }
    #[doc = "0x140 - Drive Delay Control Register"]
    #[inline(always)]
    pub const fn smhc_drv_dl(&self) -> &SMHC_DRV_DL {
        &self.smhc_drv_dl
    }
    #[doc = "0x144 - Sample Delay Control Register"]
    #[inline(always)]
    pub const fn smhc_smap_dl(&self) -> &SMHC_SMAP_DL {
        &self.smhc_smap_dl
    }
    #[doc = "0x148 - Data Strobe Delay Control Register"]
    #[inline(always)]
    pub const fn smhc_ds_dl(&self) -> &SMHC_DS_DL {
        &self.smhc_ds_dl
    }
    #[doc = "0x14c - HS400 Delay Control Register"]
    #[inline(always)]
    pub const fn smhc_hs400_dl(&self) -> &SMHC_HS400_DL {
        &self.smhc_hs400_dl
    }
    #[doc = "0x200 - Read/Write FIFO"]
    #[inline(always)]
    pub const fn smhc_fifo(&self) -> &SMHC_FIFO {
        &self.smhc_fifo
    }
}
#[doc = "smhc_ctrl (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_ctrl`] module"]
pub type SMHC_CTRL = crate::Reg<smhc_ctrl::SMHC_CTRL_SPEC>;
#[doc = "Control Register"]
pub mod smhc_ctrl;
#[doc = "smhc_clkdiv (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_clkdiv`] module"]
pub type SMHC_CLKDIV = crate::Reg<smhc_clkdiv::SMHC_CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod smhc_clkdiv;
#[doc = "smhc_tmout (rw) register accessor: Time Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_tmout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_tmout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_tmout`] module"]
pub type SMHC_TMOUT = crate::Reg<smhc_tmout::SMHC_TMOUT_SPEC>;
#[doc = "Time Out Register"]
pub mod smhc_tmout;
#[doc = "smhc_ctype (rw) register accessor: Bus Width Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ctype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ctype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_ctype`] module"]
pub type SMHC_CTYPE = crate::Reg<smhc_ctype::SMHC_CTYPE_SPEC>;
#[doc = "Bus Width Register"]
pub mod smhc_ctype;
#[doc = "smhc_blksiz (rw) register accessor: Block Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_blksiz`] module"]
pub type SMHC_BLKSIZ = crate::Reg<smhc_blksiz::SMHC_BLKSIZ_SPEC>;
#[doc = "Block Size Register"]
pub mod smhc_blksiz;
#[doc = "smhc_bytcnt (rw) register accessor: Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_bytcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_bytcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_bytcnt`] module"]
pub type SMHC_BYTCNT = crate::Reg<smhc_bytcnt::SMHC_BYTCNT_SPEC>;
#[doc = "Byte Count Register"]
pub mod smhc_bytcnt;
#[doc = "smhc_cmd (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_cmd`] module"]
pub type SMHC_CMD = crate::Reg<smhc_cmd::SMHC_CMD_SPEC>;
#[doc = "Command Register"]
pub mod smhc_cmd;
#[doc = "smhc_cmdarg (rw) register accessor: Command Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_cmdarg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_cmdarg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_cmdarg`] module"]
pub type SMHC_CMDARG = crate::Reg<smhc_cmdarg::SMHC_CMDARG_SPEC>;
#[doc = "Command Argument Register"]
pub mod smhc_cmdarg;
#[doc = "smhc_resp0 (r) register accessor: Response 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_resp0`] module"]
pub type SMHC_RESP0 = crate::Reg<smhc_resp0::SMHC_RESP0_SPEC>;
#[doc = "Response 0 Register"]
pub mod smhc_resp0;
#[doc = "smhc_resp1 (r) register accessor: Response 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_resp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_resp1`] module"]
pub type SMHC_RESP1 = crate::Reg<smhc_resp1::SMHC_RESP1_SPEC>;
#[doc = "Response 1 Register"]
pub mod smhc_resp1;
#[doc = "smhc_resp2 (r) register accessor: Response 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_resp2`] module"]
pub type SMHC_RESP2 = crate::Reg<smhc_resp2::SMHC_RESP2_SPEC>;
#[doc = "Response 2 Register"]
pub mod smhc_resp2;
#[doc = "smhc_resp3 (r) register accessor: Response 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_resp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_resp3`] module"]
pub type SMHC_RESP3 = crate::Reg<smhc_resp3::SMHC_RESP3_SPEC>;
#[doc = "Response 3 Register"]
pub mod smhc_resp3;
#[doc = "smhc_intmask (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_intmask`] module"]
pub type SMHC_INTMASK = crate::Reg<smhc_intmask::SMHC_INTMASK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod smhc_intmask;
#[doc = "smhc_mintsts (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_mintsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_mintsts`] module"]
pub type SMHC_MINTSTS = crate::Reg<smhc_mintsts::SMHC_MINTSTS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod smhc_mintsts;
#[doc = "smhc_rintsts (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_rintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_rintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_rintsts`] module"]
pub type SMHC_RINTSTS = crate::Reg<smhc_rintsts::SMHC_RINTSTS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod smhc_rintsts;
#[doc = "smhc_status (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_status`] module"]
pub type SMHC_STATUS = crate::Reg<smhc_status::SMHC_STATUS_SPEC>;
#[doc = "Status Register"]
pub mod smhc_status;
#[doc = "smhc_fifoth (rw) register accessor: FIFO Water Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_fifoth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_fifoth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_fifoth`] module"]
pub type SMHC_FIFOTH = crate::Reg<smhc_fifoth::SMHC_FIFOTH_SPEC>;
#[doc = "FIFO Water Level Register"]
pub mod smhc_fifoth;
#[doc = "smhc_funs (rw) register accessor: FIFO Function Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_funs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_funs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_funs`] module"]
pub type SMHC_FUNS = crate::Reg<smhc_funs::SMHC_FUNS_SPEC>;
#[doc = "FIFO Function Select Register"]
pub mod smhc_funs;
#[doc = "smhc_tbc0 (r) register accessor: Transferred Byte Count between Controller and Card\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_tbc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_tbc0`] module"]
pub type SMHC_TBC0 = crate::Reg<smhc_tbc0::SMHC_TBC0_SPEC>;
#[doc = "Transferred Byte Count between Controller and Card"]
pub mod smhc_tbc0;
#[doc = "smhc_tbc1 (r) register accessor: Transferred Byte Count between Host Memory and Internal FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_tbc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_tbc1`] module"]
pub type SMHC_TBC1 = crate::Reg<smhc_tbc1::SMHC_TBC1_SPEC>;
#[doc = "Transferred Byte Count between Host Memory and Internal FIFO"]
pub mod smhc_tbc1;
#[doc = "smhc_dbgc (rw) register accessor: Current Debug Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_dbgc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_dbgc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_dbgc`] module"]
pub type SMHC_DBGC = crate::Reg<smhc_dbgc::SMHC_DBGC_SPEC>;
#[doc = "Current Debug Control Register"]
pub mod smhc_dbgc;
#[doc = "smhc_csdc (rw) register accessor: CRC Status Detect Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_csdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_csdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_csdc`] module"]
pub type SMHC_CSDC = crate::Reg<smhc_csdc::SMHC_CSDC_SPEC>;
#[doc = "CRC Status Detect Control Registers"]
pub mod smhc_csdc;
#[doc = "smhc_a12a (rw) register accessor: Auto Command 12 Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_a12a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_a12a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_a12a`] module"]
pub type SMHC_A12A = crate::Reg<smhc_a12a::SMHC_A12A_SPEC>;
#[doc = "Auto Command 12 Argument Register"]
pub mod smhc_a12a;
#[doc = "smhc_ntsr (rw) register accessor: SD New Timing Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ntsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ntsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_ntsr`] module"]
pub type SMHC_NTSR = crate::Reg<smhc_ntsr::SMHC_NTSR_SPEC>;
#[doc = "SD New Timing Set Register"]
pub mod smhc_ntsr;
#[doc = "smhc_hwrst (rw) register accessor: Hardware Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_hwrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_hwrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_hwrst`] module"]
pub type SMHC_HWRST = crate::Reg<smhc_hwrst::SMHC_HWRST_SPEC>;
#[doc = "Hardware Reset Register"]
pub mod smhc_hwrst;
#[doc = "smhc_idmac (rw) register accessor: IDMAC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_idmac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_idmac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_idmac`] module"]
pub type SMHC_IDMAC = crate::Reg<smhc_idmac::SMHC_IDMAC_SPEC>;
#[doc = "IDMAC Control Register"]
pub mod smhc_idmac;
#[doc = "smhc_dlba (rw) register accessor: Descriptor List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_dlba::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_dlba::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_dlba`] module"]
pub type SMHC_DLBA = crate::Reg<smhc_dlba::SMHC_DLBA_SPEC>;
#[doc = "Descriptor List Base Address Register"]
pub mod smhc_dlba;
#[doc = "smhc_idst (rw) register accessor: IDMAC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_idst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_idst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_idst`] module"]
pub type SMHC_IDST = crate::Reg<smhc_idst::SMHC_IDST_SPEC>;
#[doc = "IDMAC Status Register"]
pub mod smhc_idst;
#[doc = "smhc_idie (rw) register accessor: IDMAC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_idie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_idie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_idie`] module"]
pub type SMHC_IDIE = crate::Reg<smhc_idie::SMHC_IDIE_SPEC>;
#[doc = "IDMAC Interrupt Enable Register"]
pub mod smhc_idie;
#[doc = "smhc_thld (rw) register accessor: Card Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_thld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_thld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_thld`] module"]
pub type SMHC_THLD = crate::Reg<smhc_thld::SMHC_THLD_SPEC>;
#[doc = "Card Threshold Control Register"]
pub mod smhc_thld;
#[doc = "smhc_sfc (rw) register accessor: Sample FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_sfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_sfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_sfc`] module"]
pub type SMHC_SFC = crate::Reg<smhc_sfc::SMHC_SFC_SPEC>;
#[doc = "Sample FIFO Control Register"]
pub mod smhc_sfc;
#[doc = "smhc_a23a (rw) register accessor: Auto Command 23 Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_a23a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_a23a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_a23a`] module"]
pub type SMHC_A23A = crate::Reg<smhc_a23a::SMHC_A23A_SPEC>;
#[doc = "Auto Command 23 Argument Register"]
pub mod smhc_a23a;
#[doc = "emmc_ddr_sbit_det (rw) register accessor: eMMC4.5 DDR Start Bit Detection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmc_ddr_sbit_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmc_ddr_sbit_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmc_ddr_sbit_det`] module"]
pub type EMMC_DDR_SBIT_DET = crate::Reg<emmc_ddr_sbit_det::EMMC_DDR_SBIT_DET_SPEC>;
#[doc = "eMMC4.5 DDR Start Bit Detection Control Register"]
pub mod emmc_ddr_sbit_det;
#[doc = "smhc_ext_cmd (rw) register accessor: Extended Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ext_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ext_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_ext_cmd`] module"]
pub type SMHC_EXT_CMD = crate::Reg<smhc_ext_cmd::SMHC_EXT_CMD_SPEC>;
#[doc = "Extended Command Register"]
pub mod smhc_ext_cmd;
#[doc = "smhc_ext_resp (r) register accessor: Extended Response Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ext_resp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_ext_resp`] module"]
pub type SMHC_EXT_RESP = crate::Reg<smhc_ext_resp::SMHC_EXT_RESP_SPEC>;
#[doc = "Extended Response Register"]
pub mod smhc_ext_resp;
#[doc = "smhc_drv_dl (rw) register accessor: Drive Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_drv_dl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_drv_dl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_drv_dl`] module"]
pub type SMHC_DRV_DL = crate::Reg<smhc_drv_dl::SMHC_DRV_DL_SPEC>;
#[doc = "Drive Delay Control Register"]
pub mod smhc_drv_dl;
#[doc = "smhc_smap_dl (rw) register accessor: Sample Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_smap_dl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_smap_dl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_smap_dl`] module"]
pub type SMHC_SMAP_DL = crate::Reg<smhc_smap_dl::SMHC_SMAP_DL_SPEC>;
#[doc = "Sample Delay Control Register"]
pub mod smhc_smap_dl;
#[doc = "smhc_ds_dl (rw) register accessor: Data Strobe Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ds_dl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ds_dl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_ds_dl`] module"]
pub type SMHC_DS_DL = crate::Reg<smhc_ds_dl::SMHC_DS_DL_SPEC>;
#[doc = "Data Strobe Delay Control Register"]
pub mod smhc_ds_dl;
#[doc = "smhc_hs400_dl (rw) register accessor: HS400 Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_hs400_dl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_hs400_dl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_hs400_dl`] module"]
pub type SMHC_HS400_DL = crate::Reg<smhc_hs400_dl::SMHC_HS400_DL_SPEC>;
#[doc = "HS400 Delay Control Register"]
pub mod smhc_hs400_dl;
#[doc = "smhc_fifo (rw) register accessor: Read/Write FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_fifo`] module"]
pub type SMHC_FIFO = crate::Reg<smhc_fifo::SMHC_FIFO_SPEC>;
#[doc = "Read/Write FIFO"]
pub mod smhc_fifo;
