#[doc = "Register `ac_adc_drc_lkl` reader"]
pub struct R(crate::R<AC_ADC_DRC_LKL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DRC_LKL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DRC_LKL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DRC_LKL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_drc_lkl` writer"]
pub struct W(crate::W<AC_ADC_DRC_LKL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DRC_LKL_SPEC>;
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
impl From<crate::W<AC_ADC_DRC_LKL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DRC_LKL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_drc_lkl` reader - The slope of the limiter, which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is <50:1>)"]
pub type ADC_DRC_LKL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adc_drc_lkl` writer - The slope of the limiter, which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is <50:1>)"]
pub type ADC_DRC_LKL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_ADC_DRC_LKL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The slope of the limiter, which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is <50:1>)"]
    #[inline(always)]
    pub fn adc_drc_lkl(&self) -> ADC_DRC_LKL_R {
        ADC_DRC_LKL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The slope of the limiter, which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is <50:1>)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_lkl(&mut self) -> ADC_DRC_LKL_W<0> {
        ADC_DRC_LKL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC DRC Limiter Slope Low Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_drc_lkl](index.html) module"]
pub struct AC_ADC_DRC_LKL_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_LKL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_drc_lkl::R](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_LKL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_drc_lkl::W](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_LKL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_lkl to value 0x1eb8"]
impl crate::Resettable for AC_ADC_DRC_LKL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1eb8;
}
