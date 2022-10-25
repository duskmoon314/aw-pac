#[doc = "Register `gp_cdata` reader"]
pub struct R(crate::R<GP_CDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_cdata` writer"]
pub struct W(crate::W<GP_CDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CDATA_SPEC>;
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
impl From<crate::W<GP_CDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gp_cdata` reader - GPADC Calibration Data"]
pub type GP_CDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gp_cdata` writer - GPADC Calibration Data"]
pub type GP_CDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_CDATA_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - GPADC Calibration Data"]
    #[inline(always)]
    pub fn gp_cdata(&self) -> GP_CDATA_R {
        GP_CDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - GPADC Calibration Data"]
    #[inline(always)]
    #[must_use]
    pub fn gp_cdata(&mut self) -> GP_CDATA_W<0> {
        GP_CDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Calibration Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_cdata](index.html) module"]
pub struct GP_CDATA_SPEC;
impl crate::RegisterSpec for GP_CDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_cdata::R](R) reader structure"]
impl crate::Readable for GP_CDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_cdata::W](W) writer structure"]
impl crate::Writable for GP_CDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_cdata to value 0"]
impl crate::Resettable for GP_CDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
