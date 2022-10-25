#[doc = "Register `ac_adc_drc_rpfhat` reader"]
pub struct R(crate::R<AC_ADC_DRC_RPFHAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DRC_RPFHAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DRC_RPFHAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DRC_RPFHAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_drc_rpfhat` writer"]
pub struct W(crate::W<AC_ADC_DRC_RPFHAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DRC_RPFHAT_SPEC>;
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
impl From<crate::W<AC_ADC_DRC_RPFHAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DRC_RPFHAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_drc_rpfhat` reader - The right peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
pub type ADC_DRC_RPFHAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adc_drc_rpfhat` writer - The right peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
pub type ADC_DRC_RPFHAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_ADC_DRC_RPFHAT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The right peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
    #[inline(always)]
    pub fn adc_drc_rpfhat(&self) -> ADC_DRC_RPFHAT_R {
        ADC_DRC_RPFHAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The right peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_rpfhat(&mut self) -> ADC_DRC_RPFHAT_W<0> {
        ADC_DRC_RPFHAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC DRC Right Peak Filter High Attack Time Coef Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_drc_rpfhat](index.html) module"]
pub struct AC_ADC_DRC_RPFHAT_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_RPFHAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_drc_rpfhat::R](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_RPFHAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_drc_rpfhat::W](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_RPFHAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_rpfhat to value 0x0b"]
impl crate::Resettable for AC_ADC_DRC_RPFHAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
