#[doc = "Register `bias` reader"]
pub struct R(crate::R<BIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bias` writer"]
pub struct W(crate::W<BIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIAS_SPEC>;
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
impl From<crate::W<BIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `biasdata` reader - Bias Current Register Setting Data"]
pub type BIASDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `biasdata` writer - Bias Current Register Setting Data"]
pub type BIASDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIAS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bias Current Register Setting Data"]
    #[inline(always)]
    pub fn biasdata(&self) -> BIASDATA_R {
        BIASDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bias Current Register Setting Data"]
    #[inline(always)]
    #[must_use]
    pub fn biasdata(&mut self) -> BIASDATA_W<0> {
        BIASDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIAS Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bias](index.html) module"]
pub struct BIAS_SPEC;
impl crate::RegisterSpec for BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bias::R](R) reader structure"]
impl crate::Readable for BIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bias::W](W) writer structure"]
impl crate::Writable for BIAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bias to value 0x80"]
impl crate::Resettable for BIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
