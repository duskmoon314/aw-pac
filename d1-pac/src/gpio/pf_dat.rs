#[doc = "Register `pf_dat` reader"]
pub struct R(crate::R<PF_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_dat` writer"]
pub struct W(crate::W<PF_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_DAT_SPEC>;
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
impl From<crate::W<PF_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf_dat` reader - PF Data"]
pub type PF_DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pf_dat` writer - PF Data"]
pub type PF_DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PF_DAT_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - PF Data"]
    #[inline(always)]
    pub fn pf_dat(&self) -> PF_DAT_R {
        PF_DAT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - PF Data"]
    #[inline(always)]
    #[must_use]
    pub fn pf_dat(&mut self) -> PF_DAT_W<0> {
        PF_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_dat](index.html) module"]
pub struct PF_DAT_SPEC;
impl crate::RegisterSpec for PF_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_dat::R](R) reader structure"]
impl crate::Readable for PF_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_dat::W](W) writer structure"]
impl crate::Writable for PF_DAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_dat to value 0"]
impl crate::Resettable for PF_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
