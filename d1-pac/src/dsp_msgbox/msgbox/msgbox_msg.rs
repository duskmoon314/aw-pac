#[doc = "Register `msgbox_msg%s` reader"]
pub type R = crate::R<MSGBOX_MSG_SPEC>;
#[doc = "Register `msgbox_msg%s` writer"]
pub type W = crate::W<MSGBOX_MSG_SPEC>;
#[doc = "Field `msg_que` reader - The message register stores the next to be read message of the message FIFO queue."]
pub type MSG_QUE_R = crate::FieldReader<u32>;
#[doc = "Field `msg_que` writer - The message register stores the next to be read message of the message FIFO queue."]
pub type MSG_QUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
    pub fn msg_que(&mut self) -> MSG_QUE_W<MSGBOX_MSG_SPEC> {
        MSG_QUE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Message Box Message Queue Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_msg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_msg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_MSG_SPEC;
impl crate::RegisterSpec for MSGBOX_MSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_msg::R`](R) reader structure"]
impl crate::Readable for MSGBOX_MSG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_msg::W`](W) writer structure"]
impl crate::Writable for MSGBOX_MSG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_msg%s to value 0"]
impl crate::Resettable for MSGBOX_MSG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
