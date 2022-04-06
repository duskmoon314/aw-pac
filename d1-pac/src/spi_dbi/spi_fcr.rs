#[doc = "Register `SPI_FCR` reader"]
pub struct R(crate::R<SPI_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FCR` writer"]
pub struct W(crate::W<SPI_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FCR_SPEC>;
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
impl From<crate::W<SPI_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tf_rst` reader - TXFIFO Reset"]
pub struct TF_RST_R(crate::FieldReader<bool, bool>);
impl TF_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_rst` writer - TXFIFO Reset"]
pub struct TF_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "TXFIFO Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_TEST_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_TEST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_TEST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tf_test_en` reader - TXFIFO Test Mode Enable"]
pub struct TF_TEST_EN_R(crate::FieldReader<bool, TF_TEST_EN_A>);
impl TF_TEST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_TEST_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_TEST_EN_A {
        match self.bits {
            false => TF_TEST_EN_A::DISABLE,
            true => TF_TEST_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TF_TEST_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TF_TEST_EN_A::ENABLE
    }
}
impl core::ops::Deref for TF_TEST_EN_R {
    type Target = crate::FieldReader<bool, TF_TEST_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_test_en` writer - TXFIFO Test Mode Enable"]
pub struct TF_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_TEST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_TEST_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_TEST_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_TEST_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "TXFIFO DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_DRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tf_drq_en` reader - TXFIFO DMA Request Enable"]
pub struct TF_DRQ_EN_R(crate::FieldReader<bool, TF_DRQ_EN_A>);
impl TF_DRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_DRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_DRQ_EN_A {
        match self.bits {
            false => TF_DRQ_EN_A::DISABLE,
            true => TF_DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TF_DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TF_DRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for TF_DRQ_EN_R {
    type Target = crate::FieldReader<bool, TF_DRQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_drq_en` writer - TXFIFO DMA Request Enable"]
pub struct TF_DRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_DRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_DRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_DRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_DRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `tf_trig_level` reader - TXFIFO Empty Request Trigger Level"]
pub struct TF_TRIG_LEVEL_R(crate::FieldReader<u8, u8>);
impl TF_TRIG_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TF_TRIG_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF_TRIG_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_trig_level` writer - TXFIFO Empty Request Trigger Level"]
pub struct TF_TRIG_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_TRIG_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `rf_rst` reader - RXFIFO Reset"]
pub struct RF_RST_R(crate::FieldReader<bool, bool>);
impl RF_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rst` writer - RXFIFO Reset"]
pub struct RF_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "RXFIFO Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_TEST_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_TEST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_TEST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rf_test_en` reader - RXFIFO Test Mode Enable"]
pub struct RF_TEST_EN_R(crate::FieldReader<bool, RF_TEST_EN_A>);
impl RF_TEST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_TEST_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_TEST_EN_A {
        match self.bits {
            false => RF_TEST_EN_A::DISABLE,
            true => RF_TEST_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RF_TEST_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RF_TEST_EN_A::ENABLE
    }
}
impl core::ops::Deref for RF_TEST_EN_R {
    type Target = crate::FieldReader<bool, RF_TEST_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_test_en` writer - RXFIFO Test Mode Enable"]
pub struct RF_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TEST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_TEST_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_TEST_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_TEST_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "RXFIFO DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_DRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rf_drq_en` reader - RXFIFO DMA Request Enable"]
pub struct RF_DRQ_EN_R(crate::FieldReader<bool, RF_DRQ_EN_A>);
impl RF_DRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_DRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_DRQ_EN_A {
        match self.bits {
            false => RF_DRQ_EN_A::DISABLE,
            true => RF_DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RF_DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RF_DRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for RF_DRQ_EN_R {
    type Target = crate::FieldReader<bool, RF_DRQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_drq_en` writer - RXFIFO DMA Request Enable"]
pub struct RF_DRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_DRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_DRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_DRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_DRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `rf_trig_level` reader - RXFIFO Ready Request Trigger Level"]
pub struct RF_TRIG_LEVEL_R(crate::FieldReader<u8, u8>);
impl RF_TRIG_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_TRIG_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TRIG_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_trig_level` writer - RXFIFO Ready Request Trigger Level"]
pub struct RF_TRIG_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRIG_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - TXFIFO Reset"]
    #[inline(always)]
    pub fn tf_rst(&self) -> TF_RST_R {
        TF_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFIFO Test Mode Enable"]
    #[inline(always)]
    pub fn tf_test_en(&self) -> TF_TEST_EN_R {
        TF_TEST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 24 - TXFIFO DMA Request Enable"]
    #[inline(always)]
    pub fn tf_drq_en(&self) -> TF_DRQ_EN_R {
        TF_DRQ_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 16:23 - TXFIFO Empty Request Trigger Level"]
    #[inline(always)]
    pub fn tf_trig_level(&self) -> TF_TRIG_LEVEL_R {
        TF_TRIG_LEVEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - RXFIFO Reset"]
    #[inline(always)]
    pub fn rf_rst(&self) -> RF_RST_R {
        RF_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - RXFIFO Test Mode Enable"]
    #[inline(always)]
    pub fn rf_test_en(&self) -> RF_TEST_EN_R {
        RF_TEST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 8 - RXFIFO DMA Request Enable"]
    #[inline(always)]
    pub fn rf_drq_en(&self) -> RF_DRQ_EN_R {
        RF_DRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - RXFIFO Ready Request Trigger Level"]
    #[inline(always)]
    pub fn rf_trig_level(&self) -> RF_TRIG_LEVEL_R {
        RF_TRIG_LEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - TXFIFO Reset"]
    #[inline(always)]
    pub fn tf_rst(&mut self) -> TF_RST_W {
        TF_RST_W { w: self }
    }
    #[doc = "Bit 30 - TXFIFO Test Mode Enable"]
    #[inline(always)]
    pub fn tf_test_en(&mut self) -> TF_TEST_EN_W {
        TF_TEST_EN_W { w: self }
    }
    #[doc = "Bit 24 - TXFIFO DMA Request Enable"]
    #[inline(always)]
    pub fn tf_drq_en(&mut self) -> TF_DRQ_EN_W {
        TF_DRQ_EN_W { w: self }
    }
    #[doc = "Bits 16:23 - TXFIFO Empty Request Trigger Level"]
    #[inline(always)]
    pub fn tf_trig_level(&mut self) -> TF_TRIG_LEVEL_W {
        TF_TRIG_LEVEL_W { w: self }
    }
    #[doc = "Bit 15 - RXFIFO Reset"]
    #[inline(always)]
    pub fn rf_rst(&mut self) -> RF_RST_W {
        RF_RST_W { w: self }
    }
    #[doc = "Bit 14 - RXFIFO Test Mode Enable"]
    #[inline(always)]
    pub fn rf_test_en(&mut self) -> RF_TEST_EN_W {
        RF_TEST_EN_W { w: self }
    }
    #[doc = "Bit 8 - RXFIFO DMA Request Enable"]
    #[inline(always)]
    pub fn rf_drq_en(&mut self) -> RF_DRQ_EN_W {
        RF_DRQ_EN_W { w: self }
    }
    #[doc = "Bits 0:7 - RXFIFO Ready Request Trigger Level"]
    #[inline(always)]
    pub fn rf_trig_level(&mut self) -> RF_TRIG_LEVEL_W {
        RF_TRIG_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fcr](index.html) module"]
pub struct SPI_FCR_SPEC;
impl crate::RegisterSpec for SPI_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fcr::R](R) reader structure"]
impl crate::Readable for SPI_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fcr::W](W) writer structure"]
impl crate::Writable for SPI_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FCR to value 0"]
impl crate::Resettable for SPI_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
