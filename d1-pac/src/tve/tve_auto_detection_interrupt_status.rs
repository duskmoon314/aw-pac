#[doc = "Register `tve_auto_detection_interrupt_status` reader"]
pub struct R(crate::R<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_auto_detection_interrupt_status` writer"]
pub struct W(crate::W<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>;
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
impl From<crate::W<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac0_auto_detect_interrupt_active_flag` reader - Write 1 to inactive DAC0 auto detection interrupt"]
pub type DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `dac0_auto_detect_interrupt_active_flag` writer - Write 1 to inactive DAC0 auto detection interrupt"]
pub type DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to inactive DAC0 auto detection interrupt"]
    #[inline(always)]
    pub fn dac0_auto_detect_interrupt_active_flag(
        &self,
    ) -> DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_R {
        DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to inactive DAC0 auto detection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_auto_detect_interrupt_active_flag(
        &mut self,
    ) -> DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_W<0> {
        DAC0_AUTO_DETECT_INTERRUPT_ACTIVE_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Auto Detection Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_auto_detection_interrupt_status](index.html) module"]
pub struct TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_auto_detection_interrupt_status::R](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_auto_detection_interrupt_status::W](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets tve_auto_detection_interrupt_status to value 0"]
impl crate::Resettable for TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
