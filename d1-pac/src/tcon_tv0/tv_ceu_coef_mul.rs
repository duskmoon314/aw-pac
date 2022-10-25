#[doc = "Register `tv_ceu_coef_mul%s` reader"]
pub struct R(crate::R<TV_CEU_COEF_MUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_CEU_COEF_MUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_CEU_COEF_MUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_CEU_COEF_MUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_ceu_coef_mul%s` writer"]
pub struct W(crate::W<TV_CEU_COEF_MUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_CEU_COEF_MUL_SPEC>;
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
impl From<crate::W<TV_CEU_COEF_MUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_CEU_COEF_MUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ceu_coef_mul_value` reader - Note: CEU_Coef_Mul_Value only can be 0 or 1."]
pub type CEU_COEF_MUL_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `ceu_coef_mul_value` writer - Note: CEU_Coef_Mul_Value only can be 0 or 1."]
pub type CEU_COEF_MUL_VALUE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_CEU_COEF_MUL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Note: CEU_Coef_Mul_Value only can be 0 or 1."]
    #[inline(always)]
    pub fn ceu_coef_mul_value(&self) -> CEU_COEF_MUL_VALUE_R {
        CEU_COEF_MUL_VALUE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Note: CEU_Coef_Mul_Value only can be 0 or 1."]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_mul_value(&mut self) -> CEU_COEF_MUL_VALUE_W<8> {
        CEU_COEF_MUL_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV CEU Coefficient Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_ceu_coef_mul](index.html) module"]
pub struct TV_CEU_COEF_MUL_SPEC;
impl crate::RegisterSpec for TV_CEU_COEF_MUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_ceu_coef_mul::R](R) reader structure"]
impl crate::Readable for TV_CEU_COEF_MUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_ceu_coef_mul::W](W) writer structure"]
impl crate::Writable for TV_CEU_COEF_MUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_ceu_coef_mul%s to value 0"]
impl crate::Resettable for TV_CEU_COEF_MUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
