#[doc = "Register `tv_data_io_tri1` reader"]
pub struct R(crate::R<TV_DATA_IO_TRI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_DATA_IO_TRI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_DATA_IO_TRI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_DATA_IO_TRI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_data_io_tri1` writer"]
pub struct W(crate::W<TV_DATA_IO_TRI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_DATA_IO_TRI1_SPEC>;
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
impl From<crate::W<TV_DATA_IO_TRI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_DATA_IO_TRI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b_cr_ch_data_out_tri_en` reader - B CR Channel Data Output Trigger Enable"]
pub type B_CR_CH_DATA_OUT_TRI_EN_R = crate::FieldReader<u16, B_CR_CH_DATA_OUT_TRI_EN_A>;
#[doc = "B CR Channel Data Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum B_CR_CH_DATA_OUT_TRI_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<B_CR_CH_DATA_OUT_TRI_EN_A> for u16 {
    #[inline(always)]
    fn from(variant: B_CR_CH_DATA_OUT_TRI_EN_A) -> Self {
        variant as _
    }
}
impl B_CR_CH_DATA_OUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<B_CR_CH_DATA_OUT_TRI_EN_A> {
        match self.bits {
            0 => Some(B_CR_CH_DATA_OUT_TRI_EN_A::DISABLE),
            1 => Some(B_CR_CH_DATA_OUT_TRI_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == B_CR_CH_DATA_OUT_TRI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == B_CR_CH_DATA_OUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `b_cr_ch_data_out_tri_en` writer - B CR Channel Data Output Trigger Enable"]
pub type B_CR_CH_DATA_OUT_TRI_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_DATA_IO_TRI1_SPEC, u16, B_CR_CH_DATA_OUT_TRI_EN_A, 10, O>;
impl<'a, const O: u8> B_CR_CH_DATA_OUT_TRI_EN_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(B_CR_CH_DATA_OUT_TRI_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(B_CR_CH_DATA_OUT_TRI_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 16:25 - B CR Channel Data Output Trigger Enable"]
    #[inline(always)]
    pub fn b_cr_ch_data_out_tri_en(&self) -> B_CR_CH_DATA_OUT_TRI_EN_R {
        B_CR_CH_DATA_OUT_TRI_EN_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - B CR Channel Data Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b_cr_ch_data_out_tri_en(&mut self) -> B_CR_CH_DATA_OUT_TRI_EN_W<16> {
        B_CR_CH_DATA_OUT_TRI_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Data IO Enable Control1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_data_io_tri1](index.html) module"]
pub struct TV_DATA_IO_TRI1_SPEC;
impl crate::RegisterSpec for TV_DATA_IO_TRI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_data_io_tri1::R](R) reader structure"]
impl crate::Readable for TV_DATA_IO_TRI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_data_io_tri1::W](W) writer structure"]
impl crate::Writable for TV_DATA_IO_TRI1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_data_io_tri1 to value 0"]
impl crate::Resettable for TV_DATA_IO_TRI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
