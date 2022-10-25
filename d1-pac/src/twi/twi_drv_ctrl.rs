#[doc = "Register `twi_drv_ctrl` reader"]
pub struct R(crate::R<TWI_DRV_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_drv_ctrl` writer"]
pub struct W(crate::W<TWI_DRV_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_CTRL_SPEC>;
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
impl From<crate::W<TWI_DRV_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> TWI_DRV_EN_A {
        match self.bits {
            false => TWI_DRV_EN_A::DISABLE,
            true => TWI_DRV_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TWI_DRV_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TWI_DRV_EN_A::ENABLE
    }
}
#[doc = "Field `twi_drv_en` writer - "]
pub type TWI_DRV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_CTRL_SPEC, TWI_DRV_EN_A, O>;
impl<'a, const O: u8> TWI_DRV_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TWI_DRV_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn variant(&self) -> SOFT_RESET_A {
        match self.bits {
            false => SOFT_RESET_A::NORMAL,
            true => SOFT_RESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SOFT_RESET_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RESET_A::RESET
    }
}
#[doc = "Field `soft_reset` writer - Software reset"]
pub type SOFT_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_CTRL_SPEC, SOFT_RESET_A, O>;
impl<'a, const O: u8> SOFT_RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SOFT_RESET_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOFT_RESET_A::RESET)
    }
}
#[doc = "Field `timeout_n` reader - Timeout number"]
pub type TIMEOUT_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timeout_n` writer - Timeout number"]
pub type TIMEOUT_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `twi_sta` reader - TWI status"]
pub type TWI_STA_R = crate::FieldReader<u8, TWI_STA_A>;
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
impl TWI_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TWI_STA_A> {
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
    #[doc = "Checks if the value of the field is `BE`"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == TWI_STA_A::BE
    }
    #[doc = "Checks if the value of the field is `SCT`"]
    #[inline(always)]
    pub fn is_sct(&self) -> bool {
        *self == TWI_STA_A::SCT
    }
    #[doc = "Checks if the value of the field is `RSCT`"]
    #[inline(always)]
    pub fn is_rsct(&self) -> bool {
        *self == TWI_STA_A::RSCT
    }
    #[doc = "Checks if the value of the field is `AWBT_AR`"]
    #[inline(always)]
    pub fn is_awbt_ar(&self) -> bool {
        *self == TWI_STA_A::AWBT_AR
    }
    #[doc = "Checks if the value of the field is `AWBT_ANR`"]
    #[inline(always)]
    pub fn is_awbt_anr(&self) -> bool {
        *self == TWI_STA_A::AWBT_ANR
    }
    #[doc = "Checks if the value of the field is `DBTM_AR`"]
    #[inline(always)]
    pub fn is_dbtm_ar(&self) -> bool {
        *self == TWI_STA_A::DBTM_AR
    }
    #[doc = "Checks if the value of the field is `DBTM_ANR`"]
    #[inline(always)]
    pub fn is_dbtm_anr(&self) -> bool {
        *self == TWI_STA_A::DBTM_ANR
    }
    #[doc = "Checks if the value of the field is `AL_A_DB`"]
    #[inline(always)]
    pub fn is_al_a_db(&self) -> bool {
        *self == TWI_STA_A::AL_A_DB
    }
    #[doc = "Checks if the value of the field is `ARBT_AR`"]
    #[inline(always)]
    pub fn is_arbt_ar(&self) -> bool {
        *self == TWI_STA_A::ARBT_AR
    }
    #[doc = "Checks if the value of the field is `ARBT_ANR`"]
    #[inline(always)]
    pub fn is_arbt_anr(&self) -> bool {
        *self == TWI_STA_A::ARBT_ANR
    }
    #[doc = "Checks if the value of the field is `DBRM_AR`"]
    #[inline(always)]
    pub fn is_dbrm_ar(&self) -> bool {
        *self == TWI_STA_A::DBRM_AR
    }
    #[doc = "Checks if the value of the field is `DBRM_ANR`"]
    #[inline(always)]
    pub fn is_dbrm_anr(&self) -> bool {
        *self == TWI_STA_A::DBRM_ANR
    }
    #[doc = "Checks if the value of the field is `T_S9SC`"]
    #[inline(always)]
    pub fn is_t_s9sc(&self) -> bool {
        *self == TWI_STA_A::T_S9SC
    }
}
#[doc = "Field `tran_result` reader - Transition result"]
pub type TRAN_RESULT_R = crate::FieldReader<u8, TRAN_RESULT_A>;
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
impl TRAN_RESULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRAN_RESULT_A> {
        match self.bits {
            0 => Some(TRAN_RESULT_A::OK),
            1 => Some(TRAN_RESULT_A::FAIL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == TRAN_RESULT_A::OK
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == TRAN_RESULT_A::FAIL
    }
}
#[doc = "Field `tran_result` writer - Transition result"]
pub type TRAN_RESULT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_CTRL_SPEC, u8, TRAN_RESULT_A, 4, O>;
impl<'a, const O: u8> TRAN_RESULT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ok(self) -> &'a mut W {
        self.variant(TRAN_RESULT_A::OK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fail(self) -> &'a mut W {
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
    pub fn variant(&self) -> READ_TRAN_MODE_A {
        match self.bits {
            false => READ_TRAN_MODE_A::SEND,
            true => READ_TRAN_MODE_A::NOT_SEND,
        }
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline(always)]
    pub fn is_send(&self) -> bool {
        *self == READ_TRAN_MODE_A::SEND
    }
    #[doc = "Checks if the value of the field is `NOT_SEND`"]
    #[inline(always)]
    pub fn is_not_send(&self) -> bool {
        *self == READ_TRAN_MODE_A::NOT_SEND
    }
}
#[doc = "Field `read_tran_mode` writer - Read transition mode"]
pub type READ_TRAN_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_CTRL_SPEC, READ_TRAN_MODE_A, O>;
impl<'a, const O: u8> READ_TRAN_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn send(self) -> &'a mut W {
        self.variant(READ_TRAN_MODE_A::SEND)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn not_send(self) -> &'a mut W {
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
    pub fn variant(&self) -> RESTART_MODE_A {
        match self.bits {
            false => RESTART_MODE_A::RESTART,
            true => RESTART_MODE_A::STOP_RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == RESTART_MODE_A::RESTART
    }
    #[doc = "Checks if the value of the field is `STOP_RESTART`"]
    #[inline(always)]
    pub fn is_stop_restart(&self) -> bool {
        *self == RESTART_MODE_A::STOP_RESTART
    }
}
#[doc = "Field `restart_mode` writer - Restart mode"]
pub type RESTART_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_CTRL_SPEC, RESTART_MODE_A, O>;
impl<'a, const O: u8> RESTART_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(RESTART_MODE_A::RESTART)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn stop_restart(self) -> &'a mut W {
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
    pub fn variant(&self) -> START_TRAN_A {
        match self.bits {
            false => START_TRAN_A::IDLE,
            true => START_TRAN_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == START_TRAN_A::IDLE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_TRAN_A::START
    }
}
#[doc = "Field `start_tran` writer - Start transmission"]
pub type START_TRAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_CTRL_SPEC, START_TRAN_A, O>;
impl<'a, const O: u8> START_TRAN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(START_TRAN_A::IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
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
    pub fn twi_drv_en(&mut self) -> TWI_DRV_EN_W<0> {
        TWI_DRV_EN_W::new(self)
    }
    #[doc = "Bit 1 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<1> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Bits 8:15 - Timeout number"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_n(&mut self) -> TIMEOUT_N_W<8> {
        TIMEOUT_N_W::new(self)
    }
    #[doc = "Bits 24:27 - Transition result"]
    #[inline(always)]
    #[must_use]
    pub fn tran_result(&mut self) -> TRAN_RESULT_W<24> {
        TRAN_RESULT_W::new(self)
    }
    #[doc = "Bit 28 - Read transition mode"]
    #[inline(always)]
    #[must_use]
    pub fn read_tran_mode(&mut self) -> READ_TRAN_MODE_W<28> {
        READ_TRAN_MODE_W::new(self)
    }
    #[doc = "Bit 29 - Restart mode"]
    #[inline(always)]
    #[must_use]
    pub fn restart_mode(&mut self) -> RESTART_MODE_W<29> {
        RESTART_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Start transmission"]
    #[inline(always)]
    #[must_use]
    pub fn start_tran(&mut self) -> START_TRAN_W<31> {
        START_TRAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_ctrl](index.html) module"]
pub struct TWI_DRV_CTRL_SPEC;
impl crate::RegisterSpec for TWI_DRV_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_ctrl::R](R) reader structure"]
impl crate::Readable for TWI_DRV_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_ctrl::W](W) writer structure"]
impl crate::Writable for TWI_DRV_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_ctrl to value 0"]
impl crate::Resettable for TWI_DRV_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
