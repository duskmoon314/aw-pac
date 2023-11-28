#[doc = "Register `cpu_axi_cfg` reader"]
pub type R = crate::R<CPU_AXI_CFG_SPEC>;
#[doc = "Register `cpu_axi_cfg` writer"]
pub type W = crate::W<CPU_AXI_CFG_SPEC>;
#[doc = "Field `cpu_div1` reader - Factor M"]
pub type CPU_DIV1_R = crate::FieldReader;
#[doc = "Field `cpu_div1` writer - Factor M"]
pub type CPU_DIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpu_div2` reader - Factor N"]
pub type CPU_DIV2_R = crate::FieldReader;
#[doc = "Field `cpu_div2` writer - Factor N"]
pub type CPU_DIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll_cpu_out_ext_divp` reader - PLL Output External Divider P"]
pub type PLL_CPU_OUT_EXT_DIVP_R = crate::FieldReader<PLL_CPU_OUT_EXT_DIVP_A>;
#[doc = "PLL Output External Divider P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PLL_CPU_OUT_EXT_DIVP_A {
    type Ux = u8;
}
impl PLL_CPU_OUT_EXT_DIVP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL_CPU_OUT_EXT_DIVP_A> {
        match self.bits {
            0 => Some(PLL_CPU_OUT_EXT_DIVP_A::P1),
            1 => Some(PLL_CPU_OUT_EXT_DIVP_A::P2),
            2 => Some(PLL_CPU_OUT_EXT_DIVP_A::P4),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == PLL_CPU_OUT_EXT_DIVP_A::P1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == PLL_CPU_OUT_EXT_DIVP_A::P2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == PLL_CPU_OUT_EXT_DIVP_A::P4
    }
}
#[doc = "Field `pll_cpu_out_ext_divp` writer - PLL Output External Divider P"]
pub type PLL_CPU_OUT_EXT_DIVP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL_CPU_OUT_EXT_DIVP_A>;
impl<'a, REG> PLL_CPU_OUT_EXT_DIVP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL_CPU_OUT_EXT_DIVP_A::P1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL_CPU_OUT_EXT_DIVP_A::P2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL_CPU_OUT_EXT_DIVP_A::P4)
    }
}
#[doc = "Field `cpu_clk_sel` reader - Clock Source Select"]
pub type CPU_CLK_SEL_R = crate::FieldReader<CPU_CLK_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CPU_CLK_SEL_A {
    type Ux = u8;
}
impl CPU_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CPU_CLK_SEL_A> {
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == CPU_CLK_SEL_A::HOSC
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        *self == CPU_CLK_SEL_A::CLK32K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_clk16m_rc(&self) -> bool {
        *self == CPU_CLK_SEL_A::CLK16M_RC
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pll_cpu_p(&self) -> bool {
        *self == CPU_CLK_SEL_A::PLL_CPU_P
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        *self == CPU_CLK_SEL_A::PLL_PERI_1X
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CPU_CLK_SEL_A::PLL_PERI_2X
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        *self == CPU_CLK_SEL_A::PLL_PERI_800M
    }
}
#[doc = "Field `cpu_clk_sel` writer - Clock Source Select"]
pub type CPU_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CPU_CLK_SEL_A>;
impl<'a, REG> CPU_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::CLK32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m_rc(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::CLK16M_RC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_cpu_p(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::PLL_CPU_P)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::PLL_PERI_1X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_CLK_SEL_A::PLL_PERI_800M)
    }
}
impl R {
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn cpu_div1(&self) -> CPU_DIV1_R {
        CPU_DIV1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn cpu_div2(&self) -> CPU_DIV2_R {
        CPU_DIV2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PLL Output External Divider P"]
    #[inline(always)]
    pub fn pll_cpu_out_ext_divp(&self) -> PLL_CPU_OUT_EXT_DIVP_R {
        PLL_CPU_OUT_EXT_DIVP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn cpu_clk_sel(&self) -> CPU_CLK_SEL_R {
        CPU_CLK_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_div1(&mut self) -> CPU_DIV1_W<CPU_AXI_CFG_SPEC> {
        CPU_DIV1_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_div2(&mut self) -> CPU_DIV2_W<CPU_AXI_CFG_SPEC> {
        CPU_DIV2_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - PLL Output External Divider P"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cpu_out_ext_divp(&mut self) -> PLL_CPU_OUT_EXT_DIVP_W<CPU_AXI_CFG_SPEC> {
        PLL_CPU_OUT_EXT_DIVP_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_clk_sel(&mut self) -> CPU_CLK_SEL_W<CPU_AXI_CFG_SPEC> {
        CPU_CLK_SEL_W::new(self, 24)
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
#[doc = "CPU_AXI Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_axi_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_axi_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_AXI_CFG_SPEC;
impl crate::RegisterSpec for CPU_AXI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_axi_cfg::R`](R) reader structure"]
impl crate::Readable for CPU_AXI_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_axi_cfg::W`](W) writer structure"]
impl crate::Writable for CPU_AXI_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_axi_cfg to value 0"]
impl crate::Resettable for CPU_AXI_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
