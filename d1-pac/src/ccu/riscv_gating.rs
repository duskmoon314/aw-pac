#[doc = "Register `riscv_gating` reader"]
pub struct R(crate::R<RISCV_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `riscv_gating` writer"]
pub struct W(crate::W<RISCV_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_GATING_SPEC>;
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
impl From<crate::W<RISCV_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gating_field` reader - "]
pub type GATING_FIELD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gating_field` writer - "]
pub type GATING_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RISCV_GATING_SPEC, u16, u16, 16, O>;
#[doc = "Field `gating` reader - Gating Clock"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::MASK,
            true => GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == GATING_A::PASS
    }
}
#[doc = "Field `gating` writer - Gating Clock"]
pub type GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, RISCV_GATING_SPEC, GATING_A, O>;
impl<'a, const O: u8> GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(GATING_A::PASS)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gating_field(&self) -> GATING_FIELD_R {
        GATING_FIELD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gating_field(&mut self) -> GATING_FIELD_W<0> {
        GATING_FIELD_W::new(self)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<31> {
        GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISC-V GATING Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_gating](index.html) module"]
pub struct RISCV_GATING_SPEC;
impl crate::RegisterSpec for RISCV_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_gating::R](R) reader structure"]
impl crate::Readable for RISCV_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_gating::W](W) writer structure"]
impl crate::Writable for RISCV_GATING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_gating to value 0"]
impl crate::Resettable for RISCV_GATING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
