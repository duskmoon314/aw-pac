#[doc = "Register `emac_rgmii_sta` reader"]
pub type R = crate::R<EMAC_RGMII_STA_SPEC>;
#[doc = "Register `emac_rgmii_sta` writer"]
pub type W = crate::W<EMAC_RGMII_STA_SPEC>;
#[doc = "Field `rgmii_link_md` reader - The link mode of the RGMII interface"]
pub type RGMII_LINK_MD_R = crate::BitReader<RGMII_LINK_MD_A>;
#[doc = "The link mode of the RGMII interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGMII_LINK_MD_A {
    #[doc = "0: `0`"]
    HALF_DUPLEX = 0,
    #[doc = "1: `1`"]
    FULL_DUPLEX = 1,
}
impl From<RGMII_LINK_MD_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_LINK_MD_A) -> Self {
        variant as u8 != 0
    }
}
impl RGMII_LINK_MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RGMII_LINK_MD_A {
        match self.bits {
            false => RGMII_LINK_MD_A::HALF_DUPLEX,
            true => RGMII_LINK_MD_A::FULL_DUPLEX,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == RGMII_LINK_MD_A::HALF_DUPLEX
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RGMII_LINK_MD_A::FULL_DUPLEX
    }
}
#[doc = "Field `rgmii_link_md` writer - The link mode of the RGMII interface"]
pub type RGMII_LINK_MD_W<'a, REG> = crate::BitWriter<'a, REG, RGMII_LINK_MD_A>;
impl<'a, REG> RGMII_LINK_MD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_MD_A::HALF_DUPLEX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_MD_A::FULL_DUPLEX)
    }
}
#[doc = "Field `rgmii_link_spd` reader - The link speed of the RGMII interface"]
pub type RGMII_LINK_SPD_R = crate::FieldReader<RGMII_LINK_SPD_A>;
#[doc = "The link speed of the RGMII interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGMII_LINK_SPD_A {
    #[doc = "0: `0`"]
    S2_5 = 0,
    #[doc = "1: `1`"]
    S25 = 1,
    #[doc = "2: `10`"]
    S125 = 2,
}
impl From<RGMII_LINK_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: RGMII_LINK_SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RGMII_LINK_SPD_A {
    type Ux = u8;
}
impl RGMII_LINK_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RGMII_LINK_SPD_A> {
        match self.bits {
            0 => Some(RGMII_LINK_SPD_A::S2_5),
            1 => Some(RGMII_LINK_SPD_A::S25),
            2 => Some(RGMII_LINK_SPD_A::S125),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_s2_5(&self) -> bool {
        *self == RGMII_LINK_SPD_A::S2_5
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_s25(&self) -> bool {
        *self == RGMII_LINK_SPD_A::S25
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_s125(&self) -> bool {
        *self == RGMII_LINK_SPD_A::S125
    }
}
#[doc = "Field `rgmii_link_spd` writer - The link speed of the RGMII interface"]
pub type RGMII_LINK_SPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RGMII_LINK_SPD_A>;
impl<'a, REG> RGMII_LINK_SPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s2_5(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_SPD_A::S2_5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn s25(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_SPD_A::S25)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s125(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_SPD_A::S125)
    }
}
#[doc = "Field `rgmii_link` reader - The link status of the RGMII interface"]
pub type RGMII_LINK_R = crate::BitReader<RGMII_LINK_A>;
#[doc = "The link status of the RGMII interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGMII_LINK_A {
    #[doc = "0: `0`"]
    DOWN = 0,
    #[doc = "1: `1`"]
    UP = 1,
}
impl From<RGMII_LINK_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_LINK_A) -> Self {
        variant as u8 != 0
    }
}
impl RGMII_LINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RGMII_LINK_A {
        match self.bits {
            false => RGMII_LINK_A::DOWN,
            true => RGMII_LINK_A::UP,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == RGMII_LINK_A::DOWN
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == RGMII_LINK_A::UP
    }
}
#[doc = "Field `rgmii_link` writer - The link status of the RGMII interface"]
pub type RGMII_LINK_W<'a, REG> = crate::BitWriter<'a, REG, RGMII_LINK_A>;
impl<'a, REG> RGMII_LINK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_A::DOWN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_LINK_A::UP)
    }
}
impl R {
    #[doc = "Bit 0 - The link mode of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link_md(&self) -> RGMII_LINK_MD_R {
        RGMII_LINK_MD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The link speed of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link_spd(&self) -> RGMII_LINK_SPD_R {
        RGMII_LINK_SPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - The link status of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link(&self) -> RGMII_LINK_R {
        RGMII_LINK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The link mode of the RGMII interface"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_link_md(&mut self) -> RGMII_LINK_MD_W<EMAC_RGMII_STA_SPEC> {
        RGMII_LINK_MD_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - The link speed of the RGMII interface"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_link_spd(&mut self) -> RGMII_LINK_SPD_W<EMAC_RGMII_STA_SPEC> {
        RGMII_LINK_SPD_W::new(self, 1)
    }
    #[doc = "Bit 3 - The link status of the RGMII interface"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_link(&mut self) -> RGMII_LINK_W<EMAC_RGMII_STA_SPEC> {
        RGMII_LINK_W::new(self, 3)
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
#[doc = "EMAC RGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rgmii_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rgmii_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_RGMII_STA_SPEC;
impl crate::RegisterSpec for EMAC_RGMII_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_rgmii_sta::R`](R) reader structure"]
impl crate::Readable for EMAC_RGMII_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_rgmii_sta::W`](W) writer structure"]
impl crate::Writable for EMAC_RGMII_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_rgmii_sta to value 0"]
impl crate::Resettable for EMAC_RGMII_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
