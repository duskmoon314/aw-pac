#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - Message Box Read Interrupt Enable Register"]
    pub msgbox_rd_irq_en: MSGBOX_RD_IRQ_EN,
    #[doc = "0x24 - Message Box Read Interrupt Status Register"]
    pub msgbox_rd_irq_status: MSGBOX_RD_IRQ_STATUS,
    _reserved2: [u8; 0x08],
    #[doc = "0x30 - Message Box Write Interrupt Enable Register"]
    pub msgbox_wr_irq_en: MSGBOX_WR_IRQ_EN,
    #[doc = "0x34 - Message Box Write Interrupt Status Register"]
    pub msgbox_wr_irq_status: MSGBOX_WR_IRQ_STATUS,
    _reserved4: [u8; 0x08],
    #[doc = "0x40 - Message Box Debug Register"]
    pub msgbox_debug: MSGBOX_DEBUG,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50..0x60 - Message Box FIFO Status Register"]
    pub msgbox_fifo_status: [MSGBOX_FIFO_STATUS; 4],
    #[doc = "0x60..0x70 - Message Box Message Status Register"]
    pub msgbox_msg_status: [MSGBOX_MSG_STATUS; 4],
    #[doc = "0x70..0x80 - Message Box Message Queue Register"]
    pub msgbox_msg: [MSGBOX_MSG; 4],
    #[doc = "0x80..0x90 - Message Box Write Interrupt Threshold Register"]
    pub msgbox_wr_int_threshold: [MSGBOX_WR_INT_THRESHOLD; 4],
}
#[doc = "msgbox_rd_irq_en (rw) register accessor: an alias for `Reg<MSGBOX_RD_IRQ_EN_SPEC>`"]
pub type MSGBOX_RD_IRQ_EN = crate::Reg<msgbox_rd_irq_en::MSGBOX_RD_IRQ_EN_SPEC>;
#[doc = "Message Box Read Interrupt Enable Register"]
pub mod msgbox_rd_irq_en;
#[doc = "msgbox_rd_irq_status (rw) register accessor: an alias for `Reg<MSGBOX_RD_IRQ_STATUS_SPEC>`"]
pub type MSGBOX_RD_IRQ_STATUS = crate::Reg<msgbox_rd_irq_status::MSGBOX_RD_IRQ_STATUS_SPEC>;
#[doc = "Message Box Read Interrupt Status Register"]
pub mod msgbox_rd_irq_status;
#[doc = "msgbox_wr_irq_en (rw) register accessor: an alias for `Reg<MSGBOX_WR_IRQ_EN_SPEC>`"]
pub type MSGBOX_WR_IRQ_EN = crate::Reg<msgbox_wr_irq_en::MSGBOX_WR_IRQ_EN_SPEC>;
#[doc = "Message Box Write Interrupt Enable Register"]
pub mod msgbox_wr_irq_en;
#[doc = "msgbox_wr_irq_status (rw) register accessor: an alias for `Reg<MSGBOX_WR_IRQ_STATUS_SPEC>`"]
pub type MSGBOX_WR_IRQ_STATUS = crate::Reg<msgbox_wr_irq_status::MSGBOX_WR_IRQ_STATUS_SPEC>;
#[doc = "Message Box Write Interrupt Status Register"]
pub mod msgbox_wr_irq_status;
#[doc = "msgbox_debug (rw) register accessor: an alias for `Reg<MSGBOX_DEBUG_SPEC>`"]
pub type MSGBOX_DEBUG = crate::Reg<msgbox_debug::MSGBOX_DEBUG_SPEC>;
#[doc = "Message Box Debug Register"]
pub mod msgbox_debug;
#[doc = "msgbox_fifo_status (r) register accessor: an alias for `Reg<MSGBOX_FIFO_STATUS_SPEC>`"]
pub type MSGBOX_FIFO_STATUS = crate::Reg<msgbox_fifo_status::MSGBOX_FIFO_STATUS_SPEC>;
#[doc = "Message Box FIFO Status Register"]
pub mod msgbox_fifo_status;
#[doc = "msgbox_msg_status (r) register accessor: an alias for `Reg<MSGBOX_MSG_STATUS_SPEC>`"]
pub type MSGBOX_MSG_STATUS = crate::Reg<msgbox_msg_status::MSGBOX_MSG_STATUS_SPEC>;
#[doc = "Message Box Message Status Register"]
pub mod msgbox_msg_status;
#[doc = "msgbox_msg (rw) register accessor: an alias for `Reg<MSGBOX_MSG_SPEC>`"]
pub type MSGBOX_MSG = crate::Reg<msgbox_msg::MSGBOX_MSG_SPEC>;
#[doc = "Message Box Message Queue Register"]
pub mod msgbox_msg;
#[doc = "msgbox_wr_int_threshold (rw) register accessor: an alias for `Reg<MSGBOX_WR_INT_THRESHOLD_SPEC>`"]
pub type MSGBOX_WR_INT_THRESHOLD =
    crate::Reg<msgbox_wr_int_threshold::MSGBOX_WR_INT_THRESHOLD_SPEC>;
#[doc = "Message Box Write Interrupt Threshold Register"]
pub mod msgbox_wr_int_threshold;
