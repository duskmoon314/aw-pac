#[doc = "Register `tve_auto_detect_cfg0` reader"]
pub struct R(crate::R<TVE_AUTO_DETECT_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_AUTO_DETECT_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_AUTO_DETECT_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_AUTO_DETECT_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_auto_detect_cfg0` writer"]
pub struct W(crate::W<TVE_AUTO_DETECT_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_AUTO_DETECT_CFG0_SPEC>;
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
impl From<crate::W<TVE_AUTO_DETECT_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_AUTO_DETECT_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `detect_pulse_value` reader - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
pub type DETECT_PULSE_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `detect_pulse_value` writer - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
pub type DETECT_PULSE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_AUTO_DETECT_CFG0_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
    #[inline(always)]
    pub fn detect_pulse_value(&self) -> DETECT_PULSE_VALUE_R {
        DETECT_PULSE_VALUE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Use for DAC data input at auto detect pluse. Set the pulse amplitude."]
    #[inline(always)]
    #[must_use]
    pub fn detect_pulse_value(&mut self) -> DETECT_PULSE_VALUE_W<0> {
        DETECT_PULSE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Auto Detect Configuration Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_auto_detect_cfg0](index.html) module"]
pub struct TVE_AUTO_DETECT_CFG0_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECT_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_auto_detect_cfg0::R](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECT_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_auto_detect_cfg0::W](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECT_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detect_cfg0 to value 0"]
impl crate::Resettable for TVE_AUTO_DETECT_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
