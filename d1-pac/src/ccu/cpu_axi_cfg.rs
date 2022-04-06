#[doc = "Register `CPU_AXI_CFG` reader"]
pub struct R(crate::R<CPU_AXI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_AXI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_AXI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_AXI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_AXI_CFG` writer"]
pub struct W(crate::W<CPU_AXI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_AXI_CFG_SPEC>;
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
impl From<crate::W<CPU_AXI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_AXI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU_CLK_SEL_A {
    #[doc = "0: `0`"]
    HOSC = 0,
    #[doc = "1: `1`"]
    CLK32K = 1,
    #[doc = "2: `10`"]
    CLK16M_RC = 2,
    #[doc = "3: `11`"]
    PLL_CPU_P = 3,
    #[doc = "4: `100`"]
    PLL_PERI_1X = 4,
    #[doc = "5: `101`"]
    PLL_PERI_2X = 5,
    #[doc = "6: `110`"]
    PLL_PERI_800M = 6,
}
impl From<CPU_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPU_CLK_SEL` reader - Clock Source Select"]
pub struct CPU_CLK_SEL_R(crate::FieldReader<u8, CPU_CLK_SEL_A>);
impl CPU_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPU_CLK_SEL_A> {
        match self.bits {
            0 => Some(CPU_CLK_SEL_A::HOSC),
            1 => Some(CPU_CLK_SEL_A::CLK32K),
            2 => Some(CPU_CLK_SEL_A::CLK16M_RC),
            3 => Some(CPU_CLK_SEL_A::PLL_CPU_P),
            4 => Some(CPU_CLK_SEL_A::PLL_PERI_1X),
            5 => Some(CPU_CLK_SEL_A::PLL_PERI_2X),
            6 => Some(CPU_CLK_SEL_A::PLL_PERI_800M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HOSC`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        **self == CPU_CLK_SEL_A::HOSC
    }
    #[doc = "Checks if the value of the field is `CLK32K`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        **self == CPU_CLK_SEL_A::CLK32K
    }
    #[doc = "Checks if the value of the field is `CLK16M_RC`"]
    #[inline(always)]
    pub fn is_clk16m_rc(&self) -> bool {
        **self == CPU_CLK_SEL_A::CLK16M_RC
    }
    #[doc = "Checks if the value of the field is `PLL_CPU_P`"]
    #[inline(always)]
    pub fn is_pll_cpu_p(&self) -> bool {
        **self == CPU_CLK_SEL_A::PLL_CPU_P
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_1X`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        **self == CPU_CLK_SEL_A::PLL_PERI_1X
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        **self == CPU_CLK_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_800M`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        **self == CPU_CLK_SEL_A::PLL_PERI_800M
    }
}
impl core::ops::Deref for CPU_CLK_SEL_R {
    type Target = crate::FieldReader<u8, CPU_CLK_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_CLK_SEL` writer - Clock Source Select"]
pub struct CPU_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU_CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::CLK32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m_rc(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::CLK16M_RC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_cpu_p(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::PLL_CPU_P)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::PLL_PERI_1X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut W {
        self.variant(CPU_CLK_SEL_A::PLL_PERI_800M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "PLL Output External Divider P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_CPU_OUT_EXT_DIVP_A {
    #[doc = "0: `0`"]
    P1 = 0,
    #[doc = "1: `1`"]
    P2 = 1,
    #[doc = "2: `10`"]
    P4 = 2,
}
impl From<PLL_CPU_OUT_EXT_DIVP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_CPU_OUT_EXT_DIVP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLL_CPU_OUT_EXT_DIVP` reader - PLL Output External Divider P"]
pub struct PLL_CPU_OUT_EXT_DIVP_R(crate::FieldReader<u8, PLL_CPU_OUT_EXT_DIVP_A>);
impl PLL_CPU_OUT_EXT_DIVP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CPU_OUT_EXT_DIVP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLL_CPU_OUT_EXT_DIVP_A> {
        match self.bits {
            0 => Some(PLL_CPU_OUT_EXT_DIVP_A::P1),
            1 => Some(PLL_CPU_OUT_EXT_DIVP_A::P2),
            2 => Some(PLL_CPU_OUT_EXT_DIVP_A::P4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        **self == PLL_CPU_OUT_EXT_DIVP_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        **self == PLL_CPU_OUT_EXT_DIVP_A::P2
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        **self == PLL_CPU_OUT_EXT_DIVP_A::P4
    }
}
impl core::ops::Deref for PLL_CPU_OUT_EXT_DIVP_R {
    type Target = crate::FieldReader<u8, PLL_CPU_OUT_EXT_DIVP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CPU_OUT_EXT_DIVP` writer - PLL Output External Divider P"]
pub struct PLL_CPU_OUT_EXT_DIVP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CPU_OUT_EXT_DIVP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CPU_OUT_EXT_DIVP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(PLL_CPU_OUT_EXT_DIVP_A::P1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(PLL_CPU_OUT_EXT_DIVP_A::P2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(PLL_CPU_OUT_EXT_DIVP_A::P4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `CPU_DIV2` reader - Factor N"]
pub struct CPU_DIV2_R(crate::FieldReader<u8, u8>);
impl CPU_DIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_DIV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_DIV2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_DIV2` writer - Factor N"]
pub struct CPU_DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_DIV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `CPU_DIV1` reader - Factor M"]
pub struct CPU_DIV1_R(crate::FieldReader<u8, u8>);
impl CPU_DIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_DIV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_DIV1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_DIV1` writer - Factor M"]
pub struct CPU_DIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_DIV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn cpu_clk_sel(&self) -> CPU_CLK_SEL_R {
        CPU_CLK_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 16:17 - PLL Output External Divider P"]
    #[inline(always)]
    pub fn pll_cpu_out_ext_divp(&self) -> PLL_CPU_OUT_EXT_DIVP_R {
        PLL_CPU_OUT_EXT_DIVP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn cpu_div2(&self) -> CPU_DIV2_R {
        CPU_DIV2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn cpu_div1(&self) -> CPU_DIV1_R {
        CPU_DIV1_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn cpu_clk_sel(&mut self) -> CPU_CLK_SEL_W {
        CPU_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - PLL Output External Divider P"]
    #[inline(always)]
    pub fn pll_cpu_out_ext_divp(&mut self) -> PLL_CPU_OUT_EXT_DIVP_W {
        PLL_CPU_OUT_EXT_DIVP_W { w: self }
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn cpu_div2(&mut self) -> CPU_DIV2_W {
        CPU_DIV2_W { w: self }
    }
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn cpu_div1(&mut self) -> CPU_DIV1_W {
        CPU_DIV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU_AXI Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_axi_cfg](index.html) module"]
pub struct CPU_AXI_CFG_SPEC;
impl crate::RegisterSpec for CPU_AXI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_axi_cfg::R](R) reader structure"]
impl crate::Readable for CPU_AXI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_axi_cfg::W](W) writer structure"]
impl crate::Writable for CPU_AXI_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_AXI_CFG to value 0"]
impl crate::Resettable for CPU_AXI_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
