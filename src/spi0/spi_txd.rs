#[doc = "Register `SPI_TXD` reader"]
pub struct R(crate::R<SPI_TXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_TXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_TXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_TXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_TXD` writer"]
pub struct W(crate::W<SPI_TXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_TXD_SPEC>;
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
impl From<crate::W<SPI_TXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_TXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tdata` reader - Transmit Data"]
pub struct TDATA_R(crate::FieldReader<u32, u32>);
impl TDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tdata` writer - Transmit Data"]
pub struct TDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&self) -> TDATA_R {
        TDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&mut self) -> TDATA_W {
        TDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI TX Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_txd](index.html) module"]
pub struct SPI_TXD_SPEC;
impl crate::RegisterSpec for SPI_TXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_txd::R](R) reader structure"]
impl crate::Readable for SPI_TXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_txd::W](W) writer structure"]
impl crate::Writable for SPI_TXD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_TXD to value 0"]
impl crate::Resettable for SPI_TXD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
