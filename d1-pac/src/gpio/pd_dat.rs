#[doc = "Register `pd_dat` reader"]
pub struct R(crate::R<PD_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_dat` writer"]
pub struct W(crate::W<PD_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_DAT_SPEC>;
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
impl From<crate::W<PD_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pd_dat` reader - "]
pub type PD_DAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `pd_dat` writer - "]
pub type PD_DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD_DAT_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn pd_dat(&self) -> PD_DAT_R {
        PD_DAT_R::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    #[must_use]
    pub fn pd_dat(&mut self) -> PD_DAT_W<0> {
        PD_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_dat](index.html) module"]
pub struct PD_DAT_SPEC;
impl crate::RegisterSpec for PD_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_dat::R](R) reader structure"]
impl crate::Readable for PD_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_dat::W](W) writer structure"]
impl crate::Writable for PD_DAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_dat to value 0"]
impl crate::Resettable for PD_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
