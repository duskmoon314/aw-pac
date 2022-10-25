#[doc = "Register `tve_configuration1` reader"]
pub struct R(crate::R<TVE_CONFIGURATION1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_CONFIGURATION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_CONFIGURATION1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_CONFIGURATION1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_configuration1` writer"]
pub struct W(crate::W<TVE_CONFIGURATION1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_CONFIGURATION1_SPEC>;
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
impl From<crate::W<TVE_CONFIGURATION1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_CONFIGURATION1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bypass_yclamp` reader - Y input clamping selection\n\nThis bit selects whether the Video Encoder Y input is clamped to 64 to 940 or not. When not clamped the expected range is 0 to 1023. The U and V inputs are always clamped to the range 64 to 960."]
pub type BYPASS_YCLAMP_R = crate::BitReader<BYPASS_YCLAMP_A>;
#[doc = "Y input clamping selection\n\nThis bit selects whether the Video Encoder Y input is clamped to 64 to 940 or not. When not clamped the expected range is 0 to 1023. The U and V inputs are always clamped to the range 64 to 960.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_YCLAMP_A {
    #[doc = "0: The Video Encoder Y input is clamped"]
    CLAMPED = 0,
    #[doc = "1: The Video Encoder Y input is not clamped"]
    NOT_CLAMPED = 1,
}
impl From<BYPASS_YCLAMP_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_YCLAMP_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_YCLAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_YCLAMP_A {
        match self.bits {
            false => BYPASS_YCLAMP_A::CLAMPED,
            true => BYPASS_YCLAMP_A::NOT_CLAMPED,
        }
    }
    #[doc = "Checks if the value of the field is `CLAMPED`"]
    #[inline(always)]
    pub fn is_clamped(&self) -> bool {
        *self == BYPASS_YCLAMP_A::CLAMPED
    }
    #[doc = "Checks if the value of the field is `NOT_CLAMPED`"]
    #[inline(always)]
    pub fn is_not_clamped(&self) -> bool {
        *self == BYPASS_YCLAMP_A::NOT_CLAMPED
    }
}
#[doc = "Field `bypass_yclamp` writer - Y input clamping selection\n\nThis bit selects whether the Video Encoder Y input is clamped to 64 to 940 or not. When not clamped the expected range is 0 to 1023. The U and V inputs are always clamped to the range 64 to 960."]
pub type BYPASS_YCLAMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_CONFIGURATION1_SPEC, BYPASS_YCLAMP_A, O>;
impl<'a, const O: u8> BYPASS_YCLAMP_W<'a, O> {
    #[doc = "The Video Encoder Y input is clamped"]
    #[inline(always)]
    pub fn clamped(self) -> &'a mut W {
        self.variant(BYPASS_YCLAMP_A::CLAMPED)
    }
    #[doc = "The Video Encoder Y input is not clamped"]
    #[inline(always)]
    pub fn not_clamped(self) -> &'a mut W {
        self.variant(BYPASS_YCLAMP_A::NOT_CLAMPED)
    }
}
#[doc = "Field `rgb_setup` reader - \"Set-up\" enable for RGB outputs. This bit specifies if the \"set-up\" implied value (black_level - blank_level) specified for the CVBS signal is used also for the RGB signals."]
pub type RGB_SETUP_R = crate::BitReader<RGB_SETUP_A>;
#[doc = "\"Set-up\" enable for RGB outputs. This bit specifies if the \"set-up\" implied value (black_level - blank_level) specified for the CVBS signal is used also for the RGB signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGB_SETUP_A {
    #[doc = "0: The \"set-up\" is not used, or i.e. comp_yuv is equal to b'1'."]
    NOT_USED = 0,
    #[doc = "1: The implied \"set-up\" is used for the RGB signals"]
    USED = 1,
}
impl From<RGB_SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: RGB_SETUP_A) -> Self {
        variant as u8 != 0
    }
}
impl RGB_SETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_SETUP_A {
        match self.bits {
            false => RGB_SETUP_A::NOT_USED,
            true => RGB_SETUP_A::USED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == RGB_SETUP_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == RGB_SETUP_A::USED
    }
}
#[doc = "Field `rgb_setup` writer - \"Set-up\" enable for RGB outputs. This bit specifies if the \"set-up\" implied value (black_level - blank_level) specified for the CVBS signal is used also for the RGB signals."]
pub type RGB_SETUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_CONFIGURATION1_SPEC, RGB_SETUP_A, O>;
impl<'a, const O: u8> RGB_SETUP_W<'a, O> {
    #[doc = "The \"set-up\" is not used, or i.e. comp_yuv is equal to b'1'."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(RGB_SETUP_A::NOT_USED)
    }
    #[doc = "The implied \"set-up\" is used for the RGB signals"]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(RGB_SETUP_A::USED)
    }
}
#[doc = "Field `rgb_sync` reader - R, G and B signals sync embedding selection.\n\nThese bits specify whether the sync signal is added to each of the R, G and B components (b'1') or not (b'0'). The bit\\[26\\] specify if the R signal has embedded syncs, the bit\\[25\\] specify if the G signal has embedded syncs and the bit\\[24\\] specify if the B signal has embedded syncs. When comp_yuv is equal to b'1', these bits are N.A. and should be set to b'000'. When the value is different from b'000', RGB_Setup should be set to b'1'."]
pub type RGB_SYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rgb_sync` writer - R, G and B signals sync embedding selection.\n\nThese bits specify whether the sync signal is added to each of the R, G and B components (b'1') or not (b'0'). The bit\\[26\\] specify if the R signal has embedded syncs, the bit\\[25\\] specify if the G signal has embedded syncs and the bit\\[24\\] specify if the B signal has embedded syncs. When comp_yuv is equal to b'1', these bits are N.A. and should be set to b'000'. When the value is different from b'000', RGB_Setup should be set to b'1'."]
pub type RGB_SYNC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_CONFIGURATION1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Y input clamping selection\n\nThis bit selects whether the Video Encoder Y input is clamped to 64 to 940 or not. When not clamped the expected range is 0 to 1023. The U and V inputs are always clamped to the range 64 to 960."]
    #[inline(always)]
    pub fn bypass_yclamp(&self) -> BYPASS_YCLAMP_R {
        BYPASS_YCLAMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - \"Set-up\" enable for RGB outputs. This bit specifies if the \"set-up\" implied value (black_level - blank_level) specified for the CVBS signal is used also for the RGB signals."]
    #[inline(always)]
    pub fn rgb_setup(&self) -> RGB_SETUP_R {
        RGB_SETUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - R, G and B signals sync embedding selection.\n\nThese bits specify whether the sync signal is added to each of the R, G and B components (b'1') or not (b'0'). The bit\\[26\\] specify if the R signal has embedded syncs, the bit\\[25\\] specify if the G signal has embedded syncs and the bit\\[24\\] specify if the B signal has embedded syncs. When comp_yuv is equal to b'1', these bits are N.A. and should be set to b'000'. When the value is different from b'000', RGB_Setup should be set to b'1'."]
    #[inline(always)]
    pub fn rgb_sync(&self) -> RGB_SYNC_R {
        RGB_SYNC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Y input clamping selection\n\nThis bit selects whether the Video Encoder Y input is clamped to 64 to 940 or not. When not clamped the expected range is 0 to 1023. The U and V inputs are always clamped to the range 64 to 960."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_yclamp(&mut self) -> BYPASS_YCLAMP_W<0> {
        BYPASS_YCLAMP_W::new(self)
    }
    #[doc = "Bit 16 - \"Set-up\" enable for RGB outputs. This bit specifies if the \"set-up\" implied value (black_level - blank_level) specified for the CVBS signal is used also for the RGB signals."]
    #[inline(always)]
    #[must_use]
    pub fn rgb_setup(&mut self) -> RGB_SETUP_W<16> {
        RGB_SETUP_W::new(self)
    }
    #[doc = "Bits 24:26 - R, G and B signals sync embedding selection.\n\nThese bits specify whether the sync signal is added to each of the R, G and B components (b'1') or not (b'0'). The bit\\[26\\] specify if the R signal has embedded syncs, the bit\\[25\\] specify if the G signal has embedded syncs and the bit\\[24\\] specify if the B signal has embedded syncs. When comp_yuv is equal to b'1', these bits are N.A. and should be set to b'000'. When the value is different from b'000', RGB_Setup should be set to b'1'."]
    #[inline(always)]
    #[must_use]
    pub fn rgb_sync(&mut self) -> RGB_SYNC_W<24> {
        RGB_SYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Configuration Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_configuration1](index.html) module"]
pub struct TVE_CONFIGURATION1_SPEC;
impl crate::RegisterSpec for TVE_CONFIGURATION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_configuration1::R](R) reader structure"]
impl crate::Readable for TVE_CONFIGURATION1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_configuration1::W](W) writer structure"]
impl crate::Writable for TVE_CONFIGURATION1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_configuration1 to value 0x01"]
impl crate::Resettable for TVE_CONFIGURATION1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
