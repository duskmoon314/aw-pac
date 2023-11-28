#[doc = "Register `ccu_fan_gate` reader"]
pub type R = crate::R<CCU_FAN_GATE_SPEC>;
#[doc = "Register `ccu_fan_gate` writer"]
pub type W = crate::W<CCU_FAN_GATE_SPEC>;
#[doc = "Field `clk24m_en` reader - Gating for CLK24M"]
pub type CLK24M_EN_R = crate::BitReader<CLK24M_EN_A>;
#[doc = "Gating for CLK24M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK24M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK24M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK24M_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK24M_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK24M_EN_A {
        match self.bits {
            false => CLK24M_EN_A::OFF,
            true => CLK24M_EN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK24M_EN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK24M_EN_A::ON
    }
}
#[doc = "Field `clk24m_en` writer - Gating for CLK24M"]
pub type CLK24M_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK24M_EN_A>;
impl<'a, REG> CLK24M_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK24M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK24M_EN_A::ON)
    }
}
#[doc = "Field `clk12m_en` reader - Gating for CLK12M"]
pub type CLK12M_EN_R = crate::BitReader<CLK12M_EN_A>;
#[doc = "Gating for CLK12M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK12M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK12M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK12M_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK12M_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK12M_EN_A {
        match self.bits {
            false => CLK12M_EN_A::OFF,
            true => CLK12M_EN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK12M_EN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK12M_EN_A::ON
    }
}
#[doc = "Field `clk12m_en` writer - Gating for CLK12M"]
pub type CLK12M_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK12M_EN_A>;
impl<'a, REG> CLK12M_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK12M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK12M_EN_A::ON)
    }
}
#[doc = "Field `clk16m_en` reader - Gating for CLK16M"]
pub type CLK16M_EN_R = crate::BitReader<CLK16M_EN_A>;
#[doc = "Gating for CLK16M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK16M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK16M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK16M_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK16M_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK16M_EN_A {
        match self.bits {
            false => CLK16M_EN_A::OFF,
            true => CLK16M_EN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK16M_EN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK16M_EN_A::ON
    }
}
#[doc = "Field `clk16m_en` writer - Gating for CLK16M"]
pub type CLK16M_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK16M_EN_A>;
impl<'a, REG> CLK16M_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK16M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK16M_EN_A::ON)
    }
}
#[doc = "Field `clk25m_en` reader - Gating for CLK25M"]
pub type CLK25M_EN_R = crate::BitReader<CLK25M_EN_A>;
#[doc = "Gating for CLK25M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK25M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK25M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK25M_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK25M_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK25M_EN_A {
        match self.bits {
            false => CLK25M_EN_A::OFF,
            true => CLK25M_EN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK25M_EN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK25M_EN_A::ON
    }
}
#[doc = "Field `clk25m_en` writer - Gating for CLK25M"]
pub type CLK25M_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK25M_EN_A>;
impl<'a, REG> CLK25M_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK25M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK25M_EN_A::ON)
    }
}
#[doc = "Field `clk32k_en` reader - Gating for CLK32K"]
pub type CLK32K_EN_R = crate::BitReader<CLK32K_EN_A>;
#[doc = "Gating for CLK32K\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK32K_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK32K_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK32K_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK32K_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK32K_EN_A {
        match self.bits {
            false => CLK32K_EN_A::OFF,
            true => CLK32K_EN_A::ON,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK32K_EN_A::OFF
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK32K_EN_A::ON
    }
}
#[doc = "Field `clk32k_en` writer - Gating for CLK32K"]
pub type CLK32K_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK32K_EN_A>;
impl<'a, REG> CLK32K_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLK32K_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CLK32K_EN_A::ON)
    }
}
impl R {
    #[doc = "Bit 0 - Gating for CLK24M"]
    #[inline(always)]
    pub fn clk24m_en(&self) -> CLK24M_EN_R {
        CLK24M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating for CLK12M"]
    #[inline(always)]
    pub fn clk12m_en(&self) -> CLK12M_EN_R {
        CLK12M_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating for CLK16M"]
    #[inline(always)]
    pub fn clk16m_en(&self) -> CLK16M_EN_R {
        CLK16M_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating for CLK25M"]
    #[inline(always)]
    pub fn clk25m_en(&self) -> CLK25M_EN_R {
        CLK25M_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Gating for CLK32K"]
    #[inline(always)]
    pub fn clk32k_en(&self) -> CLK32K_EN_R {
        CLK32K_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating for CLK24M"]
    #[inline(always)]
    #[must_use]
    pub fn clk24m_en(&mut self) -> CLK24M_EN_W<CCU_FAN_GATE_SPEC> {
        CLK24M_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating for CLK12M"]
    #[inline(always)]
    #[must_use]
    pub fn clk12m_en(&mut self) -> CLK12M_EN_W<CCU_FAN_GATE_SPEC> {
        CLK12M_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Gating for CLK16M"]
    #[inline(always)]
    #[must_use]
    pub fn clk16m_en(&mut self) -> CLK16M_EN_W<CCU_FAN_GATE_SPEC> {
        CLK16M_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Gating for CLK25M"]
    #[inline(always)]
    #[must_use]
    pub fn clk25m_en(&mut self) -> CLK25M_EN_W<CCU_FAN_GATE_SPEC> {
        CLK25M_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Gating for CLK32K"]
    #[inline(always)]
    #[must_use]
    pub fn clk32k_en(&mut self) -> CLK32K_EN_W<CCU_FAN_GATE_SPEC> {
        CLK32K_EN_W::new(self, 4)
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
#[doc = "CCU FANOUT CLOCK GATE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_fan_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_fan_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCU_FAN_GATE_SPEC;
impl crate::RegisterSpec for CCU_FAN_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccu_fan_gate::R`](R) reader structure"]
impl crate::Readable for CCU_FAN_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccu_fan_gate::W`](W) writer structure"]
impl crate::Writable for CCU_FAN_GATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_fan_gate to value 0"]
impl crate::Resettable for CCU_FAN_GATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
