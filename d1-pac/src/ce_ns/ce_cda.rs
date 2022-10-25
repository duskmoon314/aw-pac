#[doc = "Register `ce_cda` reader"]
pub struct R(crate::R<CE_CDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_CDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_CDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_CDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ce_cda` writer"]
pub struct W(crate::W<CE_CDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_CDA_SPEC>;
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
impl From<crate::W<CE_CDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_CDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cur_dst_addr` reader - Current destination address"]
pub type CUR_DST_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current destination address"]
    #[inline(always)]
    pub fn cur_dst_addr(&self) -> CUR_DST_ADDR_R {
        CUR_DST_ADDR_R::new(self.bits)
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
#[doc = "Current Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_cda](index.html) module"]
pub struct CE_CDA_SPEC;
impl crate::RegisterSpec for CE_CDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_cda::R](R) reader structure"]
impl crate::Readable for CE_CDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_cda::W](W) writer structure"]
impl crate::Writable for CE_CDA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_cda to value 0"]
impl crate::Resettable for CE_CDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
