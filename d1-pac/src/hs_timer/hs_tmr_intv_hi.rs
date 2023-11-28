#[doc = "Register `hs_tmr%s_intv_hi` reader"]
pub type R = crate::R<HS_TMR_INTV_HI_SPEC>;
#[doc = "Register `hs_tmr%s_intv_hi` writer"]
pub type W = crate::W<HS_TMR_INTV_HI_SPEC>;
#[doc = "Field `hs_tmr_intv_value_hi` reader - "]
pub type HS_TMR_INTV_VALUE_HI_R = crate::FieldReader<u32>;
#[doc = "Field `hs_tmr_intv_value_hi` writer - "]
pub type HS_TMR_INTV_VALUE_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn hs_tmr_intv_value_hi(&self) -> HS_TMR_INTV_VALUE_HI_R {
        HS_TMR_INTV_VALUE_HI_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_intv_value_hi(&mut self) -> HS_TMR_INTV_VALUE_HI_W<HS_TMR_INTV_HI_SPEC> {
        HS_TMR_INTV_VALUE_HI_W::new(self, 0)
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
#[doc = "HS Timer Interval Value High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_intv_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_intv_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_TMR_INTV_HI_SPEC;
impl crate::RegisterSpec for HS_TMR_INTV_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_tmr_intv_hi::R`](R) reader structure"]
impl crate::Readable for HS_TMR_INTV_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_tmr_intv_hi::W`](W) writer structure"]
impl crate::Writable for HS_TMR_INTV_HI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hs_tmr%s_intv_hi to value 0"]
impl crate::Resettable for HS_TMR_INTV_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
