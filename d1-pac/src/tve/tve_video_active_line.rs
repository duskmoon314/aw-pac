#[doc = "Register `tve_video_active_line` reader"]
pub type R = crate::R<TVE_VIDEO_ACTIVE_LINE_SPEC>;
#[doc = "Register `tve_video_active_line` writer"]
pub type W = crate::W<TVE_VIDEO_ACTIVE_LINE_SPEC>;
#[doc = "Field `active_line` reader - Specify the width of the video line in encoder clock cycles. 12- bit unsigned multiple of 4 integer. Allowed range is from 0 to 4092."]
pub type ACTIVE_LINE_R = crate::FieldReader<u16>;
#[doc = "Field `active_line` writer - Specify the width of the video line in encoder clock cycles. 12- bit unsigned multiple of 4 integer. Allowed range is from 0 to 4092."]
pub type ACTIVE_LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
    pub fn active_line(&mut self) -> ACTIVE_LINE_W<TVE_VIDEO_ACTIVE_LINE_SPEC> {
        ACTIVE_LINE_W::new(self, 0)
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
#[doc = "TV Encoder Video Active Line Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_video_active_line::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_video_active_line::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_VIDEO_ACTIVE_LINE_SPEC;
impl crate::RegisterSpec for TVE_VIDEO_ACTIVE_LINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_video_active_line::R`](R) reader structure"]
impl crate::Readable for TVE_VIDEO_ACTIVE_LINE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_video_active_line::W`](W) writer structure"]
impl crate::Writable for TVE_VIDEO_ACTIVE_LINE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_video_active_line to value 0x05a0"]
impl crate::Resettable for TVE_VIDEO_ACTIVE_LINE_SPEC {
    const RESET_VALUE: Self::Ux = 0x05a0;
}
