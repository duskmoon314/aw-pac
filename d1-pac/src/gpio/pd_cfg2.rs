#[doc = "Register `pd_cfg2` reader"]
pub type R = crate::R<PD_CFG2_SPEC>;
#[doc = "Register `pd_cfg2` writer"]
pub type W = crate::W<PD_CFG2_SPEC>;
#[doc = "Field `pd16_select` reader - PD16 Select"]
pub type PD16_SELECT_R = crate::FieldReader<PD16_SELECT_A>;
#[doc = "PD16 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD16_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D22 = 2,
    #[doc = "4: `100`"]
    DMIC_DATA3 = 4,
    #[doc = "14: `1110`"]
    PD_EINT16 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_CKP = 3,
    #[doc = "5: `101`"]
    PWM0 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD16_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD16_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD16_SELECT_A {
    type Ux = u8;
}
impl PD16_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD16_SELECT_A> {
        match self.bits {
            0 => Some(PD16_SELECT_A::INPUT),
            2 => Some(PD16_SELECT_A::LCD0_D22),
            4 => Some(PD16_SELECT_A::DMIC_DATA3),
            14 => Some(PD16_SELECT_A::PD_EINT16),
            1 => Some(PD16_SELECT_A::OUTPUT),
            3 => Some(PD16_SELECT_A::LVDS1_CKP),
            5 => Some(PD16_SELECT_A::PWM0),
            15 => Some(PD16_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD16_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d22(&self) -> bool {
        *self == PD16_SELECT_A::LCD0_D22
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dmic_data3(&self) -> bool {
        *self == PD16_SELECT_A::DMIC_DATA3
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint16(&self) -> bool {
        *self == PD16_SELECT_A::PD_EINT16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD16_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds1_ckp(&self) -> bool {
        *self == PD16_SELECT_A::LVDS1_CKP
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == PD16_SELECT_A::PWM0
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD16_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd16_select` writer - PD16 Select"]
pub type PD16_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD16_SELECT_A>;
impl<'a, REG> PD16_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d22(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::LCD0_D22)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dmic_data3(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::DMIC_DATA3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint16(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::PD_EINT16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_ckp(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::LVDS1_CKP)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::PWM0)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD16_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd17_select` reader - PD17 Select"]
pub type PD17_SELECT_R = crate::FieldReader<PD17_SELECT_A>;
#[doc = "PD17 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD17_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D23 = 2,
    #[doc = "4: `100`"]
    DMIC_DATA2 = 4,
    #[doc = "14: `1110`"]
    PD_EINT17 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_CKN = 3,
    #[doc = "5: `101`"]
    PWM1 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD17_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD17_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD17_SELECT_A {
    type Ux = u8;
}
impl PD17_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD17_SELECT_A> {
        match self.bits {
            0 => Some(PD17_SELECT_A::INPUT),
            2 => Some(PD17_SELECT_A::LCD0_D23),
            4 => Some(PD17_SELECT_A::DMIC_DATA2),
            14 => Some(PD17_SELECT_A::PD_EINT17),
            1 => Some(PD17_SELECT_A::OUTPUT),
            3 => Some(PD17_SELECT_A::LVDS1_CKN),
            5 => Some(PD17_SELECT_A::PWM1),
            15 => Some(PD17_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD17_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_d23(&self) -> bool {
        *self == PD17_SELECT_A::LCD0_D23
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dmic_data2(&self) -> bool {
        *self == PD17_SELECT_A::DMIC_DATA2
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint17(&self) -> bool {
        *self == PD17_SELECT_A::PD_EINT17
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD17_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds1_ckn(&self) -> bool {
        *self == PD17_SELECT_A::LVDS1_CKN
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == PD17_SELECT_A::PWM1
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD17_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd17_select` writer - PD17 Select"]
pub type PD17_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD17_SELECT_A>;
impl<'a, REG> PD17_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d23(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::LCD0_D23)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dmic_data2(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::DMIC_DATA2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint17(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::PD_EINT17)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_ckn(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::LVDS1_CKN)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::PWM1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD17_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd18_select` reader - PD18 Select"]
pub type PD18_SELECT_R = crate::FieldReader<PD18_SELECT_A>;
#[doc = "PD18 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD18_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_CLK = 2,
    #[doc = "4: `100`"]
    DMIC_DATA1 = 4,
    #[doc = "14: `1110`"]
    PD_EINT18 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V3P = 3,
    #[doc = "5: `101`"]
    PWM2 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD18_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD18_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD18_SELECT_A {
    type Ux = u8;
}
impl PD18_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD18_SELECT_A> {
        match self.bits {
            0 => Some(PD18_SELECT_A::INPUT),
            2 => Some(PD18_SELECT_A::LCD0_CLK),
            4 => Some(PD18_SELECT_A::DMIC_DATA1),
            14 => Some(PD18_SELECT_A::PD_EINT18),
            1 => Some(PD18_SELECT_A::OUTPUT),
            3 => Some(PD18_SELECT_A::LVDS1_V3P),
            5 => Some(PD18_SELECT_A::PWM2),
            15 => Some(PD18_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD18_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_clk(&self) -> bool {
        *self == PD18_SELECT_A::LCD0_CLK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dmic_data1(&self) -> bool {
        *self == PD18_SELECT_A::DMIC_DATA1
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint18(&self) -> bool {
        *self == PD18_SELECT_A::PD_EINT18
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD18_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds1_v3p(&self) -> bool {
        *self == PD18_SELECT_A::LVDS1_V3P
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == PD18_SELECT_A::PWM2
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD18_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd18_select` writer - PD18 Select"]
pub type PD18_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD18_SELECT_A>;
impl<'a, REG> PD18_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::LCD0_CLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dmic_data1(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::DMIC_DATA1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint18(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::PD_EINT18)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v3p(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::LVDS1_V3P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::PWM2)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD18_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd19_select` reader - PD19 Select"]
pub type PD19_SELECT_R = crate::FieldReader<PD19_SELECT_A>;
#[doc = "PD19 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD19_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_DE = 2,
    #[doc = "4: `100`"]
    DMIC_DATA0 = 4,
    #[doc = "14: `1110`"]
    PD_EINT19 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V3N = 3,
    #[doc = "5: `101`"]
    PWM3 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD19_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD19_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD19_SELECT_A {
    type Ux = u8;
}
impl PD19_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD19_SELECT_A> {
        match self.bits {
            0 => Some(PD19_SELECT_A::INPUT),
            2 => Some(PD19_SELECT_A::LCD0_DE),
            4 => Some(PD19_SELECT_A::DMIC_DATA0),
            14 => Some(PD19_SELECT_A::PD_EINT19),
            1 => Some(PD19_SELECT_A::OUTPUT),
            3 => Some(PD19_SELECT_A::LVDS1_V3N),
            5 => Some(PD19_SELECT_A::PWM3),
            15 => Some(PD19_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD19_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_de(&self) -> bool {
        *self == PD19_SELECT_A::LCD0_DE
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dmic_data0(&self) -> bool {
        *self == PD19_SELECT_A::DMIC_DATA0
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint19(&self) -> bool {
        *self == PD19_SELECT_A::PD_EINT19
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD19_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lvds1_v3n(&self) -> bool {
        *self == PD19_SELECT_A::LVDS1_V3N
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == PD19_SELECT_A::PWM3
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD19_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd19_select` writer - PD19 Select"]
pub type PD19_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD19_SELECT_A>;
impl<'a, REG> PD19_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_de(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::LCD0_DE)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dmic_data0(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::DMIC_DATA0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint19(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::PD_EINT19)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v3n(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::LVDS1_V3N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::PWM3)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD19_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd20_select` reader - PD20 Select"]
pub type PD20_SELECT_R = crate::FieldReader<PD20_SELECT_A>;
#[doc = "PD20 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD20_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_HSYNC = 2,
    #[doc = "4: `100`"]
    DMIC_CLK = 4,
    #[doc = "14: `1110`"]
    PD_EINT20 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SCK = 3,
    #[doc = "5: `101`"]
    PWM4 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD20_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD20_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD20_SELECT_A {
    type Ux = u8;
}
impl PD20_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD20_SELECT_A> {
        match self.bits {
            0 => Some(PD20_SELECT_A::INPUT),
            2 => Some(PD20_SELECT_A::LCD0_HSYNC),
            4 => Some(PD20_SELECT_A::DMIC_CLK),
            14 => Some(PD20_SELECT_A::PD_EINT20),
            1 => Some(PD20_SELECT_A::OUTPUT),
            3 => Some(PD20_SELECT_A::TWI2_SCK),
            5 => Some(PD20_SELECT_A::PWM4),
            15 => Some(PD20_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD20_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_hsync(&self) -> bool {
        *self == PD20_SELECT_A::LCD0_HSYNC
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_dmic_clk(&self) -> bool {
        *self == PD20_SELECT_A::DMIC_CLK
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint20(&self) -> bool {
        *self == PD20_SELECT_A::PD_EINT20
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD20_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        *self == PD20_SELECT_A::TWI2_SCK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == PD20_SELECT_A::PWM4
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD20_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd20_select` writer - PD20 Select"]
pub type PD20_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD20_SELECT_A>;
impl<'a, REG> PD20_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_hsync(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::LCD0_HSYNC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dmic_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::DMIC_CLK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint20(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::PD_EINT20)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::TWI2_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::PWM4)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD20_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd21_select` reader - PD21 Select"]
pub type PD21_SELECT_R = crate::FieldReader<PD21_SELECT_A>;
#[doc = "PD21 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD21_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_VSYNC = 2,
    #[doc = "4: `100`"]
    UART1_TX = 4,
    #[doc = "14: `1110`"]
    PD_EINT21 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SDA = 3,
    #[doc = "5: `101`"]
    PWM5 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD21_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD21_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD21_SELECT_A {
    type Ux = u8;
}
impl PD21_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD21_SELECT_A> {
        match self.bits {
            0 => Some(PD21_SELECT_A::INPUT),
            2 => Some(PD21_SELECT_A::LCD0_VSYNC),
            4 => Some(PD21_SELECT_A::UART1_TX),
            14 => Some(PD21_SELECT_A::PD_EINT21),
            1 => Some(PD21_SELECT_A::OUTPUT),
            3 => Some(PD21_SELECT_A::TWI2_SDA),
            5 => Some(PD21_SELECT_A::PWM5),
            15 => Some(PD21_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD21_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_lcd0_vsync(&self) -> bool {
        *self == PD21_SELECT_A::LCD0_VSYNC
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PD21_SELECT_A::UART1_TX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint21(&self) -> bool {
        *self == PD21_SELECT_A::PD_EINT21
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD21_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        *self == PD21_SELECT_A::TWI2_SDA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == PD21_SELECT_A::PWM5
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD21_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd21_select` writer - PD21 Select"]
pub type PD21_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD21_SELECT_A>;
impl<'a, REG> PD21_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_vsync(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::LCD0_VSYNC)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::UART1_TX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint21(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::PD_EINT21)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::TWI2_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::PWM5)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD21_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd22_select` reader - PD22 Select"]
pub type PD22_SELECT_R = crate::FieldReader<PD22_SELECT_A>;
#[doc = "PD22 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD22_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    OWA_OUT = 2,
    #[doc = "4: `100`"]
    UART1_RX = 4,
    #[doc = "14: `1110`"]
    PD_EINT22 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    IR_RX = 3,
    #[doc = "5: `101`"]
    PWM7 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD22_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD22_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD22_SELECT_A {
    type Ux = u8;
}
impl PD22_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD22_SELECT_A> {
        match self.bits {
            0 => Some(PD22_SELECT_A::INPUT),
            2 => Some(PD22_SELECT_A::OWA_OUT),
            4 => Some(PD22_SELECT_A::UART1_RX),
            14 => Some(PD22_SELECT_A::PD_EINT22),
            1 => Some(PD22_SELECT_A::OUTPUT),
            3 => Some(PD22_SELECT_A::IR_RX),
            5 => Some(PD22_SELECT_A::PWM7),
            15 => Some(PD22_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD22_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_owa_out(&self) -> bool {
        *self == PD22_SELECT_A::OWA_OUT
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PD22_SELECT_A::UART1_RX
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pd_eint22(&self) -> bool {
        *self == PD22_SELECT_A::PD_EINT22
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD22_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PD22_SELECT_A::IR_RX
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pwm7(&self) -> bool {
        *self == PD22_SELECT_A::PWM7
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD22_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd22_select` writer - PD22 Select"]
pub type PD22_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PD22_SELECT_A>;
impl<'a, REG> PD22_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn owa_out(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::OWA_OUT)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::UART1_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint22(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::PD_EINT22)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::IR_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm7(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::PWM7)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD22_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PD16 Select"]
    #[inline(always)]
    pub fn pd16_select(&self) -> PD16_SELECT_R {
        PD16_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PD17 Select"]
    #[inline(always)]
    pub fn pd17_select(&self) -> PD17_SELECT_R {
        PD17_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PD18 Select"]
    #[inline(always)]
    pub fn pd18_select(&self) -> PD18_SELECT_R {
        PD18_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PD19 Select"]
    #[inline(always)]
    pub fn pd19_select(&self) -> PD19_SELECT_R {
        PD19_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PD20 Select"]
    #[inline(always)]
    pub fn pd20_select(&self) -> PD20_SELECT_R {
        PD20_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PD21 Select"]
    #[inline(always)]
    pub fn pd21_select(&self) -> PD21_SELECT_R {
        PD21_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PD22 Select"]
    #[inline(always)]
    pub fn pd22_select(&self) -> PD22_SELECT_R {
        PD22_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PD16 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd16_select(&mut self) -> PD16_SELECT_W<PD_CFG2_SPEC> {
        PD16_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PD17 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd17_select(&mut self) -> PD17_SELECT_W<PD_CFG2_SPEC> {
        PD17_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - PD18 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd18_select(&mut self) -> PD18_SELECT_W<PD_CFG2_SPEC> {
        PD18_SELECT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PD19 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd19_select(&mut self) -> PD19_SELECT_W<PD_CFG2_SPEC> {
        PD19_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - PD20 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd20_select(&mut self) -> PD20_SELECT_W<PD_CFG2_SPEC> {
        PD20_SELECT_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - PD21 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd21_select(&mut self) -> PD21_SELECT_W<PD_CFG2_SPEC> {
        PD21_SELECT_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - PD22 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd22_select(&mut self) -> PD22_SELECT_W<PD_CFG2_SPEC> {
        PD22_SELECT_W::new(self, 24)
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
#[doc = "PD Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_CFG2_SPEC;
impl crate::RegisterSpec for PD_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_cfg2::R`](R) reader structure"]
impl crate::Readable for PD_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_cfg2::W`](W) writer structure"]
impl crate::Writable for PD_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_cfg2 to value 0"]
impl crate::Resettable for PD_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
