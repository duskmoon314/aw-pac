#[doc = "Register `psi_clk` reader"]
pub struct R(crate::R<PSI_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSI_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSI_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSI_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psi_clk` writer"]
pub struct W(crate::W<PSI_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSI_CLK_SPEC>;
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
impl From<crate::W<PSI_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSI_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `factor_m` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `factor_m` writer - Factor M"]
pub type FACTOR_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSI_CLK_SPEC, u8, u8, 2, O>;
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
    crate::FieldWriterSafe<'a, u32, PSI_CLK_SPEC, u8, FACTOR_N_A, 2, O>;
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
    HOSC = 0,
    #[doc = "1: `1`"]
    CLK32K = 1,
    #[doc = "2: `10`"]
    CLK16M_RC = 2,
    #[doc = "3: `11`"]
    PLL_PERI_1X = 3,
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
    pub fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            0 => CLK_SRC_SEL_A::HOSC,
            1 => CLK_SRC_SEL_A::CLK32K,
            2 => CLK_SRC_SEL_A::CLK16M_RC,
            3 => CLK_SRC_SEL_A::PLL_PERI_1X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HOSC`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == CLK_SRC_SEL_A::HOSC
    }
    #[doc = "Checks if the value of the field is `CLK32K`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK32K
    }
    #[doc = "Checks if the value of the field is `CLK16M_RC`"]
    #[inline(always)]
    pub fn is_clk16m_rc(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK16M_RC
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_1X`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_1X
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PSI_CLK_SPEC, u8, CLK_SRC_SEL_A, 2, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m_rc(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK16M_RC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_1X)
    }
}
impl R {
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&self) -> FACTOR_M_R {
        FACTOR_M_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn factor_n(&self) -> FACTOR_N_R {
        FACTOR_N_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Factor M"]
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
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<24> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PSI Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psi_clk](index.html) module"]
pub struct PSI_CLK_SPEC;
impl crate::RegisterSpec for PSI_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psi_clk::R](R) reader structure"]
impl crate::Readable for PSI_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psi_clk::W](W) writer structure"]
impl crate::Writable for PSI_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psi_clk to value 0"]
impl crate::Resettable for PSI_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
