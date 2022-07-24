#[doc = "Register `prs_c1_output_hsize` reader"]
pub struct R(crate::R<PRS_C1_OUTPUT_HSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_C1_OUTPUT_HSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_C1_OUTPUT_HSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_C1_OUTPUT_HSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prs_c1_output_hsize` writer"]
pub struct W(crate::W<PRS_C1_OUTPUT_HSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_C1_OUTPUT_HSIZE_SPEC>;
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
impl From<crate::W<PRS_C1_OUTPUT_HSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_C1_OUTPUT_HSIZE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parser Channel_1 Output Horizontal Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_c1_output_hsize](index.html) module"]
pub struct PRS_C1_OUTPUT_HSIZE_SPEC;
impl crate::RegisterSpec for PRS_C1_OUTPUT_HSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_c1_output_hsize::R](R) reader structure"]
impl crate::Readable for PRS_C1_OUTPUT_HSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs_c1_output_hsize::W](W) writer structure"]
impl crate::Writable for PRS_C1_OUTPUT_HSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets prs_c1_output_hsize to value 0"]
impl crate::Resettable for PRS_C1_OUTPUT_HSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
