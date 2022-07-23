#[doc = "Register `ac_adc_drc_lopl` reader"]
pub struct R(crate::R<AC_ADC_DRC_LOPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DRC_LOPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DRC_LOPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DRC_LOPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_drc_lopl` writer"]
pub struct W(crate::W<AC_ADC_DRC_LOPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DRC_LOPL_SPEC>;
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
impl From<crate::W<AC_ADC_DRC_LOPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DRC_LOPL_SPEC>) -> Self {
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
#[doc = "ADC DRC Limiter Low Output at Limiter Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_drc_lopl](index.html) module"]
pub struct AC_ADC_DRC_LOPL_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_LOPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_drc_lopl::R](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_LOPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_drc_lopl::W](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_LOPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ac_adc_drc_lopl to value 0"]
impl crate::Resettable for AC_ADC_DRC_LOPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
