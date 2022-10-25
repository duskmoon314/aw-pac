#[doc = "Register `csic_mulf_mod` reader"]
pub struct R(crate::R<CSIC_MULF_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_MULF_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_MULF_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_MULF_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_mulf_mod` writer"]
pub struct W(crate::W<CSIC_MULF_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_MULF_MOD_SPEC>;
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
impl From<crate::W<CSIC_MULF_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_MULF_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mulf_en` reader - "]
pub type MULF_EN_R = crate::BitReader<bool>;
#[doc = "Field `mulf_en` writer - "]
pub type MULF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_MULF_MOD_SPEC, bool, O>;
#[doc = "Field `mulf_cs` reader - "]
pub type MULF_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mulf_cs` writer - "]
pub type MULF_CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSIC_MULF_MOD_SPEC, u8, u8, 8, O>;
#[doc = "Field `mulf_status` reader - "]
pub type MULF_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mulf_en(&self) -> MULF_EN_R {
        MULF_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mulf_cs(&self) -> MULF_CS_R {
        MULF_CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mulf_status(&self) -> MULF_STATUS_R {
        MULF_STATUS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_en(&mut self) -> MULF_EN_W<0> {
        MULF_EN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_cs(&mut self) -> MULF_CS_W<8> {
        MULF_CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Multi-Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_mulf_mod](index.html) module"]
pub struct CSIC_MULF_MOD_SPEC;
impl crate::RegisterSpec for CSIC_MULF_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_mulf_mod::R](R) reader structure"]
impl crate::Readable for CSIC_MULF_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_mulf_mod::W](W) writer structure"]
impl crate::Writable for CSIC_MULF_MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_mulf_mod to value 0"]
impl crate::Resettable for CSIC_MULF_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
