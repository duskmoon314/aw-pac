#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub smhc_ctrl: SMHC_CTRL,
    #[doc = "0x04 - Clock Control Register"]
    pub smhc_clkdiv: SMHC_CLKDIV,
    #[doc = "0x08 - Time Out Register"]
    pub smhc_tmout: SMHC_TMOUT,
    #[doc = "0x0c - Bus Width Register"]
    pub smhc_ctype: SMHC_CTYPE,
    #[doc = "0x10 - Block Size Register"]
    pub smhc_blksiz: SMHC_BLKSIZ,
    #[doc = "0x14 - Byte Count Register"]
    pub smhc_bytcnt: SMHC_BYTCNT,
    #[doc = "0x18 - Command Register"]
    pub smhc_cmd: SMHC_CMD,
    #[doc = "0x1c - Command Argument Register"]
    pub smhc_cmdarg: SMHC_CMDARG,
    #[doc = "0x20 - Response 0 Register"]
    pub smhc_resp0: SMHC_RESP0,
    #[doc = "0x24 - Response 1 Register"]
    pub smhc_resp1: SMHC_RESP1,
    #[doc = "0x28 - Response 2 Register"]
    pub smhc_resp2: SMHC_RESP2,
    #[doc = "0x2c - Response 3 Register"]
    pub smhc_resp3: SMHC_RESP3,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub smhc_intmask: SMHC_INTMASK,
    #[doc = "0x34 - Masked Interrupt Status Register"]
    pub smhc_mintsts: SMHC_MINTSTS,
    #[doc = "0x38 - Raw Interrupt Status Register"]
    pub smhc_rintsts: SMHC_RINTSTS,
    #[doc = "0x3c - Status Register"]
    pub smhc_status: SMHC_STATUS,
    #[doc = "0x40 - FIFO Water Level Register"]
    pub smhc_fifoth: SMHC_FIFOTH,
    #[doc = "0x44 - FIFO Function Select Register"]
    pub smhc_funs: SMHC_FUNS,
    #[doc = "0x48 - Transferred Byte Count between Controller and Card"]
    pub smhc_tbc0: SMHC_TBC0,
    #[doc = "0x4c - Transferred Byte Count between Host Memory and Internal FIFO"]
    pub smhc_tbc1: SMHC_TBC1,
    #[doc = "0x50 - Current Debug Control Register"]
    pub smhc_dbgc: SMHC_DBGC,
    #[doc = "0x54 - CRC Status Detect Control Registers"]
    pub smhc_csdc: SMHC_CSDC,
    #[doc = "0x58 - Auto Command 12 Argument Register"]
    pub smhc_a12a: SMHC_A12A,
    #[doc = "0x5c - SD New Timing Set Register"]
    pub smhc_ntsr: SMHC_NTSR,
    _reserved24: [u8; 0x18],
    #[doc = "0x78 - Hardware Reset Register"]
    pub smhc_hwrst: SMHC_HWRST,
    _reserved25: [u8; 0x04],
    #[doc = "0x80 - IDMAC Control Register"]
    pub smhc_idmac: SMHC_IDMAC,
    #[doc = "0x84 - Descriptor List Base Address Register"]
    pub smhc_dlba: SMHC_DLBA,
    #[doc = "0x88 - IDMAC Status Register"]
    pub smhc_idst: SMHC_IDST,
    #[doc = "0x8c - IDMAC Interrupt Enable Register"]
    pub smhc_idie: SMHC_IDIE,
    _reserved29: [u8; 0x70],
    #[doc = "0x100 - Card Threshold Control Register"]
    pub smhc_thld: SMHC_THLD,
    #[doc = "0x104 - Sample FIFO Control Register"]
    pub smhc_sfc: SMHC_SFC,
    #[doc = "0x108 - Auto Command 23 Argument Register"]
    pub smhc_a23a: SMHC_A23A,
    #[doc = "0x10c - eMMC4.5 DDR Start Bit Detection Control Register"]
    pub emmc_ddr_sbit_det: EMMC_DDR_SBIT_DET,
    _reserved33: [u8; 0x28],
    #[doc = "0x138 - Extended Command Register"]
    pub smhc_ext_cmd: SMHC_EXT_CMD,
    #[doc = "0x13c - Extended Response Register"]
    pub smhc_ext_resp: SMHC_EXT_RESP,
    #[doc = "0x140 - Drive Delay Control Register"]
    pub smhc_drv_dl: SMHC_DRV_DL,
    #[doc = "0x144 - Sample Delay Control Register"]
    pub smhc_smap_dl: SMHC_SMAP_DL,
    #[doc = "0x148 - Data Strobe Delay Control Register"]
    pub smhc_ds_dl: SMHC_DS_DL,
    #[doc = "0x14c - HS400 Delay Control Register"]
    pub smhc_hs400_dl: SMHC_HS400_DL,
    _reserved39: [u8; 0xb0],
    #[doc = "0x200 - Read/Write FIFO"]
    pub smhc_fifo: SMHC_FIFO,
}
#[doc = "smhc_ctrl (rw) register accessor: an alias for `Reg<SMHC_CTRL_SPEC>`"]
pub type SMHC_CTRL = crate::Reg<smhc_ctrl::SMHC_CTRL_SPEC>;
#[doc = "Control Register"]
pub mod smhc_ctrl;
#[doc = "smhc_clkdiv (rw) register accessor: an alias for `Reg<SMHC_CLKDIV_SPEC>`"]
pub type SMHC_CLKDIV = crate::Reg<smhc_clkdiv::SMHC_CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod smhc_clkdiv;
#[doc = "smhc_tmout (rw) register accessor: an alias for `Reg<SMHC_TMOUT_SPEC>`"]
pub type SMHC_TMOUT = crate::Reg<smhc_tmout::SMHC_TMOUT_SPEC>;
#[doc = "Time Out Register"]
pub mod smhc_tmout;
#[doc = "smhc_ctype (rw) register accessor: an alias for `Reg<SMHC_CTYPE_SPEC>`"]
pub type SMHC_CTYPE = crate::Reg<smhc_ctype::SMHC_CTYPE_SPEC>;
#[doc = "Bus Width Register"]
pub mod smhc_ctype;
#[doc = "smhc_blksiz (rw) register accessor: an alias for `Reg<SMHC_BLKSIZ_SPEC>`"]
pub type SMHC_BLKSIZ = crate::Reg<smhc_blksiz::SMHC_BLKSIZ_SPEC>;
#[doc = "Block Size Register"]
pub mod smhc_blksiz;
#[doc = "smhc_bytcnt (rw) register accessor: an alias for `Reg<SMHC_BYTCNT_SPEC>`"]
pub type SMHC_BYTCNT = crate::Reg<smhc_bytcnt::SMHC_BYTCNT_SPEC>;
#[doc = "Byte Count Register"]
pub mod smhc_bytcnt;
#[doc = "smhc_cmd (rw) register accessor: an alias for `Reg<SMHC_CMD_SPEC>`"]
pub type SMHC_CMD = crate::Reg<smhc_cmd::SMHC_CMD_SPEC>;
#[doc = "Command Register"]
pub mod smhc_cmd;
#[doc = "smhc_cmdarg (rw) register accessor: an alias for `Reg<SMHC_CMDARG_SPEC>`"]
pub type SMHC_CMDARG = crate::Reg<smhc_cmdarg::SMHC_CMDARG_SPEC>;
#[doc = "Command Argument Register"]
pub mod smhc_cmdarg;
#[doc = "smhc_resp0 (r) register accessor: an alias for `Reg<SMHC_RESP0_SPEC>`"]
pub type SMHC_RESP0 = crate::Reg<smhc_resp0::SMHC_RESP0_SPEC>;
#[doc = "Response 0 Register"]
pub mod smhc_resp0;
#[doc = "smhc_resp1 (r) register accessor: an alias for `Reg<SMHC_RESP1_SPEC>`"]
pub type SMHC_RESP1 = crate::Reg<smhc_resp1::SMHC_RESP1_SPEC>;
#[doc = "Response 1 Register"]
pub mod smhc_resp1;
#[doc = "smhc_resp2 (r) register accessor: an alias for `Reg<SMHC_RESP2_SPEC>`"]
pub type SMHC_RESP2 = crate::Reg<smhc_resp2::SMHC_RESP2_SPEC>;
#[doc = "Response 2 Register"]
pub mod smhc_resp2;
#[doc = "smhc_resp3 (r) register accessor: an alias for `Reg<SMHC_RESP3_SPEC>`"]
pub type SMHC_RESP3 = crate::Reg<smhc_resp3::SMHC_RESP3_SPEC>;
#[doc = "Response 3 Register"]
pub mod smhc_resp3;
#[doc = "smhc_intmask (rw) register accessor: an alias for `Reg<SMHC_INTMASK_SPEC>`"]
pub type SMHC_INTMASK = crate::Reg<smhc_intmask::SMHC_INTMASK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod smhc_intmask;
#[doc = "smhc_mintsts (r) register accessor: an alias for `Reg<SMHC_MINTSTS_SPEC>`"]
pub type SMHC_MINTSTS = crate::Reg<smhc_mintsts::SMHC_MINTSTS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod smhc_mintsts;
#[doc = "smhc_rintsts (rw) register accessor: an alias for `Reg<SMHC_RINTSTS_SPEC>`"]
pub type SMHC_RINTSTS = crate::Reg<smhc_rintsts::SMHC_RINTSTS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod smhc_rintsts;
#[doc = "smhc_status (r) register accessor: an alias for `Reg<SMHC_STATUS_SPEC>`"]
pub type SMHC_STATUS = crate::Reg<smhc_status::SMHC_STATUS_SPEC>;
#[doc = "Status Register"]
pub mod smhc_status;
#[doc = "smhc_fifoth (rw) register accessor: an alias for `Reg<SMHC_FIFOTH_SPEC>`"]
pub type SMHC_FIFOTH = crate::Reg<smhc_fifoth::SMHC_FIFOTH_SPEC>;
#[doc = "FIFO Water Level Register"]
pub mod smhc_fifoth;
#[doc = "smhc_funs (rw) register accessor: an alias for `Reg<SMHC_FUNS_SPEC>`"]
pub type SMHC_FUNS = crate::Reg<smhc_funs::SMHC_FUNS_SPEC>;
#[doc = "FIFO Function Select Register"]
pub mod smhc_funs;
#[doc = "smhc_tbc0 (r) register accessor: an alias for `Reg<SMHC_TBC0_SPEC>`"]
pub type SMHC_TBC0 = crate::Reg<smhc_tbc0::SMHC_TBC0_SPEC>;
#[doc = "Transferred Byte Count between Controller and Card"]
pub mod smhc_tbc0;
#[doc = "smhc_tbc1 (r) register accessor: an alias for `Reg<SMHC_TBC1_SPEC>`"]
pub type SMHC_TBC1 = crate::Reg<smhc_tbc1::SMHC_TBC1_SPEC>;
#[doc = "Transferred Byte Count between Host Memory and Internal FIFO"]
pub mod smhc_tbc1;
#[doc = "smhc_dbgc (rw) register accessor: an alias for `Reg<SMHC_DBGC_SPEC>`"]
pub type SMHC_DBGC = crate::Reg<smhc_dbgc::SMHC_DBGC_SPEC>;
#[doc = "Current Debug Control Register"]
pub mod smhc_dbgc;
#[doc = "smhc_csdc (rw) register accessor: an alias for `Reg<SMHC_CSDC_SPEC>`"]
pub type SMHC_CSDC = crate::Reg<smhc_csdc::SMHC_CSDC_SPEC>;
#[doc = "CRC Status Detect Control Registers"]
pub mod smhc_csdc;
#[doc = "smhc_a12a (rw) register accessor: an alias for `Reg<SMHC_A12A_SPEC>`"]
pub type SMHC_A12A = crate::Reg<smhc_a12a::SMHC_A12A_SPEC>;
#[doc = "Auto Command 12 Argument Register"]
pub mod smhc_a12a;
#[doc = "smhc_ntsr (rw) register accessor: an alias for `Reg<SMHC_NTSR_SPEC>`"]
pub type SMHC_NTSR = crate::Reg<smhc_ntsr::SMHC_NTSR_SPEC>;
#[doc = "SD New Timing Set Register"]
pub mod smhc_ntsr;
#[doc = "smhc_hwrst (rw) register accessor: an alias for `Reg<SMHC_HWRST_SPEC>`"]
pub type SMHC_HWRST = crate::Reg<smhc_hwrst::SMHC_HWRST_SPEC>;
#[doc = "Hardware Reset Register"]
pub mod smhc_hwrst;
#[doc = "smhc_idmac (rw) register accessor: an alias for `Reg<SMHC_IDMAC_SPEC>`"]
pub type SMHC_IDMAC = crate::Reg<smhc_idmac::SMHC_IDMAC_SPEC>;
#[doc = "IDMAC Control Register"]
pub mod smhc_idmac;
#[doc = "smhc_dlba (rw) register accessor: an alias for `Reg<SMHC_DLBA_SPEC>`"]
pub type SMHC_DLBA = crate::Reg<smhc_dlba::SMHC_DLBA_SPEC>;
#[doc = "Descriptor List Base Address Register"]
pub mod smhc_dlba;
#[doc = "smhc_idst (rw) register accessor: an alias for `Reg<SMHC_IDST_SPEC>`"]
pub type SMHC_IDST = crate::Reg<smhc_idst::SMHC_IDST_SPEC>;
#[doc = "IDMAC Status Register"]
pub mod smhc_idst;
#[doc = "smhc_idie (rw) register accessor: an alias for `Reg<SMHC_IDIE_SPEC>`"]
pub type SMHC_IDIE = crate::Reg<smhc_idie::SMHC_IDIE_SPEC>;
#[doc = "IDMAC Interrupt Enable Register"]
pub mod smhc_idie;
#[doc = "smhc_thld (rw) register accessor: an alias for `Reg<SMHC_THLD_SPEC>`"]
pub type SMHC_THLD = crate::Reg<smhc_thld::SMHC_THLD_SPEC>;
#[doc = "Card Threshold Control Register"]
pub mod smhc_thld;
#[doc = "smhc_sfc (rw) register accessor: an alias for `Reg<SMHC_SFC_SPEC>`"]
pub type SMHC_SFC = crate::Reg<smhc_sfc::SMHC_SFC_SPEC>;
#[doc = "Sample FIFO Control Register"]
pub mod smhc_sfc;
#[doc = "smhc_a23a (rw) register accessor: an alias for `Reg<SMHC_A23A_SPEC>`"]
pub type SMHC_A23A = crate::Reg<smhc_a23a::SMHC_A23A_SPEC>;
#[doc = "Auto Command 23 Argument Register"]
pub mod smhc_a23a;
#[doc = "emmc_ddr_sbit_det (rw) register accessor: an alias for `Reg<EMMC_DDR_SBIT_DET_SPEC>`"]
pub type EMMC_DDR_SBIT_DET = crate::Reg<emmc_ddr_sbit_det::EMMC_DDR_SBIT_DET_SPEC>;
#[doc = "eMMC4.5 DDR Start Bit Detection Control Register"]
pub mod emmc_ddr_sbit_det;
#[doc = "smhc_ext_cmd (rw) register accessor: an alias for `Reg<SMHC_EXT_CMD_SPEC>`"]
pub type SMHC_EXT_CMD = crate::Reg<smhc_ext_cmd::SMHC_EXT_CMD_SPEC>;
#[doc = "Extended Command Register"]
pub mod smhc_ext_cmd;
#[doc = "smhc_ext_resp (r) register accessor: an alias for `Reg<SMHC_EXT_RESP_SPEC>`"]
pub type SMHC_EXT_RESP = crate::Reg<smhc_ext_resp::SMHC_EXT_RESP_SPEC>;
#[doc = "Extended Response Register"]
pub mod smhc_ext_resp;
#[doc = "smhc_drv_dl (rw) register accessor: an alias for `Reg<SMHC_DRV_DL_SPEC>`"]
pub type SMHC_DRV_DL = crate::Reg<smhc_drv_dl::SMHC_DRV_DL_SPEC>;
#[doc = "Drive Delay Control Register"]
pub mod smhc_drv_dl;
#[doc = "smhc_smap_dl (rw) register accessor: an alias for `Reg<SMHC_SMAP_DL_SPEC>`"]
pub type SMHC_SMAP_DL = crate::Reg<smhc_smap_dl::SMHC_SMAP_DL_SPEC>;
#[doc = "Sample Delay Control Register"]
pub mod smhc_smap_dl;
#[doc = "smhc_ds_dl (rw) register accessor: an alias for `Reg<SMHC_DS_DL_SPEC>`"]
pub type SMHC_DS_DL = crate::Reg<smhc_ds_dl::SMHC_DS_DL_SPEC>;
#[doc = "Data Strobe Delay Control Register"]
pub mod smhc_ds_dl;
#[doc = "smhc_hs400_dl (rw) register accessor: an alias for `Reg<SMHC_HS400_DL_SPEC>`"]
pub type SMHC_HS400_DL = crate::Reg<smhc_hs400_dl::SMHC_HS400_DL_SPEC>;
#[doc = "HS400 Delay Control Register"]
pub mod smhc_hs400_dl;
#[doc = "smhc_fifo (rw) register accessor: an alias for `Reg<SMHC_FIFO_SPEC>`"]
pub type SMHC_FIFO = crate::Reg<smhc_fifo::SMHC_FIFO_SPEC>;
#[doc = "Read/Write FIFO"]
pub mod smhc_fifo;
