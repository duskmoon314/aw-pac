#[doc = "Register `fboot_info%s` reader"]
pub struct R(crate::R<FBOOT_INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBOOT_INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBOOT_INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBOOT_INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fboot_info%s` writer"]
pub struct W(crate::W<FBOOT_INFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBOOT_INFO_SPEC>;
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
impl From<crate::W<FBOOT_INFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBOOT_INFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fboot_info` reader - Fast Boot Information \\[i\\], refer to BROM spec."]
pub type FBOOT_INFO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `fboot_info` writer - Fast Boot Information \\[i\\], refer to BROM spec."]
pub type FBOOT_INFO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FBOOT_INFO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Fast Boot Information \\[i\\], refer to BROM spec."]
    #[inline(always)]
    pub fn fboot_info(&self) -> FBOOT_INFO_R {
        FBOOT_INFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fast Boot Information \\[i\\], refer to BROM spec."]
    #[inline(always)]
    #[must_use]
    pub fn fboot_info(&mut self) -> FBOOT_INFO_W<0> {
        FBOOT_INFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Boot Information Register \\[01\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fboot_info](index.html) module"]
pub struct FBOOT_INFO_SPEC;
impl crate::RegisterSpec for FBOOT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fboot_info::R](R) reader structure"]
impl crate::Readable for FBOOT_INFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fboot_info::W](W) writer structure"]
impl crate::Writable for FBOOT_INFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fboot_info%s to value 0"]
impl crate::Resettable for FBOOT_INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
