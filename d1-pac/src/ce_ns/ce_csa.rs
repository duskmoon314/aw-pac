#[doc = "Register `CE_CSA` reader"]
pub struct R(crate::R<CE_CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_CSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_CSA` writer"]
pub struct W(crate::W<CE_CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_CSA_SPEC>;
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
impl From<crate::W<CE_CSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUR_SRC_ADDR` reader - Current source address"]
pub struct CUR_SRC_ADDR_R(crate::FieldReader<u32>);
impl CUR_SRC_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CUR_SRC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUR_SRC_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Current source address"]
    #[inline(always)]
    pub fn cur_src_addr(&self) -> CUR_SRC_ADDR_R {
        CUR_SRC_ADDR_R::new(self.bits)
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
#[doc = "Current Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_csa](index.html) module"]
pub struct CE_CSA_SPEC;
impl crate::RegisterSpec for CE_CSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_csa::R](R) reader structure"]
impl crate::Readable for CE_CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_csa::W](W) writer structure"]
impl crate::Writable for CE_CSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_CSA to value 0"]
impl crate::Resettable for CE_CSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
