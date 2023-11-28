#[doc = "Register `fcc` reader"]
pub type R = crate::R<FCC_SPEC>;
#[doc = "Register `fcc` writer"]
pub type W = crate::W<FCC_SPEC>;
#[doc = "Field `rx_fifo_clock_enable` reader - "]
pub type RX_FIFO_CLOCK_ENABLE_R = crate::BitReader<RX_FIFO_CLOCK_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_CLOCK_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_FIFO_CLOCK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLOCK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_CLOCK_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_FIFO_CLOCK_ENABLE_A {
        match self.bits {
            false => RX_FIFO_CLOCK_ENABLE_A::DISABLE,
            true => RX_FIFO_CLOCK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_FIFO_CLOCK_ENABLE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_FIFO_CLOCK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `rx_fifo_clock_enable` writer - "]
pub type RX_FIFO_CLOCK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, RX_FIFO_CLOCK_ENABLE_A>;
impl<'a, REG> RX_FIFO_CLOCK_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLOCK_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLOCK_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `tx_fifo_clock_enable` reader - "]
pub type TX_FIFO_CLOCK_ENABLE_R = crate::BitReader<TX_FIFO_CLOCK_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FIFO_CLOCK_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_FIFO_CLOCK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_CLOCK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FIFO_CLOCK_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_FIFO_CLOCK_ENABLE_A {
        match self.bits {
            false => TX_FIFO_CLOCK_ENABLE_A::DISABLE,
            true => TX_FIFO_CLOCK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_FIFO_CLOCK_ENABLE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_FIFO_CLOCK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `tx_fifo_clock_enable` writer - "]
pub type TX_FIFO_CLOCK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, TX_FIFO_CLOCK_ENABLE_A>;
impl<'a, REG> TX_FIFO_CLOCK_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_CLOCK_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_CLOCK_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `rx_fifo_clock_mode` reader - "]
pub type RX_FIFO_CLOCK_MODE_R = crate::BitReader<RX_FIFO_CLOCK_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_CLOCK_MODE_A {
    #[doc = "0: Sync mode, writing/reading clocks use apb clock"]
    WR_APB = 0,
    #[doc = "1: Sync mode, writing clock uses apb clock, reading clock uses ahb clock"]
    W_APB_R_AHB = 1,
}
impl From<RX_FIFO_CLOCK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLOCK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_CLOCK_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_FIFO_CLOCK_MODE_A {
        match self.bits {
            false => RX_FIFO_CLOCK_MODE_A::WR_APB,
            true => RX_FIFO_CLOCK_MODE_A::W_APB_R_AHB,
        }
    }
    #[doc = "Sync mode, writing/reading clocks use apb clock"]
    #[inline(always)]
    pub fn is_wr_apb(&self) -> bool {
        *self == RX_FIFO_CLOCK_MODE_A::WR_APB
    }
    #[doc = "Sync mode, writing clock uses apb clock, reading clock uses ahb clock"]
    #[inline(always)]
    pub fn is_w_apb_r_ahb(&self) -> bool {
        *self == RX_FIFO_CLOCK_MODE_A::W_APB_R_AHB
    }
}
#[doc = "Field `rx_fifo_clock_mode` writer - "]
pub type RX_FIFO_CLOCK_MODE_W<'a, REG> = crate::BitWriter<'a, REG, RX_FIFO_CLOCK_MODE_A>;
impl<'a, REG> RX_FIFO_CLOCK_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sync mode, writing/reading clocks use apb clock"]
    #[inline(always)]
    pub fn wr_apb(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLOCK_MODE_A::WR_APB)
    }
    #[doc = "Sync mode, writing clock uses apb clock, reading clock uses ahb clock"]
    #[inline(always)]
    pub fn w_apb_r_ahb(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_CLOCK_MODE_A::W_APB_R_AHB)
    }
}
#[doc = "Field `fifo_depth` reader - "]
pub type FIFO_DEPTH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clock_enable(&self) -> RX_FIFO_CLOCK_ENABLE_R {
        RX_FIFO_CLOCK_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_clock_enable(&self) -> TX_FIFO_CLOCK_ENABLE_R {
        TX_FIFO_CLOCK_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_clock_mode(&self) -> RX_FIFO_CLOCK_MODE_R {
        RX_FIFO_CLOCK_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn fifo_depth(&self) -> FIFO_DEPTH_R {
        FIFO_DEPTH_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_clock_enable(&mut self) -> RX_FIFO_CLOCK_ENABLE_W<FCC_SPEC> {
        RX_FIFO_CLOCK_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_clock_enable(&mut self) -> TX_FIFO_CLOCK_ENABLE_W<FCC_SPEC> {
        TX_FIFO_CLOCK_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_clock_mode(&mut self) -> RX_FIFO_CLOCK_MODE_W<FCC_SPEC> {
        RX_FIFO_CLOCK_MODE_W::new(self, 2)
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
#[doc = "UART FIFO Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCC_SPEC;
impl crate::RegisterSpec for FCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcc::R`](R) reader structure"]
impl crate::Readable for FCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcc::W`](W) writer structure"]
impl crate::Writable for FCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fcc to value 0"]
impl crate::Resettable for FCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
