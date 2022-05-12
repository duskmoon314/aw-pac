#[doc = "Register `CIR_CTL` reader"]
pub struct R(crate::R<CIR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR_CTL` writer"]
pub struct W(crate::W<CIR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_CTL_SPEC>;
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
impl From<crate::W<CIR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Active Pulse Accept Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APAM_A {
    #[doc = "0: Both positive and negative pulses are valid as a leading code"]
    BOTH_VALID = 0,
    #[doc = "2: Only negative pulse is valid as a leading code"]
    ONLY_NEGATIVE = 2,
    #[doc = "3: Only positive pulse is valid as a leading code"]
    ONLY_POSITIVE = 3,
}
impl From<APAM_A> for u8 {
    #[inline(always)]
    fn from(variant: APAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APAM` reader - Active Pulse Accept Mode"]
pub type APAM_R = crate::FieldReader<u8, APAM_A>;
impl APAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APAM_A> {
        match self.bits {
            0 => Some(APAM_A::BOTH_VALID),
            2 => Some(APAM_A::ONLY_NEGATIVE),
            3 => Some(APAM_A::ONLY_POSITIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH_VALID`"]
    #[inline(always)]
    pub fn is_both_valid(&self) -> bool {
        *self == APAM_A::BOTH_VALID
    }
    #[doc = "Checks if the value of the field is `ONLY_NEGATIVE`"]
    #[inline(always)]
    pub fn is_only_negative(&self) -> bool {
        *self == APAM_A::ONLY_NEGATIVE
    }
    #[doc = "Checks if the value of the field is `ONLY_POSITIVE`"]
    #[inline(always)]
    pub fn is_only_positive(&self) -> bool {
        *self == APAM_A::ONLY_POSITIVE
    }
}
#[doc = "Field `APAM` writer - Active Pulse Accept Mode"]
pub type APAM_W<'a> = crate::FieldWriter<'a, u32, CIR_CTL_SPEC, u8, APAM_A, 2, 6>;
impl<'a> APAM_W<'a> {
    #[doc = "Both positive and negative pulses are valid as a leading code"]
    #[inline(always)]
    pub fn both_valid(self) -> &'a mut W {
        self.variant(APAM_A::BOTH_VALID)
    }
    #[doc = "Only negative pulse is valid as a leading code"]
    #[inline(always)]
    pub fn only_negative(self) -> &'a mut W {
        self.variant(APAM_A::ONLY_NEGATIVE)
    }
    #[doc = "Only positive pulse is valid as a leading code"]
    #[inline(always)]
    pub fn only_positive(self) -> &'a mut W {
        self.variant(APAM_A::ONLY_POSITIVE)
    }
}
#[doc = "CIR Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIREN_A {
    #[doc = "3: CIR mode enable"]
    ENABLE = 3,
}
impl From<CIREN_A> for u8 {
    #[inline(always)]
    fn from(variant: CIREN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CIREN` reader - CIR Enable"]
pub type CIREN_R = crate::FieldReader<u8, CIREN_A>;
impl CIREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIREN_A> {
        match self.bits {
            3 => Some(CIREN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CIREN_A::ENABLE
    }
}
#[doc = "Field `CIREN` writer - CIR Enable"]
pub type CIREN_W<'a> = crate::FieldWriter<'a, u32, CIR_CTL_SPEC, u8, CIREN_A, 2, 4>;
impl<'a> CIREN_W<'a> {
    #[doc = "CIR mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CIREN_A::ENABLE)
    }
}
#[doc = "Receiver Block Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - Receiver Block Enable"]
pub type RXEN_R = crate::BitReader<RXEN_A>;
impl RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::DISABLE,
            true => RXEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXEN_A::ENABLE
    }
}
#[doc = "Field `RXEN` writer - Receiver Block Enable"]
pub type RXEN_W<'a> = crate::BitWriter<'a, u32, CIR_CTL_SPEC, RXEN_A, 1>;
impl<'a> RXEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXEN_A::ENABLE)
    }
}
#[doc = "Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<GEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEN` reader - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
pub type GEN_R = crate::BitReader<GEN_A>;
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEN_A {
        match self.bits {
            false => GEN_A::DISABLE,
            true => GEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GEN_A::ENABLE
    }
}
#[doc = "Field `GEN` writer - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
pub type GEN_W<'a> = crate::BitWriter<'a, u32, CIR_CTL_SPEC, GEN_A, 0>;
impl<'a> GEN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 6:7 - Active Pulse Accept Mode"]
    #[inline(always)]
    pub fn apam(&self) -> APAM_R {
        APAM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CIR Enable"]
    #[inline(always)]
    pub fn ciren(&self) -> CIREN_R {
        CIREN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 1 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Active Pulse Accept Mode"]
    #[inline(always)]
    pub fn apam(&mut self) -> APAM_W {
        APAM_W::new(self)
    }
    #[doc = "Bits 4:5 - CIR Enable"]
    #[inline(always)]
    pub fn ciren(&mut self) -> CIREN_W {
        CIREN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W::new(self)
    }
    #[doc = "Bit 0 - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_ctl](index.html) module"]
pub struct CIR_CTL_SPEC;
impl crate::RegisterSpec for CIR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_ctl::R](R) reader structure"]
impl crate::Readable for CIR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_ctl::W](W) writer structure"]
impl crate::Writable for CIR_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR_CTL to value 0"]
impl crate::Resettable for CIR_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
