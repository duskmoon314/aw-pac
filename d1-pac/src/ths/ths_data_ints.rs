#[doc = "Register `ths_data_ints` reader"]
pub type R = crate::R<THS_DATA_INTS_SPEC>;
#[doc = "Register `ths_data_ints` writer"]
pub type W = crate::W<THS_DATA_INTS_SPEC>;
#[doc = "Field `ths_data_irq_sts` reader - Indicates the pending status of the sensor's data interrupt.\n\nWrite 1 to clear the pending status."]
pub type THS_DATA_IRQ_STS_R = crate::BitReader<THS_DATA_IRQ_STS_A>;
#[doc = "Indicates the pending status of the sensor's data interrupt.\n\nWrite 1 to clear the pending status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THS_DATA_IRQ_STS_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Pending"]
    PENDING = 1,
}
impl From<THS_DATA_IRQ_STS_A> for bool {
    #[inline(always)]
    fn from(variant: THS_DATA_IRQ_STS_A) -> Self {
        variant as u8 != 0
    }
}
impl THS_DATA_IRQ_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THS_DATA_IRQ_STS_A {
        match self.bits {
            false => THS_DATA_IRQ_STS_A::NO_EFFECT,
            true => THS_DATA_IRQ_STS_A::PENDING,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == THS_DATA_IRQ_STS_A::NO_EFFECT
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == THS_DATA_IRQ_STS_A::PENDING
    }
}
#[doc = "Field `ths_data_irq_sts` writer - Indicates the pending status of the sensor's data interrupt.\n\nWrite 1 to clear the pending status."]
pub type THS_DATA_IRQ_STS_W<'a, REG> = crate::BitWriter1C<'a, REG, THS_DATA_IRQ_STS_A>;
impl<'a, REG> THS_DATA_IRQ_STS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(THS_DATA_IRQ_STS_A::NO_EFFECT)
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(THS_DATA_IRQ_STS_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates the pending status of the sensor's data interrupt.\n\nWrite 1 to clear the pending status."]
    #[inline(always)]
    pub fn ths_data_irq_sts(&self) -> THS_DATA_IRQ_STS_R {
        THS_DATA_IRQ_STS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the pending status of the sensor's data interrupt.\n\nWrite 1 to clear the pending status."]
    #[inline(always)]
    #[must_use]
    pub fn ths_data_irq_sts(&mut self) -> THS_DATA_IRQ_STS_W<THS_DATA_INTS_SPEC> {
        THS_DATA_IRQ_STS_W::new(self, 0)
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
#[doc = "THS Data Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_data_ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_data_ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_DATA_INTS_SPEC;
impl crate::RegisterSpec for THS_DATA_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_data_ints::R`](R) reader structure"]
impl crate::Readable for THS_DATA_INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_data_ints::W`](W) writer structure"]
impl crate::Writable for THS_DATA_INTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets ths_data_ints to value 0"]
impl crate::Resettable for THS_DATA_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
