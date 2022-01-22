#[doc = "Register `wdog_cfg` reader"]
pub struct R(crate::R<WDOG_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_cfg` writer"]
pub struct W(crate::W<WDOG_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_CFG_SPEC>;
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
impl From<crate::W<WDOG_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_FIELD` writer - Key Field"]
pub struct KEY_FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Select the clock source for the watchdog.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_CLK_SRC_A {
    #[doc = "0: `0`"]
    HOSC_32K = 0,
    #[doc = "1: `1`"]
    LOSC_32K = 1,
}
impl From<WDOG_CLK_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_CLK_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG_CLK_SRC` reader - Select the clock source for the watchdog."]
pub struct WDOG_CLK_SRC_R(crate::FieldReader<bool, WDOG_CLK_SRC_A>);
impl WDOG_CLK_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_CLK_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_CLK_SRC_A {
        match self.bits {
            false => WDOG_CLK_SRC_A::HOSC_32K,
            true => WDOG_CLK_SRC_A::LOSC_32K,
        }
    }
    #[doc = "Checks if the value of the field is `HOSC_32K`"]
    #[inline(always)]
    pub fn is_hosc_32k(&self) -> bool {
        **self == WDOG_CLK_SRC_A::HOSC_32K
    }
    #[doc = "Checks if the value of the field is `LOSC_32K`"]
    #[inline(always)]
    pub fn is_losc_32k(&self) -> bool {
        **self == WDOG_CLK_SRC_A::LOSC_32K
    }
}
impl core::ops::Deref for WDOG_CLK_SRC_R {
    type Target = crate::FieldReader<bool, WDOG_CLK_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_CLK_SRC` writer - Select the clock source for the watchdog."]
pub struct WDOG_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_CLK_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc_32k(self) -> &'a mut W {
        self.variant(WDOG_CLK_SRC_A::HOSC_32K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn losc_32k(self) -> &'a mut W {
        self.variant(WDOG_CLK_SRC_A::LOSC_32K)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Configure the operating mode for the watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDOG_MODE_A {
    #[doc = "1: `1`"]
    WHOLD_SYSTEM = 1,
    #[doc = "2: `10`"]
    ONLY_INTERRUPT = 2,
}
impl From<WDOG_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOG_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDOG_MODE` reader - Configure the operating mode for the watchdog"]
pub struct WDOG_MODE_R(crate::FieldReader<u8, WDOG_MODE_A>);
impl WDOG_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOG_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDOG_MODE_A> {
        match self.bits {
            1 => Some(WDOG_MODE_A::WHOLD_SYSTEM),
            2 => Some(WDOG_MODE_A::ONLY_INTERRUPT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WHOLD_SYSTEM`"]
    #[inline(always)]
    pub fn is_whold_system(&self) -> bool {
        **self == WDOG_MODE_A::WHOLD_SYSTEM
    }
    #[doc = "Checks if the value of the field is `ONLY_INTERRUPT`"]
    #[inline(always)]
    pub fn is_only_interrupt(&self) -> bool {
        **self == WDOG_MODE_A::ONLY_INTERRUPT
    }
}
impl core::ops::Deref for WDOG_MODE_R {
    type Target = crate::FieldReader<u8, WDOG_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_MODE` writer - Configure the operating mode for the watchdog"]
pub struct WDOG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn whold_system(self) -> &'a mut W {
        self.variant(WDOG_MODE_A::WHOLD_SYSTEM)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn only_interrupt(self) -> &'a mut W {
        self.variant(WDOG_MODE_A::ONLY_INTERRUPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Select the clock source for the watchdog."]
    #[inline(always)]
    pub fn wdog_clk_src(&self) -> WDOG_CLK_SRC_R {
        WDOG_CLK_SRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Configure the operating mode for the watchdog"]
    #[inline(always)]
    pub fn wdog_mode(&self) -> WDOG_MODE_R {
        WDOG_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Key Field"]
    #[inline(always)]
    pub fn key_field(&mut self) -> KEY_FIELD_W {
        KEY_FIELD_W { w: self }
    }
    #[doc = "Bit 8 - Select the clock source for the watchdog."]
    #[inline(always)]
    pub fn wdog_clk_src(&mut self) -> WDOG_CLK_SRC_W {
        WDOG_CLK_SRC_W { w: self }
    }
    #[doc = "Bits 0:1 - Configure the operating mode for the watchdog"]
    #[inline(always)]
    pub fn wdog_mode(&mut self) -> WDOG_MODE_W {
        WDOG_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_cfg](index.html) module"]
pub struct WDOG_CFG_SPEC;
impl crate::RegisterSpec for WDOG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_cfg::R](R) reader structure"]
impl crate::Readable for WDOG_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_cfg::W](W) writer structure"]
impl crate::Writable for WDOG_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdog_cfg to value 0"]
impl crate::Resettable for WDOG_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
