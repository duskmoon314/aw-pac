#[doc = "Register `tve_video_active_line` reader"]
pub struct R(crate::R<TVE_VIDEO_ACTIVE_LINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_VIDEO_ACTIVE_LINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_VIDEO_ACTIVE_LINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_VIDEO_ACTIVE_LINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_video_active_line` writer"]
pub struct W(crate::W<TVE_VIDEO_ACTIVE_LINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_VIDEO_ACTIVE_LINE_SPEC>;
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
impl From<crate::W<TVE_VIDEO_ACTIVE_LINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_VIDEO_ACTIVE_LINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `active_line` reader - Specify the width of the video line in encoder clock cycles. 12- bit unsigned multiple of 4 integer. Allowed range is from 0 to 4092."]
pub type ACTIVE_LINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `active_line` writer - Specify the width of the video line in encoder clock cycles. 12- bit unsigned multiple of 4 integer. Allowed range is from 0 to 4092."]
pub type ACTIVE_LINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_VIDEO_ACTIVE_LINE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Specify the width of the video line in encoder clock cycles. 12- bit unsigned multiple of 4 integer. Allowed range is from 0 to 4092."]
    #[inline(always)]
    pub fn active_line(&self) -> ACTIVE_LINE_R {
        ACTIVE_LINE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Specify the width of the video line in encoder clock cycles. 12- bit unsigned multiple of 4 integer. Allowed range is from 0 to 4092."]
    #[inline(always)]
    #[must_use]
    pub fn active_line(&mut self) -> ACTIVE_LINE_W<0> {
        ACTIVE_LINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Video Active Line Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_video_active_line](index.html) module"]
pub struct TVE_VIDEO_ACTIVE_LINE_SPEC;
impl crate::RegisterSpec for TVE_VIDEO_ACTIVE_LINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_video_active_line::R](R) reader structure"]
impl crate::Readable for TVE_VIDEO_ACTIVE_LINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_video_active_line::W](W) writer structure"]
impl crate::Writable for TVE_VIDEO_ACTIVE_LINE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_video_active_line to value 0x05a0"]
impl crate::Resettable for TVE_VIDEO_ACTIVE_LINE_SPEC {
    const RESET_VALUE: Self::Ux = 0x05a0;
}
