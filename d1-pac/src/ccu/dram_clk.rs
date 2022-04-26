#[doc = "Register `DRAM_CLK` reader"]
pub struct R(crate::R<DRAM_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRAM_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRAM_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRAM_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRAM_CLK` writer"]
pub struct W(crate::W<DRAM_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRAM_CLK_SPEC>;
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
impl From<crate::W<DRAM_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRAM_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_GATING` reader - Gating Clock"]
pub struct CLK_GATING_R(crate::FieldReader<bool>);
impl CLK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_GATING_A {
        match self.bits {
            false => CLK_GATING_A::OFF,
            true => CLK_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK_GATING_A::ON
    }
}
impl core::ops::Deref for CLK_GATING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_GATING` writer - Gating Clock"]
pub struct CLK_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK_GATING_A::ON)
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
#[doc = "SDRCLK Configuration 0 Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDRCLK_UPD_A {
    #[doc = "0: `0`"]
    INVALID = 0,
    #[doc = "1: `1`"]
    VALID = 1,
}
impl From<SDRCLK_UPD_A> for bool {
    #[inline(always)]
    fn from(variant: SDRCLK_UPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDRCLK_UPD` reader - SDRCLK Configuration 0 Update"]
pub struct SDRCLK_UPD_R(crate::FieldReader<bool>);
impl SDRCLK_UPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDRCLK_UPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDRCLK_UPD_A {
        match self.bits {
            false => SDRCLK_UPD_A::INVALID,
            true => SDRCLK_UPD_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == SDRCLK_UPD_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == SDRCLK_UPD_A::VALID
    }
}
impl core::ops::Deref for SDRCLK_UPD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDRCLK_UPD` writer - SDRCLK Configuration 0 Update"]
pub struct SDRCLK_UPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDRCLK_UPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDRCLK_UPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(SDRCLK_UPD_A::INVALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(SDRCLK_UPD_A::VALID)
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_DDR = 0,
    #[doc = "1: `1`"]
    PLL_AUDIO1_DIV2 = 1,
    #[doc = "2: `10`"]
    PLL_PERI_2X = 2,
    #[doc = "3: `11`"]
    PLL_PERI_800M = 3,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select"]
pub struct CLK_SRC_SEL_R(crate::FieldReader<u8>);
impl CLK_SRC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::PLL_DDR),
            1 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            2 => Some(CLK_SRC_SEL_A::PLL_PERI_2X),
            3 => Some(CLK_SRC_SEL_A::PLL_PERI_800M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_DDR`"]
    #[inline(always)]
    pub fn is_pll_ddr(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_DDR
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1_DIV2`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_800M`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_PERI_800M
    }
}
impl core::ops::Deref for CLK_SRC_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select"]
pub struct CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ddr(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_DDR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_800M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Factor N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRAM_DIV2_A {
    #[doc = "0: `0`"]
    N1 = 0,
    #[doc = "1: `1`"]
    N2 = 1,
    #[doc = "2: `10`"]
    N4 = 2,
    #[doc = "3: `11`"]
    N8 = 3,
}
impl From<DRAM_DIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: DRAM_DIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRAM_DIV2` reader - Factor N"]
pub struct DRAM_DIV2_R(crate::FieldReader<u8>);
impl DRAM_DIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRAM_DIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM_DIV2_A {
        match self.bits {
            0 => DRAM_DIV2_A::N1,
            1 => DRAM_DIV2_A::N2,
            2 => DRAM_DIV2_A::N4,
            3 => DRAM_DIV2_A::N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `N1`"]
    #[inline(always)]
    pub fn is_n1(&self) -> bool {
        **self == DRAM_DIV2_A::N1
    }
    #[doc = "Checks if the value of the field is `N2`"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        **self == DRAM_DIV2_A::N2
    }
    #[doc = "Checks if the value of the field is `N4`"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        **self == DRAM_DIV2_A::N4
    }
    #[doc = "Checks if the value of the field is `N8`"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        **self == DRAM_DIV2_A::N8
    }
}
impl core::ops::Deref for DRAM_DIV2_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRAM_DIV2` writer - Factor N"]
pub struct DRAM_DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM_DIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM_DIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn n1(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `DRAM_DIV1` reader - Factor M"]
pub struct DRAM_DIV1_R(crate::FieldReader<u8>);
impl DRAM_DIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRAM_DIV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRAM_DIV1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRAM_DIV1` writer - Factor M"]
pub struct DRAM_DIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM_DIV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 27 - SDRCLK Configuration 0 Update"]
    #[inline(always)]
    pub fn sdrclk_upd(&self) -> SDRCLK_UPD_R {
        SDRCLK_UPD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn dram_div2(&self) -> DRAM_DIV2_R {
        DRAM_DIV2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn dram_div1(&self) -> DRAM_DIV1_R {
        DRAM_DIV1_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&mut self) -> CLK_GATING_W {
        CLK_GATING_W { w: self }
    }
    #[doc = "Bit 27 - SDRCLK Configuration 0 Update"]
    #[inline(always)]
    pub fn sdrclk_upd(&mut self) -> SDRCLK_UPD_W {
        SDRCLK_UPD_W { w: self }
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn dram_div2(&mut self) -> DRAM_DIV2_W {
        DRAM_DIV2_W { w: self }
    }
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn dram_div1(&mut self) -> DRAM_DIV1_W {
        DRAM_DIV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DRAM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dram_clk](index.html) module"]
pub struct DRAM_CLK_SPEC;
impl crate::RegisterSpec for DRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dram_clk::R](R) reader structure"]
impl crate::Readable for DRAM_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dram_clk::W](W) writer structure"]
impl crate::Writable for DRAM_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRAM_CLK to value 0"]
impl crate::Resettable for DRAM_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
