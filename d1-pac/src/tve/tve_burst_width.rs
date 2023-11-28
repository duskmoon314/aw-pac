#[doc = "Register `tve_burst_width` reader"]
pub type R = crate::R<TVE_BURST_WIDTH_SPEC>;
#[doc = "Register `tve_burst_width` writer"]
pub type W = crate::W<TVE_BURST_WIDTH_SPEC>;
#[doc = "Field `hsync_width` reader - Specify the width of the horizontal sync pulse in encoder clock cycles. Min value is 16. Max value is (FrontPorch + ActiveLine - BackPorch). Default value is 126. The sum of HSyncSize and BackPorch is restricted to be divisible by 4.\n\nFor 720p mode, the value is 40.\n\nFor 1080i/p mode, the value is 44."]
pub type HSYNC_WIDTH_R = crate::FieldReader;
#[doc = "Field `hsync_width` writer - Specify the width of the horizontal sync pulse in encoder clock cycles. Min value is 16. Max value is (FrontPorch + ActiveLine - BackPorch). Default value is 126. The sum of HSyncSize and BackPorch is restricted to be divisible by 4.\n\nFor 720p mode, the value is 40.\n\nFor 1080i/p mode, the value is 44."]
pub type HSYNC_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `burst_width` reader - Specify the width of the color frequency burst in encoder clock cycles. 7 bit unsigned integer. Allowed range is 0 to 127. In hd mode, it is ignored."]
pub type BURST_WIDTH_R = crate::FieldReader;
#[doc = "Field `burst_width` writer - Specify the width of the color frequency burst in encoder clock cycles. 7 bit unsigned integer. Allowed range is 0 to 127. In hd mode, it is ignored."]
pub type BURST_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `breezeway` reader - Must be even\n\nSpecify the width of the breezeway in encoder clock cycles. 5 bit unsigned integer. Allowed range is 0 to 31.\n\nFor 1080i mode, the value is 44.\n\nFor 1080p mode, the value is 44.\n\nFor 720p mode, the value is 40."]
pub type BREEZEWAY_R = crate::FieldReader;
#[doc = "Field `breezeway` writer - Must be even\n\nSpecify the width of the breezeway in encoder clock cycles. 5 bit unsigned integer. Allowed range is 0 to 31.\n\nFor 1080i mode, the value is 44.\n\nFor 1080p mode, the value is 44.\n\nFor 720p mode, the value is 40."]
pub type BREEZEWAY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `back_porch` reader - Breezeway like in HD mode VSync\n\nFor 720p mode, the value is 220\n\nFor 2080i/p mode, the value is 88 (default)"]
pub type BACK_PORCH_R = crate::FieldReader;
#[doc = "Field `back_porch` writer - Breezeway like in HD mode VSync\n\nFor 720p mode, the value is 220\n\nFor 2080i/p mode, the value is 88 (default)"]
pub type BACK_PORCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specify the width of the horizontal sync pulse in encoder clock cycles. Min value is 16. Max value is (FrontPorch + ActiveLine - BackPorch). Default value is 126. The sum of HSyncSize and BackPorch is restricted to be divisible by 4.\n\nFor 720p mode, the value is 40.\n\nFor 1080i/p mode, the value is 44."]
    #[inline(always)]
    pub fn hsync_width(&self) -> HSYNC_WIDTH_R {
        HSYNC_WIDTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Specify the width of the color frequency burst in encoder clock cycles. 7 bit unsigned integer. Allowed range is 0 to 127. In hd mode, it is ignored."]
    #[inline(always)]
    pub fn burst_width(&self) -> BURST_WIDTH_R {
        BURST_WIDTH_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Must be even\n\nSpecify the width of the breezeway in encoder clock cycles. 5 bit unsigned integer. Allowed range is 0 to 31.\n\nFor 1080i mode, the value is 44.\n\nFor 1080p mode, the value is 44.\n\nFor 720p mode, the value is 40."]
    #[inline(always)]
    pub fn breezeway(&self) -> BREEZEWAY_R {
        BREEZEWAY_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Breezeway like in HD mode VSync\n\nFor 720p mode, the value is 220\n\nFor 2080i/p mode, the value is 88 (default)"]
    #[inline(always)]
    pub fn back_porch(&self) -> BACK_PORCH_R {
        BACK_PORCH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specify the width of the horizontal sync pulse in encoder clock cycles. Min value is 16. Max value is (FrontPorch + ActiveLine - BackPorch). Default value is 126. The sum of HSyncSize and BackPorch is restricted to be divisible by 4.\n\nFor 720p mode, the value is 40.\n\nFor 1080i/p mode, the value is 44."]
    #[inline(always)]
    #[must_use]
    pub fn hsync_width(&mut self) -> HSYNC_WIDTH_W<TVE_BURST_WIDTH_SPEC> {
        HSYNC_WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Specify the width of the color frequency burst in encoder clock cycles. 7 bit unsigned integer. Allowed range is 0 to 127. In hd mode, it is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn burst_width(&mut self) -> BURST_WIDTH_W<TVE_BURST_WIDTH_SPEC> {
        BURST_WIDTH_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Must be even\n\nSpecify the width of the breezeway in encoder clock cycles. 5 bit unsigned integer. Allowed range is 0 to 31.\n\nFor 1080i mode, the value is 44.\n\nFor 1080p mode, the value is 44.\n\nFor 720p mode, the value is 40."]
    #[inline(always)]
    #[must_use]
    pub fn breezeway(&mut self) -> BREEZEWAY_W<TVE_BURST_WIDTH_SPEC> {
        BREEZEWAY_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Breezeway like in HD mode VSync\n\nFor 720p mode, the value is 220\n\nFor 2080i/p mode, the value is 88 (default)"]
    #[inline(always)]
    #[must_use]
    pub fn back_porch(&mut self) -> BACK_PORCH_W<TVE_BURST_WIDTH_SPEC> {
        BACK_PORCH_W::new(self, 24)
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
#[doc = "TV Encoder Burst Width Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_burst_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_burst_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_BURST_WIDTH_SPEC;
impl crate::RegisterSpec for TVE_BURST_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_burst_width::R`](R) reader structure"]
impl crate::Readable for TVE_BURST_WIDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_burst_width::W`](W) writer structure"]
impl crate::Writable for TVE_BURST_WIDTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_burst_width to value 0x0016_447e"]
impl crate::Resettable for TVE_BURST_WIDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0016_447e;
}
