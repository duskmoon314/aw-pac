#[doc = "Register `csic_mbus_req_max` reader"]
pub struct R(crate::R<CSIC_MBUS_REQ_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_MBUS_REQ_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_MBUS_REQ_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_MBUS_REQ_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_mbus_req_max` writer"]
pub struct W(crate::W<CSIC_MBUS_REQ_MAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_MBUS_REQ_MAX_SPEC>;
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
impl From<crate::W<CSIC_MBUS_REQ_MAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_MBUS_REQ_MAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mcsi_mem_req_max` reader - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
pub type MCSI_MEM_REQ_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mcsi_mem_req_max` writer - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
pub type MCSI_MEM_REQ_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_MBUS_REQ_MAX_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
    #[inline(always)]
    pub fn mcsi_mem_req_max(&self) -> MCSI_MEM_REQ_MAX_R {
        MCSI_MEM_REQ_MAX_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_mem_req_max(&mut self) -> MCSI_MEM_REQ_MAX_W<0> {
        MCSI_MEM_REQ_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC MBUS REQ MAX Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_mbus_req_max](index.html) module"]
pub struct CSIC_MBUS_REQ_MAX_SPEC;
impl crate::RegisterSpec for CSIC_MBUS_REQ_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_mbus_req_max::R](R) reader structure"]
impl crate::Readable for CSIC_MBUS_REQ_MAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_mbus_req_max::W](W) writer structure"]
impl crate::Writable for CSIC_MBUS_REQ_MAX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_mbus_req_max to value 0x0f"]
impl crate::Resettable for CSIC_MBUS_REQ_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
