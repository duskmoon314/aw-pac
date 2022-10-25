#[doc = "Register `tvd_adc_ctl%s` reader"]
pub struct R(crate::R<TVD_ADC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVD_ADC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVD_ADC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVD_ADC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tvd_adc_ctl%s` writer"]
pub struct W(crate::W<TVD_ADC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVD_ADC_CTL_SPEC>;
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
impl From<crate::W<TVD_ADC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVD_ADC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TVD ADC CONTROL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tvd_adc_ctl](index.html) module"]
pub struct TVD_ADC_CTL_SPEC;
impl crate::RegisterSpec for TVD_ADC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tvd_adc_ctl::R](R) reader structure"]
impl crate::Readable for TVD_ADC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tvd_adc_ctl::W](W) writer structure"]
impl crate::Writable for TVD_ADC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tvd_adc_ctl%s to value 0"]
impl crate::Resettable for TVD_ADC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
