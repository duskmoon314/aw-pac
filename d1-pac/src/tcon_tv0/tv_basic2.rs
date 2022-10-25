#[doc = "Register `tv_basic2` reader"]
pub struct R(crate::R<TV_BASIC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_BASIC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_BASIC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_BASIC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_basic2` writer"]
pub struct W(crate::W<TV_BASIC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_BASIC2_SPEC>;
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
impl From<crate::W<TV_BASIC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_BASIC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tv_yo` reader - Height is TV_YO+1"]
pub type TV_YO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tv_yo` writer - Height is TV_YO+1"]
pub type TV_YO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC2_SPEC, u16, u16, 12, O>;
#[doc = "Field `tv_xo` reader - Width is TV_XO+1"]
pub type TV_XO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tv_xo` writer - Width is TV_XO+1"]
pub type TV_XO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Height is TV_YO+1"]
    #[inline(always)]
    pub fn tv_yo(&self) -> TV_YO_R {
        TV_YO_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Width is TV_XO+1"]
    #[inline(always)]
    pub fn tv_xo(&self) -> TV_XO_R {
        TV_XO_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Height is TV_YO+1"]
    #[inline(always)]
    #[must_use]
    pub fn tv_yo(&mut self) -> TV_YO_W<0> {
        TV_YO_W::new(self)
    }
    #[doc = "Bits 16:27 - Width is TV_XO+1"]
    #[inline(always)]
    #[must_use]
    pub fn tv_xo(&mut self) -> TV_XO_W<16> {
        TV_XO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Basic Timing Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_basic2](index.html) module"]
pub struct TV_BASIC2_SPEC;
impl crate::RegisterSpec for TV_BASIC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_basic2::R](R) reader structure"]
impl crate::Readable for TV_BASIC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_basic2::W](W) writer structure"]
impl crate::Writable for TV_BASIC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic2 to value 0"]
impl crate::Resettable for TV_BASIC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
