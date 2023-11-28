#[doc = "Register `ve_clk` reader"]
pub type R = crate::R<VE_CLK_SPEC>;
#[doc = "Register `ve_clk` writer"]
pub type W = crate::W<VE_CLK_SPEC>;
#[doc = "Field `factor_m` reader - Factor M"]
pub type FACTOR_M_R = crate::FieldReader;
#[doc = "Field `factor_m` writer - Factor M"]
pub type FACTOR_M_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::BitReader<CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    VEPLL = 0,
    #[doc = "1: `1`"]
    PLL_PERI_2X = 1,
}
impl From<CLK_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            false => CLK_SRC_SEL_A::VEPLL,
            true => CLK_SRC_SEL_A::PLL_PERI_2X,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_vepll(&self) -> bool {
        *self == CLK_SRC_SEL_A::VEPLL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG, CLK_SRC_SEL_A>;
impl<'a, REG> CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn vepll(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::VEPLL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
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
    pub const fn variant(&self) -> CLK_GATING_A {
        match self.bits {
            false => CLK_GATING_A::OFF,
            true => CLK_GATING_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_GATING_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_GATING_A::ON
    }
}
#[doc = "Field `clk_gating` writer - Gating Clock"]
pub type CLK_GATING_W<'a, REG> = crate::BitWriter<'a, REG, CLK_GATING_A>;
impl<'a, REG> CLK_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_GATING_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&self) -> FACTOR_M_R {
        FACTOR_M_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn factor_m(&mut self) -> FACTOR_M_W<VE_CLK_SPEC> {
        FACTOR_M_W::new(self, 0)
    }
    #[doc = "Bit 24 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<VE_CLK_SPEC> {
        CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gating(&mut self) -> CLK_GATING_W<VE_CLK_SPEC> {
        CLK_GATING_W::new(self, 31)
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
#[doc = "VE Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ve_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ve_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VE_CLK_SPEC;
impl crate::RegisterSpec for VE_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ve_clk::R`](R) reader structure"]
impl crate::Readable for VE_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ve_clk::W`](W) writer structure"]
impl crate::Writable for VE_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ve_clk to value 0"]
impl crate::Resettable for VE_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
