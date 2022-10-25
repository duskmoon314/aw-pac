#[doc = "Register `twi_addr` reader"]
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
#[doc = "Register `twi_addr` writer"]
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
#[doc = "Field `gce` reader - "]
pub type GCE_R = crate::BitReader<GCE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl GCE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == GCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GCE_A::ENABLE
    }
}
#[doc = "Field `gce` writer - "]
pub type GCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_ADDR_SPEC, GCE_A, O>;
impl<'a, const O: u8> GCE_W<'a, O> {
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
}
#[doc = "Field `sla` reader - Slave Address"]
pub type SLA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sla` writer - Slave Address"]
pub type SLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TWI_ADDR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gce(&self) -> GCE_R {
        GCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address"]
    #[inline(always)]
    pub fn sla(&self) -> SLA_R {
        SLA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gce(&mut self) -> GCE_W<0> {
        GCE_W::new(self)
    }
    #[doc = "Bits 1:7 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn sla(&mut self) -> SLA_W<1> {
        SLA_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_addr to value 0"]
impl crate::Resettable for TWI_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
