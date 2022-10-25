#[doc = "Register `ramp` reader"]
pub struct R(crate::R<RAMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ramp` writer"]
pub struct W(crate::W<RAMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMP_SPEC>;
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
impl From<crate::W<RAMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rd_en` reader - Ramp Digital Enable"]
pub type RD_EN_R = crate::BitReader<RD_EN_A>;
#[doc = "Ramp Digital Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_EN_A {
        match self.bits {
            false => RD_EN_A::DISABLE,
            true => RD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RD_EN_A::ENABLE
    }
}
#[doc = "Field `rd_en` writer - Ramp Digital Enable"]
pub type RD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMP_SPEC, RD_EN_A, O>;
impl<'a, const O: u8> RD_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RD_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RD_EN_A::ENABLE)
    }
}
#[doc = "Field `rmc_en` reader - Ramp Manual Control Enable"]
pub type RMC_EN_R = crate::BitReader<RMC_EN_A>;
#[doc = "Ramp Manual Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMC_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RMC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RMC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RMC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMC_EN_A {
        match self.bits {
            false => RMC_EN_A::DISABLE,
            true => RMC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMC_EN_A::ENABLE
    }
}
#[doc = "Field `rmc_en` writer - Ramp Manual Control Enable"]
pub type RMC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMP_SPEC, RMC_EN_A, O>;
impl<'a, const O: u8> RMC_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RMC_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RMC_EN_A::ENABLE)
    }
}
#[doc = "Field `rmu_en` reader - Ramp Manual Up Enable"]
pub type RMU_EN_R = crate::BitReader<RMU_EN_A>;
#[doc = "Ramp Manual Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMU_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RMU_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RMU_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RMU_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMU_EN_A {
        match self.bits {
            false => RMU_EN_A::DISABLE,
            true => RMU_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMU_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMU_EN_A::ENABLE
    }
}
#[doc = "Field `rmu_en` writer - Ramp Manual Up Enable"]
pub type RMU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMP_SPEC, RMU_EN_A, O>;
impl<'a, const O: u8> RMU_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RMU_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RMU_EN_A::ENABLE)
    }
}
#[doc = "Field `rmd_en` reader - Ramp Manual Down Enable"]
pub type RMD_EN_R = crate::BitReader<RMD_EN_A>;
#[doc = "Ramp Manual Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMD_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMD_EN_A {
        match self.bits {
            false => RMD_EN_A::DISABLE,
            true => RMD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMD_EN_A::ENABLE
    }
}
#[doc = "Field `rmd_en` writer - Ramp Manual Down Enable"]
pub type RMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMP_SPEC, RMD_EN_A, O>;
impl<'a, const O: u8> RMD_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RMD_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RMD_EN_A::ENABLE)
    }
}
#[doc = "Field `ramp_step` reader - RK Frequency Gear, Control Ramp Rise/Fall Total Time\n\nRamp Rise/Fall Total Time = (Ramp Step/Ramp Clk Freq)*4096"]
pub type RAMP_STEP_R = crate::FieldReader<u8, RAMP_STEP_A>;
#[doc = "RK Frequency Gear, Control Ramp Rise/Fall Total Time\n\nRamp Rise/Fall Total Time = (Ramp Step/Ramp Clk Freq)*4096\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMP_STEP_A {
    #[doc = "0: `0`"]
    S20 = 0,
    #[doc = "1: `1`"]
    S30 = 1,
    #[doc = "2: `10`"]
    S40 = 2,
    #[doc = "3: `11`"]
    S60 = 3,
    #[doc = "4: `100`"]
    S80 = 4,
    #[doc = "5: `101`"]
    S120 = 5,
    #[doc = "6: `110`"]
    S160 = 6,
    #[doc = "7: `111`"]
    S240 = 7,
}
impl From<RAMP_STEP_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMP_STEP_A) -> Self {
        variant as _
    }
}
impl RAMP_STEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_STEP_A {
        match self.bits {
            0 => RAMP_STEP_A::S20,
            1 => RAMP_STEP_A::S30,
            2 => RAMP_STEP_A::S40,
            3 => RAMP_STEP_A::S60,
            4 => RAMP_STEP_A::S80,
            5 => RAMP_STEP_A::S120,
            6 => RAMP_STEP_A::S160,
            7 => RAMP_STEP_A::S240,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S20`"]
    #[inline(always)]
    pub fn is_s20(&self) -> bool {
        *self == RAMP_STEP_A::S20
    }
    #[doc = "Checks if the value of the field is `S30`"]
    #[inline(always)]
    pub fn is_s30(&self) -> bool {
        *self == RAMP_STEP_A::S30
    }
    #[doc = "Checks if the value of the field is `S40`"]
    #[inline(always)]
    pub fn is_s40(&self) -> bool {
        *self == RAMP_STEP_A::S40
    }
    #[doc = "Checks if the value of the field is `S60`"]
    #[inline(always)]
    pub fn is_s60(&self) -> bool {
        *self == RAMP_STEP_A::S60
    }
    #[doc = "Checks if the value of the field is `S80`"]
    #[inline(always)]
    pub fn is_s80(&self) -> bool {
        *self == RAMP_STEP_A::S80
    }
    #[doc = "Checks if the value of the field is `S120`"]
    #[inline(always)]
    pub fn is_s120(&self) -> bool {
        *self == RAMP_STEP_A::S120
    }
    #[doc = "Checks if the value of the field is `S160`"]
    #[inline(always)]
    pub fn is_s160(&self) -> bool {
        *self == RAMP_STEP_A::S160
    }
    #[doc = "Checks if the value of the field is `S240`"]
    #[inline(always)]
    pub fn is_s240(&self) -> bool {
        *self == RAMP_STEP_A::S240
    }
}
#[doc = "Field `ramp_step` writer - RK Frequency Gear, Control Ramp Rise/Fall Total Time\n\nRamp Rise/Fall Total Time = (Ramp Step/Ramp Clk Freq)*4096"]
pub type RAMP_STEP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RAMP_SPEC, u8, RAMP_STEP_A, 3, O>;
impl<'a, const O: u8> RAMP_STEP_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s20(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S20)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn s30(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S30)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s40(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S40)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn s60(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S60)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn s80(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S80)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn s120(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S120)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn s160(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S160)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn s240(self) -> &'a mut W {
        self.variant(RAMP_STEP_A::S240)
    }
}
#[doc = "Field `gap_step` reader - Gap Step"]
pub type GAP_STEP_R = crate::FieldReader<u8, GAP_STEP_A>;
#[doc = "Gap Step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAP_STEP_A {
    #[doc = "0: `0`"]
    RAMP_STEP_X1 = 0,
    #[doc = "1: `1`"]
    RAMP_STEP_X2 = 1,
    #[doc = "2: `10`"]
    RAMP_STEP_X3 = 2,
    #[doc = "3: `11`"]
    RAMP_STEP_X4 = 3,
}
impl From<GAP_STEP_A> for u8 {
    #[inline(always)]
    fn from(variant: GAP_STEP_A) -> Self {
        variant as _
    }
}
impl GAP_STEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAP_STEP_A {
        match self.bits {
            0 => GAP_STEP_A::RAMP_STEP_X1,
            1 => GAP_STEP_A::RAMP_STEP_X2,
            2 => GAP_STEP_A::RAMP_STEP_X3,
            3 => GAP_STEP_A::RAMP_STEP_X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP_STEP_X1`"]
    #[inline(always)]
    pub fn is_ramp_step_x1(&self) -> bool {
        *self == GAP_STEP_A::RAMP_STEP_X1
    }
    #[doc = "Checks if the value of the field is `RAMP_STEP_X2`"]
    #[inline(always)]
    pub fn is_ramp_step_x2(&self) -> bool {
        *self == GAP_STEP_A::RAMP_STEP_X2
    }
    #[doc = "Checks if the value of the field is `RAMP_STEP_X3`"]
    #[inline(always)]
    pub fn is_ramp_step_x3(&self) -> bool {
        *self == GAP_STEP_A::RAMP_STEP_X3
    }
    #[doc = "Checks if the value of the field is `RAMP_STEP_X4`"]
    #[inline(always)]
    pub fn is_ramp_step_x4(&self) -> bool {
        *self == GAP_STEP_A::RAMP_STEP_X4
    }
}
#[doc = "Field `gap_step` writer - Gap Step"]
pub type GAP_STEP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RAMP_SPEC, u8, GAP_STEP_A, 2, O>;
impl<'a, const O: u8> GAP_STEP_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ramp_step_x1(self) -> &'a mut W {
        self.variant(GAP_STEP_A::RAMP_STEP_X1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ramp_step_x2(self) -> &'a mut W {
        self.variant(GAP_STEP_A::RAMP_STEP_X2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ramp_step_x3(self) -> &'a mut W {
        self.variant(GAP_STEP_A::RAMP_STEP_X3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ramp_step_x4(self) -> &'a mut W {
        self.variant(GAP_STEP_A::RAMP_STEP_X4)
    }
}
#[doc = "Field `ramp_hold_step` reader - Ramp Hold Step\n\nRamp Hold Time = Ramp Hold Step/Ramp Clk Freq"]
pub type RAMP_HOLD_STEP_R = crate::FieldReader<u8, RAMP_HOLD_STEP_A>;
#[doc = "Ramp Hold Step\n\nRamp Hold Time = Ramp Hold Step/Ramp Clk Freq\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMP_HOLD_STEP_A {
    #[doc = "0: `0`"]
    S9600 = 0,
    #[doc = "1: `1`"]
    S19200 = 1,
    #[doc = "2: `10`"]
    S38400 = 2,
    #[doc = "3: `11`"]
    S76800 = 3,
    #[doc = "4: `100`"]
    S96000 = 4,
    #[doc = "5: `101`"]
    S115200 = 5,
    #[doc = "6: `110`"]
    S153600 = 6,
    #[doc = "7: `111`"]
    S192000 = 7,
}
impl From<RAMP_HOLD_STEP_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMP_HOLD_STEP_A) -> Self {
        variant as _
    }
}
impl RAMP_HOLD_STEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_HOLD_STEP_A {
        match self.bits {
            0 => RAMP_HOLD_STEP_A::S9600,
            1 => RAMP_HOLD_STEP_A::S19200,
            2 => RAMP_HOLD_STEP_A::S38400,
            3 => RAMP_HOLD_STEP_A::S76800,
            4 => RAMP_HOLD_STEP_A::S96000,
            5 => RAMP_HOLD_STEP_A::S115200,
            6 => RAMP_HOLD_STEP_A::S153600,
            7 => RAMP_HOLD_STEP_A::S192000,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S9600`"]
    #[inline(always)]
    pub fn is_s9600(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S9600
    }
    #[doc = "Checks if the value of the field is `S19200`"]
    #[inline(always)]
    pub fn is_s19200(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S19200
    }
    #[doc = "Checks if the value of the field is `S38400`"]
    #[inline(always)]
    pub fn is_s38400(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S38400
    }
    #[doc = "Checks if the value of the field is `S76800`"]
    #[inline(always)]
    pub fn is_s76800(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S76800
    }
    #[doc = "Checks if the value of the field is `S96000`"]
    #[inline(always)]
    pub fn is_s96000(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S96000
    }
    #[doc = "Checks if the value of the field is `S115200`"]
    #[inline(always)]
    pub fn is_s115200(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S115200
    }
    #[doc = "Checks if the value of the field is `S153600`"]
    #[inline(always)]
    pub fn is_s153600(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S153600
    }
    #[doc = "Checks if the value of the field is `S192000`"]
    #[inline(always)]
    pub fn is_s192000(&self) -> bool {
        *self == RAMP_HOLD_STEP_A::S192000
    }
}
#[doc = "Field `ramp_hold_step` writer - Ramp Hold Step\n\nRamp Hold Time = Ramp Hold Step/Ramp Clk Freq"]
pub type RAMP_HOLD_STEP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RAMP_SPEC, u8, RAMP_HOLD_STEP_A, 3, O>;
impl<'a, const O: u8> RAMP_HOLD_STEP_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s9600(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S9600)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn s19200(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S19200)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s38400(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S38400)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn s76800(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S76800)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn s96000(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S96000)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn s115200(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S115200)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn s153600(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S153600)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn s192000(self) -> &'a mut W {
        self.variant(RAMP_HOLD_STEP_A::S192000)
    }
}
#[doc = "Field `hp_pull_out_en` reader - Headphone Pullout Enable"]
pub type HP_PULL_OUT_EN_R = crate::BitReader<HP_PULL_OUT_EN_A>;
#[doc = "Headphone Pullout Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_PULL_OUT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HP_PULL_OUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HP_PULL_OUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_PULL_OUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_PULL_OUT_EN_A {
        match self.bits {
            false => HP_PULL_OUT_EN_A::DISABLE,
            true => HP_PULL_OUT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HP_PULL_OUT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HP_PULL_OUT_EN_A::ENABLE
    }
}
#[doc = "Field `hp_pull_out_en` writer - Headphone Pullout Enable"]
pub type HP_PULL_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RAMP_SPEC, HP_PULL_OUT_EN_A, O>;
impl<'a, const O: u8> HP_PULL_OUT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HP_PULL_OUT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HP_PULL_OUT_EN_A::ENABLE)
    }
}
#[doc = "Field `ramp_clk_div_m` reader - Analog Ramp Clk Div Freq Value : M (from 0 to 31, Default: 24).\n\nAna_Ramp_Clk= 24MHz/(M+1)\n\nDefault Ramp Clk Freq: 24MHz/(24+1)=960 kHz"]
pub type RAMP_CLK_DIV_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ramp_clk_div_m` writer - Analog Ramp Clk Div Freq Value : M (from 0 to 31, Default: 24).\n\nAna_Ramp_Clk= 24MHz/(M+1)\n\nDefault Ramp Clk Freq: 24MHz/(24+1)=960 kHz"]
pub type RAMP_CLK_DIV_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAMP_SPEC, u8, u8, 5, O>;
#[doc = "Field `ramp_srst` reader - Ramp Soft Reset"]
pub type RAMP_SRST_R = crate::BitReader<RAMP_SRST_A>;
#[doc = "Ramp Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_SRST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RAMP_SRST_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_SRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_SRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_SRST_A {
        match self.bits {
            false => RAMP_SRST_A::DISABLE,
            true => RAMP_SRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAMP_SRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAMP_SRST_A::ENABLE
    }
}
#[doc = "Field `ramp_srst` writer - Ramp Soft Reset"]
pub type RAMP_SRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMP_SPEC, RAMP_SRST_A, O>;
impl<'a, const O: u8> RAMP_SRST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMP_SRST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMP_SRST_A::ENABLE)
    }
}
#[doc = "Field `ramp_fall_int` reader - RK Downward Decrease Finish and Rampen Pull Down Instruction"]
pub type RAMP_FALL_INT_R = crate::BitReader<RAMP_FALL_INT_A>;
#[doc = "RK Downward Decrease Finish and Rampen Pull Down Instruction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_FALL_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RAMP_FALL_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_FALL_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_FALL_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_FALL_INT_A {
        match self.bits {
            false => RAMP_FALL_INT_A::NO_PENDING,
            true => RAMP_FALL_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RAMP_FALL_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RAMP_FALL_INT_A::PENDING
    }
}
#[doc = "Field `ramp_fall_int` writer - RK Downward Decrease Finish and Rampen Pull Down Instruction"]
pub type RAMP_FALL_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RAMP_SPEC, RAMP_FALL_INT_A, O>;
impl<'a, const O: u8> RAMP_FALL_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RAMP_FALL_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RAMP_FALL_INT_A::PENDING)
    }
}
#[doc = "Field `ramp_fall_int_en` reader - RAMP Fall Int Enable"]
pub type RAMP_FALL_INT_EN_R = crate::BitReader<RAMP_FALL_INT_EN_A>;
#[doc = "RAMP Fall Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_FALL_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RAMP_FALL_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_FALL_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_FALL_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_FALL_INT_EN_A {
        match self.bits {
            false => RAMP_FALL_INT_EN_A::DISABLE,
            true => RAMP_FALL_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAMP_FALL_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAMP_FALL_INT_EN_A::ENABLE
    }
}
#[doc = "Field `ramp_fall_int_en` writer - RAMP Fall Int Enable"]
pub type RAMP_FALL_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RAMP_SPEC, RAMP_FALL_INT_EN_A, O>;
impl<'a, const O: u8> RAMP_FALL_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMP_FALL_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMP_FALL_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `ramp_rise_int` reader - RK Increase Upward Finish and Rampen Pull Down Instruction"]
pub type RAMP_RISE_INT_R = crate::BitReader<RAMP_RISE_INT_A>;
#[doc = "RK Increase Upward Finish and Rampen Pull Down Instruction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_RISE_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RAMP_RISE_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_RISE_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_RISE_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_RISE_INT_A {
        match self.bits {
            false => RAMP_RISE_INT_A::NO_PENDING,
            true => RAMP_RISE_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RAMP_RISE_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RAMP_RISE_INT_A::PENDING
    }
}
#[doc = "Field `ramp_rise_int` writer - RK Increase Upward Finish and Rampen Pull Down Instruction"]
pub type RAMP_RISE_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RAMP_SPEC, RAMP_RISE_INT_A, O>;
impl<'a, const O: u8> RAMP_RISE_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RAMP_RISE_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RAMP_RISE_INT_A::PENDING)
    }
}
#[doc = "Field `ramp_rise_int_en` reader - RAMP Rise Interrupt Enable"]
pub type RAMP_RISE_INT_EN_R = crate::BitReader<RAMP_RISE_INT_EN_A>;
#[doc = "RAMP Rise Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_RISE_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RAMP_RISE_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_RISE_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_RISE_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_RISE_INT_EN_A {
        match self.bits {
            false => RAMP_RISE_INT_EN_A::DISABLE,
            true => RAMP_RISE_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAMP_RISE_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAMP_RISE_INT_EN_A::ENABLE
    }
}
#[doc = "Field `ramp_rise_int_en` writer - RAMP Rise Interrupt Enable"]
pub type RAMP_RISE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RAMP_SPEC, RAMP_RISE_INT_EN_A, O>;
impl<'a, const O: u8> RAMP_RISE_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMP_RISE_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMP_RISE_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Ramp Digital Enable"]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ramp Manual Control Enable"]
    #[inline(always)]
    pub fn rmc_en(&self) -> RMC_EN_R {
        RMC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ramp Manual Up Enable"]
    #[inline(always)]
    pub fn rmu_en(&self) -> RMU_EN_R {
        RMU_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ramp Manual Down Enable"]
    #[inline(always)]
    pub fn rmd_en(&self) -> RMD_EN_R {
        RMD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RK Frequency Gear, Control Ramp Rise/Fall Total Time\n\nRamp Rise/Fall Total Time = (Ramp Step/Ramp Clk Freq)*4096"]
    #[inline(always)]
    pub fn ramp_step(&self) -> RAMP_STEP_R {
        RAMP_STEP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Gap Step"]
    #[inline(always)]
    pub fn gap_step(&self) -> GAP_STEP_R {
        GAP_STEP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Ramp Hold Step\n\nRamp Hold Time = Ramp Hold Step/Ramp Clk Freq"]
    #[inline(always)]
    pub fn ramp_hold_step(&self) -> RAMP_HOLD_STEP_R {
        RAMP_HOLD_STEP_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Headphone Pullout Enable"]
    #[inline(always)]
    pub fn hp_pull_out_en(&self) -> HP_PULL_OUT_EN_R {
        HP_PULL_OUT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Analog Ramp Clk Div Freq Value : M (from 0 to 31, Default: 24).\n\nAna_Ramp_Clk= 24MHz/(M+1)\n\nDefault Ramp Clk Freq: 24MHz/(24+1)=960 kHz"]
    #[inline(always)]
    pub fn ramp_clk_div_m(&self) -> RAMP_CLK_DIV_M_R {
        RAMP_CLK_DIV_M_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Ramp Soft Reset"]
    #[inline(always)]
    pub fn ramp_srst(&self) -> RAMP_SRST_R {
        RAMP_SRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - RK Downward Decrease Finish and Rampen Pull Down Instruction"]
    #[inline(always)]
    pub fn ramp_fall_int(&self) -> RAMP_FALL_INT_R {
        RAMP_FALL_INT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RAMP Fall Int Enable"]
    #[inline(always)]
    pub fn ramp_fall_int_en(&self) -> RAMP_FALL_INT_EN_R {
        RAMP_FALL_INT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RK Increase Upward Finish and Rampen Pull Down Instruction"]
    #[inline(always)]
    pub fn ramp_rise_int(&self) -> RAMP_RISE_INT_R {
        RAMP_RISE_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RAMP Rise Interrupt Enable"]
    #[inline(always)]
    pub fn ramp_rise_int_en(&self) -> RAMP_RISE_INT_EN_R {
        RAMP_RISE_INT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ramp Digital Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RD_EN_W<0> {
        RD_EN_W::new(self)
    }
    #[doc = "Bit 1 - Ramp Manual Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmc_en(&mut self) -> RMC_EN_W<1> {
        RMC_EN_W::new(self)
    }
    #[doc = "Bit 2 - Ramp Manual Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmu_en(&mut self) -> RMU_EN_W<2> {
        RMU_EN_W::new(self)
    }
    #[doc = "Bit 3 - Ramp Manual Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmd_en(&mut self) -> RMD_EN_W<3> {
        RMD_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - RK Frequency Gear, Control Ramp Rise/Fall Total Time\n\nRamp Rise/Fall Total Time = (Ramp Step/Ramp Clk Freq)*4096"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_step(&mut self) -> RAMP_STEP_W<4> {
        RAMP_STEP_W::new(self)
    }
    #[doc = "Bits 8:9 - Gap Step"]
    #[inline(always)]
    #[must_use]
    pub fn gap_step(&mut self) -> GAP_STEP_W<8> {
        GAP_STEP_W::new(self)
    }
    #[doc = "Bits 12:14 - Ramp Hold Step\n\nRamp Hold Time = Ramp Hold Step/Ramp Clk Freq"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_hold_step(&mut self) -> RAMP_HOLD_STEP_W<12> {
        RAMP_HOLD_STEP_W::new(self)
    }
    #[doc = "Bit 15 - Headphone Pullout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_pull_out_en(&mut self) -> HP_PULL_OUT_EN_W<15> {
        HP_PULL_OUT_EN_W::new(self)
    }
    #[doc = "Bits 16:20 - Analog Ramp Clk Div Freq Value : M (from 0 to 31, Default: 24).\n\nAna_Ramp_Clk= 24MHz/(M+1)\n\nDefault Ramp Clk Freq: 24MHz/(24+1)=960 kHz"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_clk_div_m(&mut self) -> RAMP_CLK_DIV_M_W<16> {
        RAMP_CLK_DIV_M_W::new(self)
    }
    #[doc = "Bit 24 - Ramp Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_srst(&mut self) -> RAMP_SRST_W<24> {
        RAMP_SRST_W::new(self)
    }
    #[doc = "Bit 28 - RK Downward Decrease Finish and Rampen Pull Down Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_fall_int(&mut self) -> RAMP_FALL_INT_W<28> {
        RAMP_FALL_INT_W::new(self)
    }
    #[doc = "Bit 29 - RAMP Fall Int Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_fall_int_en(&mut self) -> RAMP_FALL_INT_EN_W<29> {
        RAMP_FALL_INT_EN_W::new(self)
    }
    #[doc = "Bit 30 - RK Increase Upward Finish and Rampen Pull Down Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_rise_int(&mut self) -> RAMP_RISE_INT_W<30> {
        RAMP_RISE_INT_W::new(self)
    }
    #[doc = "Bit 31 - RAMP Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_rise_int_en(&mut self) -> RAMP_RISE_INT_EN_W<31> {
        RAMP_RISE_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIAS Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramp](index.html) module"]
pub struct RAMP_SPEC;
impl crate::RegisterSpec for RAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramp::R](R) reader structure"]
impl crate::Readable for RAMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramp::W](W) writer structure"]
impl crate::Writable for RAMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ramp to value 0"]
impl crate::Resettable for RAMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
