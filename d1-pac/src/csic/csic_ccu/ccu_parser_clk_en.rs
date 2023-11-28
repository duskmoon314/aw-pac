#[doc = "Register `ccu_parser_clk_en` reader"]
pub type R = crate::R<CCU_PARSER_CLK_EN_SPEC>;
#[doc = "Register `ccu_parser_clk_en` writer"]
pub type W = crate::W<CCU_PARSER_CLK_EN_SPEC>;
#[doc = "Field `mcsi_parser0_clk_enable` reader - "]
pub type MCSI_PARSER0_CLK_ENABLE_R = crate::BitReader<MCSI_PARSER0_CLK_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCSI_PARSER0_CLK_ENABLE_A {
    #[doc = "0: CSI Parser0 clock disable"]
    DISABLE = 0,
    #[doc = "1: CSI Parser0 clock enable"]
    ENABLE = 1,
}
impl From<MCSI_PARSER0_CLK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MCSI_PARSER0_CLK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCSI_PARSER0_CLK_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCSI_PARSER0_CLK_ENABLE_A {
        match self.bits {
            false => MCSI_PARSER0_CLK_ENABLE_A::DISABLE,
            true => MCSI_PARSER0_CLK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "CSI Parser0 clock disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCSI_PARSER0_CLK_ENABLE_A::DISABLE
    }
    #[doc = "CSI Parser0 clock enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCSI_PARSER0_CLK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `mcsi_parser0_clk_enable` writer - "]
pub type MCSI_PARSER0_CLK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, MCSI_PARSER0_CLK_ENABLE_A>;
impl<'a, REG> MCSI_PARSER0_CLK_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI Parser0 clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MCSI_PARSER0_CLK_ENABLE_A::DISABLE)
    }
    #[doc = "CSI Parser0 clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MCSI_PARSER0_CLK_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mcsi_parser0_clk_enable(&self) -> MCSI_PARSER0_CLK_ENABLE_R {
        MCSI_PARSER0_CLK_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_parser0_clk_enable(&mut self) -> MCSI_PARSER0_CLK_ENABLE_W<CCU_PARSER_CLK_EN_SPEC> {
        MCSI_PARSER0_CLK_ENABLE_W::new(self, 0)
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
#[doc = "CCU Parser Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_parser_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_parser_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCU_PARSER_CLK_EN_SPEC;
impl crate::RegisterSpec for CCU_PARSER_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccu_parser_clk_en::R`](R) reader structure"]
impl crate::Readable for CCU_PARSER_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccu_parser_clk_en::W`](W) writer structure"]
impl crate::Writable for CCU_PARSER_CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_parser_clk_en to value 0"]
impl crate::Resettable for CCU_PARSER_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
