#[doc = "Register `tve_sync_vbi_level` reader"]
pub type R = crate::R<TVE_SYNC_VBI_LEVEL_SPEC>;
#[doc = "Register `tve_sync_vbi_level` writer"]
pub type W = crate::W<TVE_SYNC_VBI_LEVEL_SPEC>;
#[doc = "Field `vblank_level` reader - Specify the blank level setting for non active lines. 10-bit unsigned integer. Allow range is from 0 to 1023."]
pub type VBLANK_LEVEL_R = crate::FieldReader<u16>;
#[doc = "Field `vblank_level` writer - Specify the blank level setting for non active lines. 10-bit unsigned integer. Allow range is from 0 to 1023."]
pub type VBLANK_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `sync_level` reader - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
pub type SYNC_LEVEL_R = crate::FieldReader<u16>;
#[doc = "Field `sync_level` writer - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
pub type SYNC_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
    pub fn vblank_level(&mut self) -> VBLANK_LEVEL_W<TVE_SYNC_VBI_LEVEL_SPEC> {
        VBLANK_LEVEL_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Specify the sync pulse level setting. 8-bit unsigned integer. Allowed range is from 0 to ABlankLevel-1 or VBlankLevel-1 (whichever is smaller)."]
    #[inline(always)]
    #[must_use]
    pub fn sync_level(&mut self) -> SYNC_LEVEL_W<TVE_SYNC_VBI_LEVEL_SPEC> {
        SYNC_LEVEL_W::new(self, 16)
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
#[doc = "TV Encoder Sync and VBI Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_sync_vbi_level::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_sync_vbi_level::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_SYNC_VBI_LEVEL_SPEC;
impl crate::RegisterSpec for TVE_SYNC_VBI_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_sync_vbi_level::R`](R) reader structure"]
impl crate::Readable for TVE_SYNC_VBI_LEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_sync_vbi_level::W`](W) writer structure"]
impl crate::Writable for TVE_SYNC_VBI_LEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_sync_vbi_level to value 0x0010_00f0"]
impl crate::Resettable for TVE_SYNC_VBI_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_00f0;
}
