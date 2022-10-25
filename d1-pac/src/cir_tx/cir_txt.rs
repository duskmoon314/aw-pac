#[doc = "Register `cir_txt` reader"]
pub struct R(crate::R<CIR_TXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_txt` writer"]
pub struct W(crate::W<CIR_TXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TXT_SPEC>;
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
impl From<crate::W<CIR_TXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `nctt` reader - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
pub type NCTT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `nctt` writer - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
pub type NCTT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_TXT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
    #[inline(always)]
    pub fn nctt(&self) -> NCTT_R {
        NCTT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
    #[inline(always)]
    #[must_use]
    pub fn nctt(&mut self) -> NCTT_W<0> {
        NCTT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_txt](index.html) module"]
pub struct CIR_TXT_SPEC;
impl crate::RegisterSpec for CIR_TXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_txt::R](R) reader structure"]
impl crate::Readable for CIR_TXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_txt::W](W) writer structure"]
impl crate::Writable for CIR_TXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_txt to value 0"]
impl crate::Resettable for CIR_TXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
