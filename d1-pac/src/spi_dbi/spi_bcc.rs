#[doc = "Register `SPI_BCC` reader"]
pub struct R(crate::R<SPI_BCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_BCC` writer"]
pub struct W(crate::W<SPI_BCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BCC_SPEC>;
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
impl From<crate::W<SPI_BCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Quad Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUAD_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<QUAD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: QUAD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `quad_en` reader - Quad Mode Enable"]
pub struct QUAD_EN_R(crate::FieldReader<bool>);
impl QUAD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUAD_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUAD_EN_A {
        match self.bits {
            false => QUAD_EN_A::DISABLE,
            true => QUAD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == QUAD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == QUAD_EN_A::ENABLE
    }
}
impl core::ops::Deref for QUAD_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `quad_en` writer - Quad Mode Enable"]
pub struct QUAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUAD_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(QUAD_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(QUAD_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Master Dual Mode RX Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRM_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DUAL = 1,
}
impl From<DRM_A> for bool {
    #[inline(always)]
    fn from(variant: DRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `drm` reader - Master Dual Mode RX Enable"]
pub struct DRM_R(crate::FieldReader<bool>);
impl DRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRM_A {
        match self.bits {
            false => DRM_A::SINGLE,
            true => DRM_A::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == DRM_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == DRM_A::DUAL
    }
}
impl core::ops::Deref for DRM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `drm` writer - Master Dual Mode RX Enable"]
pub struct DRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DRM_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DRM_A::DUAL)
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `dbc` reader - Master Dummy Burst Counter"]
pub struct DBC_R(crate::FieldReader<u8>);
impl DBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbc` writer - Master Dummy Burst Counter"]
pub struct DBC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `stc` reader - Master Single Mode Transmit Counter"]
pub struct STC_R(crate::FieldReader<u32>);
impl STC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STC_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `stc` writer - Master Single Mode Transmit Counter"]
pub struct STC_W<'a> {
    w: &'a mut W,
}
impl<'a> STC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - Quad Mode Enable"]
    #[inline(always)]
    pub fn quad_en(&self) -> QUAD_EN_R {
        QUAD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Master Dual Mode RX Enable"]
    #[inline(always)]
    pub fn drm(&self) -> DRM_R {
        DRM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Master Dummy Burst Counter"]
    #[inline(always)]
    pub fn dbc(&self) -> DBC_R {
        DBC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - Master Single Mode Transmit Counter"]
    #[inline(always)]
    pub fn stc(&self) -> STC_R {
        STC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 29 - Quad Mode Enable"]
    #[inline(always)]
    pub fn quad_en(&mut self) -> QUAD_EN_W {
        QUAD_EN_W { w: self }
    }
    #[doc = "Bit 28 - Master Dual Mode RX Enable"]
    #[inline(always)]
    pub fn drm(&mut self) -> DRM_W {
        DRM_W { w: self }
    }
    #[doc = "Bits 24:27 - Master Dummy Burst Counter"]
    #[inline(always)]
    pub fn dbc(&mut self) -> DBC_W {
        DBC_W { w: self }
    }
    #[doc = "Bits 0:23 - Master Single Mode Transmit Counter"]
    #[inline(always)]
    pub fn stc(&mut self) -> STC_W {
        STC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Burst Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bcc](index.html) module"]
pub struct SPI_BCC_SPEC;
impl crate::RegisterSpec for SPI_BCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_bcc::R](R) reader structure"]
impl crate::Readable for SPI_BCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_bcc::W](W) writer structure"]
impl crate::Writable for SPI_BCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_BCC to value 0"]
impl crate::Resettable for SPI_BCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
