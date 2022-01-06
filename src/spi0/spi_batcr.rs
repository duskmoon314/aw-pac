#[doc = "Register `SPI_BATCR` reader"]
pub struct R(crate::R<SPI_BATCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BATCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BATCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BATCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_BATCR` writer"]
pub struct W(crate::W<SPI_BATCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BATCR_SPEC>;
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
impl From<crate::W<SPI_BATCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BATCR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Bit-Aligned Transfer Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_batcr](index.html) module"]
pub struct SPI_BATCR_SPEC;
impl crate::RegisterSpec for SPI_BATCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_batcr::R](R) reader structure"]
impl crate::Readable for SPI_BATCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_batcr::W](W) writer structure"]
impl crate::Writable for SPI_BATCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_BATCR to value 0"]
impl crate::Resettable for SPI_BATCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
