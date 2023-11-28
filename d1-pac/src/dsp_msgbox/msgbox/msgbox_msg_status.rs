#[doc = "Register `msgbox_msg_status%s` reader"]
pub type R = crate::R<MSGBOX_MSG_STATUS_SPEC>;
#[doc = "Field `msg_num` reader - Message Number\n\nNumber of unread messages in the message queue. Here, limit to eight messages per message queue."]
pub type MSG_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Message Number\n\nNumber of unread messages in the message queue. Here, limit to eight messages per message queue."]
    #[inline(always)]
    pub fn msg_num(&self) -> MSG_NUM_R {
        MSG_NUM_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Message Box Message Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_msg_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_MSG_STATUS_SPEC;
impl crate::RegisterSpec for MSGBOX_MSG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_msg_status::R`](R) reader structure"]
impl crate::Readable for MSGBOX_MSG_STATUS_SPEC {}
#[doc = "`reset()` method sets msgbox_msg_status%s to value 0"]
impl crate::Resettable for MSGBOX_MSG_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
