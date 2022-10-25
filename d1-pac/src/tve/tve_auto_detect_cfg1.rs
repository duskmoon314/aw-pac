#[doc = "Register `tve_auto_detect_cfg1` reader"]
pub struct R(crate::R<TVE_AUTO_DETECT_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_AUTO_DETECT_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_AUTO_DETECT_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_AUTO_DETECT_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_auto_detect_cfg1` writer"]
pub struct W(crate::W<TVE_AUTO_DETECT_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_AUTO_DETECT_CFG1_SPEC>;
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
impl From<crate::W<TVE_AUTO_DETECT_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_AUTO_DETECT_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `detect_pulse_start` reader - Detect signal start time"]
pub type DETECT_PULSE_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `detect_pulse_start` writer - Detect signal start time"]
pub type DETECT_PULSE_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_AUTO_DETECT_CFG1_SPEC, u16, u16, 15, O>;
#[doc = "Field `detect_pulse_periods` reader - Use 32K clock"]
pub type DETECT_PULSE_PERIODS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `detect_pulse_periods` writer - Use 32K clock"]
pub type DETECT_PULSE_PERIODS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_AUTO_DETECT_CFG1_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Detect signal start time"]
    #[inline(always)]
    pub fn detect_pulse_start(&self) -> DETECT_PULSE_START_R {
        DETECT_PULSE_START_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Use 32K clock"]
    #[inline(always)]
    pub fn detect_pulse_periods(&self) -> DETECT_PULSE_PERIODS_R {
        DETECT_PULSE_PERIODS_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Detect signal start time"]
    #[inline(always)]
    #[must_use]
    pub fn detect_pulse_start(&mut self) -> DETECT_PULSE_START_W<0> {
        DETECT_PULSE_START_W::new(self)
    }
    #[doc = "Bits 16:30 - Use 32K clock"]
    #[inline(always)]
    #[must_use]
    pub fn detect_pulse_periods(&mut self) -> DETECT_PULSE_PERIODS_W<16> {
        DETECT_PULSE_PERIODS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Auto Detect Configuration Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_auto_detect_cfg1](index.html) module"]
pub struct TVE_AUTO_DETECT_CFG1_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECT_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_auto_detect_cfg1::R](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECT_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_auto_detect_cfg1::W](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECT_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detect_cfg1 to value 0"]
impl crate::Resettable for TVE_AUTO_DETECT_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
