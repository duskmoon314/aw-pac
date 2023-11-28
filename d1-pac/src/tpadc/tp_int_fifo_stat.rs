#[doc = "Register `tp_int_fifo_stat` reader"]
pub type R = crate::R<TP_INT_FIFO_STAT_SPEC>;
#[doc = "Register `tp_int_fifo_stat` writer"]
pub type W = crate::W<TP_INT_FIFO_STAT_SPEC>;
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
    pub const fn variant(&self) -> TP_DOWN_PENDING_A {
        match self.bits {
            false => TP_DOWN_PENDING_A::NO_PENDING,
            true => TP_DOWN_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TP_DOWN_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TP_DOWN_PENDING_A::PENDING
    }
}
#[doc = "Field `tp_down_pending` writer - TP First Touch (Stylus DOWN) Pending"]
pub type TP_DOWN_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, TP_DOWN_PENDING_A>;
impl<'a, REG> TP_DOWN_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(TP_DOWN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TP_UP_PENDING_A {
        match self.bits {
            false => TP_UP_PENDING_A::NO_PENDING,
            true => TP_UP_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TP_UP_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TP_UP_PENDING_A::PENDING
    }
}
#[doc = "Field `tp_up_pending` writer - TP Last Touch (Stylus UP) Pending"]
pub type TP_UP_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, TP_UP_PENDING_A>;
impl<'a, REG> TP_UP_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(TP_UP_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TP_IDLE_FLG_A {
        match self.bits {
            false => TP_IDLE_FLG_A::IDLE,
            true => TP_IDLE_FLG_A::NOT_IDLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TP_IDLE_FLG_A::IDLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == TP_IDLE_FLG_A::NOT_IDLE
    }
}
#[doc = "Field `rxa_cnt` reader - TP FIFO Available Sample Word Count"]
pub type RXA_CNT_R = crate::FieldReader;
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
    pub const fn variant(&self) -> FIFO_DATA_PENDING_A {
        match self.bits {
            false => FIFO_DATA_PENDING_A::NO_PENDING,
            true => FIFO_DATA_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == FIFO_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FIFO_DATA_PENDING_A::PENDING
    }
}
#[doc = "Field `fifo_data_pending` writer - TP FIFO Data Available Pending"]
pub type FIFO_DATA_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, FIFO_DATA_PENDING_A>;
impl<'a, REG> FIFO_DATA_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_DATA_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> FIFO_OVERRUN_PENDING_A {
        match self.bits {
            false => FIFO_OVERRUN_PENDING_A::NO_PENDING,
            true => FIFO_OVERRUN_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == FIFO_OVERRUN_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FIFO_OVERRUN_PENDING_A::PENDING
    }
}
#[doc = "Field `fifo_overrun_pending` writer - TP FIFO Overrun Pending"]
pub type FIFO_OVERRUN_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, FIFO_OVERRUN_PENDING_A>;
impl<'a, REG> FIFO_OVERRUN_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERRUN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
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
    pub fn tp_down_pending(&mut self) -> TP_DOWN_PENDING_W<TP_INT_FIFO_STAT_SPEC> {
        TP_DOWN_PENDING_W::new(self, 0)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) Pending"]
    #[inline(always)]
    #[must_use]
    pub fn tp_up_pending(&mut self) -> TP_UP_PENDING_W<TP_INT_FIFO_STAT_SPEC> {
        TP_UP_PENDING_W::new(self, 1)
    }
    #[doc = "Bit 16 - TP FIFO Data Available Pending"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data_pending(&mut self) -> FIFO_DATA_PENDING_W<TP_INT_FIFO_STAT_SPEC> {
        FIFO_DATA_PENDING_W::new(self, 16)
    }
    #[doc = "Bit 17 - TP FIFO Overrun Pending"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_pending(&mut self) -> FIFO_OVERRUN_PENDING_W<TP_INT_FIFO_STAT_SPEC> {
        FIFO_OVERRUN_PENDING_W::new(self, 17)
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
#[doc = "TP Interrupt FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_int_fifo_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_int_fifo_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TP_INT_FIFO_STAT_SPEC;
impl crate::RegisterSpec for TP_INT_FIFO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tp_int_fifo_stat::R`](R) reader structure"]
impl crate::Readable for TP_INT_FIFO_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tp_int_fifo_stat::W`](W) writer structure"]
impl crate::Writable for TP_INT_FIFO_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0003;
}
#[doc = "`reset()` method sets tp_int_fifo_stat to value 0"]
impl crate::Resettable for TP_INT_FIFO_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
