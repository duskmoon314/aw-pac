#[doc = "Register `ccu_post0_clk_en` reader"]
pub struct R(crate::R<CCU_POST0_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCU_POST0_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCU_POST0_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCU_POST0_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ccu_post0_clk_en` writer"]
pub struct W(crate::W<CCU_POST0_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCU_POST0_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCU_POST0_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCU_POST0_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> MCSI_BK_CLK_ENABLE_A {
        match self.bits {
            false => MCSI_BK_CLK_ENABLE_A::DISABLE,
            true => MCSI_BK_CLK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCSI_BK_CLK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCSI_BK_CLK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `mcsi_bk_clk_enable[0-1]` writer - "]
pub type MCSI_BK_CLK_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCU_POST0_CLK_EN_SPEC, MCSI_BK_CLK_ENABLE_A, O>;
impl<'a, const O: u8> MCSI_BK_CLK_ENABLE_W<'a, O> {
    #[doc = "BK\\[i\\] clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCSI_BK_CLK_ENABLE_A::DISABLE)
    }
    #[doc = "BK\\[i\\] clock enable,when MCSI_POST0_CLK_ENABLE is 1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn variant(&self) -> MCSI_POST0_CLK_ENABLE_A {
        match self.bits {
            false => MCSI_POST0_CLK_ENABLE_A::DISABLE,
            true => MCSI_POST0_CLK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCSI_POST0_CLK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCSI_POST0_CLK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `mcsi_post0_clk_enable` writer - "]
pub type MCSI_POST0_CLK_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCU_POST0_CLK_EN_SPEC, MCSI_POST0_CLK_ENABLE_A, O>;
impl<'a, const O: u8> MCSI_POST0_CLK_ENABLE_W<'a, O> {
    #[doc = "POST0 clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCSI_POST0_CLK_ENABLE_A::DISABLE)
    }
    #[doc = "POST0 clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCSI_POST0_CLK_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn mcsi_bk_clk_enable(&self, n: u8) -> MCSI_BK_CLK_ENABLE_R {
        MCSI_BK_CLK_ENABLE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mcsi_bk0_clk_enable(&self) -> MCSI_BK_CLK_ENABLE_R {
        MCSI_BK_CLK_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
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
    #[doc = ""]
    #[inline(always)]
    #[must_use]
    pub unsafe fn mcsi_bk_clk_enable<const O: u8>(&mut self) -> MCSI_BK_CLK_ENABLE_W<O> {
        MCSI_BK_CLK_ENABLE_W::new(self)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_bk0_clk_enable(&mut self) -> MCSI_BK_CLK_ENABLE_W<0> {
        MCSI_BK_CLK_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_bk1_clk_enable(&mut self) -> MCSI_BK_CLK_ENABLE_W<1> {
        MCSI_BK_CLK_ENABLE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_post0_clk_enable(&mut self) -> MCSI_POST0_CLK_ENABLE_W<16> {
        MCSI_POST0_CLK_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU Post0 Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccu_post0_clk_en](index.html) module"]
pub struct CCU_POST0_CLK_EN_SPEC;
impl crate::RegisterSpec for CCU_POST0_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccu_post0_clk_en::R](R) reader structure"]
impl crate::Readable for CCU_POST0_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccu_post0_clk_en::W](W) writer structure"]
impl crate::Writable for CCU_POST0_CLK_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_post0_clk_en to value 0"]
impl crate::Resettable for CCU_POST0_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
