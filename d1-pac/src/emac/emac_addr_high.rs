#[doc = "Register `emac_addr_high%s` reader"]
pub type R = crate::R<EMAC_ADDR_HIGH_SPEC>;
#[doc = "Register `emac_addr_high%s` writer"]
pub type W = crate::W<EMAC_ADDR_HIGH_SPEC>;
#[doc = "Field `mac_addr_high` reader - "]
pub type MAC_ADDR_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `mac_addr_high` writer - "]
pub type MAC_ADDR_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `mac_addr_byte_ctl` reader - MAC Address Byte Control Mask"]
pub type MAC_ADDR_BYTE_CTL_R = crate::FieldReader;
#[doc = "Field `mac_addr_byte_ctl` writer - MAC Address Byte Control Mask"]
pub type MAC_ADDR_BYTE_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `mac_addr_type` reader - MAC Address Type"]
pub type MAC_ADDR_TYPE_R = crate::BitReader<MAC_ADDR_TYPE_A>;
#[doc = "MAC Address Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAC_ADDR_TYPE_A {
    #[doc = "0: `0`"]
    DST = 0,
    #[doc = "1: `1`"]
    SRC = 1,
}
impl From<MAC_ADDR_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_ADDR_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl MAC_ADDR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MAC_ADDR_TYPE_A {
        match self.bits {
            false => MAC_ADDR_TYPE_A::DST,
            true => MAC_ADDR_TYPE_A::SRC,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_dst(&self) -> bool {
        *self == MAC_ADDR_TYPE_A::DST
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_src(&self) -> bool {
        *self == MAC_ADDR_TYPE_A::SRC
    }
}
#[doc = "Field `mac_addr_type` writer - MAC Address Type"]
pub type MAC_ADDR_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, MAC_ADDR_TYPE_A>;
impl<'a, REG> MAC_ADDR_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dst(self) -> &'a mut crate::W<REG> {
        self.variant(MAC_ADDR_TYPE_A::DST)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn src(self) -> &'a mut crate::W<REG> {
        self.variant(MAC_ADDR_TYPE_A::SRC)
    }
}
#[doc = "Field `mac_addr_ctl` reader - MAC Address Valid"]
pub type MAC_ADDR_CTL_R = crate::BitReader<MAC_ADDR_CTL_A>;
#[doc = "MAC Address Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAC_ADDR_CTL_A {
    #[doc = "0: `0`"]
    INVALID = 0,
    #[doc = "1: `1`"]
    VALID = 1,
}
impl From<MAC_ADDR_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_ADDR_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl MAC_ADDR_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MAC_ADDR_CTL_A {
        match self.bits {
            false => MAC_ADDR_CTL_A::INVALID,
            true => MAC_ADDR_CTL_A::VALID,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == MAC_ADDR_CTL_A::INVALID
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == MAC_ADDR_CTL_A::VALID
    }
}
#[doc = "Field `mac_addr_ctl` writer - MAC Address Valid"]
pub type MAC_ADDR_CTL_W<'a, REG> = crate::BitWriter<'a, REG, MAC_ADDR_CTL_A>;
impl<'a, REG> MAC_ADDR_CTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(MAC_ADDR_CTL_A::INVALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(MAC_ADDR_CTL_A::VALID)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_addr_high(&self) -> MAC_ADDR_HIGH_R {
        MAC_ADDR_HIGH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MAC Address Byte Control Mask"]
    #[inline(always)]
    pub fn mac_addr_byte_ctl(&self) -> MAC_ADDR_BYTE_CTL_R {
        MAC_ADDR_BYTE_CTL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - MAC Address Type"]
    #[inline(always)]
    pub fn mac_addr_type(&self) -> MAC_ADDR_TYPE_R {
        MAC_ADDR_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MAC Address Valid"]
    #[inline(always)]
    pub fn mac_addr_ctl(&self) -> MAC_ADDR_CTL_R {
        MAC_ADDR_CTL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mac_addr_high(&mut self) -> MAC_ADDR_HIGH_W<EMAC_ADDR_HIGH_SPEC> {
        MAC_ADDR_HIGH_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - MAC Address Byte Control Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mac_addr_byte_ctl(&mut self) -> MAC_ADDR_BYTE_CTL_W<EMAC_ADDR_HIGH_SPEC> {
        MAC_ADDR_BYTE_CTL_W::new(self, 24)
    }
    #[doc = "Bit 30 - MAC Address Type"]
    #[inline(always)]
    #[must_use]
    pub fn mac_addr_type(&mut self) -> MAC_ADDR_TYPE_W<EMAC_ADDR_HIGH_SPEC> {
        MAC_ADDR_TYPE_W::new(self, 30)
    }
    #[doc = "Bit 31 - MAC Address Valid"]
    #[inline(always)]
    #[must_use]
    pub fn mac_addr_ctl(&mut self) -> MAC_ADDR_CTL_W<EMAC_ADDR_HIGH_SPEC> {
        MAC_ADDR_CTL_W::new(self, 31)
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
#[doc = "EMAC MAC Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_addr_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_addr_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for EMAC_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_addr_high::R`](R) reader structure"]
impl crate::Readable for EMAC_ADDR_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_addr_high::W`](W) writer structure"]
impl crate::Writable for EMAC_ADDR_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_addr_high%s to value 0"]
impl crate::Resettable for EMAC_ADDR_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
