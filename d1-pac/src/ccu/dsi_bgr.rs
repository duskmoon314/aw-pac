#[doc = "Register `dsi_bgr` reader"]
pub struct R(crate::R<DSI_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dsi_bgr` writer"]
pub struct W(crate::W<DSI_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_BGR_SPEC>;
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
impl From<crate::W<DSI_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gating` reader - Gating Clock"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::MASK,
            true => GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == GATING_A::PASS
    }
}
#[doc = "Field `gating` writer - Gating Clock"]
pub type GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSI_BGR_SPEC, GATING_A, O>;
impl<'a, const O: u8> GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(GATING_A::PASS)
    }
}
#[doc = "Field `rst` reader - Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::ASSERT,
            true => RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == RST_A::DEASSERT
    }
}
#[doc = "Field `rst` writer - Reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSI_BGR_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<0> {
        GATING_W::new(self)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<16> {
        RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_bgr](index.html) module"]
pub struct DSI_BGR_SPEC;
impl crate::RegisterSpec for DSI_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_bgr::R](R) reader structure"]
impl crate::Readable for DSI_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_bgr::W](W) writer structure"]
impl crate::Writable for DSI_BGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dsi_bgr to value 0"]
impl crate::Resettable for DSI_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
