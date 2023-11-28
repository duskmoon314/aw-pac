#[doc = "Register `ccu_fan` reader"]
pub type R = crate::R<CCU_FAN_SPEC>;
#[doc = "Register `ccu_fan` writer"]
pub type W = crate::W<CCU_FAN_SPEC>;
#[doc = "Field `clk_fanout_sel[0-2]` reader - "]
pub type CLK_FANOUT_SEL_R = crate::FieldReader<CLK_FANOUT_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_FANOUT_SEL_A {
    #[doc = "0: `0`"]
    CLK32K = 0,
    #[doc = "1: `1`"]
    CLK12M = 1,
    #[doc = "2: `10`"]
    CLK16M = 2,
    #[doc = "3: `11`"]
    CLK24M = 3,
    #[doc = "4: `100`"]
    CLK25M = 4,
    #[doc = "5: `101`"]
    CLK27M = 5,
    #[doc = "6: `110`"]
    PCLK = 6,
}
impl From<CLK_FANOUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_FANOUT_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_FANOUT_SEL_A {
    type Ux = u8;
}
impl CLK_FANOUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK_FANOUT_SEL_A> {
        match self.bits {
            0 => Some(CLK_FANOUT_SEL_A::CLK32K),
            1 => Some(CLK_FANOUT_SEL_A::CLK12M),
            2 => Some(CLK_FANOUT_SEL_A::CLK16M),
            3 => Some(CLK_FANOUT_SEL_A::CLK24M),
            4 => Some(CLK_FANOUT_SEL_A::CLK25M),
            5 => Some(CLK_FANOUT_SEL_A::CLK27M),
            6 => Some(CLK_FANOUT_SEL_A::PCLK),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::CLK32K
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clk12m(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::CLK12M
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_clk16m(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::CLK16M
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_clk24m(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::CLK24M
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_clk25m(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::CLK25M
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_clk27m(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::CLK27M
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CLK_FANOUT_SEL_A::PCLK
    }
}
#[doc = "Field `clk_fanout_sel[0-2]` writer - "]
pub type CLK_FANOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLK_FANOUT_SEL_A>;
impl<'a, REG> CLK_FANOUT_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::CLK32K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk12m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::CLK12M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::CLK16M)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn clk24m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::CLK24M)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clk25m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::CLK25M)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk27m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::CLK27M)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_SEL_A::PCLK)
    }
}
#[doc = "Field `clk_fanout_en[0-2]` reader - Gating for CLK_FANOUT"]
pub type CLK_FANOUT_EN_R = crate::BitReader<CLK_FANOUT_EN_A>;
#[doc = "Gating for CLK_FANOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_FANOUT_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_FANOUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_FANOUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_FANOUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_FANOUT_EN_A {
        match self.bits {
            false => CLK_FANOUT_EN_A::OFF,
            true => CLK_FANOUT_EN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_FANOUT_EN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_FANOUT_EN_A::ON
    }
}
#[doc = "Field `clk_fanout_en[0-2]` writer - Gating for CLK_FANOUT"]
pub type CLK_FANOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK_FANOUT_EN_A>;
impl<'a, REG> CLK_FANOUT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_FANOUT_EN_A::ON)
    }
}
impl R {
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `clk_fanout0_sel` field"]
    #[inline(always)]
    pub fn clk_fanout_sel(&self, n: u8) -> CLK_FANOUT_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CLK_FANOUT_SEL_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Bits 0:2 - clk_fanout0_sel"]
    #[inline(always)]
    pub fn clk_fanout0_sel(&self) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - clk_fanout1_sel"]
    #[inline(always)]
    pub fn clk_fanout1_sel(&self) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - clk_fanout2_sel"]
    #[inline(always)]
    pub fn clk_fanout2_sel(&self) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Gating for CLK_FANOUT\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `clk_fanout0_en` field"]
    #[inline(always)]
    pub fn clk_fanout_en(&self, n: u8) -> CLK_FANOUT_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CLK_FANOUT_EN_R::new(((self.bits >> (n + 21)) & 1) != 0)
    }
    #[doc = "Bit 21 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout0_en(&self) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout1_en(&self) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout2_en(&self) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `clk_fanout0_sel` field"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout_sel(&mut self, n: u8) -> CLK_FANOUT_SEL_W<CCU_FAN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CLK_FANOUT_SEL_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - clk_fanout0_sel"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout0_sel(&mut self) -> CLK_FANOUT_SEL_W<CCU_FAN_SPEC> {
        CLK_FANOUT_SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - clk_fanout1_sel"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout1_sel(&mut self) -> CLK_FANOUT_SEL_W<CCU_FAN_SPEC> {
        CLK_FANOUT_SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - clk_fanout2_sel"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout2_sel(&mut self) -> CLK_FANOUT_SEL_W<CCU_FAN_SPEC> {
        CLK_FANOUT_SEL_W::new(self, 6)
    }
    #[doc = "Gating for CLK_FANOUT\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `clk_fanout0_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout_en(&mut self, n: u8) -> CLK_FANOUT_EN_W<CCU_FAN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CLK_FANOUT_EN_W::new(self, n + 21)
    }
    #[doc = "Bit 21 - Gating for CLK_FANOUT"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout0_en(&mut self) -> CLK_FANOUT_EN_W<CCU_FAN_SPEC> {
        CLK_FANOUT_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Gating for CLK_FANOUT"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout1_en(&mut self) -> CLK_FANOUT_EN_W<CCU_FAN_SPEC> {
        CLK_FANOUT_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Gating for CLK_FANOUT"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fanout2_en(&mut self) -> CLK_FANOUT_EN_W<CCU_FAN_SPEC> {
        CLK_FANOUT_EN_W::new(self, 23)
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
#[doc = "CCU FANOUT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_fan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_fan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCU_FAN_SPEC;
impl crate::RegisterSpec for CCU_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccu_fan::R`](R) reader structure"]
impl crate::Readable for CCU_FAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccu_fan::W`](W) writer structure"]
impl crate::Writable for CCU_FAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_fan to value 0"]
impl crate::Resettable for CCU_FAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
