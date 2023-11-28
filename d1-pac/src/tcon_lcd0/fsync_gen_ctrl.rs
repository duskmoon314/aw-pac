#[doc = "Register `fsync_gen_ctrl` reader"]
pub type R = crate::R<FSYNC_GEN_CTRL_SPEC>;
#[doc = "Register `fsync_gen_ctrl` writer"]
pub type W = crate::W<FSYNC_GEN_CTRL_SPEC>;
#[doc = "Field `fsync_gen_en` reader - Fsync Generate Enable"]
pub type FSYNC_GEN_EN_R = crate::BitReader<FSYNC_GEN_EN_A>;
#[doc = "Fsync Generate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSYNC_GEN_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FSYNC_GEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FSYNC_GEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FSYNC_GEN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSYNC_GEN_EN_A {
        match self.bits {
            false => FSYNC_GEN_EN_A::DISABLE,
            true => FSYNC_GEN_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSYNC_GEN_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FSYNC_GEN_EN_A::ENABLE
    }
}
#[doc = "Field `fsync_gen_en` writer - Fsync Generate Enable"]
pub type FSYNC_GEN_EN_W<'a, REG> = crate::BitWriter<'a, REG, FSYNC_GEN_EN_A>;
impl<'a, REG> FSYNC_GEN_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FSYNC_GEN_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FSYNC_GEN_EN_A::ENABLE)
    }
}
#[doc = "Field `sel_vsync_en` reader - Select Vsync Enable"]
pub type SEL_VSYNC_EN_R = crate::BitReader<SEL_VSYNC_EN_A>;
#[doc = "Select Vsync Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_VSYNC_EN_A {
    #[doc = "0: Select vsync falling edge to start state machine"]
    FALLING = 0,
    #[doc = "1: Select vsync rising edge to start state machine"]
    RISING = 1,
}
impl From<SEL_VSYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_VSYNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_VSYNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEL_VSYNC_EN_A {
        match self.bits {
            false => SEL_VSYNC_EN_A::FALLING,
            true => SEL_VSYNC_EN_A::RISING,
        }
    }
    #[doc = "Select vsync falling edge to start state machine"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == SEL_VSYNC_EN_A::FALLING
    }
    #[doc = "Select vsync rising edge to start state machine"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == SEL_VSYNC_EN_A::RISING
    }
}
#[doc = "Field `sel_vsync_en` writer - Select Vsync Enable"]
pub type SEL_VSYNC_EN_W<'a, REG> = crate::BitWriter<'a, REG, SEL_VSYNC_EN_A>;
impl<'a, REG> SEL_VSYNC_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select vsync falling edge to start state machine"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VSYNC_EN_A::FALLING)
    }
    #[doc = "Select vsync rising edge to start state machine"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VSYNC_EN_A::RISING)
    }
}
#[doc = "Field `hsync_pol_sel` reader - Hsync Polarity Select"]
pub type HSYNC_POL_SEL_R = crate::BitReader<HSYNC_POL_SEL_A>;
#[doc = "Hsync Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNC_POL_SEL_A {
    #[doc = "0: Normal"]
    NORMAL = 0,
    #[doc = "1: Opposite hsync to hysnc counter"]
    O_PPOSITE = 1,
}
impl From<HSYNC_POL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNC_POL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HSYNC_POL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSYNC_POL_SEL_A {
        match self.bits {
            false => HSYNC_POL_SEL_A::NORMAL,
            true => HSYNC_POL_SEL_A::O_PPOSITE,
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == HSYNC_POL_SEL_A::NORMAL
    }
    #[doc = "Opposite hsync to hysnc counter"]
    #[inline(always)]
    pub fn is_o_pposite(&self) -> bool {
        *self == HSYNC_POL_SEL_A::O_PPOSITE
    }
}
#[doc = "Field `hsync_pol_sel` writer - Hsync Polarity Select"]
pub type HSYNC_POL_SEL_W<'a, REG> = crate::BitWriter<'a, REG, HSYNC_POL_SEL_A>;
impl<'a, REG> HSYNC_POL_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(HSYNC_POL_SEL_A::NORMAL)
    }
    #[doc = "Opposite hsync to hysnc counter"]
    #[inline(always)]
    pub fn o_pposite(self) -> &'a mut crate::W<REG> {
        self.variant(HSYNC_POL_SEL_A::O_PPOSITE)
    }
}
#[doc = "Field `sensor_dis_value` reader - Sensor Disable Value"]
pub type SENSOR_DIS_VALUE_R = crate::BitReader<SENSOR_DIS_VALUE_A>;
#[doc = "Sensor Disable Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENSOR_DIS_VALUE_A {
    #[doc = "0: Fsync disable period output 0"]
    OUTPUT_0 = 0,
    #[doc = "1: Fsync disable period output 1"]
    OUTPUT_1 = 1,
}
impl From<SENSOR_DIS_VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: SENSOR_DIS_VALUE_A) -> Self {
        variant as u8 != 0
    }
}
impl SENSOR_DIS_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SENSOR_DIS_VALUE_A {
        match self.bits {
            false => SENSOR_DIS_VALUE_A::OUTPUT_0,
            true => SENSOR_DIS_VALUE_A::OUTPUT_1,
        }
    }
    #[doc = "Fsync disable period output 0"]
    #[inline(always)]
    pub fn is_output_0(&self) -> bool {
        *self == SENSOR_DIS_VALUE_A::OUTPUT_0
    }
    #[doc = "Fsync disable period output 1"]
    #[inline(always)]
    pub fn is_output_1(&self) -> bool {
        *self == SENSOR_DIS_VALUE_A::OUTPUT_1
    }
}
#[doc = "Field `sensor_dis_value` writer - Sensor Disable Value"]
pub type SENSOR_DIS_VALUE_W<'a, REG> = crate::BitWriter<'a, REG, SENSOR_DIS_VALUE_A>;
impl<'a, REG> SENSOR_DIS_VALUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fsync disable period output 0"]
    #[inline(always)]
    pub fn output_0(self) -> &'a mut crate::W<REG> {
        self.variant(SENSOR_DIS_VALUE_A::OUTPUT_0)
    }
    #[doc = "Fsync disable period output 1"]
    #[inline(always)]
    pub fn output_1(self) -> &'a mut crate::W<REG> {
        self.variant(SENSOR_DIS_VALUE_A::OUTPUT_1)
    }
}
#[doc = "Field `sensor_act0_value` reader - Sensor Active0 Value"]
pub type SENSOR_ACT0_VALUE_R = crate::BitReader<SENSOR_ACT0_VALUE_A>;
#[doc = "Sensor Active0 Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENSOR_ACT0_VALUE_A {
    #[doc = "0: Fsync active_0 period output 0"]
    OUTPUT_0 = 0,
    #[doc = "1: Fsync active_0 period output 1"]
    OUTPUT_1 = 1,
}
impl From<SENSOR_ACT0_VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: SENSOR_ACT0_VALUE_A) -> Self {
        variant as u8 != 0
    }
}
impl SENSOR_ACT0_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SENSOR_ACT0_VALUE_A {
        match self.bits {
            false => SENSOR_ACT0_VALUE_A::OUTPUT_0,
            true => SENSOR_ACT0_VALUE_A::OUTPUT_1,
        }
    }
    #[doc = "Fsync active_0 period output 0"]
    #[inline(always)]
    pub fn is_output_0(&self) -> bool {
        *self == SENSOR_ACT0_VALUE_A::OUTPUT_0
    }
    #[doc = "Fsync active_0 period output 1"]
    #[inline(always)]
    pub fn is_output_1(&self) -> bool {
        *self == SENSOR_ACT0_VALUE_A::OUTPUT_1
    }
}
#[doc = "Field `sensor_act0_value` writer - Sensor Active0 Value"]
pub type SENSOR_ACT0_VALUE_W<'a, REG> = crate::BitWriter<'a, REG, SENSOR_ACT0_VALUE_A>;
impl<'a, REG> SENSOR_ACT0_VALUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fsync active_0 period output 0"]
    #[inline(always)]
    pub fn output_0(self) -> &'a mut crate::W<REG> {
        self.variant(SENSOR_ACT0_VALUE_A::OUTPUT_0)
    }
    #[doc = "Fsync active_0 period output 1"]
    #[inline(always)]
    pub fn output_1(self) -> &'a mut crate::W<REG> {
        self.variant(SENSOR_ACT0_VALUE_A::OUTPUT_1)
    }
}
#[doc = "Field `sensor_act1_value` reader - Sensor Active1 Value"]
pub type SENSOR_ACT1_VALUE_R = crate::BitReader<SENSOR_ACT1_VALUE_A>;
#[doc = "Sensor Active1 Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENSOR_ACT1_VALUE_A {
    #[doc = "0: Fsync active_1 period output 0"]
    OUTPUT_0 = 0,
    #[doc = "1: Fsync active_1 period output 1"]
    OUTPUT_1 = 1,
}
impl From<SENSOR_ACT1_VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: SENSOR_ACT1_VALUE_A) -> Self {
        variant as u8 != 0
    }
}
impl SENSOR_ACT1_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SENSOR_ACT1_VALUE_A {
        match self.bits {
            false => SENSOR_ACT1_VALUE_A::OUTPUT_0,
            true => SENSOR_ACT1_VALUE_A::OUTPUT_1,
        }
    }
    #[doc = "Fsync active_1 period output 0"]
    #[inline(always)]
    pub fn is_output_0(&self) -> bool {
        *self == SENSOR_ACT1_VALUE_A::OUTPUT_0
    }
    #[doc = "Fsync active_1 period output 1"]
    #[inline(always)]
    pub fn is_output_1(&self) -> bool {
        *self == SENSOR_ACT1_VALUE_A::OUTPUT_1
    }
}
#[doc = "Field `sensor_act1_value` writer - Sensor Active1 Value"]
pub type SENSOR_ACT1_VALUE_W<'a, REG> = crate::BitWriter<'a, REG, SENSOR_ACT1_VALUE_A>;
impl<'a, REG> SENSOR_ACT1_VALUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fsync active_1 period output 0"]
    #[inline(always)]
    pub fn output_0(self) -> &'a mut crate::W<REG> {
        self.variant(SENSOR_ACT1_VALUE_A::OUTPUT_0)
    }
    #[doc = "Fsync active_1 period output 1"]
    #[inline(always)]
    pub fn output_1(self) -> &'a mut crate::W<REG> {
        self.variant(SENSOR_ACT1_VALUE_A::OUTPUT_1)
    }
}
#[doc = "Field `sensor_dis_time` reader - Delay 0-2047 Hsync Period\n\nWhen hsync_pol_sel is 0, the actual delay is sensor_dis_time-1.\n\nWhen hsync_pol_sel is 1, the actual delay is sensor_dis_time."]
pub type SENSOR_DIS_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `sensor_dis_time` writer - Delay 0-2047 Hsync Period\n\nWhen hsync_pol_sel is 0, the actual delay is sensor_dis_time-1.\n\nWhen hsync_pol_sel is 1, the actual delay is sensor_dis_time."]
pub type SENSOR_DIS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Fsync Generate Enable"]
    #[inline(always)]
    pub fn fsync_gen_en(&self) -> FSYNC_GEN_EN_R {
        FSYNC_GEN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Vsync Enable"]
    #[inline(always)]
    pub fn sel_vsync_en(&self) -> SEL_VSYNC_EN_R {
        SEL_VSYNC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hsync Polarity Select"]
    #[inline(always)]
    pub fn hsync_pol_sel(&self) -> HSYNC_POL_SEL_R {
        HSYNC_POL_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Sensor Disable Value"]
    #[inline(always)]
    pub fn sensor_dis_value(&self) -> SENSOR_DIS_VALUE_R {
        SENSOR_DIS_VALUE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sensor Active0 Value"]
    #[inline(always)]
    pub fn sensor_act0_value(&self) -> SENSOR_ACT0_VALUE_R {
        SENSOR_ACT0_VALUE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sensor Active1 Value"]
    #[inline(always)]
    pub fn sensor_act1_value(&self) -> SENSOR_ACT1_VALUE_R {
        SENSOR_ACT1_VALUE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:18 - Delay 0-2047 Hsync Period\n\nWhen hsync_pol_sel is 0, the actual delay is sensor_dis_time-1.\n\nWhen hsync_pol_sel is 1, the actual delay is sensor_dis_time."]
    #[inline(always)]
    pub fn sensor_dis_time(&self) -> SENSOR_DIS_TIME_R {
        SENSOR_DIS_TIME_R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Fsync Generate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsync_gen_en(&mut self) -> FSYNC_GEN_EN_W<FSYNC_GEN_CTRL_SPEC> {
        FSYNC_GEN_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select Vsync Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sel_vsync_en(&mut self) -> SEL_VSYNC_EN_W<FSYNC_GEN_CTRL_SPEC> {
        SEL_VSYNC_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hsync Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_pol_sel(&mut self) -> HSYNC_POL_SEL_W<FSYNC_GEN_CTRL_SPEC> {
        HSYNC_POL_SEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Sensor Disable Value"]
    #[inline(always)]
    #[must_use]
    pub fn sensor_dis_value(&mut self) -> SENSOR_DIS_VALUE_W<FSYNC_GEN_CTRL_SPEC> {
        SENSOR_DIS_VALUE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Sensor Active0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn sensor_act0_value(&mut self) -> SENSOR_ACT0_VALUE_W<FSYNC_GEN_CTRL_SPEC> {
        SENSOR_ACT0_VALUE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Sensor Active1 Value"]
    #[inline(always)]
    #[must_use]
    pub fn sensor_act1_value(&mut self) -> SENSOR_ACT1_VALUE_W<FSYNC_GEN_CTRL_SPEC> {
        SENSOR_ACT1_VALUE_W::new(self, 6)
    }
    #[doc = "Bits 8:18 - Delay 0-2047 Hsync Period\n\nWhen hsync_pol_sel is 0, the actual delay is sensor_dis_time-1.\n\nWhen hsync_pol_sel is 1, the actual delay is sensor_dis_time."]
    #[inline(always)]
    #[must_use]
    pub fn sensor_dis_time(&mut self) -> SENSOR_DIS_TIME_W<FSYNC_GEN_CTRL_SPEC> {
        SENSOR_DIS_TIME_W::new(self, 8)
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
#[doc = "FSYNC_GEN_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsync_gen_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsync_gen_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSYNC_GEN_CTRL_SPEC;
impl crate::RegisterSpec for FSYNC_GEN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsync_gen_ctrl::R`](R) reader structure"]
impl crate::Readable for FSYNC_GEN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsync_gen_ctrl::W`](W) writer structure"]
impl crate::Writable for FSYNC_GEN_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsync_gen_ctrl to value 0"]
impl crate::Resettable for FSYNC_GEN_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
