#[doc = "Register `hc_fm_number` reader"]
pub struct R(crate::R<HC_FM_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_FM_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_FM_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_FM_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_fm_number` writer"]
pub struct W(crate::W<HC_FM_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_FM_NUMBER_SPEC>;
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
impl From<crate::W<HC_FM_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_FM_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_number` reader - FrameNumber\n\nThis is incremented when is re-loaded. It will be rolled over to 0x0 after 0x0ffff. When entering the USBOPERATIONAL state, this will be incremented automatically. The content will be written to HCCA after HC has incremented the FrameNumber at each frame boundary and sent a SOF but before HC reads the first ED in that Frame. After writing to HCCA, HC will set the StartofFrame in HcInterruptStatus"]
pub type FRAME_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `frame_number` writer - FrameNumber\n\nThis is incremented when is re-loaded. It will be rolled over to 0x0 after 0x0ffff. When entering the USBOPERATIONAL state, this will be incremented automatically. The content will be written to HCCA after HC has incremented the FrameNumber at each frame boundary and sent a SOF but before HC reads the first ED in that Frame. After writing to HCCA, HC will set the StartofFrame in HcInterruptStatus"]
pub type FRAME_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_FM_NUMBER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - FrameNumber\n\nThis is incremented when is re-loaded. It will be rolled over to 0x0 after 0x0ffff. When entering the USBOPERATIONAL state, this will be incremented automatically. The content will be written to HCCA after HC has incremented the FrameNumber at each frame boundary and sent a SOF but before HC reads the first ED in that Frame. After writing to HCCA, HC will set the StartofFrame in HcInterruptStatus"]
    #[inline(always)]
    pub fn frame_number(&self) -> FRAME_NUMBER_R {
        FRAME_NUMBER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FrameNumber\n\nThis is incremented when is re-loaded. It will be rolled over to 0x0 after 0x0ffff. When entering the USBOPERATIONAL state, this will be incremented automatically. The content will be written to HCCA after HC has incremented the FrameNumber at each frame boundary and sent a SOF but before HC reads the first ED in that Frame. After writing to HCCA, HC will set the StartofFrame in HcInterruptStatus"]
    #[inline(always)]
    #[must_use]
    pub fn frame_number(&mut self) -> FRAME_NUMBER_W<0> {
        FRAME_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_fm_number](index.html) module"]
pub struct HC_FM_NUMBER_SPEC;
impl crate::RegisterSpec for HC_FM_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_fm_number::R](R) reader structure"]
impl crate::Readable for HC_FM_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_fm_number::W](W) writer structure"]
impl crate::Writable for HC_FM_NUMBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_fm_number to value 0"]
impl crate::Resettable for HC_FM_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
