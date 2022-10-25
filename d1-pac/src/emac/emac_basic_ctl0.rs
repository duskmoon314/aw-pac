#[doc = "Register `emac_basic_ctl0` reader"]
pub struct R(crate::R<EMAC_BASIC_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_BASIC_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_BASIC_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_BASIC_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_basic_ctl0` writer"]
pub struct W(crate::W<EMAC_BASIC_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_BASIC_CTL0_SPEC>;
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
impl From<crate::W<EMAC_BASIC_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_BASIC_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> DUPLEX_A {
        match self.bits {
            false => DUPLEX_A::HALF_DUPLEX,
            true => DUPLEX_A::FULL_DUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_DUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DUPLEX_A::HALF_DUPLEX
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DUPLEX_A::FULL_DUPLEX
    }
}
#[doc = "Field `duplex` writer - EMAC Transfer Mode"]
pub type DUPLEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMAC_BASIC_CTL0_SPEC, DUPLEX_A, O>;
impl<'a, const O: u8> DUPLEX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(DUPLEX_A::HALF_DUPLEX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
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
    pub fn variant(&self) -> LOOPBACK_A {
        match self.bits {
            false => LOOPBACK_A::DISABLE,
            true => LOOPBACK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOPBACK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOPBACK_A::ENABLE
    }
}
#[doc = "Field `loopback` writer - EMAC Loopback Mode For Test"]
pub type LOOPBACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_BASIC_CTL0_SPEC, LOOPBACK_A, O>;
impl<'a, const O: u8> LOOPBACK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPBACK_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOPBACK_A::ENABLE)
    }
}
#[doc = "Field `speed` reader - EMAC Working Speed"]
pub type SPEED_R = crate::FieldReader<u8, SPEED_A>;
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
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::S1000,
            2 => SPEED_A::S10,
            3 => SPEED_A::S100,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S1000`"]
    #[inline(always)]
    pub fn is_s1000(&self) -> bool {
        *self == SPEED_A::S1000
    }
    #[doc = "Checks if the value of the field is `S10`"]
    #[inline(always)]
    pub fn is_s10(&self) -> bool {
        *self == SPEED_A::S10
    }
    #[doc = "Checks if the value of the field is `S100`"]
    #[inline(always)]
    pub fn is_s100(&self) -> bool {
        *self == SPEED_A::S100
    }
}
#[doc = "Field `speed` writer - EMAC Working Speed"]
pub type SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMAC_BASIC_CTL0_SPEC, u8, SPEED_A, 2, O>;
impl<'a, const O: u8> SPEED_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s1000(self) -> &'a mut W {
        self.variant(SPEED_A::S1000)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s10(self) -> &'a mut W {
        self.variant(SPEED_A::S10)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn s100(self) -> &'a mut W {
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
    pub fn duplex(&mut self) -> DUPLEX_W<0> {
        DUPLEX_W::new(self)
    }
    #[doc = "Bit 1 - EMAC Loopback Mode For Test"]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<1> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bits 2:3 - EMAC Working Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<2> {
        SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Basic Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_basic_ctl0](index.html) module"]
pub struct EMAC_BASIC_CTL0_SPEC;
impl crate::RegisterSpec for EMAC_BASIC_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_basic_ctl0::R](R) reader structure"]
impl crate::Readable for EMAC_BASIC_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_basic_ctl0::W](W) writer structure"]
impl crate::Writable for EMAC_BASIC_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_basic_ctl0 to value 0"]
impl crate::Resettable for EMAC_BASIC_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
