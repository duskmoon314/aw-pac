#[doc = "Register `twi_drv_ctrl` reader"]
pub type R = crate::R<TWI_DRV_CTRL_SPEC>;
#[doc = "Register `twi_drv_ctrl` writer"]
pub type W = crate::W<TWI_DRV_CTRL_SPEC>;
#[doc = "Field `twi_drv_en` reader - "]
pub type TWI_DRV_EN_R = crate::BitReader<TWI_DRV_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWI_DRV_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TWI_DRV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TWI_DRV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TWI_DRV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TWI_DRV_EN_A {
        match self.bits {
            false => TWI_DRV_EN_A::DISABLE,
            true => TWI_DRV_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TWI_DRV_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TWI_DRV_EN_A::ENABLE
    }
}
#[doc = "Field `twi_drv_en` writer - "]
pub type TWI_DRV_EN_W<'a, REG> = crate::BitWriter<'a, REG, TWI_DRV_EN_A>;
impl<'a, REG> TWI_DRV_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TWI_DRV_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TWI_DRV_EN_A::ENABLE)
    }
}
#[doc = "Field `soft_reset` reader - Software reset"]
pub type SOFT_RESET_R = crate::BitReader<SOFT_RESET_A>;
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFT_RESET_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<SOFT_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFT_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFT_RESET_A {
        match self.bits {
            false => SOFT_RESET_A::NORMAL,
            true => SOFT_RESET_A::RESET,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SOFT_RESET_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RESET_A::RESET
    }
}
#[doc = "Field `soft_reset` writer - Software reset"]
pub type SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG, SOFT_RESET_A>;
impl<'a, REG> SOFT_RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SOFT_RESET_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SOFT_RESET_A::RESET)
    }
}
#[doc = "Field `timeout_n` reader - Timeout number"]
pub type TIMEOUT_N_R = crate::FieldReader;
#[doc = "Field `timeout_n` writer - Timeout number"]
pub type TIMEOUT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `twi_sta` reader - TWI status"]
pub type TWI_STA_R = crate::FieldReader<TWI_STA_A>;
#[doc = "TWI status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWI_STA_A {
    #[doc = "0: bus error"]
    BE = 0,
    #[doc = "8: START condition transmitted"]
    SCT = 8,
    #[doc = "16: Repeated START condition transmitted"]
    RSCT = 16,
    #[doc = "24: Address + Write bit transmitted, ACK received"]
    AWBT_AR = 24,
    #[doc = "32: Address + Write bit transmitted, ACK not received"]
    AWBT_ANR = 32,
    #[doc = "40: Data byte transmitted in master mode, ACK received"]
    DBTM_AR = 40,
    #[doc = "48: Data byte transmitted in master mode, ACK not received"]
    DBTM_ANR = 48,
    #[doc = "56: Arbitration lost in address or data byte"]
    AL_A_DB = 56,
    #[doc = "64: Address + Read bit transmitted, ACK received"]
    ARBT_AR = 64,
    #[doc = "72: Address + Read bit transmitted, ACK not received"]
    ARBT_ANR = 72,
    #[doc = "80: Data byte received in master mode, ACK received"]
    DBRM_AR = 80,
    #[doc = "88: Data byte received in master mode, ACK not received"]
    DBRM_ANR = 88,
    #[doc = "1: Timeout when sending the 9th SCL clock"]
    T_S9SC = 1,
}
impl From<TWI_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: TWI_STA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TWI_STA_A {
    type Ux = u8;
}
impl TWI_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TWI_STA_A> {
        match self.bits {
            0 => Some(TWI_STA_A::BE),
            8 => Some(TWI_STA_A::SCT),
            16 => Some(TWI_STA_A::RSCT),
            24 => Some(TWI_STA_A::AWBT_AR),
            32 => Some(TWI_STA_A::AWBT_ANR),
            40 => Some(TWI_STA_A::DBTM_AR),
            48 => Some(TWI_STA_A::DBTM_ANR),
            56 => Some(TWI_STA_A::AL_A_DB),
            64 => Some(TWI_STA_A::ARBT_AR),
            72 => Some(TWI_STA_A::ARBT_ANR),
            80 => Some(TWI_STA_A::DBRM_AR),
            88 => Some(TWI_STA_A::DBRM_ANR),
            1 => Some(TWI_STA_A::T_S9SC),
            _ => None,
        }
    }
    #[doc = "bus error"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == TWI_STA_A::BE
    }
    #[doc = "START condition transmitted"]
    #[inline(always)]
    pub fn is_sct(&self) -> bool {
        *self == TWI_STA_A::SCT
    }
    #[doc = "Repeated START condition transmitted"]
    #[inline(always)]
    pub fn is_rsct(&self) -> bool {
        *self == TWI_STA_A::RSCT
    }
    #[doc = "Address + Write bit transmitted, ACK received"]
    #[inline(always)]
    pub fn is_awbt_ar(&self) -> bool {
        *self == TWI_STA_A::AWBT_AR
    }
    #[doc = "Address + Write bit transmitted, ACK not received"]
    #[inline(always)]
    pub fn is_awbt_anr(&self) -> bool {
        *self == TWI_STA_A::AWBT_ANR
    }
    #[doc = "Data byte transmitted in master mode, ACK received"]
    #[inline(always)]
    pub fn is_dbtm_ar(&self) -> bool {
        *self == TWI_STA_A::DBTM_AR
    }
    #[doc = "Data byte transmitted in master mode, ACK not received"]
    #[inline(always)]
    pub fn is_dbtm_anr(&self) -> bool {
        *self == TWI_STA_A::DBTM_ANR
    }
    #[doc = "Arbitration lost in address or data byte"]
    #[inline(always)]
    pub fn is_al_a_db(&self) -> bool {
        *self == TWI_STA_A::AL_A_DB
    }
    #[doc = "Address + Read bit transmitted, ACK received"]
    #[inline(always)]
    pub fn is_arbt_ar(&self) -> bool {
        *self == TWI_STA_A::ARBT_AR
    }
    #[doc = "Address + Read bit transmitted, ACK not received"]
    #[inline(always)]
    pub fn is_arbt_anr(&self) -> bool {
        *self == TWI_STA_A::ARBT_ANR
    }
    #[doc = "Data byte received in master mode, ACK received"]
    #[inline(always)]
    pub fn is_dbrm_ar(&self) -> bool {
        *self == TWI_STA_A::DBRM_AR
    }
    #[doc = "Data byte received in master mode, ACK not received"]
    #[inline(always)]
    pub fn is_dbrm_anr(&self) -> bool {
        *self == TWI_STA_A::DBRM_ANR
    }
    #[doc = "Timeout when sending the 9th SCL clock"]
    #[inline(always)]
    pub fn is_t_s9sc(&self) -> bool {
        *self == TWI_STA_A::T_S9SC
    }
}
#[doc = "Field `tran_result` reader - Transition result"]
pub type TRAN_RESULT_R = crate::FieldReader<TRAN_RESULT_A>;
#[doc = "Transition result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRAN_RESULT_A {
    #[doc = "0: `0`"]
    OK = 0,
    #[doc = "1: `1`"]
    FAIL = 1,
}
impl From<TRAN_RESULT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRAN_RESULT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRAN_RESULT_A {
    type Ux = u8;
}
impl TRAN_RESULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRAN_RESULT_A> {
        match self.bits {
            0 => Some(TRAN_RESULT_A::OK),
            1 => Some(TRAN_RESULT_A::FAIL),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == TRAN_RESULT_A::OK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == TRAN_RESULT_A::FAIL
    }
}
#[doc = "Field `tran_result` writer - Transition result"]
pub type TRAN_RESULT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TRAN_RESULT_A>;
impl<'a, REG> TRAN_RESULT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ok(self) -> &'a mut crate::W<REG> {
        self.variant(TRAN_RESULT_A::OK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fail(self) -> &'a mut crate::W<REG> {
        self.variant(TRAN_RESULT_A::FAIL)
    }
}
#[doc = "Field `read_tran_mode` reader - Read transition mode"]
pub type READ_TRAN_MODE_R = crate::BitReader<READ_TRAN_MODE_A>;
#[doc = "Read transition mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_TRAN_MODE_A {
    #[doc = "0: `0`"]
    SEND = 0,
    #[doc = "1: `1`"]
    NOT_SEND = 1,
}
impl From<READ_TRAN_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_TRAN_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_TRAN_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> READ_TRAN_MODE_A {
        match self.bits {
            false => READ_TRAN_MODE_A::SEND,
            true => READ_TRAN_MODE_A::NOT_SEND,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_send(&self) -> bool {
        *self == READ_TRAN_MODE_A::SEND
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_not_send(&self) -> bool {
        *self == READ_TRAN_MODE_A::NOT_SEND
    }
}
#[doc = "Field `read_tran_mode` writer - Read transition mode"]
pub type READ_TRAN_MODE_W<'a, REG> = crate::BitWriter<'a, REG, READ_TRAN_MODE_A>;
impl<'a, REG> READ_TRAN_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn send(self) -> &'a mut crate::W<REG> {
        self.variant(READ_TRAN_MODE_A::SEND)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn not_send(self) -> &'a mut crate::W<REG> {
        self.variant(READ_TRAN_MODE_A::NOT_SEND)
    }
}
#[doc = "Field `restart_mode` reader - Restart mode"]
pub type RESTART_MODE_R = crate::BitReader<RESTART_MODE_A>;
#[doc = "Restart mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESTART_MODE_A {
    #[doc = "0: `0`"]
    RESTART = 0,
    #[doc = "1: `1`"]
    STOP_RESTART = 1,
}
impl From<RESTART_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RESTART_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESTART_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESTART_MODE_A {
        match self.bits {
            false => RESTART_MODE_A::RESTART,
            true => RESTART_MODE_A::STOP_RESTART,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == RESTART_MODE_A::RESTART
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_stop_restart(&self) -> bool {
        *self == RESTART_MODE_A::STOP_RESTART
    }
}
#[doc = "Field `restart_mode` writer - Restart mode"]
pub type RESTART_MODE_W<'a, REG> = crate::BitWriter<'a, REG, RESTART_MODE_A>;
impl<'a, REG> RESTART_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(RESTART_MODE_A::RESTART)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn stop_restart(self) -> &'a mut crate::W<REG> {
        self.variant(RESTART_MODE_A::STOP_RESTART)
    }
}
#[doc = "Field `start_tran` reader - Start transmission"]
pub type START_TRAN_R = crate::BitReader<START_TRAN_A>;
#[doc = "Start transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_TRAN_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<START_TRAN_A> for bool {
    #[inline(always)]
    fn from(variant: START_TRAN_A) -> Self {
        variant as u8 != 0
    }
}
impl START_TRAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_TRAN_A {
        match self.bits {
            false => START_TRAN_A::IDLE,
            true => START_TRAN_A::START,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == START_TRAN_A::IDLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_TRAN_A::START
    }
}
#[doc = "Field `start_tran` writer - Start transmission"]
pub type START_TRAN_W<'a, REG> = crate::BitWriter<'a, REG, START_TRAN_A>;
impl<'a, REG> START_TRAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(START_TRAN_A::IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START_TRAN_A::START)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn twi_drv_en(&self) -> TWI_DRV_EN_R {
        TWI_DRV_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software reset"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Timeout number"]
    #[inline(always)]
    pub fn timeout_n(&self) -> TIMEOUT_N_R {
        TIMEOUT_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TWI status"]
    #[inline(always)]
    pub fn twi_sta(&self) -> TWI_STA_R {
        TWI_STA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Transition result"]
    #[inline(always)]
    pub fn tran_result(&self) -> TRAN_RESULT_R {
        TRAN_RESULT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Read transition mode"]
    #[inline(always)]
    pub fn read_tran_mode(&self) -> READ_TRAN_MODE_R {
        READ_TRAN_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Restart mode"]
    #[inline(always)]
    pub fn restart_mode(&self) -> RESTART_MODE_R {
        RESTART_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Start transmission"]
    #[inline(always)]
    pub fn start_tran(&self) -> START_TRAN_R {
        START_TRAN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn twi_drv_en(&mut self) -> TWI_DRV_EN_W<TWI_DRV_CTRL_SPEC> {
        TWI_DRV_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<TWI_DRV_CTRL_SPEC> {
        SOFT_RESET_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - Timeout number"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_n(&mut self) -> TIMEOUT_N_W<TWI_DRV_CTRL_SPEC> {
        TIMEOUT_N_W::new(self, 8)
    }
    #[doc = "Bits 24:27 - Transition result"]
    #[inline(always)]
    #[must_use]
    pub fn tran_result(&mut self) -> TRAN_RESULT_W<TWI_DRV_CTRL_SPEC> {
        TRAN_RESULT_W::new(self, 24)
    }
    #[doc = "Bit 28 - Read transition mode"]
    #[inline(always)]
    #[must_use]
    pub fn read_tran_mode(&mut self) -> READ_TRAN_MODE_W<TWI_DRV_CTRL_SPEC> {
        READ_TRAN_MODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Restart mode"]
    #[inline(always)]
    #[must_use]
    pub fn restart_mode(&mut self) -> RESTART_MODE_W<TWI_DRV_CTRL_SPEC> {
        RESTART_MODE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Start transmission"]
    #[inline(always)]
    #[must_use]
    pub fn start_tran(&mut self) -> START_TRAN_W<TWI_DRV_CTRL_SPEC> {
        START_TRAN_W::new(self, 31)
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
#[doc = "TWI_DRV Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_CTRL_SPEC;
impl crate::RegisterSpec for TWI_DRV_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_ctrl::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_ctrl::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_ctrl to value 0"]
impl crate::Resettable for TWI_DRV_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
