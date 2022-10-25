#[doc = "Register `emac_rgmii_sta` reader"]
pub struct R(crate::R<EMAC_RGMII_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RGMII_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RGMII_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RGMII_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_rgmii_sta` writer"]
pub struct W(crate::W<EMAC_RGMII_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RGMII_STA_SPEC>;
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
impl From<crate::W<EMAC_RGMII_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RGMII_STA_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> RGMII_LINK_MD_A {
        match self.bits {
            false => RGMII_LINK_MD_A::HALF_DUPLEX,
            true => RGMII_LINK_MD_A::FULL_DUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_DUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == RGMII_LINK_MD_A::HALF_DUPLEX
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RGMII_LINK_MD_A::FULL_DUPLEX
    }
}
#[doc = "Field `rgmii_link_md` writer - The link mode of the RGMII interface"]
pub type RGMII_LINK_MD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_RGMII_STA_SPEC, RGMII_LINK_MD_A, O>;
impl<'a, const O: u8> RGMII_LINK_MD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(RGMII_LINK_MD_A::HALF_DUPLEX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RGMII_LINK_MD_A::FULL_DUPLEX)
    }
}
#[doc = "Field `rgmii_link_spd` reader - The link speed of the RGMII interface"]
pub type RGMII_LINK_SPD_R = crate::FieldReader<u8, RGMII_LINK_SPD_A>;
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
impl RGMII_LINK_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGMII_LINK_SPD_A> {
        match self.bits {
            0 => Some(RGMII_LINK_SPD_A::S2_5),
            1 => Some(RGMII_LINK_SPD_A::S25),
            2 => Some(RGMII_LINK_SPD_A::S125),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `S2_5`"]
    #[inline(always)]
    pub fn is_s2_5(&self) -> bool {
        *self == RGMII_LINK_SPD_A::S2_5
    }
    #[doc = "Checks if the value of the field is `S25`"]
    #[inline(always)]
    pub fn is_s25(&self) -> bool {
        *self == RGMII_LINK_SPD_A::S25
    }
    #[doc = "Checks if the value of the field is `S125`"]
    #[inline(always)]
    pub fn is_s125(&self) -> bool {
        *self == RGMII_LINK_SPD_A::S125
    }
}
#[doc = "Field `rgmii_link_spd` writer - The link speed of the RGMII interface"]
pub type RGMII_LINK_SPD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMAC_RGMII_STA_SPEC, u8, RGMII_LINK_SPD_A, 2, O>;
impl<'a, const O: u8> RGMII_LINK_SPD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s2_5(self) -> &'a mut W {
        self.variant(RGMII_LINK_SPD_A::S2_5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn s25(self) -> &'a mut W {
        self.variant(RGMII_LINK_SPD_A::S25)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s125(self) -> &'a mut W {
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
    pub fn variant(&self) -> RGMII_LINK_A {
        match self.bits {
            false => RGMII_LINK_A::DOWN,
            true => RGMII_LINK_A::UP,
        }
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == RGMII_LINK_A::DOWN
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == RGMII_LINK_A::UP
    }
}
#[doc = "Field `rgmii_link` writer - The link status of the RGMII interface"]
pub type RGMII_LINK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_RGMII_STA_SPEC, RGMII_LINK_A, O>;
impl<'a, const O: u8> RGMII_LINK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(RGMII_LINK_A::DOWN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
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
    pub fn rgmii_link_md(&mut self) -> RGMII_LINK_MD_W<0> {
        RGMII_LINK_MD_W::new(self)
    }
    #[doc = "Bits 1:2 - The link speed of the RGMII interface"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_link_spd(&mut self) -> RGMII_LINK_SPD_W<1> {
        RGMII_LINK_SPD_W::new(self)
    }
    #[doc = "Bit 3 - The link status of the RGMII interface"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_link(&mut self) -> RGMII_LINK_W<3> {
        RGMII_LINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC RGMII Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rgmii_sta](index.html) module"]
pub struct EMAC_RGMII_STA_SPEC;
impl crate::RegisterSpec for EMAC_RGMII_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rgmii_sta::R](R) reader structure"]
impl crate::Readable for EMAC_RGMII_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rgmii_sta::W](W) writer structure"]
impl crate::Writable for EMAC_RGMII_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_rgmii_sta to value 0"]
impl crate::Resettable for EMAC_RGMII_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
