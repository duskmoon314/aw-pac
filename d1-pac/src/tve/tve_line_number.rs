#[doc = "Register `tve_line_number` reader"]
pub type R = crate::R<TVE_LINE_NUMBER_SPEC>;
#[doc = "Register `tve_line_number` writer"]
pub type W = crate::W<TVE_LINE_NUMBER_SPEC>;
#[doc = "Field `num_lines` reader - Specify the total number of lines in a video frame. 11 bits unsigned integer. Allowed range is 0 to 2048. For interlaced video: When NTSC, and FirstVideoLine is greater than 20, then NumLines is restricted to be greater than 2*(FirstVideoLine+18).\n\nWhen NTSC, and FirstVideoLine is not greater than 20, then NumLines is restricted to be greater than 77. When PAL, and FirstVideoLine is greater than 22, then NumLines is restricted to be greater than 2*(FirstVideoLine+18). When PAL, and FirstVideoLine is not greater than 22, then NumLines is restricted to be greater than 81.\n\nIf NumLines is even, then it is restricted to be divisible by 4. If NumLines is odd, then it is restricted to be divisible by 4 with a remainder of 1."]
pub type NUM_LINES_R = crate::FieldReader<u16>;
#[doc = "Field `num_lines` writer - Specify the total number of lines in a video frame. 11 bits unsigned integer. Allowed range is 0 to 2048. For interlaced video: When NTSC, and FirstVideoLine is greater than 20, then NumLines is restricted to be greater than 2*(FirstVideoLine+18).\n\nWhen NTSC, and FirstVideoLine is not greater than 20, then NumLines is restricted to be greater than 77. When PAL, and FirstVideoLine is greater than 22, then NumLines is restricted to be greater than 2*(FirstVideoLine+18). When PAL, and FirstVideoLine is not greater than 22, then NumLines is restricted to be greater than 81.\n\nIf NumLines is even, then it is restricted to be divisible by 4. If NumLines is odd, then it is restricted to be divisible by 4 with a remainder of 1."]
pub type NUM_LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `first_video_line` reader - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
pub type FIRST_VIDEO_LINE_R = crate::FieldReader;
#[doc = "Field `first_video_line` writer - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
pub type FIRST_VIDEO_LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn num_lines(&mut self) -> NUM_LINES_W<TVE_LINE_NUMBER_SPEC> {
        NUM_LINES_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Specify the index of the first line in a field/frame to have active video. 8 bits unsigned integer.\n\nFor interlaced video: When VSync5=B'0', FirstVideoLine is restricted to be greater than 7. When VSync5=B'1', FirstVideoLine is restricted to be greater than 9."]
    #[inline(always)]
    #[must_use]
    pub fn first_video_line(&mut self) -> FIRST_VIDEO_LINE_W<TVE_LINE_NUMBER_SPEC> {
        FIRST_VIDEO_LINE_W::new(self, 16)
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
#[doc = "TV Encoder Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_line_number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_line_number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_LINE_NUMBER_SPEC;
impl crate::RegisterSpec for TVE_LINE_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_line_number::R`](R) reader structure"]
impl crate::Readable for TVE_LINE_NUMBER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_line_number::W`](W) writer structure"]
impl crate::Writable for TVE_LINE_NUMBER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_line_number to value 0x0016_020d"]
impl crate::Resettable for TVE_LINE_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0016_020d;
}
