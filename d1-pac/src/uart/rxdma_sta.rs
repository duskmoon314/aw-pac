#[doc = "Register `rxdma_sta` reader"]
pub type R = crate::R<RXDMA_STA_SPEC>;
#[doc = "Register `rxdma_sta` writer"]
pub type W = crate::W<RXDMA_STA_SPEC>;
#[doc = "Field `busy` reader - "]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `busy` writer - "]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG, BUSY_A>;
impl<'a, REG> BUSY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(BUSY_A::IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(BUSY_A::BUSY)
    }
}
#[doc = "Field `buffer_read_address_updating` reader - "]
pub type BUFFER_READ_ADDRESS_UPDATING_R = crate::BitReader<BUFFER_READ_ADDRESS_UPDATING_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFFER_READ_ADDRESS_UPDATING_A {
    #[doc = "0: `0`"]
    READY = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BUFFER_READ_ADDRESS_UPDATING_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_READ_ADDRESS_UPDATING_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFFER_READ_ADDRESS_UPDATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUFFER_READ_ADDRESS_UPDATING_A {
        match self.bits {
            false => BUFFER_READ_ADDRESS_UPDATING_A::READY,
            true => BUFFER_READ_ADDRESS_UPDATING_A::BUSY,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BUFFER_READ_ADDRESS_UPDATING_A::READY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUFFER_READ_ADDRESS_UPDATING_A::BUSY
    }
}
#[doc = "Field `buffer_read_address_updating` writer - "]
pub type BUFFER_READ_ADDRESS_UPDATING_W<'a, REG> =
    crate::BitWriter<'a, REG, BUFFER_READ_ADDRESS_UPDATING_A>;
impl<'a, REG> BUFFER_READ_ADDRESS_UPDATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(BUFFER_READ_ADDRESS_UPDATING_A::READY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(BUFFER_READ_ADDRESS_UPDATING_A::BUSY)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn buffer_read_address_updating(&self) -> BUFFER_READ_ADDRESS_UPDATING_R {
        BUFFER_READ_ADDRESS_UPDATING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<RXDMA_STA_SPEC> {
        BUSY_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_read_address_updating(
        &mut self,
    ) -> BUFFER_READ_ADDRESS_UPDATING_W<RXDMA_STA_SPEC> {
        BUFFER_READ_ADDRESS_UPDATING_W::new(self, 1)
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
#[doc = "UART RXDMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_STA_SPEC;
impl crate::RegisterSpec for RXDMA_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_sta::R`](R) reader structure"]
impl crate::Readable for RXDMA_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_sta::W`](W) writer structure"]
impl crate::Writable for RXDMA_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_sta to value 0"]
impl crate::Resettable for RXDMA_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
