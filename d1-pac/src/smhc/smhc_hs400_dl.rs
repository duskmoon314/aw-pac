#[doc = "Register `smhc_hs400_dl` reader"]
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
#[doc = "Register `smhc_hs400_dl` writer"]
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
#[doc = "Field `hs400_dl_sw` reader - HS400 Delay Software"]
pub type HS400_DL_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hs400_dl_sw` writer - HS400 Delay Software"]
pub type HS400_DL_SW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMHC_HS400_DL_SPEC, u8, u8, 4, O>;
#[doc = "Field `hs400_dl_sw_en` reader - Sample Delay Software Enable"]
pub type HS400_DL_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `hs400_dl_sw_en` writer - Sample Delay Software Enable"]
pub type HS400_DL_SW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_HS400_DL_SPEC, bool, O>;
#[doc = "Field `hs400_dl` reader - HS400 Delay"]
pub type HS400_DL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hs400_dl_cal_done` reader - HS400 Delay Calibration Done"]
pub type HS400_DL_CAL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `hs400_dl_cal_start` reader - HS400 Delay Calibration Start"]
pub type HS400_DL_CAL_START_R = crate::BitReader<bool>;
#[doc = "Field `hs400_dl_cal_start` writer - HS400 Delay Calibration Start"]
pub type HS400_DL_CAL_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_HS400_DL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - HS400 Delay Software"]
    #[inline(always)]
    pub fn hs400_dl_sw(&self) -> HS400_DL_SW_R {
        HS400_DL_SW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn hs400_dl_sw_en(&self) -> HS400_DL_SW_EN_R {
        HS400_DL_SW_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - HS400 Delay"]
    #[inline(always)]
    pub fn hs400_dl(&self) -> HS400_DL_R {
        HS400_DL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - HS400 Delay Calibration Done"]
    #[inline(always)]
    pub fn hs400_dl_cal_done(&self) -> HS400_DL_CAL_DONE_R {
        HS400_DL_CAL_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - HS400 Delay Calibration Start"]
    #[inline(always)]
    pub fn hs400_dl_cal_start(&self) -> HS400_DL_CAL_START_R {
        HS400_DL_CAL_START_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HS400 Delay Software"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_dl_sw(&mut self) -> HS400_DL_SW_W<0> {
        HS400_DL_SW_W::new(self)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_dl_sw_en(&mut self) -> HS400_DL_SW_EN_W<7> {
        HS400_DL_SW_EN_W::new(self)
    }
    #[doc = "Bit 15 - HS400 Delay Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_dl_cal_start(&mut self) -> HS400_DL_CAL_START_W<15> {
        HS400_DL_CAL_START_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_hs400_dl to value 0"]
impl crate::Resettable for SMHC_HS400_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
