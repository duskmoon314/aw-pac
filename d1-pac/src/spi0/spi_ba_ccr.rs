#[doc = "Register `spi_ba_ccr` reader"]
pub type R = crate::R<SPI_BA_CCR_SPEC>;
#[doc = "Register `spi_ba_ccr` writer"]
pub type W = crate::W<SPI_BA_CCR_SPEC>;
#[doc = "Field `cdr_n` reader - Clock Divide Rate"]
pub type CDR_N_R = crate::FieldReader;
#[doc = "Field `cdr_n` writer - Clock Divide Rate"]
pub type CDR_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock Divide Rate"]
    #[inline(always)]
    pub fn cdr_n(&self) -> CDR_N_R {
        CDR_N_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divide Rate"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_n(&mut self) -> CDR_N_W<SPI_BA_CCR_SPEC> {
        CDR_N_W::new(self, 0)
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
#[doc = "SPI Bit-Aligned Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ba_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ba_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_BA_CCR_SPEC;
impl crate::RegisterSpec for SPI_BA_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ba_ccr::R`](R) reader structure"]
impl crate::Readable for SPI_BA_CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ba_ccr::W`](W) writer structure"]
impl crate::Writable for SPI_BA_CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_ba_ccr to value 0"]
impl crate::Resettable for SPI_BA_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
