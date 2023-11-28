#[doc = "Register `lcr` reader"]
pub type R = crate::R<LCR_SPEC>;
#[doc = "Register `lcr` writer"]
pub type W = crate::W<LCR_SPEC>;
#[doc = "Field `dls` reader - Data Length Select"]
pub type DLS_R = crate::FieldReader<DLS_A>;
#[doc = "Data Length Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLS_A {
    #[doc = "0: 5 bits"]
    FIVE = 0,
    #[doc = "1: 6 bits"]
    SIX = 1,
    #[doc = "2: 7 bits"]
    SEVEN = 2,
    #[doc = "3: 8 bits"]
    EIGHT = 3,
}
impl From<DLS_A> for u8 {
    #[inline(always)]
    fn from(variant: DLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLS_A {
    type Ux = u8;
}
impl DLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLS_A {
        match self.bits {
            0 => DLS_A::FIVE,
            1 => DLS_A::SIX,
            2 => DLS_A::SEVEN,
            3 => DLS_A::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == DLS_A::FIVE
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == DLS_A::SIX
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DLS_A::SEVEN
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DLS_A::EIGHT
    }
}
#[doc = "Field `dls` writer - Data Length Select"]
pub type DLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DLS_A>;
impl<'a, REG> DLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(DLS_A::FIVE)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(DLS_A::SIX)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(DLS_A::SEVEN)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(DLS_A::EIGHT)
    }
}
#[doc = "Field `stop` reader - Number of stop bits"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Number of stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    ONE = 0,
    #[doc = "1: 1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    TWO = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::ONE,
            true => STOP_A::TWO,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOP_A::ONE
    }
    #[doc = "1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOP_A::TWO
    }
}
#[doc = "Field `stop` writer - Number of stop bits"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP_A>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::ONE)
    }
    #[doc = "1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::TWO)
    }
}
#[doc = "Field `pen` reader - Parity Enable"]
pub type PEN_R = crate::BitReader<PEN_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEN_A {
        match self.bits {
            false => PEN_A::DISABLED,
            true => PEN_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEN_A::ENABLED
    }
}
#[doc = "Field `pen` writer - Parity Enable"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG, PEN_A>;
impl<'a, REG> PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEN_A::ENABLED)
    }
}
#[doc = "Field `eps` reader - Even Parity Select"]
pub type EPS_R = crate::FieldReader<EPS_A>;
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPS_A {
    #[doc = "0: `0`"]
    ODD = 0,
    #[doc = "1: `1`"]
    EVEN = 1,
    #[doc = "2: `10`"]
    RS485_DATA = 2,
    #[doc = "3: `11`"]
    RS485_ADDR = 3,
}
impl From<EPS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPS_A {
    type Ux = u8;
}
impl EPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPS_A {
        match self.bits {
            0 => EPS_A::ODD,
            1 => EPS_A::EVEN,
            2 => EPS_A::RS485_DATA,
            3 => EPS_A::RS485_ADDR,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == EPS_A::ODD
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == EPS_A::EVEN
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rs485_data(&self) -> bool {
        *self == EPS_A::RS485_DATA
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_rs485_addr(&self) -> bool {
        *self == EPS_A::RS485_ADDR
    }
}
#[doc = "Field `eps` writer - Even Parity Select"]
pub type EPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EPS_A>;
impl<'a, REG> EPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(EPS_A::ODD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(EPS_A::EVEN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rs485_data(self) -> &'a mut crate::W<REG> {
        self.variant(EPS_A::RS485_DATA)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rs485_addr(self) -> &'a mut crate::W<REG> {
        self.variant(EPS_A::RS485_ADDR)
    }
}
#[doc = "Field `bc` reader - Break Control Bit"]
pub type BC_R = crate::BitReader;
#[doc = "Field `bc` writer - Break Control Bit"]
pub type BC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dlab` reader - Divisor Latch Access Bit"]
pub type DLAB_R = crate::BitReader<DLAB_A>;
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLAB_A {
    #[doc = "0: `0`"]
    RX_BUFFER = 0,
    #[doc = "1: `1`"]
    DIVISOR_LATCH = 1,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        variant as u8 != 0
    }
}
impl DLAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::RX_BUFFER,
            true => DLAB_A::DIVISOR_LATCH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_rx_buffer(&self) -> bool {
        *self == DLAB_A::RX_BUFFER
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_divisor_latch(&self) -> bool {
        *self == DLAB_A::DIVISOR_LATCH
    }
}
#[doc = "Field `dlab` writer - Divisor Latch Access Bit"]
pub type DLAB_W<'a, REG> = crate::BitWriter<'a, REG, DLAB_A>;
impl<'a, REG> DLAB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(DLAB_A::RX_BUFFER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn divisor_latch(self) -> &'a mut crate::W<REG> {
        self.variant(DLAB_A::DIVISOR_LATCH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&self) -> DLS_R {
        DLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    #[must_use]
    pub fn dls(&mut self) -> DLS_W<LCR_SPEC> {
        DLS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<LCR_SPEC> {
        STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<LCR_SPEC> {
        PEN_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EPS_W<LCR_SPEC> {
        EPS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<LCR_SPEC> {
        BC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DLAB_W<LCR_SPEC> {
        DLAB_W::new(self, 7)
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
#[doc = "UART Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcr to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
