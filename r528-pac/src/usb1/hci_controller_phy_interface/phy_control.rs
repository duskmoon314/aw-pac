#[doc = "Register `PHY_CONTROL` reader"]
pub struct R(crate::R<PHY_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_CONTROL` writer"]
pub struct W(crate::W<PHY_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CONTROL_SPEC>;
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
impl From<crate::W<PHY_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIST_EN_A` reader - "]
pub type BIST_EN_A_R = crate::BitReader<bool>;
#[doc = "Field `BIST_EN_A` writer - "]
pub type BIST_EN_A_W<'a> = crate::BitWriter<'a, u32, PHY_CONTROL_SPEC, bool, 16>;
#[doc = "Field `VC_ADDR` reader - vc_addr"]
pub type VC_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VC_ADDR` writer - vc_addr"]
pub type VC_ADDR_W<'a> = crate::FieldWriter<'a, u32, PHY_CONTROL_SPEC, u8, u8, 8, 8>;
#[doc = "Field `VC_DI` reader - vc_di"]
pub type VC_DI_R = crate::BitReader<bool>;
#[doc = "Field `VC_DI` writer - vc_di"]
pub type VC_DI_W<'a> = crate::BitWriter<'a, u32, PHY_CONTROL_SPEC, bool, 7>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIDDQ_A {
    #[doc = "1: Write 1 to disable phy"]
    DISABLE = 1,
    #[doc = "0: Write 0 to enable phy"]
    ENABLE = 0,
}
impl From<SIDDQ_A> for bool {
    #[inline(always)]
    fn from(variant: SIDDQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIDDQ` reader - "]
pub type SIDDQ_R = crate::BitReader<SIDDQ_A>;
impl SIDDQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIDDQ_A {
        match self.bits {
            true => SIDDQ_A::DISABLE,
            false => SIDDQ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SIDDQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SIDDQ_A::ENABLE
    }
}
#[doc = "Field `SIDDQ` writer - "]
pub type SIDDQ_W<'a> = crate::BitWriter<'a, u32, PHY_CONTROL_SPEC, SIDDQ_A, 3>;
impl<'a> SIDDQ_W<'a> {
    #[doc = "Write 1 to disable phy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SIDDQ_A::DISABLE)
    }
    #[doc = "Write 0 to enable phy"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SIDDQ_A::ENABLE)
    }
}
#[doc = "Field `VC_CLK` reader - vc_clk"]
pub type VC_CLK_R = crate::BitReader<bool>;
#[doc = "Field `VC_CLK` writer - vc_clk"]
pub type VC_CLK_W<'a> = crate::BitWriter<'a, u32, PHY_CONTROL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bist_en_a(&self) -> BIST_EN_A_R {
        BIST_EN_A_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:15 - vc_addr"]
    #[inline(always)]
    pub fn vc_addr(&self) -> VC_ADDR_R {
        VC_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - vc_di"]
    #[inline(always)]
    pub fn vc_di(&self) -> VC_DI_R {
        VC_DI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn siddq(&self) -> SIDDQ_R {
        SIDDQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 0 - vc_clk"]
    #[inline(always)]
    pub fn vc_clk(&self) -> VC_CLK_R {
        VC_CLK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bist_en_a(&mut self) -> BIST_EN_A_W {
        BIST_EN_A_W::new(self)
    }
    #[doc = "Bits 8:15 - vc_addr"]
    #[inline(always)]
    pub fn vc_addr(&mut self) -> VC_ADDR_W {
        VC_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - vc_di"]
    #[inline(always)]
    pub fn vc_di(&mut self) -> VC_DI_W {
        VC_DI_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn siddq(&mut self) -> SIDDQ_W {
        SIDDQ_W::new(self)
    }
    #[doc = "Bit 0 - vc_clk"]
    #[inline(always)]
    pub fn vc_clk(&mut self) -> VC_CLK_W {
        VC_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_control](index.html) module"]
pub struct PHY_CONTROL_SPEC;
impl crate::RegisterSpec for PHY_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_control::R](R) reader structure"]
impl crate::Readable for PHY_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_control::W](W) writer structure"]
impl crate::Writable for PHY_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHY_CONTROL to value 0"]
impl crate::Resettable for PHY_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
