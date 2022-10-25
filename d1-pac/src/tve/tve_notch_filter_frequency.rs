#[doc = "Register `tve_notch_filter_frequency` reader"]
pub struct R(crate::R<TVE_NOTCH_FILTER_FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_NOTCH_FILTER_FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_NOTCH_FILTER_FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_NOTCH_FILTER_FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_notch_filter_frequency` writer"]
pub struct W(crate::W<TVE_NOTCH_FILTER_FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_NOTCH_FILTER_FREQUENCY_SPEC>;
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
impl From<crate::W<TVE_NOTCH_FILTER_FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_NOTCH_FILTER_FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `notch_freq` reader - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
pub type NOTCH_FREQ_R = crate::FieldReader<u8, NOTCH_FREQ_A>;
#[doc = "Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NOTCH_FREQ_A {
    #[doc = "0: 1.1875"]
    _1_1875 = 0,
    #[doc = "1: 1.1406"]
    _1_1406 = 1,
    #[doc = "2: 1.0938. When notch_wide value is B'1' (this selection is proper for CCIR-NTSC), or 1.0000 when notch_wide value is B'0'."]
    _1_0938 = 2,
    #[doc = "3: 0.9922. This selection is proper for NTSC with square pixels."]
    _0_9922 = 3,
    #[doc = "4: 0.9531. This selection is proper for PAL with square pixel."]
    _0_9531 = 4,
    #[doc = "5: 0.8359 when notch_wide value is B'1' (this selection is proper for CCIR-PAL), or 0.7734 when notch_wide value is B'0'."]
    _0_8359 = 5,
    #[doc = "6: 0.7813"]
    _0_7813 = 6,
    #[doc = "7: 0.7188"]
    _0_7188 = 7,
}
impl From<NOTCH_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: NOTCH_FREQ_A) -> Self {
        variant as _
    }
}
impl NOTCH_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTCH_FREQ_A {
        match self.bits {
            0 => NOTCH_FREQ_A::_1_1875,
            1 => NOTCH_FREQ_A::_1_1406,
            2 => NOTCH_FREQ_A::_1_0938,
            3 => NOTCH_FREQ_A::_0_9922,
            4 => NOTCH_FREQ_A::_0_9531,
            5 => NOTCH_FREQ_A::_0_8359,
            6 => NOTCH_FREQ_A::_0_7813,
            7 => NOTCH_FREQ_A::_0_7188,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_1875`"]
    #[inline(always)]
    pub fn is_1_1875(&self) -> bool {
        *self == NOTCH_FREQ_A::_1_1875
    }
    #[doc = "Checks if the value of the field is `_1_1406`"]
    #[inline(always)]
    pub fn is_1_1406(&self) -> bool {
        *self == NOTCH_FREQ_A::_1_1406
    }
    #[doc = "Checks if the value of the field is `_1_0938`"]
    #[inline(always)]
    pub fn is_1_0938(&self) -> bool {
        *self == NOTCH_FREQ_A::_1_0938
    }
    #[doc = "Checks if the value of the field is `_0_9922`"]
    #[inline(always)]
    pub fn is_0_9922(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_9922
    }
    #[doc = "Checks if the value of the field is `_0_9531`"]
    #[inline(always)]
    pub fn is_0_9531(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_9531
    }
    #[doc = "Checks if the value of the field is `_0_8359`"]
    #[inline(always)]
    pub fn is_0_8359(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_8359
    }
    #[doc = "Checks if the value of the field is `_0_7813`"]
    #[inline(always)]
    pub fn is_0_7813(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_7813
    }
    #[doc = "Checks if the value of the field is `_0_7188`"]
    #[inline(always)]
    pub fn is_0_7188(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_7188
    }
}
#[doc = "Field `notch_freq` writer - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
pub type NOTCH_FREQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TVE_NOTCH_FILTER_FREQUENCY_SPEC, u8, NOTCH_FREQ_A, 3, O>;
impl<'a, const O: u8> NOTCH_FREQ_W<'a, O> {
    #[doc = "1.1875"]
    #[inline(always)]
    pub fn _1_1875(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_1_1875)
    }
    #[doc = "1.1406"]
    #[inline(always)]
    pub fn _1_1406(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_1_1406)
    }
    #[doc = "1.0938. When notch_wide value is B'1' (this selection is proper for CCIR-NTSC), or 1.0000 when notch_wide value is B'0'."]
    #[inline(always)]
    pub fn _1_0938(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_1_0938)
    }
    #[doc = "0.9922. This selection is proper for NTSC with square pixels."]
    #[inline(always)]
    pub fn _0_9922(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_0_9922)
    }
    #[doc = "0.9531. This selection is proper for PAL with square pixel."]
    #[inline(always)]
    pub fn _0_9531(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_0_9531)
    }
    #[doc = "0.8359 when notch_wide value is B'1' (this selection is proper for CCIR-PAL), or 0.7734 when notch_wide value is B'0'."]
    #[inline(always)]
    pub fn _0_8359(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_0_8359)
    }
    #[doc = "0.7813"]
    #[inline(always)]
    pub fn _0_7813(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_0_7813)
    }
    #[doc = "0.7188"]
    #[inline(always)]
    pub fn _0_7188(self) -> &'a mut W {
        self.variant(NOTCH_FREQ_A::_0_7188)
    }
}
impl R {
    #[doc = "Bits 0:2 - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
    #[inline(always)]
    pub fn notch_freq(&self) -> NOTCH_FREQ_R {
        NOTCH_FREQ_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
    #[inline(always)]
    #[must_use]
    pub fn notch_freq(&mut self) -> NOTCH_FREQ_W<0> {
        NOTCH_FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Notch Filter Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_notch_filter_frequency](index.html) module"]
pub struct TVE_NOTCH_FILTER_FREQUENCY_SPEC;
impl crate::RegisterSpec for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_notch_filter_frequency::R](R) reader structure"]
impl crate::Readable for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_notch_filter_frequency::W](W) writer structure"]
impl crate::Writable for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_notch_filter_frequency to value 0x02"]
impl crate::Resettable for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
