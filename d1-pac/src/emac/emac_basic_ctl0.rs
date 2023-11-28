#[doc = "Register `emac_basic_ctl0` reader"]
pub type R = crate::R<EMAC_BASIC_CTL0_SPEC>;
#[doc = "Register `emac_basic_ctl0` writer"]
pub type W = crate::W<EMAC_BASIC_CTL0_SPEC>;
#[doc = "Field `duplex` reader - EMAC Transfer Mode"]
pub type DUPLEX_R = crate::BitReader<DUPLEX_A>;
#[doc = "EMAC Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUPLEX_A {
    #[doc = "0: `0`"]
    HALF_DUPLEX = 0,
    #[doc = "1: `1`"]
    FULL_DUPLEX = 1,
}
impl From<DUPLEX_A> for bool {
    #[inline(always)]
    fn from(variant: DUPLEX_A) -> Self {
        variant as u8 != 0
    }
}
impl DUPLEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DUPLEX_A {
        match self.bits {
            false => DUPLEX_A::HALF_DUPLEX,
            true => DUPLEX_A::FULL_DUPLEX,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DUPLEX_A::HALF_DUPLEX
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DUPLEX_A::FULL_DUPLEX
    }
}
#[doc = "Field `duplex` writer - EMAC Transfer Mode"]
pub type DUPLEX_W<'a, REG> = crate::BitWriter<'a, REG, DUPLEX_A>;
impl<'a, REG> DUPLEX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(DUPLEX_A::HALF_DUPLEX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(DUPLEX_A::FULL_DUPLEX)
    }
}
#[doc = "Field `loopback` reader - EMAC Loopback Mode For Test"]
pub type LOOPBACK_R = crate::BitReader<LOOPBACK_A>;
#[doc = "EMAC Loopback Mode For Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPBACK_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LOOPBACK_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPBACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPBACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOOPBACK_A {
        match self.bits {
            false => LOOPBACK_A::DISABLE,
            true => LOOPBACK_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOPBACK_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOPBACK_A::ENABLE
    }
}
#[doc = "Field `loopback` writer - EMAC Loopback Mode For Test"]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG, LOOPBACK_A>;
impl<'a, REG> LOOPBACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LOOPBACK_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LOOPBACK_A::ENABLE)
    }
}
#[doc = "Field `speed` reader - EMAC Working Speed"]
pub type SPEED_R = crate::FieldReader<SPEED_A>;
#[doc = "EMAC Working Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: `0`"]
    S1000 = 0,
    #[doc = "2: `10`"]
    S10 = 2,
    #[doc = "3: `11`"]
    S100 = 3,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEED_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::S1000,
            2 => SPEED_A::S10,
            3 => SPEED_A::S100,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_s1000(&self) -> bool {
        *self == SPEED_A::S1000
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_s10(&self) -> bool {
        *self == SPEED_A::S10
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_s100(&self) -> bool {
        *self == SPEED_A::S100
    }
}
#[doc = "Field `speed` writer - EMAC Working Speed"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPEED_A>;
impl<'a, REG> SPEED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s1000(self) -> &'a mut crate::W<REG> {
        self.variant(SPEED_A::S1000)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s10(self) -> &'a mut crate::W<REG> {
        self.variant(SPEED_A::S10)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn s100(self) -> &'a mut crate::W<REG> {
        self.variant(SPEED_A::S100)
    }
}
impl R {
    #[doc = "Bit 0 - EMAC Transfer Mode"]
    #[inline(always)]
    pub fn duplex(&self) -> DUPLEX_R {
        DUPLEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EMAC Loopback Mode For Test"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - EMAC Working Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EMAC Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn duplex(&mut self) -> DUPLEX_W<EMAC_BASIC_CTL0_SPEC> {
        DUPLEX_W::new(self, 0)
    }
    #[doc = "Bit 1 - EMAC Loopback Mode For Test"]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<EMAC_BASIC_CTL0_SPEC> {
        LOOPBACK_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - EMAC Working Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<EMAC_BASIC_CTL0_SPEC> {
        SPEED_W::new(self, 2)
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
#[doc = "EMAC Basic Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_basic_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_basic_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_BASIC_CTL0_SPEC;
impl crate::RegisterSpec for EMAC_BASIC_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_basic_ctl0::R`](R) reader structure"]
impl crate::Readable for EMAC_BASIC_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_basic_ctl0::W`](W) writer structure"]
impl crate::Writable for EMAC_BASIC_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_basic_ctl0 to value 0"]
impl crate::Resettable for EMAC_BASIC_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
