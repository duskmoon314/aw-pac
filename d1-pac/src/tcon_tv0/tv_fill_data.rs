#[doc = "Register `tv_fill_data%s` reader"]
pub struct R(crate::R<TV_FILL_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_FILL_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_FILL_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_FILL_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_fill_data%s` writer"]
pub struct W(crate::W<TV_FILL_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_FILL_DATA_SPEC>;
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
impl From<crate::W<TV_FILL_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_FILL_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fill_value` reader - Fill Value"]
pub type FILL_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `fill_value` writer - Fill Value"]
pub type FILL_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_FILL_DATA_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Fill Value"]
    #[inline(always)]
    pub fn fill_value(&self) -> FILL_VALUE_R {
        FILL_VALUE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Fill Value"]
    #[inline(always)]
    #[must_use]
    pub fn fill_value(&mut self) -> FILL_VALUE_W<0> {
        FILL_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Fill Data Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_fill_data](index.html) module"]
pub struct TV_FILL_DATA_SPEC;
impl crate::RegisterSpec for TV_FILL_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_fill_data::R](R) reader structure"]
impl crate::Readable for TV_FILL_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_fill_data::W](W) writer structure"]
impl crate::Writable for TV_FILL_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_fill_data%s to value 0"]
impl crate::Resettable for TV_FILL_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
