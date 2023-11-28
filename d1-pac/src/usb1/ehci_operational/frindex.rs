#[doc = "Register `frindex` reader"]
pub type R = crate::R<FRINDEX_SPEC>;
#[doc = "Register `frindex` writer"]
pub type W = crate::W<FRINDEX_SPEC>;
#[doc = "Field `frame_index` reader - Frame Index\n\nThe value in this register increment at the end of each time frame (e.g. micro-frame). Bits\\[N:3\\] are used for the Frame List current index. It means that each location of the frame list is accessed 8 times (frames or Micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register."]
pub type FRAME_INDEX_R = crate::FieldReader<u16>;
#[doc = "Field `frame_index` writer - Frame Index\n\nThe value in this register increment at the end of each time frame (e.g. micro-frame). Bits\\[N:3\\] are used for the Frame List current index. It means that each location of the frame list is accessed 8 times (frames or Micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register."]
pub type FRAME_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Index\n\nThe value in this register increment at the end of each time frame (e.g. micro-frame). Bits\\[N:3\\] are used for the Frame List current index. It means that each location of the frame list is accessed 8 times (frames or Micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register."]
    #[inline(always)]
    pub fn frame_index(&self) -> FRAME_INDEX_R {
        FRAME_INDEX_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Index\n\nThe value in this register increment at the end of each time frame (e.g. micro-frame). Bits\\[N:3\\] are used for the Frame List current index. It means that each location of the frame list is accessed 8 times (frames or Micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_index(&mut self) -> FRAME_INDEX_W<FRINDEX_SPEC> {
        FRAME_INDEX_W::new(self, 0)
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
#[doc = "EHCI Frame Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frindex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frindex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRINDEX_SPEC;
impl crate::RegisterSpec for FRINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frindex::R`](R) reader structure"]
impl crate::Readable for FRINDEX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frindex::W`](W) writer structure"]
impl crate::Writable for FRINDEX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets frindex to value 0"]
impl crate::Resettable for FRINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
