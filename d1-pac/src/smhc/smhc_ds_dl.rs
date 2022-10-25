#[doc = "Register `smhc_ds_dl` reader"]
pub struct R(crate::R<SMHC_DS_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_DS_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_DS_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_DS_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_ds_dl` writer"]
pub struct W(crate::W<SMHC_DS_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_DS_DL_SPEC>;
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
impl From<crate::W<SMHC_DS_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_DS_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ds_dl_sw` reader - Data Storbe Delay Software"]
pub type DS_DL_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ds_dl_sw` writer - Data Storbe Delay Software"]
pub type DS_DL_SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_DS_DL_SPEC, u8, u8, 6, O>;
#[doc = "Field `ds_dl_sw_en` reader - Sample Delay Software Enable"]
pub type DS_DL_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `ds_dl_sw_en` writer - Sample Delay Software Enable"]
pub type DS_DL_SW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_DS_DL_SPEC, bool, O>;
#[doc = "Field `ds_dl` reader - Data Strobe Delay"]
pub type DS_DL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ds_dl_cal_done` reader - Data Strobe Delay Calibration Done"]
pub type DS_DL_CAL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ds_dl_cal_start` reader - Data Strobe Delay Calibration Start"]
pub type DS_DL_CAL_START_R = crate::BitReader<bool>;
#[doc = "Field `ds_dl_cal_start` writer - Data Strobe Delay Calibration Start"]
pub type DS_DL_CAL_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_DS_DL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Data Storbe Delay Software"]
    #[inline(always)]
    pub fn ds_dl_sw(&self) -> DS_DL_SW_R {
        DS_DL_SW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn ds_dl_sw_en(&self) -> DS_DL_SW_EN_R {
        DS_DL_SW_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Data Strobe Delay"]
    #[inline(always)]
    pub fn ds_dl(&self) -> DS_DL_R {
        DS_DL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Data Strobe Delay Calibration Done"]
    #[inline(always)]
    pub fn ds_dl_cal_done(&self) -> DS_DL_CAL_DONE_R {
        DS_DL_CAL_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Strobe Delay Calibration Start"]
    #[inline(always)]
    pub fn ds_dl_cal_start(&self) -> DS_DL_CAL_START_R {
        DS_DL_CAL_START_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Storbe Delay Software"]
    #[inline(always)]
    #[must_use]
    pub fn ds_dl_sw(&mut self) -> DS_DL_SW_W<0> {
        DS_DL_SW_W::new(self)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ds_dl_sw_en(&mut self) -> DS_DL_SW_EN_W<7> {
        DS_DL_SW_EN_W::new(self)
    }
    #[doc = "Bit 15 - Data Strobe Delay Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn ds_dl_cal_start(&mut self) -> DS_DL_CAL_START_W<15> {
        DS_DL_CAL_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Strobe Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ds_dl](index.html) module"]
pub struct SMHC_DS_DL_SPEC;
impl crate::RegisterSpec for SMHC_DS_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ds_dl::R](R) reader structure"]
impl crate::Readable for SMHC_DS_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ds_dl::W](W) writer structure"]
impl crate::Writable for SMHC_DS_DL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ds_dl to value 0"]
impl crate::Resettable for SMHC_DS_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
