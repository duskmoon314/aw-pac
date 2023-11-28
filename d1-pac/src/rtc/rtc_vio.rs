#[doc = "Register `rtc_vio` reader"]
pub type R = crate::R<RTC_VIO_SPEC>;
#[doc = "Register `rtc_vio` writer"]
pub type W = crate::W<RTC_VIO_SPEC>;
#[doc = "Field `rtc_viou` reader - RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V."]
pub type RTC_VIOU_R = crate::FieldReader<RTC_VIOU_A>;
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
impl crate::FieldSpec for RTC_VIOU_A {
    type Ux = u8;
}
impl RTC_VIOU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_VIOU_A {
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
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn is_v_1_0(&self) -> bool {
        *self == RTC_VIOU_A::V_1_0
    }
    #[doc = "0.65 V (the configuration can cause RTC reset)"]
    #[inline(always)]
    pub fn is_v_0_65(&self) -> bool {
        *self == RTC_VIOU_A::V_0_65
    }
    #[doc = "0.7 V"]
    #[inline(always)]
    pub fn is_v_0_7(&self) -> bool {
        *self == RTC_VIOU_A::V_0_7
    }
    #[doc = "0.8 V"]
    #[inline(always)]
    pub fn is_v_0_8(&self) -> bool {
        *self == RTC_VIOU_A::V_0_8
    }
    #[doc = "0.9 V"]
    #[inline(always)]
    pub fn is_v_0_9(&self) -> bool {
        *self == RTC_VIOU_A::V_0_9
    }
    #[doc = "1.1 V"]
    #[inline(always)]
    pub fn is_v_1_1(&self) -> bool {
        *self == RTC_VIOU_A::V_1_1
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn is_v_1_2(&self) -> bool {
        *self == RTC_VIOU_A::V_1_2
    }
    #[doc = "1.3 V"]
    #[inline(always)]
    pub fn is_v_1_3(&self) -> bool {
        *self == RTC_VIOU_A::V_1_3
    }
}
#[doc = "Field `rtc_viou` writer - RTC_VIO Voltage Select\n\nThe RTC-VIO is provided power for RTC digital part.\n\nThese bits are useful for regulating the RTC_VIO from 0.65 V to 1.3 V."]
pub type RTC_VIOU_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, RTC_VIOU_A>;
impl<'a, REG> RTC_VIOU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn v_1_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_1_0)
    }
    #[doc = "0.65 V (the configuration can cause RTC reset)"]
    #[inline(always)]
    pub fn v_0_65(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_0_65)
    }
    #[doc = "0.7 V"]
    #[inline(always)]
    pub fn v_0_7(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_0_7)
    }
    #[doc = "0.8 V"]
    #[inline(always)]
    pub fn v_0_8(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_0_8)
    }
    #[doc = "0.9 V"]
    #[inline(always)]
    pub fn v_0_9(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_0_9)
    }
    #[doc = "1.1 V"]
    #[inline(always)]
    pub fn v_1_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_1_1)
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn v_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_VIOU_A::V_1_2)
    }
    #[doc = "1.3 V"]
    #[inline(always)]
    pub fn v_1_3(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> V_SEL_A {
        match self.bits {
            false => V_SEL_A::RESISTANCE,
            true => V_SEL_A::BAND,
        }
    }
    #[doc = "Resistance divider"]
    #[inline(always)]
    pub fn is_resistance(&self) -> bool {
        *self == V_SEL_A::RESISTANCE
    }
    #[doc = "Band gap"]
    #[inline(always)]
    pub fn is_band(&self) -> bool {
        *self == V_SEL_A::BAND
    }
}
#[doc = "Field `v_sel` writer - VDD Select"]
pub type V_SEL_W<'a, REG> = crate::BitWriter<'a, REG, V_SEL_A>;
impl<'a, REG> V_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resistance divider"]
    #[inline(always)]
    pub fn resistance(self) -> &'a mut crate::W<REG> {
        self.variant(V_SEL_A::RESISTANCE)
    }
    #[doc = "Band gap"]
    #[inline(always)]
    pub fn band(self) -> &'a mut crate::W<REG> {
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
    pub fn rtc_viou(&mut self) -> RTC_VIOU_W<RTC_VIO_SPEC> {
        RTC_VIOU_W::new(self, 0)
    }
    #[doc = "Bit 4 - VDD Select"]
    #[inline(always)]
    #[must_use]
    pub fn v_sel(&mut self) -> V_SEL_W<RTC_VIO_SPEC> {
        V_SEL_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC_VIO Regulation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_vio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_vio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_VIO_SPEC;
impl crate::RegisterSpec for RTC_VIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_vio::R`](R) reader structure"]
impl crate::Readable for RTC_VIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_vio::W`](W) writer structure"]
impl crate::Writable for RTC_VIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_vio to value 0x04"]
impl crate::Resettable for RTC_VIO_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
