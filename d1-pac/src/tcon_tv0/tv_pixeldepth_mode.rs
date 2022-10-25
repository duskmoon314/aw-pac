#[doc = "Register `tv_pixeldepth_mode` reader"]
pub struct R(crate::R<TV_PIXELDEPTH_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_PIXELDEPTH_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_PIXELDEPTH_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_PIXELDEPTH_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_pixeldepth_mode` writer"]
pub struct W(crate::W<TV_PIXELDEPTH_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_PIXELDEPTH_MODE_SPEC>;
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
impl From<crate::W<TV_PIXELDEPTH_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_PIXELDEPTH_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `colorbar_pd_mode` reader - Colorbar Pixeldepth mode"]
pub type COLORBAR_PD_MODE_R = crate::BitReader<COLORBAR_PD_MODE_A>;
#[doc = "Colorbar Pixeldepth mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLORBAR_PD_MODE_A {
    #[doc = "0: 8 bit mode\n\nWhen data source is the embedded colorbar, the 8-bit colorbar pattern is transmitted."]
    B_IT8 = 0,
    #[doc = "1: 10 bit mode\n\nWhen data source is the embedded colorbar, the 10-bit colorbar pattern is transmitted."]
    B_IT10 = 1,
}
impl From<COLORBAR_PD_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: COLORBAR_PD_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl COLORBAR_PD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLORBAR_PD_MODE_A {
        match self.bits {
            false => COLORBAR_PD_MODE_A::B_IT8,
            true => COLORBAR_PD_MODE_A::B_IT10,
        }
    }
    #[doc = "Checks if the value of the field is `B_IT8`"]
    #[inline(always)]
    pub fn is_b_it8(&self) -> bool {
        *self == COLORBAR_PD_MODE_A::B_IT8
    }
    #[doc = "Checks if the value of the field is `B_IT10`"]
    #[inline(always)]
    pub fn is_b_it10(&self) -> bool {
        *self == COLORBAR_PD_MODE_A::B_IT10
    }
}
#[doc = "Field `colorbar_pd_mode` writer - Colorbar Pixeldepth mode"]
pub type COLORBAR_PD_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_PIXELDEPTH_MODE_SPEC, COLORBAR_PD_MODE_A, O>;
impl<'a, const O: u8> COLORBAR_PD_MODE_W<'a, O> {
    #[doc = "8 bit mode\n\nWhen data source is the embedded colorbar, the 8-bit colorbar pattern is transmitted."]
    #[inline(always)]
    pub fn b_it8(self) -> &'a mut W {
        self.variant(COLORBAR_PD_MODE_A::B_IT8)
    }
    #[doc = "10 bit mode\n\nWhen data source is the embedded colorbar, the 10-bit colorbar pattern is transmitted."]
    #[inline(always)]
    pub fn b_it10(self) -> &'a mut W {
        self.variant(COLORBAR_PD_MODE_A::B_IT10)
    }
}
impl R {
    #[doc = "Bit 0 - Colorbar Pixeldepth mode"]
    #[inline(always)]
    pub fn colorbar_pd_mode(&self) -> COLORBAR_PD_MODE_R {
        COLORBAR_PD_MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Colorbar Pixeldepth mode"]
    #[inline(always)]
    #[must_use]
    pub fn colorbar_pd_mode(&mut self) -> COLORBAR_PD_MODE_W<0> {
        COLORBAR_PD_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Pixeldepth Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_pixeldepth_mode](index.html) module"]
pub struct TV_PIXELDEPTH_MODE_SPEC;
impl crate::RegisterSpec for TV_PIXELDEPTH_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_pixeldepth_mode::R](R) reader structure"]
impl crate::Readable for TV_PIXELDEPTH_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_pixeldepth_mode::W](W) writer structure"]
impl crate::Writable for TV_PIXELDEPTH_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_pixeldepth_mode to value 0"]
impl crate::Resettable for TV_PIXELDEPTH_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
