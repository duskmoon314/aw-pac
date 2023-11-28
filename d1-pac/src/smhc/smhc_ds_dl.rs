#[doc = "Register `smhc_ds_dl` reader"]
pub type R = crate::R<SMHC_DS_DL_SPEC>;
#[doc = "Register `smhc_ds_dl` writer"]
pub type W = crate::W<SMHC_DS_DL_SPEC>;
#[doc = "Field `ds_dl_sw` reader - Data Storbe Delay Software"]
pub type DS_DL_SW_R = crate::FieldReader;
#[doc = "Field `ds_dl_sw` writer - Data Storbe Delay Software"]
pub type DS_DL_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ds_dl_sw_en` reader - Sample Delay Software Enable"]
pub type DS_DL_SW_EN_R = crate::BitReader;
#[doc = "Field `ds_dl_sw_en` writer - Sample Delay Software Enable"]
pub type DS_DL_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ds_dl` reader - Data Strobe Delay"]
pub type DS_DL_R = crate::FieldReader;
#[doc = "Field `ds_dl_cal_done` reader - Data Strobe Delay Calibration Done"]
pub type DS_DL_CAL_DONE_R = crate::BitReader;
#[doc = "Field `ds_dl_cal_start` reader - Data Strobe Delay Calibration Start"]
pub type DS_DL_CAL_START_R = crate::BitReader;
#[doc = "Field `ds_dl_cal_start` writer - Data Strobe Delay Calibration Start"]
pub type DS_DL_CAL_START_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn ds_dl_sw(&mut self) -> DS_DL_SW_W<SMHC_DS_DL_SPEC> {
        DS_DL_SW_W::new(self, 0)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ds_dl_sw_en(&mut self) -> DS_DL_SW_EN_W<SMHC_DS_DL_SPEC> {
        DS_DL_SW_EN_W::new(self, 7)
    }
    #[doc = "Bit 15 - Data Strobe Delay Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn ds_dl_cal_start(&mut self) -> DS_DL_CAL_START_W<SMHC_DS_DL_SPEC> {
        DS_DL_CAL_START_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data Strobe Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ds_dl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ds_dl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_DS_DL_SPEC;
impl crate::RegisterSpec for SMHC_DS_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_ds_dl::R`](R) reader structure"]
impl crate::Readable for SMHC_DS_DL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_ds_dl::W`](W) writer structure"]
impl crate::Writable for SMHC_DS_DL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ds_dl to value 0"]
impl crate::Resettable for SMHC_DS_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
