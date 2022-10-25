#[doc = "Register `tve_auto_detection_debounce_setting` reader"]
pub struct R(crate::R<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_auto_detection_debounce_setting` writer"]
pub struct W(crate::W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>;
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
impl From<crate::W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac0_de_bounce_times` reader - The de_bounce time for hot plug detect function."]
pub type DAC0_DE_BOUNCE_TIMES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dac0_de_bounce_times` writer - The de_bounce time for hot plug detect function."]
pub type DAC0_DE_BOUNCE_TIMES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC, u8, u8, 4, O>;
#[doc = "Field `dac_test_register` reader - DAC test register."]
pub type DAC_TEST_REGISTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dac_test_register` writer - DAC test register."]
pub type DAC_TEST_REGISTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:3 - The de_bounce time for hot plug detect function."]
    #[inline(always)]
    pub fn dac0_de_bounce_times(&self) -> DAC0_DE_BOUNCE_TIMES_R {
        DAC0_DE_BOUNCE_TIMES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - DAC test register."]
    #[inline(always)]
    pub fn dac_test_register(&self) -> DAC_TEST_REGISTER_R {
        DAC_TEST_REGISTER_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - The de_bounce time for hot plug detect function."]
    #[inline(always)]
    #[must_use]
    pub fn dac0_de_bounce_times(&mut self) -> DAC0_DE_BOUNCE_TIMES_W<0> {
        DAC0_DE_BOUNCE_TIMES_W::new(self)
    }
    #[doc = "Bits 16:25 - DAC test register."]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_register(&mut self) -> DAC_TEST_REGISTER_W<16> {
        DAC_TEST_REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Auto Detection De-bounce Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_auto_detection_debounce_setting](index.html) module"]
pub struct TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_auto_detection_debounce_setting::R](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_auto_detection_debounce_setting::W](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detection_debounce_setting to value 0"]
impl crate::Resettable for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
