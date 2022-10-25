#[doc = "Register `msgbox_msg%s` reader"]
pub struct R(crate::R<MSGBOX_MSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_MSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_MSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_MSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msgbox_msg%s` writer"]
pub struct W(crate::W<MSGBOX_MSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_MSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MSGBOX_MSG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_MSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `msg_que` reader - The message register stores the next to be read message of the message FIFO queue."]
pub type MSG_QUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `msg_que` writer - The message register stores the next to be read message of the message FIFO queue."]
pub type MSG_QUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSGBOX_MSG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The message register stores the next to be read message of the message FIFO queue."]
    #[inline(always)]
    pub fn msg_que(&self) -> MSG_QUE_R {
        MSG_QUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The message register stores the next to be read message of the message FIFO queue."]
    #[inline(always)]
    #[must_use]
    pub fn msg_que(&mut self) -> MSG_QUE_W<0> {
        MSG_QUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Box Message Queue Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_msg](index.html) module"]
pub struct MSGBOX_MSG_SPEC;
impl crate::RegisterSpec for MSGBOX_MSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_msg::R](R) reader structure"]
impl crate::Readable for MSGBOX_MSG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_msg::W](W) writer structure"]
impl crate::Writable for MSGBOX_MSG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_msg%s to value 0"]
impl crate::Resettable for MSGBOX_MSG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
