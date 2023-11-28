#[doc = "Register `ccu_clk_mode` reader"]
pub type R = crate::R<CCU_CLK_MODE_SPEC>;
#[doc = "Register `ccu_clk_mode` writer"]
pub type W = crate::W<CCU_CLK_MODE_SPEC>;
#[doc = "Field `ccu_clk_gating_disable` reader - "]
pub type CCU_CLK_GATING_DISABLE_R = crate::BitReader<CCU_CLK_GATING_DISABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU_CLK_GATING_DISABLE_A {
    #[doc = "0: CCU Clock Gating Registers(0x0004~0x0010) effect"]
    EFFECT = 0,
    #[doc = "1: CCU Clock Gating Registers(0x0004~0x0010) not effect"]
    NOT_EFFECT = 1,
}
impl From<CCU_CLK_GATING_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CCU_CLK_GATING_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU_CLK_GATING_DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU_CLK_GATING_DISABLE_A {
        match self.bits {
            false => CCU_CLK_GATING_DISABLE_A::EFFECT,
            true => CCU_CLK_GATING_DISABLE_A::NOT_EFFECT,
        }
    }
    #[doc = "CCU Clock Gating Registers(0x0004~0x0010) effect"]
    #[inline(always)]
    pub fn is_effect(&self) -> bool {
        *self == CCU_CLK_GATING_DISABLE_A::EFFECT
    }
    #[doc = "CCU Clock Gating Registers(0x0004~0x0010) not effect"]
    #[inline(always)]
    pub fn is_not_effect(&self) -> bool {
        *self == CCU_CLK_GATING_DISABLE_A::NOT_EFFECT
    }
}
#[doc = "Field `ccu_clk_gating_disable` writer - "]
pub type CCU_CLK_GATING_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG, CCU_CLK_GATING_DISABLE_A>;
impl<'a, REG> CCU_CLK_GATING_DISABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCU Clock Gating Registers(0x0004~0x0010) effect"]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(CCU_CLK_GATING_DISABLE_A::EFFECT)
    }
    #[doc = "CCU Clock Gating Registers(0x0004~0x0010) not effect"]
    #[inline(always)]
    pub fn not_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CCU_CLK_GATING_DISABLE_A::NOT_EFFECT)
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ccu_clk_gating_disable(&self) -> CCU_CLK_GATING_DISABLE_R {
        CCU_CLK_GATING_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ccu_clk_gating_disable(&mut self) -> CCU_CLK_GATING_DISABLE_W<CCU_CLK_MODE_SPEC> {
        CCU_CLK_GATING_DISABLE_W::new(self, 31)
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
#[doc = "CCU Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_clk_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_clk_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCU_CLK_MODE_SPEC;
impl crate::RegisterSpec for CCU_CLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccu_clk_mode::R`](R) reader structure"]
impl crate::Readable for CCU_CLK_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccu_clk_mode::W`](W) writer structure"]
impl crate::Writable for CCU_CLK_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_clk_mode to value 0"]
impl crate::Resettable for CCU_CLK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
