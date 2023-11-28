#[doc = "Register `hc_fm_number` reader"]
pub type R = crate::R<HC_FM_NUMBER_SPEC>;
#[doc = "Register `hc_fm_number` writer"]
pub type W = crate::W<HC_FM_NUMBER_SPEC>;
#[doc = "Field `frame_number` reader - FrameNumber\n\nThis is incremented when is re-loaded. It will be rolled over to 0x0 after 0x0ffff. When entering the USBOPERATIONAL state, this will be incremented automatically. The content will be written to HCCA after HC has incremented the FrameNumber at each frame boundary and sent a SOF but before HC reads the first ED in that Frame. After writing to HCCA, HC will set the StartofFrame in HcInterruptStatus"]
pub type FRAME_NUMBER_R = crate::FieldReader<u16>;
#[doc = "Field `frame_number` writer - FrameNumber\n\nThis is incremented when is re-loaded. It will be rolled over to 0x0 after 0x0ffff. When entering the USBOPERATIONAL state, this will be incremented automatically. The content will be written to HCCA after HC has incremented the FrameNumber at each frame boundary and sent a SOF but before HC reads the first ED in that Frame. After writing to HCCA, HC will set the StartofFrame in HcInterruptStatus"]
pub type FRAME_NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn frame_number(&mut self) -> FRAME_NUMBER_W<HC_FM_NUMBER_SPEC> {
        FRAME_NUMBER_W::new(self, 0)
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
#[doc = "OHCI Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_fm_number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_fm_number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_FM_NUMBER_SPEC;
impl crate::RegisterSpec for HC_FM_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_fm_number::R`](R) reader structure"]
impl crate::Readable for HC_FM_NUMBER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_fm_number::W`](W) writer structure"]
impl crate::Writable for HC_FM_NUMBER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_fm_number to value 0"]
impl crate::Resettable for HC_FM_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
