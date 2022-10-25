#[doc = "Register `rtc_spi_clk_ctrl` reader"]
pub struct R(crate::R<RTC_SPI_CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SPI_CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SPI_CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SPI_CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_spi_clk_ctrl` writer"]
pub struct W(crate::W<RTC_SPI_CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SPI_CLK_CTRL_SPEC>;
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
impl From<crate::W<RTC_SPI_CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SPI_CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_spi_clk_div` reader - RTC Reg CFG SPI Clock Divider: M\n\nActual SPI Clock = AHBS1/(M+1), (0 to 15) The default frequency of AHBS1 is 200 MHz, and the default frequency of SPI Clock is 20 MHz.\n\nNote: The SPI clock can not exceed 50 MHz, or else the RTC register may be abnormal."]
pub type RTC_SPI_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_spi_clk_div` writer - RTC Reg CFG SPI Clock Divider: M\n\nActual SPI Clock = AHBS1/(M+1), (0 to 15) The default frequency of AHBS1 is 200 MHz, and the default frequency of SPI Clock is 20 MHz.\n\nNote: The SPI clock can not exceed 50 MHz, or else the RTC register may be abnormal."]
pub type RTC_SPI_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_SPI_CLK_CTRL_SPEC, u8, u8, 5, O>;
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
    pub fn variant(&self) -> RTC_SPI_CLK_GATING_A {
        match self.bits {
            false => RTC_SPI_CLK_GATING_A::GATING,
            true => RTC_SPI_CLK_GATING_A::NOT_GATING,
        }
    }
    #[doc = "Checks if the value of the field is `GATING`"]
    #[inline(always)]
    pub fn is_gating(&self) -> bool {
        *self == RTC_SPI_CLK_GATING_A::GATING
    }
    #[doc = "Checks if the value of the field is `NOT_GATING`"]
    #[inline(always)]
    pub fn is_not_gating(&self) -> bool {
        *self == RTC_SPI_CLK_GATING_A::NOT_GATING
    }
}
#[doc = "Field `rtc_spi_clk_gating` writer - RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time."]
pub type RTC_SPI_CLK_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RTC_SPI_CLK_CTRL_SPEC, RTC_SPI_CLK_GATING_A, O>;
impl<'a, const O: u8> RTC_SPI_CLK_GATING_W<'a, O> {
    #[doc = "Gating"]
    #[inline(always)]
    pub fn gating(self) -> &'a mut W {
        self.variant(RTC_SPI_CLK_GATING_A::GATING)
    }
    #[doc = "Not Gating"]
    #[inline(always)]
    pub fn not_gating(self) -> &'a mut W {
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
    pub fn rtc_spi_clk_div(&mut self) -> RTC_SPI_CLK_DIV_W<0> {
        RTC_SPI_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 31 - RTC Reg CFG SPI Clock Gating\n\nBefore configurating RTC register, the clock divider of SPI needs be configured firstly, then clock gating needs be enabled.\n\nNote: Frequency division and clock gating can not be set at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_spi_clk_gating(&mut self) -> RTC_SPI_CLK_GATING_W<31> {
        RTC_SPI_CLK_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC SPI Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_spi_clk_ctrl](index.html) module"]
pub struct RTC_SPI_CLK_CTRL_SPEC;
impl crate::RegisterSpec for RTC_SPI_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_spi_clk_ctrl::R](R) reader structure"]
impl crate::Readable for RTC_SPI_CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_spi_clk_ctrl::W](W) writer structure"]
impl crate::Writable for RTC_SPI_CLK_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_spi_clk_ctrl to value 0x09"]
impl crate::Resettable for RTC_SPI_CLK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
