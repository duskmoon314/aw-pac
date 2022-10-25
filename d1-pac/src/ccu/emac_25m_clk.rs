#[doc = "Register `emac_25m_clk` reader"]
pub struct R(crate::R<EMAC_25M_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_25M_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_25M_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_25M_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_25m_clk` writer"]
pub struct W(crate::W<EMAC_25M_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_25M_CLK_SPEC>;
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
impl From<crate::W<EMAC_25M_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_25M_CLK_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> CLK_GATING_A {
        match self.bits {
            false => CLK_GATING_A::OFF,
            true => CLK_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_GATING_A::ON
    }
}
#[doc = "Field `clk_gating` writer - Gating Special Clock"]
pub type CLK_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_25M_CLK_SPEC, CLK_GATING_A, O>;
impl<'a, const O: u8> CLK_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
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
    pub fn variant(&self) -> CLK_SRC_GATING_A {
        match self.bits {
            false => CLK_SRC_GATING_A::OFF,
            true => CLK_SRC_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_SRC_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_SRC_GATING_A::ON
    }
}
#[doc = "Field `clk_src_gating` writer - Gating the Source Clock of Special Clock"]
pub type CLK_SRC_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_25M_CLK_SPEC, CLK_SRC_GATING_A, O>;
impl<'a, const O: u8> CLK_SRC_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_SRC_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
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
    pub fn clk_gating(&mut self) -> CLK_GATING_W<31> {
        CLK_GATING_W::new(self)
    }
    #[doc = "Bit 31 - Gating the Source Clock of Special Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_gating(&mut self) -> CLK_SRC_GATING_W<31> {
        CLK_SRC_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC_25M Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_25m_clk](index.html) module"]
pub struct EMAC_25M_CLK_SPEC;
impl crate::RegisterSpec for EMAC_25M_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_25m_clk::R](R) reader structure"]
impl crate::Readable for EMAC_25M_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_25m_clk::W](W) writer structure"]
impl crate::Writable for EMAC_25M_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_25m_clk to value 0"]
impl crate::Resettable for EMAC_25M_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
