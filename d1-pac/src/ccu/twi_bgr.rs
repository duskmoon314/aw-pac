#[doc = "Register `TWI_BGR` reader"]
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
#[doc = "Register `TWI_BGR` writer"]
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
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `TWI(0-3)_RST` reader - Reset"]
pub struct TWI_RST_R(crate::FieldReader<bool, TWI_RST_A>);
impl TWI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TWI_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TWI_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == TWI_RST_A::DEASSERT
    }
}
impl core::ops::Deref for TWI_RST_R {
    type Target = crate::FieldReader<bool, TWI_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `TWI(0-3)_RST` writer - Reset"]
pub struct TWI_RST_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> TWI_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `TWI(0-3)_RST` const generic writer - Reset"]
pub struct TWI_RST_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> TWI_RST_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `TWI(0-3)_GATING` reader - Gating Clock"]
pub struct TWI_GATING_R(crate::FieldReader<bool, TWI_GATING_A>);
impl TWI_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TWI_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TWI_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == TWI_GATING_A::PASS
    }
}
impl core::ops::Deref for TWI_GATING_R {
    type Target = crate::FieldReader<bool, TWI_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `TWI(0-3)_GATING` writer - Gating Clock"]
pub struct TWI_GATING_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> TWI_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `TWI(0-3)_GATING` const generic writer - Gating Clock"]
pub struct TWI_GATING_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> TWI_GATING_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn twi_rst(&self, n: usize) -> TWI_RST_R {
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
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn twi_gating(&self, n: usize) -> TWI_GATING_R {
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
}
impl W {
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn twi_rst(&mut self, n: usize) -> TWI_RST_W {
        TWI_RST_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn twi0_rst(&mut self) -> TWI_RST_CGW<16> {
        TWI_RST_CGW { w: self }
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn twi1_rst(&mut self) -> TWI_RST_CGW<17> {
        TWI_RST_CGW { w: self }
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn twi2_rst(&mut self) -> TWI_RST_CGW<18> {
        TWI_RST_CGW { w: self }
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    pub fn twi3_rst(&mut self) -> TWI_RST_CGW<19> {
        TWI_RST_CGW { w: self }
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn twi_gating(&mut self, n: usize) -> TWI_GATING_W {
        TWI_GATING_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn twi0_gating(&mut self) -> TWI_GATING_CGW<0> {
        TWI_GATING_CGW { w: self }
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn twi1_gating(&mut self) -> TWI_GATING_CGW<1> {
        TWI_GATING_CGW { w: self }
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn twi2_gating(&mut self) -> TWI_GATING_CGW<2> {
        TWI_GATING_CGW { w: self }
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    pub fn twi3_gating(&mut self) -> TWI_GATING_CGW<3> {
        TWI_GATING_CGW { w: self }
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
}
#[doc = "`reset()` method sets TWI_BGR to value 0"]
impl crate::Resettable for TWI_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
