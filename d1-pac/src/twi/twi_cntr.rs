#[doc = "Register `twi_cntr` reader"]
pub type R = crate::R<TWI_CNTR_SPEC>;
#[doc = "Register `twi_cntr` writer"]
pub type W = crate::W<TWI_CNTR_SPEC>;
#[doc = "Field `clk_count_mode` reader - "]
pub type CLK_COUNT_MODE_R = crate::BitReader<CLK_COUNT_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_COUNT_MODE_A {
    #[doc = "0: scl clock high period count on oscl"]
    OSCL = 0,
    #[doc = "1: scl clock high period count on iscl"]
    ISCL = 1,
}
impl From<CLK_COUNT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_COUNT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_COUNT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_COUNT_MODE_A {
        match self.bits {
            false => CLK_COUNT_MODE_A::OSCL,
            true => CLK_COUNT_MODE_A::ISCL,
        }
    }
    #[doc = "scl clock high period count on oscl"]
    #[inline(always)]
    pub fn is_oscl(&self) -> bool {
        *self == CLK_COUNT_MODE_A::OSCL
    }
    #[doc = "scl clock high period count on iscl"]
    #[inline(always)]
    pub fn is_iscl(&self) -> bool {
        *self == CLK_COUNT_MODE_A::ISCL
    }
}
#[doc = "Field `clk_count_mode` writer - "]
pub type CLK_COUNT_MODE_W<'a, REG> = crate::BitWriter<'a, REG, CLK_COUNT_MODE_A>;
impl<'a, REG> CLK_COUNT_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "scl clock high period count on oscl"]
    #[inline(always)]
    pub fn oscl(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_COUNT_MODE_A::OSCL)
    }
    #[doc = "scl clock high period count on iscl"]
    #[inline(always)]
    pub fn iscl(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_COUNT_MODE_A::ISCL)
    }
}
#[doc = "Field `a_ack` reader - Assert Acknowledge"]
pub type A_ACK_R = crate::BitReader;
#[doc = "Field `a_ack` writer - Assert Acknowledge"]
pub type A_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int_flag` reader - Interrupt Flag"]
pub type INT_FLAG_R = crate::BitReader;
#[doc = "Field `int_flag` writer - Interrupt Flag"]
pub type INT_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `m_stp` reader - Master Mode Stop"]
pub type M_STP_R = crate::BitReader;
#[doc = "Field `m_stp` writer - Master Mode Stop"]
pub type M_STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `m_sta` reader - Master Mode Start"]
pub type M_STA_R = crate::BitReader;
#[doc = "Field `m_sta` writer - Master Mode Start"]
pub type M_STA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bus_en` reader - TWI Bus Enable"]
pub type BUS_EN_R = crate::BitReader<BUS_EN_A>;
#[doc = "TWI Bus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUS_EN_A {
    #[doc = "0: `0`"]
    IGNORED = 0,
    #[doc = "1: `1`"]
    RESPOND = 1,
}
impl From<BUS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BUS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUS_EN_A {
        match self.bits {
            false => BUS_EN_A::IGNORED,
            true => BUS_EN_A::RESPOND,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == BUS_EN_A::IGNORED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == BUS_EN_A::RESPOND
    }
}
#[doc = "Field `bus_en` writer - TWI Bus Enable"]
pub type BUS_EN_W<'a, REG> = crate::BitWriter<'a, REG, BUS_EN_A>;
impl<'a, REG> BUS_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(BUS_EN_A::IGNORED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn respond(self) -> &'a mut crate::W<REG> {
        self.variant(BUS_EN_A::RESPOND)
    }
}
#[doc = "Field `int_en` reader - Interrupt Enable"]
pub type INT_EN_R = crate::BitReader<INT_EN_A>;
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN_A {
    #[doc = "0: The interrupt line always low"]
    LOW = 0,
    #[doc = "1: The interrupt line will go high when INT_FLAG is set"]
    HIGH = 1,
}
impl From<INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_EN_A {
        match self.bits {
            false => INT_EN_A::LOW,
            true => INT_EN_A::HIGH,
        }
    }
    #[doc = "The interrupt line always low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INT_EN_A::LOW
    }
    #[doc = "The interrupt line will go high when INT_FLAG is set"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INT_EN_A::HIGH
    }
}
#[doc = "Field `int_en` writer - Interrupt Enable"]
pub type INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, INT_EN_A>;
impl<'a, REG> INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt line always low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EN_A::LOW)
    }
    #[doc = "The interrupt line will go high when INT_FLAG is set"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EN_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_count_mode(&self) -> CLK_COUNT_MODE_R {
        CLK_COUNT_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Assert Acknowledge"]
    #[inline(always)]
    pub fn a_ack(&self) -> A_ACK_R {
        A_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Flag"]
    #[inline(always)]
    pub fn int_flag(&self) -> INT_FLAG_R {
        INT_FLAG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Mode Stop"]
    #[inline(always)]
    pub fn m_stp(&self) -> M_STP_R {
        M_STP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Mode Start"]
    #[inline(always)]
    pub fn m_sta(&self) -> M_STA_R {
        M_STA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TWI Bus Enable"]
    #[inline(always)]
    pub fn bus_en(&self) -> BUS_EN_R {
        BUS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_count_mode(&mut self) -> CLK_COUNT_MODE_W<TWI_CNTR_SPEC> {
        CLK_COUNT_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Assert Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn a_ack(&mut self) -> A_ACK_W<TWI_CNTR_SPEC> {
        A_ACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn int_flag(&mut self) -> INT_FLAG_W<TWI_CNTR_SPEC> {
        INT_FLAG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master Mode Stop"]
    #[inline(always)]
    #[must_use]
    pub fn m_stp(&mut self) -> M_STP_W<TWI_CNTR_SPEC> {
        M_STP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Master Mode Start"]
    #[inline(always)]
    #[must_use]
    pub fn m_sta(&mut self) -> M_STA_W<TWI_CNTR_SPEC> {
        M_STA_W::new(self, 5)
    }
    #[doc = "Bit 6 - TWI Bus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bus_en(&mut self) -> BUS_EN_W<TWI_CNTR_SPEC> {
        BUS_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<TWI_CNTR_SPEC> {
        INT_EN_W::new(self, 7)
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
#[doc = "TWI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_CNTR_SPEC;
impl crate::RegisterSpec for TWI_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_cntr::R`](R) reader structure"]
impl crate::Readable for TWI_CNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_cntr::W`](W) writer structure"]
impl crate::Writable for TWI_CNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_cntr to value 0"]
impl crate::Resettable for TWI_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
