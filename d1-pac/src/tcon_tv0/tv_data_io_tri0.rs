#[doc = "Register `tv_data_io_tri0` reader"]
pub struct R(crate::R<TV_DATA_IO_TRI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_DATA_IO_TRI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_DATA_IO_TRI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_DATA_IO_TRI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_data_io_tri0` writer"]
pub struct W(crate::W<TV_DATA_IO_TRI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_DATA_IO_TRI0_SPEC>;
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
impl From<crate::W<TV_DATA_IO_TRI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_DATA_IO_TRI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `g_y_ch_data_out_tri_en` reader - G Y Channel Data Output Trigger Enable"]
pub type G_Y_CH_DATA_OUT_TRI_EN_R = crate::FieldReader<u16, G_Y_CH_DATA_OUT_TRI_EN_A>;
#[doc = "G Y Channel Data Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum G_Y_CH_DATA_OUT_TRI_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<G_Y_CH_DATA_OUT_TRI_EN_A> for u16 {
    #[inline(always)]
    fn from(variant: G_Y_CH_DATA_OUT_TRI_EN_A) -> Self {
        variant as _
    }
}
impl G_Y_CH_DATA_OUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<G_Y_CH_DATA_OUT_TRI_EN_A> {
        match self.bits {
            0 => Some(G_Y_CH_DATA_OUT_TRI_EN_A::DISABLE),
            1 => Some(G_Y_CH_DATA_OUT_TRI_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == G_Y_CH_DATA_OUT_TRI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == G_Y_CH_DATA_OUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `g_y_ch_data_out_tri_en` writer - G Y Channel Data Output Trigger Enable"]
pub type G_Y_CH_DATA_OUT_TRI_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_DATA_IO_TRI0_SPEC, u16, G_Y_CH_DATA_OUT_TRI_EN_A, 10, O>;
impl<'a, const O: u8> G_Y_CH_DATA_OUT_TRI_EN_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(G_Y_CH_DATA_OUT_TRI_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(G_Y_CH_DATA_OUT_TRI_EN_A::ENABLE)
    }
}
#[doc = "Field `r_cb_ch_data_out_tri_en` reader - R CB Channel Data Output Trigger Enable"]
pub type R_CB_CH_DATA_OUT_TRI_EN_R = crate::FieldReader<u16, R_CB_CH_DATA_OUT_TRI_EN_A>;
#[doc = "R CB Channel Data Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum R_CB_CH_DATA_OUT_TRI_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<R_CB_CH_DATA_OUT_TRI_EN_A> for u16 {
    #[inline(always)]
    fn from(variant: R_CB_CH_DATA_OUT_TRI_EN_A) -> Self {
        variant as _
    }
}
impl R_CB_CH_DATA_OUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<R_CB_CH_DATA_OUT_TRI_EN_A> {
        match self.bits {
            0 => Some(R_CB_CH_DATA_OUT_TRI_EN_A::DISABLE),
            1 => Some(R_CB_CH_DATA_OUT_TRI_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == R_CB_CH_DATA_OUT_TRI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == R_CB_CH_DATA_OUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `r_cb_ch_data_out_tri_en` writer - R CB Channel Data Output Trigger Enable"]
pub type R_CB_CH_DATA_OUT_TRI_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_DATA_IO_TRI0_SPEC, u16, R_CB_CH_DATA_OUT_TRI_EN_A, 10, O>;
impl<'a, const O: u8> R_CB_CH_DATA_OUT_TRI_EN_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(R_CB_CH_DATA_OUT_TRI_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(R_CB_CH_DATA_OUT_TRI_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:9 - G Y Channel Data Output Trigger Enable"]
    #[inline(always)]
    pub fn g_y_ch_data_out_tri_en(&self) -> G_Y_CH_DATA_OUT_TRI_EN_R {
        G_Y_CH_DATA_OUT_TRI_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - R CB Channel Data Output Trigger Enable"]
    #[inline(always)]
    pub fn r_cb_ch_data_out_tri_en(&self) -> R_CB_CH_DATA_OUT_TRI_EN_R {
        R_CB_CH_DATA_OUT_TRI_EN_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - G Y Channel Data Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn g_y_ch_data_out_tri_en(&mut self) -> G_Y_CH_DATA_OUT_TRI_EN_W<0> {
        G_Y_CH_DATA_OUT_TRI_EN_W::new(self)
    }
    #[doc = "Bits 16:25 - R CB Channel Data Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn r_cb_ch_data_out_tri_en(&mut self) -> R_CB_CH_DATA_OUT_TRI_EN_W<16> {
        R_CB_CH_DATA_OUT_TRI_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Data IO Enable Control0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_data_io_tri0](index.html) module"]
pub struct TV_DATA_IO_TRI0_SPEC;
impl crate::RegisterSpec for TV_DATA_IO_TRI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_data_io_tri0::R](R) reader structure"]
impl crate::Readable for TV_DATA_IO_TRI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_data_io_tri0::W](W) writer structure"]
impl crate::Writable for TV_DATA_IO_TRI0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_data_io_tri0 to value 0"]
impl crate::Resettable for TV_DATA_IO_TRI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
