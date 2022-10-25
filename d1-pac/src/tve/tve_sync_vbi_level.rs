#[doc = "Register `tve_sync_vbi_level` reader"]
pub struct R(crate::R<TVE_SYNC_VBI_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_SYNC_VBI_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_SYNC_VBI_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_SYNC_VBI_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_sync_vbi_level` writer"]
pub struct W(crate::W<TVE_SYNC_VBI_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_SYNC_VBI_LEVEL_SPEC>;
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
impl From<crate::W<TVE_SYNC_VBI_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_SYNC_VBI_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vblank_level` reader - Specify the blank level setting for non active lines. 10-bit unsigned integer. Allow range is from 0 to 1023."]
pub type VBLANK_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `vblank_level` writer - Specify the blank level setting for non active lines. 10-bit unsigned integer. Allow range is from 0 to 1023."]
pub type VBLANK_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_SYNC_VBI_LEVEL_SPEC, u16, u16, 10, O>;
#[doc = "Field `sync_level` reader - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
pub type SYNC_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sync_level` writer - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
pub type SYNC_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_SYNC_VBI_LEVEL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Specify the blank level setting for non active lines. 10-bit unsigned integer. Allow range is from 0 to 1023."]
    #[inline(always)]
    pub fn vblank_level(&self) -> VBLANK_LEVEL_R {
        VBLANK_LEVEL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
    #[inline(always)]
    pub fn sync_level(&self) -> SYNC_LEVEL_R {
        SYNC_LEVEL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specify the blank level setting for non active lines. 10-bit unsigned integer. Allow range is from 0 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn vblank_level(&mut self) -> VBLANK_LEVEL_W<0> {
        VBLANK_LEVEL_W::new(self)
    }
    #[doc = "Bits 16:25 - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
    #[inline(always)]
    #[must_use]
    pub fn sync_level(&mut self) -> SYNC_LEVEL_W<16> {
        SYNC_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Sync and VBI Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_sync_vbi_level](index.html) module"]
pub struct TVE_SYNC_VBI_LEVEL_SPEC;
impl crate::RegisterSpec for TVE_SYNC_VBI_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_sync_vbi_level::R](R) reader structure"]
impl crate::Readable for TVE_SYNC_VBI_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_sync_vbi_level::W](W) writer structure"]
impl crate::Writable for TVE_SYNC_VBI_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_sync_vbi_level to value 0x0010_00f0"]
impl crate::Resettable for TVE_SYNC_VBI_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_00f0;
}
