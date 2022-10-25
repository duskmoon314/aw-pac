#[doc = "Register `tp_cali_data` reader"]
pub struct R(crate::R<TP_CALI_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CALI_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CALI_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CALI_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tp_cali_data` writer"]
pub struct W(crate::W<TP_CALI_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CALI_DATA_SPEC>;
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
impl From<crate::W<TP_CALI_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CALI_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tp_cdat` reader - TP Common Data"]
pub type TP_CDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tp_cdat` writer - TP Common Data"]
pub type TP_CDAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TP_CALI_DATA_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - TP Common Data"]
    #[inline(always)]
    pub fn tp_cdat(&self) -> TP_CDAT_R {
        TP_CDAT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TP Common Data"]
    #[inline(always)]
    #[must_use]
    pub fn tp_cdat(&mut self) -> TP_CDAT_W<0> {
        TP_CDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Calibration Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_cali_data](index.html) module"]
pub struct TP_CALI_DATA_SPEC;
impl crate::RegisterSpec for TP_CALI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_cali_data::R](R) reader structure"]
impl crate::Readable for TP_CALI_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_cali_data::W](W) writer structure"]
impl crate::Writable for TP_CALI_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tp_cali_data to value 0"]
impl crate::Resettable for TP_CALI_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
