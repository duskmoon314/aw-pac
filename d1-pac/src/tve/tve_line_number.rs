#[doc = "Register `tve_line_number` reader"]
pub struct R(crate::R<TVE_LINE_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LINE_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LINE_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LINE_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_line_number` writer"]
pub struct W(crate::W<TVE_LINE_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LINE_NUMBER_SPEC>;
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
impl From<crate::W<TVE_LINE_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LINE_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `num_lines` reader - Specify the total number of lines in a video frame. 11 bits unsigned integer. Allowed range is 0 to 2048. For interlaced video: When NTSC, and FirstVideoLine is greater than 20, then NumLines is restricted to be greater than 2*(FirstVideoLine+18).\n\nWhen NTSC, and FirstVideoLine is not greater than 20, then NumLines is restricted to be greater than 77. When PAL, and FirstVideoLine is greater than 22, then NumLines is restricted to be greater than 2*(FirstVideoLine+18). When PAL, and FirstVideoLine is not greater than 22, then NumLines is restricted to be greater than 81.\n\nIf NumLines is even, then it is restricted to be divisible by 4. If NumLines is odd, then it is restricted to be divisible by 4 with a remainder of 1."]
pub type NUM_LINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `num_lines` writer - Specify the total number of lines in a video frame. 11 bits unsigned integer. Allowed range is 0 to 2048. For interlaced video: When NTSC, and FirstVideoLine is greater than 20, then NumLines is restricted to be greater than 2*(FirstVideoLine+18).\n\nWhen NTSC, and FirstVideoLine is not greater than 20, then NumLines is restricted to be greater than 77. When PAL, and FirstVideoLine is greater than 22, then NumLines is restricted to be greater than 2*(FirstVideoLine+18). When PAL, and FirstVideoLine is not greater than 22, then NumLines is restricted to be greater than 81.\n\nIf NumLines is even, then it is restricted to be divisible by 4. If NumLines is odd, then it is restricted to be divisible by 4 with a remainder of 1."]
pub type NUM_LINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LINE_NUMBER_SPEC, u16, u16, 11, O>;
#[doc = "Field `first_video_line` reader - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
pub type FIRST_VIDEO_LINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `first_video_line` writer - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
pub type FIRST_VIDEO_LINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LINE_NUMBER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:10 - Specify the total number of lines in a video frame. 11 bits unsigned integer. Allowed range is 0 to 2048. For interlaced video: When NTSC, and FirstVideoLine is greater than 20, then NumLines is restricted to be greater than 2*(FirstVideoLine+18).\n\nWhen NTSC, and FirstVideoLine is not greater than 20, then NumLines is restricted to be greater than 77. When PAL, and FirstVideoLine is greater than 22, then NumLines is restricted to be greater than 2*(FirstVideoLine+18). When PAL, and FirstVideoLine is not greater than 22, then NumLines is restricted to be greater than 81.\n\nIf NumLines is even, then it is restricted to be divisible by 4. If NumLines is odd, then it is restricted to be divisible by 4 with a remainder of 1."]
    #[inline(always)]
    pub fn num_lines(&self) -> NUM_LINES_R {
        NUM_LINES_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
    #[inline(always)]
    pub fn first_video_line(&self) -> FIRST_VIDEO_LINE_R {
        FIRST_VIDEO_LINE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Specify the total number of lines in a video frame. 11 bits unsigned integer. Allowed range is 0 to 2048. For interlaced video: When NTSC, and FirstVideoLine is greater than 20, then NumLines is restricted to be greater than 2*(FirstVideoLine+18).\n\nWhen NTSC, and FirstVideoLine is not greater than 20, then NumLines is restricted to be greater than 77. When PAL, and FirstVideoLine is greater than 22, then NumLines is restricted to be greater than 2*(FirstVideoLine+18). When PAL, and FirstVideoLine is not greater than 22, then NumLines is restricted to be greater than 81.\n\nIf NumLines is even, then it is restricted to be divisible by 4. If NumLines is odd, then it is restricted to be divisible by 4 with a remainder of 1."]
    #[inline(always)]
    #[must_use]
    pub fn num_lines(&mut self) -> NUM_LINES_W<0> {
        NUM_LINES_W::new(self)
    }
    #[doc = "Bits 16:23 - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
    #[inline(always)]
    #[must_use]
    pub fn first_video_line(&mut self) -> FIRST_VIDEO_LINE_W<16> {
        FIRST_VIDEO_LINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Line Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_line_number](index.html) module"]
pub struct TVE_LINE_NUMBER_SPEC;
impl crate::RegisterSpec for TVE_LINE_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_line_number::R](R) reader structure"]
impl crate::Readable for TVE_LINE_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_line_number::W](W) writer structure"]
impl crate::Writable for TVE_LINE_NUMBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_line_number to value 0x0016_020d"]
impl crate::Resettable for TVE_LINE_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0016_020d;
}
