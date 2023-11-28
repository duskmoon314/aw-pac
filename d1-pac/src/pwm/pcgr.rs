#[doc = "Register `pcgr` reader"]
pub type R = crate::R<PCGR_SPEC>;
#[doc = "Register `pcgr` writer"]
pub type W = crate::W<PCGR_SPEC>;
#[doc = "Field `pwm_clk_gating[0-7]` reader - Gating clock for PWM"]
pub type PWM_CLK_GATING_R = crate::BitReader<PWM_CLK_GATING_A>;
#[doc = "Gating clock for PWM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_CLK_GATING_A {
    #[doc = "0: Mask"]
    MASK = 0,
    #[doc = "1: Pass"]
    PASS = 1,
}
impl From<PWM_CLK_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_CLK_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_CLK_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWM_CLK_GATING_A {
        match self.bits {
            false => PWM_CLK_GATING_A::MASK,
            true => PWM_CLK_GATING_A::PASS,
        }
    }
    #[doc = "Mask"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == PWM_CLK_GATING_A::MASK
    }
    #[doc = "Pass"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == PWM_CLK_GATING_A::PASS
    }
}
#[doc = "Field `pwm_clk_gating[0-7]` writer - Gating clock for PWM"]
pub type PWM_CLK_GATING_W<'a, REG> = crate::BitWriter<'a, REG, PWM_CLK_GATING_A>;
impl<'a, REG> PWM_CLK_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_CLK_GATING_A::MASK)
    }
    #[doc = "Pass"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_CLK_GATING_A::PASS)
    }
}
#[doc = "Field `pwm_clk_bypass[0-7]` reader - Bypass clock source (after pre-scale) to PWM output"]
pub type PWM_CLK_BYPASS_R = crate::BitReader<PWM_CLK_BYPASS_A>;
#[doc = "Bypass clock source (after pre-scale) to PWM output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_CLK_BYPASS_A {
    #[doc = "0: not bypass"]
    NOT_BYPASS = 0,
    #[doc = "1: bypass"]
    BYPASS = 1,
}
impl From<PWM_CLK_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_CLK_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_CLK_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWM_CLK_BYPASS_A {
        match self.bits {
            false => PWM_CLK_BYPASS_A::NOT_BYPASS,
            true => PWM_CLK_BYPASS_A::BYPASS,
        }
    }
    #[doc = "not bypass"]
    #[inline(always)]
    pub fn is_not_bypass(&self) -> bool {
        *self == PWM_CLK_BYPASS_A::NOT_BYPASS
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == PWM_CLK_BYPASS_A::BYPASS
    }
}
#[doc = "Field `pwm_clk_bypass[0-7]` writer - Bypass clock source (after pre-scale) to PWM output"]
pub type PWM_CLK_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, PWM_CLK_BYPASS_A>;
impl<'a, REG> PWM_CLK_BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not bypass"]
    #[inline(always)]
    pub fn not_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_CLK_BYPASS_A::NOT_BYPASS)
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_CLK_BYPASS_A::BYPASS)
    }
}
impl R {
    #[doc = "Gating clock for PWM\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pwm0_clk_gating` field"]
    #[inline(always)]
    pub fn pwm_clk_gating(&self, n: u8) -> PWM_CLK_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PWM_CLK_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm0_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm1_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm2_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm3_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm4_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm5_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm6_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gating clock for PWM"]
    #[inline(always)]
    pub fn pwm7_clk_gating(&self) -> PWM_CLK_GATING_R {
        PWM_CLK_GATING_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bypass clock source (after pre-scale) to PWM output\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pwm0_clk_bypass` field"]
    #[inline(always)]
    pub fn pwm_clk_bypass(&self, n: u8) -> PWM_CLK_BYPASS_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PWM_CLK_BYPASS_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm0_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm1_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm2_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm3_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm4_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm5_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm6_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    pub fn pwm7_clk_bypass(&self) -> PWM_CLK_BYPASS_R {
        PWM_CLK_BYPASS_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating clock for PWM\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pwm0_clk_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_clk_gating(&mut self, n: u8) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PWM_CLK_GATING_W::new(self, n)
    }
    #[doc = "Bit 0 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 1)
    }
    #[doc = "Bit 2 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 2)
    }
    #[doc = "Bit 3 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 3)
    }
    #[doc = "Bit 4 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm4_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 4)
    }
    #[doc = "Bit 5 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm5_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 5)
    }
    #[doc = "Bit 6 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm6_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 6)
    }
    #[doc = "Bit 7 - Gating clock for PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pwm7_clk_gating(&mut self) -> PWM_CLK_GATING_W<PCGR_SPEC> {
        PWM_CLK_GATING_W::new(self, 7)
    }
    #[doc = "Bypass clock source (after pre-scale) to PWM output\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pwm0_clk_bypass` field"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_clk_bypass(&mut self, n: u8) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PWM_CLK_BYPASS_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 19)
    }
    #[doc = "Bit 20 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm4_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 20)
    }
    #[doc = "Bit 21 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm5_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 21)
    }
    #[doc = "Bit 22 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm6_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 22)
    }
    #[doc = "Bit 23 - Bypass clock source (after pre-scale) to PWM output"]
    #[inline(always)]
    #[must_use]
    pub fn pwm7_clk_bypass(&mut self) -> PWM_CLK_BYPASS_W<PCGR_SPEC> {
        PWM_CLK_BYPASS_W::new(self, 23)
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
#[doc = "PWM Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGR_SPEC;
impl crate::RegisterSpec for PCGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgr::R`](R) reader structure"]
impl crate::Readable for PCGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgr::W`](W) writer structure"]
impl crate::Writable for PCGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pcgr to value 0"]
impl crate::Resettable for PCGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
