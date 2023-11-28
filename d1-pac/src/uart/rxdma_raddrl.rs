#[doc = "Register `rxdma_raddrl` reader"]
pub type R = crate::R<RXDMA_RADDRL_SPEC>;
#[doc = "Register `rxdma_raddrl` writer"]
pub type W = crate::W<RXDMA_RADDRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDMA_RADDRL_SPEC> {
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
#[doc = "UART RXDMA Read Address Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_raddrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_raddrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_RADDRL_SPEC;
impl crate::RegisterSpec for RXDMA_RADDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_raddrl::R`](R) reader structure"]
impl crate::Readable for RXDMA_RADDRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_raddrl::W`](W) writer structure"]
impl crate::Writable for RXDMA_RADDRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_raddrl to value 0"]
impl crate::Resettable for RXDMA_RADDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
