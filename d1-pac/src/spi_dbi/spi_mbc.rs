#[doc = "Register `spi_mbc` reader"]
pub type R = crate::R<SPI_MBC_SPEC>;
#[doc = "Register `spi_mbc` writer"]
pub type W = crate::W<SPI_MBC_SPEC>;
#[doc = "Field `mbc` reader - Master Burst Counter"]
pub type MBC_R = crate::FieldReader<u32>;
#[doc = "Field `mbc` writer - Master Burst Counter"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Master Burst Counter"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Master Burst Counter"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<SPI_MBC_SPEC> {
        MBC_W::new(self, 0)
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
#[doc = "SPI Master Burst Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MBC_SPEC;
impl crate::RegisterSpec for SPI_MBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mbc::R`](R) reader structure"]
impl crate::Readable for SPI_MBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mbc::W`](W) writer structure"]
impl crate::Writable for SPI_MBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_mbc to value 0"]
impl crate::Resettable for SPI_MBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
