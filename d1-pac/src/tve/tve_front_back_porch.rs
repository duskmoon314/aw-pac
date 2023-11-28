#[doc = "Register `tve_front_back_porch` reader"]
pub type R = crate::R<TVE_FRONT_BACK_PORCH_SPEC>;
#[doc = "Register `tve_front_back_porch` writer"]
pub type W = crate::W<TVE_FRONT_BACK_PORCH_SPEC>;
#[doc = "Field `front_porch` reader - Must be even\n\nSpecify the width of the front porch in encoder clock cycles. 6 bits unsigned even integer. Allowed range is form 10 to 62.\n\nThe default value is 32.\n\nFor 1080i mode, the value is 44."]
pub type FRONT_PORCH_R = crate::FieldReader<u16>;
#[doc = "Field `front_porch` writer - Must be even\n\nSpecify the width of the front porch in encoder clock cycles. 6 bits unsigned even integer. Allowed range is form 10 to 62.\n\nThe default value is 32.\n\nFor 1080i mode, the value is 44."]
pub type FRONT_PORCH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `back_porch` reader - Specify the width of the back porch in encoder clock cycles. Min value is (burst_width+breeze_way+17). 8 bits unsigned integer.\n\nThe default value is 118.\n\nFor 720p mode, the value is 260.\n\nFor 1080i/p mode, the value is 192."]
pub type BACK_PORCH_R = crate::FieldReader<u16>;
#[doc = "Field `back_porch` writer - Specify the width of the back porch in encoder clock cycles. Min value is (burst_width+breeze_way+17). 8 bits unsigned integer.\n\nThe default value is 118.\n\nFor 720p mode, the value is 260.\n\nFor 1080i/p mode, the value is 192."]
pub type BACK_PORCH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:11 - Must be even\n\nSpecify the width of the front porch in encoder clock cycles. 6 bits unsigned even integer. Allowed range is form 10 to 62.\n\nThe default value is 32.\n\nFor 1080i mode, the value is 44."]
    #[inline(always)]
    pub fn front_porch(&self) -> FRONT_PORCH_R {
        FRONT_PORCH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:24 - Specify the width of the back porch in encoder clock cycles. Min value is (burst_width+breeze_way+17). 8 bits unsigned integer.\n\nThe default value is 118.\n\nFor 720p mode, the value is 260.\n\nFor 1080i/p mode, the value is 192."]
    #[inline(always)]
    pub fn back_porch(&self) -> BACK_PORCH_R {
        BACK_PORCH_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Must be even\n\nSpecify the width of the front porch in encoder clock cycles. 6 bits unsigned even integer. Allowed range is form 10 to 62.\n\nThe default value is 32.\n\nFor 1080i mode, the value is 44."]
    #[inline(always)]
    #[must_use]
    pub fn front_porch(&mut self) -> FRONT_PORCH_W<TVE_FRONT_BACK_PORCH_SPEC> {
        FRONT_PORCH_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - Specify the width of the back porch in encoder clock cycles. Min value is (burst_width+breeze_way+17). 8 bits unsigned integer.\n\nThe default value is 118.\n\nFor 720p mode, the value is 260.\n\nFor 1080i/p mode, the value is 192."]
    #[inline(always)]
    #[must_use]
    pub fn back_porch(&mut self) -> BACK_PORCH_W<TVE_FRONT_BACK_PORCH_SPEC> {
        BACK_PORCH_W::new(self, 16)
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
#[doc = "TV Encoder Front/Back Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_front_back_porch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_front_back_porch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_FRONT_BACK_PORCH_SPEC;
impl crate::RegisterSpec for TVE_FRONT_BACK_PORCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_front_back_porch::R`](R) reader structure"]
impl crate::Readable for TVE_FRONT_BACK_PORCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_front_back_porch::W`](W) writer structure"]
impl crate::Writable for TVE_FRONT_BACK_PORCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_front_back_porch to value 0x0076_0020"]
impl crate::Resettable for TVE_FRONT_BACK_PORCH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0076_0020;
}
