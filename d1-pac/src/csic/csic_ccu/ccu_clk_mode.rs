#[doc = "Register `ccu_clk_mode` reader"]
pub struct R(crate::R<CCU_CLK_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCU_CLK_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCU_CLK_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCU_CLK_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ccu_clk_mode` writer"]
pub struct W(crate::W<CCU_CLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCU_CLK_MODE_SPEC>;
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
impl From<crate::W<CCU_CLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCU_CLK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> CCU_CLK_GATING_DISABLE_A {
        match self.bits {
            false => CCU_CLK_GATING_DISABLE_A::EFFECT,
            true => CCU_CLK_GATING_DISABLE_A::NOT_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `EFFECT`"]
    #[inline(always)]
    pub fn is_effect(&self) -> bool {
        *self == CCU_CLK_GATING_DISABLE_A::EFFECT
    }
    #[doc = "Checks if the value of the field is `NOT_EFFECT`"]
    #[inline(always)]
    pub fn is_not_effect(&self) -> bool {
        *self == CCU_CLK_GATING_DISABLE_A::NOT_EFFECT
    }
}
#[doc = "Field `ccu_clk_gating_disable` writer - "]
pub type CCU_CLK_GATING_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCU_CLK_MODE_SPEC, CCU_CLK_GATING_DISABLE_A, O>;
impl<'a, const O: u8> CCU_CLK_GATING_DISABLE_W<'a, O> {
    #[doc = "CCU Clock Gating Registers(0x0004~0x0010) effect"]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(CCU_CLK_GATING_DISABLE_A::EFFECT)
    }
    #[doc = "CCU Clock Gating Registers(0x0004~0x0010) not effect"]
    #[inline(always)]
    pub fn not_effect(self) -> &'a mut W {
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
    pub fn ccu_clk_gating_disable(&mut self) -> CCU_CLK_GATING_DISABLE_W<31> {
        CCU_CLK_GATING_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccu_clk_mode](index.html) module"]
pub struct CCU_CLK_MODE_SPEC;
impl crate::RegisterSpec for CCU_CLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccu_clk_mode::R](R) reader structure"]
impl crate::Readable for CCU_CLK_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccu_clk_mode::W](W) writer structure"]
impl crate::Writable for CCU_CLK_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ccu_clk_mode to value 0"]
impl crate::Resettable for CCU_CLK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
