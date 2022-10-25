#[doc = "Register `tve_color_burst_phase_reset_cfg` reader"]
pub struct R(crate::R<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_color_burst_phase_reset_cfg` writer"]
pub struct W(crate::W<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>;
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
impl From<crate::W<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `color_phase_reset` reader - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
pub type COLOR_PHASE_RESET_R = crate::FieldReader<u8, COLOR_PHASE_RESET_A>;
#[doc = "Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COLOR_PHASE_RESET_A {
    #[doc = "0: 8 field"]
    LINE8 = 0,
    #[doc = "1: 4 field"]
    LINE4 = 1,
    #[doc = "2: 2 lines"]
    LINE2 = 2,
    #[doc = "3: only once"]
    ONLY_ONCE = 3,
}
impl From<COLOR_PHASE_RESET_A> for u8 {
    #[inline(always)]
    fn from(variant: COLOR_PHASE_RESET_A) -> Self {
        variant as _
    }
}
impl COLOR_PHASE_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLOR_PHASE_RESET_A {
        match self.bits {
            0 => COLOR_PHASE_RESET_A::LINE8,
            1 => COLOR_PHASE_RESET_A::LINE4,
            2 => COLOR_PHASE_RESET_A::LINE2,
            3 => COLOR_PHASE_RESET_A::ONLY_ONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LINE8`"]
    #[inline(always)]
    pub fn is_line8(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::LINE8
    }
    #[doc = "Checks if the value of the field is `LINE4`"]
    #[inline(always)]
    pub fn is_line4(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::LINE4
    }
    #[doc = "Checks if the value of the field is `LINE2`"]
    #[inline(always)]
    pub fn is_line2(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::LINE2
    }
    #[doc = "Checks if the value of the field is `ONLY_ONCE`"]
    #[inline(always)]
    pub fn is_only_once(&self) -> bool {
        *self == COLOR_PHASE_RESET_A::ONLY_ONCE
    }
}
#[doc = "Field `color_phase_reset` writer - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
pub type COLOR_PHASE_RESET_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC,
    u8,
    COLOR_PHASE_RESET_A,
    2,
    O,
>;
impl<'a, const O: u8> COLOR_PHASE_RESET_W<'a, O> {
    #[doc = "8 field"]
    #[inline(always)]
    pub fn line8(self) -> &'a mut W {
        self.variant(COLOR_PHASE_RESET_A::LINE8)
    }
    #[doc = "4 field"]
    #[inline(always)]
    pub fn line4(self) -> &'a mut W {
        self.variant(COLOR_PHASE_RESET_A::LINE4)
    }
    #[doc = "2 lines"]
    #[inline(always)]
    pub fn line2(self) -> &'a mut W {
        self.variant(COLOR_PHASE_RESET_A::LINE2)
    }
    #[doc = "only once"]
    #[inline(always)]
    pub fn only_once(self) -> &'a mut W {
        self.variant(COLOR_PHASE_RESET_A::ONLY_ONCE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
    #[inline(always)]
    pub fn color_phase_reset(&self) -> COLOR_PHASE_RESET_R {
        COLOR_PHASE_RESET_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Color burst phase period selection\n\nThese bits select the number of fields or lines after which the color burst phase is reset to its initial value as specified by the ChromaPhase parameter, This parameter is application only for interlaced video."]
    #[inline(always)]
    #[must_use]
    pub fn color_phase_reset(&mut self) -> COLOR_PHASE_RESET_W<0> {
        COLOR_PHASE_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Color Burst Phase Reset Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_color_burst_phase_reset_cfg](index.html) module"]
pub struct TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC;
impl crate::RegisterSpec for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_color_burst_phase_reset_cfg::R](R) reader structure"]
impl crate::Readable for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_color_burst_phase_reset_cfg::W](W) writer structure"]
impl crate::Writable for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_color_burst_phase_reset_cfg to value 0"]
impl crate::Resettable for TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
