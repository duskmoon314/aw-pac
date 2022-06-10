#[doc = "Register `HS_TMR%s_INTV_HI` reader"]
pub struct R(crate::R<HS_TMR_INTV_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_TMR_INTV_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_TMR_INTV_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_TMR_INTV_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HS_TMR%s_INTV_HI` writer"]
pub struct W(crate::W<HS_TMR_INTV_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_TMR_INTV_HI_SPEC>;
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
impl From<crate::W<HS_TMR_INTV_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_TMR_INTV_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HS_TMR_INTV_VALUE_HI` reader - "]
pub type HS_TMR_INTV_VALUE_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HS_TMR_INTV_VALUE_HI` writer - "]
pub type HS_TMR_INTV_VALUE_HI_W<'a> =
    crate::FieldWriter<'a, u32, HS_TMR_INTV_HI_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn hs_tmr_intv_value_hi(&self) -> HS_TMR_INTV_VALUE_HI_R {
        HS_TMR_INTV_VALUE_HI_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn hs_tmr_intv_value_hi(&mut self) -> HS_TMR_INTV_VALUE_HI_W {
        HS_TMR_INTV_VALUE_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS Timer Interval Value High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_tmr_intv_hi](index.html) module"]
pub struct HS_TMR_INTV_HI_SPEC;
impl crate::RegisterSpec for HS_TMR_INTV_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_tmr_intv_hi::R](R) reader structure"]
impl crate::Readable for HS_TMR_INTV_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_tmr_intv_hi::W](W) writer structure"]
impl crate::Writable for HS_TMR_INTV_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HS_TMR%s_INTV_HI to value 0"]
impl crate::Resettable for HS_TMR_INTV_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
