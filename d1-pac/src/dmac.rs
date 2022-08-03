#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC IRQ Enable Register 0"]
    pub dmac_irq_en0: DMAC_IRQ_EN0,
    #[doc = "0x04 - DMAC IRQ Enable Register 1"]
    pub dmac_irq_en1: DMAC_IRQ_EN1,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - DMAC IRQ Pending Register 0"]
    pub dmac_irq_pend0: DMAC_IRQ_PEND0,
    #[doc = "0x14 - DMAC IRQ Pending Register 1"]
    pub dmac_irq_pend1: DMAC_IRQ_PEND1,
    _reserved4: [u8; 0x10],
    #[doc = "0x28 - DMAC Auto Gating Register"]
    pub dmac_auto_gate: DMAC_AUTO_GATE,
    _reserved5: [u8; 0x04],
    #[doc = "0x30 - DMAC Status Register"]
    pub dmac_sta: DMAC_STA,
    _reserved6: [u8; 0xcc],
    #[doc = "0x100 - DMAC Channel Enable Register"]
    pub dmac_en0: DMAC_EN,
    #[doc = "0x104 - DMAC Channel Pause Register"]
    pub dmac_pau0: DMAC_PAU,
    #[doc = "0x108 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr0: DMAC_DESC_ADDR,
    #[doc = "0x10c - DMAC Channel Configuration Register"]
    pub dmac_cfg0: DMAC_CFG,
    #[doc = "0x110 - DMAC Channel Current Source Register"]
    pub dmac_cur_src0: DMAC_CUR_SRC,
    #[doc = "0x114 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest0: DMAC_CUR_DEST,
    #[doc = "0x118 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left0: DMAC_BCNT_LEFT,
    #[doc = "0x11c - DMAC Channel Parameter Register"]
    pub dmac_para0: DMAC_PARA,
    _reserved14: [u8; 0x08],
    #[doc = "0x128 - DMAC Mode Register"]
    pub dmac_mode0: DMAC_MODE,
    #[doc = "0x12c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr0: DMAC_FDESC_ADDR,
    #[doc = "0x130 - DMAC Package Number Register"]
    pub dmac_pkg_num0: DMAC_PKG_NUM,
    _reserved17: [u8; 0x0c],
    #[doc = "0x140 - DMAC Channel Enable Register"]
    pub dmac_en1: DMAC_EN,
    #[doc = "0x144 - DMAC Channel Pause Register"]
    pub dmac_pau1: DMAC_PAU,
    #[doc = "0x148 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr1: DMAC_DESC_ADDR,
    #[doc = "0x14c - DMAC Channel Configuration Register"]
    pub dmac_cfg1: DMAC_CFG,
    #[doc = "0x150 - DMAC Channel Current Source Register"]
    pub dmac_cur_src1: DMAC_CUR_SRC,
    #[doc = "0x154 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest1: DMAC_CUR_DEST,
    #[doc = "0x158 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left1: DMAC_BCNT_LEFT,
    #[doc = "0x15c - DMAC Channel Parameter Register"]
    pub dmac_para1: DMAC_PARA,
    _reserved25: [u8; 0x08],
    #[doc = "0x168 - DMAC Mode Register"]
    pub dmac_mode1: DMAC_MODE,
    #[doc = "0x16c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr1: DMAC_FDESC_ADDR,
    #[doc = "0x170 - DMAC Package Number Register"]
    pub dmac_pkg_num1: DMAC_PKG_NUM,
    _reserved28: [u8; 0x0c],
    #[doc = "0x180 - DMAC Channel Enable Register"]
    pub dmac_en2: DMAC_EN,
    #[doc = "0x184 - DMAC Channel Pause Register"]
    pub dmac_pau2: DMAC_PAU,
    #[doc = "0x188 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr2: DMAC_DESC_ADDR,
    #[doc = "0x18c - DMAC Channel Configuration Register"]
    pub dmac_cfg2: DMAC_CFG,
    #[doc = "0x190 - DMAC Channel Current Source Register"]
    pub dmac_cur_src2: DMAC_CUR_SRC,
    #[doc = "0x194 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest2: DMAC_CUR_DEST,
    #[doc = "0x198 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left2: DMAC_BCNT_LEFT,
    #[doc = "0x19c - DMAC Channel Parameter Register"]
    pub dmac_para2: DMAC_PARA,
    _reserved36: [u8; 0x08],
    #[doc = "0x1a8 - DMAC Mode Register"]
    pub dmac_mode2: DMAC_MODE,
    #[doc = "0x1ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr2: DMAC_FDESC_ADDR,
    #[doc = "0x1b0 - DMAC Package Number Register"]
    pub dmac_pkg_num2: DMAC_PKG_NUM,
    _reserved39: [u8; 0x0c],
    #[doc = "0x1c0 - DMAC Channel Enable Register"]
    pub dmac_en3: DMAC_EN,
    #[doc = "0x1c4 - DMAC Channel Pause Register"]
    pub dmac_pau3: DMAC_PAU,
    #[doc = "0x1c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr3: DMAC_DESC_ADDR,
    #[doc = "0x1cc - DMAC Channel Configuration Register"]
    pub dmac_cfg3: DMAC_CFG,
    #[doc = "0x1d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src3: DMAC_CUR_SRC,
    #[doc = "0x1d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest3: DMAC_CUR_DEST,
    #[doc = "0x1d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left3: DMAC_BCNT_LEFT,
    #[doc = "0x1dc - DMAC Channel Parameter Register"]
    pub dmac_para3: DMAC_PARA,
    _reserved47: [u8; 0x08],
    #[doc = "0x1e8 - DMAC Mode Register"]
    pub dmac_mode3: DMAC_MODE,
    #[doc = "0x1ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr3: DMAC_FDESC_ADDR,
    #[doc = "0x1f0 - DMAC Package Number Register"]
    pub dmac_pkg_num3: DMAC_PKG_NUM,
    _reserved50: [u8; 0x0c],
    #[doc = "0x200 - DMAC Channel Enable Register"]
    pub dmac_en4: DMAC_EN,
    #[doc = "0x204 - DMAC Channel Pause Register"]
    pub dmac_pau4: DMAC_PAU,
    #[doc = "0x208 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr4: DMAC_DESC_ADDR,
    #[doc = "0x20c - DMAC Channel Configuration Register"]
    pub dmac_cfg4: DMAC_CFG,
    #[doc = "0x210 - DMAC Channel Current Source Register"]
    pub dmac_cur_src4: DMAC_CUR_SRC,
    #[doc = "0x214 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest4: DMAC_CUR_DEST,
    #[doc = "0x218 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left4: DMAC_BCNT_LEFT,
    #[doc = "0x21c - DMAC Channel Parameter Register"]
    pub dmac_para4: DMAC_PARA,
    _reserved58: [u8; 0x08],
    #[doc = "0x228 - DMAC Mode Register"]
    pub dmac_mode4: DMAC_MODE,
    #[doc = "0x22c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr4: DMAC_FDESC_ADDR,
    #[doc = "0x230 - DMAC Package Number Register"]
    pub dmac_pkg_num4: DMAC_PKG_NUM,
    _reserved61: [u8; 0x0c],
    #[doc = "0x240 - DMAC Channel Enable Register"]
    pub dmac_en5: DMAC_EN,
    #[doc = "0x244 - DMAC Channel Pause Register"]
    pub dmac_pau5: DMAC_PAU,
    #[doc = "0x248 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr5: DMAC_DESC_ADDR,
    #[doc = "0x24c - DMAC Channel Configuration Register"]
    pub dmac_cfg5: DMAC_CFG,
    #[doc = "0x250 - DMAC Channel Current Source Register"]
    pub dmac_cur_src5: DMAC_CUR_SRC,
    #[doc = "0x254 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest5: DMAC_CUR_DEST,
    #[doc = "0x258 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left5: DMAC_BCNT_LEFT,
    #[doc = "0x25c - DMAC Channel Parameter Register"]
    pub dmac_para5: DMAC_PARA,
    _reserved69: [u8; 0x08],
    #[doc = "0x268 - DMAC Mode Register"]
    pub dmac_mode5: DMAC_MODE,
    #[doc = "0x26c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr5: DMAC_FDESC_ADDR,
    #[doc = "0x270 - DMAC Package Number Register"]
    pub dmac_pkg_num5: DMAC_PKG_NUM,
    _reserved72: [u8; 0x0c],
    #[doc = "0x280 - DMAC Channel Enable Register"]
    pub dmac_en6: DMAC_EN,
    #[doc = "0x284 - DMAC Channel Pause Register"]
    pub dmac_pau6: DMAC_PAU,
    #[doc = "0x288 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr6: DMAC_DESC_ADDR,
    #[doc = "0x28c - DMAC Channel Configuration Register"]
    pub dmac_cfg6: DMAC_CFG,
    #[doc = "0x290 - DMAC Channel Current Source Register"]
    pub dmac_cur_src6: DMAC_CUR_SRC,
    #[doc = "0x294 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest6: DMAC_CUR_DEST,
    #[doc = "0x298 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left6: DMAC_BCNT_LEFT,
    #[doc = "0x29c - DMAC Channel Parameter Register"]
    pub dmac_para6: DMAC_PARA,
    _reserved80: [u8; 0x08],
    #[doc = "0x2a8 - DMAC Mode Register"]
    pub dmac_mode6: DMAC_MODE,
    #[doc = "0x2ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr6: DMAC_FDESC_ADDR,
    #[doc = "0x2b0 - DMAC Package Number Register"]
    pub dmac_pkg_num6: DMAC_PKG_NUM,
    _reserved83: [u8; 0x0c],
    #[doc = "0x2c0 - DMAC Channel Enable Register"]
    pub dmac_en7: DMAC_EN,
    #[doc = "0x2c4 - DMAC Channel Pause Register"]
    pub dmac_pau7: DMAC_PAU,
    #[doc = "0x2c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr7: DMAC_DESC_ADDR,
    #[doc = "0x2cc - DMAC Channel Configuration Register"]
    pub dmac_cfg7: DMAC_CFG,
    #[doc = "0x2d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src7: DMAC_CUR_SRC,
    #[doc = "0x2d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest7: DMAC_CUR_DEST,
    #[doc = "0x2d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left7: DMAC_BCNT_LEFT,
    #[doc = "0x2dc - DMAC Channel Parameter Register"]
    pub dmac_para7: DMAC_PARA,
    _reserved91: [u8; 0x08],
    #[doc = "0x2e8 - DMAC Mode Register"]
    pub dmac_mode7: DMAC_MODE,
    #[doc = "0x2ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr7: DMAC_FDESC_ADDR,
    #[doc = "0x2f0 - DMAC Package Number Register"]
    pub dmac_pkg_num7: DMAC_PKG_NUM,
    _reserved94: [u8; 0x0c],
    #[doc = "0x300 - DMAC Channel Enable Register"]
    pub dmac_en8: DMAC_EN,
    #[doc = "0x304 - DMAC Channel Pause Register"]
    pub dmac_pau8: DMAC_PAU,
    #[doc = "0x308 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr8: DMAC_DESC_ADDR,
    #[doc = "0x30c - DMAC Channel Configuration Register"]
    pub dmac_cfg8: DMAC_CFG,
    #[doc = "0x310 - DMAC Channel Current Source Register"]
    pub dmac_cur_src8: DMAC_CUR_SRC,
    #[doc = "0x314 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest8: DMAC_CUR_DEST,
    #[doc = "0x318 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left8: DMAC_BCNT_LEFT,
    #[doc = "0x31c - DMAC Channel Parameter Register"]
    pub dmac_para8: DMAC_PARA,
    _reserved102: [u8; 0x08],
    #[doc = "0x328 - DMAC Mode Register"]
    pub dmac_mode8: DMAC_MODE,
    #[doc = "0x32c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr8: DMAC_FDESC_ADDR,
    #[doc = "0x330 - DMAC Package Number Register"]
    pub dmac_pkg_num8: DMAC_PKG_NUM,
    _reserved105: [u8; 0x0c],
    #[doc = "0x340 - DMAC Channel Enable Register"]
    pub dmac_en9: DMAC_EN,
    #[doc = "0x344 - DMAC Channel Pause Register"]
    pub dmac_pau9: DMAC_PAU,
    #[doc = "0x348 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr9: DMAC_DESC_ADDR,
    #[doc = "0x34c - DMAC Channel Configuration Register"]
    pub dmac_cfg9: DMAC_CFG,
    #[doc = "0x350 - DMAC Channel Current Source Register"]
    pub dmac_cur_src9: DMAC_CUR_SRC,
    #[doc = "0x354 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest9: DMAC_CUR_DEST,
    #[doc = "0x358 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left9: DMAC_BCNT_LEFT,
    #[doc = "0x35c - DMAC Channel Parameter Register"]
    pub dmac_para9: DMAC_PARA,
    _reserved113: [u8; 0x08],
    #[doc = "0x368 - DMAC Mode Register"]
    pub dmac_mode9: DMAC_MODE,
    #[doc = "0x36c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr9: DMAC_FDESC_ADDR,
    #[doc = "0x370 - DMAC Package Number Register"]
    pub dmac_pkg_num9: DMAC_PKG_NUM,
    _reserved116: [u8; 0x0c],
    #[doc = "0x380 - DMAC Channel Enable Register"]
    pub dmac_en10: DMAC_EN,
    #[doc = "0x384 - DMAC Channel Pause Register"]
    pub dmac_pau10: DMAC_PAU,
    #[doc = "0x388 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr10: DMAC_DESC_ADDR,
    #[doc = "0x38c - DMAC Channel Configuration Register"]
    pub dmac_cfg10: DMAC_CFG,
    #[doc = "0x390 - DMAC Channel Current Source Register"]
    pub dmac_cur_src10: DMAC_CUR_SRC,
    #[doc = "0x394 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest10: DMAC_CUR_DEST,
    #[doc = "0x398 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left10: DMAC_BCNT_LEFT,
    #[doc = "0x39c - DMAC Channel Parameter Register"]
    pub dmac_para10: DMAC_PARA,
    _reserved124: [u8; 0x08],
    #[doc = "0x3a8 - DMAC Mode Register"]
    pub dmac_mode10: DMAC_MODE,
    #[doc = "0x3ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr10: DMAC_FDESC_ADDR,
    #[doc = "0x3b0 - DMAC Package Number Register"]
    pub dmac_pkg_num10: DMAC_PKG_NUM,
    _reserved127: [u8; 0x0c],
    #[doc = "0x3c0 - DMAC Channel Enable Register"]
    pub dmac_en11: DMAC_EN,
    #[doc = "0x3c4 - DMAC Channel Pause Register"]
    pub dmac_pau11: DMAC_PAU,
    #[doc = "0x3c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr11: DMAC_DESC_ADDR,
    #[doc = "0x3cc - DMAC Channel Configuration Register"]
    pub dmac_cfg11: DMAC_CFG,
    #[doc = "0x3d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src11: DMAC_CUR_SRC,
    #[doc = "0x3d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest11: DMAC_CUR_DEST,
    #[doc = "0x3d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left11: DMAC_BCNT_LEFT,
    #[doc = "0x3dc - DMAC Channel Parameter Register"]
    pub dmac_para11: DMAC_PARA,
    _reserved135: [u8; 0x08],
    #[doc = "0x3e8 - DMAC Mode Register"]
    pub dmac_mode11: DMAC_MODE,
    #[doc = "0x3ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr11: DMAC_FDESC_ADDR,
    #[doc = "0x3f0 - DMAC Package Number Register"]
    pub dmac_pkg_num11: DMAC_PKG_NUM,
    _reserved138: [u8; 0x0c],
    #[doc = "0x400 - DMAC Channel Enable Register"]
    pub dmac_en12: DMAC_EN,
    #[doc = "0x404 - DMAC Channel Pause Register"]
    pub dmac_pau12: DMAC_PAU,
    #[doc = "0x408 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr12: DMAC_DESC_ADDR,
    #[doc = "0x40c - DMAC Channel Configuration Register"]
    pub dmac_cfg12: DMAC_CFG,
    #[doc = "0x410 - DMAC Channel Current Source Register"]
    pub dmac_cur_src12: DMAC_CUR_SRC,
    #[doc = "0x414 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest12: DMAC_CUR_DEST,
    #[doc = "0x418 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left12: DMAC_BCNT_LEFT,
    #[doc = "0x41c - DMAC Channel Parameter Register"]
    pub dmac_para12: DMAC_PARA,
    _reserved146: [u8; 0x08],
    #[doc = "0x428 - DMAC Mode Register"]
    pub dmac_mode12: DMAC_MODE,
    #[doc = "0x42c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr12: DMAC_FDESC_ADDR,
    #[doc = "0x430 - DMAC Package Number Register"]
    pub dmac_pkg_num12: DMAC_PKG_NUM,
    _reserved149: [u8; 0x0c],
    #[doc = "0x440 - DMAC Channel Enable Register"]
    pub dmac_en13: DMAC_EN,
    #[doc = "0x444 - DMAC Channel Pause Register"]
    pub dmac_pau13: DMAC_PAU,
    #[doc = "0x448 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr13: DMAC_DESC_ADDR,
    #[doc = "0x44c - DMAC Channel Configuration Register"]
    pub dmac_cfg13: DMAC_CFG,
    #[doc = "0x450 - DMAC Channel Current Source Register"]
    pub dmac_cur_src13: DMAC_CUR_SRC,
    #[doc = "0x454 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest13: DMAC_CUR_DEST,
    #[doc = "0x458 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left13: DMAC_BCNT_LEFT,
    #[doc = "0x45c - DMAC Channel Parameter Register"]
    pub dmac_para13: DMAC_PARA,
    _reserved157: [u8; 0x08],
    #[doc = "0x468 - DMAC Mode Register"]
    pub dmac_mode13: DMAC_MODE,
    #[doc = "0x46c - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr13: DMAC_FDESC_ADDR,
    #[doc = "0x470 - DMAC Package Number Register"]
    pub dmac_pkg_num13: DMAC_PKG_NUM,
    _reserved160: [u8; 0x0c],
    #[doc = "0x480 - DMAC Channel Enable Register"]
    pub dmac_en14: DMAC_EN,
    #[doc = "0x484 - DMAC Channel Pause Register"]
    pub dmac_pau14: DMAC_PAU,
    #[doc = "0x488 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr14: DMAC_DESC_ADDR,
    #[doc = "0x48c - DMAC Channel Configuration Register"]
    pub dmac_cfg14: DMAC_CFG,
    #[doc = "0x490 - DMAC Channel Current Source Register"]
    pub dmac_cur_src14: DMAC_CUR_SRC,
    #[doc = "0x494 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest14: DMAC_CUR_DEST,
    #[doc = "0x498 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left14: DMAC_BCNT_LEFT,
    #[doc = "0x49c - DMAC Channel Parameter Register"]
    pub dmac_para14: DMAC_PARA,
    _reserved168: [u8; 0x08],
    #[doc = "0x4a8 - DMAC Mode Register"]
    pub dmac_mode14: DMAC_MODE,
    #[doc = "0x4ac - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr14: DMAC_FDESC_ADDR,
    #[doc = "0x4b0 - DMAC Package Number Register"]
    pub dmac_pkg_num14: DMAC_PKG_NUM,
    _reserved171: [u8; 0x0c],
    #[doc = "0x4c0 - DMAC Channel Enable Register"]
    pub dmac_en15: DMAC_EN,
    #[doc = "0x4c4 - DMAC Channel Pause Register"]
    pub dmac_pau15: DMAC_PAU,
    #[doc = "0x4c8 - DMAC Channel Start Address Register"]
    pub dmac_desc_addr15: DMAC_DESC_ADDR,
    #[doc = "0x4cc - DMAC Channel Configuration Register"]
    pub dmac_cfg15: DMAC_CFG,
    #[doc = "0x4d0 - DMAC Channel Current Source Register"]
    pub dmac_cur_src15: DMAC_CUR_SRC,
    #[doc = "0x4d4 - DMAC Channel Current Destination Register"]
    pub dmac_cur_dest15: DMAC_CUR_DEST,
    #[doc = "0x4d8 - DMAC Channel Byte Counter Left Register"]
    pub dmac_bcnt_left15: DMAC_BCNT_LEFT,
    #[doc = "0x4dc - DMAC Channel Parameter Register"]
    pub dmac_para15: DMAC_PARA,
    _reserved179: [u8; 0x08],
    #[doc = "0x4e8 - DMAC Mode Register"]
    pub dmac_mode15: DMAC_MODE,
    #[doc = "0x4ec - DMAC Former Descriptor Address Register"]
    pub dmac_fdesc_addr15: DMAC_FDESC_ADDR,
    #[doc = "0x4f0 - DMAC Package Number Register"]
    pub dmac_pkg_num15: DMAC_PKG_NUM,
}
#[doc = "dmac_irq_en0 (rw) register accessor: an alias for `Reg<DMAC_IRQ_EN0_SPEC>`"]
pub type DMAC_IRQ_EN0 = crate::Reg<dmac_irq_en0::DMAC_IRQ_EN0_SPEC>;
#[doc = "DMAC IRQ Enable Register 0"]
pub mod dmac_irq_en0;
#[doc = "dmac_irq_en1 (rw) register accessor: an alias for `Reg<DMAC_IRQ_EN1_SPEC>`"]
pub type DMAC_IRQ_EN1 = crate::Reg<dmac_irq_en1::DMAC_IRQ_EN1_SPEC>;
#[doc = "DMAC IRQ Enable Register 1"]
pub mod dmac_irq_en1;
#[doc = "dmac_irq_pend0 (rw) register accessor: an alias for `Reg<DMAC_IRQ_PEND0_SPEC>`"]
pub type DMAC_IRQ_PEND0 = crate::Reg<dmac_irq_pend0::DMAC_IRQ_PEND0_SPEC>;
#[doc = "DMAC IRQ Pending Register 0"]
pub mod dmac_irq_pend0;
#[doc = "dmac_irq_pend1 (rw) register accessor: an alias for `Reg<DMAC_IRQ_PEND1_SPEC>`"]
pub type DMAC_IRQ_PEND1 = crate::Reg<dmac_irq_pend1::DMAC_IRQ_PEND1_SPEC>;
#[doc = "DMAC IRQ Pending Register 1"]
pub mod dmac_irq_pend1;
#[doc = "dmac_auto_gate (rw) register accessor: an alias for `Reg<DMAC_AUTO_GATE_SPEC>`"]
pub type DMAC_AUTO_GATE = crate::Reg<dmac_auto_gate::DMAC_AUTO_GATE_SPEC>;
#[doc = "DMAC Auto Gating Register"]
pub mod dmac_auto_gate;
#[doc = "dmac_sta (r) register accessor: an alias for `Reg<DMAC_STA_SPEC>`"]
pub type DMAC_STA = crate::Reg<dmac_sta::DMAC_STA_SPEC>;
#[doc = "DMAC Status Register"]
pub mod dmac_sta;
#[doc = "dmac_en (rw) register accessor: an alias for `Reg<DMAC_EN_SPEC>`"]
pub type DMAC_EN = crate::Reg<dmac_en::DMAC_EN_SPEC>;
#[doc = "DMAC Channel Enable Register"]
pub mod dmac_en;
#[doc = "dmac_pau (rw) register accessor: an alias for `Reg<DMAC_PAU_SPEC>`"]
pub type DMAC_PAU = crate::Reg<dmac_pau::DMAC_PAU_SPEC>;
#[doc = "DMAC Channel Pause Register"]
pub mod dmac_pau;
#[doc = "dmac_desc_addr (rw) register accessor: an alias for `Reg<DMAC_DESC_ADDR_SPEC>`"]
pub type DMAC_DESC_ADDR = crate::Reg<dmac_desc_addr::DMAC_DESC_ADDR_SPEC>;
#[doc = "DMAC Channel Start Address Register"]
pub mod dmac_desc_addr;
#[doc = "dmac_cfg (r) register accessor: an alias for `Reg<DMAC_CFG_SPEC>`"]
pub type DMAC_CFG = crate::Reg<dmac_cfg::DMAC_CFG_SPEC>;
#[doc = "DMAC Channel Configuration Register"]
pub mod dmac_cfg;
#[doc = "dmac_cur_src (r) register accessor: an alias for `Reg<DMAC_CUR_SRC_SPEC>`"]
pub type DMAC_CUR_SRC = crate::Reg<dmac_cur_src::DMAC_CUR_SRC_SPEC>;
#[doc = "DMAC Channel Current Source Register"]
pub mod dmac_cur_src;
#[doc = "dmac_cur_dest (r) register accessor: an alias for `Reg<DMAC_CUR_DEST_SPEC>`"]
pub type DMAC_CUR_DEST = crate::Reg<dmac_cur_dest::DMAC_CUR_DEST_SPEC>;
#[doc = "DMAC Channel Current Destination Register"]
pub mod dmac_cur_dest;
#[doc = "dmac_bcnt_left (r) register accessor: an alias for `Reg<DMAC_BCNT_LEFT_SPEC>`"]
pub type DMAC_BCNT_LEFT = crate::Reg<dmac_bcnt_left::DMAC_BCNT_LEFT_SPEC>;
#[doc = "DMAC Channel Byte Counter Left Register"]
pub mod dmac_bcnt_left;
#[doc = "dmac_para (r) register accessor: an alias for `Reg<DMAC_PARA_SPEC>`"]
pub type DMAC_PARA = crate::Reg<dmac_para::DMAC_PARA_SPEC>;
#[doc = "DMAC Channel Parameter Register"]
pub mod dmac_para;
#[doc = "dmac_mode (rw) register accessor: an alias for `Reg<DMAC_MODE_SPEC>`"]
pub type DMAC_MODE = crate::Reg<dmac_mode::DMAC_MODE_SPEC>;
#[doc = "DMAC Mode Register"]
pub mod dmac_mode;
#[doc = "dmac_fdesc_addr (r) register accessor: an alias for `Reg<DMAC_FDESC_ADDR_SPEC>`"]
pub type DMAC_FDESC_ADDR = crate::Reg<dmac_fdesc_addr::DMAC_FDESC_ADDR_SPEC>;
#[doc = "DMAC Former Descriptor Address Register"]
pub mod dmac_fdesc_addr;
#[doc = "dmac_pkg_num (r) register accessor: an alias for `Reg<DMAC_PKG_NUM_SPEC>`"]
pub type DMAC_PKG_NUM = crate::Reg<dmac_pkg_num::DMAC_PKG_NUM_SPEC>;
#[doc = "DMAC Package Number Register"]
pub mod dmac_pkg_num;
