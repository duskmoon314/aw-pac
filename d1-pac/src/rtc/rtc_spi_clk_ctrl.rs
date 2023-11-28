#[doc = "Register `rtc_spi_clk_ctrl` reader"]
pub type R = crate::R<RTC_SPI_CLK_CTRL_SPEC>;
#[doc = "Register `rtc_spi_clk_ctrl` writer"]
pub type W = crate::W<RTC_SPI_CLK_CTRL_SPEC>;
#[doc = "Field `rtc_spi_clk_div` reader - RTC Reg CFG SPI Clock Divider: M\n\nActual SPI Clock = AHBS1/(M+1), (0 to 15) The default frequency of AHBS1 is 200 MHz, and the default frequency of SPI Clock is 20 MHz.\n\nNote: The SPI clock can not exceed 50 MHz, or else the RTC register may be abnormal."]
pub type RTC_SPI_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `rtc_spi_clk_div` writer - RTC Reg CFG SPI Clock Divider: M\n\nActual SPI Clock = AHBS1/(M+1), (0 to 15) The default frequency of AHBS1 is 200 MHz, and the default frequency of SPI Clock is 20 MHz.\n\nNote: The SPI clock can not exceed 50 MHz, or else the RTC register may be abnormal."]
pub type RTC_SPI_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rtc_spi_clk_gating` reader - RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time."]
pub type RTC_SPI_CLK_GATING_R = crate::BitReader<RTC_SPI_CLK_GATING_A>;
#[doc = "RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_SPI_CLK_GATING_A {
    #[doc = "0: Gating"]
    GATING = 0,
    #[doc = "1: Not Gating"]
    NOT_GATING = 1,
}
impl From<RTC_SPI_CLK_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_SPI_CLK_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_SPI_CLK_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_SPI_CLK_GATING_A {
        match self.bits {
            false => RTC_SPI_CLK_GATING_A::GATING,
            true => RTC_SPI_CLK_GATING_A::NOT_GATING,
        }
    }
    #[doc = "Gating"]
    #[inline(always)]
    pub fn is_gating(&self) -> bool {
        *self == RTC_SPI_CLK_GATING_A::GATING
    }
    #[doc = "Not Gating"]
    #[inline(always)]
    pub fn is_not_gating(&self) -> bool {
        *self == RTC_SPI_CLK_GATING_A::NOT_GATING
    }
}
#[doc = "Field `rtc_spi_clk_gating` writer - RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time."]
pub type RTC_SPI_CLK_GATING_W<'a, REG> = crate::BitWriter<'a, REG, RTC_SPI_CLK_GATING_A>;
impl<'a, REG> RTC_SPI_CLK_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gating"]
    #[inline(always)]
    pub fn gating(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_SPI_CLK_GATING_A::GATING)
    }
    #[doc = "Not Gating"]
    #[inline(always)]
    pub fn not_gating(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_SPI_CLK_GATING_A::NOT_GATING)
    }
}
impl R {
    #[doc = "Bits 0:4 - RTC Reg CFG SPI Clock Divider: M\n\nActual SPI Clock = AHBS1/(M+1), (0 to 15) The default frequency of AHBS1 is 200 MHz, and the default frequency of SPI Clock is 20 MHz.\n\nNote: The SPI clock can not exceed 50 MHz, or else the RTC register may be abnormal."]
    #[inline(always)]
    pub fn rtc_spi_clk_div(&self) -> RTC_SPI_CLK_DIV_R {
        RTC_SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time."]
    #[inline(always)]
    pub fn rtc_spi_clk_gating(&self) -> RTC_SPI_CLK_GATING_R {
        RTC_SPI_CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - RTC Reg CFG SPI Clock Divider: M\n\nActual SPI Clock = AHBS1/(M+1), (0 to 15) The default frequency of AHBS1 is 200 MHz, and the default frequency of SPI Clock is 20 MHz.\n\nNote: The SPI clock can not exceed 50 MHz, or else the RTC register may be abnormal."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_spi_clk_div(&mut self) -> RTC_SPI_CLK_DIV_W<RTC_SPI_CLK_CTRL_SPEC> {
        RTC_SPI_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bit 31 - RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_spi_clk_gating(&mut self) -> RTC_SPI_CLK_GATING_W<RTC_SPI_CLK_CTRL_SPEC> {
        RTC_SPI_CLK_GATING_W::new(self, 31)
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
#[doc = "RTC SPI Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_spi_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_spi_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_SPI_CLK_CTRL_SPEC;
impl crate::RegisterSpec for RTC_SPI_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_spi_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for RTC_SPI_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_spi_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for RTC_SPI_CLK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_spi_clk_ctrl to value 0x09"]
impl crate::Resettable for RTC_SPI_CLK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
