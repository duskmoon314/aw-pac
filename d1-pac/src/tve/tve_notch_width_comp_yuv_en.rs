#[doc = "Register `tve_notch_width_comp_yuv_en` reader"]
pub struct R(crate::R<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_notch_width_comp_yuv_en` writer"]
pub struct W(crate::W<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>;
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
impl From<crate::W<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `comp_yuv_en` reader - This bit selects if the components video output are the RGB components or the YUV components."]
pub type COMP_YUV_EN_R = crate::BitReader<COMP_YUV_EN_A>;
#[doc = "This bit selects if the components video output are the RGB components or the YUV components.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP_YUV_EN_A {
    #[doc = "0: The three component outputs are the RGB components."]
    RGB = 0,
    #[doc = "1: The three component outputs are the YUV components, (i.e. the color conversion unit is bypassed)"]
    YUV = 1,
}
impl From<COMP_YUV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_YUV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP_YUV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_YUV_EN_A {
        match self.bits {
            false => COMP_YUV_EN_A::RGB,
            true => COMP_YUV_EN_A::YUV,
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == COMP_YUV_EN_A::RGB
    }
    #[doc = "Checks if the value of the field is `YUV`"]
    #[inline(always)]
    pub fn is_yuv(&self) -> bool {
        *self == COMP_YUV_EN_A::YUV
    }
}
#[doc = "Field `comp_yuv_en` writer - This bit selects if the components video output are the RGB components or the YUV components."]
pub type COMP_YUV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC, COMP_YUV_EN_A, O>;
impl<'a, const O: u8> COMP_YUV_EN_W<'a, O> {
    #[doc = "The three component outputs are the RGB components."]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(COMP_YUV_EN_A::RGB)
    }
    #[doc = "The three component outputs are the YUV components, (i.e. the color conversion unit is bypassed)"]
    #[inline(always)]
    pub fn yuv(self) -> &'a mut W {
        self.variant(COMP_YUV_EN_A::YUV)
    }
}
#[doc = "Field `notch_width` reader - Luma notch filter width selection\n\nThis bit selects the luma notch filter (which is a band-reject filter) width."]
pub type NOTCH_WIDTH_R = crate::BitReader<NOTCH_WIDTH_A>;
#[doc = "Luma notch filter width selection\n\nThis bit selects the luma notch filter (which is a band-reject filter) width.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTCH_WIDTH_A {
    #[doc = "0: Narrow"]
    N_ARROW = 0,
    #[doc = "1: Wide"]
    W_IDE = 1,
}
impl From<NOTCH_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: NOTCH_WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTCH_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTCH_WIDTH_A {
        match self.bits {
            false => NOTCH_WIDTH_A::N_ARROW,
            true => NOTCH_WIDTH_A::W_IDE,
        }
    }
    #[doc = "Checks if the value of the field is `N_ARROW`"]
    #[inline(always)]
    pub fn is_n_arrow(&self) -> bool {
        *self == NOTCH_WIDTH_A::N_ARROW
    }
    #[doc = "Checks if the value of the field is `W_IDE`"]
    #[inline(always)]
    pub fn is_w_ide(&self) -> bool {
        *self == NOTCH_WIDTH_A::W_IDE
    }
}
#[doc = "Field `notch_width` writer - Luma notch filter width selection\n\nThis bit selects the luma notch filter (which is a band-reject filter) width."]
pub type NOTCH_WIDTH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC, NOTCH_WIDTH_A, O>;
impl<'a, const O: u8> NOTCH_WIDTH_W<'a, O> {
    #[doc = "Narrow"]
    #[inline(always)]
    pub fn n_arrow(self) -> &'a mut W {
        self.variant(NOTCH_WIDTH_A::N_ARROW)
    }
    #[doc = "Wide"]
    #[inline(always)]
    pub fn w_ide(self) -> &'a mut W {
        self.variant(NOTCH_WIDTH_A::W_IDE)
    }
}
impl R {
    #[doc = "Bit 0 - This bit selects if the components video output are the RGB components or the YUV components."]
    #[inline(always)]
    pub fn comp_yuv_en(&self) -> COMP_YUV_EN_R {
        COMP_YUV_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Luma notch filter width selection\n\nThis bit selects the luma notch filter (which is a band-reject filter) width."]
    #[inline(always)]
    pub fn notch_width(&self) -> NOTCH_WIDTH_R {
        NOTCH_WIDTH_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects if the components video output are the RGB components or the YUV components."]
    #[inline(always)]
    #[must_use]
    pub fn comp_yuv_en(&mut self) -> COMP_YUV_EN_W<0> {
        COMP_YUV_EN_W::new(self)
    }
    #[doc = "Bit 8 - Luma notch filter width selection\n\nThis bit selects the luma notch filter (which is a band-reject filter) width."]
    #[inline(always)]
    #[must_use]
    pub fn notch_width(&mut self) -> NOTCH_WIDTH_W<8> {
        NOTCH_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_notch_width_comp_yuv_en](index.html) module"]
pub struct TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC;
impl crate::RegisterSpec for TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_notch_width_comp_yuv_en::R](R) reader structure"]
impl crate::Readable for TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_notch_width_comp_yuv_en::W](W) writer structure"]
impl crate::Writable for TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_notch_width_comp_yuv_en to value 0x0101"]
impl crate::Resettable for TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
