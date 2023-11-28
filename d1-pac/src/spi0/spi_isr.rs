#[doc = "Register `spi_isr` reader"]
pub type R = crate::R<SPI_ISR_SPEC>;
#[doc = "Register `spi_isr` writer"]
pub type W = crate::W<SPI_ISR_SPEC>;
#[doc = "Field `rf_rdy` reader - RXFIFO Ready"]
pub type RF_RDY_R = crate::BitReader;
#[doc = "Field `rf_rdy` writer - RXFIFO Ready"]
pub type RF_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_emp` reader - RXFIFO Empty"]
pub type RF_EMP_R = crate::BitReader<RF_EMP_A>;
#[doc = "RXFIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_EMP_A {
    #[doc = "0: `0`"]
    NOT_EMPTY = 0,
    #[doc = "1: `1`"]
    EMPTY = 1,
}
impl From<RF_EMP_A> for bool {
    #[inline(always)]
    fn from(variant: RF_EMP_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_EMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF_EMP_A {
        match self.bits {
            false => RF_EMP_A::NOT_EMPTY,
            true => RF_EMP_A::EMPTY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RF_EMP_A::NOT_EMPTY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RF_EMP_A::EMPTY
    }
}
#[doc = "Field `rf_emp` writer - RXFIFO Empty"]
pub type RF_EMP_W<'a, REG> = crate::BitWriter<'a, REG, RF_EMP_A>;
impl<'a, REG> RF_EMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(RF_EMP_A::NOT_EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(RF_EMP_A::EMPTY)
    }
}
#[doc = "Field `rf_full` reader - RXFIFO Full"]
pub type RF_FULL_R = crate::BitReader<RF_FULL_A>;
#[doc = "RXFIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_FULL_A {
    #[doc = "0: `0`"]
    NOT_FULL = 0,
    #[doc = "1: `1`"]
    FULL = 1,
}
impl From<RF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF_FULL_A {
        match self.bits {
            false => RF_FULL_A::NOT_FULL,
            true => RF_FULL_A::FULL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RF_FULL_A::NOT_FULL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RF_FULL_A::FULL
    }
}
#[doc = "Field `rf_full` writer - RXFIFO Full"]
pub type RF_FULL_W<'a, REG> = crate::BitWriter<'a, REG, RF_FULL_A>;
impl<'a, REG> RF_FULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut crate::W<REG> {
        self.variant(RF_FULL_A::NOT_FULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(RF_FULL_A::FULL)
    }
}
#[doc = "Field `tf_ready` reader - TXFIFO Ready"]
pub type TF_READY_R = crate::BitReader;
#[doc = "Field `tf_ready` writer - TXFIFO Ready"]
pub type TF_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tf_emp` reader - TXFIFO Empty"]
pub type TF_EMP_R = crate::BitReader<TF_EMP_A>;
#[doc = "TXFIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_EMP_A {
    #[doc = "0: `0`"]
    NOT_EMPTY = 0,
    #[doc = "1: `1`"]
    EMPTY = 1,
}
impl From<TF_EMP_A> for bool {
    #[inline(always)]
    fn from(variant: TF_EMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_EMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TF_EMP_A {
        match self.bits {
            false => TF_EMP_A::NOT_EMPTY,
            true => TF_EMP_A::EMPTY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TF_EMP_A::NOT_EMPTY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TF_EMP_A::EMPTY
    }
}
#[doc = "Field `tf_emp` writer - TXFIFO Empty"]
pub type TF_EMP_W<'a, REG> = crate::BitWriter<'a, REG, TF_EMP_A>;
impl<'a, REG> TF_EMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TF_EMP_A::NOT_EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TF_EMP_A::EMPTY)
    }
}
#[doc = "Field `tf_full` reader - TXFIFO Full"]
pub type TF_FULL_R = crate::BitReader<TF_FULL_A>;
#[doc = "TXFIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_FULL_A {
    #[doc = "0: `0`"]
    NOT_FULL = 0,
    #[doc = "1: `1`"]
    FULL = 1,
}
impl From<TF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: TF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TF_FULL_A {
        match self.bits {
            false => TF_FULL_A::NOT_FULL,
            true => TF_FULL_A::FULL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TF_FULL_A::NOT_FULL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TF_FULL_A::FULL
    }
}
#[doc = "Field `tf_full` writer - TXFIFO Full"]
pub type TF_FULL_W<'a, REG> = crate::BitWriter<'a, REG, TF_FULL_A>;
impl<'a, REG> TF_FULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut crate::W<REG> {
        self.variant(TF_FULL_A::NOT_FULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(TF_FULL_A::FULL)
    }
}
#[doc = "Field `rf_ovf` reader - RXFIFO Overflow"]
pub type RF_OVF_R = crate::BitReader<RF_OVF_A>;
#[doc = "RXFIFO Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_OVF_A {
    #[doc = "0: `0`"]
    NOT_OVERFLOW = 0,
    #[doc = "1: `1`"]
    OVERFLOW = 1,
}
impl From<RF_OVF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_OVF_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_OVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF_OVF_A {
        match self.bits {
            false => RF_OVF_A::NOT_OVERFLOW,
            true => RF_OVF_A::OVERFLOW,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        *self == RF_OVF_A::NOT_OVERFLOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == RF_OVF_A::OVERFLOW
    }
}
#[doc = "Field `rf_ovf` writer - RXFIFO Overflow"]
pub type RF_OVF_W<'a, REG> = crate::BitWriter<'a, REG, RF_OVF_A>;
impl<'a, REG> RF_OVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(RF_OVF_A::NOT_OVERFLOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(RF_OVF_A::OVERFLOW)
    }
}
#[doc = "Field `rf_udr` reader - RXFIFO Underrun"]
pub type RF_UDR_R = crate::BitReader<RF_UDR_A>;
#[doc = "RXFIFO Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_UDR_A {
    #[doc = "0: `0`"]
    NOT_UNDERRUN = 0,
    #[doc = "1: `1`"]
    UNDERRUN = 1,
}
impl From<RF_UDR_A> for bool {
    #[inline(always)]
    fn from(variant: RF_UDR_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_UDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF_UDR_A {
        match self.bits {
            false => RF_UDR_A::NOT_UNDERRUN,
            true => RF_UDR_A::UNDERRUN,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_underrun(&self) -> bool {
        *self == RF_UDR_A::NOT_UNDERRUN
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == RF_UDR_A::UNDERRUN
    }
}
#[doc = "Field `rf_udr` writer - RXFIFO Underrun"]
pub type RF_UDR_W<'a, REG> = crate::BitWriter<'a, REG, RF_UDR_A>;
impl<'a, REG> RF_UDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(RF_UDR_A::NOT_UNDERRUN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(RF_UDR_A::UNDERRUN)
    }
}
#[doc = "Field `tf_ovf` reader - TXFIFO Overflow"]
pub type TF_OVF_R = crate::BitReader<TF_OVF_A>;
#[doc = "TXFIFO Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_OVF_A {
    #[doc = "0: `0`"]
    NOT_OVERFLOW = 0,
    #[doc = "1: `1`"]
    OVERFLOW = 1,
}
impl From<TF_OVF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_OVF_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_OVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TF_OVF_A {
        match self.bits {
            false => TF_OVF_A::NOT_OVERFLOW,
            true => TF_OVF_A::OVERFLOW,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        *self == TF_OVF_A::NOT_OVERFLOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TF_OVF_A::OVERFLOW
    }
}
#[doc = "Field `tf_ovf` writer - TXFIFO Overflow"]
pub type TF_OVF_W<'a, REG> = crate::BitWriter<'a, REG, TF_OVF_A>;
impl<'a, REG> TF_OVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(TF_OVF_A::NOT_OVERFLOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(TF_OVF_A::OVERFLOW)
    }
}
#[doc = "Field `tf_udr` reader - TXFIFO Underrun"]
pub type TF_UDR_R = crate::BitReader<TF_UDR_A>;
#[doc = "TXFIFO Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_UDR_A {
    #[doc = "0: `0`"]
    NOT_UNDERRUN = 0,
    #[doc = "1: `1`"]
    UNDERRUN = 1,
}
impl From<TF_UDR_A> for bool {
    #[inline(always)]
    fn from(variant: TF_UDR_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_UDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TF_UDR_A {
        match self.bits {
            false => TF_UDR_A::NOT_UNDERRUN,
            true => TF_UDR_A::UNDERRUN,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_underrun(&self) -> bool {
        *self == TF_UDR_A::NOT_UNDERRUN
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TF_UDR_A::UNDERRUN
    }
}
#[doc = "Field `tf_udr` writer - TXFIFO Underrun"]
pub type TF_UDR_W<'a, REG> = crate::BitWriter<'a, REG, TF_UDR_A>;
impl<'a, REG> TF_UDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(TF_UDR_A::NOT_UNDERRUN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(TF_UDR_A::UNDERRUN)
    }
}
#[doc = "Field `tc` reader - Transfer Completed"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transfer Completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: `0`"]
    BUSY = 0,
    #[doc = "1: `1`"]
    TRANSFER_COMPLETED = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::BUSY,
            true => TC_A::TRANSFER_COMPLETED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TC_A::BUSY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        *self == TC_A::TRANSFER_COMPLETED
    }
}
#[doc = "Field `tc` writer - Transfer Completed"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG, TC_A>;
impl<'a, REG> TC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(TC_A::BUSY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn transfer_completed(self) -> &'a mut crate::W<REG> {
        self.variant(TC_A::TRANSFER_COMPLETED)
    }
}
#[doc = "Field `ssi` reader - SS Invalid Enable"]
pub type SSI_R = crate::BitReader;
#[doc = "Field `ssi` writer - SS Invalid Enable"]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXFIFO Ready"]
    #[inline(always)]
    pub fn rf_rdy(&self) -> RF_RDY_R {
        RF_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXFIFO Empty"]
    #[inline(always)]
    pub fn rf_emp(&self) -> RF_EMP_R {
        RF_EMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXFIFO Full"]
    #[inline(always)]
    pub fn rf_full(&self) -> RF_FULL_R {
        RF_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFIFO Ready"]
    #[inline(always)]
    pub fn tf_ready(&self) -> TF_READY_R {
        TF_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFIFO Empty"]
    #[inline(always)]
    pub fn tf_emp(&self) -> TF_EMP_R {
        TF_EMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXFIFO Full"]
    #[inline(always)]
    pub fn tf_full(&self) -> TF_FULL_R {
        TF_FULL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXFIFO Overflow"]
    #[inline(always)]
    pub fn rf_ovf(&self) -> RF_OVF_R {
        RF_OVF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFIFO Underrun"]
    #[inline(always)]
    pub fn rf_udr(&self) -> RF_UDR_R {
        RF_UDR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXFIFO Overflow"]
    #[inline(always)]
    pub fn tf_ovf(&self) -> TF_OVF_R {
        TF_OVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXFIFO Underrun"]
    #[inline(always)]
    pub fn tf_udr(&self) -> TF_UDR_R {
        TF_UDR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transfer Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SS Invalid Enable"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO Ready"]
    #[inline(always)]
    #[must_use]
    pub fn rf_rdy(&mut self) -> RF_RDY_W<SPI_ISR_SPEC> {
        RF_RDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - RXFIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn rf_emp(&mut self) -> RF_EMP_W<SPI_ISR_SPEC> {
        RF_EMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - RXFIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf_full(&mut self) -> RF_FULL_W<SPI_ISR_SPEC> {
        RF_FULL_W::new(self, 2)
    }
    #[doc = "Bit 4 - TXFIFO Ready"]
    #[inline(always)]
    #[must_use]
    pub fn tf_ready(&mut self) -> TF_READY_W<SPI_ISR_SPEC> {
        TF_READY_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tf_emp(&mut self) -> TF_EMP_W<SPI_ISR_SPEC> {
        TF_EMP_W::new(self, 5)
    }
    #[doc = "Bit 6 - TXFIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn tf_full(&mut self) -> TF_FULL_W<SPI_ISR_SPEC> {
        TF_FULL_W::new(self, 6)
    }
    #[doc = "Bit 8 - RXFIFO Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ovf(&mut self) -> RF_OVF_W<SPI_ISR_SPEC> {
        RF_OVF_W::new(self, 8)
    }
    #[doc = "Bit 9 - RXFIFO Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn rf_udr(&mut self) -> RF_UDR_W<SPI_ISR_SPEC> {
        RF_UDR_W::new(self, 9)
    }
    #[doc = "Bit 10 - TXFIFO Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn tf_ovf(&mut self) -> TF_OVF_W<SPI_ISR_SPEC> {
        TF_OVF_W::new(self, 10)
    }
    #[doc = "Bit 11 - TXFIFO Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tf_udr(&mut self) -> TF_UDR_W<SPI_ISR_SPEC> {
        TF_UDR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Transfer Completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<SPI_ISR_SPEC> {
        TC_W::new(self, 12)
    }
    #[doc = "Bit 13 - SS Invalid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<SPI_ISR_SPEC> {
        SSI_W::new(self, 13)
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
#[doc = "SPI Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_ISR_SPEC;
impl crate::RegisterSpec for SPI_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_isr::R`](R) reader structure"]
impl crate::Readable for SPI_ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_isr::W`](W) writer structure"]
impl crate::Writable for SPI_ISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_isr to value 0"]
impl crate::Resettable for SPI_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
