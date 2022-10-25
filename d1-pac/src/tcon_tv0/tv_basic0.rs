#[doc = "Register `tv_basic0` reader"]
pub struct R(crate::R<TV_BASIC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_BASIC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_BASIC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_BASIC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_basic0` writer"]
pub struct W(crate::W<TV_BASIC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_BASIC0_SPEC>;
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
impl From<crate::W<TV_BASIC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_BASIC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `height_yi` reader - Source Height Is Y+1"]
pub type HEIGHT_YI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `height_yi` writer - Source Height Is Y+1"]
pub type HEIGHT_YI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_BASIC0_SPEC, u16, u16, 12, O>;
#[doc = "Field `width_xi` reader - Source Width Is X+1"]
pub type WIDTH_XI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `width_xi` writer - Source Width Is X+1"]
pub type WIDTH_XI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Source Height Is Y+1"]
    #[inline(always)]
    pub fn height_yi(&self) -> HEIGHT_YI_R {
        HEIGHT_YI_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Source Width Is X+1"]
    #[inline(always)]
    pub fn width_xi(&self) -> WIDTH_XI_R {
        WIDTH_XI_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Source Height Is Y+1"]
    #[inline(always)]
    #[must_use]
    pub fn height_yi(&mut self) -> HEIGHT_YI_W<0> {
        HEIGHT_YI_W::new(self)
    }
    #[doc = "Bits 16:27 - Source Width Is X+1"]
    #[inline(always)]
    #[must_use]
    pub fn width_xi(&mut self) -> WIDTH_XI_W<16> {
        WIDTH_XI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Basic Timing Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_basic0](index.html) module"]
pub struct TV_BASIC0_SPEC;
impl crate::RegisterSpec for TV_BASIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_basic0::R](R) reader structure"]
impl crate::Readable for TV_BASIC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_basic0::W](W) writer structure"]
impl crate::Writable for TV_BASIC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic0 to value 0"]
impl crate::Resettable for TV_BASIC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
