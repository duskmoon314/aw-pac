#[doc = "Register `SPI_RBR` reader"]
pub struct R(crate::R<SPI_RBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_RBR` writer"]
pub struct W(crate::W<SPI_RBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_RBR_SPEC>;
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
impl From<crate::W<SPI_RBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_RBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vrb` reader - The Value of the Receive Bits"]
pub struct VRB_R(crate::FieldReader<u32, u32>);
impl VRB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VRB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VRB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vrb` writer - The Value of the Receive Bits"]
pub struct VRB_W<'a> {
    w: &'a mut W,
}
impl<'a> VRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The Value of the Receive Bits"]
    #[inline(always)]
    pub fn vrb(&self) -> VRB_R {
        VRB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Value of the Receive Bits"]
    #[inline(always)]
    pub fn vrb(&mut self) -> VRB_W {
        VRB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI RX Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rbr](index.html) module"]
pub struct SPI_RBR_SPEC;
impl crate::RegisterSpec for SPI_RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_rbr::R](R) reader structure"]
impl crate::Readable for SPI_RBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_rbr::W](W) writer structure"]
impl crate::Writable for SPI_RBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_RBR to value 0"]
impl crate::Resettable for SPI_RBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
