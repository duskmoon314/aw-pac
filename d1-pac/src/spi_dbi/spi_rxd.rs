#[doc = "Register `spi_rxd` reader"]
pub type R = crate::R<SPI_RXD_SPEC>;
#[doc = "Register `spi_rxd` writer"]
pub type W = crate::W<SPI_RXD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPI_RXD_SPEC> {
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
#[doc = "SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rxd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_RXD_SPEC;
impl crate::RegisterSpec for SPI_RXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rxd::R`](R) reader structure"]
impl crate::Readable for SPI_RXD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_rxd::W`](W) writer structure"]
impl crate::Writable for SPI_RXD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_rxd to value 0"]
impl crate::Resettable for SPI_RXD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
