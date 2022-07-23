#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC IRQ Enable Register 0"]
    pub dmac_irq_en_reg0: crate::Reg<dmac_irq_en_reg0::DMAC_IRQ_EN_REG0_SPEC>,
    #[doc = "0x04 - DMAC IRQ Enable Register 1"]
    pub dmac_irq_en_reg1: crate::Reg<dmac_irq_en_reg1::DMAC_IRQ_EN_REG1_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - DMAC IRQ Pending Register 0"]
    pub dmac_irq_pend_reg0: crate::Reg<dmac_irq_pend_reg0::DMAC_IRQ_PEND_REG0_SPEC>,
    #[doc = "0x14 - DMAC IRQ Pending Register 1"]
    pub dmac_irq_pend_reg1: crate::Reg<dmac_irq_pend_reg1::DMAC_IRQ_PEND_REG1_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x28 - DMAC Auto Gating Register"]
    pub dmac_auto_gate_reg: crate::Reg<dmac_auto_gate_reg::DMAC_AUTO_GATE_REG_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x30 - DMAC Status Register"]
    pub dmac_sta_reg: crate::Reg<dmac_sta_reg::DMAC_STA_REG_SPEC>,
    _reserved6: [u8; 0xcc],
    #[doc = "0x100 - DMAC Channel Enable Register"]
    pub dmac_en_reg0: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x104 - DMAC Channel Pause Register"]
    pub dmac_pau_reg0: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x108 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg0: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x10c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg0: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x110 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg0: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x114 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg0: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x118 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg0: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x11c - DMAC Channel Parameter Register"]
    pub dmac_para_reg0: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x128 - DMAC Mode Register"]
    pub dmac_mode_reg0: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x12c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg0: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x130 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg0: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved17: [u8; 0x0c],
    #[doc = "0x140 - DMAC Channel Enable Register"]
    pub dmac_en_reg1: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x144 - DMAC Channel Pause Register"]
    pub dmac_pau_reg1: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x148 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg1: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x14c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg1: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x150 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg1: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x154 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg1: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x158 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg1: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x15c - DMAC Channel Parameter Register"]
    pub dmac_para_reg1: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x168 - DMAC Mode Register"]
    pub dmac_mode_reg1: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x16c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg1: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x170 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg1: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0x180 - DMAC Channel Enable Register"]
    pub dmac_en_reg2: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x184 - DMAC Channel Pause Register"]
    pub dmac_pau_reg2: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x188 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg2: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x18c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg2: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x190 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg2: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x194 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg2: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x198 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg2: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x19c - DMAC Channel Parameter Register"]
    pub dmac_para_reg2: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved36: [u8; 0x08],
    #[doc = "0x1a8 - DMAC Mode Register"]
    pub dmac_mode_reg2: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x1ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg2: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x1b0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg2: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved39: [u8; 0x0c],
    #[doc = "0x1c0 - DMAC Channel Enable Register"]
    pub dmac_en_reg3: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x1c4 - DMAC Channel Pause Register"]
    pub dmac_pau_reg3: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x1c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg3: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x1cc - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg3: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x1d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg3: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x1d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg3: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x1d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg3: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x1dc - DMAC Channel Parameter Register"]
    pub dmac_para_reg3: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0x1e8 - DMAC Mode Register"]
    pub dmac_mode_reg3: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x1ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg3: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x1f0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg3: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved50: [u8; 0x0c],
    #[doc = "0x200 - DMAC Channel Enable Register"]
    pub dmac_en_reg4: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x204 - DMAC Channel Pause Register"]
    pub dmac_pau_reg4: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x208 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg4: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x20c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg4: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x210 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg4: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x214 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg4: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x218 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg4: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x21c - DMAC Channel Parameter Register"]
    pub dmac_para_reg4: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved58: [u8; 0x08],
    #[doc = "0x228 - DMAC Mode Register"]
    pub dmac_mode_reg4: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x22c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg4: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x230 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg4: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved61: [u8; 0x0c],
    #[doc = "0x240 - DMAC Channel Enable Register"]
    pub dmac_en_reg5: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x244 - DMAC Channel Pause Register"]
    pub dmac_pau_reg5: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x248 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg5: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x24c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg5: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x250 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg5: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x254 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg5: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x258 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg5: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x25c - DMAC Channel Parameter Register"]
    pub dmac_para_reg5: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved69: [u8; 0x08],
    #[doc = "0x268 - DMAC Mode Register"]
    pub dmac_mode_reg5: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x26c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg5: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x270 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg5: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved72: [u8; 0x0c],
    #[doc = "0x280 - DMAC Channel Enable Register"]
    pub dmac_en_reg6: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x284 - DMAC Channel Pause Register"]
    pub dmac_pau_reg6: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x288 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg6: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x28c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg6: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x290 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg6: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x294 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg6: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x298 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg6: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x29c - DMAC Channel Parameter Register"]
    pub dmac_para_reg6: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved80: [u8; 0x08],
    #[doc = "0x2a8 - DMAC Mode Register"]
    pub dmac_mode_reg6: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x2ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg6: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x2b0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg6: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved83: [u8; 0x0c],
    #[doc = "0x2c0 - DMAC Channel Enable Register"]
    pub dmac_en_reg7: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x2c4 - DMAC Channel Pause Register"]
    pub dmac_pau_reg7: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x2c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg7: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x2cc - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg7: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x2d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg7: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x2d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg7: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x2d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg7: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x2dc - DMAC Channel Parameter Register"]
    pub dmac_para_reg7: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved91: [u8; 0x08],
    #[doc = "0x2e8 - DMAC Mode Register"]
    pub dmac_mode_reg7: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x2ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg7: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x2f0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg7: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved94: [u8; 0x0c],
    #[doc = "0x300 - DMAC Channel Enable Register"]
    pub dmac_en_reg8: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x304 - DMAC Channel Pause Register"]
    pub dmac_pau_reg8: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x308 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg8: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x30c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg8: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x310 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg8: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x314 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg8: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x318 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg8: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x31c - DMAC Channel Parameter Register"]
    pub dmac_para_reg8: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved102: [u8; 0x08],
    #[doc = "0x328 - DMAC Mode Register"]
    pub dmac_mode_reg8: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x32c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg8: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x330 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg8: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved105: [u8; 0x0c],
    #[doc = "0x340 - DMAC Channel Enable Register"]
    pub dmac_en_reg9: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x344 - DMAC Channel Pause Register"]
    pub dmac_pau_reg9: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x348 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg9: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x34c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg9: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x350 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg9: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x354 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg9: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x358 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg9: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x35c - DMAC Channel Parameter Register"]
    pub dmac_para_reg9: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved113: [u8; 0x08],
    #[doc = "0x368 - DMAC Mode Register"]
    pub dmac_mode_reg9: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x36c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg9: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x370 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg9: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved116: [u8; 0x0c],
    #[doc = "0x380 - DMAC Channel Enable Register"]
    pub dmac_en_reg10: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x384 - DMAC Channel Pause Register"]
    pub dmac_pau_reg10: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x388 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg10: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x38c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg10: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x390 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg10: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x394 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg10: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x398 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg10: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x39c - DMAC Channel Parameter Register"]
    pub dmac_para_reg10: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved124: [u8; 0x08],
    #[doc = "0x3a8 - DMAC Mode Register"]
    pub dmac_mode_reg10: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x3ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg10: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x3b0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg10: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved127: [u8; 0x0c],
    #[doc = "0x3c0 - DMAC Channel Enable Register"]
    pub dmac_en_reg11: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x3c4 - DMAC Channel Pause Register"]
    pub dmac_pau_reg11: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x3c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg11: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x3cc - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg11: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x3d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg11: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x3d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg11: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x3d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg11: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x3dc - DMAC Channel Parameter Register"]
    pub dmac_para_reg11: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved135: [u8; 0x08],
    #[doc = "0x3e8 - DMAC Mode Register"]
    pub dmac_mode_reg11: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x3ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg11: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x3f0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg11: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved138: [u8; 0x0c],
    #[doc = "0x400 - DMAC Channel Enable Register"]
    pub dmac_en_reg12: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x404 - DMAC Channel Pause Register"]
    pub dmac_pau_reg12: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x408 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg12: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x40c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg12: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x410 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg12: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x414 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg12: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x418 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg12: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x41c - DMAC Channel Parameter Register"]
    pub dmac_para_reg12: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved146: [u8; 0x08],
    #[doc = "0x428 - DMAC Mode Register"]
    pub dmac_mode_reg12: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x42c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg12: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x430 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg12: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved149: [u8; 0x0c],
    #[doc = "0x440 - DMAC Channel Enable Register"]
    pub dmac_en_reg13: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x444 - DMAC Channel Pause Register"]
    pub dmac_pau_reg13: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x448 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg13: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x44c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg13: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x450 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg13: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x454 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg13: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x458 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg13: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x45c - DMAC Channel Parameter Register"]
    pub dmac_para_reg13: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved157: [u8; 0x08],
    #[doc = "0x468 - DMAC Mode Register"]
    pub dmac_mode_reg13: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x46c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg13: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x470 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg13: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved160: [u8; 0x0c],
    #[doc = "0x480 - DMAC Channel Enable Register"]
    pub dmac_en_reg14: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x484 - DMAC Channel Pause Register"]
    pub dmac_pau_reg14: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x488 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg14: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x48c - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg14: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x490 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg14: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x494 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg14: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x498 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg14: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x49c - DMAC Channel Parameter Register"]
    pub dmac_para_reg14: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved168: [u8; 0x08],
    #[doc = "0x4a8 - DMAC Mode Register"]
    pub dmac_mode_reg14: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x4ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg14: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x4b0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg14: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
    _reserved171: [u8; 0x0c],
    #[doc = "0x4c0 - DMAC Channel Enable Register"]
    pub dmac_en_reg15: crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>,
    #[doc = "0x4c4 - DMAC Channel Pause Register"]
    pub dmac_pau_reg15: crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>,
    #[doc = "0x4c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr_reg15: crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>,
    #[doc = "0x4cc - DMAC Channel Configuration Register"]
    pub dmac_cfg_reg15: crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>,
    #[doc = "0x4d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src_reg15: crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>,
    #[doc = "0x4d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest_reg15: crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>,
    #[doc = "0x4d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left_reg15: crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>,
    #[doc = "0x4dc - DMAC Channel Parameter Register"]
    pub dmac_para_reg15: crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>,
    _reserved179: [u8; 0x08],
    #[doc = "0x4e8 - DMAC Mode Register"]
    pub dmac_mode_reg15: crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>,
    #[doc = "0x4ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr_reg15: crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>,
    #[doc = "0x4f0 - DMAC Package Number Register"]
    pub dmac_pkg_num_reg15: crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>,
}
#[doc = "dmac_irq_en_reg0 register accessor: an alias for `Reg<DMAC_IRQ_EN_REG0_SPEC>`"]
pub type DMAC_IRQ_EN_REG0 = crate::Reg<dmac_irq_en_reg0::DMAC_IRQ_EN_REG0_SPEC>;
#[doc = "DMAC IRQ Enable Register 0"]
pub mod dmac_irq_en_reg0;
#[doc = "dmac_irq_en_reg1 register accessor: an alias for `Reg<DMAC_IRQ_EN_REG1_SPEC>`"]
pub type DMAC_IRQ_EN_REG1 = crate::Reg<dmac_irq_en_reg1::DMAC_IRQ_EN_REG1_SPEC>;
#[doc = "DMAC IRQ Enable Register 1"]
pub mod dmac_irq_en_reg1;
#[doc = "dmac_irq_pend_reg0 register accessor: an alias for `Reg<DMAC_IRQ_PEND_REG0_SPEC>`"]
pub type DMAC_IRQ_PEND_REG0 = crate::Reg<dmac_irq_pend_reg0::DMAC_IRQ_PEND_REG0_SPEC>;
#[doc = "DMAC IRQ Pending Register 0"]
pub mod dmac_irq_pend_reg0;
#[doc = "dmac_irq_pend_reg1 register accessor: an alias for `Reg<DMAC_IRQ_PEND_REG1_SPEC>`"]
pub type DMAC_IRQ_PEND_REG1 = crate::Reg<dmac_irq_pend_reg1::DMAC_IRQ_PEND_REG1_SPEC>;
#[doc = "DMAC IRQ Pending Register 1"]
pub mod dmac_irq_pend_reg1;
#[doc = "dmac_auto_gate_reg register accessor: an alias for `Reg<DMAC_AUTO_GATE_REG_SPEC>`"]
pub type DMAC_AUTO_GATE_REG = crate::Reg<dmac_auto_gate_reg::DMAC_AUTO_GATE_REG_SPEC>;
#[doc = "DMAC Auto Gating Register"]
pub mod dmac_auto_gate_reg;
#[doc = "dmac_sta_reg register accessor: an alias for `Reg<DMAC_STA_REG_SPEC>`"]
pub type DMAC_STA_REG = crate::Reg<dmac_sta_reg::DMAC_STA_REG_SPEC>;
#[doc = "DMAC Status Register"]
pub mod dmac_sta_reg;
#[doc = "dmac_en_reg register accessor: an alias for `Reg<DMAC_EN_REG_SPEC>`"]
pub type DMAC_EN_REG = crate::Reg<dmac_en_reg::DMAC_EN_REG_SPEC>;
#[doc = "DMAC Channel Enable Register"]
pub mod dmac_en_reg;
#[doc = "dmac_pau_reg register accessor: an alias for `Reg<DMAC_PAU_REG_SPEC>`"]
pub type DMAC_PAU_REG = crate::Reg<dmac_pau_reg::DMAC_PAU_REG_SPEC>;
#[doc = "DMAC Channel Pause Register"]
pub mod dmac_pau_reg;
#[doc = "dmac_desc_addr_reg register accessor: an alias for `Reg<DMAC_DESC_ADDR_REG_SPEC>`"]
pub type DMAC_DESC_ADDR_REG = crate::Reg<dmac_desc_addr_reg::DMAC_DESC_ADDR_REG_SPEC>;
#[doc = "DMAC Channel Start Address Register"]
pub mod dmac_desc_addr_reg;
#[doc = "dmac_cfg_reg register accessor: an alias for `Reg<DMAC_CFG_REG_SPEC>`"]
pub type DMAC_CFG_REG = crate::Reg<dmac_cfg_reg::DMAC_CFG_REG_SPEC>;
#[doc = "DMAC Channel Configuration Register"]
pub mod dmac_cfg_reg;
#[doc = "dmac_cur_src_reg register accessor: an alias for `Reg<DMAC_CUR_SRC_REG_SPEC>`"]
pub type DMAC_CUR_SRC_REG = crate::Reg<dmac_cur_src_reg::DMAC_CUR_SRC_REG_SPEC>;
#[doc = "DMAC Channel Current Source Register"]
pub mod dmac_cur_src_reg;
#[doc = "dmac_cur_dest_reg register accessor: an alias for `Reg<DMAC_CUR_DEST_REG_SPEC>`"]
pub type DMAC_CUR_DEST_REG = crate::Reg<dmac_cur_dest_reg::DMAC_CUR_DEST_REG_SPEC>;
#[doc = "DMAC Channel Current Destination Register"]
pub mod dmac_cur_dest_reg;
#[doc = "dmac_bcnt_left_reg register accessor: an alias for `Reg<DMAC_BCNT_LEFT_REG_SPEC>`"]
pub type DMAC_BCNT_LEFT_REG = crate::Reg<dmac_bcnt_left_reg::DMAC_BCNT_LEFT_REG_SPEC>;
#[doc = "DMAC Channel Byte Counter Left Register"]
pub mod dmac_bcnt_left_reg;
#[doc = "dmac_para_reg register accessor: an alias for `Reg<DMAC_PARA_REG_SPEC>`"]
pub type DMAC_PARA_REG = crate::Reg<dmac_para_reg::DMAC_PARA_REG_SPEC>;
#[doc = "DMAC Channel Parameter Register"]
pub mod dmac_para_reg;
#[doc = "dmac_mode_reg register accessor: an alias for `Reg<DMAC_MODE_REG_SPEC>`"]
pub type DMAC_MODE_REG = crate::Reg<dmac_mode_reg::DMAC_MODE_REG_SPEC>;
#[doc = "DMAC Mode Register"]
pub mod dmac_mode_reg;
#[doc = "dmac_fdesc_addr_reg register accessor: an alias for `Reg<DMAC_FDESC_ADDR_REG_SPEC>`"]
pub type DMAC_FDESC_ADDR_REG = crate::Reg<dmac_fdesc_addr_reg::DMAC_FDESC_ADDR_REG_SPEC>;
#[doc = "DMAC Former Descriptor Address Register"]
pub mod dmac_fdesc_addr_reg;
#[doc = "dmac_pkg_num_reg register accessor: an alias for `Reg<DMAC_PKG_NUM_REG_SPEC>`"]
pub type DMAC_PKG_NUM_REG = crate::Reg<dmac_pkg_num_reg::DMAC_PKG_NUM_REG_SPEC>;
#[doc = "DMAC Package Number Register"]
pub mod dmac_pkg_num_reg;
