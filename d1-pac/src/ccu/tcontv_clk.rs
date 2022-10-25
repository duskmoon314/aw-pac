#[doc = "Register `tcontv_clk` reader"]
pub struct R(crate::R<TCONTV_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCONTV_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCONTV_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCONTV_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tcontv_clk` writer"]
pub struct W(crate::W<TCONTV_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCONTV_CLK_SPEC>;
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
impl From<crate::W<TCONTV_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCONTV_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `factor_m` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `factor_m` writer - Factor M"]
pub type FACTOR_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCONTV_CLK_SPEC, u8, u8, 4, O>;
#[doc = "Field `factor_n` reader - Factor N"]
pub type FACTOR_N_R = crate::FieldReader<u8, FACTOR_N_A>;
#[doc = "Factor N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FACTOR_N_A {
    #[doc = "0: `0`"]
    N1 = 0,
    #[doc = "1: `1`"]
    N2 = 1,
    #[doc = "2: `10`"]
    N4 = 2,
    #[doc = "3: `11`"]
    N8 = 3,
}
impl From<FACTOR_N_A> for u8 {
    #[inline(always)]
    fn from(variant: FACTOR_N_A) -> Self {
        variant as _
    }
}
impl FACTOR_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACTOR_N_A {
        match self.bits {
            0 => FACTOR_N_A::N1,
            1 => FACTOR_N_A::N2,
            2 => FACTOR_N_A::N4,
            3 => FACTOR_N_A::N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `N1`"]
    #[inline(always)]
    pub fn is_n1(&self) -> bool {
        *self == FACTOR_N_A::N1
    }
    #[doc = "Checks if the value of the field is `N2`"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == FACTOR_N_A::N2
    }
    #[doc = "Checks if the value of the field is `N4`"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == FACTOR_N_A::N4
    }
    #[doc = "Checks if the value of the field is `N8`"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == FACTOR_N_A::N8
    }
}
#[doc = "Field `factor_n` writer - Factor N"]
pub type FACTOR_N_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TCONTV_CLK_SPEC, u8, FACTOR_N_A, 2, O>;
impl<'a, const O: u8> FACTOR_N_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn n1(self) -> &'a mut W {
        self.variant(FACTOR_N_A::N1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut W {
        self.variant(FACTOR_N_A::N2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut W {
        self.variant(FACTOR_N_A::N4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut W {
        self.variant(FACTOR_N_A::N8)
    }
}
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_VIDEO0_1X = 0,
    #[doc = "1: `1`"]
    PLL_VIDEO0_4X = 1,
    #[doc = "2: `10`"]
    PLL_VIDEO1_1X = 2,
    #[doc = "3: `11`"]
    PLL_VIDEO1_4X = 3,
    #[doc = "4: `100`"]
    PLL_PERI_2X = 4,
    #[doc = "5: `101`"]
    PLL_AUDIO1_DIV2 = 5,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_1X),
            1 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_4X),
            2 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_1X),
            3 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_4X),
            4 => Some(CLK_SRC_SEL_A::PLL_PERI_2X),
            5 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_1X`"]
    #[inline(always)]
    pub fn is_pll_video0_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_1X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_4X`"]
    #[inline(always)]
    pub fn is_pll_video0_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_4X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_1X`"]
    #[inline(always)]
    pub fn is_pll_video1_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_1X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_4X`"]
    #[inline(always)]
    pub fn is_pll_video1_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_4X
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1_DIV2`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TCONTV_CLK_SPEC, u8, CLK_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_video0_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_1X)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_video0_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_4X)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_video1_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_1X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_video1_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_4X)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
}
#[doc = "Field `clk_gating` reader - Gating Clock"]
pub type CLK_GATING_R = crate::BitReader<CLK_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CLK_GATING_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CLK_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_GATING_A::ON
    }
}
#[doc = "Field `clk_gating` writer - Gating Clock"]
pub type CLK_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TCONTV_CLK_SPEC, CLK_GATING_A, O>;
impl<'a, const O: u8> CLK_GATING_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:3 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&self) -> FACTOR_M_R {
        FACTOR_M_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn factor_n(&self) -> FACTOR_N_R {
        FACTOR_N_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn factor_m(&mut self) -> FACTOR_M_W<0> {
        FACTOR_M_W::new(self)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn factor_n(&mut self) -> FACTOR_N_W<8> {
        FACTOR_N_W::new(self)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<24> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gating(&mut self) -> CLK_GATING_W<31> {
        CLK_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCONTV Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcontv_clk](index.html) module"]
pub struct TCONTV_CLK_SPEC;
impl crate::RegisterSpec for TCONTV_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcontv_clk::R](R) reader structure"]
impl crate::Readable for TCONTV_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcontv_clk::W](W) writer structure"]
impl crate::Writable for TCONTV_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tcontv_clk to value 0"]
impl crate::Resettable for TCONTV_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
