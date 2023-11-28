#[doc = "Register `usb1_clk` reader"]
pub type R = crate::R<USB1_CLK_SPEC>;
#[doc = "Register `usb1_clk` writer"]
pub type W = crate::W<USB1_CLK_SPEC>;
#[doc = "Field `clk12m_sel` reader - OHCI 12M Source Select"]
pub type CLK12M_SEL_R = crate::FieldReader<CLK12M_SEL_A>;
#[doc = "OHCI 12M Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK12M_SEL_A {
    #[doc = "0: `0`"]
    DIV_48M = 0,
    #[doc = "1: `1`"]
    DIV_24M = 1,
    #[doc = "2: `10`"]
    RTC_32K = 2,
}
impl From<CLK12M_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK12M_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK12M_SEL_A {
    type Ux = u8;
}
impl CLK12M_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK12M_SEL_A> {
        match self.bits {
            0 => Some(CLK12M_SEL_A::DIV_48M),
            1 => Some(CLK12M_SEL_A::DIV_24M),
            2 => Some(CLK12M_SEL_A::RTC_32K),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_div_48m(&self) -> bool {
        *self == CLK12M_SEL_A::DIV_48M
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_div_24m(&self) -> bool {
        *self == CLK12M_SEL_A::DIV_24M
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rtc_32k(&self) -> bool {
        *self == CLK12M_SEL_A::RTC_32K
    }
}
#[doc = "Field `clk12m_sel` writer - OHCI 12M Source Select"]
pub type CLK12M_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK12M_SEL_A>;
impl<'a, REG> CLK12M_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div_48m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK12M_SEL_A::DIV_48M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div_24m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK12M_SEL_A::DIV_24M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rtc_32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLK12M_SEL_A::RTC_32K)
    }
}
#[doc = "Field `rstn` reader - PHY Reset"]
pub type RSTN_R = crate::BitReader<RSTN_A>;
#[doc = "PHY Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTN_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<RSTN_A> for bool {
    #[inline(always)]
    fn from(variant: RSTN_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTN_A {
        match self.bits {
            false => RSTN_A::ASSERT,
            true => RSTN_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == RSTN_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == RSTN_A::DEASSERT
    }
}
#[doc = "Field `rstn` writer - PHY Reset"]
pub type RSTN_W<'a, REG> = crate::BitWriter<'a, REG, RSTN_A>;
impl<'a, REG> RSTN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(RSTN_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(RSTN_A::DEASSERT)
    }
}
#[doc = "Field `clken` reader - Gating Special Clock"]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
#[doc = "Gating Special Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::OFF,
            true => CLKEN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLKEN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLKEN_A::ON
    }
}
#[doc = "Field `clken` writer - Gating Special Clock"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN_A>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::ON)
    }
}
impl R {
    #[doc = "Bits 24:25 - OHCI 12M Source Select"]
    #[inline(always)]
    pub fn clk12m_sel(&self) -> CLK12M_SEL_R {
        CLK12M_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - PHY Reset"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - OHCI 12M Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk12m_sel(&mut self) -> CLK12M_SEL_W<USB1_CLK_SPEC> {
        CLK12M_SEL_W::new(self, 24)
    }
    #[doc = "Bit 30 - PHY Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn(&mut self) -> RSTN_W<USB1_CLK_SPEC> {
        RSTN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<USB1_CLK_SPEC> {
        CLKEN_W::new(self, 31)
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
#[doc = "USB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb1_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb1_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB1_CLK_SPEC;
impl crate::RegisterSpec for USB1_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_clk::R`](R) reader structure"]
impl crate::Readable for USB1_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb1_clk::W`](W) writer structure"]
impl crate::Writable for USB1_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb1_clk to value 0"]
impl crate::Resettable for USB1_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
