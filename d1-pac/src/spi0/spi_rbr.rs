#[doc = "Register `spi_rbr` reader"]
pub type R = crate::R<SPI_RBR_SPEC>;
#[doc = "Register `spi_rbr` writer"]
pub type W = crate::W<SPI_RBR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPI_RBR_SPEC> {
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
#[doc = "SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_RBR_SPEC;
impl crate::RegisterSpec for SPI_RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rbr::R`](R) reader structure"]
impl crate::Readable for SPI_RBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_rbr::W`](W) writer structure"]
impl crate::Writable for SPI_RBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_rbr to value 0"]
impl crate::Resettable for SPI_RBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
