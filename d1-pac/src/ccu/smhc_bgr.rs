#[doc = "Register `smhc_bgr` reader"]
pub struct R(crate::R<SMHC_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_bgr` writer"]
pub struct W(crate::W<SMHC_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_BGR_SPEC>;
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
impl From<crate::W<SMHC_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `smhc_gating[0-2]` reader - Gating Clock"]
pub type SMHC_GATING_R = crate::BitReader<SMHC_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMHC_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<SMHC_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: SMHC_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl SMHC_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMHC_GATING_A {
        match self.bits {
            false => SMHC_GATING_A::MASK,
            true => SMHC_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == SMHC_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == SMHC_GATING_A::PASS
    }
}
#[doc = "Field `smhc_gating[0-2]` writer - Gating Clock"]
pub type SMHC_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_BGR_SPEC, SMHC_GATING_A, O>;
impl<'a, const O: u8> SMHC_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SMHC_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(SMHC_GATING_A::PASS)
    }
}
#[doc = "Field `smhc_rst[0-2]` reader - Reset"]
pub type SMHC_RST_R = crate::BitReader<SMHC_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMHC_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<SMHC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SMHC_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SMHC_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMHC_RST_A {
        match self.bits {
            false => SMHC_RST_A::ASSERT,
            true => SMHC_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SMHC_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SMHC_RST_A::DEASSERT
    }
}
#[doc = "Field `smhc_rst[0-2]` writer - Reset"]
pub type SMHC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_BGR_SPEC, SMHC_RST_A, O>;
impl<'a, const O: u8> SMHC_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SMHC_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(SMHC_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn smhc_gating(&self, n: u8) -> SMHC_GATING_R {
        SMHC_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn smhc0_gating(&self) -> SMHC_GATING_R {
        SMHC_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn smhc1_gating(&self) -> SMHC_GATING_R {
        SMHC_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn smhc2_gating(&self) -> SMHC_GATING_R {
        SMHC_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn smhc_rst(&self, n: u8) -> SMHC_RST_R {
        SMHC_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn smhc0_rst(&self) -> SMHC_RST_R {
        SMHC_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn smhc1_rst(&self) -> SMHC_RST_R {
        SMHC_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn smhc2_rst(&self) -> SMHC_RST_R {
        SMHC_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn smhc_gating<const O: u8>(&mut self) -> SMHC_GATING_W<O> {
        SMHC_GATING_W::new(self)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn smhc0_gating(&mut self) -> SMHC_GATING_W<0> {
        SMHC_GATING_W::new(self)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn smhc1_gating(&mut self) -> SMHC_GATING_W<1> {
        SMHC_GATING_W::new(self)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn smhc2_gating(&mut self) -> SMHC_GATING_W<2> {
        SMHC_GATING_W::new(self)
    }
    #[doc = "Reset"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn smhc_rst<const O: u8>(&mut self) -> SMHC_RST_W<O> {
        SMHC_RST_W::new(self)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn smhc0_rst(&mut self) -> SMHC_RST_W<16> {
        SMHC_RST_W::new(self)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn smhc1_rst(&mut self) -> SMHC_RST_W<17> {
        SMHC_RST_W::new(self)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn smhc2_rst(&mut self) -> SMHC_RST_W<18> {
        SMHC_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMHC Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_bgr](index.html) module"]
pub struct SMHC_BGR_SPEC;
impl crate::RegisterSpec for SMHC_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_bgr::R](R) reader structure"]
impl crate::Readable for SMHC_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_bgr::W](W) writer structure"]
impl crate::Writable for SMHC_BGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_bgr to value 0"]
impl crate::Resettable for SMHC_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
