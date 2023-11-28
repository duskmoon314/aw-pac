#[doc = "Register `pclk_fan` reader"]
pub type R = crate::R<PCLK_FAN_SPEC>;
#[doc = "Register `pclk_fan` writer"]
pub type W = crate::W<PCLK_FAN_SPEC>;
#[doc = "Field `div` reader - Factor M"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `div` writer - Factor M"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gating` reader - Gating for PCLK"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating for PCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
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
    pub const fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::OFF,
            true => GATING_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == GATING_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == GATING_A::ON
    }
}
#[doc = "Field `gating` writer - Gating for PCLK"]
pub type GATING_W<'a, REG> = crate::BitWriter<'a, REG, GATING_A>;
impl<'a, REG> GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Gating for PCLK"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<PCLK_FAN_SPEC> {
        DIV_W::new(self, 0)
    }
    #[doc = "Bit 31 - Gating for PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<PCLK_FAN_SPEC> {
        GATING_W::new(self, 31)
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
#[doc = "PCLK FANOUT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclk_fan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclk_fan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLK_FAN_SPEC;
impl crate::RegisterSpec for PCLK_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclk_fan::R`](R) reader structure"]
impl crate::Readable for PCLK_FAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclk_fan::W`](W) writer structure"]
impl crate::Writable for PCLK_FAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pclk_fan to value 0"]
impl crate::Resettable for PCLK_FAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
