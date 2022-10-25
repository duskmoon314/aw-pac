#[doc = "Register `smhc_fifo` reader"]
pub struct R(crate::R<SMHC_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_fifo` writer"]
pub struct W(crate::W<SMHC_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_FIFO_SPEC>;
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
impl From<crate::W<SMHC_FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_FIFO_SPEC>) -> Self {
        W(writer)
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
#[doc = "Read/Write FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_fifo](index.html) module"]
pub struct SMHC_FIFO_SPEC;
impl crate::RegisterSpec for SMHC_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_fifo::R](R) reader structure"]
impl crate::Readable for SMHC_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_fifo::W](W) writer structure"]
impl crate::Writable for SMHC_FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_fifo to value 0"]
impl crate::Resettable for SMHC_FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
