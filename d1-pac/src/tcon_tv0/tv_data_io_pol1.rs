#[doc = "Register `tv_data_io_pol1` reader"]
pub struct R(crate::R<TV_DATA_IO_POL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_DATA_IO_POL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_DATA_IO_POL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_DATA_IO_POL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_data_io_pol1` writer"]
pub struct W(crate::W<TV_DATA_IO_POL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_DATA_IO_POL1_SPEC>;
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
impl From<crate::W<TV_DATA_IO_POL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_DATA_IO_POL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b_cr_ch_data_inv` reader - B CR CHANNE DATA INV"]
pub type B_CR_CH_DATA_INV_R = crate::FieldReader<u16, B_CR_CH_DATA_INV_A>;
#[doc = "B CR CHANNE DATA INV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum B_CR_CH_DATA_INV_A {
    #[doc = "0: normal polarity"]
    NORMAL = 0,
    #[doc = "1: invert the specify output"]
    INVERT = 1,
}
impl From<B_CR_CH_DATA_INV_A> for u16 {
    #[inline(always)]
    fn from(variant: B_CR_CH_DATA_INV_A) -> Self {
        variant as _
    }
}
impl B_CR_CH_DATA_INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<B_CR_CH_DATA_INV_A> {
        match self.bits {
            0 => Some(B_CR_CH_DATA_INV_A::NORMAL),
            1 => Some(B_CR_CH_DATA_INV_A::INVERT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == B_CR_CH_DATA_INV_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == B_CR_CH_DATA_INV_A::INVERT
    }
}
#[doc = "Field `b_cr_ch_data_inv` writer - B CR CHANNE DATA INV"]
pub type B_CR_CH_DATA_INV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_DATA_IO_POL1_SPEC, u16, B_CR_CH_DATA_INV_A, 10, O>;
impl<'a, const O: u8> B_CR_CH_DATA_INV_W<'a, O> {
    #[doc = "normal polarity"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(B_CR_CH_DATA_INV_A::NORMAL)
    }
    #[doc = "invert the specify output"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(B_CR_CH_DATA_INV_A::INVERT)
    }
}
impl R {
    #[doc = "Bits 16:25 - B CR CHANNE DATA INV"]
    #[inline(always)]
    pub fn b_cr_ch_data_inv(&self) -> B_CR_CH_DATA_INV_R {
        B_CR_CH_DATA_INV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - B CR CHANNE DATA INV"]
    #[inline(always)]
    #[must_use]
    pub fn b_cr_ch_data_inv(&mut self) -> B_CR_CH_DATA_INV_W<16> {
        B_CR_CH_DATA_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Data IO Polarity Control1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_data_io_pol1](index.html) module"]
pub struct TV_DATA_IO_POL1_SPEC;
impl crate::RegisterSpec for TV_DATA_IO_POL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_data_io_pol1::R](R) reader structure"]
impl crate::Readable for TV_DATA_IO_POL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_data_io_pol1::W](W) writer structure"]
impl crate::Writable for TV_DATA_IO_POL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_data_io_pol1 to value 0"]
impl crate::Resettable for TV_DATA_IO_POL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
