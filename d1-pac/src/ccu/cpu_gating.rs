#[doc = "Register `CPU_GATING` reader"]
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
#[doc = "Register `CPU_GATING` writer"]
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
#[doc = "Gating Special Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CPU_GATING` reader - Gating Special Clock"]
pub struct CPU_GATING_R(crate::FieldReader<bool>);
impl CPU_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CPU_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CPU_GATING_A::ON
    }
}
impl core::ops::Deref for CPU_GATING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_GATING` writer - Gating Special Clock"]
pub struct CPU_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `CPU_GATING_FIELD` reader - CPU Gating Field"]
pub struct CPU_GATING_FIELD_R(crate::FieldReader<u16>);
impl CPU_GATING_FIELD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CPU_GATING_FIELD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_GATING_FIELD_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_GATING_FIELD` writer - CPU Gating Field"]
pub struct CPU_GATING_FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_GATING_FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn cpu_gating(&self) -> CPU_GATING_R {
        CPU_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 0:15 - CPU Gating Field"]
    #[inline(always)]
    pub fn cpu_gating_field(&self) -> CPU_GATING_FIELD_R {
        CPU_GATING_FIELD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn cpu_gating(&mut self) -> CPU_GATING_W {
        CPU_GATING_W { w: self }
    }
    #[doc = "Bits 0:15 - CPU Gating Field"]
    #[inline(always)]
    pub fn cpu_gating_field(&mut self) -> CPU_GATING_FIELD_W {
        CPU_GATING_FIELD_W { w: self }
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
}
#[doc = "`reset()` method sets CPU_GATING to value 0"]
impl crate::Resettable for CPU_GATING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
