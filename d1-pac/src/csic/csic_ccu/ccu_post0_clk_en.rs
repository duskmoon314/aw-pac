#[doc = "Register `ccu_post0_clk_en` reader"]
pub type R = crate::R<CCU_POST0_CLK_EN_SPEC>;
#[doc = "Register `ccu_post0_clk_en` writer"]
pub type W = crate::W<CCU_POST0_CLK_EN_SPEC>;
#[doc = "Field `mcsi_bk_clk_enable[0-1]` reader - "]
pub type MCSI_BK_CLK_ENABLE_R = crate::BitReader<MCSI_BK_CLK_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCSI_BK_CLK_ENABLE_A {
    #[doc = "0: BK\\[i\\] clock disable"]
    DISABLE = 0,
    #[doc = "1: BK\\[i\\] clock enable,when MCSI_POST0_CLK_ENABLE is 1"]
    ENABLE = 1,
}
impl From<MCSI_BK_CLK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MCSI_BK_CLK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCSI_BK_CLK_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCSI_BK_CLK_ENABLE_A {
        match self.bits {
            false => MCSI_BK_CLK_ENABLE_A::DISABLE,
            true => MCSI_BK_CLK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "BK\\[i\\] clock disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCSI_BK_CLK_ENABLE_A::DISABLE
    }
    #[doc = "BK\\[i\\] clock enable,when MCSI_POST0_CLK_ENABLE is 1"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCSI_BK_CLK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `mcsi_bk_clk_enable[0-1]` writer - "]
pub type MCSI_BK_CLK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, MCSI_BK_CLK_ENABLE_A>;
impl<'a, REG> MCSI_BK_CLK_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BK\\[i\\] clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MCSI_BK_CLK_ENABLE_A::DISABLE)
    }
    #[doc = "BK\\[i\\] clock enable,when MCSI_POST0_CLK_ENABLE is 1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MCSI_BK_CLK_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `mcsi_post0_clk_enable` reader - "]
pub type MCSI_POST0_CLK_ENABLE_R = crate::BitReader<MCSI_POST0_CLK_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCSI_POST0_CLK_ENABLE_A {
    #[doc = "0: POST0 clock disable"]
    DISABLE = 0,
    #[doc = "1: POST0 clock enable"]
    ENABLE = 1,
}
impl From<MCSI_POST0_CLK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MCSI_POST0_CLK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCSI_POST0_CLK_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCSI_POST0_CLK_ENABLE_A {
        match self.bits {
            false => MCSI_POST0_CLK_ENABLE_A::DISABLE,
            true => MCSI_POST0_CLK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "POST0 clock disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCSI_POST0_CLK_ENABLE_A::DISABLE
    }
    #[doc = "POST0 clock enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCSI_POST0_CLK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `mcsi_post0_clk_enable` writer - "]
pub type MCSI_POST0_CLK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, MCSI_POST0_CLK_ENABLE_A>;
impl<'a, REG> MCSI_POST0_CLK_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POST0 clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MCSI_POST0_CLK_ENABLE_A::DISABLE)
    }
    #[doc = "POST0 clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MCSI_POST0_CLK_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `mcsi_bk0_clk_enable` field"]
    #[inline(always)]
    pub fn mcsi_bk_clk_enable(&self, n: u8) -> MCSI_BK_CLK_ENABLE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MCSI_BK_CLK_ENABLE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - mcsi_bk0_clk_enable"]
    #[inline(always)]
    pub fn mcsi_bk0_clk_enable(&self) -> MCSI_BK_CLK_ENABLE_R {
        MCSI_BK_CLK_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mcsi_bk1_clk_enable"]
    #[inline(always)]
    pub fn mcsi_bk1_clk_enable(&self) -> MCSI_BK_CLK_ENABLE_R {
        MCSI_BK_CLK_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mcsi_post0_clk_enable(&self) -> MCSI_POST0_CLK_ENABLE_R {
        MCSI_POST0_CLK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `mcsi_bk0_clk_enable` field"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_bk_clk_enable(&mut self, n: u8) -> MCSI_BK_CLK_ENABLE_W<CCU_POST0_CLK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MCSI_BK_CLK_ENABLE_W::new(self, n)
    }
    #[doc = "Bit 0 - mcsi_bk0_clk_enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_bk0_clk_enable(&mut self) -> MCSI_BK_CLK_ENABLE_W<CCU_POST0_CLK_EN_SPEC> {
        MCSI_BK_CLK_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - mcsi_bk1_clk_enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_bk1_clk_enable(&mut self) -> MCSI_BK_CLK_ENABLE_W<CCU_POST0_CLK_EN_SPEC> {
        MCSI_BK_CLK_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_post0_clk_enable(&mut self) -> MCSI_POST0_CLK_ENABLE_W<CCU_POST0_CLK_EN_SPEC> {
        MCSI_POST0_CLK_ENABLE_W::new(self, 16)
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
#[doc = "CCU Post0 Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_post0_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_post0_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCU_POST0_CLK_EN_SPEC;
impl crate::RegisterSpec for CCU_POST0_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccu_post0_clk_en::R`](R) reader structure"]
impl crate::Readable for CCU_POST0_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccu_post0_clk_en::W`](W) writer structure"]
impl crate::Writable for CCU_POST0_CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_post0_clk_en to value 0"]
impl crate::Resettable for CCU_POST0_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
