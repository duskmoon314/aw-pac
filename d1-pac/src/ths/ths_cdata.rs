#[doc = "Register `ths_cdata` reader"]
pub struct R(crate::R<THS_CDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_CDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_CDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_CDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ths_cdata` writer"]
pub struct W(crate::W<THS_CDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_CDATA_SPEC>;
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
impl From<crate::W<THS_CDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_CDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ths_cdata` reader - Thermal sensor calibration data"]
pub type THS_CDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ths_cdata` writer - Thermal sensor calibration data"]
pub type THS_CDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THS_CDATA_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Thermal sensor calibration data"]
    #[inline(always)]
    pub fn ths_cdata(&self) -> THS_CDATA_R {
        THS_CDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Thermal sensor calibration data"]
    #[inline(always)]
    #[must_use]
    pub fn ths_cdata(&mut self) -> THS_CDATA_W<0> {
        THS_CDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Calibration Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_cdata](index.html) module"]
pub struct THS_CDATA_SPEC;
impl crate::RegisterSpec for THS_CDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_cdata::R](R) reader structure"]
impl crate::Readable for THS_CDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_cdata::W](W) writer structure"]
impl crate::Writable for THS_CDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_cdata to value 0x0800"]
impl crate::Resettable for THS_CDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
