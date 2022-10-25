#[doc = "Register `tp_int_fifo_stat` reader"]
pub struct R(crate::R<TP_INT_FIFO_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_INT_FIFO_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_INT_FIFO_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_INT_FIFO_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tp_int_fifo_stat` writer"]
pub struct W(crate::W<TP_INT_FIFO_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_INT_FIFO_STAT_SPEC>;
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
impl From<crate::W<TP_INT_FIFO_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_INT_FIFO_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tp_down_pending` reader - TP First Touch (Stylus DOWN) Pending"]
pub type TP_DOWN_PENDING_R = crate::BitReader<TP_DOWN_PENDING_A>;
#[doc = "TP First Touch (Stylus DOWN) Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_DOWN_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TP_DOWN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DOWN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_DOWN_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DOWN_PENDING_A {
        match self.bits {
            false => TP_DOWN_PENDING_A::NO_PENDING,
            true => TP_DOWN_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TP_DOWN_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TP_DOWN_PENDING_A::PENDING
    }
}
#[doc = "Field `tp_down_pending` writer - TP First Touch (Stylus DOWN) Pending"]
pub type TP_DOWN_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, TP_INT_FIFO_STAT_SPEC, TP_DOWN_PENDING_A, O>;
impl<'a, const O: u8> TP_DOWN_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TP_DOWN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TP_DOWN_PENDING_A::PENDING)
    }
}
#[doc = "Field `tp_up_pending` reader - TP Last Touch (Stylus UP) Pending"]
pub type TP_UP_PENDING_R = crate::BitReader<TP_UP_PENDING_A>;
#[doc = "TP Last Touch (Stylus UP) Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_UP_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TP_UP_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: TP_UP_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_UP_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_UP_PENDING_A {
        match self.bits {
            false => TP_UP_PENDING_A::NO_PENDING,
            true => TP_UP_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TP_UP_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TP_UP_PENDING_A::PENDING
    }
}
#[doc = "Field `tp_up_pending` writer - TP Last Touch (Stylus UP) Pending"]
pub type TP_UP_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, TP_INT_FIFO_STAT_SPEC, TP_UP_PENDING_A, O>;
impl<'a, const O: u8> TP_UP_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TP_UP_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TP_UP_PENDING_A::PENDING)
    }
}
#[doc = "Field `tp_idle_flg` reader - TP Idle Flag"]
pub type TP_IDLE_FLG_R = crate::BitReader<TP_IDLE_FLG_A>;
#[doc = "TP Idle Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_IDLE_FLG_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    NOT_IDLE = 1,
}
impl From<TP_IDLE_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: TP_IDLE_FLG_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_IDLE_FLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_IDLE_FLG_A {
        match self.bits {
            false => TP_IDLE_FLG_A::IDLE,
            true => TP_IDLE_FLG_A::NOT_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TP_IDLE_FLG_A::IDLE
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == TP_IDLE_FLG_A::NOT_IDLE
    }
}
#[doc = "Field `rxa_cnt` reader - TP FIFO Available Sample Word Count"]
pub type RXA_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fifo_data_pending` reader - TP FIFO Data Available Pending"]
pub type FIFO_DATA_PENDING_R = crate::BitReader<FIFO_DATA_PENDING_A>;
#[doc = "TP FIFO Data Available Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_DATA_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<FIFO_DATA_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_DATA_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_DATA_PENDING_A {
        match self.bits {
            false => FIFO_DATA_PENDING_A::NO_PENDING,
            true => FIFO_DATA_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == FIFO_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FIFO_DATA_PENDING_A::PENDING
    }
}
#[doc = "Field `fifo_data_pending` writer - TP FIFO Data Available Pending"]
pub type FIFO_DATA_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, TP_INT_FIFO_STAT_SPEC, FIFO_DATA_PENDING_A, O>;
impl<'a, const O: u8> FIFO_DATA_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(FIFO_DATA_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(FIFO_DATA_PENDING_A::PENDING)
    }
}
#[doc = "Field `fifo_overrun_pending` reader - TP FIFO Overrun Pending"]
pub type FIFO_OVERRUN_PENDING_R = crate::BitReader<FIFO_OVERRUN_PENDING_A>;
#[doc = "TP FIFO Overrun Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_OVERRUN_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<FIFO_OVERRUN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERRUN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_OVERRUN_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_OVERRUN_PENDING_A {
        match self.bits {
            false => FIFO_OVERRUN_PENDING_A::NO_PENDING,
            true => FIFO_OVERRUN_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == FIFO_OVERRUN_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FIFO_OVERRUN_PENDING_A::PENDING
    }
}
#[doc = "Field `fifo_overrun_pending` writer - TP FIFO Overrun Pending"]
pub type FIFO_OVERRUN_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, TP_INT_FIFO_STAT_SPEC, FIFO_OVERRUN_PENDING_A, O>;
impl<'a, const O: u8> FIFO_OVERRUN_PENDING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_PENDING_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) Pending"]
    #[inline(always)]
    pub fn tp_down_pending(&self) -> TP_DOWN_PENDING_R {
        TP_DOWN_PENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) Pending"]
    #[inline(always)]
    pub fn tp_up_pending(&self) -> TP_UP_PENDING_R {
        TP_UP_PENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TP Idle Flag"]
    #[inline(always)]
    pub fn tp_idle_flg(&self) -> TP_IDLE_FLG_R {
        TP_IDLE_FLG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:13 - TP FIFO Available Sample Word Count"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RXA_CNT_R {
        RXA_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - TP FIFO Data Available Pending"]
    #[inline(always)]
    pub fn fifo_data_pending(&self) -> FIFO_DATA_PENDING_R {
        FIFO_DATA_PENDING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TP FIFO Overrun Pending"]
    #[inline(always)]
    pub fn fifo_overrun_pending(&self) -> FIFO_OVERRUN_PENDING_R {
        FIFO_OVERRUN_PENDING_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tp_down_pending(&mut self) -> TP_DOWN_PENDING_W<0> {
        TP_DOWN_PENDING_W::new(self)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tp_up_pending(&mut self) -> TP_UP_PENDING_W<1> {
        TP_UP_PENDING_W::new(self)
    }
    #[doc = "Bit 16 - TP FIFO Data Available Pending"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data_pending(&mut self) -> FIFO_DATA_PENDING_W<16> {
        FIFO_DATA_PENDING_W::new(self)
    }
    #[doc = "Bit 17 - TP FIFO Overrun Pending"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_pending(&mut self) -> FIFO_OVERRUN_PENDING_W<17> {
        FIFO_OVERRUN_PENDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Interrupt FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_int_fifo_stat](index.html) module"]
pub struct TP_INT_FIFO_STAT_SPEC;
impl crate::RegisterSpec for TP_INT_FIFO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_int_fifo_stat::R](R) reader structure"]
impl crate::Readable for TP_INT_FIFO_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_int_fifo_stat::W](W) writer structure"]
impl crate::Writable for TP_INT_FIFO_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0003;
}
#[doc = "`reset()` method sets tp_int_fifo_stat to value 0"]
impl crate::Resettable for TP_INT_FIFO_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
