#[doc = "Register `csic_ptn_isp_size` reader"]
pub struct R(crate::R<CSIC_PTN_ISP_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PTN_ISP_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PTN_ISP_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PTN_ISP_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_ptn_isp_size` writer"]
pub struct W(crate::W<CSIC_PTN_ISP_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PTN_ISP_SIZE_SPEC>;
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
impl From<crate::W<CSIC_PTN_ISP_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PTN_ISP_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `width` reader - Width Horizontal size, only valid for ISP mode pattern generation."]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `width` writer - Width Horizontal size, only valid for ISP mode pattern generation."]
pub type WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PTN_ISP_SIZE_SPEC, u16, u16, 13, O>;
#[doc = "Field `height` reader - Height Vertical size, only valid for ISP mode pattern generation."]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `height` writer - Height Vertical size, only valid for ISP mode pattern generation."]
pub type HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PTN_ISP_SIZE_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Width Horizontal size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Height Vertical size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Width Horizontal size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 16:28 - Height Vertical size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<16> {
        HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Pattern ISP Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_ptn_isp_size](index.html) module"]
pub struct CSIC_PTN_ISP_SIZE_SPEC;
impl crate::RegisterSpec for CSIC_PTN_ISP_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_ptn_isp_size::R](R) reader structure"]
impl crate::Readable for CSIC_PTN_ISP_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_ptn_isp_size::W](W) writer structure"]
impl crate::Writable for CSIC_PTN_ISP_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_ptn_isp_size to value 0"]
impl crate::Resettable for CSIC_PTN_ISP_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
