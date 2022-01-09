#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - MSGBOX Read IRQ Enable Register"]
    pub msgbox_rd_irq_en_reg_0: crate::Reg<msgbox_rd_irq_en_reg_::MSGBOX_RD_IRQ_EN_REG__SPEC>,
    #[doc = "0x24 - MSGBOX Read IRQ Status Register"]
    pub msgbox_rd_irq_status_reg_0:
        crate::Reg<msgbox_rd_irq_status_reg_::MSGBOX_RD_IRQ_STATUS_REG__SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x30 - MSGBOX Write IRQ Enable Register"]
    pub msgbox_wr_irq_en_reg_0: crate::Reg<msgbox_wr_irq_en_reg_::MSGBOX_WR_IRQ_EN_REG__SPEC>,
    #[doc = "0x34 - MSGBOX Write IRQ Status Register"]
    pub msgbox_wr_irq_status_reg_0:
        crate::Reg<msgbox_wr_irq_status_reg_::MSGBOX_WR_IRQ_STATUS_REG__SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x40 - MSGBOX Debug Register"]
    pub msgbox_debug_reg_0: crate::Reg<msgbox_debug_reg_::MSGBOX_DEBUG_REG__SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50..0x5c - MSGBOX FIFO Status Register"]
    pub msgbox_fifo_status_reg_n0: MSGBOX_FIFO_STATUS_REG_N,
    _reserved6: [u8; 0x04],
    #[doc = "0x60..0x6c - MSGBOX Message Status Register"]
    pub msgbox_msg_status_reg_n0: MSGBOX_MSG_STATUS_REG_N,
    _reserved7: [u8; 0x04],
    #[doc = "0x70..0x7c - MSGBOX Message Queue Register"]
    pub msgbox_msg_reg_n0: MSGBOX_MSG_REG_N,
    _reserved8: [u8; 0x04],
    #[doc = "0x80..0x8c - MSGBOX Write IRQ Threshold Register"]
    pub msgbox_wr_int_threshold_reg_n0: MSGBOX_WR_INT_THRESHOLD_REG_N,
    _reserved9: [u8; 0x94],
    #[doc = "0x120 - MSGBOX Read IRQ Enable Register"]
    pub msgbox_rd_irq_en_reg_1: crate::Reg<msgbox_rd_irq_en_reg_::MSGBOX_RD_IRQ_EN_REG__SPEC>,
    #[doc = "0x124 - MSGBOX Read IRQ Status Register"]
    pub msgbox_rd_irq_status_reg_1:
        crate::Reg<msgbox_rd_irq_status_reg_::MSGBOX_RD_IRQ_STATUS_REG__SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x130 - MSGBOX Write IRQ Enable Register"]
    pub msgbox_wr_irq_en_reg_1: crate::Reg<msgbox_wr_irq_en_reg_::MSGBOX_WR_IRQ_EN_REG__SPEC>,
    #[doc = "0x134 - MSGBOX Write IRQ Status Register"]
    pub msgbox_wr_irq_status_reg_1:
        crate::Reg<msgbox_wr_irq_status_reg_::MSGBOX_WR_IRQ_STATUS_REG__SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x140 - MSGBOX Debug Register"]
    pub msgbox_debug_reg_1: crate::Reg<msgbox_debug_reg_::MSGBOX_DEBUG_REG__SPEC>,
    _reserved14: [u8; 0x0c],
    #[doc = "0x150..0x15c - MSGBOX FIFO Status Register"]
    pub msgbox_fifo_status_reg_n1: MSGBOX_FIFO_STATUS_REG_N,
    _reserved15: [u8; 0x04],
    #[doc = "0x160..0x16c - MSGBOX Message Status Register"]
    pub msgbox_msg_status_reg_n1: MSGBOX_MSG_STATUS_REG_N,
    _reserved16: [u8; 0x04],
    #[doc = "0x170..0x17c - MSGBOX Message Queue Register"]
    pub msgbox_msg_reg_n1: MSGBOX_MSG_REG_N,
    _reserved17: [u8; 0x04],
    #[doc = "0x180..0x18c - MSGBOX Write IRQ Threshold Register"]
    pub msgbox_wr_int_threshold_reg_n1: MSGBOX_WR_INT_THRESHOLD_REG_N,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX_FIFO_STATUS_REG_N {
    #[doc = "0x00..0x0c - MSGBOX FIFO Status Register"]
    pub msgbox_fifo_status_reg_p: [crate::Reg<
        self::msgbox_fifo_status_reg_n::msgbox_fifo_status_reg_p::MSGBOX_FIFO_STATUS_REG_P_SPEC,
    >; 3],
}
#[doc = r"Register block"]
#[doc = "MSGBOX FIFO Status Register"]
pub mod msgbox_fifo_status_reg_n;
#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX_MSG_STATUS_REG_N {
    #[doc = "0x00..0x0c - MSGBOX Message Status Register"]
    pub msgbox_msg_status_reg_p: [crate::Reg<
        self::msgbox_msg_status_reg_n::msgbox_msg_status_reg_p::MSGBOX_MSG_STATUS_REG_P_SPEC,
    >; 3],
}
#[doc = r"Register block"]
#[doc = "MSGBOX Message Status Register"]
pub mod msgbox_msg_status_reg_n;
#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX_MSG_REG_N {
    #[doc = "0x00..0x0c - MSGBOX Message Queue Register"]
    pub msgbox_msg_reg_p:
        [crate::Reg<self::msgbox_msg_reg_n::msgbox_msg_reg_p::MSGBOX_MSG_REG_P_SPEC>; 3],
}
#[doc = r"Register block"]
#[doc = "MSGBOX Message Queue Register"]
pub mod msgbox_msg_reg_n;
#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX_WR_INT_THRESHOLD_REG_N { # [ doc = "0x00..0x0c - MSGBOX Write IRQ Threshold Register" ] pub msgbox_wr_int_threshold_reg_p : [ crate :: Reg < self :: msgbox_wr_int_threshold_reg_n :: msgbox_wr_int_threshold_reg_p :: MSGBOX_WR_INT_THRESHOLD_REG_P_SPEC > ; 3 ] , }
#[doc = r"Register block"]
#[doc = "MSGBOX Write IRQ Threshold Register"]
pub mod msgbox_wr_int_threshold_reg_n;
#[doc = "MSGBOX_RD_IRQ_EN_REG_ register accessor: an alias for `Reg<MSGBOX_RD_IRQ_EN_REG__SPEC>`"]
pub type MSGBOX_RD_IRQ_EN_REG_ = crate::Reg<msgbox_rd_irq_en_reg_::MSGBOX_RD_IRQ_EN_REG__SPEC>;
#[doc = "MSGBOX Read IRQ Enable Register"]
pub mod msgbox_rd_irq_en_reg_;
#[doc = "MSGBOX_RD_IRQ_STATUS_REG_ register accessor: an alias for `Reg<MSGBOX_RD_IRQ_STATUS_REG__SPEC>`"]
pub type MSGBOX_RD_IRQ_STATUS_REG_ =
    crate::Reg<msgbox_rd_irq_status_reg_::MSGBOX_RD_IRQ_STATUS_REG__SPEC>;
