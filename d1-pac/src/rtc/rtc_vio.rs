#[doc = "Register `rtc_vio` reader"]
pub struct R(crate::R<RTC_VIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_VIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_VIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_VIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_vio` writer"]
pub struct W(crate::W<RTC_VIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_VIO_SPEC>;
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
impl From<crate::W<RTC_VIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_VIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_viou` reader - RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V."]
pub type RTC_VIOU_R = crate::FieldReader<u8, RTC_VIOU_A>;
#[doc = "RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC_VIOU_A {
    #[doc = "0: 1.0 V"]
    V_1_0 = 0,
    #[doc = "1: 0.65 V (the configuration can cause RTC reset)"]
    V_0_65 = 1,
    #[doc = "2: 0.7 V"]
    V_0_7 = 2,
    #[doc = "3: 0.8 V"]
    V_0_8 = 3,
    #[doc = "4: 0.9 V"]
    V_0_9 = 4,
    #[doc = "5: 1.1 V"]
    V_1_1 = 5,
    #[doc = "6: 1.2 V"]
    V_1_2 = 6,
    #[doc = "7: 1.3 V"]
    V_1_3 = 7,
}
impl From<RTC_VIOU_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_VIOU_A) -> Self {
        variant as _
    }
}
impl RTC_VIOU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_VIOU_A {
        match self.bits {
            0 => RTC_VIOU_A::V_1_0,
            1 => RTC_VIOU_A::V_0_65,
            2 => RTC_VIOU_A::V_0_7,
            3 => RTC_VIOU_A::V_0_8,
            4 => RTC_VIOU_A::V_0_9,
            5 => RTC_VIOU_A::V_1_1,
            6 => RTC_VIOU_A::V_1_2,
            7 => RTC_VIOU_A::V_1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_1_0`"]
    #[inline(always)]
    pub fn is_v_1_0(&self) -> bool {
        *self == RTC_VIOU_A::V_1_0
    }
    #[doc = "Checks if the value of the field is `V_0_65`"]
    #[inline(always)]
    pub fn is_v_0_65(&self) -> bool {
        *self == RTC_VIOU_A::V_0_65
    }
    #[doc = "Checks if the value of the field is `V_0_7`"]
    #[inline(always)]
    pub fn is_v_0_7(&self) -> bool {
        *self == RTC_VIOU_A::V_0_7
    }
    #[doc = "Checks if the value of the field is `V_0_8`"]
    #[inline(always)]
    pub fn is_v_0_8(&self) -> bool {
        *self == RTC_VIOU_A::V_0_8
    }
    #[doc = "Checks if the value of the field is `V_0_9`"]
    #[inline(always)]
    pub fn is_v_0_9(&self) -> bool {
        *self == RTC_VIOU_A::V_0_9
    }
    #[doc = "Checks if the value of the field is `V_1_1`"]
    #[inline(always)]
    pub fn is_v_1_1(&self) -> bool {
        *self == RTC_VIOU_A::V_1_1
    }
    #[doc = "Checks if the value of the field is `V_1_2`"]
    #[inline(always)]
    pub fn is_v_1_2(&self) -> bool {
        *self == RTC_VIOU_A::V_1_2
    }
    #[doc = "Checks if the value of the field is `V_1_3`"]
    #[inline(always)]
    pub fn is_v_1_3(&self) -> bool {
        *self == RTC_VIOU_A::V_1_3
    }
}
#[doc = "Field `rtc_viou` writer - RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V."]
pub type RTC_VIOU_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RTC_VIO_SPEC, u8, RTC_VIOU_A, 3, O>;
impl<'a, const O: u8> RTC_VIOU_W<'a, O> {
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn v_1_0(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_1_0)
    }
    #[doc = "0.65 V (the configuration can cause RTC reset)"]
    #[inline(always)]
    pub fn v_0_65(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_0_65)
    }
    #[doc = "0.7 V"]
    #[inline(always)]
    pub fn v_0_7(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_0_7)
    }
    #[doc = "0.8 V"]
    #[inline(always)]
    pub fn v_0_8(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_0_8)
    }
    #[doc = "0.9 V"]
    #[inline(always)]
    pub fn v_0_9(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_0_9)
    }
    #[doc = "1.1 V"]
    #[inline(always)]
    pub fn v_1_1(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_1_1)
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn v_1_2(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_1_2)
    }
    #[doc = "1.3 V"]
    #[inline(always)]
    pub fn v_1_3(self) -> &'a mut W {
        self.variant(RTC_VIOU_A::V_1_3)
    }
}
#[doc = "Field `v_sel` reader - VDD Select"]
pub type V_SEL_R = crate::BitReader<V_SEL_A>;
#[doc = "VDD Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V_SEL_A {
    #[doc = "0: Resistance divider"]
    RESISTANCE = 0,
    #[doc = "1: Band gap"]
    BAND = 1,
}
impl From<V_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: V_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl V_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_SEL_A {
        match self.bits {
            false => V_SEL_A::RESISTANCE,
            true => V_SEL_A::BAND,
        }
    }
    #[doc = "Checks if the value of the field is `RESISTANCE`"]
    #[inline(always)]
    pub fn is_resistance(&self) -> bool {
        *self == V_SEL_A::RESISTANCE
    }
    #[doc = "Checks if the value of the field is `BAND`"]
    #[inline(always)]
    pub fn is_band(&self) -> bool {
        *self == V_SEL_A::BAND
    }
}
#[doc = "Field `v_sel` writer - VDD Select"]
pub type V_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_VIO_SPEC, V_SEL_A, O>;
impl<'a, const O: u8> V_SEL_W<'a, O> {
    #[doc = "Resistance divider"]
    #[inline(always)]
    pub fn resistance(self) -> &'a mut W {
        self.variant(V_SEL_A::RESISTANCE)
    }
    #[doc = "Band gap"]
    #[inline(always)]
    pub fn band(self) -> &'a mut W {
        self.variant(V_SEL_A::BAND)
    }
}
impl R {
    #[doc = "Bits 0:2 - RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V."]
    #[inline(always)]
    pub fn rtc_viou(&self) -> RTC_VIOU_R {
        RTC_VIOU_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - VDD Select"]
    #[inline(always)]
    pub fn v_sel(&self) -> V_SEL_R {
        V_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_viou(&mut self) -> RTC_VIOU_W<0> {
        RTC_VIOU_W::new(self)
    }
    #[doc = "Bit 4 - VDD Select"]
    #[inline(always)]
    #[must_use]
    pub fn v_sel(&mut self) -> V_SEL_W<4> {
        V_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_VIO Regulation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_vio](index.html) module"]
pub struct RTC_VIO_SPEC;
impl crate::RegisterSpec for RTC_VIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_vio::R](R) reader structure"]
impl crate::Readable for RTC_VIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_vio::W](W) writer structure"]
impl crate::Writable for RTC_VIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_vio to value 0x04"]
impl crate::Resettable for RTC_VIO_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
