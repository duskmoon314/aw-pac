#[doc = "Register `twi_addr` reader"]
pub type R = crate::R<TWI_ADDR_SPEC>;
#[doc = "Register `twi_addr` writer"]
pub type W = crate::W<TWI_ADDR_SPEC>;
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
    pub const fn variant(&self) -> GCE_A {
        match self.bits {
            false => GCE_A::DISABLE,
            true => GCE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GCE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GCE_A::ENABLE
    }
}
#[doc = "Field `gce` writer - "]
pub type GCE_W<'a, REG> = crate::BitWriter<'a, REG, GCE_A>;
impl<'a, REG> GCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GCE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GCE_A::ENABLE)
    }
}
#[doc = "Field `sla` reader - Slave Address"]
pub type SLA_R = crate::FieldReader;
#[doc = "Field `sla` writer - Slave Address"]
pub type SLA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
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
    pub fn gce(&mut self) -> GCE_W<TWI_ADDR_SPEC> {
        GCE_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn sla(&mut self) -> SLA_W<TWI_ADDR_SPEC> {
        SLA_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWI Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_ADDR_SPEC;
impl crate::RegisterSpec for TWI_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_addr::R`](R) reader structure"]
impl crate::Readable for TWI_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_addr::W`](W) writer structure"]
impl crate::Writable for TWI_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_addr to value 0"]
impl crate::Resettable for TWI_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
