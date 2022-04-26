#[doc = "Register `USB0_CLK` reader"]
pub struct R(crate::R<USB0_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0_CLK` writer"]
pub struct W(crate::W<USB0_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_CLK_SPEC>;
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
impl From<crate::W<USB0_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating Special Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CLKEN` reader - Gating Special Clock"]
pub struct CLKEN_R(crate::FieldReader<bool>);
impl CLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::OFF,
            true => CLKEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLKEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLKEN_A::ON
    }
}
impl core::ops::Deref for CLKEN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKEN` writer - Gating Special Clock"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKEN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLKEN_A::ON)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "PHY Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RSTN` reader - PHY Reset"]
pub struct RSTN_R(crate::FieldReader<bool>);
impl RSTN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTN_A {
        match self.bits {
            false => RSTN_A::ASSERT,
            true => RSTN_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == RSTN_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == RSTN_A::DEASSERT
    }
}
impl core::ops::Deref for RSTN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTN` writer - PHY Reset"]
pub struct RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(RSTN_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(RSTN_A::DEASSERT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "OHCI 12M Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CLK12M_SEL` reader - OHCI 12M Source Select"]
pub struct CLK12M_SEL_R(crate::FieldReader<u8>);
impl CLK12M_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK12M_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK12M_SEL_A> {
        match self.bits {
            0 => Some(CLK12M_SEL_A::DIV_48M),
            1 => Some(CLK12M_SEL_A::DIV_24M),
            2 => Some(CLK12M_SEL_A::RTC_32K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV_48M`"]
    #[inline(always)]
    pub fn is_div_48m(&self) -> bool {
        **self == CLK12M_SEL_A::DIV_48M
    }
    #[doc = "Checks if the value of the field is `DIV_24M`"]
    #[inline(always)]
    pub fn is_div_24m(&self) -> bool {
        **self == CLK12M_SEL_A::DIV_24M
    }
    #[doc = "Checks if the value of the field is `RTC_32K`"]
    #[inline(always)]
    pub fn is_rtc_32k(&self) -> bool {
        **self == CLK12M_SEL_A::RTC_32K
    }
}
impl core::ops::Deref for CLK12M_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK12M_SEL` writer - OHCI 12M Source Select"]
pub struct CLK12M_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK12M_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK12M_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div_48m(self) -> &'a mut W {
        self.variant(CLK12M_SEL_A::DIV_48M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div_24m(self) -> &'a mut W {
        self.variant(CLK12M_SEL_A::DIV_24M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rtc_32k(self) -> &'a mut W {
        self.variant(CLK12M_SEL_A::RTC_32K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - PHY Reset"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:25 - OHCI 12M Source Select"]
    #[inline(always)]
    pub fn clk12m_sel(&self) -> CLK12M_SEL_R {
        CLK12M_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 30 - PHY Reset"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RSTN_W {
        RSTN_W { w: self }
    }
    #[doc = "Bits 24:25 - OHCI 12M Source Select"]
    #[inline(always)]
    pub fn clk12m_sel(&mut self) -> CLK12M_SEL_W {
        CLK12M_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB0 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_clk](index.html) module"]
pub struct USB0_CLK_SPEC;
impl crate::RegisterSpec for USB0_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0_clk::R](R) reader structure"]
impl crate::Readable for USB0_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_clk::W](W) writer structure"]
impl crate::Writable for USB0_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB0_CLK to value 0"]
impl crate::Resettable for USB0_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
