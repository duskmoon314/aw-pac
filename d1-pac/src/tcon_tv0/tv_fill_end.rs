#[doc = "Register `tv_fill_end%s` reader"]
pub struct R(crate::R<TV_FILL_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_FILL_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_FILL_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_FILL_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_fill_end%s` writer"]
pub struct W(crate::W<TV_FILL_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_FILL_END_SPEC>;
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
impl From<crate::W<TV_FILL_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_FILL_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fill_end` reader - Fill End"]
pub type FILL_END_R = crate::FieldReader<u32, u32>;
#[doc = "Field `fill_end` writer - Fill End"]
pub type FILL_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_FILL_END_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Fill End"]
    #[inline(always)]
    pub fn fill_end(&self) -> FILL_END_R {
        FILL_END_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Fill End"]
    #[inline(always)]
    #[must_use]
    pub fn fill_end(&mut self) -> FILL_END_W<0> {
        FILL_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Fill Data End Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_fill_end](index.html) module"]
pub struct TV_FILL_END_SPEC;
impl crate::RegisterSpec for TV_FILL_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_fill_end::R](R) reader structure"]
impl crate::Readable for TV_FILL_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_fill_end::W](W) writer structure"]
impl crate::Writable for TV_FILL_END_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_fill_end%s to value 0"]
impl crate::Resettable for TV_FILL_END_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
