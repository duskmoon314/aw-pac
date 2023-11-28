#[doc = r"Register block"]
#[repr(C)]
pub struct MSGBOX {
    _reserved0: [u8; 0x20],
    msgbox_rd_irq_en: MSGBOX_RD_IRQ_EN,
    msgbox_rd_irq_status: MSGBOX_RD_IRQ_STATUS,
    _reserved2: [u8; 0x08],
    msgbox_wr_irq_en: MSGBOX_WR_IRQ_EN,
    msgbox_wr_irq_status: MSGBOX_WR_IRQ_STATUS,
    _reserved4: [u8; 0x08],
    msgbox_debug: MSGBOX_DEBUG,
    _reserved5: [u8; 0x0c],
    msgbox_fifo_status: [MSGBOX_FIFO_STATUS; 4],
    msgbox_msg_status: [MSGBOX_MSG_STATUS; 4],
    msgbox_msg: [MSGBOX_MSG; 4],
    msgbox_wr_int_threshold: [MSGBOX_WR_INT_THRESHOLD; 4],
}
impl MSGBOX {
    #[doc = "0x20 - Message Box Read Interrupt Enable Register"]
    #[inline(always)]
    pub const fn msgbox_rd_irq_en(&self) -> &MSGBOX_RD_IRQ_EN {
        &self.msgbox_rd_irq_en
    }
    #[doc = "0x24 - Message Box Read Interrupt Status Register"]
    #[inline(always)]
    pub const fn msgbox_rd_irq_status(&self) -> &MSGBOX_RD_IRQ_STATUS {
        &self.msgbox_rd_irq_status
    }
    #[doc = "0x30 - Message Box Write Interrupt Enable Register"]
    #[inline(always)]
    pub const fn msgbox_wr_irq_en(&self) -> &MSGBOX_WR_IRQ_EN {
        &self.msgbox_wr_irq_en
    }
    #[doc = "0x34 - Message Box Write Interrupt Status Register"]
    #[inline(always)]
    pub const fn msgbox_wr_irq_status(&self) -> &MSGBOX_WR_IRQ_STATUS {
        &self.msgbox_wr_irq_status
    }
    #[doc = "0x40 - Message Box Debug Register"]
    #[inline(always)]
    pub const fn msgbox_debug(&self) -> &MSGBOX_DEBUG {
        &self.msgbox_debug
    }
    #[doc = "0x50..0x60 - Message Box FIFO Status Register"]
    #[inline(always)]
    pub const fn msgbox_fifo_status(&self, n: usize) -> &MSGBOX_FIFO_STATUS {
        &self.msgbox_fifo_status[n]
    }
    #[doc = "0x60..0x70 - Message Box Message Status Register"]
    #[inline(always)]
    pub const fn msgbox_msg_status(&self, n: usize) -> &MSGBOX_MSG_STATUS {
        &self.msgbox_msg_status[n]
    }
    #[doc = "0x70..0x80 - Message Box Message Queue Register"]
    #[inline(always)]
    pub const fn msgbox_msg(&self, n: usize) -> &MSGBOX_MSG {
        &self.msgbox_msg[n]
    }
    #[doc = "0x80..0x90 - Message Box Write Interrupt Threshold Register"]
    #[inline(always)]
    pub const fn msgbox_wr_int_threshold(&self, n: usize) -> &MSGBOX_WR_INT_THRESHOLD {
        &self.msgbox_wr_int_threshold[n]
    }
}
#[doc = "msgbox_rd_irq_en (rw) register accessor: Message Box Read Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_rd_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_rd_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_rd_irq_en`] module"]
pub type MSGBOX_RD_IRQ_EN = crate::Reg<msgbox_rd_irq_en::MSGBOX_RD_IRQ_EN_SPEC>;
#[doc = "Message Box Read Interrupt Enable Register"]
pub mod msgbox_rd_irq_en;
#[doc = "msgbox_rd_irq_status (rw) register accessor: Message Box Read Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_rd_irq_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_rd_irq_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_rd_irq_status`] module"]
pub type MSGBOX_RD_IRQ_STATUS = crate::Reg<msgbox_rd_irq_status::MSGBOX_RD_IRQ_STATUS_SPEC>;
#[doc = "Message Box Read Interrupt Status Register"]
pub mod msgbox_rd_irq_status;
#[doc = "msgbox_wr_irq_en (rw) register accessor: Message Box Write Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_wr_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_wr_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_wr_irq_en`] module"]
pub type MSGBOX_WR_IRQ_EN = crate::Reg<msgbox_wr_irq_en::MSGBOX_WR_IRQ_EN_SPEC>;
#[doc = "Message Box Write Interrupt Enable Register"]
pub mod msgbox_wr_irq_en;
#[doc = "msgbox_wr_irq_status (rw) register accessor: Message Box Write Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_wr_irq_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_wr_irq_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_wr_irq_status`] module"]
pub type MSGBOX_WR_IRQ_STATUS = crate::Reg<msgbox_wr_irq_status::MSGBOX_WR_IRQ_STATUS_SPEC>;
#[doc = "Message Box Write Interrupt Status Register"]
pub mod msgbox_wr_irq_status;
#[doc = "msgbox_debug (rw) register accessor: Message Box Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_debug`] module"]
pub type MSGBOX_DEBUG = crate::Reg<msgbox_debug::MSGBOX_DEBUG_SPEC>;
#[doc = "Message Box Debug Register"]
pub mod msgbox_debug;
#[doc = "msgbox_fifo_status (r) register accessor: Message Box FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_fifo_status`] module"]
pub type MSGBOX_FIFO_STATUS = crate::Reg<msgbox_fifo_status::MSGBOX_FIFO_STATUS_SPEC>;
#[doc = "Message Box FIFO Status Register"]
pub mod msgbox_fifo_status;
#[doc = "msgbox_msg_status (r) register accessor: Message Box Message Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_msg_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_msg_status`] module"]
pub type MSGBOX_MSG_STATUS = crate::Reg<msgbox_msg_status::MSGBOX_MSG_STATUS_SPEC>;
#[doc = "Message Box Message Status Register"]
pub mod msgbox_msg_status;
#[doc = "msgbox_msg (rw) register accessor: Message Box Message Queue Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_msg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_msg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_msg`] module"]
pub type MSGBOX_MSG = crate::Reg<msgbox_msg::MSGBOX_MSG_SPEC>;
#[doc = "Message Box Message Queue Register"]
pub mod msgbox_msg;
#[doc = "msgbox_wr_int_threshold (rw) register accessor: Message Box Write Interrupt Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_wr_int_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_wr_int_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_wr_int_threshold`] module"]
pub type MSGBOX_WR_INT_THRESHOLD =
    crate::Reg<msgbox_wr_int_threshold::MSGBOX_WR_INT_THRESHOLD_SPEC>;
#[doc = "Message Box Write Interrupt Threshold Register"]
pub mod msgbox_wr_int_threshold;
