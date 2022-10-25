#[doc = "Register `spi_mtc` reader"]
pub struct R(crate::R<SPI_MTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_mtc` writer"]
pub struct W(crate::W<SPI_MTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MTC_SPEC>;
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
impl From<crate::W<SPI_MTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mwtc` reader - Master Write Transmit Counter"]
pub type MWTC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `mwtc` writer - Master Write Transmit Counter"]
pub type MWTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_MTC_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Master Write Transmit Counter"]
    #[inline(always)]
    pub fn mwtc(&self) -> MWTC_R {
        MWTC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Master Write Transmit Counter"]
    #[inline(always)]
    #[must_use]
    pub fn mwtc(&mut self) -> MWTC_W<0> {
        MWTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Transmit Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mtc](index.html) module"]
pub struct SPI_MTC_SPEC;
impl crate::RegisterSpec for SPI_MTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mtc::R](R) reader structure"]
impl crate::Readable for SPI_MTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mtc::W](W) writer structure"]
impl crate::Writable for SPI_MTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_mtc to value 0"]
impl crate::Resettable for SPI_MTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
