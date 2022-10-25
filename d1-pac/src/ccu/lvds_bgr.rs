#[doc = "Register `lvds_bgr` reader"]
pub struct R(crate::R<LVDS_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDS_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDS_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDS_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lvds_bgr` writer"]
pub struct W(crate::W<LVDS_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDS_BGR_SPEC>;
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
impl From<crate::W<LVDS_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDS_BGR_SPEC>) -> Self {
        W(writer)
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
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LVDS_BGR_SPEC, RST_A, O>;
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
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
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
#[doc = "LVDS Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvds_bgr](index.html) module"]
pub struct LVDS_BGR_SPEC;
impl crate::RegisterSpec for LVDS_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lvds_bgr::R](R) reader structure"]
impl crate::Readable for LVDS_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvds_bgr::W](W) writer structure"]
impl crate::Writable for LVDS_BGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lvds_bgr to value 0"]
impl crate::Resettable for LVDS_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
