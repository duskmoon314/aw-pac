#[doc = "Register `SPI_SAMP_DL` reader"]
pub struct R(crate::R<SPI_SAMP_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SAMP_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SAMP_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SAMP_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SAMP_DL` writer"]
pub struct W(crate::W<SPI_SAMP_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SAMP_DL_SPEC>;
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
impl From<crate::W<SPI_SAMP_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SAMP_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `samp_dl_cal_start` reader - Sample Delay Calibration Start"]
pub struct SAMP_DL_CAL_START_R(crate::FieldReader<bool, bool>);
impl SAMP_DL_CAL_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMP_DL_CAL_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_DL_CAL_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `samp_dl_cal_start` writer - Sample Delay Calibration Start"]
pub struct SAMP_DL_CAL_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_DL_CAL_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `samp_dl_cal_done` reader - Sample Delay Calibration Dont"]
pub struct SAMP_DL_CAL_DONE_R(crate::FieldReader<bool, bool>);
impl SAMP_DL_CAL_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMP_DL_CAL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_DL_CAL_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `samp_dl` reader - Sample Delay"]
pub struct SAMP_DL_R(crate::FieldReader<u8, u8>);
impl SAMP_DL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAMP_DL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_DL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `samp_dl_sw_en` reader - Sample Delay Software Enable"]
pub struct SAMP_DL_SW_EN_R(crate::FieldReader<bool, bool>);
impl SAMP_DL_SW_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMP_DL_SW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_DL_SW_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `samp_dl_sw_en` writer - Sample Delay Software Enable"]
pub struct SAMP_DL_SW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_DL_SW_EN_W<'a> {
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
#[doc = "Field `samp_dl_sw` reader - Sample Delay Software"]
pub struct SAMP_DL_SW_R(crate::FieldReader<u8, u8>);
impl SAMP_DL_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAMP_DL_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_DL_SW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `samp_dl_sw` writer - Sample Delay Software"]
pub struct SAMP_DL_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_DL_SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Sample Delay Calibration Start"]
    #[inline(always)]
    pub fn samp_dl_cal_start(&self) -> SAMP_DL_CAL_START_R {
        SAMP_DL_CAL_START_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Sample Delay Calibration Dont"]
    #[inline(always)]
    pub fn samp_dl_cal_done(&self) -> SAMP_DL_CAL_DONE_R {
        SAMP_DL_CAL_DONE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Sample Delay"]
    #[inline(always)]
    pub fn samp_dl(&self) -> SAMP_DL_R {
        SAMP_DL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn samp_dl_sw_en(&self) -> SAMP_DL_SW_EN_R {
        SAMP_DL_SW_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Sample Delay Software"]
    #[inline(always)]
    pub fn samp_dl_sw(&self) -> SAMP_DL_SW_R {
        SAMP_DL_SW_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Sample Delay Calibration Start"]
    #[inline(always)]
    pub fn samp_dl_cal_start(&mut self) -> SAMP_DL_CAL_START_W {
        SAMP_DL_CAL_START_W { w: self }
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn samp_dl_sw_en(&mut self) -> SAMP_DL_SW_EN_W {
        SAMP_DL_SW_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - Sample Delay Software"]
    #[inline(always)]
    pub fn samp_dl_sw(&mut self) -> SAMP_DL_SW_W {
        SAMP_DL_SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Sample Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_samp_dl](index.html) module"]
pub struct SPI_SAMP_DL_SPEC;
impl crate::RegisterSpec for SPI_SAMP_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_samp_dl::R](R) reader structure"]
impl crate::Readable for SPI_SAMP_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_samp_dl::W](W) writer structure"]
impl crate::Writable for SPI_SAMP_DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SAMP_DL to value 0"]
impl crate::Resettable for SPI_SAMP_DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
