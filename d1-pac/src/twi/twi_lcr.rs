#[doc = "Register `twi_lcr` reader"]
pub type R = crate::R<TWI_LCR_SPEC>;
#[doc = "Register `twi_lcr` writer"]
pub type W = crate::W<TWI_LCR_SPEC>;
#[doc = "Field `sda_ctl_en` reader - TWI_SDA Line State Control Enable"]
pub type SDA_CTL_EN_R = crate::BitReader<SDA_CTL_EN_A>;
#[doc = "TWI_SDA Line State Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDA_CTL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SDA_CTL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_CTL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDA_CTL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDA_CTL_EN_A {
        match self.bits {
            false => SDA_CTL_EN_A::DISABLE,
            true => SDA_CTL_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDA_CTL_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDA_CTL_EN_A::ENABLE
    }
}
#[doc = "Field `sda_ctl_en` writer - TWI_SDA Line State Control Enable"]
pub type SDA_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG, SDA_CTL_EN_A>;
impl<'a, REG> SDA_CTL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SDA_CTL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SDA_CTL_EN_A::ENABLE)
    }
}
#[doc = "Field `sda_ctl` reader - TWI_SDA Line State Control Bit"]
pub type SDA_CTL_R = crate::BitReader<SDA_CTL_A>;
#[doc = "TWI_SDA Line State Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDA_CTL_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SDA_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDA_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDA_CTL_A {
        match self.bits {
            false => SDA_CTL_A::LOW,
            true => SDA_CTL_A::HIGH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SDA_CTL_A::LOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SDA_CTL_A::HIGH
    }
}
#[doc = "Field `sda_ctl` writer - TWI_SDA Line State Control Bit"]
pub type SDA_CTL_W<'a, REG> = crate::BitWriter<'a, REG, SDA_CTL_A>;
impl<'a, REG> SDA_CTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SDA_CTL_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SDA_CTL_A::HIGH)
    }
}
#[doc = "Field `scl_ctl_en` reader - TWI_SCL Line State Control Enable"]
pub type SCL_CTL_EN_R = crate::BitReader<SCL_CTL_EN_A>;
#[doc = "TWI_SCL Line State Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCL_CTL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SCL_CTL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_CTL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCL_CTL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCL_CTL_EN_A {
        match self.bits {
            false => SCL_CTL_EN_A::DISABLE,
            true => SCL_CTL_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCL_CTL_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCL_CTL_EN_A::ENABLE
    }
}
#[doc = "Field `scl_ctl_en` writer - TWI_SCL Line State Control Enable"]
pub type SCL_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG, SCL_CTL_EN_A>;
impl<'a, REG> SCL_CTL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_CTL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_CTL_EN_A::ENABLE)
    }
}
#[doc = "Field `scl_ctl` reader - TWI_SCL Line State Control Bit"]
pub type SCL_CTL_R = crate::BitReader<SCL_CTL_A>;
#[doc = "TWI_SCL Line State Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCL_CTL_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SCL_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl SCL_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCL_CTL_A {
        match self.bits {
            false => SCL_CTL_A::LOW,
            true => SCL_CTL_A::HIGH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SCL_CTL_A::LOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SCL_CTL_A::HIGH
    }
}
#[doc = "Field `scl_ctl` writer - TWI_SCL Line State Control Bit"]
pub type SCL_CTL_W<'a, REG> = crate::BitWriter<'a, REG, SCL_CTL_A>;
impl<'a, REG> SCL_CTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_CTL_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SCL_CTL_A::HIGH)
    }
}
#[doc = "Field `sda_state` reader - Current State of TWI_SDA"]
pub type SDA_STATE_R = crate::BitReader<SDA_STATE_A>;
#[doc = "Current State of TWI_SDA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDA_STATE_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SDA_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SDA_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDA_STATE_A {
        match self.bits {
            false => SDA_STATE_A::LOW,
            true => SDA_STATE_A::HIGH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SDA_STATE_A::LOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SDA_STATE_A::HIGH
    }
}
#[doc = "Field `scl_state` reader - Current State of TWI_SCL"]
pub type SCL_STATE_R = crate::BitReader<SCL_STATE_A>;
#[doc = "Current State of TWI_SCL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCL_STATE_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SCL_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCL_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCL_STATE_A {
        match self.bits {
            false => SCL_STATE_A::LOW,
            true => SCL_STATE_A::HIGH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SCL_STATE_A::LOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SCL_STATE_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - TWI_SDA Line State Control Enable"]
    #[inline(always)]
    pub fn sda_ctl_en(&self) -> SDA_CTL_EN_R {
        SDA_CTL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TWI_SDA Line State Control Bit"]
    #[inline(always)]
    pub fn sda_ctl(&self) -> SDA_CTL_R {
        SDA_CTL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TWI_SCL Line State Control Enable"]
    #[inline(always)]
    pub fn scl_ctl_en(&self) -> SCL_CTL_EN_R {
        SCL_CTL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI_SCL Line State Control Bit"]
    #[inline(always)]
    pub fn scl_ctl(&self) -> SCL_CTL_R {
        SCL_CTL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Current State of TWI_SDA"]
    #[inline(always)]
    pub fn sda_state(&self) -> SDA_STATE_R {
        SDA_STATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Current State of TWI_SCL"]
    #[inline(always)]
    pub fn scl_state(&self) -> SCL_STATE_R {
        SCL_STATE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI_SDA Line State Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sda_ctl_en(&mut self) -> SDA_CTL_EN_W<TWI_LCR_SPEC> {
        SDA_CTL_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TWI_SDA Line State Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sda_ctl(&mut self) -> SDA_CTL_W<TWI_LCR_SPEC> {
        SDA_CTL_W::new(self, 1)
    }
    #[doc = "Bit 2 - TWI_SCL Line State Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_ctl_en(&mut self) -> SCL_CTL_EN_W<TWI_LCR_SPEC> {
        SCL_CTL_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TWI_SCL Line State Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn scl_ctl(&mut self) -> SCL_CTL_W<TWI_LCR_SPEC> {
        SCL_CTL_W::new(self, 3)
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
#[doc = "TWI Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_LCR_SPEC;
impl crate::RegisterSpec for TWI_LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_lcr::R`](R) reader structure"]
impl crate::Readable for TWI_LCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_lcr::W`](W) writer structure"]
impl crate::Writable for TWI_LCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_lcr to value 0"]
impl crate::Resettable for TWI_LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
