#[doc = "Register `SPI_BGR` reader"]
pub struct R(crate::R<SPI_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_BGR` writer"]
pub struct W(crate::W<SPI_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BGR_SPEC>;
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
impl From<crate::W<SPI_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<SPI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `SPI(0-1)_RST` reader - Reset"]
pub struct SPI_RST_R(crate::FieldReader<bool>);
impl SPI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_RST_A {
        match self.bits {
            false => SPI_RST_A::ASSERT,
            true => SPI_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == SPI_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == SPI_RST_A::DEASSERT
    }
}
impl core::ops::Deref for SPI_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `SPI(0-1)_RST` const generic writer - Reset"]
pub struct SPI_RST_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> SPI_RST_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SPI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(SPI_RST_A::DEASSERT)
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
pub enum SPI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<SPI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `SPI(0-1)_GATING` reader - Gating Clock"]
pub struct SPI_GATING_R(crate::FieldReader<bool>);
impl SPI_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_GATING_A {
        match self.bits {
            false => SPI_GATING_A::MASK,
            true => SPI_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == SPI_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == SPI_GATING_A::PASS
    }
}
impl core::ops::Deref for SPI_GATING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `SPI(0-1)_GATING` const generic writer - Gating Clock"]
pub struct SPI_GATING_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> SPI_GATING_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SPI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(SPI_GATING_A::PASS)
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
    pub unsafe fn spi_rst(&self, n: usize) -> SPI_RST_R {
        SPI_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn spi0_rst(&self) -> SPI_RST_R {
        SPI_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn spi1_rst(&self) -> SPI_RST_R {
        SPI_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn spi_gating(&self, n: usize) -> SPI_GATING_R {
        SPI_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn spi0_gating(&self) -> SPI_GATING_R {
        SPI_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn spi1_gating(&self) -> SPI_GATING_R {
        SPI_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn spi_rst<const O: usize>(&mut self) -> SPI_RST_W<O> {
        SPI_RST_W { w: self }
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn spi0_rst(&mut self) -> SPI_RST_W<16> {
        SPI_RST_W { w: self }
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn spi1_rst(&mut self) -> SPI_RST_W<17> {
        SPI_RST_W { w: self }
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn spi_gating<const O: usize>(&mut self) -> SPI_GATING_W<O> {
        SPI_GATING_W { w: self }
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn spi0_gating(&mut self) -> SPI_GATING_W<0> {
        SPI_GATING_W { w: self }
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn spi1_gating(&mut self) -> SPI_GATING_W<1> {
        SPI_GATING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bgr](index.html) module"]
pub struct SPI_BGR_SPEC;
impl crate::RegisterSpec for SPI_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_bgr::R](R) reader structure"]
impl crate::Readable for SPI_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_bgr::W](W) writer structure"]
impl crate::Writable for SPI_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_BGR to value 0"]
impl crate::Resettable for SPI_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
