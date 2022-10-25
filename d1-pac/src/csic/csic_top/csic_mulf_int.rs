#[doc = "Register `csic_mulf_int` reader"]
pub struct R(crate::R<CSIC_MULF_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_MULF_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_MULF_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_MULF_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_mulf_int` writer"]
pub struct W(crate::W<CSIC_MULF_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_MULF_INT_SPEC>;
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
impl From<crate::W<CSIC_MULF_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_MULF_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mulf_done_en` reader - "]
pub type MULF_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `mulf_done_en` writer - "]
pub type MULF_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_MULF_INT_SPEC, bool, O>;
#[doc = "Field `mulf_err_en` reader - "]
pub type MULF_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `mulf_err_en` writer - "]
pub type MULF_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_MULF_INT_SPEC, bool, O>;
#[doc = "Field `mulf_done_pd` reader - "]
pub type MULF_DONE_PD_R = crate::BitReader<bool>;
#[doc = "Field `mulf_done_pd` writer - "]
pub type MULF_DONE_PD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CSIC_MULF_INT_SPEC, bool, O>;
#[doc = "Field `mulf_err_pd` reader - "]
pub type MULF_ERR_PD_R = crate::BitReader<bool>;
#[doc = "Field `mulf_err_pd` writer - "]
pub type MULF_ERR_PD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CSIC_MULF_INT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mulf_done_en(&self) -> MULF_DONE_EN_R {
        MULF_DONE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mulf_err_en(&self) -> MULF_ERR_EN_R {
        MULF_ERR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mulf_done_pd(&self) -> MULF_DONE_PD_R {
        MULF_DONE_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mulf_err_pd(&self) -> MULF_ERR_PD_R {
        MULF_ERR_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_done_en(&mut self) -> MULF_DONE_EN_W<0> {
        MULF_DONE_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_err_en(&mut self) -> MULF_ERR_EN_W<1> {
        MULF_ERR_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_done_pd(&mut self) -> MULF_DONE_PD_W<16> {
        MULF_DONE_PD_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_err_pd(&mut self) -> MULF_ERR_PD_W<17> {
        MULF_ERR_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Multi-Frame Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_mulf_int](index.html) module"]
pub struct CSIC_MULF_INT_SPEC;
impl crate::RegisterSpec for CSIC_MULF_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_mulf_int::R](R) reader structure"]
impl crate::Readable for CSIC_MULF_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_mulf_int::W](W) writer structure"]
impl crate::Writable for CSIC_MULF_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0000;
}
#[doc = "`reset()` method sets csic_mulf_int to value 0"]
impl crate::Resettable for CSIC_MULF_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
