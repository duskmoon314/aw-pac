#[doc = "Register `spi_fcr` reader"]
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
#[doc = "Register `spi_fcr` writer"]
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
#[doc = "Field `rf_trig_level` reader - RXFIFO Ready Request Trigger Level"]
pub type RF_TRIG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_trig_level` writer - RXFIFO Ready Request Trigger Level"]
pub type RF_TRIG_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_FCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `rf_drq_en` reader - RXFIFO DMA Request Enable"]
pub type RF_DRQ_EN_R = crate::BitReader<RF_DRQ_EN_A>;
#[doc = "RXFIFO DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RF_DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == RF_DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_DRQ_EN_A::ENABLE
    }
}
#[doc = "Field `rf_drq_en` writer - RXFIFO DMA Request Enable"]
pub type RF_DRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_FCR_SPEC, RF_DRQ_EN_A, O>;
impl<'a, const O: u8> RF_DRQ_EN_W<'a, O> {
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
}
#[doc = "Field `rf_test_en` reader - RXFIFO Test Mode Enable"]
pub type RF_TEST_EN_R = crate::BitReader<RF_TEST_EN_A>;
#[doc = "RXFIFO Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RF_TEST_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == RF_TEST_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_TEST_EN_A::ENABLE
    }
}
#[doc = "Field `rf_test_en` writer - RXFIFO Test Mode Enable"]
pub type RF_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_FCR_SPEC, RF_TEST_EN_A, O>;
impl<'a, const O: u8> RF_TEST_EN_W<'a, O> {
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
}
#[doc = "Field `rf_rst` reader - RXFIFO Reset"]
pub type RF_RST_R = crate::BitReader<bool>;
#[doc = "Field `rf_rst` writer - RXFIFO Reset"]
pub type RF_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_FCR_SPEC, bool, O>;
#[doc = "Field `tf_trig_level` reader - TXFIFO Empty Request Trigger Level"]
pub type TF_TRIG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tf_trig_level` writer - TXFIFO Empty Request Trigger Level"]
pub type TF_TRIG_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_FCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `tf_drq_en` reader - TXFIFO DMA Request Enable"]
pub type TF_DRQ_EN_R = crate::BitReader<TF_DRQ_EN_A>;
#[doc = "TXFIFO DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TF_DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TF_DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_DRQ_EN_A::ENABLE
    }
}
#[doc = "Field `tf_drq_en` writer - TXFIFO DMA Request Enable"]
pub type TF_DRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_FCR_SPEC, TF_DRQ_EN_A, O>;
impl<'a, const O: u8> TF_DRQ_EN_W<'a, O> {
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
}
#[doc = "Field `tf_test_en` reader - TXFIFO Test Mode Enable"]
pub type TF_TEST_EN_R = crate::BitReader<TF_TEST_EN_A>;
#[doc = "TXFIFO Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TF_TEST_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TF_TEST_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_TEST_EN_A::ENABLE
    }
}
#[doc = "Field `tf_test_en` writer - TXFIFO Test Mode Enable"]
pub type TF_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_FCR_SPEC, TF_TEST_EN_A, O>;
impl<'a, const O: u8> TF_TEST_EN_W<'a, O> {
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
}
#[doc = "Field `tf_rst` reader - TXFIFO Reset"]
pub type TF_RST_R = crate::BitReader<bool>;
#[doc = "Field `tf_rst` writer - TXFIFO Reset"]
pub type TF_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_FCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - RXFIFO Ready Request Trigger Level"]
    #[inline(always)]
    pub fn rf_trig_level(&self) -> RF_TRIG_LEVEL_R {
        RF_TRIG_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - RXFIFO DMA Request Enable"]
    #[inline(always)]
    pub fn rf_drq_en(&self) -> RF_DRQ_EN_R {
        RF_DRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - RXFIFO Test Mode Enable"]
    #[inline(always)]
    pub fn rf_test_en(&self) -> RF_TEST_EN_R {
        RF_TEST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFO Reset"]
    #[inline(always)]
    pub fn rf_rst(&self) -> RF_RST_R {
        RF_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - TXFIFO Empty Request Trigger Level"]
    #[inline(always)]
    pub fn tf_trig_level(&self) -> TF_TRIG_LEVEL_R {
        TF_TRIG_LEVEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - TXFIFO DMA Request Enable"]
    #[inline(always)]
    pub fn tf_drq_en(&self) -> TF_DRQ_EN_R {
        TF_DRQ_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFIFO Test Mode Enable"]
    #[inline(always)]
    pub fn tf_test_en(&self) -> TF_TEST_EN_R {
        TF_TEST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TXFIFO Reset"]
    #[inline(always)]
    pub fn tf_rst(&self) -> TF_RST_R {
        TF_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - RXFIFO Ready Request Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn rf_trig_level(&mut self) -> RF_TRIG_LEVEL_W<0> {
        RF_TRIG_LEVEL_W::new(self)
    }
    #[doc = "Bit 8 - RXFIFO DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_drq_en(&mut self) -> RF_DRQ_EN_W<8> {
        RF_DRQ_EN_W::new(self)
    }
    #[doc = "Bit 14 - RXFIFO Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_test_en(&mut self) -> RF_TEST_EN_W<14> {
        RF_TEST_EN_W::new(self)
    }
    #[doc = "Bit 15 - RXFIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rf_rst(&mut self) -> RF_RST_W<15> {
        RF_RST_W::new(self)
    }
    #[doc = "Bits 16:23 - TXFIFO Empty Request Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn tf_trig_level(&mut self) -> TF_TRIG_LEVEL_W<16> {
        TF_TRIG_LEVEL_W::new(self)
    }
    #[doc = "Bit 24 - TXFIFO DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_drq_en(&mut self) -> TF_DRQ_EN_W<24> {
        TF_DRQ_EN_W::new(self)
    }
    #[doc = "Bit 30 - TXFIFO Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_test_en(&mut self) -> TF_TEST_EN_W<30> {
        TF_TEST_EN_W::new(self)
    }
    #[doc = "Bit 31 - TXFIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tf_rst(&mut self) -> TF_RST_W<31> {
        TF_RST_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_fcr to value 0"]
impl crate::Resettable for SPI_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
