#[doc = "Register `fcr` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fifoe` writer - "]
pub type FIFOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `rfifor` writer - "]
pub type RFIFOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `xfifor` writer - "]
pub type XFIFOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAM_AW {
    #[doc = "0: `0`"]
    MODE_0 = 0,
    #[doc = "1: `1`"]
    MODE_1 = 1,
}
impl From<DMAM_AW> for bool {
    #[inline(always)]
    fn from(variant: DMAM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dmam` writer - "]
pub type DMAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, DMAM_AW, O>;
impl<'a, const O: u8> DMAM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(DMAM_AW::MODE_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(DMAM_AW::MODE_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFT_AW {
    #[doc = "0: `0`"]
    EMPTY = 0,
    #[doc = "1: `1`"]
    TWO_CHARACTERS = 1,
    #[doc = "2: `10`"]
    QUARTER_FULL = 2,
    #[doc = "3: `11`"]
    HALF_FULL = 3,
}
impl From<TFT_AW> for u8 {
    #[inline(always)]
    fn from(variant: TFT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `tft` writer - "]
pub type TFT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u8, TFT_AW, 2, O>;
impl<'a, const O: u8> TFT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TFT_AW::EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_characters(self) -> &'a mut W {
        self.variant(TFT_AW::TWO_CHARACTERS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut W {
        self.variant(TFT_AW::QUARTER_FULL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut W {
        self.variant(TFT_AW::HALF_FULL)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT_AW {
    #[doc = "0: `0`"]
    ONE_CHARACTER = 0,
    #[doc = "1: `1`"]
    QUARTER_FULL = 1,
    #[doc = "2: `10`"]
    HALF_FULL = 2,
    #[doc = "3: `11`"]
    TWO_LESS_THAN_FULL = 3,
}
impl From<RT_AW> for u8 {
    #[inline(always)]
    fn from(variant: RT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `rt` writer - "]
pub type RT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u8, RT_AW, 2, O>;
impl<'a, const O: u8> RT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_character(self) -> &'a mut W {
        self.variant(RT_AW::ONE_CHARACTER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut W {
        self.variant(RT_AW::QUARTER_FULL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut W {
        self.variant(RT_AW::HALF_FULL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn two_less_than_full(self) -> &'a mut W {
        self.variant(RT_AW::TWO_LESS_THAN_FULL)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifoe(&mut self) -> FIFOE_W<0> {
        FIFOE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rfifor(&mut self) -> RFIFOR_W<1> {
        RFIFOR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xfifor(&mut self) -> XFIFOR_W<2> {
        XFIFOR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dmam(&mut self) -> DMAM_W<3> {
        DMAM_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn tft(&mut self) -> TFT_W<4> {
        TFT_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RT_W<6> {
        RT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fcr to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
