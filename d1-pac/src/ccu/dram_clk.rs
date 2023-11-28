#[doc = "Register `dram_clk` reader"]
pub type R = crate::R<DRAM_CLK_SPEC>;
#[doc = "Register `dram_clk` writer"]
pub type W = crate::W<DRAM_CLK_SPEC>;
#[doc = "Field `dram_div1` reader - Factor M"]
pub type DRAM_DIV1_R = crate::FieldReader;
#[doc = "Field `dram_div1` writer - Factor M"]
pub type DRAM_DIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dram_div2` reader - Factor N"]
pub type DRAM_DIV2_R = crate::FieldReader<DRAM_DIV2_A>;
#[doc = "Factor N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for DRAM_DIV2_A {
    type Ux = u8;
}
impl DRAM_DIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRAM_DIV2_A {
        match self.bits {
            0 => DRAM_DIV2_A::N1,
            1 => DRAM_DIV2_A::N2,
            2 => DRAM_DIV2_A::N4,
            3 => DRAM_DIV2_A::N8,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_n1(&self) -> bool {
        *self == DRAM_DIV2_A::N1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == DRAM_DIV2_A::N2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == DRAM_DIV2_A::N4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == DRAM_DIV2_A::N8
    }
}
#[doc = "Field `dram_div2` writer - Factor N"]
pub type DRAM_DIV2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DRAM_DIV2_A>;
impl<'a, REG> DRAM_DIV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn n1(self) -> &'a mut crate::W<REG> {
        self.variant(DRAM_DIV2_A::N1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut crate::W<REG> {
        self.variant(DRAM_DIV2_A::N2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut crate::W<REG> {
        self.variant(DRAM_DIV2_A::N4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut crate::W<REG> {
        self.variant(DRAM_DIV2_A::N8)
    }
}
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CLK_SRC_SEL_A {
    type Ux = u8;
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::PLL_DDR),
            1 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            2 => Some(CLK_SRC_SEL_A::PLL_PERI_2X),
            3 => Some(CLK_SRC_SEL_A::PLL_PERI_800M),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pll_ddr(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_DDR
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_800M
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLK_SRC_SEL_A>;
impl<'a, REG> CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ddr(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_DDR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_800M)
    }
}
#[doc = "Field `sdrclk_upd` reader - SDRCLK Configuration 0 Update"]
pub type SDRCLK_UPD_R = crate::BitReader<SDRCLK_UPD_A>;
#[doc = "SDRCLK Configuration 0 Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SDRCLK_UPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDRCLK_UPD_A {
        match self.bits {
            false => SDRCLK_UPD_A::INVALID,
            true => SDRCLK_UPD_A::VALID,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == SDRCLK_UPD_A::INVALID
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == SDRCLK_UPD_A::VALID
    }
}
#[doc = "Field `sdrclk_upd` writer - SDRCLK Configuration 0 Update"]
pub type SDRCLK_UPD_W<'a, REG> = crate::BitWriter<'a, REG, SDRCLK_UPD_A>;
impl<'a, REG> SDRCLK_UPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(SDRCLK_UPD_A::INVALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(SDRCLK_UPD_A::VALID)
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
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn dram_div1(&self) -> DRAM_DIV1_R {
        DRAM_DIV1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn dram_div2(&self) -> DRAM_DIV2_R {
        DRAM_DIV2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - SDRCLK Configuration 0 Update"]
    #[inline(always)]
    pub fn sdrclk_upd(&self) -> SDRCLK_UPD_R {
        SDRCLK_UPD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn dram_div1(&mut self) -> DRAM_DIV1_W<DRAM_CLK_SPEC> {
        DRAM_DIV1_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn dram_div2(&mut self) -> DRAM_DIV2_W<DRAM_CLK_SPEC> {
        DRAM_DIV2_W::new(self, 8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<DRAM_CLK_SPEC> {
        CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = "Bit 27 - SDRCLK Configuration 0 Update"]
    #[inline(always)]
    #[must_use]
    pub fn sdrclk_upd(&mut self) -> SDRCLK_UPD_W<DRAM_CLK_SPEC> {
        SDRCLK_UPD_W::new(self, 27)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gating(&mut self) -> CLK_GATING_W<DRAM_CLK_SPEC> {
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
#[doc = "DRAM Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dram_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dram_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRAM_CLK_SPEC;
impl crate::RegisterSpec for DRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dram_clk::R`](R) reader structure"]
impl crate::Readable for DRAM_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dram_clk::W`](W) writer structure"]
impl crate::Writable for DRAM_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dram_clk to value 0"]
impl crate::Resettable for DRAM_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
