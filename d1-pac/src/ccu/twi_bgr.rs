#[doc = "Register `twi_bgr` reader"]
pub struct R(crate::R<TWI_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_bgr` writer"]
pub struct W(crate::W<TWI_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_BGR_SPEC>;
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
impl From<crate::W<TWI_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `twi_gating[0-3]` reader - Gating Clock"]
pub type TWI_GATING_R = crate::BitReader<TWI_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<TWI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: TWI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl TWI_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWI_GATING_A {
        match self.bits {
            false => TWI_GATING_A::MASK,
            true => TWI_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == TWI_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == TWI_GATING_A::PASS
    }
}
#[doc = "Field `twi_gating[0-3]` writer - Gating Clock"]
pub type TWI_GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_BGR_SPEC, TWI_GATING_A, O>;
impl<'a, const O: u8> TWI_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TWI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(TWI_GATING_A::PASS)
    }
}
#[doc = "Field `twi_rst[0-3]` reader - Reset"]
pub type TWI_RST_R = crate::BitReader<TWI_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<TWI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TWI_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TWI_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWI_RST_A {
        match self.bits {
            false => TWI_RST_A::ASSERT,
            true => TWI_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == TWI_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == TWI_RST_A::DEASSERT
    }
}
#[doc = "Field `twi_rst[0-3]` writer - Reset"]
pub type TWI_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_BGR_SPEC, TWI_RST_A, O>;
impl<'a, const O: u8> TWI_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(TWI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(TWI_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn twi_gating(&self, n: u8) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn twi0_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn twi1_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn twi2_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    pub fn twi3_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn twi_rst(&self, n: u8) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn twi0_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn twi1_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn twi2_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    pub fn twi3_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn twi_gating<const O: u8>(&mut self) -> TWI_GATING_W<O> {
        TWI_GATING_W::new(self)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi0_gating(&mut self) -> TWI_GATING_W<0> {
        TWI_GATING_W::new(self)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi1_gating(&mut self) -> TWI_GATING_W<1> {
        TWI_GATING_W::new(self)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi2_gating(&mut self) -> TWI_GATING_W<2> {
        TWI_GATING_W::new(self)
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi3_gating(&mut self) -> TWI_GATING_W<3> {
        TWI_GATING_W::new(self)
    }
    #[doc = "Reset"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn twi_rst<const O: u8>(&mut self) -> TWI_RST_W<O> {
        TWI_RST_W::new(self)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi0_rst(&mut self) -> TWI_RST_W<16> {
        TWI_RST_W::new(self)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi1_rst(&mut self) -> TWI_RST_W<17> {
        TWI_RST_W::new(self)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi2_rst(&mut self) -> TWI_RST_W<18> {
        TWI_RST_W::new(self)
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi3_rst(&mut self) -> TWI_RST_W<19> {
        TWI_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_bgr](index.html) module"]
pub struct TWI_BGR_SPEC;
impl crate::RegisterSpec for TWI_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_bgr::R](R) reader structure"]
impl crate::Readable for TWI_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_bgr::W](W) writer structure"]
impl crate::Writable for TWI_BGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_bgr to value 0"]
impl crate::Resettable for TWI_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