#[doc = "MSGBOX Read IRQ Status Register"]
pub mod msgbox_rd_irq_status_reg_;
#[doc = "MSGBOX_WR_IRQ_EN_REG_ register accessor: an alias for `Reg<MSGBOX_WR_IRQ_EN_REG__SPEC>`"]
pub type MSGBOX_WR_IRQ_EN_REG_ = crate::Reg<msgbox_wr_irq_en_reg_::MSGBOX_WR_IRQ_EN_REG__SPEC>;
#[doc = "MSGBOX Write IRQ Enable Register"]
pub mod msgbox_wr_irq_en_reg_;
#[doc = "MSGBOX_WR_IRQ_STATUS_REG_ register accessor: an alias for `Reg<MSGBOX_WR_IRQ_STATUS_REG__SPEC>`"]
pub type MSGBOX_WR_IRQ_STATUS_REG_ =
    crate::Reg<msgbox_wr_irq_status_reg_::MSGBOX_WR_IRQ_STATUS_REG__SPEC>;
#[doc = "MSGBOX Write IRQ Status Register"]
pub mod msgbox_wr_irq_status_reg_;
#[doc = "MSGBOX_DEBUG_REG_ register accessor: an alias for `Reg<MSGBOX_DEBUG_REG__SPEC>`"]
pub type MSGBOX_DEBUG_REG_ = crate::Reg<msgbox_debug_reg_::MSGBOX_DEBUG_REG__SPEC>;
#[doc = "MSGBOX Debug Register"]
pub mod msgbox_debug_reg_;
