#[doc = "Register `smhc_blksiz` reader"]
pub type R = crate::R<SMHC_BLKSIZ_SPEC>;
#[doc = "Register `smhc_blksiz` writer"]
pub type W = crate::W<SMHC_BLKSIZ_SPEC>;
#[doc = "Field `blk_sz` reader - Block SIze"]
pub type BLK_SZ_R = crate::FieldReader<u16>;
#[doc = "Field `blk_sz` writer - Block SIze"]
pub type BLK_SZ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Block SIze"]
    #[inline(always)]
    pub fn blk_sz(&self) -> BLK_SZ_R {
        BLK_SZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block SIze"]
    #[inline(always)]
    #[must_use]
    pub fn blk_sz(&mut self) -> BLK_SZ_W<SMHC_BLKSIZ_SPEC> {
        BLK_SZ_W::new(self, 0)
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
#[doc = "Block Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_blksiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_blksiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_BLKSIZ_SPEC;
impl crate::RegisterSpec for SMHC_BLKSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_blksiz::R`](R) reader structure"]
impl crate::Readable for SMHC_BLKSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_blksiz::W`](W) writer structure"]
impl crate::Writable for SMHC_BLKSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_blksiz to value 0"]
impl crate::Resettable for SMHC_BLKSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
