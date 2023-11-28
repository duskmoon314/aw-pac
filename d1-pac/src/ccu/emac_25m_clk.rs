#[doc = "Register `emac_25m_clk` reader"]
pub type R = crate::R<EMAC_25M_CLK_SPEC>;
#[doc = "Register `emac_25m_clk` writer"]
pub type W = crate::W<EMAC_25M_CLK_SPEC>;
#[doc = "Field `clk_gating` reader - Gating Special Clock"]
pub type CLK_GATING_R = crate::BitReader<CLK_GATING_A>;
#[doc = "Gating Special Clock\n\nValue on reset: 0"]
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
#[doc = "Field `clk_gating` writer - Gating Special Clock"]
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
#[doc = "Field `clk_src_gating` reader - Gating the Source Clock of Special Clock"]
pub type CLK_SRC_GATING_R = crate::BitReader<CLK_SRC_GATING_A>;
#[doc = "Gating the Source Clock of Special Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_SRC_GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_SRC_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SRC_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_SRC_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_SRC_GATING_A {
        match self.bits {
            false => CLK_SRC_GATING_A::OFF,
            true => CLK_SRC_GATING_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_SRC_GATING_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_SRC_GATING_A::ON
    }
}
#[doc = "Field `clk_src_gating` writer - Gating the Source Clock of Special Clock"]
pub type CLK_SRC_GATING_W<'a, REG> = crate::BitWriter<'a, REG, CLK_SRC_GATING_A>;
impl<'a, REG> CLK_SRC_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_GATING_A::ON)
    }
}
impl R {
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 31 - Gating the Source Clock of Special Clock"]
    #[inline(always)]
    pub fn clk_src_gating(&self) -> CLK_SRC_GATING_R {
        CLK_SRC_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gating(&mut self) -> CLK_GATING_W<EMAC_25M_CLK_SPEC> {
        CLK_GATING_W::new(self, 31)
    }
    #[doc = "Bit 31 - Gating the Source Clock of Special Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_gating(&mut self) -> CLK_SRC_GATING_W<EMAC_25M_CLK_SPEC> {
        CLK_SRC_GATING_W::new(self, 31)
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
#[doc = "EMAC_25M Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_25m_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_25m_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_25M_CLK_SPEC;
impl crate::RegisterSpec for EMAC_25M_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_25m_clk::R`](R) reader structure"]
impl crate::Readable for EMAC_25M_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_25m_clk::W`](W) writer structure"]
impl crate::Writable for EMAC_25M_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_25m_clk to value 0"]
impl crate::Resettable for EMAC_25M_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
