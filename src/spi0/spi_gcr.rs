#[doc = "Register `SPI_GCR` reader"]
pub struct R(crate::R<SPI_GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_GCR` writer"]
pub struct W(crate::W<SPI_GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_GCR_SPEC>;
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
impl From<crate::W<SPI_GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `srst` reader - Soft reset"]
pub struct SRST_R(crate::FieldReader<bool, bool>);
impl SRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `srst` writer - Soft reset"]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Transmit Pause Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_EN_A {
    #[doc = "0: normal operation, ignore RXFIFO status"]
    NORMAL = 0,
    #[doc = "1: Stop transmit data when RXFIFO full"]
    STOP_WHEN_FULL = 1,
}
impl From<TP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tp_en` reader - Transmit Pause Enable"]
pub struct TP_EN_R(crate::FieldReader<bool, TP_EN_A>);
impl TP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_EN_A {
        match self.bits {
            false => TP_EN_A::NORMAL,
            true => TP_EN_A::STOP_WHEN_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TP_EN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `STOP_WHEN_FULL`"]
    #[inline(always)]
    pub fn is_stop_when_full(&self) -> bool {
        **self == TP_EN_A::STOP_WHEN_FULL
    }
}
impl core::ops::Deref for TP_EN_R {
    type Target = crate::FieldReader<bool, TP_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tp_en` writer - Transmit Pause Enable"]
pub struct TP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "normal operation, ignore RXFIFO status"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TP_EN_A::NORMAL)
    }
    #[doc = "Stop transmit data when RXFIFO full"]
    #[inline(always)]
    pub fn stop_when_full(self) -> &'a mut W {
        self.variant(TP_EN_A::STOP_WHEN_FULL)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Sample timing Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_SELEC_A {
    #[doc = "0: Old mode of Sample Timing"]
    OLD_MODE = 0,
    #[doc = "1: New mode of Sample Timing"]
    NEW_MODE = 1,
}
impl From<MODE_SELEC_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_SELEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mode_selec` reader - Sample timing Mode Select"]
pub struct MODE_SELEC_R(crate::FieldReader<bool, MODE_SELEC_A>);
impl MODE_SELEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_SELEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_SELEC_A {
        match self.bits {
            false => MODE_SELEC_A::OLD_MODE,
            true => MODE_SELEC_A::NEW_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `OLD_MODE`"]
    #[inline(always)]
    pub fn is_old_mode(&self) -> bool {
        **self == MODE_SELEC_A::OLD_MODE
    }
    #[doc = "Checks if the value of the field is `NEW_MODE`"]
    #[inline(always)]
    pub fn is_new_mode(&self) -> bool {
        **self == MODE_SELEC_A::NEW_MODE
    }
}
impl core::ops::Deref for MODE_SELEC_R {
    type Target = crate::FieldReader<bool, MODE_SELEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode_selec` writer - Sample timing Mode Select"]
pub struct MODE_SELEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_SELEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_SELEC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Old mode of Sample Timing"]
    #[inline(always)]
    pub fn old_mode(self) -> &'a mut W {
        self.variant(MODE_SELEC_A::OLD_MODE)
    }
    #[doc = "New mode of Sample Timing"]
    #[inline(always)]
    pub fn new_mode(self) -> &'a mut W {
        self.variant(MODE_SELEC_A::NEW_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "SPI Function Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    SLAVE = 0,
    #[doc = "1: `1`"]
    MASTER = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mode` reader - SPI Function Mode Select"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SLAVE,
            true => MODE_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == MODE_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == MODE_A::MASTER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode` writer - SPI Function Mode Select"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MODE_A::SLAVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MODE_A::MASTER)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "SPI Module Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `en` reader - SPI Module Enable Control"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLE,
            true => EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == EN_A::ENABLE
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en` writer - SPI Module Enable Control"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Soft reset"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Pause Enable"]
    #[inline(always)]
    pub fn tp_en(&self) -> TP_EN_R {
        TP_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sample timing Mode Select"]
    #[inline(always)]
    pub fn mode_selec(&self) -> MODE_SELEC_R {
        MODE_SELEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI Function Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SPI Module Enable Control"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Soft reset"]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Bit 7 - Transmit Pause Enable"]
    #[inline(always)]
    pub fn tp_en(&mut self) -> TP_EN_W {
        TP_EN_W { w: self }
    }
    #[doc = "Bit 2 - Sample timing Mode Select"]
    #[inline(always)]
    pub fn mode_selec(&mut self) -> MODE_SELEC_W {
        MODE_SELEC_W { w: self }
    }
    #[doc = "Bit 1 - SPI Function Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 0 - SPI Module Enable Control"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_gcr](index.html) module"]
pub struct SPI_GCR_SPEC;
impl crate::RegisterSpec for SPI_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_gcr::R](R) reader structure"]
impl crate::Readable for SPI_GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_gcr::W](W) writer structure"]
impl crate::Writable for SPI_GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_GCR to value 0"]
impl crate::Resettable for SPI_GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
