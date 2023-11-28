#[doc = "Register `rescal_ctrl` reader"]
pub type R = crate::R<RESCAL_CTRL_SPEC>;
#[doc = "Register `rescal_ctrl` writer"]
pub type W = crate::W<RESCAL_CTRL_SPEC>;
#[doc = "Field `cal_en` reader - Auto Calibration Enable"]
pub type CAL_EN_R = crate::BitReader<CAL_EN_A>;
#[doc = "Auto Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CAL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL_EN_A {
        match self.bits {
            false => CAL_EN_A::DISABLE,
            true => CAL_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAL_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAL_EN_A::ENABLE
    }
}
#[doc = "Field `cal_en` writer - Auto Calibration Enable"]
pub type CAL_EN_W<'a, REG> = crate::BitWriter<'a, REG, CAL_EN_A>;
impl<'a, REG> CAL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAL_EN_A::ENABLE)
    }
}
#[doc = "Field `cal_ana_en` reader - Calibration Circuits Analog Enable"]
pub type CAL_ANA_EN_R = crate::BitReader<CAL_ANA_EN_A>;
#[doc = "Calibration Circuits Analog Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_ANA_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CAL_ANA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_ANA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CAL_ANA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL_ANA_EN_A {
        match self.bits {
            false => CAL_ANA_EN_A::DISABLE,
            true => CAL_ANA_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAL_ANA_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAL_ANA_EN_A::ENABLE
    }
}
#[doc = "Field `cal_ana_en` writer - Calibration Circuits Analog Enable"]
pub type CAL_ANA_EN_W<'a, REG> = crate::BitWriter<'a, REG, CAL_ANA_EN_A>;
impl<'a, REG> CAL_ANA_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAL_ANA_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAL_ANA_EN_A::ENABLE)
    }
}
#[doc = "Field `rescal_mode` reader - RESCAL Calibration Mode Select"]
pub type RESCAL_MODE_R = crate::BitReader<RESCAL_MODE_A>;
#[doc = "RESCAL Calibration Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESCAL_MODE_A {
    #[doc = "0: `0`"]
    AUTO_CALIBRATION = 0,
}
impl From<RESCAL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RESCAL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESCAL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESCAL_MODE_A> {
        match self.bits {
            false => Some(RESCAL_MODE_A::AUTO_CALIBRATION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_auto_calibration(&self) -> bool {
        *self == RESCAL_MODE_A::AUTO_CALIBRATION
    }
}
#[doc = "Field `rescal_mode` writer - RESCAL Calibration Mode Select"]
pub type RESCAL_MODE_W<'a, REG> = crate::BitWriter<'a, REG, RESCAL_MODE_A>;
impl<'a, REG> RESCAL_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn auto_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(RESCAL_MODE_A::AUTO_CALIBRATION)
    }
}
#[doc = "Field `ddr_res240_trimming_sel` reader - 240ohms Resistor Trimming Source Select"]
pub type DDR_RES240_TRIMMING_SEL_R = crate::BitReader<DDR_RES240_TRIMMING_SEL_A>;
#[doc = "240ohms Resistor Trimming Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDR_RES240_TRIMMING_SEL_A {
    #[doc = "0: `0`"]
    RESCAL = 0,
    #[doc = "1: `1`"]
    RES240_TRIM = 1,
}
impl From<DDR_RES240_TRIMMING_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DDR_RES240_TRIMMING_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DDR_RES240_TRIMMING_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDR_RES240_TRIMMING_SEL_A {
        match self.bits {
            false => DDR_RES240_TRIMMING_SEL_A::RESCAL,
            true => DDR_RES240_TRIMMING_SEL_A::RES240_TRIM,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_rescal(&self) -> bool {
        *self == DDR_RES240_TRIMMING_SEL_A::RESCAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_res240_trim(&self) -> bool {
        *self == DDR_RES240_TRIMMING_SEL_A::RES240_TRIM
    }
}
#[doc = "Field `ddr_res240_trimming_sel` writer - 240ohms Resistor Trimming Source Select"]
pub type DDR_RES240_TRIMMING_SEL_W<'a, REG> = crate::BitWriter<'a, REG, DDR_RES240_TRIMMING_SEL_A>;
impl<'a, REG> DDR_RES240_TRIMMING_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rescal(self) -> &'a mut crate::W<REG> {
        self.variant(DDR_RES240_TRIMMING_SEL_A::RESCAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn res240_trim(self) -> &'a mut crate::W<REG> {
        self.variant(DDR_RES240_TRIMMING_SEL_A::RES240_TRIM)
    }
}
impl R {
    #[doc = "Bit 0 - Auto Calibration Enable"]
    #[inline(always)]
    pub fn cal_en(&self) -> CAL_EN_R {
        CAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration Circuits Analog Enable"]
    #[inline(always)]
    pub fn cal_ana_en(&self) -> CAL_ANA_EN_R {
        CAL_ANA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESCAL Calibration Mode Select"]
    #[inline(always)]
    pub fn rescal_mode(&self) -> RESCAL_MODE_R {
        RESCAL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - 240ohms Resistor Trimming Source Select"]
    #[inline(always)]
    pub fn ddr_res240_trimming_sel(&self) -> DDR_RES240_TRIMMING_SEL_R {
        DDR_RES240_TRIMMING_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Calibration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cal_en(&mut self) -> CAL_EN_W<RESCAL_CTRL_SPEC> {
        CAL_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration Circuits Analog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cal_ana_en(&mut self) -> CAL_ANA_EN_W<RESCAL_CTRL_SPEC> {
        CAL_ANA_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - RESCAL Calibration Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn rescal_mode(&mut self) -> RESCAL_MODE_W<RESCAL_CTRL_SPEC> {
        RESCAL_MODE_W::new(self, 2)
    }
    #[doc = "Bit 8 - 240ohms Resistor Trimming Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_res240_trimming_sel(&mut self) -> DDR_RES240_TRIMMING_SEL_W<RESCAL_CTRL_SPEC> {
        DDR_RES240_TRIMMING_SEL_W::new(self, 8)
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
#[doc = "Resistor Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rescal_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rescal_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESCAL_CTRL_SPEC;
impl crate::RegisterSpec for RESCAL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rescal_ctrl::R`](R) reader structure"]
impl crate::Readable for RESCAL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rescal_ctrl::W`](W) writer structure"]
impl crate::Writable for RESCAL_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rescal_ctrl to value 0"]
impl crate::Resettable for RESCAL_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
