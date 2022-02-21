#[doc = "Register `pg_dat` reader"]
pub struct R(crate::R<PG_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_dat` writer"]
pub struct W(crate::W<PG_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_DAT_SPEC>;
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
impl From<crate::W<PG_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG_DAT` reader - PG Data"]
pub struct PG_DAT_R(crate::FieldReader<u32, u32>);
impl PG_DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PG_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG_DAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG_DAT` writer - PG Data"]
pub struct PG_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - PG Data"]
    #[inline(always)]
    pub fn pg_dat(&self) -> PG_DAT_R {
        PG_DAT_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - PG Data"]
    #[inline(always)]
    pub fn pg_dat(&mut self) -> PG_DAT_W {
        PG_DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_dat](index.html) module"]
pub struct PG_DAT_SPEC;
impl crate::RegisterSpec for PG_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_dat::R](R) reader structure"]
impl crate::Readable for PG_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_dat::W](W) writer structure"]
impl crate::Writable for PG_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_dat to value 0"]
impl crate::Resettable for PG_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
