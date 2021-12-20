#[doc = "Register `SPI_SAMP_DL` reader"]
pub struct R(crate::R<SPI_SAMP_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SAMP_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SAMP_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SAMP_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SAMP_DL` writer"]
pub struct W(crate::W<SPI_SAMP_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SAMP_DL_SPEC>;
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
impl From<crate::W<SPI_SAMP_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SAMP_DL_SPEC>) -> Self {
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
#[doc = "SPI Sample Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_samp_dl](index.html) module"]
pub struct SPI_SAMP_DL_SPEC;
impl crate::RegisterSpec for SPI_SAMP_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_samp_dl::R](R) reader structure"]
impl crate::Readable for SPI_SAMP_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_samp_dl::W](W) writer structure"]
impl crate::Writable for SPI_SAMP_DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SAMP_DL to value 0"]
impl crate::Resettable for SPI_SAMP_DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
