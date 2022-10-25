#[doc = "Register `pb_dat` reader"]
pub struct R(crate::R<PB_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pb_dat` writer"]
pub struct W(crate::W<PB_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_DAT_SPEC>;
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
impl From<crate::W<PB_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pb_dat` reader - "]
pub type PB_DAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pb_dat` writer - "]
pub type PB_DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PB_DAT_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn pb_dat(&self) -> PB_DAT_R {
        PB_DAT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn pb_dat(&mut self) -> PB_DAT_W<0> {
        PB_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_dat](index.html) module"]
pub struct PB_DAT_SPEC;
impl crate::RegisterSpec for PB_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_dat::R](R) reader structure"]
impl crate::Readable for PB_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_dat::W](W) writer structure"]
impl crate::Writable for PB_DAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_dat to value 0"]
impl crate::Resettable for PB_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
