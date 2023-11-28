#[doc = "Register `clk27m_fan` reader"]
pub type R = crate::R<CLK27M_FAN_SPEC>;
#[doc = "Register `clk27m_fan` writer"]
pub type W = crate::W<CLK27M_FAN_SPEC>;
#[doc = "Field `div0` reader - Factor M"]
pub type DIV0_R = crate::FieldReader;
#[doc = "Field `div0` writer - Factor M"]
pub type DIV0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `div1` reader - Factor N"]
pub type DIV1_R = crate::FieldReader;
#[doc = "Field `div1` writer - Factor N"]
pub type DIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_VIDEO0_1X = 0,
    #[doc = "1: `1`"]
    PLL_VIDEO1_1X = 1,
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
            0 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_1X),
            1 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_1X),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pll_video0_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_1X
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pll_video1_1x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_1X
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK_SRC_SEL_A>;
impl<'a, REG> CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_video0_1x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_1X)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_video1_1x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_1X)
    }
}
#[doc = "Field `gating` reader - Gating for CLK27M"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating for CLK27M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::OFF,
            true => GATING_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == GATING_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == GATING_A::ON
    }
}
#[doc = "Field `gating` writer - Gating for CLK27M"]
pub type GATING_W<'a, REG> = crate::BitWriter<'a, REG, GATING_A>;
impl<'a, REG> GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn div0(&self) -> DIV0_R {
        DIV0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn div1(&self) -> DIV1_R {
        DIV1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - Gating for CLK27M"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn div0(&mut self) -> DIV0_W<CLK27M_FAN_SPEC> {
        DIV0_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn div1(&mut self) -> DIV1_W<CLK27M_FAN_SPEC> {
        DIV1_W::new(self, 8)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<CLK27M_FAN_SPEC> {
        CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = "Bit 31 - Gating for CLK27M"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<CLK27M_FAN_SPEC> {
        GATING_W::new(self, 31)
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
#[doc = "CLK27M FANOUT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk27m_fan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk27m_fan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK27M_FAN_SPEC;
impl crate::RegisterSpec for CLK27M_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk27m_fan::R`](R) reader structure"]
impl crate::Readable for CLK27M_FAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk27m_fan::W`](W) writer structure"]
impl crate::Writable for CLK27M_FAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk27m_fan to value 0"]
impl crate::Resettable for CLK27M_FAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
