#[doc = "Register `hc_ls_threshold` reader"]
pub struct R(crate::R<HC_LS_THRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_LS_THRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_LS_THRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_LS_THRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_ls_threshold` writer"]
pub struct W(crate::W<HC_LS_THRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_LS_THRESHOLD_SPEC>;
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
impl From<crate::W<HC_LS_THRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_LS_THRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ls_threshold` reader - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
pub type LS_THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ls_threshold` writer - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
pub type LS_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_LS_THRESHOLD_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
    #[inline(always)]
    pub fn ls_threshold(&self) -> LS_THRESHOLD_R {
        LS_THRESHOLD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
    #[inline(always)]
    #[must_use]
    pub fn ls_threshold(&mut self) -> LS_THRESHOLD_W<0> {
        LS_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI LS Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_ls_threshold](index.html) module"]
pub struct HC_LS_THRESHOLD_SPEC;
impl crate::RegisterSpec for HC_LS_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_ls_threshold::R](R) reader structure"]
impl crate::Readable for HC_LS_THRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_ls_threshold::W](W) writer structure"]
impl crate::Writable for HC_LS_THRESHOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_ls_threshold to value 0x0628"]
impl crate::Resettable for HC_LS_THRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0628;
}
