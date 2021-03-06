#[doc = "Register `TWI_ADDR` reader"]
pub struct R(crate::R<TWI_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_ADDR` writer"]
pub struct W(crate::W<TWI_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_ADDR_SPEC>;
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
impl From<crate::W<TWI_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLA` reader - Slave Address"]
pub struct SLA_R(crate::FieldReader<u8, u8>);
impl SLA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLA` writer - Slave Address"]
pub struct SLA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GCE_A> for bool {
    #[inline(always)]
    fn from(variant: GCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCE` reader - "]
pub struct GCE_R(crate::FieldReader<bool, GCE_A>);
impl GCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCE_A {
        match self.bits {
            false => GCE_A::DISABLE,
            true => GCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GCE_A::ENABLE
    }
}
impl core::ops::Deref for GCE_R {
    type Target = crate::FieldReader<bool, GCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCE` writer - "]
pub struct GCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GCE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Slave Address"]
    #[inline(always)]
    pub fn sla(&self) -> SLA_R {
        SLA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gce(&self) -> GCE_R {
        GCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Slave Address"]
    #[inline(always)]
    pub fn sla(&mut self) -> SLA_W {
        SLA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gce(&mut self) -> GCE_W {
        GCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_addr](index.html) module"]
pub struct TWI_ADDR_SPEC;
impl crate::RegisterSpec for TWI_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_addr::R](R) reader structure"]
impl crate::Readable for TWI_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_addr::W](W) writer structure"]
impl crate::Writable for TWI_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_ADDR to value 0"]
impl crate::Resettable for TWI_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
