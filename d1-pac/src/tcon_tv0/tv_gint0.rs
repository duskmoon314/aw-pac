#[doc = "Register `tv_gint0` reader"]
pub struct R(crate::R<TV_GINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_GINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_GINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_GINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_gint0` writer"]
pub struct W(crate::W<TV_GINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_GINT0_SPEC>;
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
impl From<crate::W<TV_GINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_GINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tv_line_int_flag` reader - TV Line Interrupt Flag\n\nTrigger when SY1 match the current TV scan line\n\nWrite 0 to clear it."]
pub type TV_LINE_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `tv_line_int_flag` writer - TV Line Interrupt Flag\n\nTrigger when SY1 match the current TV scan line\n\nWrite 0 to clear it."]
pub type TV_LINE_INT_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TV_GINT0_SPEC, bool, O>;
#[doc = "Field `tv_vb_int_flag` reader - TV Vb Interrupt Flag\n\nAsserted during vertical no-display period every frame.\n\nWrite 0 to clear it."]
pub type TV_VB_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `tv_vb_int_flag` writer - TV Vb Interrupt Flag\n\nAsserted during vertical no-display period every frame.\n\nWrite 0 to clear it."]
pub type TV_VB_INT_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TV_GINT0_SPEC, bool, O>;
#[doc = "Field `tv_line_int_en` reader - TV Line Interrupt Enable"]
pub type TV_LINE_INT_EN_R = crate::BitReader<TV_LINE_INT_EN_A>;
#[doc = "TV Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_LINE_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_LINE_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_LINE_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_LINE_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TV_LINE_INT_EN_A {
        match self.bits {
            false => TV_LINE_INT_EN_A::DISABLE,
            true => TV_LINE_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TV_LINE_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_LINE_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tv_line_int_en` writer - TV Line Interrupt Enable"]
pub type TV_LINE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_GINT0_SPEC, TV_LINE_INT_EN_A, O>;
impl<'a, const O: u8> TV_LINE_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TV_LINE_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TV_LINE_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tv_vb_int_en` reader - TV Vb Interrupt Enable"]
pub type TV_VB_INT_EN_R = crate::BitReader<TV_VB_INT_EN_A>;
#[doc = "TV Vb Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_VB_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_VB_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_VB_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_VB_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TV_VB_INT_EN_A {
        match self.bits {
            false => TV_VB_INT_EN_A::DISABLE,
            true => TV_VB_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TV_VB_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_VB_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tv_vb_int_en` writer - TV Vb Interrupt Enable"]
pub type TV_VB_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_GINT0_SPEC, TV_VB_INT_EN_A, O>;
impl<'a, const O: u8> TV_VB_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TV_VB_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TV_VB_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 12 - TV Line Interrupt Flag\n\nTrigger when SY1 match the current TV scan line\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn tv_line_int_flag(&self) -> TV_LINE_INT_FLAG_R {
        TV_LINE_INT_FLAG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TV Vb Interrupt Flag\n\nAsserted during vertical no-display period every frame.\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn tv_vb_int_flag(&self) -> TV_VB_INT_FLAG_R {
        TV_VB_INT_FLAG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 28 - TV Line Interrupt Enable"]
    #[inline(always)]
    pub fn tv_line_int_en(&self) -> TV_LINE_INT_EN_R {
        TV_LINE_INT_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - TV Vb Interrupt Enable"]
    #[inline(always)]
    pub fn tv_vb_int_en(&self) -> TV_VB_INT_EN_R {
        TV_VB_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - TV Line Interrupt Flag\n\nTrigger when SY1 match the current TV scan line\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn tv_line_int_flag(&mut self) -> TV_LINE_INT_FLAG_W<12> {
        TV_LINE_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 14 - TV Vb Interrupt Flag\n\nAsserted during vertical no-display period every frame.\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn tv_vb_int_flag(&mut self) -> TV_VB_INT_FLAG_W<14> {
        TV_VB_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 28 - TV Line Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tv_line_int_en(&mut self) -> TV_LINE_INT_EN_W<28> {
        TV_LINE_INT_EN_W::new(self)
    }
    #[doc = "Bit 30 - TV Vb Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tv_vb_int_en(&mut self) -> TV_VB_INT_EN_W<30> {
        TV_VB_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Global Interrupt Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_gint0](index.html) module"]
pub struct TV_GINT0_SPEC;
impl crate::RegisterSpec for TV_GINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_gint0::R](R) reader structure"]
impl crate::Readable for TV_GINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_gint0::W](W) writer structure"]
impl crate::Writable for TV_GINT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_gint0 to value 0"]
impl crate::Resettable for TV_GINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
