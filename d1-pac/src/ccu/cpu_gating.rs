#[doc = "Register `cpu_gating` reader"]
pub struct R(crate::R<CPU_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_gating` writer"]
pub struct W(crate::W<CPU_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_GATING_SPEC>;
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
impl From<crate::W<CPU_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu_gating_field` reader - CPU Gating Field"]
pub type CPU_GATING_FIELD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cpu_gating_field` writer - CPU Gating Field"]
pub type CPU_GATING_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_GATING_SPEC, u16, u16, 16, O>;
#[doc = "Field `cpu_gating` reader - Gating Special Clock"]
pub type CPU_GATING_R = crate::BitReader<CPU_GATING_A>;
#[doc = "Gating Special Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPU_GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CPU_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CPU_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU_GATING_A {
        match self.bits {
            false => CPU_GATING_A::OFF,
            true => CPU_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CPU_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CPU_GATING_A::ON
    }
}
#[doc = "Field `cpu_gating` writer - Gating Special Clock"]
pub type CPU_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_GATING_SPEC, CPU_GATING_A, O>;
impl<'a, const O: u8> CPU_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CPU_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CPU_GATING_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:15 - CPU Gating Field"]
    #[inline(always)]
    pub fn cpu_gating_field(&self) -> CPU_GATING_FIELD_R {
        CPU_GATING_FIELD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn cpu_gating(&self) -> CPU_GATING_R {
        CPU_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU Gating Field"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_gating_field(&mut self) -> CPU_GATING_FIELD_W<0> {
        CPU_GATING_FIELD_W::new(self)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_gating(&mut self) -> CPU_GATING_W<31> {
        CPU_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU_GATING Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_gating](index.html) module"]
pub struct CPU_GATING_SPEC;
impl crate::RegisterSpec for CPU_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_gating::R](R) reader structure"]
impl crate::Readable for CPU_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_gating::W](W) writer structure"]
impl crate::Writable for CPU_GATING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_gating to value 0"]
impl crate::Resettable for CPU_GATING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
