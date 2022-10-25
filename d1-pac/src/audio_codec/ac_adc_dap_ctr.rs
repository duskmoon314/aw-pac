#[doc = "Register `ac_adc_dap_ctr` reader"]
pub struct R(crate::R<AC_ADC_DAP_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DAP_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DAP_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DAP_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_dap_ctr` writer"]
pub struct W(crate::W<AC_ADC_DAP_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DAP_CTR_SPEC>;
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
impl From<crate::W<AC_ADC_DAP_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DAP_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_hpf_en[1,0]` reader - ADC HPF\\[i\\] enable control"]
pub type ADC_HPF_EN_R = crate::BitReader<ADC_HPF_EN_A>;
#[doc = "ADC HPF\\[i\\] enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_HPF_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_HPF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_HPF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_HPF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_HPF_EN_A {
        match self.bits {
            false => ADC_HPF_EN_A::DISABLED,
            true => ADC_HPF_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_HPF_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_HPF_EN_A::ENABLED
    }
}
#[doc = "Field `adc_hpf_en[1,0]` writer - ADC HPF\\[i\\] enable control"]
pub type ADC_HPF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DAP_CTR_SPEC, ADC_HPF_EN_A, O>;
impl<'a, const O: u8> ADC_HPF_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_HPF_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_HPF_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_en[1,0]` reader - ADC DRC\\[i\\] enable control"]
pub type ADC_DRC_EN_R = crate::BitReader<ADC_DRC_EN_A>;
#[doc = "ADC DRC\\[i\\] enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_EN_A {
        match self.bits {
            false => ADC_DRC_EN_A::DISABLED,
            true => ADC_DRC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_en[1,0]` writer - ADC DRC\\[i\\] enable control"]
pub type ADC_DRC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DAP_CTR_SPEC, ADC_DRC_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_dap_en[1,0]` reader - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
pub type ADC_DAP_EN_R = crate::BitReader<ADC_DAP_EN_A>;
#[doc = "DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DAP_EN_A {
    #[doc = "0: Bypassed"]
    BYPASSED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DAP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DAP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DAP_EN_A {
        match self.bits {
            false => ADC_DAP_EN_A::BYPASSED,
            true => ADC_DAP_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == ADC_DAP_EN_A::BYPASSED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DAP_EN_A::ENABLED
    }
}
#[doc = "Field `adc_dap_en[1,0]` writer - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
pub type ADC_DAP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DAP_CTR_SPEC, ADC_DAP_EN_A, O>;
impl<'a, const O: u8> ADC_DAP_EN_W<'a, O> {
    #[doc = "Bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(ADC_DAP_EN_A::BYPASSED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DAP_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 24 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_hpf1_en(&self) -> ADC_HPF_EN_R {
        ADC_HPF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_hpf0_en(&self) -> ADC_HPF_EN_R {
        ADC_HPF_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_drc1_en(&self) -> ADC_DRC_EN_R {
        ADC_DRC_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    pub fn adc_drc0_en(&self) -> ADC_DRC_EN_R {
        ADC_DRC_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 27 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    pub fn adc_dap1_en(&self) -> ADC_DAP_EN_R {
        ADC_DAP_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    pub fn adc_dap0_en(&self) -> ADC_DAP_EN_R {
        ADC_DAP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_hpf_en<const O: u8>(&mut self) -> ADC_HPF_EN_W<O> {
        ADC_HPF_EN_W::new(self)
    }
    #[doc = "Bit 24 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_hpf1_en(&mut self) -> ADC_HPF_EN_W<24> {
        ADC_HPF_EN_W::new(self)
    }
    #[doc = "Bit 28 - ADC HPF\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_hpf0_en(&mut self) -> ADC_HPF_EN_W<28> {
        ADC_HPF_EN_W::new(self)
    }
    #[doc = "ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_drc_en<const O: u8>(&mut self) -> ADC_DRC_EN_W<O> {
        ADC_DRC_EN_W::new(self)
    }
    #[doc = "Bit 25 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc1_en(&mut self) -> ADC_DRC_EN_W<25> {
        ADC_DRC_EN_W::new(self)
    }
    #[doc = "Bit 29 - ADC DRC\\[i\\] enable control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc0_en(&mut self) -> ADC_DRC_EN_W<29> {
        ADC_DRC_EN_W::new(self)
    }
    #[doc = "DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_dap_en<const O: u8>(&mut self) -> ADC_DAP_EN_W<O> {
        ADC_DAP_EN_W::new(self)
    }
    #[doc = "Bit 27 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dap1_en(&mut self) -> ADC_DAP_EN_W<27> {
        ADC_DAP_EN_W::new(self)
    }
    #[doc = "Bit 31 - DAP\\[i\\] for ADC enable\n\nDAP0 control ADC1/2\n\nDAP1 control ADC3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dap0_en(&mut self) -> ADC_DAP_EN_W<31> {
        ADC_DAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC DAP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_dap_ctr](index.html) module"]
pub struct AC_ADC_DAP_CTR_SPEC;
impl crate::RegisterSpec for AC_ADC_DAP_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_dap_ctr::R](R) reader structure"]
impl crate::Readable for AC_ADC_DAP_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_dap_ctr::W](W) writer structure"]
impl crate::Writable for AC_ADC_DAP_CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_dap_ctr to value 0"]
impl crate::Resettable for AC_ADC_DAP_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
