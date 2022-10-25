#[doc = "Register `hc_fm_remaining` reader"]
pub struct R(crate::R<HC_FM_REMAINING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_FM_REMAINING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_FM_REMAINING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_FM_REMAINING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_fm_remaining` writer"]
pub struct W(crate::W<HC_FM_REMAINING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_FM_REMAINING_SPEC>;
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
impl From<crate::W<HC_FM_REMAINING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_FM_REMAINING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_remaining` reader - FrameRemaining\n\nThis counter is decremented at each bit time. When it reaches zero, it is reset by loading the FrameInterval value specified in at the next bit time boundary. When entering the USBOPERATIONAL state, HC re-loads the content with the FrameInterval of and uses the updated value from the next SOF."]
pub type FRAME_REMAINING_R = crate::FieldReader<u16, u16>;
#[doc = "Field `frame_remaining` writer - FrameRemaining\n\nThis counter is decremented at each bit time. When it reaches zero, it is reset by loading the FrameInterval value specified in at the next bit time boundary. When entering the USBOPERATIONAL state, HC re-loads the content with the FrameInterval of and uses the updated value from the next SOF."]
pub type FRAME_REMAINING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_FM_REMAINING_SPEC, u16, u16, 14, O>;
#[doc = "Field `frame_remaining_toggle` reader - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
pub type FRAME_REMAINING_TOGGLE_R = crate::BitReader<bool>;
#[doc = "Field `frame_remaining_toggle` writer - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
pub type FRAME_REMAINING_TOGGLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_FM_REMAINING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - FrameRemaining\n\nThis counter is decremented at each bit time. When it reaches zero, it is reset by loading the FrameInterval value specified in at the next bit time boundary. When entering the USBOPERATIONAL state, HC re-loads the content with the FrameInterval of and uses the updated value from the next SOF."]
    #[inline(always)]
    pub fn frame_remaining(&self) -> FRAME_REMAINING_R {
        FRAME_REMAINING_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
    #[inline(always)]
    pub fn frame_remaining_toggle(&self) -> FRAME_REMAINING_TOGGLE_R {
        FRAME_REMAINING_TOGGLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameRemaining\n\nThis counter is decremented at each bit time. When it reaches zero, it is reset by loading the FrameInterval value specified in at the next bit time boundary. When entering the USBOPERATIONAL state, HC re-loads the content with the FrameInterval of and uses the updated value from the next SOF."]
    #[inline(always)]
    #[must_use]
    pub fn frame_remaining(&mut self) -> FRAME_REMAINING_W<0> {
        FRAME_REMAINING_W::new(self)
    }
    #[doc = "Bit 31 - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
    #[inline(always)]
    #[must_use]
    pub fn frame_remaining_toggle(&mut self) -> FRAME_REMAINING_TOGGLE_W<31> {
        FRAME_REMAINING_TOGGLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Frame Remaining Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_fm_remaining](index.html) module"]
pub struct HC_FM_REMAINING_SPEC;
impl crate::RegisterSpec for HC_FM_REMAINING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_fm_remaining::R](R) reader structure"]
impl crate::Readable for HC_FM_REMAINING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_fm_remaining::W](W) writer structure"]
impl crate::Writable for HC_FM_REMAINING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_fm_remaining to value 0"]
impl crate::Resettable for HC_FM_REMAINING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
