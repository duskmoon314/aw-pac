#[doc = "Register `MSGBOX_MSG_STATUS_REG%s` reader"]
pub struct R(crate::R<MSGBOX_MSG_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_MSG_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_MSG_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_MSG_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSG_NUM` reader - Message Number\n\nNumber of unread messages in the message queue. Here, limit to eight messages per message queue."]
pub type MSG_NUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Message Number\n\nNumber of unread messages in the message queue. Here, limit to eight messages per message queue."]
    #[inline(always)]
    pub fn msg_num(&self) -> MSG_NUM_R {
        MSG_NUM_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Message Box Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_msg_status_reg](index.html) module"]
pub struct MSGBOX_MSG_STATUS_REG_SPEC;
impl crate::RegisterSpec for MSGBOX_MSG_STATUS_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_msg_status_reg::R](R) reader structure"]
impl crate::Readable for MSGBOX_MSG_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSGBOX_MSG_STATUS_REG%s to value 0"]
impl crate::Resettable for MSGBOX_MSG_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
