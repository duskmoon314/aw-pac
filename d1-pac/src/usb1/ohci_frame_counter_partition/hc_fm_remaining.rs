#[doc = "Register `hc_fm_remaining` reader"]
pub type R = crate::R<HC_FM_REMAINING_SPEC>;
#[doc = "Register `hc_fm_remaining` writer"]
pub type W = crate::W<HC_FM_REMAINING_SPEC>;
#[doc = "Field `frame_remaining` reader - FrameRemaining\n\nThis counter is decremented at each bit time. When it reaches zero, it is reset by loading the FrameInterval value specified in at the next bit time boundary. When entering the USBOPERATIONAL state, HC re-loads the content with the FrameInterval of and uses the updated value from the next SOF."]
pub type FRAME_REMAINING_R = crate::FieldReader<u16>;
#[doc = "Field `frame_remaining` writer - FrameRemaining\n\nThis counter is decremented at each bit time. When it reaches zero, it is reset by loading the FrameInterval value specified in at the next bit time boundary. When entering the USBOPERATIONAL state, HC re-loads the content with the FrameInterval of and uses the updated value from the next SOF."]
pub type FRAME_REMAINING_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `frame_remaining_toggle` reader - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
pub type FRAME_REMAINING_TOGGLE_R = crate::BitReader;
#[doc = "Field `frame_remaining_toggle` writer - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
pub type FRAME_REMAINING_TOGGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn frame_remaining(&mut self) -> FRAME_REMAINING_W<HC_FM_REMAINING_SPEC> {
        FRAME_REMAINING_W::new(self, 0)
    }
    #[doc = "Bit 31 - FrameRemaining Toggle\n\nThis bit is loaded from the FrameIntervalToggle field of whenever FrameRemaining reaches 0. This bit is used by HCD for the synchronization between FrameInterval and FrameRemaining."]
    #[inline(always)]
    #[must_use]
    pub fn frame_remaining_toggle(&mut self) -> FRAME_REMAINING_TOGGLE_W<HC_FM_REMAINING_SPEC> {
        FRAME_REMAINING_TOGGLE_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OHCI Frame Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_fm_remaining::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_fm_remaining::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_FM_REMAINING_SPEC;
impl crate::RegisterSpec for HC_FM_REMAINING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_fm_remaining::R`](R) reader structure"]
impl crate::Readable for HC_FM_REMAINING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_fm_remaining::W`](W) writer structure"]
impl crate::Writable for HC_FM_REMAINING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_fm_remaining to value 0"]
impl crate::Resettable for HC_FM_REMAINING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
