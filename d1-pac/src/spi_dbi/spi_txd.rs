#[doc = "Register `spi_txd` reader"]
pub type R = crate::R<SPI_TXD_SPEC>;
#[doc = "Register `spi_txd` writer"]
pub type W = crate::W<SPI_TXD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPI_TXD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_TXD_SPEC;
impl crate::RegisterSpec for SPI_TXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_txd::R`](R) reader structure"]
impl crate::Readable for SPI_TXD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_txd::W`](W) writer structure"]
impl crate::Writable for SPI_TXD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_txd to value 0"]
impl crate::Resettable for SPI_TXD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
