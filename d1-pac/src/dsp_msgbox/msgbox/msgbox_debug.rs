#[doc = "Register `msgbox_debug` reader"]
pub type R = crate::R<MSGBOX_DEBUG_SPEC>;
#[doc = "Register `msgbox_debug` writer"]
pub type W = crate::W<MSGBOX_DEBUG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MSGBOX_DEBUG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "Message Box Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_DEBUG_SPEC;
impl crate::RegisterSpec for MSGBOX_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_debug::R`](R) reader structure"]
impl crate::Readable for MSGBOX_DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_debug::W`](W) writer structure"]
impl crate::Writable for MSGBOX_DEBUG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_debug to value 0"]
impl crate::Resettable for MSGBOX_DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
