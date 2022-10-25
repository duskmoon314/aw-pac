#[doc = "Register `hs_tmr%s_curnt_hi` reader"]
pub struct R(crate::R<HS_TMR_CURNT_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_TMR_CURNT_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_TMR_CURNT_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_TMR_CURNT_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hs_tmr%s_curnt_hi` writer"]
pub struct W(crate::W<HS_TMR_CURNT_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_TMR_CURNT_HI_SPEC>;
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
impl From<crate::W<HS_TMR_CURNT_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_TMR_CURNT_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hs_tmr_cur_value_hi` reader - "]
pub type HS_TMR_CUR_VALUE_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `hs_tmr_cur_value_hi` writer - "]
pub type HS_TMR_CUR_VALUE_HI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HS_TMR_CURNT_HI_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn hs_tmr_cur_value_hi(&self) -> HS_TMR_CUR_VALUE_HI_R {
        HS_TMR_CUR_VALUE_HI_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_cur_value_hi(&mut self) -> HS_TMR_CUR_VALUE_HI_W<0> {
        HS_TMR_CUR_VALUE_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS Timer Current Value High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_tmr_curnt_hi](index.html) module"]
pub struct HS_TMR_CURNT_HI_SPEC;
impl crate::RegisterSpec for HS_TMR_CURNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_tmr_curnt_hi::R](R) reader structure"]
impl crate::Readable for HS_TMR_CURNT_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_tmr_curnt_hi::W](W) writer structure"]
impl crate::Writable for HS_TMR_CURNT_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hs_tmr%s_curnt_hi to value 0"]
impl crate::Resettable for HS_TMR_CURNT_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
