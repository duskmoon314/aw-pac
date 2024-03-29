#[doc = "Register `hs_tmr%s_intv_lo` reader"]
pub type R = crate::R<HS_TMR_INTV_LO_SPEC>;
#[doc = "Register `hs_tmr%s_intv_lo` writer"]
pub type W = crate::W<HS_TMR_INTV_LO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<HS_TMR_INTV_LO_SPEC> {
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
#[doc = "HS Timer Interval Value Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_intv_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_intv_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_TMR_INTV_LO_SPEC;
impl crate::RegisterSpec for HS_TMR_INTV_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_tmr_intv_lo::R`](R) reader structure"]
impl crate::Readable for HS_TMR_INTV_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_tmr_intv_lo::W`](W) writer structure"]
impl crate::Writable for HS_TMR_INTV_LO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hs_tmr%s_intv_lo to value 0"]
impl crate::Resettable for HS_TMR_INTV_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
