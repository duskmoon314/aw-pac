#[doc = "Register `power` reader"]
pub type R = crate::R<POWER_SPEC>;
#[doc = "Register `power` writer"]
pub type W = crate::W<POWER_SPEC>;
#[doc = "Field `bg_trim` reader - BG Output Voltage Trimming\n\nOnly low 6-bit is used. The BG output voltage range is from 0.7 V to 1.208 V"]
pub type BG_TRIM_R = crate::FieldReader;
#[doc = "Field `bg_trim` writer - BG Output Voltage Trimming\n\nOnly low 6-bit is used. The BG output voltage range is from 0.7 V to 1.208 V"]
pub type BG_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hpldo_output_voltage` reader - HPLDO Output Voltage Control"]
pub type HPLDO_OUTPUT_VOLTAGE_R = crate::FieldReader<HPLDO_OUTPUT_VOLTAGE_A>;
#[doc = "HPLDO Output Voltage Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPLDO_OUTPUT_VOLTAGE_A {
    #[doc = "0: `0`"]
    V165 = 0,
    #[doc = "1: `1`"]
    V170 = 1,
    #[doc = "2: `10`"]
    V175 = 2,
    #[doc = "3: `11`"]
    V180 = 3,
    #[doc = "4: `100`"]
    V185 = 4,
    #[doc = "5: `101`"]
    V190 = 5,
    #[doc = "6: `110`"]
    V195 = 6,
    #[doc = "7: `111`"]
    V200 = 7,
}
impl From<HPLDO_OUTPUT_VOLTAGE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPLDO_OUTPUT_VOLTAGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPLDO_OUTPUT_VOLTAGE_A {
    type Ux = u8;
}
impl HPLDO_OUTPUT_VOLTAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPLDO_OUTPUT_VOLTAGE_A {
        match self.bits {
            0 => HPLDO_OUTPUT_VOLTAGE_A::V165,
            1 => HPLDO_OUTPUT_VOLTAGE_A::V170,
            2 => HPLDO_OUTPUT_VOLTAGE_A::V175,
            3 => HPLDO_OUTPUT_VOLTAGE_A::V180,
            4 => HPLDO_OUTPUT_VOLTAGE_A::V185,
            5 => HPLDO_OUTPUT_VOLTAGE_A::V190,
            6 => HPLDO_OUTPUT_VOLTAGE_A::V195,
            7 => HPLDO_OUTPUT_VOLTAGE_A::V200,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_v165(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V165
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_v170(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V170
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_v175(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V175
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_v180(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V180
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_v185(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V185
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_v190(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V190
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_v195(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V195
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_v200(&self) -> bool {
        *self == HPLDO_OUTPUT_VOLTAGE_A::V200
    }
}
#[doc = "Field `hpldo_output_voltage` writer - HPLDO Output Voltage Control"]
pub type HPLDO_OUTPUT_VOLTAGE_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 3, HPLDO_OUTPUT_VOLTAGE_A>;
impl<'a, REG> HPLDO_OUTPUT_VOLTAGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn v165(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V165)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn v170(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V170)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn v175(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V175)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn v180(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V180)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn v185(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V185)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn v190(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V190)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn v195(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V195)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn v200(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_OUTPUT_VOLTAGE_A::V200)
    }
}
#[doc = "Field `aldo_output_voltage` reader - ALDO Output Voltage Control"]
pub type ALDO_OUTPUT_VOLTAGE_R = crate::FieldReader<ALDO_OUTPUT_VOLTAGE_A>;
#[doc = "ALDO Output Voltage Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALDO_OUTPUT_VOLTAGE_A {
    #[doc = "0: `0`"]
    V165 = 0,
    #[doc = "1: `1`"]
    V170 = 1,
    #[doc = "2: `10`"]
    V175 = 2,
    #[doc = "3: `11`"]
    V180 = 3,
    #[doc = "4: `100`"]
    V185 = 4,
    #[doc = "5: `101`"]
    V190 = 5,
    #[doc = "6: `110`"]
    V195 = 6,
    #[doc = "7: `111`"]
    V200 = 7,
}
impl From<ALDO_OUTPUT_VOLTAGE_A> for u8 {
    #[inline(always)]
    fn from(variant: ALDO_OUTPUT_VOLTAGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALDO_OUTPUT_VOLTAGE_A {
    type Ux = u8;
}
impl ALDO_OUTPUT_VOLTAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALDO_OUTPUT_VOLTAGE_A {
        match self.bits {
            0 => ALDO_OUTPUT_VOLTAGE_A::V165,
            1 => ALDO_OUTPUT_VOLTAGE_A::V170,
            2 => ALDO_OUTPUT_VOLTAGE_A::V175,
            3 => ALDO_OUTPUT_VOLTAGE_A::V180,
            4 => ALDO_OUTPUT_VOLTAGE_A::V185,
            5 => ALDO_OUTPUT_VOLTAGE_A::V190,
            6 => ALDO_OUTPUT_VOLTAGE_A::V195,
            7 => ALDO_OUTPUT_VOLTAGE_A::V200,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_v165(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V165
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_v170(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V170
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_v175(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V175
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_v180(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V180
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_v185(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V185
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_v190(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V190
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_v195(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V195
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_v200(&self) -> bool {
        *self == ALDO_OUTPUT_VOLTAGE_A::V200
    }
}
#[doc = "Field `aldo_output_voltage` writer - ALDO Output Voltage Control"]
pub type ALDO_OUTPUT_VOLTAGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, ALDO_OUTPUT_VOLTAGE_A>;
impl<'a, REG> ALDO_OUTPUT_VOLTAGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn v165(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V165)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn v170(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V170)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn v175(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V175)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn v180(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V180)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn v185(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V185)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn v190(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V190)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn v195(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V195)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn v200(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_OUTPUT_VOLTAGE_A::V200)
    }
}
#[doc = "Field `avccpor` reader - AVCCPOR Monitor"]
pub type AVCCPOR_R = crate::BitReader;
#[doc = "Field `var1speedup_further_ctrl` reader - VRA1 Speedup Down Further Control In Adda Analog"]
pub type VAR1SPEEDUP_FURTHER_CTRL_R = crate::BitReader<VAR1SPEEDUP_FURTHER_CTRL_A>;
#[doc = "VRA1 Speedup Down Further Control In Adda Analog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VAR1SPEEDUP_FURTHER_CTRL_A {
    #[doc = "0: `0`"]
    DIGITAL = 0,
    #[doc = "1: `1`"]
    MANUAL = 1,
}
impl From<VAR1SPEEDUP_FURTHER_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: VAR1SPEEDUP_FURTHER_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl VAR1SPEEDUP_FURTHER_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VAR1SPEEDUP_FURTHER_CTRL_A {
        match self.bits {
            false => VAR1SPEEDUP_FURTHER_CTRL_A::DIGITAL,
            true => VAR1SPEEDUP_FURTHER_CTRL_A::MANUAL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == VAR1SPEEDUP_FURTHER_CTRL_A::DIGITAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == VAR1SPEEDUP_FURTHER_CTRL_A::MANUAL
    }
}
#[doc = "Field `var1speedup_further_ctrl` writer - VRA1 Speedup Down Further Control In Adda Analog"]
pub type VAR1SPEEDUP_FURTHER_CTRL_W<'a, REG> =
    crate::BitWriter<'a, REG, VAR1SPEEDUP_FURTHER_CTRL_A>;
impl<'a, REG> VAR1SPEEDUP_FURTHER_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(VAR1SPEEDUP_FURTHER_CTRL_A::DIGITAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(VAR1SPEEDUP_FURTHER_CTRL_A::MANUAL)
    }
}
#[doc = "Field `hpldo_en` reader - HPLDO Enable"]
pub type HPLDO_EN_R = crate::BitReader<HPLDO_EN_A>;
#[doc = "HPLDO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPLDO_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HPLDO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPLDO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HPLDO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPLDO_EN_A {
        match self.bits {
            false => HPLDO_EN_A::DISABLE,
            true => HPLDO_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HPLDO_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HPLDO_EN_A::ENABLE
    }
}
#[doc = "Field `hpldo_en` writer - HPLDO Enable"]
pub type HPLDO_EN_W<'a, REG> = crate::BitWriter<'a, REG, HPLDO_EN_A>;
impl<'a, REG> HPLDO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HPLDO_EN_A::ENABLE)
    }
}
#[doc = "Field `aldo_en` reader - ALDO Enable"]
pub type ALDO_EN_R = crate::BitReader<ALDO_EN_A>;
#[doc = "ALDO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALDO_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ALDO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALDO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALDO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALDO_EN_A {
        match self.bits {
            false => ALDO_EN_A::DISABLE,
            true => ALDO_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALDO_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALDO_EN_A::ENABLE
    }
}
#[doc = "Field `aldo_en` writer - ALDO Enable"]
pub type ALDO_EN_W<'a, REG> = crate::BitWriter<'a, REG, ALDO_EN_A>;
impl<'a, REG> ALDO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALDO_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:7 - BG Output Voltage Trimming\n\nOnly low 6-bit is used. The BG output voltage range is from 0.7 V to 1.208 V"]
    #[inline(always)]
    pub fn bg_trim(&self) -> BG_TRIM_R {
        BG_TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - HPLDO Output Voltage Control"]
    #[inline(always)]
    pub fn hpldo_output_voltage(&self) -> HPLDO_OUTPUT_VOLTAGE_R {
        HPLDO_OUTPUT_VOLTAGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ALDO Output Voltage Control"]
    #[inline(always)]
    pub fn aldo_output_voltage(&self) -> ALDO_OUTPUT_VOLTAGE_R {
        ALDO_OUTPUT_VOLTAGE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - AVCCPOR Monitor"]
    #[inline(always)]
    pub fn avccpor(&self) -> AVCCPOR_R {
        AVCCPOR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 29 - VRA1 Speedup Down Further Control In Adda Analog"]
    #[inline(always)]
    pub fn var1speedup_further_ctrl(&self) -> VAR1SPEEDUP_FURTHER_CTRL_R {
        VAR1SPEEDUP_FURTHER_CTRL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HPLDO Enable"]
    #[inline(always)]
    pub fn hpldo_en(&self) -> HPLDO_EN_R {
        HPLDO_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ALDO Enable"]
    #[inline(always)]
    pub fn aldo_en(&self) -> ALDO_EN_R {
        ALDO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BG Output Voltage Trimming\n\nOnly low 6-bit is used. The BG output voltage range is from 0.7 V to 1.208 V"]
    #[inline(always)]
    #[must_use]
    pub fn bg_trim(&mut self) -> BG_TRIM_W<POWER_SPEC> {
        BG_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - HPLDO Output Voltage Control"]
    #[inline(always)]
    #[must_use]
    pub fn hpldo_output_voltage(&mut self) -> HPLDO_OUTPUT_VOLTAGE_W<POWER_SPEC> {
        HPLDO_OUTPUT_VOLTAGE_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - ALDO Output Voltage Control"]
    #[inline(always)]
    #[must_use]
    pub fn aldo_output_voltage(&mut self) -> ALDO_OUTPUT_VOLTAGE_W<POWER_SPEC> {
        ALDO_OUTPUT_VOLTAGE_W::new(self, 12)
    }
    #[doc = "Bit 29 - VRA1 Speedup Down Further Control In Adda Analog"]
    #[inline(always)]
    #[must_use]
    pub fn var1speedup_further_ctrl(&mut self) -> VAR1SPEEDUP_FURTHER_CTRL_W<POWER_SPEC> {
        VAR1SPEEDUP_FURTHER_CTRL_W::new(self, 29)
    }
    #[doc = "Bit 30 - HPLDO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpldo_en(&mut self) -> HPLDO_EN_W<POWER_SPEC> {
        HPLDO_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - ALDO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aldo_en(&mut self) -> ALDO_EN_W<POWER_SPEC> {
        ALDO_EN_W::new(self, 31)
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
#[doc = "POWER Analog Control Register\n\nThe register is not controlled by the clock and reset of Audio Codec, only controlled by the clock and reset of system bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets power to value 0"]
impl crate::Resettable for POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
