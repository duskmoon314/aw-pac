#[doc = "Register `asynclistaddr` reader"]
pub struct R(crate::R<ASYNCLISTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCLISTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCLISTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCLISTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asynclistaddr` writer"]
pub struct W(crate::W<ASYNCLISTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCLISTADDR_SPEC>;
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
impl From<crate::W<ASYNCLISTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCLISTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lp` reader - Link Pointer (LP)\n\nThis field contains the address of the next asynchronous queue head to be executed.\n\nThese bits correspond to memory address signals \\[31:5\\], respectively"]
pub type LP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `lp` writer - Link Pointer (LP)\n\nThis field contains the address of the next asynchronous queue head to be executed.\n\nThese bits correspond to memory address signals \\[31:5\\], respectively"]
pub type LP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASYNCLISTADDR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 5:31 - Link Pointer (LP)\n\nThis field contains the address of the next asynchronous queue head to be executed.\n\nThese bits correspond to memory address signals \\[31:5\\], respectively"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Link Pointer (LP)\n\nThis field contains the address of the next asynchronous queue head to be executed.\n\nThese bits correspond to memory address signals \\[31:5\\], respectively"]
    #[inline(always)]
    #[must_use]
    pub fn lp(&mut self) -> LP_W<5> {
        LP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI Current Asynchronous List Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asynclistaddr](index.html) module"]
pub struct ASYNCLISTADDR_SPEC;
impl crate::RegisterSpec for ASYNCLISTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asynclistaddr::R](R) reader structure"]
impl crate::Readable for ASYNCLISTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asynclistaddr::W](W) writer structure"]
impl crate::Writable for ASYNCLISTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asynclistaddr to value 0"]
impl crate::Resettable for ASYNCLISTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
