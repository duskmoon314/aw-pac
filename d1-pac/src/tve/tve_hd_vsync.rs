#[doc = "Register `tve_hd_vsync` reader"]
pub struct R(crate::R<TVE_HD_VSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_HD_VSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_HD_VSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_HD_VSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_hd_vsync` writer"]
pub struct W(crate::W<TVE_HD_VSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_HD_VSYNC_SPEC>;
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
impl From<crate::W<TVE_HD_VSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_HD_VSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `front_porch_like_in_hd_mode_vsync` reader - "]
pub type FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `front_porch_like_in_hd_mode_vsync` writer - "]
pub type FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_HD_VSYNC_SPEC, u16, u16, 12, O>;
#[doc = "Field `broad_plus_cycle_number_in_hd_mode_vsync` reader - "]
pub type BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `broad_plus_cycle_number_in_hd_mode_vsync` writer - "]
pub type BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_HD_VSYNC_SPEC, u16, u16, 12, O>;
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
    pub fn front_porch_like_in_hd_mode_vsync(&mut self) -> FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_W<0> {
        FRONT_PORCH_LIKE_IN_HD_MODE_VSYNC_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn broad_plus_cycle_number_in_hd_mode_vsync(
        &mut self,
    ) -> BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_W<16> {
        BROAD_PLUS_CYCLE_NUMBER_IN_HD_MODE_VSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder HD Mode VSYNC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_hd_vsync](index.html) module"]
pub struct TVE_HD_VSYNC_SPEC;
impl crate::RegisterSpec for TVE_HD_VSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_hd_vsync::R](R) reader structure"]
impl crate::Readable for TVE_HD_VSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_hd_vsync::W](W) writer structure"]
impl crate::Writable for TVE_HD_VSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_hd_vsync to value 0x16"]
impl crate::Resettable for TVE_HD_VSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x16;
}
