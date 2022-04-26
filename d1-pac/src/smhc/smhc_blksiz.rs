#[doc = "Register `SMHC_BLKSIZ` reader"]
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
#[doc = "Register `SMHC_BLKSIZ` writer"]
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
#[doc = "Field `BLK_SZ` reader - Block SIze"]
pub struct BLK_SZ_R(crate::FieldReader<u16>);
impl BLK_SZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BLK_SZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK_SZ_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK_SZ` writer - Block SIze"]
pub struct BLK_SZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_SZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn blk_sz(&mut self) -> BLK_SZ_W {
        BLK_SZ_W { w: self }
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
}
#[doc = "`reset()` method sets SMHC_BLKSIZ to value 0"]
impl crate::Resettable for SMHC_BLKSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
