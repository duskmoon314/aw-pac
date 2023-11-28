#[doc = "Register `tvd_irq_status` reader"]
pub type R = crate::R<TVD_IRQ_STATUS_SPEC>;
#[doc = "Register `tvd_irq_status` writer"]
pub type W = crate::W<TVD_IRQ_STATUS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TVD_IRQ_STATUS_SPEC> {
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
#[doc = "TVD DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_irq_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_irq_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVD_IRQ_STATUS_SPEC;
impl crate::RegisterSpec for TVD_IRQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tvd_irq_status::R`](R) reader structure"]
impl crate::Readable for TVD_IRQ_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tvd_irq_status::W`](W) writer structure"]
impl crate::Writable for TVD_IRQ_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tvd_irq_status to value 0"]
impl crate::Resettable for TVD_IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
