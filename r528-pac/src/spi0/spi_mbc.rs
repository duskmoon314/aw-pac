#[doc = "Register `SPI_MBC` reader"]
pub struct R(crate::R<SPI_MBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MBC` writer"]
pub struct W(crate::W<SPI_MBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MBC_SPEC>;
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
impl From<crate::W<SPI_MBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mbc` reader - Master Burst Counter"]
pub type MBC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `mbc` writer - Master Burst Counter"]
pub type MBC_W<'a> = crate::FieldWriter<'a, u32, SPI_MBC_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - Master Burst Counter"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Master Burst Counter"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W {
        MBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Burst Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mbc](index.html) module"]
pub struct SPI_MBC_SPEC;
impl crate::RegisterSpec for SPI_MBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mbc::R](R) reader structure"]
impl crate::Readable for SPI_MBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mbc::W](W) writer structure"]
impl crate::Writable for SPI_MBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MBC to value 0"]
impl crate::Resettable for SPI_MBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
