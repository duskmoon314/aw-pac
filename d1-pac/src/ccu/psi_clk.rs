#[doc = "Register `psi_clk` reader"]
pub type R = crate::R<PSI_CLK_SPEC>;
#[doc = "Register `psi_clk` writer"]
pub type W = crate::W<PSI_CLK_SPEC>;
#[doc = "Field `factor_m` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader;
#[doc = "Field `factor_m` writer - Factor M"]
pub type FACTOR_M_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `factor_n` reader - Factor N"]
pub type FACTOR_N_R = crate::FieldReader<FACTOR_N_A>;
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
impl crate::FieldSpec for FACTOR_N_A {
    type Ux = u8;
}
impl FACTOR_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FACTOR_N_A {
        match self.bits {
            0 => FACTOR_N_A::N1,
            1 => FACTOR_N_A::N2,
            2 => FACTOR_N_A::N4,
            3 => FACTOR_N_A::N8,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_n1(&self) -> bool {
        *self == FACTOR_N_A::N1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == FACTOR_N_A::N2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == FACTOR_N_A::N4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == FACTOR_N_A::N8
    }
}
#[doc = "Field `factor_n` writer - Factor N"]
pub type FACTOR_N_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FACTOR_N_A>;
impl<'a, REG> FACTOR_N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn n1(self) -> &'a mut crate::W<REG> {
        self.variant(FACTOR_N_A::N1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut crate::W<REG> {
        self.variant(FACTOR_N_A::N2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut crate::W<REG> {
        self.variant(FACTOR_N_A::N4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut crate::W<REG> {
        self.variant(FACTOR_N_A::N8)
    }
}
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<CLK_SRC_SEL_A>;
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
impl crate::FieldSpec for CLK_SRC_SEL_A {
    type Ux = u8;
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            0 => CLK_SRC_SEL_A::HOSC,
            1 => CLK_SRC_SEL_A::CLK32K,
            2 => CLK_SRC_SEL_A::CLK16M_RC,
            3 => CLK_SRC_SEL_A::PLL_PERI_1X,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == CLK_SRC_SEL_A::HOSC
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK32K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_clk16m_rc(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK16M_RC
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_1X
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CLK_SRC_SEL_A>;
impl<'a, REG> CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::CLK32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m_rc(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::CLK16M_RC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut crate::W<REG> {
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
    pub fn factor_m(&mut self) -> FACTOR_M_W<PSI_CLK_SPEC> {
        FACTOR_M_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn factor_n(&mut self) -> FACTOR_N_W<PSI_CLK_SPEC> {
        FACTOR_N_W::new(self, 8)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<PSI_CLK_SPEC> {
        CLK_SRC_SEL_W::new(self, 24)
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
#[doc = "PSI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psi_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psi_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSI_CLK_SPEC;
impl crate::RegisterSpec for PSI_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psi_clk::R`](R) reader structure"]
impl crate::Readable for PSI_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psi_clk::W`](W) writer structure"]
impl crate::Writable for PSI_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psi_clk to value 0"]
impl crate::Resettable for PSI_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
