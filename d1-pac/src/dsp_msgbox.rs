#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Communicate with CPU\\[N\\]"]
    pub msgbox: crate::ArrayProxy<MSGBOX, 2, 0x0100>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - Message Box Read Interrupt Enable Register"]
    pub msgbox_rd_irq_en_reg:
        crate::Reg<self::msgbox::msgbox_rd_irq_en_reg::MSGBOX_RD_IRQ_EN_REG_SPEC>,
    #[doc = "0x24 - Message Box Read Interrupt Status Register"]
    pub msgbox_rd_irq_status_reg:
        crate::Reg<self::msgbox::msgbox_rd_irq_status_reg::MSGBOX_RD_IRQ_STATUS_REG_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x30 - Message Box Write Interrupt Enable Register"]
    pub msgbox_wr_irq_en_reg:
        crate::Reg<self::msgbox::msgbox_wr_irq_en_reg::MSGBOX_WR_IRQ_EN_REG_SPEC>,
    #[doc = "0x34 - Message Box Write Interrupt Status Register"]
    pub msgbox_wr_irq_status_reg:
        crate::Reg<self::msgbox::msgbox_wr_irq_status_reg::MSGBOX_WR_IRQ_STATUS_REG_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x40 - Message Box Debug Register"]
    pub msgbox_debug_reg: crate::Reg<self::msgbox::msgbox_debug_reg::MSGBOX_DEBUG_REG_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50..0x60 - Message Box FIFO Status Register"]
    pub msgbox_fifo_status_reg:
        [crate::Reg<self::msgbox::msgbox_fifo_status_reg::MSGBOX_FIFO_STATUS_REG_SPEC>; 4],
    #[doc = "0x60..0x70 - Message Box Message Status Register"]
    pub msgbox_msg_status_reg:
        [crate::Reg<self::msgbox::msgbox_msg_status_reg::MSGBOX_MSG_STATUS_REG_SPEC>; 4],
    #[doc = "0x70..0x80 - Message Box Message Queue Register"]
    pub msgbox_msg_reg: [crate::Reg<self::msgbox::msgbox_msg_reg::MSGBOX_MSG_REG_SPEC>; 4],
    #[doc = "0x80..0x90 - Message Box Write Interrupt Threshold Register"]
    pub msgbox_wr_int_threshold_reg: [crate::Reg<
        self::msgbox::msgbox_wr_int_threshold_reg::MSGBOX_WR_INT_THRESHOLD_REG_SPEC,
    >; 4],
}
#[doc = r"Register block"]
#[doc = "Communicate with CPU\\[N\\]"]
pub mod msgbox;
