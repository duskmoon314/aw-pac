#[doc = "Register `spi_wcr` reader"]
pub struct R(crate::R<SPI_WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_wcr` writer"]
pub struct W(crate::W<SPI_WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_WCR_SPEC>;
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
impl From<crate::W<SPI_WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wwc` reader - Wait clock counter"]
pub type WWC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wwc` writer - Wait clock counter"]
pub type WWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_WCR_SPEC, u16, u16, 16, O>;
#[doc = "Field `swc` reader - Dual mode direction switch wait clock counter"]
pub type SWC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `swc` writer - Dual mode direction switch wait clock counter"]
pub type SWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_WCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:15 - Wait clock counter"]
    #[inline(always)]
    pub fn wwc(&self) -> WWC_R {
        WWC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Dual mode direction switch wait clock counter"]
    #[inline(always)]
    pub fn swc(&self) -> SWC_R {
        SWC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wait clock counter"]
    #[inline(always)]
    #[must_use]
    pub fn wwc(&mut self) -> WWC_W<0> {
        WWC_W::new(self)
    }
    #[doc = "Bits 16:19 - Dual mode direction switch wait clock counter"]
    #[inline(always)]
    #[must_use]
    pub fn swc(&mut self) -> SWC_W<16> {
        SWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Wait Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_wcr](index.html) module"]
pub struct SPI_WCR_SPEC;
impl crate::RegisterSpec for SPI_WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_wcr::R](R) reader structure"]
impl crate::Readable for SPI_WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_wcr::W](W) writer structure"]
impl crate::Writable for SPI_WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_wcr to value 0"]
impl crate::Resettable for SPI_WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
