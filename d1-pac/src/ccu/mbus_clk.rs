#[doc = "Register `MBUS_CLK` reader"]
pub struct R(crate::R<MBUS_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBUS_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBUS_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBUS_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBUS_CLK` writer"]
pub struct W(crate::W<MBUS_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBUS_CLK_SPEC>;
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
impl From<crate::W<MBUS_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBUS_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MBUS Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBUS_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<MBUS_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MBUS_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBUS_RST` reader - MBUS Reset"]
pub type MBUS_RST_R = crate::BitReader<MBUS_RST_A>;
impl MBUS_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBUS_RST_A {
        match self.bits {
            false => MBUS_RST_A::ASSERT,
            true => MBUS_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == MBUS_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == MBUS_RST_A::DEASSERT
    }
}
#[doc = "Field `MBUS_RST` writer - MBUS Reset"]
pub type MBUS_RST_W<'a> = crate::BitWriter<'a, u32, MBUS_CLK_SPEC, MBUS_RST_A, 30>;
impl<'a> MBUS_RST_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(MBUS_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(MBUS_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Bit 30 - MBUS Reset"]
    #[inline(always)]
    pub fn mbus_rst(&self) -> MBUS_RST_R {
        MBUS_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - MBUS Reset"]
    #[inline(always)]
    pub fn mbus_rst(&mut self) -> MBUS_RST_W {
        MBUS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MBUS Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbus_clk](index.html) module"]
pub struct MBUS_CLK_SPEC;
impl crate::RegisterSpec for MBUS_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbus_clk::R](R) reader structure"]
impl crate::Readable for MBUS_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbus_clk::W](W) writer structure"]
impl crate::Writable for MBUS_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBUS_CLK to value 0"]
impl crate::Resettable for MBUS_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
