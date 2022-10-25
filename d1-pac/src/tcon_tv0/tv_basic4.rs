#[doc = "Register `tv_basic4` reader"]
pub struct R(crate::R<TV_BASIC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_BASIC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_BASIC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_BASIC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_basic4` writer"]
pub struct W(crate::W<TV_BASIC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_BASIC4_SPEC>;
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
impl From<crate::W<TV_BASIC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_BASIC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `v_bp` reader - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
pub type V_BP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `v_bp` writer - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
pub type V_BP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC4_SPEC, u16, u16, 12, O>;
#[doc = "Field `v_t` reader - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
pub type V_T_R = crate::FieldReader<u16, u16>;
#[doc = "Field `v_t` writer - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
pub type V_T_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC4_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:11 - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
    #[inline(always)]
    pub fn v_bp(&self) -> V_BP_R {
        V_BP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
    #[inline(always)]
    pub fn v_t(&self) -> V_T_R {
        V_T_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
    #[inline(always)]
    #[must_use]
    pub fn v_bp(&mut self) -> V_BP_W<0> {
        V_BP_W::new(self)
    }
    #[doc = "Bits 16:28 - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
    #[inline(always)]
    #[must_use]
    pub fn v_t(&mut self) -> V_T_W<16> {
        V_T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Basic Timing Register4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_basic4](index.html) module"]
pub struct TV_BASIC4_SPEC;
impl crate::RegisterSpec for TV_BASIC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_basic4::R](R) reader structure"]
impl crate::Readable for TV_BASIC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_basic4::W](W) writer structure"]
impl crate::Writable for TV_BASIC4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic4 to value 0"]
impl crate::Resettable for TV_BASIC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
