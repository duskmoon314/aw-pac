#[doc = "Register `frindex` reader"]
pub struct R(crate::R<FRINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `frindex` writer"]
pub struct W(crate::W<FRINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRINDEX_SPEC>;
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
impl From<crate::W<FRINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_index` reader - Frame Index\n\nThe value in this register increment at the end of each time frame (e.g. micro-frame). Bits\\[N:3\\] are used for the Frame List current index. It means that each location of the frame list is accessed 8 times (frames or Micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register."]
pub type FRAME_INDEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `frame_index` writer - Frame Index\n\nThe value in this register increment at the end of each time frame (e.g. micro-frame). Bits\\[N:3\\] are used for the Frame List current index. It means that each location of the frame list is accessed 8 times (frames or Micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register."]
pub type FRAME_INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRINDEX_SPEC, u16, u16, 14, O>;
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
    pub fn frame_index(&mut self) -> FRAME_INDEX_W<0> {
        FRAME_INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI Frame Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frindex](index.html) module"]
pub struct FRINDEX_SPEC;
impl crate::RegisterSpec for FRINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frindex::R](R) reader structure"]
impl crate::Readable for FRINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frindex::W](W) writer structure"]
impl crate::Writable for FRINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets frindex to value 0"]
impl crate::Resettable for FRINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
