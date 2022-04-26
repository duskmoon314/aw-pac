#[doc = "Register `SMHC_HS400_DL` reader"]
pub struct R(crate::R<SMHC_HS400_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_HS400_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_HS400_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_HS400_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_HS400_DL` writer"]
pub struct W(crate::W<SMHC_HS400_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_HS400_DL_SPEC>;
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
impl From<crate::W<SMHC_HS400_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_HS400_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HS400_DL_CAL_START` reader - HS400 Delay Calibration Start"]
pub struct HS400_DL_CAL_START_R(crate::FieldReader<bool>);
impl HS400_DL_CAL_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS400_DL_CAL_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS400_DL_CAL_START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_DL_CAL_START` writer - HS400 Delay Calibration Start"]
pub struct HS400_DL_CAL_START_W<'a> {
    w: &'a mut W,
}
impl<'a> HS400_DL_CAL_START_W<'a> {
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
#[doc = "Field `HS400_DL_CAL_DONE` reader - HS400 Delay Calibration Done"]
pub struct HS400_DL_CAL_DONE_R(crate::FieldReader<bool>);
impl HS400_DL_CAL_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS400_DL_CAL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS400_DL_CAL_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_DL` reader - HS400 Delay"]
pub struct HS400_DL_R(crate::FieldReader<u8>);
impl HS400_DL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HS400_DL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS400_DL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_DL_SW_EN` reader - Sample Delay Software Enable"]
pub struct HS400_DL_SW_EN_R(crate::FieldReader<bool>);
impl HS400_DL_SW_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS400_DL_SW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS400_DL_SW_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_DL_SW_EN` writer - Sample Delay Software Enable"]
pub struct HS400_DL_SW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HS400_DL_SW_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `HS400_DL_SW` reader - HS400 Delay Software"]
pub struct HS400_DL_SW_R(crate::FieldReader<u8>);
impl HS400_DL_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HS400_DL_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS400_DL_SW_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_DL_SW` writer - HS400 Delay Software"]
pub struct HS400_DL_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> HS400_DL_SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - HS400 Delay Calibration Start"]
    #[inline(always)]
    pub fn hs400_dl_cal_start(&self) -> HS400_DL_CAL_START_R {
        HS400_DL_CAL_START_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - HS400 Delay Calibration Done"]
    #[inline(always)]
    pub fn hs400_dl_cal_done(&self) -> HS400_DL_CAL_DONE_R {
        HS400_DL_CAL_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 8:11 - HS400 Delay"]
    #[inline(always)]
    pub fn hs400_dl(&self) -> HS400_DL_R {
        HS400_DL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn hs400_dl_sw_en(&self) -> HS400_DL_SW_EN_R {
        HS400_DL_SW_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:3 - HS400 Delay Software"]
    #[inline(always)]
    pub fn hs400_dl_sw(&self) -> HS400_DL_SW_R {
        HS400_DL_SW_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - HS400 Delay Calibration Start"]
    #[inline(always)]
    pub fn hs400_dl_cal_start(&mut self) -> HS400_DL_CAL_START_W {
        HS400_DL_CAL_START_W { w: self }
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn hs400_dl_sw_en(&mut self) -> HS400_DL_SW_EN_W {
        HS400_DL_SW_EN_W { w: self }
    }
    #[doc = "Bits 0:3 - HS400 Delay Software"]
    #[inline(always)]
    pub fn hs400_dl_sw(&mut self) -> HS400_DL_SW_W {
        HS400_DL_SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS400 Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_hs400_dl](index.html) module"]
pub struct SMHC_HS400_DL_SPEC;
impl crate::RegisterSpec for SMHC_HS400_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_hs400_dl::R](R) reader structure"]
impl crate::Readable for SMHC_HS400_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_hs400_dl::W](W) writer structure"]
impl crate::Writable for SMHC_HS400_DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_HS400_DL to value 0"]
impl crate::Resettable for SMHC_HS400_DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
