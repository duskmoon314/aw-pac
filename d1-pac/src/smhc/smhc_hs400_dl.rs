#[doc = "Register `smhc_hs400_dl` reader"]
pub type R = crate::R<SMHC_HS400_DL_SPEC>;
#[doc = "Register `smhc_hs400_dl` writer"]
pub type W = crate::W<SMHC_HS400_DL_SPEC>;
#[doc = "Field `hs400_dl_sw` reader - HS400 Delay Software"]
pub type HS400_DL_SW_R = crate::FieldReader;
#[doc = "Field `hs400_dl_sw` writer - HS400 Delay Software"]
pub type HS400_DL_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hs400_dl_sw_en` reader - Sample Delay Software Enable"]
pub type HS400_DL_SW_EN_R = crate::BitReader;
#[doc = "Field `hs400_dl_sw_en` writer - Sample Delay Software Enable"]
pub type HS400_DL_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hs400_dl` reader - HS400 Delay"]
pub type HS400_DL_R = crate::FieldReader;
#[doc = "Field `hs400_dl_cal_done` reader - HS400 Delay Calibration Done"]
pub type HS400_DL_CAL_DONE_R = crate::BitReader;
#[doc = "Field `hs400_dl_cal_start` reader - HS400 Delay Calibration Start"]
pub type HS400_DL_CAL_START_R = crate::BitReader;
#[doc = "Field `hs400_dl_cal_start` writer - HS400 Delay Calibration Start"]
pub type HS400_DL_CAL_START_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn hs400_dl_sw(&mut self) -> HS400_DL_SW_W<SMHC_HS400_DL_SPEC> {
        HS400_DL_SW_W::new(self, 0)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_dl_sw_en(&mut self) -> HS400_DL_SW_EN_W<SMHC_HS400_DL_SPEC> {
        HS400_DL_SW_EN_W::new(self, 7)
    }
    #[doc = "Bit 15 - HS400 Delay Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_dl_cal_start(&mut self) -> HS400_DL_CAL_START_W<SMHC_HS400_DL_SPEC> {
        HS400_DL_CAL_START_W::new(self, 15)
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
#[doc = "HS400 Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_hs400_dl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_hs400_dl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_HS400_DL_SPEC;
impl crate::RegisterSpec for SMHC_HS400_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_hs400_dl::R`](R) reader structure"]
impl crate::Readable for SMHC_HS400_DL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_hs400_dl::W`](W) writer structure"]
impl crate::Writable for SMHC_HS400_DL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_hs400_dl to value 0"]
impl crate::Resettable for SMHC_HS400_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
