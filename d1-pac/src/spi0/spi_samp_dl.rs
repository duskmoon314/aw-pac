#[doc = "Register `spi_samp_dl` reader"]
pub type R = crate::R<SPI_SAMP_DL_SPEC>;
#[doc = "Register `spi_samp_dl` writer"]
pub type W = crate::W<SPI_SAMP_DL_SPEC>;
#[doc = "Field `samp_dl_sw` reader - Sample Delay Software"]
pub type SAMP_DL_SW_R = crate::FieldReader;
#[doc = "Field `samp_dl_sw` writer - Sample Delay Software"]
pub type SAMP_DL_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `samp_dl_sw_en` reader - Sample Delay Software Enable"]
pub type SAMP_DL_SW_EN_R = crate::BitReader;
#[doc = "Field `samp_dl_sw_en` writer - Sample Delay Software Enable"]
pub type SAMP_DL_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `samp_dl` reader - Sample Delay"]
pub type SAMP_DL_R = crate::FieldReader;
#[doc = "Field `samp_dl_cal_done` reader - Sample Delay Calibration Dont"]
pub type SAMP_DL_CAL_DONE_R = crate::BitReader;
#[doc = "Field `samp_dl_cal_start` reader - Sample Delay Calibration Start"]
pub type SAMP_DL_CAL_START_R = crate::BitReader;
#[doc = "Field `samp_dl_cal_start` writer - Sample Delay Calibration Start"]
pub type SAMP_DL_CAL_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Sample Delay Software"]
    #[inline(always)]
    pub fn samp_dl_sw(&self) -> SAMP_DL_SW_R {
        SAMP_DL_SW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn samp_dl_sw_en(&self) -> SAMP_DL_SW_EN_R {
        SAMP_DL_SW_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Sample Delay"]
    #[inline(always)]
    pub fn samp_dl(&self) -> SAMP_DL_R {
        SAMP_DL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Sample Delay Calibration Dont"]
    #[inline(always)]
    pub fn samp_dl_cal_done(&self) -> SAMP_DL_CAL_DONE_R {
        SAMP_DL_CAL_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sample Delay Calibration Start"]
    #[inline(always)]
    pub fn samp_dl_cal_start(&self) -> SAMP_DL_CAL_START_R {
        SAMP_DL_CAL_START_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sample Delay Software"]
    #[inline(always)]
    #[must_use]
    pub fn samp_dl_sw(&mut self) -> SAMP_DL_SW_W<SPI_SAMP_DL_SPEC> {
        SAMP_DL_SW_W::new(self, 0)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    #[must_use]
    pub fn samp_dl_sw_en(&mut self) -> SAMP_DL_SW_EN_W<SPI_SAMP_DL_SPEC> {
        SAMP_DL_SW_EN_W::new(self, 7)
    }
    #[doc = "Bit 15 - Sample Delay Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn samp_dl_cal_start(&mut self) -> SAMP_DL_CAL_START_W<SPI_SAMP_DL_SPEC> {
        SAMP_DL_CAL_START_W::new(self, 15)
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
#[doc = "SPI Sample Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_samp_dl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_samp_dl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SAMP_DL_SPEC;
impl crate::RegisterSpec for SPI_SAMP_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_samp_dl::R`](R) reader structure"]
impl crate::Readable for SPI_SAMP_DL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_samp_dl::W`](W) writer structure"]
impl crate::Writable for SPI_SAMP_DL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_samp_dl to value 0"]
impl crate::Resettable for SPI_SAMP_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
