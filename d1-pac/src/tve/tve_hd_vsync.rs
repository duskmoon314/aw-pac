#[doc = "Register `tve_hd_vsync` reader"]
pub type R = crate::R<TVE_HD_VSYNC_SPEC>;
#[doc = "Register `tve_hd_vsync` writer"]
pub type W = crate::W<TVE_HD_VSYNC_SPEC>;
#[doc = "Field `front_porch_like_in_hd_mode_vsync` reader - "]
pub type FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_R = crate::FieldReader<u16>;
#[doc = "Field `front_porch_like_in_hd_mode_vsync` writer - "]
pub type FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `broad_plus_cycle_number_in_hd_mode_vsync` reader - "]
pub type BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_R = crate::FieldReader<u16>;
#[doc = "Field `broad_plus_cycle_number_in_hd_mode_vsync` writer - "]
pub type BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn front_porch_like_in_hd_mode_vsync(&self) -> FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_R {
        FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn broad_plus_cycle_number_in_hd_mode_vsync(
        &self,
    ) -> BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_R {
        BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn front_porch_like_in_hd_mode_vsync(
        &mut self,
    ) -> FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_W<TVE_HD_VSYNC_SPEC> {
        FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_W::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn broad_plus_cycle_number_in_hd_mode_vsync(
        &mut self,
    ) -> BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_W<TVE_HD_VSYNC_SPEC> {
        BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_W::new(self, 16)
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
#[doc = "TV Encoder HD Mode VSYNC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_hd_vsync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_hd_vsync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_HD_VSYNC_SPEC;
impl crate::RegisterSpec for TVE_HD_VSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_hd_vsync::R`](R) reader structure"]
impl crate::Readable for TVE_HD_VSYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_hd_vsync::W`](W) writer structure"]
impl crate::Writable for TVE_HD_VSYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_hd_vsync to value 0x16"]
impl crate::Resettable for TVE_HD_VSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x16;
}
