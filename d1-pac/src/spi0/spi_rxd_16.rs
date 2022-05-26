#[doc = "Register `SPI_RXD_16` reader"]
pub struct R(crate::R<SPI_RXD_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RXD_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RXD_16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RXD_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_RXD_16` writer"]
pub struct W(crate::W<SPI_RXD_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_RXD_16_SPEC>;
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
impl From<crate::W<SPI_RXD_16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_RXD_16_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI RX Data Register\n\nRDATA \\[15:0\\]: Receive Data and access in half-word method\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rxd_16](index.html) module"]
pub struct SPI_RXD_16_SPEC;
impl crate::RegisterSpec for SPI_RXD_16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_rxd_16::R](R) reader structure"]
impl crate::Readable for SPI_RXD_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_rxd_16::W](W) writer structure"]
impl crate::Writable for SPI_RXD_16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_RXD_16 to value 0"]
impl crate::Resettable for SPI_RXD_16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
