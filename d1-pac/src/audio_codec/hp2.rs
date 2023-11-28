#[doc = "Register `hp2` reader"]
pub type R = crate::R<HP2_SPEC>;
#[doc = "Register `hp2` writer"]
pub type W = crate::W<HP2_SPEC>;
#[doc = "Field `hpfb_buf_output_current` reader - Headphone Feedback Buffer Output Current Select\n\nI = 7uA"]
pub type HPFB_BUF_OUTPUT_CURRENT_R = crate::FieldReader<HPFB_BUF_OUTPUT_CURRENT_A>;
#[doc = "Headphone Feedback Buffer Output Current Select\n\nI = 7uA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPFB_BUF_OUTPUT_CURRENT_A {
    #[doc = "0: `0`"]
    I35 = 0,
    #[doc = "1: `1`"]
    I28 = 1,
    #[doc = "2: `10`"]
    I45 = 2,
    #[doc = "3: `11`"]
    I38 = 3,
}
impl From<HPFB_BUF_OUTPUT_CURRENT_A> for u8 {
    #[inline(always)]
    fn from(variant: HPFB_BUF_OUTPUT_CURRENT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPFB_BUF_OUTPUT_CURRENT_A {
    type Ux = u8;
}
impl HPFB_BUF_OUTPUT_CURRENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPFB_BUF_OUTPUT_CURRENT_A {
        match self.bits {
            0 => HPFB_BUF_OUTPUT_CURRENT_A::I35,
            1 => HPFB_BUF_OUTPUT_CURRENT_A::I28,
            2 => HPFB_BUF_OUTPUT_CURRENT_A::I45,
            3 => HPFB_BUF_OUTPUT_CURRENT_A::I38,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_i35(&self) -> bool {
        *self == HPFB_BUF_OUTPUT_CURRENT_A::I35
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_i28(&self) -> bool {
        *self == HPFB_BUF_OUTPUT_CURRENT_A::I28
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_i45(&self) -> bool {
        *self == HPFB_BUF_OUTPUT_CURRENT_A::I45
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i38(&self) -> bool {
        *self == HPFB_BUF_OUTPUT_CURRENT_A::I38
    }
}
#[doc = "Field `hpfb_buf_output_current` writer - Headphone Feedback Buffer Output Current Select\n\nI = 7uA"]
pub type HPFB_BUF_OUTPUT_CURRENT_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, HPFB_BUF_OUTPUT_CURRENT_A>;
impl<'a, REG> HPFB_BUF_OUTPUT_CURRENT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn i35(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_BUF_OUTPUT_CURRENT_A::I35)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn i28(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_BUF_OUTPUT_CURRENT_A::I28)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn i45(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_BUF_OUTPUT_CURRENT_A::I45)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i38(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_BUF_OUTPUT_CURRENT_A::I38)
    }
}
#[doc = "Field `ramp_final_state_res` reader - Ramp Final State Resistor"]
pub type RAMP_FINAL_STATE_RES_R = crate::FieldReader<RAMP_FINAL_STATE_RES_A>;
#[doc = "Ramp Final State Resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMP_FINAL_STATE_RES_A {
    #[doc = "0: `0`"]
    R2500 = 0,
    #[doc = "1: `1`"]
    R5K = 1,
    #[doc = "2: `10`"]
    R10K = 2,
    #[doc = "3: `11`"]
    R20K = 3,
}
impl From<RAMP_FINAL_STATE_RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMP_FINAL_STATE_RES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMP_FINAL_STATE_RES_A {
    type Ux = u8;
}
impl RAMP_FINAL_STATE_RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMP_FINAL_STATE_RES_A {
        match self.bits {
            0 => RAMP_FINAL_STATE_RES_A::R2500,
            1 => RAMP_FINAL_STATE_RES_A::R5K,
            2 => RAMP_FINAL_STATE_RES_A::R10K,
            3 => RAMP_FINAL_STATE_RES_A::R20K,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_r2500(&self) -> bool {
        *self == RAMP_FINAL_STATE_RES_A::R2500
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        *self == RAMP_FINAL_STATE_RES_A::R5K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_r10k(&self) -> bool {
        *self == RAMP_FINAL_STATE_RES_A::R10K
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_r20k(&self) -> bool {
        *self == RAMP_FINAL_STATE_RES_A::R20K
    }
}
#[doc = "Field `ramp_final_state_res` writer - Ramp Final State Resistor"]
pub type RAMP_FINAL_STATE_RES_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, RAMP_FINAL_STATE_RES_A>;
impl<'a, REG> RAMP_FINAL_STATE_RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r2500(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_FINAL_STATE_RES_A::R2500)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r5k(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_FINAL_STATE_RES_A::R5K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r10k(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_FINAL_STATE_RES_A::R10K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r20k(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_FINAL_STATE_RES_A::R20K)
    }
}
#[doc = "Field `ramp_out_en` reader - Ramp Output Switch Enable"]
pub type RAMP_OUT_EN_R = crate::BitReader<RAMP_OUT_EN_A>;
#[doc = "Ramp Output Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_OUT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RAMP_OUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_OUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_OUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMP_OUT_EN_A {
        match self.bits {
            false => RAMP_OUT_EN_A::DISABLE,
            true => RAMP_OUT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAMP_OUT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAMP_OUT_EN_A::ENABLE
    }
}
#[doc = "Field `ramp_out_en` writer - Ramp Output Switch Enable"]
pub type RAMP_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG, RAMP_OUT_EN_A>;
impl<'a, REG> RAMP_OUT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_OUT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_OUT_EN_A::ENABLE)
    }
}
#[doc = "Field `ramp_final_control` reader - Headphone Ramp Final Step Control"]
pub type RAMP_FINAL_CONTROL_R = crate::BitReader<RAMP_FINAL_CONTROL_A>;
#[doc = "Headphone Ramp Final Step Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMP_FINAL_CONTROL_A {
    #[doc = "0: `0`"]
    SELECT_RAMP = 0,
    #[doc = "1: `1`"]
    SELECT_HPFB_BUFFER = 1,
}
impl From<RAMP_FINAL_CONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: RAMP_FINAL_CONTROL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMP_FINAL_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMP_FINAL_CONTROL_A {
        match self.bits {
            false => RAMP_FINAL_CONTROL_A::SELECT_RAMP,
            true => RAMP_FINAL_CONTROL_A::SELECT_HPFB_BUFFER,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_select_ramp(&self) -> bool {
        *self == RAMP_FINAL_CONTROL_A::SELECT_RAMP
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_select_hpfb_buffer(&self) -> bool {
        *self == RAMP_FINAL_CONTROL_A::SELECT_HPFB_BUFFER
    }
}
#[doc = "Field `ramp_final_control` writer - Headphone Ramp Final Step Control"]
pub type RAMP_FINAL_CONTROL_W<'a, REG> = crate::BitWriter<'a, REG, RAMP_FINAL_CONTROL_A>;
impl<'a, REG> RAMP_FINAL_CONTROL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn select_ramp(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_FINAL_CONTROL_A::SELECT_RAMP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn select_hpfb_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(RAMP_FINAL_CONTROL_A::SELECT_HPFB_BUFFER)
    }
}
#[doc = "Field `hpfb_in_en` reader - Headphone Feedback PAD IN Switch Enable"]
pub type HPFB_IN_EN_R = crate::BitReader<HPFB_IN_EN_A>;
#[doc = "Headphone Feedback PAD IN Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPFB_IN_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HPFB_IN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPFB_IN_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HPFB_IN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPFB_IN_EN_A {
        match self.bits {
            false => HPFB_IN_EN_A::DISABLE,
            true => HPFB_IN_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HPFB_IN_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HPFB_IN_EN_A::ENABLE
    }
}
#[doc = "Field `hpfb_in_en` writer - Headphone Feedback PAD IN Switch Enable"]
pub type HPFB_IN_EN_W<'a, REG> = crate::BitWriter<'a, REG, HPFB_IN_EN_A>;
impl<'a, REG> HPFB_IN_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_IN_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_IN_EN_A::ENABLE)
    }
}
#[doc = "Field `rampen` reader - Ramp DAC Enable"]
pub type RAMPEN_R = crate::BitReader<RAMPEN_A>;
#[doc = "Ramp DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMPEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RAMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMPEN_A {
        match self.bits {
            false => RAMPEN_A::DISABLE,
            true => RAMPEN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAMPEN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAMPEN_A::ENABLE
    }
}
#[doc = "Field `rampen` writer - Ramp DAC Enable"]
pub type RAMPEN_W<'a, REG> = crate::BitWriter<'a, REG, RAMPEN_A>;
impl<'a, REG> RAMPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPEN_A::ENABLE)
    }
}
#[doc = "Field `rswitch` reader - RSwitch"]
pub type RSWITCH_R = crate::BitReader<RSWITCH_A>;
#[doc = "RSwitch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSWITCH_A {
    #[doc = "0: `0`"]
    HPOUT = 0,
    #[doc = "1: `1`"]
    VRA1 = 1,
}
impl From<RSWITCH_A> for bool {
    #[inline(always)]
    fn from(variant: RSWITCH_A) -> Self {
        variant as u8 != 0
    }
}
impl RSWITCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSWITCH_A {
        match self.bits {
            false => RSWITCH_A::HPOUT,
            true => RSWITCH_A::VRA1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hpout(&self) -> bool {
        *self == RSWITCH_A::HPOUT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_vra1(&self) -> bool {
        *self == RSWITCH_A::VRA1
    }
}
#[doc = "Field `rswitch` writer - RSwitch"]
pub type RSWITCH_W<'a, REG> = crate::BitWriter<'a, REG, RSWITCH_A>;
impl<'a, REG> RSWITCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hpout(self) -> &'a mut crate::W<REG> {
        self.variant(RSWITCH_A::HPOUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn vra1(self) -> &'a mut crate::W<REG> {
        self.variant(RSWITCH_A::VRA1)
    }
}
#[doc = "Field `hp_drvouten` reader - Headphone Driver Output Enable"]
pub type HP_DRVOUTEN_R = crate::BitReader<HP_DRVOUTEN_A>;
#[doc = "Headphone Driver Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_DRVOUTEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HP_DRVOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HP_DRVOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_DRVOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HP_DRVOUTEN_A {
        match self.bits {
            false => HP_DRVOUTEN_A::DISABLE,
            true => HP_DRVOUTEN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HP_DRVOUTEN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HP_DRVOUTEN_A::ENABLE
    }
}
#[doc = "Field `hp_drvouten` writer - Headphone Driver Output Enable"]
pub type HP_DRVOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, HP_DRVOUTEN_A>;
impl<'a, REG> HP_DRVOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HP_DRVOUTEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HP_DRVOUTEN_A::ENABLE)
    }
}
#[doc = "Field `hp_drven` reader - Headphone Driver Enable"]
pub type HP_DRVEN_R = crate::BitReader<HP_DRVEN_A>;
#[doc = "Headphone Driver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_DRVEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HP_DRVEN_A> for bool {
    #[inline(always)]
    fn from(variant: HP_DRVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_DRVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HP_DRVEN_A {
        match self.bits {
            false => HP_DRVEN_A::DISABLE,
            true => HP_DRVEN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HP_DRVEN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HP_DRVEN_A::ENABLE
    }
}
#[doc = "Field `hp_drven` writer - Headphone Driver Enable"]
pub type HP_DRVEN_W<'a, REG> = crate::BitWriter<'a, REG, HP_DRVEN_A>;
impl<'a, REG> HP_DRVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HP_DRVEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HP_DRVEN_A::ENABLE)
    }
}
#[doc = "Field `iophp` reader - Headphone L/R OP Bias Current Select"]
pub type IOPHP_R = crate::FieldReader<IOPHP_A>;
#[doc = "Headphone L/R OP Bias Current Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOPHP_A {
    #[doc = "0: `0`"]
    C6U = 0,
    #[doc = "1: `1`"]
    C7U = 1,
    #[doc = "2: `10`"]
    C8U = 2,
    #[doc = "3: `11`"]
    C9U = 3,
}
impl From<IOPHP_A> for u8 {
    #[inline(always)]
    fn from(variant: IOPHP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOPHP_A {
    type Ux = u8;
}
impl IOPHP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOPHP_A {
        match self.bits {
            0 => IOPHP_A::C6U,
            1 => IOPHP_A::C7U,
            2 => IOPHP_A::C8U,
            3 => IOPHP_A::C9U,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_c6u(&self) -> bool {
        *self == IOPHP_A::C6U
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_c7u(&self) -> bool {
        *self == IOPHP_A::C7U
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_c8u(&self) -> bool {
        *self == IOPHP_A::C8U
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_c9u(&self) -> bool {
        *self == IOPHP_A::C9U
    }
}
#[doc = "Field `iophp` writer - Headphone L/R OP Bias Current Select"]
pub type IOPHP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IOPHP_A>;
impl<'a, REG> IOPHP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn c6u(self) -> &'a mut crate::W<REG> {
        self.variant(IOPHP_A::C6U)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn c7u(self) -> &'a mut crate::W<REG> {
        self.variant(IOPHP_A::C7U)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn c8u(self) -> &'a mut crate::W<REG> {
        self.variant(IOPHP_A::C8U)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn c9u(self) -> &'a mut crate::W<REG> {
        self.variant(IOPHP_A::C9U)
    }
}
#[doc = "Field `opdrv_cur` reader - Headphone OP Output Stage Current Setting"]
pub type OPDRV_CUR_R = crate::FieldReader<OPDRV_CUR_A>;
#[doc = "Headphone OP Output Stage Current Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPDRV_CUR_A {
    #[doc = "0: `0`"]
    MIN = 0,
    #[doc = "3: `11`"]
    MAX = 3,
}
impl From<OPDRV_CUR_A> for u8 {
    #[inline(always)]
    fn from(variant: OPDRV_CUR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPDRV_CUR_A {
    type Ux = u8;
}
impl OPDRV_CUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPDRV_CUR_A> {
        match self.bits {
            0 => Some(OPDRV_CUR_A::MIN),
            3 => Some(OPDRV_CUR_A::MAX),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == OPDRV_CUR_A::MIN
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == OPDRV_CUR_A::MAX
    }
}
#[doc = "Field `opdrv_cur` writer - Headphone OP Output Stage Current Setting"]
pub type OPDRV_CUR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OPDRV_CUR_A>;
impl<'a, REG> OPDRV_CUR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(OPDRV_CUR_A::MIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(OPDRV_CUR_A::MAX)
    }
}
#[doc = "Field `hpfb_res` reader - Headphone Feedback Big Resistor Control"]
pub type HPFB_RES_R = crate::FieldReader<HPFB_RES_A>;
#[doc = "Headphone Feedback Big Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPFB_RES_A {
    #[doc = "0: `0`"]
    R880K = 0,
    #[doc = "1: `1`"]
    R1000K = 1,
    #[doc = "2: `10`"]
    R1080K = 2,
    #[doc = "3: `11`"]
    R1200K = 3,
}
impl From<HPFB_RES_A> for u8 {
    #[inline(always)]
    fn from(variant: HPFB_RES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPFB_RES_A {
    type Ux = u8;
}
impl HPFB_RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPFB_RES_A {
        match self.bits {
            0 => HPFB_RES_A::R880K,
            1 => HPFB_RES_A::R1000K,
            2 => HPFB_RES_A::R1080K,
            3 => HPFB_RES_A::R1200K,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_r880k(&self) -> bool {
        *self == HPFB_RES_A::R880K
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_r1000k(&self) -> bool {
        *self == HPFB_RES_A::R1000K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_r1080k(&self) -> bool {
        *self == HPFB_RES_A::R1080K
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_r1200k(&self) -> bool {
        *self == HPFB_RES_A::R1200K
    }
}
#[doc = "Field `hpfb_res` writer - Headphone Feedback Big Resistor Control"]
pub type HPFB_RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HPFB_RES_A>;
impl<'a, REG> HPFB_RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r880k(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_RES_A::R880K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r1000k(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_RES_A::R1000K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r1080k(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_RES_A::R1080K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r1200k(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_RES_A::R1200K)
    }
}
#[doc = "Field `headphone_gain` reader - Headphone Gain"]
pub type HEADPHONE_GAIN_R = crate::FieldReader<HEADPHONE_GAIN_A>;
#[doc = "Headphone Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HEADPHONE_GAIN_A {
    #[doc = "0: `0`"]
    DB0 = 0,
    #[doc = "1: `1`"]
    DB6 = 1,
    #[doc = "2: `10`"]
    DB12 = 2,
    #[doc = "3: `11`"]
    DB18 = 3,
    #[doc = "4: `100`"]
    DB24 = 4,
    #[doc = "5: `101`"]
    DB30 = 5,
    #[doc = "6: `110`"]
    DB36 = 6,
    #[doc = "7: `111`"]
    DB42 = 7,
}
impl From<HEADPHONE_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: HEADPHONE_GAIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HEADPHONE_GAIN_A {
    type Ux = u8;
}
impl HEADPHONE_GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HEADPHONE_GAIN_A {
        match self.bits {
            0 => HEADPHONE_GAIN_A::DB0,
            1 => HEADPHONE_GAIN_A::DB6,
            2 => HEADPHONE_GAIN_A::DB12,
            3 => HEADPHONE_GAIN_A::DB18,
            4 => HEADPHONE_GAIN_A::DB24,
            5 => HEADPHONE_GAIN_A::DB30,
            6 => HEADPHONE_GAIN_A::DB36,
            7 => HEADPHONE_GAIN_A::DB42,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_db0(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_db6(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB6
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_db12(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB12
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_db18(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB18
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_db24(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB24
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_db30(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB30
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_db36(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB36
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_db42(&self) -> bool {
        *self == HEADPHONE_GAIN_A::DB42
    }
}
#[doc = "Field `headphone_gain` writer - Headphone Gain"]
pub type HEADPHONE_GAIN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, HEADPHONE_GAIN_A>;
impl<'a, REG> HEADPHONE_GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn db0(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn db6(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB6)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn db12(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB12)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn db18(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB18)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn db24(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB24)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn db30(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB30)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn db36(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB36)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn db42(self) -> &'a mut crate::W<REG> {
        self.variant(HEADPHONE_GAIN_A::DB42)
    }
}
#[doc = "Field `hpfb_buf_en` reader - Headphone Feedback Buffer OP Enable"]
pub type HPFB_BUF_EN_R = crate::BitReader<HPFB_BUF_EN_A>;
#[doc = "Headphone Feedback Buffer OP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPFB_BUF_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<HPFB_BUF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPFB_BUF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HPFB_BUF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPFB_BUF_EN_A {
        match self.bits {
            false => HPFB_BUF_EN_A::DISABLE,
            true => HPFB_BUF_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HPFB_BUF_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HPFB_BUF_EN_A::ENABLE
    }
}
#[doc = "Field `hpfb_buf_en` writer - Headphone Feedback Buffer OP Enable"]
pub type HPFB_BUF_EN_W<'a, REG> = crate::BitWriter<'a, REG, HPFB_BUF_EN_A>;
impl<'a, REG> HPFB_BUF_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_BUF_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HPFB_BUF_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 8:9 - Headphone Feedback Buffer Output Current Select\n\nI = 7uA"]
    #[inline(always)]
    pub fn hpfb_buf_output_current(&self) -> HPFB_BUF_OUTPUT_CURRENT_R {
        HPFB_BUF_OUTPUT_CURRENT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Ramp Final State Resistor"]
    #[inline(always)]
    pub fn ramp_final_state_res(&self) -> RAMP_FINAL_STATE_RES_R {
        RAMP_FINAL_STATE_RES_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Ramp Output Switch Enable"]
    #[inline(always)]
    pub fn ramp_out_en(&self) -> RAMP_OUT_EN_R {
        RAMP_OUT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Headphone Ramp Final Step Control"]
    #[inline(always)]
    pub fn ramp_final_control(&self) -> RAMP_FINAL_CONTROL_R {
        RAMP_FINAL_CONTROL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Headphone Feedback PAD IN Switch Enable"]
    #[inline(always)]
    pub fn hpfb_in_en(&self) -> HPFB_IN_EN_R {
        HPFB_IN_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ramp DAC Enable"]
    #[inline(always)]
    pub fn rampen(&self) -> RAMPEN_R {
        RAMPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSwitch"]
    #[inline(always)]
    pub fn rswitch(&self) -> RSWITCH_R {
        RSWITCH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Headphone Driver Output Enable"]
    #[inline(always)]
    pub fn hp_drvouten(&self) -> HP_DRVOUTEN_R {
        HP_DRVOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Headphone Driver Enable"]
    #[inline(always)]
    pub fn hp_drven(&self) -> HP_DRVEN_R {
        HP_DRVEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Headphone L/R OP Bias Current Select"]
    #[inline(always)]
    pub fn iophp(&self) -> IOPHP_R {
        IOPHP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Headphone OP Output Stage Current Setting"]
    #[inline(always)]
    pub fn opdrv_cur(&self) -> OPDRV_CUR_R {
        OPDRV_CUR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Headphone Feedback Big Resistor Control"]
    #[inline(always)]
    pub fn hpfb_res(&self) -> HPFB_RES_R {
        HPFB_RES_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Headphone Gain"]
    #[inline(always)]
    pub fn headphone_gain(&self) -> HEADPHONE_GAIN_R {
        HEADPHONE_GAIN_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Headphone Feedback Buffer OP Enable"]
    #[inline(always)]
    pub fn hpfb_buf_en(&self) -> HPFB_BUF_EN_R {
        HPFB_BUF_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Headphone Feedback Buffer Output Current Select\n\nI = 7uA"]
    #[inline(always)]
    #[must_use]
    pub fn hpfb_buf_output_current(&mut self) -> HPFB_BUF_OUTPUT_CURRENT_W<HP2_SPEC> {
        HPFB_BUF_OUTPUT_CURRENT_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Ramp Final State Resistor"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_final_state_res(&mut self) -> RAMP_FINAL_STATE_RES_W<HP2_SPEC> {
        RAMP_FINAL_STATE_RES_W::new(self, 13)
    }
    #[doc = "Bit 15 - Ramp Output Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_out_en(&mut self) -> RAMP_OUT_EN_W<HP2_SPEC> {
        RAMP_OUT_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Headphone Ramp Final Step Control"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_final_control(&mut self) -> RAMP_FINAL_CONTROL_W<HP2_SPEC> {
        RAMP_FINAL_CONTROL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Headphone Feedback PAD IN Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpfb_in_en(&mut self) -> HPFB_IN_EN_W<HP2_SPEC> {
        HPFB_IN_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Ramp DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rampen(&mut self) -> RAMPEN_W<HP2_SPEC> {
        RAMPEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - RSwitch"]
    #[inline(always)]
    #[must_use]
    pub fn rswitch(&mut self) -> RSWITCH_W<HP2_SPEC> {
        RSWITCH_W::new(self, 19)
    }
    #[doc = "Bit 20 - Headphone Driver Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_drvouten(&mut self) -> HP_DRVOUTEN_W<HP2_SPEC> {
        HP_DRVOUTEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Headphone Driver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_drven(&mut self) -> HP_DRVEN_W<HP2_SPEC> {
        HP_DRVEN_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - Headphone L/R OP Bias Current Select"]
    #[inline(always)]
    #[must_use]
    pub fn iophp(&mut self) -> IOPHP_W<HP2_SPEC> {
        IOPHP_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Headphone OP Output Stage Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn opdrv_cur(&mut self) -> OPDRV_CUR_W<HP2_SPEC> {
        OPDRV_CUR_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Headphone Feedback Big Resistor Control"]
    #[inline(always)]
    #[must_use]
    pub fn hpfb_res(&mut self) -> HPFB_RES_W<HP2_SPEC> {
        HPFB_RES_W::new(self, 26)
    }
    #[doc = "Bits 28:30 - Headphone Gain"]
    #[inline(always)]
    #[must_use]
    pub fn headphone_gain(&mut self) -> HEADPHONE_GAIN_W<HP2_SPEC> {
        HEADPHONE_GAIN_W::new(self, 28)
    }
    #[doc = "Bit 31 - Headphone Feedback Buffer OP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpfb_buf_en(&mut self) -> HPFB_BUF_EN_W<HP2_SPEC> {
        HPFB_BUF_EN_W::new(self, 31)
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
#[doc = "Headphone2 Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP2_SPEC;
impl crate::RegisterSpec for HP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2::R`](R) reader structure"]
impl crate::Readable for HP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp2::W`](W) writer structure"]
impl crate::Writable for HP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hp2 to value 0"]
impl crate::Resettable for HP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
