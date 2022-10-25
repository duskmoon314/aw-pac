#[doc = "Register `smhc_blksiz` reader"]
pub struct R(crate::R<SMHC_BLKSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_BLKSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_BLKSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_BLKSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_blksiz` writer"]
pub struct W(crate::W<SMHC_BLKSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_BLKSIZ_SPEC>;
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
impl From<crate::W<SMHC_BLKSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_BLKSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `blk_sz` reader - Block SIze"]
pub type BLK_SZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `blk_sz` writer - Block SIze"]
pub type BLK_SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_BLKSIZ_SPEC, u16, u16, 16, O>;
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
    pub fn blk_sz(&mut self) -> BLK_SZ_W<0> {
        BLK_SZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_blksiz](index.html) module"]
pub struct SMHC_BLKSIZ_SPEC;
impl crate::RegisterSpec for SMHC_BLKSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_blksiz::R](R) reader structure"]
impl crate::Readable for SMHC_BLKSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_blksiz::W](W) writer structure"]
impl crate::Writable for SMHC_BLKSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_blksiz to value 0"]
impl crate::Resettable for SMHC_BLKSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
