#[doc = "Register `gp_data%s` reader"]
pub struct R(crate::R<GP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_data%s` writer"]
pub struct W(crate::W<GP_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATA_SPEC>;
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
impl From<crate::W<GP_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gp_data` reader - "]
pub type GP_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `gp_data` writer - "]
pub type GP_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gp_data(&self) -> GP_DATA_R {
        GP_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn gp_data(&mut self) -> GP_DATA_W<0> {
        GP_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_data](index.html) module"]
pub struct GP_DATA_SPEC;
impl crate::RegisterSpec for GP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_data::R](R) reader structure"]
impl crate::Readable for GP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_data::W](W) writer structure"]
impl crate::Writable for GP_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_data%s to value 0"]
impl crate::Resettable for GP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
