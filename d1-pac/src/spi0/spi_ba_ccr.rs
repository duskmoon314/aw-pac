#[doc = "Register `spi_ba_ccr` reader"]
pub struct R(crate::R<SPI_BA_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BA_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BA_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BA_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_ba_ccr` writer"]
pub struct W(crate::W<SPI_BA_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BA_CCR_SPEC>;
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
impl From<crate::W<SPI_BA_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BA_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cdr_n` reader - Clock Divide Rate"]
pub type CDR_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cdr_n` writer - Clock Divide Rate"]
pub type CDR_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_BA_CCR_SPEC, u8, u8, 8, O>;
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
    pub fn cdr_n(&mut self) -> CDR_N_W<0> {
        CDR_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Bit-Aligned Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ba_ccr](index.html) module"]
pub struct SPI_BA_CCR_SPEC;
impl crate::RegisterSpec for SPI_BA_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ba_ccr::R](R) reader structure"]
impl crate::Readable for SPI_BA_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ba_ccr::W](W) writer structure"]
impl crate::Writable for SPI_BA_CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_ba_ccr to value 0"]
impl crate::Resettable for SPI_BA_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
